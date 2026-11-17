#![allow(dead_code)]

use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};
use gpui_telegram_server::db::SqlitePool;
use gpui_telegram_test_harness::tier4_e2e::E2eSystemDriver;
use hkdf::Hkdf;
use parking_lot::Mutex;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use x25519_dalek::{PublicKey, StaticSecret};

// ============================================================================
// M2 Pending Client Outbox Harness
// (Persists queue in SQLite WAL via SqlitePool; delegates server delivery &
//  client_nonce deduplication to EphemeralTestServer HTTP endpoints.)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxStatus {
    Pending,
    Sending,
    Sent,
    Failed,
}

impl OutboxStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Sending => "Sending",
            Self::Sent => "Sent",
            Self::Failed => "Failed",
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "Sending" => Self::Sending,
            "Sent" => Self::Sent,
            "Failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxRecord {
    pub client_nonce: u64,
    pub chat_id: i64,
    pub text: String,
    pub status: OutboxStatus,
    pub retry_count: u32,
    pub next_retry_at_ms: i64,
}

pub struct OutboxSimulator {
    pool: SqlitePool,
    nonce_counter: AtomicU64,
}

fn sql_err(e: rusqlite::Error) -> gpui_telegram_core::error::TelegramError {
    gpui_telegram_core::error::TelegramError::Database(e.to_string())
}

impl OutboxSimulator {
    pub fn new() -> Self {
        let pool = SqlitePool::open_in_memory().expect("open in-memory outbox sqlite pool");
        Self::from_pool(pool)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let pool = SqlitePool::open(path.as_ref()).map_err(|e| e.to_string())?;
        Ok(Self::from_pool(pool))
    }

    fn from_pool(pool: SqlitePool) -> Self {
        let max_nonce: i64 = pool
            .with_writer(|conn| {
                conn.execute_batch(
                    "CREATE TABLE IF NOT EXISTS outbox_messages (
                        client_nonce INTEGER PRIMARY KEY,
                        chat_id INTEGER NOT NULL,
                        text TEXT NOT NULL,
                        status TEXT NOT NULL,
                        retry_count INTEGER NOT NULL,
                        next_retry_at_ms INTEGER NOT NULL
                    );",
                )
                .map_err(sql_err)?;
                let max_n: i64 = conn
                    .query_row(
                        "SELECT COALESCE(MAX(client_nonce), 10000) FROM outbox_messages",
                        [],
                        |r| r.get(0),
                    )
                    .unwrap_or(10000);
                Ok(max_n)
            })
            .expect("init outbox_messages schema");

