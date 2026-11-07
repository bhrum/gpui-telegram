//! Authentic HistoryView Corner Buttons Component (`history_view_corner_buttons.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_corner_buttons.cpp:L36-L741`
//! Mandate #18 Citations:
//! - Stash animation constants:
//!   - `kStashDelay = 150ms` (`history_view_corner_buttons.cpp:L37`)
//!   - `kStashArrowDuration = 106ms` (`history_view_corner_buttons.cpp:L38`)
//!   - `kStashBounceDuration = 173ms` (`history_view_corner_buttons.cpp:L39`)
//!   - `kStashDuration = 429ms` (`history_view_corner_buttons.cpp:L40-L42`)

use gpui::*;
use serde::{Deserialize, Serialize};

pub const K_STASH_DELAY_MS: u64 = 150;
pub const K_STASH_ARROW_DURATION_MS: u64 = 106;
pub const K_STASH_BOUNCE_DURATION_MS: u64 = 173;
pub const K_STASH_DURATION_MS: u64 =
    K_STASH_DELAY_MS + K_STASH_ARROW_DURATION_MS + K_STASH_BOUNCE_DURATION_MS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CornerButtonType {
    Down,
    Mentions,
    Reactions,
    PollVotes,
    Stash,
}

/// Authentic GPUI View for HistoryView::CornerButtons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewCornerButtonsView {
    pub down_shown: bool,
    pub down_counter: Option<i32>,
    pub mentions_shown: bool,
    pub mentions_counter: Option<i32>,
    pub reactions_shown: bool,
    pub reactions_counter: Option<i32>,
    pub poll_votes_shown: bool,
    pub stash_shown: bool,
    pub reply_returns: Vec<u64>,
}

impl HistoryViewCornerButtonsView {
    pub fn new() -> Self {
        Self {
            down_shown: false,
            down_counter: None,
            mentions_shown: false,
            mentions_counter: None,
            reactions_shown: false,
            reactions_counter: None,
            poll_votes_shown: false,
            stash_shown: false,
            reply_returns: Vec::new(),
        }
    }

    pub fn stash_duration_ms(&self) -> u64 {
        K_STASH_DURATION_MS
    }

    pub fn push_reply_return(&mut self, item_id: u64) {
        self.reply_returns.push(item_id);
    }

    pub fn pop_reply_return(&mut self) -> Option<u64> {
        self.reply_returns.pop()
    }

    pub fn clear_reply_returns(&mut self) {
        self.reply_returns.clear();
    }

    pub fn update_visibility(&mut self, button: CornerButtonType, shown: bool) {
        match button {
            CornerButtonType::Down => self.down_shown = shown,
            CornerButtonType::Mentions => self.mentions_shown = shown,
            CornerButtonType::Reactions => self.reactions_shown = shown,
            CornerButtonType::PollVotes => self.poll_votes_shown = shown,
            CornerButtonType::Stash => self.stash_shown = shown,
        }
    }
}

impl Default for HistoryViewCornerButtonsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewCornerButtonsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut col = div()
            .flex()
            .flex_col()
            .items_end()
            .gap(px(8.0))
            .p(px(8.0));

        if self.stash_shown {
            col = col.child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0xFFFFFF))
                    .shadow_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("💾"),
            );
        }

        if self.poll_votes_shown {
            col = col.child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0xFFFFFF))
                    .shadow_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("📊"),
            );
        }

        if self.reactions_shown {
            col = col.child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0xFFFFFF))
                    .shadow_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("❤️"),
            );
        }

        if self.mentions_shown {
            col = col.child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0xFFFFFF))
                    .shadow_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("@"),
            );
        }

        if self.down_shown {
            col = col.child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0xFFFFFF))
                    .shadow_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("↓"),
            );
        }

        col
    }
}
