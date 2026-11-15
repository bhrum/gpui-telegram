use crate::db::{ServerDatabase, SqliteResultExt};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use gpui_telegram_core::error::{Result, TelegramError};
use gpui_telegram_proto::computer_contracts::{
    AuthorizedControllerEntry, BotComputerProfileDto, CloudAuthMethod, ComputerNodeDto,
    ComputerNodeKind, ComputerRunPolicy, RegisterLocalComputerRequest, UpdateBotComputerRequest,
    UpdateComputerPolicyRequest, UpsertCloudComputerRequest,
};
use hkdf::Hkdf;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use zeroize::Zeroizing;

pub const DEFAULT_SANDBOX_NODE_ID: &str = "sandbox:default";
pub const LOCAL_NODE_HEARTBEAT_TTL_MS: i64 = 60_000;
const COMPUTER_SECRET_ENV: &str = "GPUI_COMPUTER_SECRET_KEY";
const COMPUTER_SECRET_FALLBACK: &[u8] = b"gpui-telegram-server-computer-control-fallback-key-v1";
const COMPUTER_HKDF_INFO: &[u8] = b"gpui-telegram-computer-node-secret-v1";

pub const MIGRATION_V28_COMPUTER_CONTROL: &str = r#"
CREATE TABLE IF NOT EXISTS computer_nodes (
    node_id TEXT PRIMARY KEY,
    owner_user_id INTEGER NOT NULL,
    kind TEXT NOT NULL,
    display_name TEXT NOT NULL,
    ip_address TEXT NOT NULL DEFAULT '',
    host TEXT,
    ssh_port INTEGER,
    ssh_username TEXT,
    auth_method TEXT,
    agent_endpoint_url TEXT,
    workdir TEXT,
    encrypted_secret BLOB,
    secret_nonce BLOB,
    os_name TEXT NOT NULL DEFAULT '',
    resolution TEXT NOT NULL DEFAULT '',
    device_id TEXT,
    run_policy TEXT NOT NULL DEFAULT 'ask_every_time',
    authorized_controllers_json TEXT NOT NULL DEFAULT '[]',
    last_seen_ms INTEGER NOT NULL DEFAULT 0,
    latest_frame_data_url TEXT,
    created_at_ms INTEGER NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS idx_computer_nodes_owner_kind
    ON computer_nodes(owner_user_id, kind);

CREATE UNIQUE INDEX IF NOT EXISTS idx_computer_nodes_owner_device_id
    ON computer_nodes(owner_user_id, device_id)
    WHERE device_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS bot_computer_configs (
    owner_user_id INTEGER NOT NULL,
    bot_id INTEGER NOT NULL,
    bound_node_id TEXT,
    persona_prompt TEXT NOT NULL DEFAULT '',
    preferred_model TEXT NOT NULL DEFAULT '',
    approval_mode TEXT NOT NULL DEFAULT 'always_allow_authorized',
    allow_cross_device_control INTEGER NOT NULL DEFAULT 1,
    allow_cloud_self_config INTEGER NOT NULL DEFAULT 1,
    updated_at_ms INTEGER NOT NULL,
    PRIMARY KEY (owner_user_id, bot_id)
) STRICT;

CREATE TABLE IF NOT EXISTS computer_command_audit (
    command_id TEXT PRIMARY KEY,
    owner_user_id INTEGER NOT NULL,
    node_id TEXT NOT NULL,
    caller_label TEXT NOT NULL,
    op_summary TEXT NOT NULL,
    ok INTEGER NOT NULL,
    exit_code INTEGER NOT NULL,
    created_at_ms INTEGER NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS idx_computer_command_audit_owner_time
    ON computer_command_audit(owner_user_id, created_at_ms DESC);
"#;

#[derive(Debug, Clone)]
pub struct ComputerNodeSecretRecord {
    pub dto: ComputerNodeDto,
    pub encrypted_secret: Option<Vec<u8>>,
    pub secret_nonce: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputerAuditRecord {
    pub command_id: String,
    pub owner_user_id: u64,
    pub node_id: String,
    pub caller_label: String,
    pub op_summary: String,
    pub ok: bool,
    pub exit_code: i32,
    pub created_at_ms: i64,
}

pub fn resolve_computer_master_key() -> Zeroizing<Vec<u8>> {
    match std::env::var(COMPUTER_SECRET_ENV) {
        Ok(raw) if !raw.trim().is_empty() => Zeroizing::new(raw.into_bytes()),
        _ => Zeroizing::new(COMPUTER_SECRET_FALLBACK.to_vec()),
    }
}

fn derive_owner_key(master_secret: &[u8], owner_user_id: u64) -> Result<Zeroizing<[u8; 32]>> {
    let salt = owner_user_id.to_le_bytes();
    let hk = Hkdf::<Sha256>::new(Some(&salt), master_secret);
    let mut key = Zeroizing::new([0u8; 32]);
    hk.expand(COMPUTER_HKDF_INFO, &mut *key)
        .map_err(|e| TelegramError::Internal(format!("HKDF key derivation failed: {e}")))?;
    Ok(key)
}

pub fn encrypt_computer_secret(
    master_secret: &[u8],
    owner_user_id: u64,
    plaintext: &str,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = derive_owner_key(master_secret, owner_user_id)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&*key)
        .map_err(|e| TelegramError::Internal(format!("Cipher init failed: {e}")))?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let secret_buf = Zeroizing::new(plaintext.as_bytes().to_vec());
    let ciphertext = cipher
        .encrypt(&nonce, secret_buf.as_slice())
        .map_err(|e| TelegramError::Internal(format!("Secret encryption failed: {e}")))?;
    Ok((ciphertext, nonce.to_vec()))
}

pub fn decrypt_computer_secret(
    master_secret: &[u8],
    owner_user_id: u64,
    ciphertext: &[u8],
    nonce_bytes: &[u8],
) -> Result<Zeroizing<String>> {
    if nonce_bytes.len() != 12 {
        return Err(TelegramError::Internal(format!(
            "Invalid secret nonce length: {}",
            nonce_bytes.len()
        )));
    }
    let key = derive_owner_key(master_secret, owner_user_id)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&*key)
        .map_err(|e| TelegramError::Internal(format!("Cipher init failed: {e}")))?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = Zeroizing::new(
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| TelegramError::Internal(format!("Secret decryption failed: {e}")))?,
    );
    let s = String::from_utf8(plaintext.to_vec())
        .map_err(|e| TelegramError::Internal(format!("Decrypted secret is not UTF-8: {e}")))?;
    Ok(Zeroizing::new(s))
}

