//! BotAttachWebView: Telegram Mini Apps webview attachment container & lifecycle.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/bot_attach_web_view.h:L59-L85`
//! - `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp:L109-L140`
#![allow(dead_code)]

use gpui::*;

/// Mini App session prolong timeout (upstream: `kProlongTimeout = 60 * 1000ms`).
pub const K_PROLONG_TIMEOUT_SECS: u64 = 60;

/// App list refresh timeout (upstream: `kRefreshBotsTimeout = 60 * 60 * 1000ms`).
pub const K_REFRESH_BOTS_TIMEOUT_SECS: u64 = 3600;

/// Popular app bots limit (upstream: `kPopularAppBotsLimit = 100`).
pub const K_POPULAR_APP_BOTS_LIMIT: usize = 100;

/// Peer type bitflags for mini app eligibility (upstream: `InlineBots::PeerType`).
pub mod peer_types {
    pub const SAME_BOT: u8 = 0x01;
    pub const BOT: u8 = 0x02;
    pub const USER: u8 = 0x04;
    pub const GROUP: u8 = 0x08;
    pub const BROADCAST: u8 = 0x10;
}

/// Verify if peer matches requested flags (upstream: `PeerMatchesTypes`, `bot_attach_web_view.cpp`).
pub fn peer_matches_types(peer_type: u8, allowed_types: u8) -> bool {
    (peer_type & allowed_types) != 0
}

/// Attach bot menu metadata entry (upstream: `AttachWebViewBot`).
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

/// Authentic BotAttachWebView component state.
pub struct BotAttachWebView {
    pub active_bot: Option<AttachBotEntry>,
    pub url: String,
    pub is_open: bool,
    pub is_fullscreen: bool,
    pub theme_params_json: String,
    pub disclaimer_accepted: bool,
    pub last_active_ts: i64,
    pub updated_at: i64,
}

impl BotAttachWebView {
    pub fn new() -> Self {
        Self {
            active_bot: None,
            url: String::new(),
            is_open: false,
            is_fullscreen: false,
            theme_params_json: "{}".to_string(),
            disclaimer_accepted: false,
            last_active_ts: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn launch(&mut self, bot: AttachBotEntry, url: impl Into<String>) {
        self.active_bot = Some(bot);
        self.url = url.into();
        self.is_open = true;
        self.last_active_ts = chrono::Utc::now().timestamp();
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.active_bot = None;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn accept_disclaimer(&mut self) {
        self.disclaimer_accepted = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for BotAttachWebView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for BotAttachWebView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div();
        }

        let bot_name = self
            .active_bot
            .as_ref()
            .map(|b| b.name.as_str())
            .unwrap_or("Mini App");

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(44.0))
                    .px(px(12.0))
                    .bg(rgb(0xFAFAFA))
                    .border_b_1()
                    .border_color(rgb(0xEEEEEE))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .child(bot_name.to_string())
                    )
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Loading {}", self.url))
                    )
            )
    }
}