        Self {
            pool,
            nonce_counter: AtomicU64::new((max_nonce as u64).max(10000) + 1),
        }
    }

    pub fn compute_backoff_delay_ms(retry_count: u32) -> u64 {
        let base_delay = 500u64;
        let factor = 1u64.checked_shl(retry_count.min(6)).unwrap_or(64);
        (base_delay * factor).min(30_000)
    }

    pub fn enqueue(&self, chat_id: i64, text: &str) -> Result<u64, String> {
        if chat_id <= 0 {
            return Err("Invalid chat_id: must be positive".into());
        }
        if text.trim().is_empty() {
            return Err("Outbox message text cannot be empty".into());
        }
        let nonce = self.nonce_counter.fetch_add(1, Ordering::SeqCst);
        let now_ms = chrono::Utc::now().timestamp_millis();
        self.pool
            .with_writer(|conn| {
                conn.execute(
                    "INSERT INTO outbox_messages (client_nonce, chat_id, text, status, retry_count, next_retry_at_ms)
                     VALUES (?1, ?2, ?3, 'Pending', 0, ?4)",
                    rusqlite::params![nonce as i64, chat_id, text, now_ms],
                )
                .map_err(sql_err)?;
                Ok(())
            })
            .map_err(|e| e.to_string())?;
        Ok(nonce)
    }

    pub fn get_record(&self, nonce: u64) -> Option<OutboxRecord> {
        self.pool
            .with_reader(|conn| {
                let mut stmt = conn
                    .prepare(
                        "SELECT client_nonce, chat_id, text, status, retry_count, next_retry_at_ms
                         FROM outbox_messages WHERE client_nonce = ?1",
                    )
                    .map_err(sql_err)?;
                let mut rows = stmt.query(rusqlite::params![nonce as i64]).map_err(sql_err)?;
                if let Some(row) = rows.next().map_err(sql_err)? {
                    let cn: i64 = row.get(0).map_err(sql_err)?;
                    let chat_id: i64 = row.get(1).map_err(sql_err)?;
                    let text: String = row.get(2).map_err(sql_err)?;
                    let status_str: String = row.get(3).map_err(sql_err)?;
                    let retry_count: i64 = row.get(4).map_err(sql_err)?;
                    let next_retry_at_ms: i64 = row.get(5).map_err(sql_err)?;
                    Ok(Some(OutboxRecord {
                        client_nonce: cn as u64,
                        chat_id,
                        text,
                        status: OutboxStatus::from_str(&status_str),
                        retry_count: retry_count as u32,
                        next_retry_at_ms,
                    }))
                } else {
                    Ok(None)
                }
            })
            .ok()
            .flatten()
    }

    pub fn list_pending(&self) -> Vec<OutboxRecord> {
        self.pool
            .with_reader(|conn| {
                let mut stmt = conn
                    .prepare(
                        "SELECT client_nonce, chat_id, text, status, retry_count, next_retry_at_ms
                         FROM outbox_messages
                         WHERE status IN ('Pending', 'Sending')
                         ORDER BY client_nonce ASC",
                    )
                    .map_err(sql_err)?;
                let rows = stmt
                    .query_map([], |row| {
                        Ok(OutboxRecord {
                            client_nonce: row.get::<_, i64>(0)? as u64,
                            chat_id: row.get(1)?,
                            text: row.get(2)?,
                            status: OutboxStatus::from_str(&row.get::<_, String>(3)?),
                            retry_count: row.get::<_, i64>(4)? as u32,
                            next_retry_at_ms: row.get(5)?,
                        })
                    })
                    .map_err(sql_err)?;
                let mut list = Vec::new();
                for r in rows {
                    list.push(r.map_err(sql_err)?);
                }
                Ok(list)
            })
            .unwrap_or_default()
    }

    /// Checks backoff eligibility (`now_ms >= next_retry_at_ms`) and marks due items Sent.
    /// Used for offline/backoff state-machine unit verification without a server.
    pub fn flush_pending(&self, is_online: bool) -> Result<usize, String> {
        if !is_online {
            return Err("Cannot flush: client is offline".into());
        }
        let now_ms = chrono::Utc::now().timestamp_millis();
        self.pool
            .with_transaction(|tx| {
                let updated = tx
                    .execute(
                        "UPDATE outbox_messages
                         SET status = 'Sent'
                         WHERE status IN ('Pending', 'Sending') AND next_retry_at_ms <= ?1",
                        rusqlite::params![now_ms],
                    )
                    .map_err(sql_err)?;
                Ok(updated)
            })
            .map_err(|e| e.to_string())
    }

    pub fn simulate_failure(&self, nonce: u64, is_permanent: bool) {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let _ = self.pool.with_writer(|conn| {
            if let Ok((retry_count,)) = conn.query_row(
                "SELECT retry_count FROM outbox_messages WHERE client_nonce = ?1",
                rusqlite::params![nonce as i64],
                |r| Ok((r.get::<_, i64>(0)? as u32,)),
            ) {
                let next_retries = retry_count + 1;
                if is_permanent || next_retries >= 10 {
                    conn.execute(
                        "UPDATE outbox_messages SET retry_count = ?1, status = 'Failed' WHERE client_nonce = ?2",
                        rusqlite::params![next_retries as i64, nonce as i64],
                    )
                    .map_err(sql_err)?;
                } else {
                    let delay = Self::compute_backoff_delay_ms(next_retries) as i64;
                    conn.execute(
                        "UPDATE outbox_messages SET retry_count = ?1, status = 'Pending', next_retry_at_ms = ?2 WHERE client_nonce = ?3",
                        rusqlite::params![next_retries as i64, now_ms + delay, nonce as i64],
                    )
                    .map_err(sql_err)?;
                }
            }
            Ok(())
        });
    }

    /// Delivers due pending outbox messages to a live `EphemeralTestServer` with `client_nonce` idempotency.
    pub async fn flush_pending_to_server(
        &self,
        driver: &E2eSystemDriver,
        token: &str,
    ) -> Result<Vec<(u64, i64)>, String> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let due: Vec<OutboxRecord> = self
            .list_pending()
            .into_iter()
            .filter(|r| now_ms >= r.next_retry_at_ms)
            .collect();

        let mut delivered = Vec::new();
        for rec in due {
            let (status, resp) = driver
                .json_req(
                    Method::POST,
                    &format!("/api/v1/chats/{}/messages", rec.chat_id),
                    Some(token),
                    Some(json!({
                        "text": rec.text,
                        "client_nonce": rec.client_nonce
                    })),
                )
                .await
                .map_err(|e| e.to_string())?;
            if (200..300).contains(&status) {
                let msg_id = resp["message"]["message_id"]
                    .as_i64()
                    .or_else(|| resp["message_id"].as_i64())
                    .unwrap_or(0);
                self.pool
                    .with_writer(|conn| {
                        conn.execute(
                            "UPDATE outbox_messages SET status = ?1 WHERE client_nonce = ?2",
                            rusqlite::params![OutboxStatus::Sent.as_str(), rec.client_nonce as i64],
                        )
                        .map_err(sql_err)?;
                        Ok(())
                    })
                    .map_err(|e| e.to_string())?;
                delivered.push((rec.client_nonce, msg_id));
            } else {
                self.simulate_failure(rec.client_nonce, status == 400 || status == 403);
            }
        }
        Ok(delivered)
    }

    /// Clears transient backoff timers on reconnect and flushes pending messages to `EphemeralTestServer`.
    pub async fn notify_reconnected_to_server(
        &self,
        driver: &E2eSystemDriver,
        token: &str,
    ) -> Result<Vec<(u64, i64)>, String> {
        self.pool
            .with_writer(|conn| {
                conn.execute(
                    "UPDATE outbox_messages SET next_retry_at_ms = 0 WHERE status IN ('Pending', 'Sending')",
                    [],
                )
                .map_err(sql_err)?;
                Ok(())
            })
            .map_err(|e| e.to_string())?;
        self.flush_pending_to_server(driver, token).await
    }
}

