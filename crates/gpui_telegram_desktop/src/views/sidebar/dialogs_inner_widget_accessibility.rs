//! Screen reader accessibility text formatting for Dialogs rows.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget_accessibility.h:L1-70`
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget_accessibility.cpp:L35-120`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

/// Upstream constant from `dialogs_inner_widget_accessibility.cpp:L87`.
pub const K_MAX_MESSAGE_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerChatType {
    User,
    SavedMessages,
    Service,
    Bot,
    Group,
    Channel,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityDeliveryStatus {
    Sending,
    Failed,
    NotSeen,
    Seen,
    Received,
}

/// Helper struct for synthesizing screen reader announcements (`dialogs_inner_widget_accessibility.cpp:L54-100`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogsInnerWidgetAccessibility {
    pub peer_name: String,
    pub chat_type: PeerChatType,
    pub delivery_status: AccessibilityDeliveryStatus,
    pub message_text: String,
    pub unread_count: u32,
    pub is_muted: bool,
    pub is_pinned: bool,
}

impl DialogsInnerWidgetAccessibility {
    pub fn new(peer_name: String) -> Self {
        Self {
            peer_name,
            chat_type: PeerChatType::User,
            delivery_status: AccessibilityDeliveryStatus::Received,
            message_text: String::new(),
            unread_count: 0,
            is_muted: false,
            is_pinned: false,
        }
    }

    /// Truncates message body to maximum 100 characters per upstream `kMaxMessageLength`.
    pub fn format_message_snippet(text: &str) -> String {
        if text.len() > K_MAX_MESSAGE_LENGTH {
            format!("{}...", &text[..K_MAX_MESSAGE_LENGTH])
        } else {
            text.to_string()
        }
    }

    /// Builds the complete spoken description for screen readers.
    pub fn build_accessible_announcement(&self) -> String {
        let mut parts = Vec::new();
        parts.push(self.peer_name.clone());

        match self.chat_type {
            PeerChatType::SavedMessages => parts.push("Saved Messages".to_string()),
            PeerChatType::Bot => parts.push("bot".to_string()),
            PeerChatType::Group => parts.push("group".to_string()),
            PeerChatType::Channel => parts.push("channel".to_string()),
            PeerChatType::Service => parts.push("service notification".to_string()),
            PeerChatType::Deleted => parts.push("deleted account".to_string()),
            PeerChatType::User => {}
        }

        if self.is_pinned {
            parts.push("pinned".to_string());
        }

        if self.is_muted {
            parts.push("muted".to_string());
        }

        if self.unread_count > 0 {
            parts.push(format!("{} unread messages", self.unread_count));
        }

        let snippet = Self::format_message_snippet(&self.message_text);
        if !snippet.is_empty() {
            parts.push(snippet);
        }

        parts.join(", ")
    }
}
