//! InlineResultsWidget: Popup results overlay widget for inline bot queries.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_results_widget.h:L58-L85`
//! - `Telegram/SourceFiles/inline_bots/inline_results_widget.cpp:L27-L133`
#![allow(dead_code)]

use gpui::*;
use super::inline_results_inner::{InlineCacheEntry, InlineResultsInner};

/// Inline bot query debounce interval (upstream: `kInlineBotRequestDelay = 400ms`).
pub const K_INLINE_BOT_REQUEST_DELAY_MS: u64 = 400;

/// Minimum popup height in pixels (upstream: `st::inlineResultsMinHeight`).
pub const K_INLINE_RESULTS_MIN_HEIGHT: f32 = 120.0;

/// Maximum popup height in pixels (upstream: `st::inlineResultsMaxHeight`).
pub const K_INLINE_RESULTS_MAX_HEIGHT: f32 = 360.0;

/// Height ratio of available bottom space (upstream: `st::emojiPanHeightRatio`).
pub const K_EMOJI_PAN_HEIGHT_RATIO: f32 = 0.55;

/// Compute responsive content height for inline results popup
/// (upstream: `Widget::updateContentHeight`, `inline_results_widget.cpp:L98-L109`).
pub fn compute_results_height(bottom_space: f32, max_content_height: f32) -> f32 {
    let wanted = (bottom_space * K_EMOJI_PAN_HEIGHT_RATIO).round();
    wanted
        .clamp(K_INLINE_RESULTS_MIN_HEIGHT, K_INLINE_RESULTS_MAX_HEIGHT)
        .min(bottom_space)
        .min(max_content_height)
}

/// Authentic InlineResultsWidget component state.
pub struct InlineResultsWidget {
    pub bot_username: String,
    pub bot_id: u64,
    pub current_query: String,
    pub is_visible: bool,
    pub is_loading: bool,
    pub inner: InlineResultsInner,
    pub content_height: f32,
    pub bottom_anchor: f32,
    pub last_query_timestamp: i64,
    pub updated_at: i64,
}

impl InlineResultsWidget {
    pub fn new() -> Self {
        Self {
            bot_username: String::new(),
            bot_id: 0,
            current_query: String::new(),
            is_visible: false,
            is_loading: false,
            inner: InlineResultsInner::new(),
            content_height: K_INLINE_RESULTS_MIN_HEIGHT,
            bottom_anchor: 600.0,
            last_query_timestamp: 0,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn query_inline_bot(&mut self, bot_id: u64, bot_username: impl Into<String>, query: impl Into<String>) {
        self.bot_id = bot_id;
        self.bot_username = bot_username.into();
        self.current_query = query.into();
        self.is_loading = true;
        self.is_visible = true;
        self.last_query_timestamp = chrono::Utc::now().timestamp_millis();
        self.content_height = compute_results_height(self.bottom_anchor, K_INLINE_RESULTS_MAX_HEIGHT);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn receive_results(&mut self, entry: InlineCacheEntry) {
        self.is_loading = false;
        self.inner.set_results(entry);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn clear(&mut self) {
        self.is_visible = false;
        self.is_loading = false;
        self.current_query.clear();
        self.bot_username.clear();
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn update_bottom(&mut self, bottom: f32) {
        self.bottom_anchor = bottom;
        self.content_height = compute_results_height(bottom, K_INLINE_RESULTS_MAX_HEIGHT);
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for InlineResultsWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InlineResultsWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        let query = self.current_query.clone();
        let bot = self.bot_username.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h(px(self.content_height))
            .bg(rgb(0xFFFFFF))
            .rounded_t(px(8.0))
            .border_t_1()
            .border_color(rgb(0xDDDDDD))
            .shadow_md()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .px(px(10.0))
                    .py(px(6.0))
                    .bg(rgb(0xF7F7F7))
                    .border_b_1()
                    .border_color(rgb(0xEEEEEE))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child(format!("@{} {}", bot, query))
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(if self.is_loading { "Loading..." } else { "" })
                    )
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .p(px(8.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x666666))
                            .child(format!("{} results available", self.inner.cache.results.len()))
                    )
            )
    }
}
