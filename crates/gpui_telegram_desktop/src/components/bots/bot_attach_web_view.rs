//! BotAttachWebView: Telegram Mini Apps interactive native GPUI runtime, sheet & AI Co-Pilot.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

use gpui_telegram_proto::miniapp_contracts::{
    CloudStorageItem, MiniAppActionRequest, MiniAppActionResponse, MiniAppLaunchContext,
    MiniAppUiSchema,
};

use crate::theme::TdesktopThemePalette;

/// Timeout before prolonging active bot webview session (60 seconds).
pub const K_PROLONG_TIMEOUT_SECS: u64 = 60;
/// Timeout before refreshing attach bots list (3600 seconds).
pub const K_REFRESH_BOTS_TIMEOUT_SECS: u64 = 3600;
/// Limit for popular app bots in catalog searches.
pub const K_POPULAR_APP_BOTS_LIMIT: usize = 100;

/// Bitmask for allowed peer types where an attach menu bot can be launched.
pub mod peer_types {
    pub const SAME_BOT: u8 = 0x01;
    pub const BOT: u8 = 0x02;
    pub const USER: u8 = 0x04;
    pub const GROUP: u8 = 0x08;
    pub const BROADCAST: u8 = 0x10;
}

pub fn peer_matches_types(peer_type: u8, allowed_types: u8) -> bool {
    (peer_type & allowed_types) != 0
}

