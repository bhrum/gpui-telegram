use crate::presets::ProtocolKind;
use base64::engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine as _;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

/// Local AI CLI / desktop subscription families supported by Magpie discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionKind {
    CodexChatGpt,
    ClaudeCode,
    GitHubCopilot,
    GeminiCodeAssist,
    Antigravity,
    GrokSuperGrok,
}

impl SubscriptionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CodexChatGpt => "codex_chatgpt",
            Self::ClaudeCode => "claude_code",
            Self::GitHubCopilot => "github_copilot",
            Self::GeminiCodeAssist => "gemini_code_assist",
            Self::Antigravity => "antigravity",
            Self::GrokSuperGrok => "grok_supergrok",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::CodexChatGpt => "Codex / ChatGPT Plus & Pro",
            Self::ClaudeCode => "Claude Code Subscription",
            Self::GitHubCopilot => "GitHub Copilot",
            Self::GeminiCodeAssist => "Gemini Code Assist",
            Self::Antigravity => "Antigravity OAuth",
            Self::GrokSuperGrok => "Grok / SuperGrok",
        }
    }
}

/// Discovered local subscription credential summary (read-only metadata; secrets omitted from JSON).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionAccount {
    pub id: String,
    pub kind: SubscriptionKind,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    pub protocol: ProtocolKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    pub source_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at_ms: Option<i64>,
    #[serde(skip_serializing, default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

/// Snapshot of local subscription accounts discovered on this host.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionRegistry {
    #[serde(default)]
    pub accounts: Vec<SubscriptionAccount>,
    #[serde(default)]
    pub scanned_at_ms: i64,
}

impl SubscriptionRegistry {
    pub fn has_active_subscription(&self) -> bool {
        self.accounts.iter().any(|a| a.active)
    }

    pub fn active_accounts(&self) -> Vec<&SubscriptionAccount> {
        self.accounts.iter().filter(|a| a.active).collect()
    }

    pub fn find(&self, id: &str) -> Option<&SubscriptionAccount> {
        self.accounts.iter().find(|a| a.id == id)
    }
}

/// Read-only scan of standard local CLI subscription files under `home`.
/// Does not invoke any external process or perform network requests.
pub fn discover_subscriptions_in_home(
    home: &Path,
    codex_home_override: Option<&Path>,
) -> SubscriptionRegistry {
    let now_ms = Utc::now().timestamp_millis();
    let mut accounts = Vec::new();

    let codex_dir = codex_home_override
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".codex"));
    if let Some(codex_acc) = scan_codex_dir(&codex_dir, now_ms) {
        accounts.push(codex_acc);
    }

    scan_claude_dirs(home, now_ms, &mut accounts);
    scan_copilot_dir(&home.join(".config").join("github-copilot"), &mut accounts);
    if let Some(gemini_acc) = scan_gemini_dir(&home.join(".gemini"), now_ms) {
        accounts.push(gemini_acc);
    }
    if let Some(agy_acc) = scan_antigravity_dir(&home.join(".antigravity")) {
        accounts.push(agy_acc);
    }
    if let Some(grok_acc) = scan_grok_dir(&home.join(".grok")) {
        accounts.push(grok_acc);
    }

    SubscriptionRegistry {
        accounts,
        scanned_at_ms: now_ms,
    }
}

fn non_empty_str(val: Option<&Value>) -> Option<String> {
    val.and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
}

fn decode_jwt_payload(jwt: &str) -> Option<Value> {
    let mut parts = jwt.trim().split('.');
    let _hdr = parts.next()?;
    let payload_b64 = parts.next()?;
    let _sig = parts.next()?;
    let decoded = URL_SAFE_NO_PAD
        .decode(payload_b64.as_bytes())
        .or_else(|_| URL_SAFE.decode(payload_b64.as_bytes()))
        .ok()?;
    serde_json::from_slice::<Value>(&decoded).ok()
}