// ============================================================================
// Local TCP Proxy Server (Real HTTP CONNECT & SOCKS5 Listeners for F9)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyType {
    Http,
    Socks5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyConfig {
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub struct LocalTestProxyServer {
    addr: SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<()>>,
}

impl LocalTestProxyServer {
    pub async fn start_http(expected_auth: Option<(String, String)>) -> anyhow::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let (tx, mut rx) = oneshot::channel::<()>();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut rx => break,
                    res = listener.accept() => {
                        let Ok((stream, _)) = res else { break };
                        let auth = expected_auth.clone();
                        tokio::spawn(async move {
                            let _ = handle_http_connect(stream, auth).await;
                        });
                    }
                }
            }
        });
        Ok(Self {
            addr,
            shutdown_tx: Some(tx),
            task: Some(task),
        })
    }

    pub async fn start_socks5(expected_auth: Option<(String, String)>) -> anyhow::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let (tx, mut rx) = oneshot::channel::<()>();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut rx => break,
                    res = listener.accept() => {
                        let Ok((stream, _)) = res else { break };
                        let auth = expected_auth.clone();
                        tokio::spawn(async move {
                            let _ = handle_socks5(stream, auth).await;
                        });
                    }
                }
            }
        });
        Ok(Self {
            addr,
            shutdown_tx: Some(tx),
            task: Some(task),
        })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    pub async fn stop(self) {
        drop(self);
    }
}

impl Drop for LocalTestProxyServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