pub fn peer_types_mask_from_strings(peer_types_vec: &[String]) -> u8 {
    if peer_types_vec.is_empty() {
        return peer_types::SAME_BOT
            | peer_types::BOT
            | peer_types::USER
            | peer_types::GROUP
            | peer_types::BROADCAST;
    }
    let mut mask = 0u8;
    for raw in peer_types_vec {
        match raw.trim().to_ascii_lowercase().as_str() {
            "same_bot" | "samebot" => mask |= peer_types::SAME_BOT,
            "bot" | "bots" => mask |= peer_types::BOT,
            "user" | "users" | "private" => mask |= peer_types::USER,
            "group" | "groups" | "chat" | "supergroup" => mask |= peer_types::GROUP,
            "broadcast" | "channel" | "channels" => mask |= peer_types::BROADCAST,
            _ => {}
        }
    }
    if mask == 0 {
        peer_types::SAME_BOT | peer_types::BOT | peer_types::USER | peer_types::GROUP
    } else {
        mask
    }
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachBotEntry {
    pub bot_id: u64,
    pub name: String,
    pub short_name: String,
    pub allowed_peer_types: u8,
    pub in_main_menu: bool,
    pub in_attach_menu: bool,
    pub disclaimer_required: bool,
    pub request_write_access: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotWebViewInputTarget {
    WidgetText(String),
    CopilotPrompt,
    CloudKey,
    CloudValue,
}

#[derive(Debug, Clone)]
pub enum BotWebViewPendingAction {
    Close,
    Minimize,
    Reload {
        bot_username: String,
        short_name: String,
        chat_id: Option<i64>,
    },
    AcceptDisclaimer {
        bot_username: String,
        short_name: String,
        allow_write_access: bool,
    },
    OpenCompanionChat {
        companion_chat_id: i64,
        bot_username: String,
    },
    OpenInBrowser {
        path: String,
    },
    UninstallApp {
        bot_username: String,
        short_name: String,
    },
    SendAppAction {
        bot_username: String,
        short_name: String,
        req: MiniAppActionRequest,
    },
    CloudStorageRefresh {
        bot_username: String,
        short_name: String,
    },
    CloudStorageSet {
        bot_username: String,
        short_name: String,
        key: String,
        value: String,
    },
    CloudStorageDelete {
        bot_username: String,
        short_name: String,
        key: String,
    },
}

/// Authentic BotAttachWebView component state + full Telegram Desktop Mini App interactive shell.
pub struct BotAttachWebView {
    pub active_bot: Option<AttachBotEntry>,
    pub url: String,
    pub is_open: bool,
    pub is_fullscreen: bool,
    pub theme_params_json: String,
    pub disclaimer_accepted: bool,
    pub last_active_ts: u64,
    pub updated_at: u64,

    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub bot_username: String,
    pub companion_chat_id: Option<i64>,
    pub source_chat_id: Option<i64>,
    pub launch_context: Option<MiniAppLaunchContext>,
    pub ui_schema: MiniAppUiSchema,
    pub cloud_storage: Vec<CloudStorageItem>,
    pub active_tab: u8,
    pub menu_open: bool,
    pub confirm_close_open: bool,
    pub is_dirty: bool,
    pub is_loading: bool,
    pub status_toast: Option<String>,
    pub allow_write_access_checked: bool,
    pub copilot_expanded: bool,
    pub copilot_prompt: String,
    pub copilot_last_reply: Option<String>,
    pub focused_input: Option<BotWebViewInputTarget>,
    pub cloud_key_draft: String,
    pub cloud_value_draft: String,
    pub pending_actions: VecDeque<BotWebViewPendingAction>,
}

impl BotAttachWebView {
    pub fn new() -> Self {
        let palette = TdesktopThemePalette::default();
        Self {
            active_bot: None,
            url: String::new(),
            is_open: false,
            is_fullscreen: false,
            theme_params_json: Self::build_theme_params_json(&palette),
            disclaimer_accepted: false,
            last_active_ts: 0,
            updated_at: 0,
            palette,
            language_id: "en".to_string(),
            bot_username: String::new(),
            companion_chat_id: None,
            source_chat_id: None,
            launch_context: None,
            ui_schema: MiniAppUiSchema::default(),
            cloud_storage: Vec::new(),
            active_tab: 0,
            menu_open: false,
            confirm_close_open: false,
            is_dirty: false,
            is_loading: false,
            status_toast: None,
            allow_write_access_checked: true,
            copilot_expanded: true,
            copilot_prompt: String::new(),
            copilot_last_reply: None,
            focused_input: None,
            cloud_key_draft: String::new(),
            cloud_value_draft: String::new(),
            pending_actions: VecDeque::new(),
        }
    }

    fn build_theme_params_json(palette: &TdesktopThemePalette) -> String {
        format!(
            "{{\"bg_color\":\"#{:06x}\",\"secondary_bg_color\":\"#{:06x}\",\"text_color\":\"#{:06x}\",\"hint_color\":\"#{:06x}\",\"button_color\":\"#{:06x}\"}}",
            palette.modal_bg & 0xFFFFFF,
            palette.card_bg & 0xFFFFFF,
            palette.text_primary & 0xFFFFFF,
            palette.text_secondary & 0xFFFFFF,
            palette.accent & 0xFFFFFF
        )
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
        self.theme_params_json = Self::build_theme_params_json(&palette);
    }

    pub fn set_language(&mut self, lang_id: impl Into<String>) {
        self.language_id = lang_id.into();
    }

    pub fn launch(&mut self, bot: AttachBotEntry, url: impl Into<String>) {
        let ts = now_unix_secs();
        self.disclaimer_accepted = !bot.disclaimer_required;
        self.allow_write_access_checked = bot.request_write_access;
        if self.bot_username.is_empty() {
            self.bot_username = bot.short_name.clone();
        }
        self.active_bot = Some(bot);
        self.url = url.into();
        self.is_open = true;
        self.menu_open = false;
        self.confirm_close_open = false;
        self.last_active_ts = ts;
        self.updated_at = ts;
    }

    pub fn load_context(&mut self, ctx: MiniAppLaunchContext) {
        let entry = AttachBotEntry {
            bot_id: ctx.app.bot_user_id.max(0) as u64,
            name: ctx.app.title.clone(),
            short_name: ctx.app.short_name.clone(),
            allowed_peer_types: peer_types_mask_from_strings(&ctx.app.peer_types),
            in_main_menu: true,
            in_attach_menu: ctx.companion_bot.allow_attach_menu,
            disclaimer_required: !ctx.app.is_installed,
            request_write_access: true,
        };
        let deep_url = if ctx.app.web_url.is_empty() {
            ctx.app.gpui_uri()
        } else {
            ctx.app.web_url.clone()
        };
        self.bot_username = ctx.app.bot_username.clone();
        self.companion_chat_id = Some(ctx.companion_chat_id);
        self.source_chat_id = ctx.source_chat_id;
        self.ui_schema = ctx.app.ui_schema.clone();
        self.cloud_storage = ctx.cloud_storage.clone();
        self.launch(entry, deep_url);
        self.disclaimer_accepted = ctx.app.is_installed;
        self.allow_write_access_checked = true;
        self.is_dirty = false;
        self.is_loading = false;
        self.status_toast = None;
        self.launch_context = Some(ctx);
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.active_bot = None;
        self.url.clear();
        self.menu_open = false;
        self.confirm_close_open = false;
        self.focused_input = None;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    pub fn accept_disclaimer(&mut self) {
        self.disclaimer_accepted = true;
        if let Some(ref mut bot) = self.active_bot {
            bot.disclaimer_required = false;
        }
    }

    pub fn apply_action_response(&mut self, resp: MiniAppActionResponse) {
        self.is_loading = false;
        self.ui_schema = resp.ui_schema;
        if let Some(cid) = resp.companion_chat_id {
            self.companion_chat_id = Some(cid);
        }
        if let Some(reply) = resp.ai_reply.or(resp.companion_bot_message) {
            self.copilot_last_reply = Some(reply);
            self.copilot_expanded = true;
        }
        if let Some(toast) = resp.toast {
            self.status_toast = Some(toast);
        }
        if resp.close_app {
            self.close();
            self.pending_actions.push_back(BotWebViewPendingAction::Close);
        }
    }

    pub fn apply_cloud_storage(&mut self, items: Vec<CloudStorageItem>) {
        self.cloud_storage = items;
    }

    pub fn take_pending_actions(&mut self) -> Vec<BotWebViewPendingAction> {
        self.pending_actions.drain(..).collect()
    }

    pub fn has_focused_text_input(&self) -> bool {
        self.is_open && self.focused_input.is_some()
    }

    pub fn active_input_text(&self) -> String {
        match &self.focused_input {
            Some(BotWebViewInputTarget::CopilotPrompt) => self.copilot_prompt.clone(),
            Some(BotWebViewInputTarget::CloudKey) => self.cloud_key_draft.clone(),
            Some(BotWebViewInputTarget::CloudValue) => self.cloud_value_draft.clone(),
            Some(BotWebViewInputTarget::WidgetText(id)) => self
                .ui_schema
                .widgets
                .iter()
                .find(|w| &w.id == id)
                .map(|w| w.value.clone())
                .unwrap_or_default(),
            None => String::new(),
        }
    }

    pub fn push_text_to_focused_input(&mut self, text: &str) -> bool {
        let Some(target) = self.focused_input.clone() else {
            return false;
        };
        match target {
            BotWebViewInputTarget::CopilotPrompt => {
                self.copilot_prompt.push_str(text);
                true
            }
            BotWebViewInputTarget::CloudKey => {
                self.cloud_key_draft.push_str(text);
                true
            }
            BotWebViewInputTarget::CloudValue => {
                self.cloud_value_draft.push_str(text);
                true
            }
            BotWebViewInputTarget::WidgetText(id) => {
                if let Some(w) = self.ui_schema.widgets.iter_mut().find(|w| w.id == id) {
                    w.value.push_str(text);
                    self.is_dirty = true;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn backspace_focused_input(&mut self) -> bool {
        let Some(target) = self.focused_input.clone() else {
            return false;
        };
        match target {
            BotWebViewInputTarget::CopilotPrompt => {
                self.copilot_prompt.pop();
                true
            }
            BotWebViewInputTarget::CloudKey => {
                self.cloud_key_draft.pop();
                true
            }
            BotWebViewInputTarget::CloudValue => {
                self.cloud_value_draft.pop();
                true
            }
            BotWebViewInputTarget::WidgetText(id) => {
                if let Some(w) = self.ui_schema.widgets.iter_mut().find(|w| w.id == id) {
                    w.value.pop();
                    self.is_dirty = true;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn submit_focused_input(&mut self) -> bool {
        let Some(target) = self.focused_input.clone() else {
            return false;
        };
        match target {
            BotWebViewInputTarget::CopilotPrompt => {
                self.trigger_ai_copilot();
                true
            }
            BotWebViewInputTarget::CloudKey | BotWebViewInputTarget::CloudValue => {
                self.trigger_save_cloud_storage();
                true
            }
            BotWebViewInputTarget::WidgetText(id) => {
                if let Some(val) = self
                    .ui_schema
                    .widgets
                    .iter()
                    .find(|w| w.id == id)
                    .map(|w| w.value.clone())
                {
                    self.queue_widget_event(id, serde_json::json!(val));
                    true
                } else {
                    false
                }
            }
        }
    }

    pub(crate) fn current_short_name(&self) -> String {
        self.active_bot
            .as_ref()
            .map(|b| b.short_name.clone())
            .filter(|s| !s.is_empty())
            .or_else(|| self.launch_context.as_ref().map(|c| c.app.short_name.clone()))
            .unwrap_or_else(|| "app".to_string())
    }

    pub(crate) fn target_chat_i64(&self) -> Option<i64> {
        self.source_chat_id.or(self.companion_chat_id)
    }

    pub fn queue_widget_event(&mut self, widget_id: String, value: serde_json::Value) {
        self.is_dirty = true;
        self.is_loading = true;
        self.pending_actions
            .push_back(BotWebViewPendingAction::SendAppAction {
                bot_username: self.bot_username.clone(),
                short_name: self.current_short_name(),
                req: MiniAppActionRequest::widget_event(
                    widget_id,
                    value,
                    self.target_chat_i64(),
                ),
            });
    }

    pub fn trigger_ai_copilot(&mut self) {
        let prompt = self.copilot_prompt.trim().to_string();
        if prompt.is_empty() {
            return;
        }
        self.copilot_prompt.clear();
        self.is_loading = true;
        self.pending_actions
            .push_back(BotWebViewPendingAction::SendAppAction {
                bot_username: self.bot_username.clone(),
                short_name: self.current_short_name(),
                req: MiniAppActionRequest::ai_copilot(prompt, self.target_chat_i64()),
            });
    }

    pub fn trigger_save_cloud_storage(&mut self) {
        let key = self.cloud_key_draft.trim().to_string();
        let val = self.cloud_value_draft.clone();
        if key.is_empty() {
            return;
        }
        self.pending_actions
            .push_back(BotWebViewPendingAction::CloudStorageSet {
                bot_username: self.bot_username.clone(),
                short_name: self.current_short_name(),
                key,
                value: val,
            });
        self.cloud_key_draft.clear();
        self.cloud_value_draft.clear();
    }
}

impl Default for BotAttachWebView {
    fn default() -> Self {
        Self::new()
    }
}