pub fn parse_node_kind(raw: &str) -> ComputerNodeKind {
    match raw.trim().to_ascii_lowercase().as_str() {
        "local_device" => ComputerNodeKind::LocalDevice,
        "cloud_server" => ComputerNodeKind::CloudServer,
        "cloud_sandbox" => ComputerNodeKind::CloudSandbox,
        _ => ComputerNodeKind::CloudServer,
    }
}

pub fn parse_run_policy(raw: &str, default_kind: ComputerNodeKind) -> ComputerRunPolicy {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ask_every_time" => ComputerRunPolicy::AskEveryTime,
        "allow_read_only" => ComputerRunPolicy::AllowReadOnly,
        "always_allow_authorized" => ComputerRunPolicy::AlwaysAllowAuthorized,
        "disabled" => ComputerRunPolicy::Disabled,
        _ => ComputerRunPolicy::default_for_kind(default_kind),
    }
}

pub fn parse_cloud_auth_method(raw: &str) -> Option<CloudAuthMethod> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ssh_password" | "password" => Some(CloudAuthMethod::SshPassword),
        "ssh_private_key" | "private_key" | "key" => Some(CloudAuthMethod::SshPrivateKey),
        "agent_http_token" | "http_token" | "agent_token" | "token" => {
            Some(CloudAuthMethod::AgentHttpToken)
        }
        _ => None,
    }
}

fn sandbox_storage_id(owner_user_id: u64) -> String {
    format!("sandbox:{owner_user_id}")
}

fn to_storage_node_id(owner_user_id: u64, external_node_id: &str) -> String {
    let trimmed = external_node_id.trim();
    if trimmed == DEFAULT_SANDBOX_NODE_ID {
        sandbox_storage_id(owner_user_id)
    } else {
        trimmed.to_string()
    }
}

fn to_external_node_id(owner_user_id: u64, storage_node_id: &str) -> String {
    if storage_node_id == sandbox_storage_id(owner_user_id)
        || storage_node_id == DEFAULT_SANDBOX_NODE_ID
    {
        DEFAULT_SANDBOX_NODE_ID.to_string()
    } else {
        storage_node_id.to_string()
    }
}

fn normalize_bound_node_id(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix("sandbox:") {
        if rest == "default" || rest.chars().all(|c| c.is_ascii_digit()) {
            return DEFAULT_SANDBOX_NODE_ID.to_string();
        }
    }
    trimmed.to_string()
}

pub fn default_virtual_sandbox_dto(
    owner_user_id: u64,
    bound_bot_ids: Vec<u64>,
    now_ms: i64,
) -> ComputerNodeDto {
    ComputerNodeDto {
        node_id: DEFAULT_SANDBOX_NODE_ID.to_string(),
        owner_user_id,
        kind: ComputerNodeKind::CloudSandbox,
        display_name: "免费云端沙箱 (Free Sandbox)".to_string(),
        ip_address: "127.0.0.1".to_string(),
        host: Some("sandbox.local".to_string()),
        ssh_port: None,
        ssh_username: None,
        auth_method: None,
        agent_endpoint_url: None,
        workdir: Some(
            std::env::temp_dir()
                .join(format!("gpui_sandbox_{owner_user_id}"))
                .to_string_lossy()
                .to_string(),
        ),
        has_saved_credential: true,
        os_name: format!("Sandbox ({})", std::env::consts::OS),
        resolution: "headless".to_string(),
        online: true,
        is_current_device: false,
        device_id: None,
        tier_badge: ComputerNodeKind::CloudSandbox
            .default_tier_badge()
            .to_string(),
        run_policy: ComputerRunPolicy::default_for_kind(ComputerNodeKind::CloudSandbox),
        authorized_controllers: Vec::new(),
        bound_bot_ids,
        last_seen_ms: now_ms,
        latest_frame_data_url: None,
    }
}

#[derive(Debug)]
struct RawComputerNodeRow {
    node_id: String,
    owner_user_id: i64,
    kind: String,
    display_name: String,
    ip_address: String,
    host: Option<String>,
    ssh_port: Option<i64>,
    ssh_username: Option<String>,
    auth_method: Option<String>,
    agent_endpoint_url: Option<String>,
    workdir: Option<String>,
    encrypted_secret: Option<Vec<u8>>,
    secret_nonce: Option<Vec<u8>>,
    os_name: String,
    resolution: String,
    device_id: Option<String>,
    run_policy: String,
    authorized_controllers_json: String,
    last_seen_ms: i64,
    latest_frame_data_url: Option<String>,
    created_at_ms: i64,
}