fn scan_codex_dir(dir: &Path, now_ms: i64) -> Option<SubscriptionAccount> {
    let auth_path = dir.join("auth.json");
    let config_path = dir.join("config.toml");
    let auth_raw = std::fs::read_to_string(&auth_path).ok();
    let cfg_raw = std::fs::read_to_string(&config_path).ok();
    if auth_raw.is_none() && cfg_raw.is_none() {
        return None;
    }

    let parsed_cfg = cfg_raw
        .as_deref()
        .map(parse_codex_config_summary)
        .unwrap_or_default();

    let mut access_token = None;
    let mut account_id = None;
    let mut email = None;
    let mut plan = None;
    let mut expires_at_ms = None;

    if let Some(raw) = &auth_raw {
        if let Ok(val) = serde_json::from_str::<Value>(raw) {
            let tokens = val.get("tokens");
            access_token = non_empty_str(tokens.and_then(|t| t.get("access_token")))
                .or_else(|| non_empty_str(val.get("OPENAI_API_KEY")));
            let id_token = non_empty_str(tokens.and_then(|t| t.get("id_token")));
            account_id = non_empty_str(tokens.and_then(|t| t.get("account_id")))
                .or_else(|| non_empty_str(val.get("account_id")));
            email = non_empty_str(val.get("email"))
                .or_else(|| non_empty_str(tokens.and_then(|t| t.get("email"))));

            for cand in [access_token.as_deref(), id_token.as_deref()]
                .into_iter()
                .flatten()
            {
                if let Some(claims) = decode_jwt_payload(cand) {
                    if email.is_none() {
                        email = non_empty_str(claims.get("email"));
                    }
                    if expires_at_ms.is_none() {
                        expires_at_ms = claims
                            .get("exp")
                            .and_then(Value::as_i64)
                            .map(|sec| sec.saturating_mul(1_000));
                    }
                    if let Some(auth_obj) = claims.get("https://api.openai.com/auth") {
                        if account_id.is_none() {
                            account_id = non_empty_str(auth_obj.get("chatgpt_account_id"));
                        }
                        if plan.is_none() {
                            plan = non_empty_str(auth_obj.get("chatgpt_plan_type"));
                        }
                    }
                }
            }
        }
    }

    let has_token = access_token.is_some();
    let expired_no_refresh = expires_at_ms.is_some_and(|exp| exp <= now_ms)
        && auth_raw
            .as_deref()
            .and_then(|r| serde_json::from_str::<Value>(r).ok())
            .and_then(|v| {
                non_empty_str(v.get("tokens").and_then(|t| t.get("refresh_token")))
            })
            .is_none();
    let has_local_magpie = parsed_cfg
        .magpie_base_url
        .as_deref()
        .is_some_and(|u| !u.is_empty())
        || parsed_cfg
            .openai_base_url
            .as_deref()
            .is_some_and(|u| u.contains("127.0.0.1") || u.contains("localhost"));

    if !has_token && !has_local_magpie {
        return None;
    }

    let base_url = parsed_cfg
        .openai_base_url
        .or(parsed_cfg.magpie_base_url)
        .or_else(|| Some("https://chatgpt.com/backend-api/codex".to_string()));
    let mut headers = parsed_cfg.magpie_headers;
    headers.insert("originator".to_string(), "codex_cli_rs".to_string());
    if let Some(ref acct) = account_id {
        headers.insert("ChatGPT-Account-ID".to_string(), acct.clone());
    }

    let active = (has_token && !expired_no_refresh) || has_local_magpie;
    let id_suffix = email
        .as_deref()
        .or(account_id.as_deref())
        .unwrap_or("default");
    let label = match (&email, &plan) {
        (Some(em), Some(pl)) => format!("Codex ({em} · {pl})"),
        (Some(em), None) => format!("Codex ({em})"),
        (None, Some(pl)) => format!("Codex ({pl})"),
        (None, None) => "Codex ChatGPT".to_string(),
    };

    Some(SubscriptionAccount {
        id: format!("codex:{id_suffix}"),
        kind: SubscriptionKind::CodexChatGpt,
        label,
        email,
        account_id,
        plan,
        active,
        base_url,
        protocol: ProtocolKind::CodexResponses,
        default_model: parsed_cfg.model.or_else(|| Some("codex-mini-latest".to_string())),
        source_path: if auth_raw.is_some() {
            auth_path.display().to_string()
        } else {
            config_path.display().to_string()
        },
        expires_at_ms,
        access_token,
        headers,
    })
}

