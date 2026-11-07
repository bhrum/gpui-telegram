//! Session-scoped UI presentation context for modal dialogs and toasts
//! Upstream: Telegram/SourceFiles/main/session/session_show.h, session_show.cpp
//! Subsystem: main | Feature ID: TCG-N02 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// UI presentation delegate bound to an active Telegram session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionShowSession {
    pub user_id: u64,
    pub is_frozen: bool,
    pub appeal_url: String,
    pub toast_history: Vec<String>,
    pub active_box: Option<String>,
}

impl Default for SessionShowSession {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl SessionShowSession {
    /// Creates a new presentation context for a session user.
    pub fn new(user_id: u64, is_frozen: bool) -> Self {
        Self {
            user_id,
            is_frozen,
            appeal_url: "https://telegram.org/support".into(),
            toast_history: Vec::new(),
            active_box: None,
        }
    }

    /// Checks if the session is currently frozen.
    pub fn is_frozen(&self) -> bool {
        self.is_frozen
    }

    /// Sets the frozen state and appeal link.
    pub fn set_frozen(&mut self, frozen: bool, appeal_url: Option<String>) {
        self.is_frozen = frozen;
        if let Some(url) = appeal_url {
            self.appeal_url = url;
        }
    }

    /// Displays a frozen error toast/dialog if the account is frozen.
    /// Returns `true` if an error was triggered and interaction blocked.
    /// In accordance with C++ `SessionShow::showFrozenError`:
    pub fn show_frozen_error(&mut self) -> bool {
        if self.is_frozen {
            self.show_toast(&format!(
                "This account is temporarily frozen. You can submit an appeal at: {}",
                self.appeal_url
            ));
            true
        } else {
            false
        }
    }

    /// Displays a temporary floating toast message.
    pub fn show_toast(&mut self, text: &str) {
        self.toast_history.push(text.to_string());
    }

    /// Returns the most recently presented toast message.
    pub fn last_toast(&self) -> Option<&str> {
        self.toast_history.last().map(|s| s.as_str())
    }

    /// Opens a modal dialog box by descriptor name.
    pub fn show_box(&mut self, box_name: &str) {
        self.active_box = Some(box_name.to_string());
    }

    /// Dismisses any active modal dialog box.
    pub fn hide_box(&mut self) {
        self.active_box = None;
    }

    /// Returns true if a modal box is currently displayed.
    pub fn is_box_shown(&self) -> bool {
        self.active_box.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_show_frozen_toast() {
        let mut show = SessionShowSession::new(1001, false);
        assert!(!show.show_frozen_error());

        show.set_frozen(true, Some("https://t.me/appeal".into()));
        assert!(show.show_frozen_error());
        assert!(show.last_toast().unwrap().contains("https://t.me/appeal"));

        show.show_box("ConfirmLogoutBox");
        assert!(show.is_box_shown());
        show.hide_box();
        assert!(!show.is_box_shown());
    }
}