impl RawComputerNodeRow {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            node_id: row.get(0)?,
            owner_user_id: row.get(1)?,
            kind: row.get(2)?,
            display_name: row.get(3)?,
            ip_address: row.get(4)?,
            host: row.get(5)?,
            ssh_port: row.get(6)?,
            ssh_username: row.get(7)?,
            auth_method: row.get(8)?,
            agent_endpoint_url: row.get(9)?,
            workdir: row.get(10)?,
            encrypted_secret: row.get(11)?,
            secret_nonce: row.get(12)?,
            os_name: row.get(13)?,
            resolution: row.get(14)?,
            device_id: row.get(15)?,
            run_policy: row.get(16)?,
            authorized_controllers_json: row.get(17)?,
            last_seen_ms: row.get(18)?,
            latest_frame_data_url: row.get(19)?,
            created_at_ms: row.get(20)?,
        })
    }

    fn into_secret_record(
        self,
        owner_user_id: u64,
        current_device_id: Option<&str>,
        bound_bot_ids_map: &HashMap<String, Vec<u64>>,
        now_ms: i64,
    ) -> ComputerNodeSecretRecord {
        let kind = parse_node_kind(&self.kind);
        let run_policy = parse_run_policy(&self.run_policy, kind);
        let auth_method = self.auth_method.as_deref().and_then(parse_cloud_auth_method);
        let authorized_controllers: Vec<AuthorizedControllerEntry> =
            serde_json::from_str(&self.authorized_controllers_json).unwrap_or_default();
        let external_node_id = to_external_node_id(owner_user_id, &self.node_id);
        let has_saved_credential = self
            .encrypted_secret
            .as_ref()
            .is_some_and(|b| !b.is_empty())
            && self.secret_nonce.as_ref().is_some_and(|n| n.len() == 12);
        let is_current_device = kind == ComputerNodeKind::LocalDevice
            && current_device_id
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .is_some_and(|cur| self.device_id.as_deref() == Some(cur));
        let online = match kind {
            ComputerNodeKind::LocalDevice => {
                self.last_seen_ms > 0
                    && now_ms.saturating_sub(self.last_seen_ms) < LOCAL_NODE_HEARTBEAT_TTL_MS
            }
            ComputerNodeKind::CloudSandbox => true,
            ComputerNodeKind::CloudServer => self.last_seen_ms > 0 || has_saved_credential,
        };
        let bound_bot_ids = bound_bot_ids_map
            .get(&external_node_id)
            .cloned()
            .unwrap_or_default();
        let last_seen_ms = if kind == ComputerNodeKind::CloudSandbox && self.last_seen_ms == 0 {
            now_ms.max(self.created_at_ms)
        } else {
            self.last_seen_ms
        };
        let workdir = if kind == ComputerNodeKind::CloudSandbox {
            self.workdir.or_else(|| {
                Some(
                    std::env::temp_dir()
                        .join(format!("gpui_sandbox_{owner_user_id}"))
                        .to_string_lossy()
                        .to_string(),
                )
            })
        } else {
            self.workdir
        };

        ComputerNodeSecretRecord {
            dto: ComputerNodeDto {
                node_id: external_node_id,
                owner_user_id: self.owner_user_id.max(0) as u64,
                kind,
                display_name: self.display_name,
                ip_address: self.ip_address,
                host: self.host,
                ssh_port: self.ssh_port.and_then(|p| u16::try_from(p).ok()),
                ssh_username: self.ssh_username,
                auth_method,
                agent_endpoint_url: self.agent_endpoint_url,
                workdir,
                has_saved_credential,
                os_name: self.os_name,
                resolution: self.resolution,
                online,
                is_current_device,
                device_id: self.device_id,
                tier_badge: kind.default_tier_badge().to_string(),
                run_policy,
                authorized_controllers,
                bound_bot_ids,
                last_seen_ms,
                latest_frame_data_url: self.latest_frame_data_url,
            },
            encrypted_secret: self.encrypted_secret,
            secret_nonce: self.secret_nonce,
        }
    }
}

const SELECT_NODE_COLS: &str = r#"
    node_id, owner_user_id, kind, display_name, ip_address, host, ssh_port,
    ssh_username, auth_method, agent_endpoint_url, workdir, encrypted_secret,
    secret_nonce, os_name, resolution, device_id, run_policy,
    authorized_controllers_json, last_seen_ms, latest_frame_data_url, created_at_ms
"#;

impl ServerDatabase {
    fn load_bound_bot_ids_map(&self, owner_user_id: u64) -> Result<HashMap<String, Vec<u64>>> {
        self.pool.with_conn(|conn| {
            let mut stmt = conn
                .prepare(
                    r#"
                    SELECT bot_id, bound_node_id
                    FROM bot_computer_configs
                    WHERE owner_user_id = ?1 AND bound_node_id IS NOT NULL
                    ORDER BY bot_id ASC
                    "#,
                )
                .map_db()?;
            let mut rows = stmt.query(params![owner_user_id as i64]).map_db()?;
            let mut map: HashMap<String, Vec<u64>> = HashMap::new();
            while let Some(row) = rows.next().map_db()? {
                let bot_id: i64 = row.get(0).map_db()?;
                let bound_node_id: String = row.get(1).map_db()?;
                let normalized = normalize_bound_node_id(&bound_node_id);
                if !normalized.is_empty() && bot_id > 0 {
                    map.entry(normalized).or_default().push(bot_id as u64);
                }
            }
            Ok(map)
        })
    }

    pub fn list_computer_nodes(
        &self,
        owner_user_id: u64,
        current_device_id: Option<&str>,
        now_ms: i64,
    ) -> Result<Vec<ComputerNodeDto>> {
        let bound_map = self.load_bound_bot_ids_map(owner_user_id)?;
        let rows = self.pool.with_conn(|conn| {
            let sql = format!(
                "SELECT {SELECT_NODE_COLS} FROM computer_nodes WHERE owner_user_id = ?1 ORDER BY created_at_ms ASC, node_id ASC"
            );
            let mut stmt = conn.prepare(&sql).map_db()?;
            let mapped = stmt
                .query_map(params![owner_user_id as i64], RawComputerNodeRow::from_row)
                .map_db()?;
            let mut out = Vec::new();
            for item in mapped {
                out.push(item.map_db()?);
            }
            Ok(out)
        })?;

        let mut local_nodes = Vec::new();
        let mut cloud_nodes = Vec::new();
        let mut sandbox_node: Option<ComputerNodeDto> = None;

        for raw in rows {
            let dto = raw
                .into_secret_record(owner_user_id, current_device_id, &bound_map, now_ms)
                .dto;
            match dto.kind {
                ComputerNodeKind::LocalDevice => local_nodes.push(dto),
                ComputerNodeKind::CloudServer => cloud_nodes.push(dto),
                ComputerNodeKind::CloudSandbox => {
                    if dto.node_id == DEFAULT_SANDBOX_NODE_ID {
                        sandbox_node = Some(dto);
                    } else {
                        cloud_nodes.push(dto);
                    }
                }
            }
        }

        let mut combined = Vec::with_capacity(local_nodes.len() + cloud_nodes.len() + 1);
        combined.extend(local_nodes);
        combined.extend(cloud_nodes);
        combined.push(sandbox_node.unwrap_or_else(|| {
            default_virtual_sandbox_dto(
                owner_user_id,
                bound_map
                    .get(DEFAULT_SANDBOX_NODE_ID)
                    .cloned()
                    .unwrap_or_default(),
                now_ms,
            )
        }));
        Ok(combined)
    }

