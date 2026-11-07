//! Authentic HistoryView Element Base View Component (`history_view_element.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_element.cpp:L82-L3970`
//! Mandate #18 Citations:
//! - `kAttachMessageToPreviousSecondsDelta = 900` (`history_view_element.cpp:L105`)
//! - `kMaxShownLine = 1048576` (`history_view_element.cpp:L106`)

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::history_element::{
    ElementChatMode, ElementContext, ElementFlags, ElementMargins, HistoryElementBase,
    K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA, K_MAX_SHOWN_LINE,
};

/// Authentic GPUI wrapper for HistoryView::Element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewElementView {
    pub base: HistoryElementBase,
    pub is_under_cursor: bool,
    pub pending_resize: bool,
    pub date_tooltip: Option<String>,
}

impl HistoryViewElementView {
    pub fn new() -> Self {
        Self {
            base: HistoryElementBase::new(0, 0, 0, false),
            is_under_cursor: false,
            pending_resize: false,
            date_tooltip: None,
        }
    }

    pub fn with_item(item_id: u64, sender_id: u64, date: i64, is_outgoing: bool) -> Self {
        Self {
            base: HistoryElementBase::new(item_id, sender_id, date, is_outgoing),
            is_under_cursor: false,
            pending_resize: false,
            date_tooltip: None,
        }
    }

    pub fn set_pending_resize(&mut self, pending: bool) {
        self.pending_resize = pending;
        self.base.flags.set(ElementFlags::NEEDS_RESIZE, pending);
    }

    pub fn set_under_cursor(&mut self, under: bool) {
        self.is_under_cursor = under;
    }

    pub fn set_attached_to_previous(&mut self, attached: bool) {
        self.base.set_attached_to_previous(attached);
    }

    pub fn set_attached_to_next(&mut self, attached: bool) {
        self.base.set_attached_to_next(attached);
    }
}

impl Default for HistoryViewElementView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewElementView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let rounding = self.base.bubble_rounding;
        let margins = self.base.margins;

        div()
            .flex()
            .flex_col()
            .w_full()
            .mt(px(margins.top))
            .mb(px(margins.bottom))
            .pl(px(margins.left))
            .pr(px(margins.right))
            .rounded_tl(px(rounding.top_left))
            .rounded_tr(px(rounding.top_right))
            .rounded_bl(px(rounding.bottom_left))
            .rounded_br(px(rounding.bottom_right))
    }
}
