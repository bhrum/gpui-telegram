//! Genuine History View Ephemeral Plate Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.h:L20-L54`
//! - `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.cpp:L22-L58`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::location::{K_MAX_LINES, K_FIT_SLACK};

#[derive(Debug, Clone)]
pub struct HistoryViewEphemeralPlateViewMedia {
    pub period_seconds: u32,
    pub receiver_username: Option<String>,
}

impl HistoryViewEphemeralPlateViewMedia {
    pub fn new(period_seconds: u32) -> Self {
        Self {
            period_seconds,
            receiver_username: None,
        }
    }

    pub fn with_receiver(mut self, username: impl Into<String>) -> Self {
        self.receiver_username = Some(username.into());
        self
    }
}

impl Default for HistoryViewEphemeralPlateViewMedia {
    fn default() -> Self {
        Self::new(86400) // 24 hours
    }
}

impl Render for HistoryViewEphemeralPlateViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let period_label = if self.period_seconds >= 86400 {
            format!("{}d", self.period_seconds / 86400)
        } else if self.period_seconds >= 3600 {
            format!("{}h", self.period_seconds / 3600)
        } else {
            format!("{}m", self.period_seconds / 60)
        };

        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(rgba(0x00000044))
            .text_size(px(11.0))
            .text_color(rgb(0x8A9AA8))
            .child(div().child("⏱️"))
            .child(div().child(format!("Auto-delete in {}", period_label)))
    }
}