fn scan_claude_dirs(home: &Path, now_ms: i64, out: &mut Vec<SubscriptionAccount>) {
    let mut dirs = Vec::new();
    let default_claude = home.join(".claude");
    if default_claude.is_dir() {
        dirs.push(("default".to_string(), default_claude));
    }
    if let Ok(entries) = std::fs::read_dir(home) {
        let mut extras: Vec<(String, PathBuf)> = entries
            .filter_map(Result::ok)
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                let path = e.path();
                if path.is_dir() && name.starts_with(".claude") && name != ".claude" {
                    let suffix = name
                        .strip_prefix(".claude")
                        .unwrap_or("")
                        .trim_matches(|c| c == '-' || c == '_');
                    if !suffix.is_empty() && !suffix.starts_with('.') && suffix != "launch" {
                        return Some((suffix.to_string(), path));
                    }
                }
                None
            })
            .collect();
        extras.sort_by(|a, b| a.0.cmp(&b.0));
        dirs.extend(extras);
    }

    for (profile_name, dir) in dirs {
        let cred_path = dir.join(".credentials.json");
        let Ok(raw) = std::fs::read_to_string(&cred_path) else {
            continue;
        };
        let Ok(val) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };
        let oauth = val.get("claudeAiOauth").unwrap_or(&val);
        let access_token = non_empty_str(oauth.get("accessToken"))
            .or_else(|| non_empty_str(val.get("accessToken")));
        let refresh_token = non_empty_str(oauth.get("refreshToken"));
        if access_token.is_none() && refresh_token.is_none() {
            continue;
        }
        let expires_at_ms = oauth.get("expiresAt").and_then(Value::as_i64);
        let plan = non_empty_str(oauth.get("subscriptionType"))
            .or_else(|| non_empty_str(oauth.get("rateLimitTier")));
        let email = non_empty_str(val.get("oauthAccount").and_then(|o| o.get("emailAddress")))
            .or_else(|| non_empty_str(oauth.get("email")));
        let active = access_token.is_some()
            && (expires_at_ms.is_none_or(|exp| exp > now_ms) || refresh_token.is_some());

        let label = match (&profile_name, &email, &plan) {
            (p, Some(em), _) if p == "default" => format!("Claude Code ({em})"),
            (p, _, Some(pl)) if p == "default" => format!("Claude Code ({pl})"),
            (p, _, _) if p == "default" => "Claude Code".to_string(),
            (p, _, _) => format!("Claude Code · {p}"),
        };

        out.push(SubscriptionAccount {
            id: format!("claude_code:{profile_name}"),
            kind: SubscriptionKind::ClaudeCode,
            label,
            email,
            account_id: None,
            plan,
            active,
            base_url: Some("https://api.anthropic.com/v1".to_string()),
            protocol: ProtocolKind::AnthropicMessages,
            default_model: Some("claude-sonnet-4-20250514".to_string()),
            source_path: cred_path.display().to_string(),
            expires_at_ms,
            access_token,
            headers: BTreeMap::new(),
        });
    }
}

fn scan_copilot_dir(dir: &Path, out: &mut Vec<SubscriptionAccount>) {
    let mut seen_users = HashMap::<String, usize>::new();
    for filename in ["hosts.json", "apps.json"] {
        let path = dir.join(filename);
        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };
        for (host_key, entry) in map {
            let Some(token) = non_empty_str(entry.get("oauth_token")) else {
                continue;
            };
            let user = non_empty_str(entry.get("user")).unwrap_or_else(|| host_key.clone());
            if seen_users.contains_key(&user) {
                continue;
            }
            seen_users.insert(user.clone(), out.len());
            out.push(SubscriptionAccount {
                id: format!("github_copilot:{user}"),
                kind: SubscriptionKind::GitHubCopilot,
                label: format!("GitHub Copilot ({user})"),
                email: None,
                account_id: Some(user),
                plan: Some("copilot".to_string()),
                active: true,
                base_url: Some("https://api.githubcopilot.com".to_string()),
                protocol: ProtocolKind::ChatCompletions,
                default_model: Some("gpt-4o".to_string()),
                source_path: path.display().to_string(),
                expires_at_ms: None,
                access_token: Some(token),
                headers: BTreeMap::new(),
            });
        }
    }
}

fn scan_gemini_dir(dir: &Path, now_ms: i64) -> Option<SubscriptionAccount> {
    let cred_path = dir.join("oauth_creds.json");
    let raw = std::fs::read_to_string(&cred_path).ok()?;
    let val = serde_json::from_str::<Value>(&raw).ok()?;
    let access_token = non_empty_str(val.get("access_token"));
    let refresh_token = non_empty_str(val.get("refresh_token"));
    if access_token.is_none() && refresh_token.is_none() {
        return None;
    }
    let expires_at_ms = val.get("expiry_date").and_then(Value::as_i64);
    let mut email = non_empty_str(val.get("email"));
    if email.is_none() {
        if let Some(idt) = non_empty_str(val.get("id_token")) {
            if let Some(claims) = decode_jwt_payload(&idt) {
                email = non_empty_str(claims.get("email"));
            }
        }
    }
    let active = (access_token.is_some() && expires_at_ms.is_none_or(|exp| exp > now_ms))
        || refresh_token.is_some();
    let label = match &email {
        Some(em) => format!("Gemini Code Assist ({em})"),
        None => "Gemini Code Assist".to_string(),
    };
    Some(SubscriptionAccount {
        id: format!(
            "gemini_code_assist:{}",
            email.as_deref().unwrap_or("default")
        ),
        kind: SubscriptionKind::GeminiCodeAssist,
        label,
        email,
        account_id: None,
        plan: Some("code_assist".to_string()),
        active,
        base_url: Some("https://cloudcode-pa.googleapis.com/v1internal".to_string()),
        protocol: ProtocolKind::GeminiGenerateContent,
        default_model: Some("gemini-2.5-pro".to_string()),
        source_path: cred_path.display().to_string(),
        expires_at_ms,
        access_token,
        headers: BTreeMap::new(),
    })
}