    pub fn get_computer_node_with_secret(
        &self,
        owner_user_id: u64,
        external_node_id: &str,
        current_device_id: Option<&str>,
        now_ms: i64,
    ) -> Result<Option<ComputerNodeSecretRecord>> {
        let bound_map = self.load_bound_bot_ids_map(owner_user_id)?;
        let storage_id = to_storage_node_id(owner_user_id, external_node_id);

        let raw = self.pool.with_conn(|conn| {
            let sql = format!(
                "SELECT {SELECT_NODE_COLS} FROM computer_nodes WHERE owner_user_id = ?1 AND node_id = ?2 LIMIT 1"
            );
            conn.query_row(
                &sql,
                params![owner_user_id as i64, &storage_id],
                RawComputerNodeRow::from_row,
            )
            .optional()
            .map_db()
        })?;

        if let Some(raw) = raw {
            return Ok(Some(raw.into_secret_record(
                owner_user_id,
                current_device_id,
                &bound_map,
                now_ms,
            )));
        }

        if external_node_id.trim() == DEFAULT_SANDBOX_NODE_ID {
            return Ok(Some(ComputerNodeSecretRecord {
                dto: default_virtual_sandbox_dto(
                    owner_user_id,
                    bound_map
                        .get(DEFAULT_SANDBOX_NODE_ID)
                        .cloned()
                        .unwrap_or_default(),
                    now_ms,
                ),
                encrypted_secret: None,
                secret_nonce: None,
            }));
        }

        Ok(None)
    }

    pub fn get_computer_node(
        &self,
        owner_user_id: u64,
        external_node_id: &str,
        current_device_id: Option<&str>,
        now_ms: i64,
    ) -> Result<Option<ComputerNodeDto>> {
        Ok(self
            .get_computer_node_with_secret(owner_user_id, external_node_id, current_device_id, now_ms)?
            .map(|rec| rec.dto))
    }

