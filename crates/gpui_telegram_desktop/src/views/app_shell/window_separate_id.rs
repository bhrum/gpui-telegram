//! WindowSeparateId: Unique identifier for detached multi-window peer, media, and forum views
//! Upstream: Telegram/SourceFiles/window/window_separate_id.h, window_separate_id.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeparateType {
    Primary,
    Chat,
    Media,
    Forum,
    SharedMedia,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowSeparateId {
    pub separate_type: SeparateType,
    pub peer_id: Option<u64>,
    pub topic_id: Option<i64>,
    pub msg_id: Option<i64>,
    pub account_id: u32,
    pub folder_id: Option<i32>,
}

impl WindowSeparateId {
    pub fn primary(account_id: u32) -> Self {
        Self {
            separate_type: SeparateType::Primary,
            peer_id: None,
            topic_id: None,
            msg_id: None,
            account_id,
            folder_id: None,
        }
    }

    pub fn for_chat(peer_id: u64, account_id: u32) -> Self {
        Self {
            separate_type: SeparateType::Chat,
            peer_id: Some(peer_id),
            topic_id: None,
            msg_id: None,
            account_id,
            folder_id: None,
        }
    }

    pub fn for_topic(channel_id: u64, topic_id: i64, account_id: u32) -> Self {
        Self {
            separate_type: SeparateType::Forum,
            peer_id: Some(channel_id),
            topic_id: Some(topic_id),
            msg_id: None,
            account_id,
            folder_id: None,
        }
    }

    pub fn for_media(msg_id: i64, account_id: u32) -> Self {
        Self {
            separate_type: SeparateType::Media,
            peer_id: None,
            topic_id: None,
            msg_id: Some(msg_id),
            account_id,
            folder_id: None,
        }
    }

    pub fn is_primary(&self) -> bool {
        self.separate_type == SeparateType::Primary
    }

    pub fn matches_peer(&self, peer: u64) -> bool {
        self.peer_id == Some(peer)
    }
}

impl Default for WindowSeparateId {
    fn default() -> Self {
        Self::primary(0)
    }
}

impl Render for WindowSeparateId {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SeparateId: {:?} (Acc: {})", self.separate_type, self.account_id))
    }
}
