use crate::client::TelegramClient;
use crate::net::http_client::{HttpClient, HttpClientError, SearchResultItemDto};
use crate::net::http_helpers::percent_encode_rfc3986;
use gpui_telegram_proto::contracts::SearchHitDto;
use gpui_telegram_proto::miniapp_contracts::{
    BotRegistryEntry, BotTokenResponse, ChatCallbackRequest, CloudStorageDeleteRequest,
    CloudStorageItem, CloudStorageSetRequest, CreateBotRequest, CreateMiniAppRequest,
    InstallMiniAppRequest, LaunchMiniAppRequest, MiniAppActionRequest, MiniAppActionResponse,
    MiniAppEntry, MiniAppLaunchContext, MiniAppLaunchMode, NaturalLanguageCreateRequest,
    NaturalLanguageCreateResponse, PeerBotActionResult, ValidateInitDataRequest,
    ValidateInitDataResponse,
};
use hmac::{Hmac, Mac};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GlobalCatalogSearchResponse {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub total: usize,
    #[serde(default)]
    pub hits: Vec<SearchHitDto>,
    #[serde(default)]
    pub results: Vec<SearchResultItemDto>,
    #[serde(default)]
    pub apps: Vec<MiniAppEntry>,
    #[serde(default)]
    pub bots: Vec<BotRegistryEntry>,
}

fn clean_bot_username(raw: &str) -> String {
    raw.trim().trim_start_matches('@').to_string()
}

fn miniapp_base_path(bot_username: &str, short_name: &str) -> String {
    let clean_bot = clean_bot_username(bot_username);
    format!(
        "/api/v1/miniapps/{}/{}",
        percent_encode_rfc3986(&clean_bot),
        percent_encode_rfc3986(short_name.trim())
    )
}

fn percent_decode_component(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
        } else if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(val) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(val);
                i += 3;
                continue;
            }
            out.push(bytes[i]);
            i += 1;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

/// Computes the exact Telegram Mini App HMAC-SHA256 signature:
/// `secret_key = HMAC_SHA256("WebAppData", bot_token)`
/// `hash = hex(HMAC_SHA256(secret_key, data_check_string))`
pub fn compute_telegram_init_data_hash(data_check_string: &str, bot_token: &str) -> String {
    let mut secret_mac =
        HmacSha256::new_from_slice(b"WebAppData").expect("HMAC accepts arbitrary key length");
    secret_mac.update(bot_token.as_bytes());
    let secret_key = secret_mac.finalize().into_bytes();

    let mut data_mac =
        HmacSha256::new_from_slice(&secret_key).expect("HMAC accepts 32-byte secret key");
    data_mac.update(data_check_string.as_bytes());
    to_hex_lower(&data_mac.finalize().into_bytes())
}

/// Builds Telegram's `data_check_string` (sorted `key=value` lines excluding `hash`) and extracts `hash`.
pub fn parse_init_data_check_string(raw_query_string: &str) -> Result<(String, String), String> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut provided_hash: Option<String> = None;
    for part in raw_query_string.trim().trim_start_matches('?').split('&') {
        if part.is_empty() {
            continue;
        }
        let (raw_k, raw_v) = match part.split_once('=') {
            Some((k, v)) => (k, v),
            None => (part, ""),
        };
        let key = percent_decode_component(raw_k);
        let val = percent_decode_component(raw_v);
        if key == "hash" {
            provided_hash = Some(val);
        } else {
            pairs.push((key, val));
        }
    }
    let hash = provided_hash
        .filter(|h| !h.is_empty())
        .ok_or_else(|| "Missing 'hash' parameter in initData".to_string())?;
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let data_check_string = pairs
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n");
    Ok((data_check_string, hash))
}

/// Verifies a Telegram Mini App `initData` query string against `bot_token`.
pub fn verify_telegram_init_data_hmac(
    raw_query_string: &str,
    bot_token: &str,
) -> Result<bool, String> {
    let (data_check_string, provided_hash) = parse_init_data_check_string(raw_query_string)?;
    let expected = compute_telegram_init_data_hash(&data_check_string, bot_token);
    let provided_lower = provided_hash.to_ascii_lowercase();
    if expected.len() != provided_lower.len() {
        return Ok(false);
    }
    let diff = expected
        .bytes()
        .zip(provided_lower.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b));
    Ok(diff == 0)
}