fn scan_antigravity_dir(dir: &Path) -> Option<SubscriptionAccount> {
    let path = dir.join("accounts.json");
    let raw = std::fs::read_to_string(&path).ok()?;
    let val = serde_json::from_str::<Value>(&raw).ok()?;
    let arr = val
        .get("accounts")
        .and_then(Value::as_array)
        .or_else(|| val.as_array())?;
    let first = arr.first()?;
    let token = non_empty_str(first.get("refresh_token"))
        .or_else(|| non_empty_str(first.get("token").and_then(|t| t.get("refresh_token"))));
    let email = non_empty_str(first.get("email"));
    if token.is_none() && email.is_none() {
        return None;
    }
    Some(SubscriptionAccount {
        id: format!("antigravity:{}", email.as_deref().unwrap_or("default")),
        kind: SubscriptionKind::Antigravity,
        label: email
            .as_ref()
            .map(|e| format!("Antigravity ({e})"))
            .unwrap_or_else(|| "Antigravity".to_string()),
        email,
        account_id: None,
        plan: non_empty_str(first.get("plan")),
        active: token.is_some(),
        base_url: Some("https://cloudcode-pa.googleapis.com/v1internal".to_string()),
        protocol: ProtocolKind::GeminiGenerateContent,
        default_model: Some("gemini-3.1-pro-preview".to_string()),
        source_path: path.display().to_string(),
        expires_at_ms: None,
        access_token: token,
        headers: BTreeMap::new(),
    })
}

fn scan_grok_dir(dir: &Path) -> Option<SubscriptionAccount> {
    let path = dir.join("session.json");
    let raw = std::fs::read_to_string(&path).ok()?;
    let val = serde_json::from_str::<Value>(&raw).ok()?;
    let token = non_empty_str(val.get("token"))
        .or_else(|| non_empty_str(val.get("sso")))?;
    let email = non_empty_str(val.get("email"));
    Some(SubscriptionAccount {
        id: format!("grok_supergrok:{}", email.as_deref().unwrap_or("default")),
        kind: SubscriptionKind::GrokSuperGrok,
        label: email
            .as_ref()
            .map(|e| format!("SuperGrok ({e})"))
            .unwrap_or_else(|| "SuperGrok".to_string()),
        email,
        account_id: None,
        plan: non_empty_str(val.get("plan")).or_else(|| Some("supergrok".to_string())),
        active: true,
        base_url: Some("https://api.x.ai/v1".to_string()),
        protocol: ProtocolKind::ChatCompletions,
        default_model: Some("grok-3-beta".to_string()),
        source_path: path.display().to_string(),
        expires_at_ms: None,
        access_token: Some(token),
        headers: BTreeMap::new(),
    })
}

#[derive(Default)]
struct CodexConfigSummary {
    model: Option<String>,
    openai_base_url: Option<String>,
    magpie_base_url: Option<String>,
    magpie_headers: BTreeMap<String, String>,
}

fn parse_codex_config_summary(raw: &str) -> CodexConfigSummary {
    let mut summary = CodexConfigSummary::default();
    let mut section = String::new();
    for raw_line in raw.lines() {
        let no_comment = raw_line.split('#').next().unwrap_or("").trim();
        if no_comment.is_empty() {
            continue;
        }
        if no_comment.starts_with('[') && no_comment.ends_with(']') {
            section = no_comment[1..no_comment.len() - 1]
                .trim()
                .to_ascii_lowercase();
            continue;
        }
        let Some((k, v)) = no_comment.split_once('=') else {
            continue;
        };
        let key = k.trim().trim_matches('"').trim_matches('\'');
        let val = v.trim().trim_matches('"').trim_matches('\'');
        if val.is_empty() {
            continue;
        }
        if section.is_empty() {
            if key == "model" {
                summary.model = Some(val.to_string());
            } else if key == "openai_base_url" {
                summary.openai_base_url = Some(val.to_string());
            }
        } else if section == "model_providers.magpie" && key == "base_url" {
            summary.magpie_base_url = Some(val.to_string());
        } else if section == "model_providers.magpie.http_headers" {
            summary
                .magpie_headers
                .insert(key.to_string(), val.to_string());
        }
    }
    summary
}
