//! History View Unsupported Notice Media Component (`history_view_unsupported_notice`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.h` (L1-L48)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.cpp` (L1-L125)
//!
//! Renders notice card for messages containing unsupported media or future protocol formats,
//! offering an interactive "Update Telegram" button.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Maximum width for the unsupported notice card.
/// Upstream: `history_view_unsupported_notice.cpp:L41` (`st::unsupportedNoticeMaxWidth`) (~360px)
pub const K_UNSUPPORTED_NOTICE_MAX_WIDTH: f32 = 360.0;

/// Default notice title.
/// Upstream: `history_view_unsupported_notice.cpp:L34` (`tr::lng_unsupported_message_title`)
pub const K_UNSUPPORTED_TITLE: &str = "Unsupported Message";

/// Default notice explanatory body text.
/// Upstream: `history_view_unsupported_notice.cpp:L35` (`tr::lng_unsupported_message_text`)
pub const K_UNSUPPORTED_TEXT: &str =
    "This message is not supported on your version of Telegram Desktop. Please update the app to view it.";

/// Default update button label.
/// Upstream: `history_view_unsupported_notice.cpp:L36` (`tr::lng_unsupported_message_update`)
pub const K_UNSUPPORTED_BUTTON_TEXT: &str = "Update Telegram";

/// Authentic GPUI View for unsupported message notice in chat history.
/// Upstream: `history_view_unsupported_notice.h:L15-L46`, `history_view_unsupported_notice.cpp:L30-L124`
#[derive(Debug, Clone)]
pub struct HistoryViewUnsupportedNoticeViewMedia {
    pub title_text: String,
    pub body_text: String,
    pub button_text: String,
    pub is_update_requested: bool,
    pub max_card_width: f32,
    pub is_card_active: bool,
}

impl HistoryViewUnsupportedNoticeViewMedia {
    /// Creates an unsupported message notice with official Telegram copy.
    /// Upstream: `history_view_unsupported_notice.cpp:L30-L37`
    pub fn new() -> Self {
        Self {
            title_text: K_UNSUPPORTED_TITLE.to_string(),
            body_text: K_UNSUPPORTED_TEXT.to_string(),
            button_text: K_UNSUPPORTED_BUTTON_TEXT.to_string(),
            is_update_requested: false,
            max_card_width: K_UNSUPPORTED_NOTICE_MAX_WIDTH,
            is_card_active: true,
        }
    }

    /// Whether this media element needs standard message bubble decoration.
    /// Upstream: `history_view_unsupported_notice.cpp:L111-L113` (`needsBubble() const { return false; }`)
    pub fn needs_bubble(&self) -> bool {
        false
    }

    /// Whether this media element uses custom info layout instead of timestamp corner ticks.
    /// Upstream: `history_view_unsupported_notice.cpp:L115-L117` (`customInfoLayout() const { return true; }`)
    pub fn custom_info_layout(&self) -> bool {
        true
    }

    /// Triggers the app update action.
    /// Upstream: `history_view_unsupported_notice.cpp:L22-L26` (`MakeUpdateTelegramHandler`)
    pub fn trigger_update(&mut self) {
        self.is_update_requested = true;
    }
}

impl Default for HistoryViewUnsupportedNoticeViewMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewUnsupportedNoticeViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_updating = self.is_update_requested;

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .py(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .max_w(px(self.max_card_width))
                    .p(px(14.0))
                    .rounded(px(10.0))
                    .bg(rgb(0xF8F9FA))
                    .border_1()
                    .border_color(rgb(0xDFE3E8))
                    // Warning icon badge
                    .child(
                        div()
                            .w(px(44.0))
                            .h(px(44.0))
                            .rounded_full()
                            .bg(rgb(0xFFF3CD))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(22.0))
                            .mb(px(8.0))
                            .child("⚠️")
                    )
                    // Notice title
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .mb(px(4.0))
                            .child(self.title_text.clone())
                    )
                    // Notice explanation
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x666666))
                            .text_align(TextAlign::Center)
                            .mb(px(12.0))
                            .child(self.body_text.clone())
                    )
                    // Update button
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(if is_updating { rgb(0x40B34F) } else { rgb(0x27A6E5) })
                            .text_color(rgb(0xFFFFFF))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(if is_updating { "Checking for updates...".to_string() } else { self.button_text.clone() })
                    )
            )
    }
}