impl HttpClient {
    pub async fn search_global_catalog(
        &self,
        token: &str,
        query: &str,
        category: Option<&str>,
    ) -> Result<GlobalCatalogSearchResponse, HttpClientError> {
        let mut path = format!("/api/v1/search?q={}", percent_encode_rfc3986(query));
        if let Some(cat) = category.filter(|c| !c.trim().is_empty()) {
            path.push_str("&category=");
            path.push_str(&percent_encode_rfc3986(cat.trim()));
        }
        self.request_json(Method::GET, &path, Some(token), None::<&()>)
            .await
    }

    pub async fn list_mini_apps(
        &self,
        token: &str,
        query: Option<&str>,
        category: Option<&str>,
    ) -> Result<Vec<MiniAppEntry>, HttpClientError> {
        let mut params = Vec::new();
        if let Some(q) = query.map(str::trim).filter(|s| !s.is_empty()) {
            params.push(format!("q={}", percent_encode_rfc3986(q)));
        }
        if let Some(cat) = category.map(str::trim).filter(|s| !s.is_empty()) {
            params.push(format!("category={}", percent_encode_rfc3986(cat)));
        }
        let path = if params.is_empty() {
            "/api/v1/miniapps".to_string()
        } else {
            format!("/api/v1/miniapps?{}", params.join("&"))
        };
        self.request_vec_or_field(
            Method::GET,
            &path,
            Some(token),
            &["apps", "miniapps", "items"],
        )
        .await
    }

    pub async fn list_installed_mini_apps(
        &self,
        token: &str,
    ) -> Result<Vec<MiniAppEntry>, HttpClientError> {
        self.request_vec_or_field(
            Method::GET,
            "/api/v1/miniapps/installed",
            Some(token),
            &["apps", "miniapps", "items"],
        )
        .await
    }

    pub async fn list_attach_menu_bots(
        &self,
        token: &str,
    ) -> Result<Vec<BotRegistryEntry>, HttpClientError> {
        self.request_vec_or_field(
            Method::GET,
            "/api/v1/bots/attach-menu",
            Some(token),
            &["bots", "items"],
        )
        .await
    }

    pub async fn list_my_bots(&self, token: &str) -> Result<Vec<BotRegistryEntry>, HttpClientError> {
        self.request_vec_or_field(
            Method::GET,
            "/api/v1/bots/my",
            Some(token),
            &["bots", "items"],
        )
        .await
    }

    pub async fn create_bot(
        &self,
        token: &str,
        req: &CreateBotRequest,
    ) -> Result<BotTokenResponse, HttpClientError> {
        self.request_json(Method::POST, "/api/v1/bots", Some(token), Some(req))
            .await
    }

    pub async fn create_mini_app(
        &self,
        token: &str,
        req: &CreateMiniAppRequest,
    ) -> Result<MiniAppEntry, HttpClientError> {
        self.request_json(Method::POST, "/api/v1/miniapps", Some(token), Some(req))
            .await
    }

    pub async fn create_from_natural_language(
        &self,
        token: &str,
        prompt: &str,
    ) -> Result<NaturalLanguageCreateResponse, HttpClientError> {
        let req = NaturalLanguageCreateRequest {
            prompt: prompt.to_string(),
        };
        self.request_json(
            Method::POST,
            "/api/v1/botfather/nl-create",
            Some(token),
            Some(&req),
        )
        .await
    }

    pub async fn install_mini_app(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        allow_write_access: bool,
    ) -> Result<PeerBotActionResult, HttpClientError> {
        let path = format!("{}/install", miniapp_base_path(bot_username, short_name));
        let body = InstallMiniAppRequest { allow_write_access };
        self.request_json(Method::POST, &path, Some(token), Some(&body))
            .await
    }

    pub async fn uninstall_mini_app(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
    ) -> Result<PeerBotActionResult, HttpClientError> {
        let path = format!("{}/install", miniapp_base_path(bot_username, short_name));
        self.request_json(Method::DELETE, &path, Some(token), None::<&()>)
            .await
    }

    pub async fn launch_mini_app(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        chat_id: Option<i64>,
        launch_mode: MiniAppLaunchMode,
        start_param: Option<&str>,
    ) -> Result<MiniAppLaunchContext, HttpClientError> {
        let path = format!("{}/launch", miniapp_base_path(bot_username, short_name));
        let body = LaunchMiniAppRequest {
            chat_id,
            launch_mode,
            start_param: start_param.map(|s| s.to_string()),
        };
        self.request_json(Method::POST, &path, Some(token), Some(&body))
            .await
    }

    pub async fn validate_mini_app_init_data(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        init_data: &str,
    ) -> Result<ValidateInitDataResponse, HttpClientError> {
        let path = format!(
            "{}/validate-init-data",
            miniapp_base_path(bot_username, short_name)
        );
        let body = ValidateInitDataRequest {
            init_data: init_data.to_string(),
        };
        self.request_json(Method::POST, &path, Some(token), Some(&body))
            .await
    }