async fn handle_http_connect(
    mut client: TcpStream,
    expected_auth: Option<(String, String)>,
) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(2048);
    let mut tmp = [0u8; 512];
    loop {
        let n = client.read(&mut tmp).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") || buf.len() > 16384 {
            break;
        }
    }
    let header_str = String::from_utf8_lossy(&buf);
    let mut lines = header_str.split("\r\n");
    let req_line = lines.next().unwrap_or("");
    let mut parts = req_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("");
    if !method.eq_ignore_ascii_case("CONNECT") || target.is_empty() {
        client
            .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n")
            .await?;
        return Ok(());
    }

    if let Some((exp_u, exp_p)) = expected_auth {
        let expected_b64 = base64_encode(format!("{exp_u}:{exp_p}").as_bytes());
        let expected_hdr = format!("Basic {expected_b64}");
        let authorized = lines.any(|l| {
            if let Some((k, v)) = l.split_once(':') {
                k.trim().eq_ignore_ascii_case("proxy-authorization") && v.trim() == expected_hdr
            } else {
                false
            }
        });
        if !authorized {
            client
                .write_all(
                    b"HTTP/1.1 407 Proxy Authentication Required\r\nProxy-Authenticate: Basic realm=\"test\"\r\nContent-Length: 0\r\n\r\n",
                )
                .await?;
            return Ok(());
        }
    }

    let mut upstream = TcpStream::connect(target).await?;
    client
        .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await?;
    let _ = tokio::io::copy_bidirectional(&mut client, &mut upstream).await;
    Ok(())
}

async fn handle_socks5(
    mut client: TcpStream,
    expected_auth: Option<(String, String)>,
) -> anyhow::Result<()> {
    let mut ver_n = [0u8; 2];
    client.read_exact(&mut ver_n).await?;
    if ver_n[0] != 0x05 {
        return Ok(());
    }
    let mut methods = vec![0u8; ver_n[1] as usize];
    client.read_exact(&mut methods).await?;

    if let Some((exp_u, exp_p)) = expected_auth {
        if !methods.contains(&0x02) {
            client.write_all(&[0x05, 0xFF]).await?;
            return Ok(());
        }
        client.write_all(&[0x05, 0x02]).await?;
        let mut hdr = [0u8; 2];
        client.read_exact(&mut hdr).await?;
        let mut u_buf = vec![0u8; hdr[1] as usize];
        client.read_exact(&mut u_buf).await?;
        let mut plen = [0u8; 1];
        client.read_exact(&mut plen).await?;
        let mut p_buf = vec![0u8; plen[0] as usize];
        client.read_exact(&mut p_buf).await?;

        let u_str = String::from_utf8_lossy(&u_buf);
        let p_str = String::from_utf8_lossy(&p_buf);
        if u_str != exp_u || p_str != exp_p {
            client.write_all(&[0x01, 0x01]).await?;
            return Ok(());
        }
        client.write_all(&[0x01, 0x00]).await?;
    } else {
        if !methods.contains(&0x00) {
            client.write_all(&[0x05, 0xFF]).await?;
            return Ok(());
        }
        client.write_all(&[0x05, 0x00]).await?;
    }

    let mut req_hdr = [0u8; 4];
    client.read_exact(&mut req_hdr).await?;
    if req_hdr[0] != 0x05 || req_hdr[1] != 0x01 {
        client
            .write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
            .await?;
        return Ok(());
    }

    let target_addr: String = match req_hdr[3] {
        0x01 => {
            let mut ip = [0u8; 4];
            client.read_exact(&mut ip).await?;
            let mut port_b = [0u8; 2];
            client.read_exact(&mut port_b).await?;
            let port = u16::from_be_bytes(port_b);
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(ip), port)).to_string()
        }
        0x03 => {
            let mut len_b = [0u8; 1];
            client.read_exact(&mut len_b).await?;
            let mut domain = vec![0u8; len_b[0] as usize];
            client.read_exact(&mut domain).await?;
            let mut port_b = [0u8; 2];
            client.read_exact(&mut port_b).await?;
            let port = u16::from_be_bytes(port_b);
            format!("{}:{}", String::from_utf8_lossy(&domain), port)
        }
        _ => {
            client
                .write_all(&[0x05, 0x08, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
                .await?;
            return Ok(());
        }
    };

    let mut upstream = TcpStream::connect(&target_addr).await?;
    client
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;
    let _ = tokio::io::copy_bidirectional(&mut client, &mut upstream).await;
    Ok(())
}

