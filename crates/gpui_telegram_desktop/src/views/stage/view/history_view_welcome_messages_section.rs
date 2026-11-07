//! History View Welcome Messages Section View Component (`history_view_welcome_messages_section`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.h` (L1-L290)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.cpp` (L1-L1265)

/// Welcome greeting message item
#[derive(Debug, Clone, PartialEq)]
pub struct WelcomeMessageItem {
    pub id: i64,
    pub text: String,
    pub is_away_greeting: bool,
    pub recipient_filter: String,
}

/// GPUI view component for business welcome & away messages section
#[derive(Debug, Clone)]
pub struct HistoryViewWelcomeMessagesSectionView {
    pub peer_id: i64,
    pub messages: Vec<WelcomeMessageItem>,
    pub is_enabled: bool,
    pub empty_placeholder_text: String,
}

impl HistoryViewWelcomeMessagesSectionView {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            messages: Vec::new(),
            is_enabled: true,
            empty_placeholder_text: "No welcome messages configured.".to_string(),
        }
    }

    pub fn set_messages(&mut self, list: Vec<WelcomeMessageItem>) {
        self.messages = list;
    }

    pub fn count(&self) -> usize {
        self.messages.len()
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.is_enabled = !self.is_enabled;
        self.is_enabled
    }
}