    pub async fn send_mini_app_action(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        req: &MiniAppActionRequest,
    ) -> Result<MiniAppActionResponse, HttpClientError> {
        let path = format!("{}/action", miniapp_base_path(bot_username, short_name));
        self.request_json(Method::POST, &path, Some(token), Some(req))
            .await
    }

    pub async fn get_cloud_storage(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
    ) -> Result<Vec<CloudStorageItem>, HttpClientError> {
        let path = format!("{}/storage", miniapp_base_path(bot_username, short_name));
        self.request_vec_or_field(
            Method::GET,
            &path,
            Some(token),
            &["items", "cloud_storage", "storage"],
        )
        .await
    }

    pub async fn set_cloud_storage_item(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        key: &str,
        value: &str,
    ) -> Result<CloudStorageItem, HttpClientError> {
        let path = format!("{}/storage", miniapp_base_path(bot_username, short_name));
        let body = CloudStorageSetRequest {
            key: key.to_string(),
            value: value.to_string(),
        };
        self.request_json(Method::PUT, &path, Some(token), Some(&body))
            .await
    }

    pub async fn delete_cloud_storage_item(
        &self,
        token: &str,
        bot_username: &str,
        short_name: &str,
        key: &str,
    ) -> Result<bool, HttpClientError> {
        let path = format!(
            "{}/storage?key={}",
            miniapp_base_path(bot_username, short_name),
            percent_encode_rfc3986(key)
        );
        let body = CloudStorageDeleteRequest {
            key: key.to_string(),
        };
        let val: serde_json::Value = self
            .request_json(Method::DELETE, &path, Some(token), Some(&body))
            .await?;
        Ok(val
            .get("deleted")
            .or_else(|| val.get("ok"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true))
    }

    pub async fn send_chat_callback(
        &self,
        token: &str,
        chat_id: i64,
        msg_id: i64,
        callback_data: &str,
    ) -> Result<PeerBotActionResult, HttpClientError> {
        let path = format!("/api/v1/chats/{}/callback", chat_id);
        let body = ChatCallbackRequest {
            message_id: msg_id,
            callback_data: callback_data.to_string(),
        };
        self.request_json(Method::POST, &path, Some(token), Some(&body))
            .await
    }
}

impl TelegramClient {
    pub async fn search_global_catalog(
        &self,
        query: &str,
        category: Option<&str>,
    ) -> std::result::Result<GlobalCatalogSearchResponse, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .search_global_catalog(&token, query, category)
            .await
    }