impl ProxyConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("Proxy host cannot be empty".into());
        }
        if self.port == 0 {
            return Err("Proxy port cannot be 0".into());
        }
        Ok(())
    }

    pub async fn tunnel_http_get(
        &self,
        target_addr: SocketAddr,
        path: &str,
    ) -> Result<(u16, String), String> {
        self.tunnel_http_request(target_addr, "GET", path, None, None, None)
            .await
    }

    pub async fn tunnel_http_request(
        &self,
        target_addr: SocketAddr,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Option<&[u8]>,
        content_type: Option<&str>,
    ) -> Result<(u16, String), String> {
        self.validate()?;
        let proxy_addr = format!("{}:{}", self.host, self.port);
        let mut stream = tokio::time::timeout(Duration::from_secs(3), TcpStream::connect(&proxy_addr))
            .await
            .map_err(|_| format!("Connection to proxy {proxy_addr} timed out"))?
            .map_err(|e| format!("Failed to connect to proxy {proxy_addr}: {e}"))?;

        match self.proxy_type {
            ProxyType::Http => {
                let mut req = format!("CONNECT {target_addr} HTTP/1.1\r\nHost: {target_addr}\r\n");
                if let (Some(u), Some(p)) = (&self.username, &self.password) {
                    let b64 = base64_encode(format!("{u}:{p}").as_bytes());
                    req.push_str(&format!("Proxy-Authorization: Basic {b64}\r\n"));
                }
                req.push_str("\r\n");
                stream
                    .write_all(req.as_bytes())
                    .await
                    .map_err(|e| format!("Proxy CONNECT write failed: {e}"))?;

                let mut resp_buf = Vec::with_capacity(512);
                let mut tmp = [0u8; 256];
                loop {
                    let n = stream
                        .read(&mut tmp)
                        .await
                        .map_err(|e| format!("Proxy CONNECT read failed: {e}"))?;
                    if n == 0 {
                        break;
                    }
                    resp_buf.extend_from_slice(&tmp[..n]);
                    if resp_buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                let resp_str = String::from_utf8_lossy(&resp_buf);
                let status_line = resp_str.lines().next().unwrap_or("");
                if status_line.contains("407") {
                    return Err("HTTP 407 Proxy Authentication Required".into());
                }
                if !status_line.contains("200") {
                    return Err(format!("HTTP CONNECT rejected: {status_line}"));
                }
            }
            ProxyType::Socks5 => {
                if self.username.is_some() && self.password.is_some() {
                    stream
                        .write_all(&[0x05, 0x02, 0x00, 0x02])
                        .await
                        .map_err(|e| format!("SOCKS5 greeting write failed: {e}"))?;
                } else {
                    stream
                        .write_all(&[0x05, 0x01, 0x00])
                        .await
                        .map_err(|e| format!("SOCKS5 greeting write failed: {e}"))?;
                }
                let mut sel = [0u8; 2];
                stream
                    .read_exact(&mut sel)
                    .await
                    .map_err(|e| format!("SOCKS5 method read failed: {e}"))?;
                if sel[0] != 0x05 || sel[1] == 0xFF {
                    return Err("SOCKS5 no acceptable auth method".into());
                }
                if sel[1] == 0x02 {
                    let u = self.username.as_deref().unwrap_or("").as_bytes();
                    let p = self.password.as_deref().unwrap_or("").as_bytes();
                    let mut auth_msg = Vec::with_capacity(3 + u.len() + p.len());
                    auth_msg.push(0x01);
                    auth_msg.push(u.len() as u8);
                    auth_msg.extend_from_slice(u);
                    auth_msg.push(p.len() as u8);
                    auth_msg.extend_from_slice(p);
                    stream
                        .write_all(&auth_msg)
                        .await
                        .map_err(|e| format!("SOCKS5 auth write failed: {e}"))?;
                    let mut auth_resp = [0u8; 2];
                    stream
                        .read_exact(&mut auth_resp)
                        .await
                        .map_err(|e| format!("SOCKS5 auth read failed: {e}"))?;
                    if auth_resp[1] != 0x00 {
                        return Err("SOCKS5 authentication failed".into());
                    }
                }

                let SocketAddr::V4(v4) = target_addr else {
                    return Err("Only IPv4 targets supported in local SOCKS5 test helper".into());
                };
                let mut req = Vec::with_capacity(10);
                req.extend_from_slice(&[0x05, 0x01, 0x00, 0x01]);
                req.extend_from_slice(&v4.ip().octets());
                req.extend_from_slice(&v4.port().to_be_bytes());
                stream
                    .write_all(&req)
                    .await
                    .map_err(|e| format!("SOCKS5 CONNECT write failed: {e}"))?;

                let mut reply = [0u8; 10];
                stream
                    .read_exact(&mut reply)
                    .await
                    .map_err(|e| format!("SOCKS5 CONNECT reply failed: {e}"))?;
                if reply[1] != 0x00 {
                    return Err(format!("SOCKS5 CONNECT failed with code {}", reply[1]));
                }
            }
        }

        // Execute real HTTP/1.1 request over the established proxy tunnel
        let payload_bytes = body.unwrap_or(&[]);
        let mut http_req = format!(
            "{method} {path} HTTP/1.1\r\nHost: {target_addr}\r\nConnection: close\r\n"
        );
        if let Some(tok) = token {
            http_req.push_str(&format!("Authorization: Bearer {tok}\r\n"));
        }
        if !payload_bytes.is_empty() || method == "POST" || method == "PUT" {
            let ct = content_type.unwrap_or("application/json");
            http_req.push_str(&format!("Content-Type: {ct}\r\n"));
            http_req.push_str(&format!("Content-Length: {}\r\n", payload_bytes.len()));
        }
        http_req.push_str("\r\n");

        stream
            .write_all(http_req.as_bytes())
            .await
            .map_err(|e| format!("Tunneled HTTP header write failed: {e}"))?;
        if !payload_bytes.is_empty() {
            stream
                .write_all(payload_bytes)
                .await
                .map_err(|e| format!("Tunneled HTTP body write failed: {e}"))?;
        }
        stream
            .flush()
            .await
            .map_err(|e| format!("Tunneled stream flush failed: {e}"))?;

        let mut raw_resp = Vec::new();
        tokio::time::timeout(Duration::from_secs(5), stream.read_to_end(&mut raw_resp))
            .await
            .map_err(|_| "Tunneled HTTP response timed out".to_string())?
            .map_err(|e| format!("Tunneled HTTP response read failed: {e}"))?;

        let resp_str = String::from_utf8_lossy(&raw_resp);
        let (hdrs, body_str) = resp_str
            .split_once("\r\n\r\n")
            .ok_or_else(|| format!("Malformed HTTP response over tunnel: {resp_str}"))?;
        let status_code = hdrs
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or_else(|| format!("Missing HTTP status code in: {hdrs}"))?;
        Ok((status_code, body_str.to_string()))
    }
}

