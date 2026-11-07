//! Top controls coordinator managing pinned message banners, group call bars, and join requests.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_top_controls.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_top_controls.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedMessageState {
    pub msg_id: i32,
    pub sender_name: String,
    pub text_snippet: String,
    pub is_pinned_list_shown: bool,
}

pub struct HistoryViewTopControlsViewControls {
    pub pinned: Option<PinnedMessageState>,
    pub group_call_active: bool,
    pub join_requests_count: usize,
    pub height_px: i32,
    pub is_visible: bool,
}

impl HistoryViewTopControlsViewControls {
    pub fn new() -> Self {
        Self {
            pinned: None,
            group_call_active: false,
            join_requests_count: 0,
            height_px: 0,
            is_visible: false,
        }
    }

    pub fn set_pinned(&mut self, msg_id: i32, sender: impl Into<String>, snippet: impl Into<String>) {
        self.pinned = Some(PinnedMessageState {
            msg_id,
            sender_name: sender.into(),
            text_snippet: snippet.into(),
            is_pinned_list_shown: false,
        });
        self.recompute_height();
    }

    pub fn clear_pinned(&mut self) {
        self.pinned = None;
        self.recompute_height();
    }

    pub fn set_group_call(&mut self, active: bool) {
        self.group_call_active = active;
        self.recompute_height();
    }

    pub fn set_join_requests(&mut self, count: usize) {
        self.join_requests_count = count;
        self.recompute_height();
    }

    fn recompute_height(&mut self) {
        let mut h = 0;
        if self.pinned.is_some() {
            h += 38;
        }
        if self.group_call_active {
            h += 40;
        }
        if self.join_requests_count > 0 {
            h += 36;
        }
        self.height_px = h;
        self.is_visible = h > 0;
    }
}

impl Default for HistoryViewTopControlsViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewTopControlsViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let pinned_info = self.pinned.as_ref().map(|p| {
            format!("Pinned Message #{}: {}", p.msg_id, p.text_snippet)
        });

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x18222d))
            .border_b_1()
            .border_color(rgb(0x2a394a));

        if self.group_call_active {
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h_10()
                    .px_3()
                    .bg(rgb(0x1e3b2e))
                    .text_xs()
                    .text_color(rgb(0x4cd964))
                    .child("Live Video Chat Active")
                    .child("Join"),
            );
        }

        if let Some(info) = pinned_info {
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h_9()
                    .px_3()
                    .border_l_2()
                    .border_color(rgb(0x2481cc))
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(info)
                    .child(div().text_color(rgb(0x8e8e93)).child("✕")),
            );
        }

        root.into_any_element()
    }
}
