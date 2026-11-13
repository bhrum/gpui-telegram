//! WindowChatSwitchProcess: Coordinates draft persistence and message slice loading during chat switches
//! Upstream: Telegram/SourceFiles/window/window_chat_switch_process.h, window_chat_switch_process.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowChatSwitchProcess {
    pub from_peer_id: Option<u64>,
    pub to_peer_id: u64,
    pub is_in_progress: bool,
    pub pending_draft_saved: bool,
}

impl WindowChatSwitchProcess {
    pub fn new() -> Self {
        Self {
            from_peer_id: None,
            to_peer_id: 0,
            is_in_progress: false,
            pending_draft_saved: false,
        }
    }

    pub fn begin_switch(&mut self, from: Option<u64>, to: u64) {
        self.from_peer_id = from;
        self.to_peer_id = to;
        self.is_in_progress = true;
        self.pending_draft_saved = false;
    }

    pub fn mark_draft_saved(&mut self) {
        self.pending_draft_saved = true;
    }

    pub fn complete_switch(&mut self) -> u64 {
        self.is_in_progress = false;
        self.to_peer_id
    }

    pub fn abort_switch(&mut self) {
        self.is_in_progress = false;
    }
}

impl Default for WindowChatSwitchProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowChatSwitchProcess {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("ChatSwitch: in_progress={}", self.is_in_progress))
    }
}