// ============================================================================
// M2 Pending E2EE Harness (Genuine 3-DH/4-DH X25519 + HKDF-SHA256 + ChaCha20-Poly1305)
// ============================================================================

#[derive(Debug, Clone)]
pub struct PrekeyBundle {
    pub user_id: u64,
    pub device_id: String,
    pub identity_key: PublicKey,
    pub signed_prekey: PublicKey,
    pub one_time_prekeys: Vec<PublicKey>,
}

pub struct E2eeSimulator {
    device_bundles: Mutex<HashMap<(u64, String), PrekeyBundle>>,
    seen_envelopes: Mutex<HashSet<(String, u32)>>,
}

impl E2eeSimulator {
    pub fn new() -> Self {
        Self {
            device_bundles: Mutex::new(HashMap::new()),
            seen_envelopes: Mutex::new(HashSet::new()),
        }
    }

    pub fn publish_prekeys(&self, bundle: PrekeyBundle) {
        self.device_bundles
            .lock()
            .insert((bundle.user_id, bundle.device_id.clone()), bundle);
    }

    pub fn get_prekey_bundle(&self, user_id: u64, device_id: &str) -> Option<PrekeyBundle> {
        self.device_bundles
            .lock()
            .get(&(user_id, device_id.to_string()))
            .cloned()
    }

