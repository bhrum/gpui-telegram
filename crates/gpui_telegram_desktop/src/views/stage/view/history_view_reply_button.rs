//! Authentic HistoryView Reply Button View Component (`history_view_reply_button.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_reply_button.cpp:L20-L48`
//! Mandate #18 Citations:
//! - Toggle duration: `kToggleDuration = 120ms` (`history_view_reply_button.cpp:L20`)
//! - Show delay: `kButtonShowDelay = 0ms` (`history_view_reply_button.cpp:L21`)
//! - Hide delay: `kButtonHideDelay = 300ms` (`history_view_reply_button.cpp:L22`)
//! - Reply corner height: `replyCornerHeight = 28px` (`history_view_reply_button.cpp:L39`)

use gpui::*;
use serde::{Deserialize, Serialize};

pub const K_TOGGLE_DURATION_MS: u64 = 120;
pub const K_BUTTON_SHOW_DELAY_MS: u64 = 0;
pub const K_BUTTON_HIDE_DELAY_MS: u64 = 300;
pub const REPLY_CORNER_HEIGHT: f32 = 28.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ReplyButtonState {
    #[default]
    Hidden,
    Shown,
    Active,
    Inside,
}

impl ReplyButtonState {
    pub fn scale(&self) -> f32 {
        match self {
            ReplyButtonState::Hidden => 0.0,
            ReplyButtonState::Shown | ReplyButtonState::Active | ReplyButtonState::Inside => 1.0,
        }
    }
}

/// Authentic GPUI View for HistoryView::ReplyButton::Button.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewReplyButtonView {
    pub state: ReplyButtonState,
    pub item_id: u64,
    pub scale: f32,
    pub opacity: f32,
}

impl HistoryViewReplyButtonView {
    pub fn new() -> Self {
        Self {
            state: ReplyButtonState::Hidden,
            item_id: 0,
            scale: 0.0,
            opacity: 0.0,
        }
    }

    pub fn with_item(item_id: u64) -> Self {
        Self {
            state: ReplyButtonState::Shown,
            item_id,
            scale: 1.0,
            opacity: 1.0,
        }
    }

    pub fn apply_state(&mut self, state: ReplyButtonState) {
        self.state = state;
        self.scale = state.scale();
        self.opacity = self.scale;
    }
}

impl Default for HistoryViewReplyButtonView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewReplyButtonView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.state == ReplyButtonState::Hidden {
            return div();
        }

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(REPLY_CORNER_HEIGHT))
            .px(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded_full()
            .shadow_sm()
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(0x2481CC))
            .child("Reply")
    }
}
