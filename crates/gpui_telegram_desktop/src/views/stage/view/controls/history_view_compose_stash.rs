//! Draft stashing manager for swapping temporary input drafts and send-file boxes.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_stash.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_stash.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposeStashEntry {
    pub key: String,
    pub text: String,
    pub reply_msg_id: Option<i32>,
    pub file_paths: Vec<String>,
}

pub struct HistoryViewComposeStashViewControls {
    pub stashes: Vec<ComposeStashEntry>,
    pub current_stash: Option<ComposeStashEntry>,
}

impl HistoryViewComposeStashViewControls {
    pub fn new() -> Self {
        Self {
            stashes: Vec::new(),
            current_stash: None,
        }
    }

    pub fn can_exchange(&self) -> bool {
        self.current_stash.is_some() || !self.stashes.is_empty()
    }

    pub fn stash_draft(&mut self, key: impl Into<String>, text: impl Into<String>, reply_msg_id: Option<i32>, file_paths: Vec<String>) {
        let entry = ComposeStashEntry {
            key: key.into(),
            text: text.into(),
            reply_msg_id,
            file_paths,
        };
        self.stashes.push(entry);
    }

    pub fn pop_stash(&mut self, key: &str) -> Option<ComposeStashEntry> {
        if let Some(pos) = self.stashes.iter().position(|s| s.key == key) {
            Some(self.stashes.remove(pos))
        } else {
            None
        }
    }
}

impl Default for HistoryViewComposeStashViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeStashViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.stashes.len();

        div()
            .flex()
            .items_center()
            .gap_1()
            .text_xs()
            .text_color(rgb(0x8e8e93))
            .child(format!("Stashed drafts: {}", count))
    }
}