    /// Retrieves a device bundle and consumes one one-time prekey (if available).
    pub fn consume_prekey_bundle(
        &self,
        user_id: u64,
        device_id: &str,
    ) -> Option<(PrekeyBundle, Option<PublicKey>)> {
        let mut lock = self.device_bundles.lock();
        let bundle = lock.get_mut(&(user_id, device_id.to_string()))?;
        let otp = if bundle.one_time_prekeys.is_empty() {
            None
        } else {
            Some(bundle.one_time_prekeys.remove(0))
        };
        Some((bundle.clone(), otp))
    }

    /// Initiator side of 3-DH / 4-DH X3DH using HKDF-SHA256.
    pub fn derive_x3dh_initiator(
        alice_identity_sec: &StaticSecret,
        alice_ephemeral_sec: &StaticSecret,
        bob_identity_pub: &PublicKey,
        bob_signed_prekey_pub: &PublicKey,
        bob_one_time_pub: Option<&PublicKey>,
    ) -> [u8; 32] {
        let dh1 = alice_identity_sec.diffie_hellman(bob_signed_prekey_pub);
        let dh2 = alice_ephemeral_sec.diffie_hellman(bob_identity_pub);
        let dh3 = alice_ephemeral_sec.diffie_hellman(bob_signed_prekey_pub);
        let mut ikm = Vec::with_capacity(128);
        ikm.extend_from_slice(&[0xFFu8; 32]);
        ikm.extend_from_slice(dh1.as_bytes());
        ikm.extend_from_slice(dh2.as_bytes());
        ikm.extend_from_slice(dh3.as_bytes());
        if let Some(otp) = bob_one_time_pub {
            let dh4 = alice_ephemeral_sec.diffie_hellman(otp);
            ikm.extend_from_slice(dh4.as_bytes());
        }
        let hk = Hkdf::<Sha256>::new(None, &ikm);
        let mut okm = [0u8; 32];
        hk.expand(b"gpui-telegram-x3dh-v1", &mut okm)
            .expect("valid hkdf length");
        okm
    }

    /// Responder side of 3-DH / 4-DH X3DH using HKDF-SHA256.
    pub fn derive_x3dh_responder(
        bob_identity_sec: &StaticSecret,
        bob_signed_prekey_sec: &StaticSecret,
        bob_one_time_sec: Option<&StaticSecret>,
        alice_identity_pub: &PublicKey,
        alice_ephemeral_pub: &PublicKey,
    ) -> [u8; 32] {
        let dh1 = bob_signed_prekey_sec.diffie_hellman(alice_identity_pub);
        let dh2 = bob_identity_sec.diffie_hellman(alice_ephemeral_pub);
        let dh3 = bob_signed_prekey_sec.diffie_hellman(alice_ephemeral_pub);
        let mut ikm = Vec::with_capacity(128);
        ikm.extend_from_slice(&[0xFFu8; 32]);
        ikm.extend_from_slice(dh1.as_bytes());
        ikm.extend_from_slice(dh2.as_bytes());
        ikm.extend_from_slice(dh3.as_bytes());
        if let Some(otp_sec) = bob_one_time_sec {
            let dh4 = otp_sec.diffie_hellman(alice_ephemeral_pub);
            ikm.extend_from_slice(dh4.as_bytes());
        }
        let hk = Hkdf::<Sha256>::new(None, &ikm);
        let mut okm = [0u8; 32];
        hk.expand(b"gpui-telegram-x3dh-v1", &mut okm)
            .expect("valid hkdf length");
        okm
    }

    pub fn encrypt_message(
        key: &[u8; 32],
        seq: u32,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, String> {
        let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|e| e.to_string())?;
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[8..12].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);
        cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption error: {e}"))
    }

    pub fn decrypt_message(
        &self,
        sender_device_id: &str,
        key: &[u8; 32],
        seq: u32,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, String> {
        {
            let mut seen = self.seen_envelopes.lock();
            if !seen.insert((sender_device_id.to_string(), seq)) {
                return Err("Replay attack detected: duplicate sequence number".into());
            }
        }
        let cipher = ChaCha20Poly1305::new_from_slice(key).map_err(|e| e.to_string())?;
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[8..12].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption authentication failed: {e}"))
    }
}