    pub fn upsert_local_computer_node(
        &self,
        owner_user_id: u64,
        req: &RegisterLocalComputerRequest,
        now_ms: i64,
    ) -> Result<ComputerNodeDto> {
        let device_id = req.device_id.trim();
        if device_id.is_empty() {
            return Err(TelegramError::Validation(
                "device_id must not be empty".to_string(),
            ));
        }

        let display_name = if req.display_name.trim().is_empty() {
            format!("Device {device_id}")
        } else {
            req.display_name.trim().to_string()
        };
        let ip_address = req.ip_address.trim().to_string();
        let os_name = req.os_name.trim().to_string();
        let resolution = req.resolution.trim().to_string();

        let existing_node_id: Option<String> = self.pool.with_conn(|conn| {
            conn.query_row(
                "SELECT node_id FROM computer_nodes WHERE owner_user_id = ?1 AND device_id = ?2 LIMIT 1",
                params![owner_user_id as i64, device_id],
                |row| row.get(0),
            )
            .optional()
            .map_db()
        })?;

        let node_id = existing_node_id.unwrap_or_else(|| format!("local:{owner_user_id}:{device_id}"));

        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                INSERT INTO computer_nodes (
                    node_id, owner_user_id, kind, display_name, ip_address,
                    os_name, resolution, device_id, run_policy,
                    authorized_controllers_json, last_seen_ms, created_at_ms, updated_at_ms
                ) VALUES (?1, ?2, 'local_device', ?3, ?4, ?5, ?6, ?7, 'ask_every_time', '[]', ?8, ?8, ?8)
                ON CONFLICT(node_id) DO UPDATE SET
                    display_name = excluded.display_name,
                    ip_address = excluded.ip_address,
                    os_name = excluded.os_name,
                    resolution = excluded.resolution,
                    device_id = excluded.device_id,
                    last_seen_ms = excluded.last_seen_ms,
                    updated_at_ms = excluded.updated_at_ms
                "#,
                params![
                    &node_id,
                    owner_user_id as i64,
                    &display_name,
                    &ip_address,
                    &os_name,
                    &resolution,
                    device_id,
                    now_ms,
                ],
            )
            .map_db()?;
            Ok(())
        })?;

        self.get_computer_node(owner_user_id, &node_id, Some(device_id), now_ms)?
            .ok_or_else(|| TelegramError::Internal("Failed to reload upserted local node".into()))
    }

    pub fn upsert_cloud_computer_node(
        &self,
        owner_user_id: u64,
        req: &UpsertCloudComputerRequest,
        master_secret: &[u8],
        current_device_id: Option<&str>,
        now_ms: i64,
    ) -> Result<ComputerNodeDto> {
        let host_trimmed = req.host.trim();
        let agent_url_trimmed = req
            .agent_endpoint_url
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        if host_trimmed.is_empty() && agent_url_trimmed.is_none() {
            return Err(TelegramError::Validation(
                "Cloud host or agent_endpoint_url is required".to_string(),
            ));
        }

        let requested_id = req
            .node_id
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let node_id = match requested_id {
            Some(id) => {
                if id == DEFAULT_SANDBOX_NODE_ID || id.starts_with("sandbox:") {
                    return Err(TelegramError::Validation(
                        "Cannot overwrite built-in cloud sandbox with cloud server config"
                            .to_string(),
                    ));
                }
                if let Some(existing_kind) = self.pool.with_conn(|conn| {
                    conn.query_row(
                        "SELECT kind, owner_user_id FROM computer_nodes WHERE node_id = ?1",
                        params![id],
                        |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)),
                    )
                    .optional()
                    .map_db()
                })? {
                    if existing_kind.1 != owner_user_id as i64 {
                        return Err(TelegramError::PermissionDenied(
                            "Computer node belongs to another user".to_string(),
                        ));
                    }
                    if existing_kind.0 != ComputerNodeKind::CloudServer.as_str() {
                        return Err(TelegramError::Validation(
                            "Target node_id is not a cloud server".to_string(),
                        ));
                    }
                }
                id.to_string()
            }
            None => format!("cloud:{}", uuid::Uuid::new_v4()),
        };

        let existing_creds: Option<(Option<Vec<u8>>, Option<Vec<u8>>)> =
            self.pool.with_conn(|conn| {
                conn.query_row(
                    "SELECT encrypted_secret, secret_nonce FROM computer_nodes WHERE owner_user_id = ?1 AND node_id = ?2",
                    params![owner_user_id as i64, &node_id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()
                .map_db()
            })?;

        let (encrypted_secret, secret_nonce) = match req.secret.as_deref() {
            Some(secret) if !secret.is_empty() => {
                let (ct, nonce) = encrypt_computer_secret(master_secret, owner_user_id, secret)?;
                (Some(ct), Some(nonce))
            }
            _ => existing_creds.unwrap_or((None, None)),
        };

        let display_name = if req.display_name.trim().is_empty() {
            if !host_trimmed.is_empty() {
                host_trimmed.to_string()
            } else {
                agent_url_trimmed.unwrap_or("Cloud Server").to_string()
            }
        } else {
            req.display_name.trim().to_string()
        };
        let effective_host = if host_trimmed.is_empty() {
            agent_url_trimmed.unwrap_or("localhost").to_string()
        } else {
            host_trimmed.to_string()
        };
        let effective_port = if req.port == 0 { 22 } else { req.port };
        let effective_username = if req.username.trim().is_empty() {
            "root".to_string()
        } else {
            req.username.trim().to_string()
        };
        let workdir = req
            .workdir
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        let default_policy = ComputerRunPolicy::default_for_kind(ComputerNodeKind::CloudServer);

        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                INSERT INTO computer_nodes (
                    node_id, owner_user_id, kind, display_name, ip_address, host,
                    ssh_port, ssh_username, auth_method, agent_endpoint_url, workdir,
                    encrypted_secret, secret_nonce, os_name, resolution,
                    run_policy, authorized_controllers_json, last_seen_ms,
                    created_at_ms, updated_at_ms
                ) VALUES (
                    ?1, ?2, 'cloud_server', ?3, ?4, ?4,
                    ?5, ?6, ?7, ?8, ?9,
                    ?10, ?11, 'Linux Cloud', 'headless',
                    ?12, '[]', ?13,
                    ?13, ?13
                )
                ON CONFLICT(node_id) DO UPDATE SET
                    display_name = excluded.display_name,
                    ip_address = excluded.ip_address,
                    host = excluded.host,
                    ssh_port = excluded.ssh_port,
                    ssh_username = excluded.ssh_username,
                    auth_method = excluded.auth_method,
                    agent_endpoint_url = excluded.agent_endpoint_url,
                    workdir = excluded.workdir,
                    encrypted_secret = excluded.encrypted_secret,
                    secret_nonce = excluded.secret_nonce,
                    last_seen_ms = excluded.last_seen_ms,
                    updated_at_ms = excluded.updated_at_ms
                "#,
                params![
                    &node_id,
                    owner_user_id as i64,
                    &display_name,
                    &effective_host,
                    effective_port as i64,
                    &effective_username,
                    req.auth_method.as_str(),
                    agent_url_trimmed,
                    workdir.as_deref(),
                    encrypted_secret.as_deref(),
                    secret_nonce.as_deref(),
                    default_policy.as_str(),
                    now_ms,
                ],
            )
            .map_db()?;
            Ok(())
        })?;

        if let Some(bot_id) = req.bind_bot_id.filter(|b| *b > 0) {
            let bind_req = UpdateBotComputerRequest {
                bound_node_id: Some(node_id.clone()),
                ..Default::default()
            };
            self.upsert_bot_computer_profile(owner_user_id, bot_id, &bind_req, now_ms)?;
        }

        self.get_computer_node(owner_user_id, &node_id, current_device_id, now_ms)?
            .ok_or_else(|| TelegramError::Internal("Failed to reload upserted cloud node".into()))
    }

    pub fn update_computer_node_policy(
        &self,
        owner_user_id: u64,
        external_node_id: &str,
        req: &UpdateComputerPolicyRequest,
        current_device_id: Option<&str>,
        now_ms: i64,
    ) -> Result<Option<ComputerNodeDto>> {
        let storage_id = to_storage_node_id(owner_user_id, external_node_id);

        if external_node_id.trim() == DEFAULT_SANDBOX_NODE_ID {
            let virtual_default = default_virtual_sandbox_dto(owner_user_id, Vec::new(), now_ms);
            self.pool.with_conn(|conn| {
                conn.execute(
                    r#"
                    INSERT OR IGNORE INTO computer_nodes (
                        node_id, owner_user_id, kind, display_name, ip_address, host,
                        workdir, os_name, resolution, run_policy,
                        authorized_controllers_json, last_seen_ms, created_at_ms, updated_at_ms
                    ) VALUES (
                        ?1, ?2, 'cloud_sandbox', ?3, ?4, ?5,
                        ?6, ?7, ?8, ?9,
                        '[]', ?10, ?10, ?10
                    )
                    "#,
                    params![
                        &storage_id,
                        owner_user_id as i64,
                        &virtual_default.display_name,
                        &virtual_default.ip_address,
                        virtual_default.host.as_deref(),
                        virtual_default.workdir.as_deref(),
                        &virtual_default.os_name,
                        &virtual_default.resolution,
                        virtual_default.run_policy.as_str(),
                        now_ms,
                    ],
                )
                .map_db()?;
                Ok(())
            })?;
        }

        let current: Option<(String, String, String, String)> = self.pool.with_conn(|conn| {
            conn.query_row(
                r#"
                SELECT kind, display_name, run_policy, authorized_controllers_json
                FROM computer_nodes
                WHERE owner_user_id = ?1 AND node_id = ?2
                LIMIT 1
                "#,
                params![owner_user_id as i64, &storage_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .optional()
            .map_db()
        })?;

        let Some((kind_str, mut display_name, mut policy_str, controllers_json)) = current else {
            return Ok(None);
        };

        let _kind = parse_node_kind(&kind_str);
        if let Some(ref p) = req.run_policy {
            policy_str = p.as_str().to_string();
        }
        if let Some(ref name) = req.display_name {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                display_name = trimmed.to_string();
            }
        }

        let mut controllers: Vec<AuthorizedControllerEntry> =
            serde_json::from_str(&controllers_json).unwrap_or_default();

        if let Some(ref remove_id) = req.remove_controller_id {
            let trimmed = remove_id.trim();
            controllers.retain(|entry| entry.controller_id != trimmed);
        }
        if let Some(ref add) = req.add_controller {
            let mut normalized = add.clone();
            normalized.controller_id = normalized.controller_id.trim().to_string();
            if normalized.added_at_ms == 0 {
                normalized.added_at_ms = now_ms;
            }
            if !normalized.controller_id.is_empty() {
                if let Some(existing) = controllers
                    .iter_mut()
                    .find(|e| e.controller_id == normalized.controller_id)
                {
                    *existing = normalized;
                } else {
                    controllers.push(normalized);
                }
            }
        }

        let updated_json = serde_json::to_string(&controllers)?;

        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                UPDATE computer_nodes
                SET display_name = ?1,
                    run_policy = ?2,
                    authorized_controllers_json = ?3,
                    updated_at_ms = ?4
                WHERE owner_user_id = ?5 AND node_id = ?6
                "#,
                params![
                    &display_name,
                    &policy_str,
                    &updated_json,
                    now_ms,
                    owner_user_id as i64,
                    &storage_id,
                ],
            )
            .map_db()?;
            Ok(())
        })?;

        self.get_computer_node(owner_user_id, external_node_id, current_device_id, now_ms)
    }

    pub fn delete_computer_node(
        &self,
        owner_user_id: u64,
        external_node_id: &str,
        now_ms: i64,
    ) -> Result<bool> {
        let trimmed = external_node_id.trim();
        if trimmed == DEFAULT_SANDBOX_NODE_ID || trimmed == sandbox_storage_id(owner_user_id) {
            return Err(TelegramError::Validation(
                "Built-in cloud sandbox cannot be deleted".to_string(),
            ));
        }

        let deleted = self.pool.with_conn(|conn| {
            let changed = conn
                .execute(
                    "DELETE FROM computer_nodes WHERE owner_user_id = ?1 AND node_id = ?2",
                    params![owner_user_id as i64, trimmed],
                )
                .map_db()?;
            if changed > 0 {
                conn.execute(
                    r#"
                    UPDATE bot_computer_configs
                    SET bound_node_id = NULL, updated_at_ms = ?1
                    WHERE owner_user_id = ?2 AND bound_node_id = ?3
                    "#,
                    params![now_ms, owner_user_id as i64, trimmed],
                )
                .map_db()?;
            }
            Ok(changed > 0)
        })?;
        Ok(deleted)
    }

    pub fn touch_computer_node_heartbeat_and_frame(
        &self,
        owner_user_id: u64,
        external_node_id: &str,
        frame_data_url: Option<&str>,
        now_ms: i64,
    ) -> Result<()> {
        let storage_id = to_storage_node_id(owner_user_id, external_node_id);
        let frame = frame_data_url
            .map(str::trim)
            .filter(|s| !s.is_empty());
        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                UPDATE computer_nodes
                SET last_seen_ms = ?1,
                    updated_at_ms = ?1,
                    latest_frame_data_url = COALESCE(?2, latest_frame_data_url)
                WHERE owner_user_id = ?3 AND node_id = ?4
                "#,
                params![now_ms, frame, owner_user_id as i64, &storage_id],
            )
            .map_db()?;
            Ok(())
        })
    }

    pub fn get_bot_computer_profile(
        &self,
        owner_user_id: u64,
        bot_id: u64,
    ) -> Result<Option<BotComputerProfileDto>> {
        self.pool.with_conn(|conn| {
            conn.query_row(
                r#"
                SELECT bot_id, bound_node_id, persona_prompt, preferred_model,
                       approval_mode, allow_cross_device_control, allow_cloud_self_config,
                       updated_at_ms
                FROM bot_computer_configs
                WHERE owner_user_id = ?1 AND bot_id = ?2
                LIMIT 1
                "#,
                params![owner_user_id as i64, bot_id as i64],
                |row| {
                    let bot_id_raw: i64 = row.get(0)?;
                    let bound_raw: Option<String> = row.get(1)?;
                    let approval_raw: String = row.get(4)?;
                    let cross: i64 = row.get(5)?;
                    let self_cfg: i64 = row.get(6)?;
                    Ok(BotComputerProfileDto {
                        bot_id: bot_id_raw.max(0) as u64,
                        bound_node_id: bound_raw.map(|s| normalize_bound_node_id(&s)),
                        persona_prompt: row.get(2)?,
                        preferred_model: row.get(3)?,
                        approval_mode: parse_run_policy(
                            &approval_raw,
                            ComputerNodeKind::CloudServer,
                        ),
                        allow_cross_device_control: cross != 0,
                        allow_cloud_self_config: self_cfg != 0,
                        updated_at_ms: row.get(7)?,
                    })
                },
            )
            .optional()
            .map_db()
        })
    }

    pub fn get_bot_computer_profile_or_default(
        &self,
        owner_user_id: u64,
        bot_id: u64,
    ) -> Result<BotComputerProfileDto> {
        if let Some(existing) = self.get_bot_computer_profile(owner_user_id, bot_id)? {
            return Ok(existing);
        }
        Ok(BotComputerProfileDto {
            bot_id,
            bound_node_id: None,
            persona_prompt: String::new(),
            preferred_model: String::new(),
            approval_mode: ComputerRunPolicy::AlwaysAllowAuthorized,
            allow_cross_device_control: true,
            allow_cloud_self_config: true,
            updated_at_ms: 0,
        })
    }

    pub fn upsert_bot_computer_profile(
        &self,
        owner_user_id: u64,
        bot_id: u64,
        req: &UpdateBotComputerRequest,
        now_ms: i64,
    ) -> Result<BotComputerProfileDto> {
        let mut current = self.get_bot_computer_profile_or_default(owner_user_id, bot_id)?;

        if req.clear_bound_node {
            current.bound_node_id = None;
        } else if let Some(ref node_id) = req.bound_node_id {
            let normalized = normalize_bound_node_id(node_id);
            if normalized.is_empty() {
                current.bound_node_id = None;
            } else {
                let exists = self
                    .get_computer_node(owner_user_id, &normalized, None, now_ms)?
                    .is_some();
                if !exists {
                    return Err(TelegramError::NotFound(format!(
                        "Computer node `{normalized}` not found for this user"
                    )));
                }
                current.bound_node_id = Some(normalized);
            }
        }

        if let Some(ref persona) = req.persona_prompt {
            current.persona_prompt = persona.clone();
        }
        if let Some(ref model) = req.preferred_model {
            current.preferred_model = model.trim().to_string();
        }
        if let Some(mode) = req.approval_mode {
            current.approval_mode = mode;
        }
        if let Some(cross) = req.allow_cross_device_control {
            current.allow_cross_device_control = cross;
        }
        if let Some(self_cfg) = req.allow_cloud_self_config {
            current.allow_cloud_self_config = self_cfg;
        }
        current.updated_at_ms = now_ms;

        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                INSERT INTO bot_computer_configs (
                    owner_user_id, bot_id, bound_node_id, persona_prompt, preferred_model,
                    approval_mode, allow_cross_device_control, allow_cloud_self_config, updated_at_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ON CONFLICT(owner_user_id, bot_id) DO UPDATE SET
                    bound_node_id = excluded.bound_node_id,
                    persona_prompt = excluded.persona_prompt,
                    preferred_model = excluded.preferred_model,
                    approval_mode = excluded.approval_mode,
                    allow_cross_device_control = excluded.allow_cross_device_control,
                    allow_cloud_self_config = excluded.allow_cloud_self_config,
                    updated_at_ms = excluded.updated_at_ms
                "#,
                params![
                    owner_user_id as i64,
                    bot_id as i64,
                    current.bound_node_id.as_deref(),
                    &current.persona_prompt,
                    &current.preferred_model,
                    current.approval_mode.as_str(),
                    if current.allow_cross_device_control { 1i64 } else { 0i64 },
                    if current.allow_cloud_self_config { 1i64 } else { 0i64 },
                    current.updated_at_ms,
                ],
            )
            .map_db()?;
            Ok(())
        })?;

        Ok(current)
    }

    pub fn sync_bot_ai_persona(
        &self,
        bot_id: u64,
        persona_prompt: &str,
        now_ms: i64,
    ) -> Result<()> {
        self.pool.with_conn(|conn| {
            conn.execute(
                "UPDATE bots_registry SET ai_persona = ?1, updated_at = ?2 WHERE bot_user_id = ?3",
                params![persona_prompt, now_ms, bot_id as i64],
            )
            .map_db()?;
            Ok(())
        })
    }

    pub fn record_computer_audit(
        &self,
        command_id: &str,
        owner_user_id: u64,
        external_node_id: &str,
        caller_label: &str,
        op_summary: &str,
        ok: bool,
        exit_code: i32,
        created_at_ms: i64,
    ) -> Result<()> {
        let normalized_node_id = normalize_bound_node_id(external_node_id);
        self.pool.with_conn(|conn| {
            conn.execute(
                r#"
                INSERT INTO computer_command_audit (
                    command_id, owner_user_id, node_id, caller_label, op_summary, ok, exit_code, created_at_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ON CONFLICT(command_id) DO UPDATE SET
                    ok = excluded.ok,
                    exit_code = excluded.exit_code,
                    created_at_ms = excluded.created_at_ms
                "#,
                params![
                    command_id,
                    owner_user_id as i64,
                    &normalized_node_id,
                    caller_label,
                    op_summary,
                    if ok { 1i64 } else { 0i64 },
                    exit_code as i64,
                    created_at_ms,
                ],
            )
            .map_db()?;
            Ok(())
        })
    }

    pub fn list_computer_audit(
        &self,
        owner_user_id: u64,
        limit: usize,
    ) -> Result<Vec<ComputerAuditRecord>> {
        let lim = limit.clamp(1, 500) as i64;
        self.pool.with_conn(|conn| {
            let mut stmt = conn
                .prepare(
                    r#"
                    SELECT command_id, owner_user_id, node_id, caller_label, op_summary, ok, exit_code, created_at_ms
                    FROM computer_command_audit
                    WHERE owner_user_id = ?1
                    ORDER BY created_at_ms DESC
                    LIMIT ?2
                    "#,
                )
                .map_db()?;
            let rows = stmt
                .query_map(params![owner_user_id as i64, lim], |row| {
                    Ok(ComputerAuditRecord {
                        command_id: row.get(0)?,
                        owner_user_id: row.get::<_, i64>(1)?.max(0) as u64,
                        node_id: row.get(2)?,
                        caller_label: row.get(3)?,
                        op_summary: row.get(4)?,
                        ok: row.get::<_, i64>(5)? != 0,
                        exit_code: row.get::<_, i64>(6)? as i32,
                        created_at_ms: row.get(7)?,
                    })
                })
                .map_db()?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row.map_db()?);
            }
            Ok(out)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_telegram_proto::computer_contracts::AuthorizedControllerKind;

    #[test]
    fn test_v28_migration_and_secret_encryption_roundtrip() {
        let db = ServerDatabase::open_in_memory().expect("open in-memory db");
        let master = b"test-server-master-secret-32-bytes";
        let secret_plain = "super-secret-ssh-token-12345";

        let req = UpsertCloudComputerRequest {
            node_id: None,
            display_name: "Prod Cloud Box".to_string(),
            host: "203.0.113.10".to_string(),
            port: 2222,
            username: "deploy".to_string(),
            auth_method: CloudAuthMethod::SshPassword,
            secret: Some(secret_plain.to_string()),
            agent_endpoint_url: None,
            workdir: Some("/srv/app".to_string()),
            bind_bot_id: Some(9001),
            verify_connection: false,
        };

        let saved = db
            .upsert_cloud_computer_node(42, &req, master, None, 1_700_000_000_000)
            .expect("upsert cloud server");
        assert!(saved.has_saved_credential);
        assert_eq!(saved.kind, ComputerNodeKind::CloudServer);
        assert_eq!(saved.run_policy, ComputerRunPolicy::AlwaysAllowAuthorized);
        assert_eq!(saved.bound_bot_ids, vec![9001]);

        let serialized = serde_json::to_string(&saved).expect("serialize dto");
        assert!(!serialized.contains(secret_plain));

        let with_secret = db
            .get_computer_node_with_secret(42, &saved.node_id, None, 1_700_000_000_000)
            .expect("query secret record")
            .expect("record exists");
        let ct = with_secret.encrypted_secret.as_deref().expect("ciphertext");
        let nonce = with_secret.secret_nonce.as_deref().expect("nonce");
        assert_ne!(ct, secret_plain.as_bytes());
        assert_eq!(nonce.len(), 12);

        let decrypted = decrypt_computer_secret(master, 42, ct, nonce).expect("decrypt");
        assert_eq!(decrypted.as_str(), secret_plain);

        // Wrong owner cannot decrypt due to owner-scoped HKDF salt.
        assert!(decrypt_computer_secret(master, 43, ct, nonce).is_err());

        // Updating without providing `secret` preserves existing encrypted credential.
        let update_req = UpsertCloudComputerRequest {
            node_id: Some(saved.node_id.clone()),
            display_name: "Prod Cloud Box Renamed".to_string(),
            host: "203.0.113.10".to_string(),
            port: 2222,
            username: "deploy".to_string(),
            auth_method: CloudAuthMethod::SshPassword,
            secret: None,
            agent_endpoint_url: None,
            workdir: Some("/srv/app2".to_string()),
            bind_bot_id: None,
            verify_connection: false,
        };
        let updated = db
            .upsert_cloud_computer_node(42, &update_req, master, None, 1_700_000_001_000)
            .expect("update without secret");
        assert!(updated.has_saved_credential);
        assert_eq!(updated.display_name, "Prod Cloud Box Renamed");

        let reloaded = db
            .get_computer_node_with_secret(42, &saved.node_id, None, 1_700_000_001_000)
            .expect("reload")
            .expect("exists");
        let decrypted2 = decrypt_computer_secret(
            master,
            42,
            reloaded.encrypted_secret.as_deref().unwrap(),
            reloaded.secret_nonce.as_deref().unwrap(),
        )
        .expect("decrypt preserved");
        assert_eq!(decrypted2.as_str(), secret_plain);
    }

    #[test]
    fn test_local_device_policy_controllers_binding_and_audit() {
        let db = ServerDatabase::open_in_memory().expect("open db");
        let local = db
            .upsert_local_computer_node(
                7,
                &RegisterLocalComputerRequest {
                    device_id: "mac-studio-1".to_string(),
                    display_name: "Studio".to_string(),
                    ip_address: "192.168.1.50".to_string(),
                    os_name: "macOS".to_string(),
                    resolution: "2560x1440".to_string(),
                },
                10_000,
            )
            .expect("register local");
        assert_eq!(local.run_policy, ComputerRunPolicy::AskEveryTime);
        assert!(local.is_current_device);
        assert!(local.online);

        let patched = db
            .update_computer_node_policy(
                7,
                &local.node_id,
                &UpdateComputerPolicyRequest {
                    run_policy: Some(ComputerRunPolicy::AlwaysAllowAuthorized),
                    add_controller: Some(AuthorizedControllerEntry {
                        controller_id: "bot:501".to_string(),
                        kind: AuthorizedControllerKind::Bot,
                        display_name: "OpsBot".to_string(),
                        added_at_ms: 10_500,
                    }),
                    remove_controller_id: None,
                    display_name: Some("My Mac Studio".to_string()),
                },
                Some("mac-studio-1"),
                11_000,
            )
            .expect("patch")
            .expect("node exists");
        assert_eq!(patched.run_policy, ComputerRunPolicy::AlwaysAllowAuthorized);
        assert_eq!(patched.display_name, "My Mac Studio");
        assert_eq!(patched.authorized_controllers.len(), 1);

        // Heartbeat upsert preserves existing policy and controllers.
        let heartbeat = db
            .upsert_local_computer_node(
                7,
                &RegisterLocalComputerRequest {
                    device_id: "mac-studio-1".to_string(),
                    display_name: "My Mac Studio".to_string(),
                    ip_address: "192.168.1.51".to_string(),
                    os_name: "macOS".to_string(),
                    resolution: "2560x1440".to_string(),
                },
                20_000,
            )
            .expect("heartbeat");
        assert_eq!(heartbeat.run_policy, ComputerRunPolicy::AlwaysAllowAuthorized);
        assert_eq!(heartbeat.authorized_controllers.len(), 1);

        // Online check beyond 60s TTL.
        let stale_list = db
            .list_computer_nodes(7, Some("mac-studio-1"), 90_000)
            .expect("list");
        let stale_local = stale_list
            .iter()
            .find(|n| n.node_id == local.node_id)
            .expect("local node");
        assert!(!stale_local.online);
        assert!(stale_list
            .iter()
            .any(|n| n.node_id == DEFAULT_SANDBOX_NODE_ID && n.tier_badge == "免费"));

        // Bot binding + deletion clears bound_node_id.
        let profile = db
            .upsert_bot_computer_profile(
                7,
                501,
                &UpdateBotComputerRequest {
                    bound_node_id: Some(local.node_id.clone()),
                    persona_prompt: Some("Cloud Ops Assistant".to_string()),
                    preferred_model: Some("gpt-5".to_string()),
                    ..Default::default()
                },
                21_000,
            )
            .expect("bind bot");
        assert_eq!(profile.bound_node_id.as_deref(), Some(local.node_id.as_str()));

        db.record_computer_audit("cmd-1", 7, &local.node_id, "OpsBot", "shell_exec", true, 0, 22_000)
            .expect("audit");
        let audits = db.list_computer_audit(7, 10).expect("list audit");
        assert_eq!(audits.len(), 1);

        assert!(db.delete_computer_node(7, &local.node_id, 23_000).expect("delete"));
        let cleared = db
            .get_bot_computer_profile(7, 501)
            .expect("get bot")
            .expect("bot row");
        assert_eq!(cleared.bound_node_id, None);
    }
}
