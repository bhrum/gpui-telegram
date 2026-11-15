//! 1:1 Telegram Mini App, BotFather & AI Companion Bot Contracts.
//!
//! Bridges `fabushi_messaging_core::miniapp` permissions/domain concepts with
//! Telegram-compatible BotFather, Mini App UI schema, `initData` HMAC,
//! CloudStorage quota enforcement, and companion AI bot workflows.

use crate::message::InlineKeyboardButton;
pub use fabushi_messaging_core::miniapp::MiniAppPermission;
use serde::{Deserialize, Serialize};

/// Reserved official `@BotFather` user ID (`UserId(777_001)`).
pub const BOTFATHER_USER_ID: i64 = 777_001;
/// Canonical `@BotFather` username.
pub const BOTFATHER_USERNAME: &str = "BotFather";

/// Telegram Mini App CloudStorage limits (1:1 parity).
pub const CLOUD_STORAGE_MAX_KEYS: usize = 1024;
pub const CLOUD_STORAGE_MAX_KEY_BYTES: usize = 128;
pub const CLOUD_STORAGE_MAX_VALUE_BYTES: usize = 4096;
pub const CLOUD_STORAGE_MAX_TOTAL_BYTES: usize = 5 * 1024 * 1024;

/// Telegram Desktop `Data::BotApp` prolong & refresh intervals.
pub const MINIAPP_PROLONG_TIMEOUT_SECS: u64 = 60;
pub const MINIAPP_REFRESH_BOTS_TIMEOUT_SECS: u64 = 3600;

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct BotCommandSpec {
    pub command: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BotRegistryEntry {
    pub bot_user_id: i64,
    pub owner_user_id: i64,
    pub username: String,
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub about: String,
    #[serde(default)]
    pub ai_persona: String,
    #[serde(default)]
    pub menu_button_text: String,
    #[serde(default)]
    pub menu_button_url: String,
    #[serde(default)]
    pub is_official: bool,
    #[serde(default)]
    pub is_inline: bool,
    #[serde(default)]
    pub allow_attach_menu: bool,
    #[serde(default)]
    pub attach_peer_types: Vec<String>,
    #[serde(default)]
    pub commands: Vec<BotCommandSpec>,
    #[serde(default)]
    pub created_at: i64,
}

/// Returned ONLY to the bot owner upon bot creation or token fetch/revocation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BotTokenResponse {
    pub bot: BotRegistryEntry,
    pub bot_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion_chat_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiniAppThemeParams {
    pub bg_color: String,
    pub secondary_bg_color: String,
    pub text_color: String,
    pub hint_color: String,
    pub button_color: String,
    pub button_text_color: String,
}

impl Default for MiniAppThemeParams {
    fn default() -> Self {
        Self {
            bg_color: "#ffffff".to_string(),
            secondary_bg_color: "#f4f5f7".to_string(),
            text_color: "#111827".to_string(),
            hint_color: "#6b7280".to_string(),
            button_color: "#2481cc".to_string(),
            button_text_color: "#ffffff".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MiniAppActionButton {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(default = "default_true")]
    pub is_visible: bool,
    #[serde(default = "default_true")]
    pub is_active: bool,
    #[serde(default)]
    pub is_progress_visible: bool,
    #[serde(default)]
    pub action_id: String,
    #[serde(default)]
    pub sends_data: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MiniAppWidgetType {
    #[serde(alias = "header")]
    Header,
    #[serde(alias = "badge")]
    Badge,
    #[serde(alias = "metric_card", alias = "metricCard")]
    MetricCard,
    #[default]
    #[serde(alias = "text")]
    Text,
    #[serde(alias = "text_input", alias = "textInput")]
    TextInput,
    #[serde(alias = "select")]
    Select,
    #[serde(alias = "toggle")]
    Toggle,
    #[serde(alias = "counter")]
    Counter,
    #[serde(alias = "list_item", alias = "listItem")]
    ListItem,
    #[serde(alias = "button")]
    Button,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppUiWidget {
    pub id: String,
    #[serde(alias = "type", alias = "widget_type")]
    pub kind: MiniAppWidgetType,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default)]
    pub checked: bool,
    #[serde(default)]
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppUiSchema {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub theme: MiniAppThemeParams,
    #[serde(default)]
    pub widgets: Vec<MiniAppUiWidget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_button: Option<MiniAppActionButton>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_button: Option<MiniAppActionButton>,
    #[serde(default)]
    pub back_button_visible: bool,
    #[serde(default)]
    pub settings_button_visible: bool,
    #[serde(default)]
    pub copilot_suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppManifestDto {
    pub app_id: String,
    pub bot_username: String,
    pub short_name: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub icon_emoji: String,
    #[serde(default)]
    pub web_url: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub peer_types: Vec<String>,
    #[serde(default)]
    pub requested_permissions: Vec<MiniAppPermission>,
    #[serde(default)]
    pub ui_schema: MiniAppUiSchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppEntry {
    pub app_id: String,
    pub bot_user_id: i64,
    pub bot_username: String,
    pub bot_display_name: String,
    pub short_name: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub icon_emoji: String,
    #[serde(default)]
    pub web_url: String,
    #[serde(default)]
    pub ui_schema: MiniAppUiSchema,
    #[serde(default)]
    pub peer_types: Vec<String>,
    #[serde(default)]
    pub install_count: i64,
    #[serde(default)]
    pub is_installed: bool,
    #[serde(default)]
    pub has_write_access: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<i64>,
    #[serde(default)]
    pub created_by_nl: bool,
    #[serde(default)]
    pub updated_at: i64,
}

impl MiniAppEntry {
    pub fn deep_link(&self) -> String {
        format!(
            "https://t.me/{}/{}",
            self.bot_username.trim_start_matches('@'),
            self.short_name
        )
    }

    pub fn gpui_uri(&self) -> String {
        format!(
            "gpui-miniapp://{}/{}",
            self.bot_username.trim_start_matches('@'),
            self.short_name
        )
    }

    pub fn relative_web_path(&self) -> String {
        format!(
            "/miniapps/{}/{}",
            self.bot_username.trim_start_matches('@'),
            self.short_name
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MiniAppLaunchMode {
    #[default]
    #[serde(alias = "compact_sheet")]
    CompactSheet,
    #[serde(alias = "full_size")]
    FullSize,
    #[serde(alias = "full_screen")]
    FullScreen,
    #[serde(alias = "attach_menu")]
    AttachMenu,
    #[serde(alias = "menu_button")]
    MenuButton,
    #[serde(alias = "inline_keyboard")]
    InlineKeyboard,
    #[serde(alias = "direct_link")]
    DirectLink,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MiniAppInitDataDto {
    pub query_id: String,
    pub user_json: String,
    pub auth_date: i64,
    pub hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_param: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_instance: Option<String>,
    pub raw_query_string: String,
    #[serde(default)]
    pub hmac_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CloudStorageItem {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppLaunchContext {
    pub app: MiniAppEntry,
    pub companion_bot: BotRegistryEntry,
    pub companion_chat_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_chat_id: Option<i64>,
    #[serde(default)]
    pub launch_mode: MiniAppLaunchMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_param: Option<String>,
    #[serde(default)]
    pub init_data: MiniAppInitDataDto,
    #[serde(default)]
    pub cloud_storage: Vec<CloudStorageItem>,
    #[serde(default)]
    pub state_json: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MiniAppActionKind {
    SendData,
    WidgetEvent,
    AiCopilot,
    MainButtonClick,
    SecondaryButtonClick,
    BackButtonClick,
    SettingsButtonClick,
}

impl MiniAppActionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SendData => "send_data",
            Self::WidgetEvent => "widget_event",
            Self::AiCopilot => "ai_copilot",
            Self::MainButtonClick => "main_button_click",
            Self::SecondaryButtonClick => "secondary_button_click",
            Self::BackButtonClick => "back_button_click",
            Self::SettingsButtonClick => "settings_button_click",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppActionRequest {
    pub action_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

impl MiniAppActionRequest {
    pub fn send_data(data: impl Into<String>, chat_id: Option<i64>) -> Self {
        Self {
            action_type: MiniAppActionKind::SendData.as_str().to_string(),
            data: Some(data.into()),
            chat_id,
            ..Default::default()
        }
    }

    pub fn widget_event(
        widget_id: impl Into<String>,
        value: serde_json::Value,
        chat_id: Option<i64>,
    ) -> Self {
        Self {
            action_type: MiniAppActionKind::WidgetEvent.as_str().to_string(),
            widget_id: Some(widget_id.into()),
            value: Some(value),
            chat_id,
            ..Default::default()
        }
    }

    pub fn ai_copilot(prompt: impl Into<String>, chat_id: Option<i64>) -> Self {
        Self {
            action_type: MiniAppActionKind::AiCopilot.as_str().to_string(),
            prompt: Some(prompt.into()),
            chat_id,
            ..Default::default()
        }
    }

    pub fn main_button_click(chat_id: Option<i64>) -> Self {
        Self {
            action_type: MiniAppActionKind::MainButtonClick.as_str().to_string(),
            chat_id,
            ..Default::default()
        }
    }

    pub fn secondary_button_click(chat_id: Option<i64>) -> Self {
        Self {
            action_type: MiniAppActionKind::SecondaryButtonClick.as_str().to_string(),
            chat_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MiniAppActionResponse {
    pub ok: bool,
    pub ui_schema: MiniAppUiSchema,
    #[serde(default)]
    pub state_json: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion_bot_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion_chat_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toast: Option<String>,
    #[serde(default)]
    pub close_app: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_reply: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CreateBotRequest {
    pub display_name: String,
    pub username: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub about: String,
    #[serde(default)]
    pub ai_persona: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_button_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_button_url: Option<String>,
    #[serde(default)]
    pub allow_attach_menu: bool,
    #[serde(default)]
    pub attach_peer_types: Vec<String>,
    #[serde(default)]
    pub commands: Vec<BotCommandSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateMiniAppRequest {
    pub bot_username: String,
    pub short_name: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub icon_emoji: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui_schema: Option<MiniAppUiSchema>,
    #[serde(default)]
    pub peer_types: Vec<String>,
    #[serde(default)]
    pub is_attach_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NaturalLanguageCreateRequest {
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NaturalLanguageCreateResponse {
    pub bot: BotRegistryEntry,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<MiniAppEntry>,
    pub bot_token: String,
    pub companion_chat_id: i64,
    pub summary_markdown: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InstallMiniAppRequest {
    #[serde(default)]
    pub allow_write_access: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LaunchMiniAppRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(default)]
    pub launch_mode: MiniAppLaunchMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_param: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ValidateInitDataRequest {
    pub init_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ValidateInitDataResponse {
    pub valid: bool,
    #[serde(default)]
    pub bot_username: String,
    #[serde(default)]
    pub short_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_date: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CloudStorageSetRequest {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CloudStorageDeleteRequest {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChatCallbackRequest {
    pub message_id: i64,
    pub callback_data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PeerBotActionResult {
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<MiniAppEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion_bot: Option<BotRegistryEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companion_chat_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_message_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toast: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_keyboard: Option<Vec<Vec<InlineKeyboardButton>>>,
}

/// Validates a Telegram bot username (5-32 ASCII alphanumeric/underscore chars, ends with `bot`).
pub fn validate_bot_username(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim().trim_start_matches('@');
    if trimmed.len() < 5 || trimmed.len() > 32 {
        return Err("Bot username must be 5-32 characters long".to_string());
    }
    let bytes = trimmed.as_bytes();
    if !bytes[0].is_ascii_alphabetic() {
        return Err("Bot username must start with an ASCII letter".to_string());
    }
    if !trimmed
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return Err("Bot username may only contain letters, digits, and underscores".to_string());
    }
    if !trimmed.to_ascii_lowercase().ends_with("bot") {
        return Err("Sorry, the username must end in 'bot' (e.g. 'TetrisBot' or 'tetris_bot')".to_string());
    }
    Ok(trimmed.to_string())
}

/// Validates and normalizes a Mini App `short_name` (`2-32` chars `[a-zA-Z0-9_-]`).
pub fn validate_miniapp_short_name(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.len() < 2 || trimmed.len() > 32 {
        return Err("Mini App short_name must be 2-32 characters long".to_string());
    }
    if !trimmed
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err("Mini App short_name may only contain letters, digits, '_' or '-'".to_string());
    }
    Ok(trimmed.to_string())
}

/// Validates a bot slash command (`1-32` lowercase alphanumeric/underscore chars).
pub fn validate_bot_command(raw: &str) -> Result<String, String> {
    let clean = raw.trim().trim_start_matches('/');
    if clean.is_empty() || clean.len() > 32 {
        return Err("Bot command must be 1-32 characters long".to_string());
    }
    if !clean
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        return Err("Bot command must contain only lowercase letters, digits, or '_'".to_string());
    }
    Ok(clean.to_string())
}

/// Enforces Telegram CloudStorage limits (`1024` keys, `128B` key, `4096B` value, `5MB` total).
pub fn validate_cloud_storage_entry(
    key: &str,
    value: &str,
    existing: &[CloudStorageItem],
) -> Result<(), String> {
    if key.is_empty() || key.len() > CLOUD_STORAGE_MAX_KEY_BYTES {
        return Err(format!(
            "CloudStorage key must be 1..={} bytes (got {})",
            CLOUD_STORAGE_MAX_KEY_BYTES,
            key.len()
        ));
    }
    if value.len() > CLOUD_STORAGE_MAX_VALUE_BYTES {
        return Err(format!(
            "CloudStorage value exceeds {} bytes limit (got {})",
            CLOUD_STORAGE_MAX_VALUE_BYTES,
            value.len()
        ));
    }
    let existing_pos = existing.iter().position(|item| item.key == key);
    if existing_pos.is_none() && existing.len() >= CLOUD_STORAGE_MAX_KEYS {
        return Err(format!(
            "CloudStorage max keys limit ({}) exceeded",
            CLOUD_STORAGE_MAX_KEYS
        ));
    }
    let other_bytes: usize = existing
        .iter()
        .filter(|item| item.key != key)
        .map(|item| item.key.len() + item.value.len())
        .sum();
    let total_bytes = other_bytes + key.len() + value.len();
    if total_bytes > CLOUD_STORAGE_MAX_TOTAL_BYTES {
        return Err(format!(
            "CloudStorage 5MB quota exceeded ({} > {})",
            total_bytes, CLOUD_STORAGE_MAX_TOTAL_BYTES
        ));
    }
    Ok(())
}

/// Parses `gpui-miniapp://bot/short`, `/miniapps/bot/short`, `http(s)://host/miniapps/bot/short`,
/// or `https://t.me/bot/short?startapp=xyz` into `(bot_username, short_name, start_param)`.
pub fn parse_miniapp_uri(raw: &str) -> Option<(String, String, Option<String>)> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (path_part, query_part) = match trimmed.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (trimmed, None),
    };

    let start_param = query_part.and_then(|q| {
        q.split('&').find_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            if k == "startapp" || k == "start_param" {
                let val = v.trim();
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
            None
        })
    });

    let rest = if let Some(after) = path_part.strip_prefix("gpui-miniapp://") {
        after
    } else if let Some(idx) = path_part.find("/miniapps/") {
        &path_part[idx + "/miniapps/".len()..]
    } else if let Some(after) = path_part
        .strip_prefix("https://t.me/")
        .or_else(|| path_part.strip_prefix("http://t.me/"))
        .or_else(|| path_part.strip_prefix("t.me/"))
    {
        after
    } else {
        return None;
    };

    let mut segments = rest.trim_matches('/').split('/').filter(|s| !s.is_empty());
    let bot = segments.next()?.trim_start_matches('@').to_string();
    let short = segments.next()?.to_string();
    if bot.is_empty() || short.is_empty() {
        return None;
    }
    Some((bot, short, start_param))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_username_and_short_name_validation() {
        assert_eq!(
            validate_bot_username("@CoffeeOrderBot").unwrap(),
            "CoffeeOrderBot"
        );
        assert_eq!(
            validate_bot_username("task_checkin_bot").unwrap(),
            "task_checkin_bot"
        );
        assert!(validate_bot_username("foo").is_err());
        assert!(validate_bot_username("myhelper").is_err());
        assert!(validate_bot_username("1badbot").is_err());

        assert_eq!(validate_miniapp_short_name("focus").unwrap(), "focus");
        assert_eq!(validate_miniapp_short_name("my-app_1").unwrap(), "my-app_1");
        assert!(validate_miniapp_short_name("a").is_err());
        assert!(validate_miniapp_short_name("bad name").is_err());

        assert_eq!(validate_bot_command("/newbot").unwrap(), "newbot");
        assert!(validate_bot_command("/BadCmd").is_err());
    }

    #[test]
    fn test_cloud_storage_limits_enforced() {
        let mut items = Vec::new();
        assert!(validate_cloud_storage_entry("theme", "dark", &items).is_ok());
        assert!(validate_cloud_storage_entry("", "dark", &items).is_err());
        let long_key = "k".repeat(CLOUD_STORAGE_MAX_KEY_BYTES + 1);
        assert!(validate_cloud_storage_entry(&long_key, "v", &items).is_err());
        let long_val = "v".repeat(CLOUD_STORAGE_MAX_VALUE_BYTES + 1);
        assert!(validate_cloud_storage_entry("ok", &long_val, &items).is_err());

        for i in 0..CLOUD_STORAGE_MAX_KEYS {
            items.push(CloudStorageItem {
                key: format!("k{}", i),
                value: "1".to_string(),
                updated_at: 1,
            });
        }
        assert!(validate_cloud_storage_entry("k_over", "1", &items).is_err());
        assert!(validate_cloud_storage_entry("k0", "updated", &items).is_ok());
    }

    #[test]
    fn test_parse_miniapp_uri_variants() {
        assert_eq!(
            parse_miniapp_uri("gpui-miniapp://@PomodoroFocusBot/focus?startapp=work"),
            Some((
                "PomodoroFocusBot".to_string(),
                "focus".to_string(),
                Some("work".to_string())
            ))
        );
        assert_eq!(
            parse_miniapp_uri("http://127.0.0.1:8080/miniapps/SplitBillBot/split"),
            Some(("SplitBillBot".to_string(), "split".to_string(), None))
        );
        assert_eq!(
            parse_miniapp_uri("https://t.me/PollMasterBot/poll?startapp=team"),
            Some((
                "PollMasterBot".to_string(),
                "poll".to_string(),
                Some("team".to_string())
            ))
        );
        assert_eq!(parse_miniapp_uri("https://example.com/docs"), None);
    }

    #[test]
    fn test_miniapp_launch_context_roundtrip() {
        let ctx = MiniAppLaunchContext {
            app: MiniAppEntry {
                app_id: "app_1".to_string(),
                bot_user_id: 600_001,
                bot_username: "CoffeeOrderBot".to_string(),
                bot_display_name: "Coffee Order AI".to_string(),
                short_name: "order".to_string(),
                title: "Coffee Order".to_string(),
                ui_schema: MiniAppUiSchema {
                    title: "Coffee Order".to_string(),
                    widgets: vec![MiniAppUiWidget {
                        id: "qty".to_string(),
                        kind: MiniAppWidgetType::Counter,
                        label: "Cups".to_string(),
                        count: 2,
                        min: Some(1),
                        max: Some(10),
                        ..Default::default()
                    }],
                    main_button: Some(MiniAppActionButton {
                        text: "Submit Order".to_string(),
                        action_id: "submit".to_string(),
                        sends_data: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
            companion_bot: BotRegistryEntry {
                bot_user_id: 600_001,
                owner_user_id: 100_001,
                username: "CoffeeOrderBot".to_string(),
                display_name: "Coffee Order AI".to_string(),
                commands: vec![BotCommandSpec {
                    command: "start".to_string(),
                    description: "Open bot".to_string(),
                }],
                ..Default::default()
            },
            companion_chat_id: 600_000_001,
            launch_mode: MiniAppLaunchMode::CompactSheet,
            state_json: serde_json::json!({"cups": 2}),
            ..Default::default()
        };
        let encoded = serde_json::to_string(&ctx).expect("serialize launch context");
        let decoded: MiniAppLaunchContext =
            serde_json::from_str(&encoded).expect("deserialize launch context");
        assert_eq!(ctx, decoded);
    }
}
