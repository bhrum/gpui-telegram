//! Paid Reaction Toast View Wrapper
//! Faithful upstream C++ extraction:
//! - `history_view_paid_reaction_toast.cpp:L35-L80`

use gpui::*;
use crate::views::stage::reactions::paid_toast::{PaidReactionToast, UNDO_COUNTDOWN_TOTAL_MS, PAID_REACTION_STAR_TIERS};

#[derive(Debug, Clone)]
pub struct HistoryViewPaidReactionToastView {
    pub inner: PaidReactionToast,
    pub is_visible: bool,
}

impl HistoryViewPaidReactionToastView {
    pub fn new(message_id: u64, stars: u64) -> Self {
        Self {
            inner: PaidReactionToast::new(message_id, stars, false),
            is_visible: true,
        }
    }

    pub fn tick(&mut self, elapsed_ms: i64) {
        if !self.inner.tick_countdown(elapsed_ms) {
            self.is_visible = false;
        }
    }

    pub fn undo(&mut self) {
        self.inner.undo();
        self.is_visible = false;
    }
}

impl Default for HistoryViewPaidReactionToastView {
    fn default() -> Self {
        Self::new(0, 50)
    }
}

impl Render for HistoryViewPaidReactionToastView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let secs = self.inner.seconds_left();
        let stars = self.inner.stars_count;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(14.0))
            .py(px(8.0))
            .bg(rgba(0x111827EE))
            .rounded(px(16.0))
            .shadow_lg()
            .text_size(px(13.0))
            .text_color(rgb(0xFFFFFF))
            .child(format!("⭐ Sent {} Stars! (Undo: {}s)", stars, secs))
            .into_any_element()
    }
}
