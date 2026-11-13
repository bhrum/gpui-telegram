//! Stash hint toast manager prompting users when long drafts are cut/cleared.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutMemory {
    pub text_hash: u64,
    pub length: usize,
    pub timestamp_ms: u64,
}

pub struct HistoryViewComposeStashHintViewControls {
    pub memory: Option<CutMemory>,
    pub peak_length: usize,
    pub is_toast_shown: bool,
}

impl HistoryViewComposeStashHintViewControls {
    pub fn new() -> Self {
        Self {
            memory: None,
            peak_length: 0,
            is_toast_shown: false,
        }
    }

    /// Track input changes: if user cuts a significant message (> 50 chars), record memory and offer restore
    /// Upstream: `StashHintManager::trackChange` (`history_view_compose_stash_hint.cpp:L35-60`)
    pub fn track_change(&mut self, current_length: usize, hash: u64, now_ms: u64) {
        if current_length > self.peak_length {
            self.peak_length = current_length;
        } else if self.peak_length > 50 && current_length == 0 {
            self.memory = Some(CutMemory {
                text_hash: hash,
                length: self.peak_length,
                timestamp_ms: now_ms,
            });
            self.is_toast_shown = true;
        }
    }

    pub fn dismiss_toast(&mut self) {
        self.is_toast_shown = false;
    }
}

impl Default for HistoryViewComposeStashHintViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeStashHintViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_toast_shown {
            return div().into_any_element();
        }

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1p5()
            .bg(rgb(0x18222d))
            .border_1()
            .border_color(rgb(0x2481cc))
            .rounded_md()
            .text_xs()
            .text_color(rgb(0xffffff))
            .child("Draft cleared. Restore?")
            .child(
                div()
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .bg(rgb(0x2481cc))
                    .child("Restore"),
            )
            .into_any_element()
    }
}
