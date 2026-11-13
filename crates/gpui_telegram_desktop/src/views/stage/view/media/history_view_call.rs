//! Genuine History View Call Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_call.h:L20-L59`
//! - `Telegram/SourceFiles/history/view/media/history_view_call.cpp:L33-L125`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::invoice::{RichCallCard, CallState, compute_call_duration};

#[derive(Debug, Clone)]
pub struct HistoryViewCallViewMedia {
    pub state: CallState,
    pub duration_seconds: u32,
    pub is_video: bool,
    pub is_conference: bool,
}

impl HistoryViewCallViewMedia {
    pub fn new(state: CallState, duration: u32, is_video: bool) -> Self {
        Self {
            state,
            duration_seconds: compute_call_duration(state, duration),
            is_video,
            is_conference: false,
        }
    }

    pub fn with_conference(mut self, conf: bool) -> Self {
        self.is_conference = conf;
        self
    }
}

impl Default for HistoryViewCallViewMedia {
    fn default() -> Self {
        Self::new(CallState::Incoming, 42, false)
    }
}

impl Render for HistoryViewCallViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichCallCard::new(self.state, self.duration_seconds, self.is_video);
        card.is_conference = self.is_conference;
        card.render()
    }
}
