//! Message preview rendering for dialog row last message snippet.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_message_view.cpp:L1-640`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogMessageMediaBadge {
    None,
    Photo,
    Video,
    Document,
    Audio,
    Voice,
    Location,
    Poll,
    Contact,
    Game,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsMessageViewUi {
    pub sender_name: Option<String>,
    pub text: String,
    pub is_draft: bool,
    pub media_badge: DialogMessageMediaBadge,
}

impl Default for DialogsMessageViewUi {
    fn default() -> Self {
        Self {
            sender_name: None,
            text: String::new(),
            is_draft: false,
            media_badge: DialogMessageMediaBadge::None,
        }
    }
}

impl DialogsMessageViewUi {
    pub fn new(text: String) -> Self {
        Self {
            sender_name: None,
            text,
            is_draft: false,
            media_badge: DialogMessageMediaBadge::None,
        }
    }

    pub fn as_draft(draft_text: String) -> Self {
        Self {
            sender_name: None,
            text: draft_text,
            is_draft: true,
            media_badge: DialogMessageMediaBadge::None,
        }
    }
}