    pub async fn list_mini_apps(
        &self,
        query: Option<&str>,
        category: Option<&str>,
    ) -> std::result::Result<Vec<MiniAppEntry>, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.list_mini_apps(&token, query, category).await
    }

    pub async fn list_installed_mini_apps(
        &self,
    ) -> std::result::Result<Vec<MiniAppEntry>, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.list_installed_mini_apps(&token).await
    }

    pub async fn list_attach_menu_bots(
        &self,
    ) -> std::result::Result<Vec<BotRegistryEntry>, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.list_attach_menu_bots(&token).await
    }

    pub async fn list_my_bots(
        &self,
    ) -> std::result::Result<Vec<BotRegistryEntry>, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.list_my_bots(&token).await
    }

    pub async fn create_bot(
        &self,
        req: &CreateBotRequest,
    ) -> std::result::Result<BotTokenResponse, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.create_bot(&token, req).await
    }

    pub async fn create_mini_app(
        &self,
        req: &CreateMiniAppRequest,
    ) -> std::result::Result<MiniAppEntry, HttpClientError> {
        let token = self.require_token()?;
        self.http_client.create_mini_app(&token, req).await
    }

    pub async fn create_from_natural_language(
        &self,
        prompt: &str,
    ) -> std::result::Result<NaturalLanguageCreateResponse, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .create_from_natural_language(&token, prompt)
            .await
    }

    pub async fn install_mini_app(
        &self,
        bot_username: &str,
        short_name: &str,
        allow_write_access: bool,
    ) -> std::result::Result<PeerBotActionResult, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .install_mini_app(&token, bot_username, short_name, allow_write_access)
            .await
    }

    pub async fn uninstall_mini_app(
        &self,
        bot_username: &str,
        short_name: &str,
    ) -> std::result::Result<PeerBotActionResult, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .uninstall_mini_app(&token, bot_username, short_name)
            .await
    }

    pub async fn launch_mini_app(
        &self,
        bot_username: &str,
        short_name: &str,
        chat_id: Option<i64>,
        launch_mode: MiniAppLaunchMode,
        start_param: Option<&str>,
    ) -> std::result::Result<MiniAppLaunchContext, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .launch_mini_app(&token, bot_username, short_name, chat_id, launch_mode, start_param)
            .await
    }

    pub async fn validate_mini_app_init_data(
        &self,
        bot_username: &str,
        short_name: &str,
        init_data: &str,
    ) -> std::result::Result<ValidateInitDataResponse, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .validate_mini_app_init_data(&token, bot_username, short_name, init_data)
            .await
    }

    pub async fn send_mini_app_action(
        &self,
        bot_username: &str,
        short_name: &str,
        req: &MiniAppActionRequest,
    ) -> std::result::Result<MiniAppActionResponse, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .send_mini_app_action(&token, bot_username, short_name, req)
            .await
    }

    pub async fn get_cloud_storage(
        &self,
        bot_username: &str,
        short_name: &str,
    ) -> std::result::Result<Vec<CloudStorageItem>, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .get_cloud_storage(&token, bot_username, short_name)
            .await
    }

    pub async fn set_cloud_storage_item(
        &self,
        bot_username: &str,
        short_name: &str,
        key: &str,
        value: &str,
    ) -> std::result::Result<CloudStorageItem, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .set_cloud_storage_item(&token, bot_username, short_name, key, value)
            .await
    }

    pub async fn delete_cloud_storage_item(
        &self,
        bot_username: &str,
        short_name: &str,
        key: &str,
    ) -> std::result::Result<bool, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .delete_cloud_storage_item(&token, bot_username, short_name, key)
            .await
    }

    pub async fn send_chat_callback(
        &self,
        chat_id: i64,
        msg_id: i64,
        callback_data: &str,
    ) -> std::result::Result<PeerBotActionResult, HttpClientError> {
        let token = self.require_token()?;
        self.http_client
            .send_chat_callback(&token, chat_id, msg_id, callback_data)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telegram_init_data_hmac_verification_and_tamper_rejection() {
        let bot_token = "600001:secret_token_xyz";
        let data_check = "auth_date=1731600000\nquery_id=qid_42\nuser={\"id\":100001,\"first_name\":\"Alice\"}";
        let sig = compute_telegram_init_data_hash(data_check, bot_token);
        let raw = format!(
            "query_id=qid_42&user=%7B%22id%22%3A100001%2C%22first_name%22%3A%22Alice%22%7D&auth_date=1731600000&hash={}",
            sig
        );
        assert!(verify_telegram_init_data_hmac(&raw, bot_token).unwrap());

        let tampered = raw.replace("100001", "100002");
        assert!(!verify_telegram_init_data_hmac(&tampered, bot_token).unwrap());
        assert!(!verify_telegram_init_data_hmac(&raw, "600001:wrong_token").unwrap());
    }

    #[test]
    fn test_search_and_hermes_dto_backward_compatibility() {
        let legacy_search_item = serde_json::json!({
            "chat_id": 750000001,
            "title": "Alice",
            "subtitle": "Hey",
            "chat_type": "private",
            "is_bot": false
        });
        let parsed_legacy: SearchResultItemDto =
            serde_json::from_value(legacy_search_item).expect("legacy search item");
        assert_eq!(parsed_legacy.chat_id, 750000001);
        assert!(!parsed_legacy.is_bot);

        // Global Mini App search hit without joined chat_id
        let global_app_item = serde_json::json!({
            "title": "Pomodoro Focus AI",
            "description": "Focus timer mini app",
            "category": "apps",
            "kind": "mini_app",
            "bot_user_id": 777010,
            "bot_username": "PomodoroFocusBot",
            "short_name": "focus",
            "icon_emoji": "🍅",
            "is_bot": true,
            "is_installed": false,
            "install_count": 42
        });
        let parsed_app: SearchResultItemDto =
            serde_json::from_value(global_app_item).expect("global mini app item");
        assert_eq!(parsed_app.chat_id, 0);
        assert_eq!(parsed_app.bot_username.as_deref(), Some("PomodoroFocusBot"));
        assert_eq!(parsed_app.short_name.as_deref(), Some("focus"));
        assert_eq!(parsed_app.install_count, Some(42));

        let legacy_rich_chat = serde_json::json!({
            "chat_id": 700000001,
            "title": "Code Assistant",
            "is_bot": true,
            "peer_user_id": 777000
        });
        let rich: crate::net::hermes_client::HermesRichChatDto =
            serde_json::from_value(legacy_rich_chat).expect("legacy rich chat");
        assert!(rich.is_bot);
        assert!(!rich.has_mini_app);
        assert!(rich.menu_button_url.is_none());
    }
}
