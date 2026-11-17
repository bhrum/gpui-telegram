//! macOS Modern UNUserNotificationCenter Native Notifications Manager & osascript Dispatcher
//! Upstream Source: Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.h, Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed metadata from native UNNotification userInfo dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UNNotificationParsed {
    pub session_id: u64,
    pub peer_id: u64,
    pub topic_root_id: i64,
    pub monoforum_peer_id: u64,
    pub msg_id: i64,
    pub valid: bool,
}

impl UNNotificationParsed {
    pub fn new(session_id: u64, peer_id: u64, msg_id: i64) -> Self {
        Self {
            session_id,
            peer_id,
            topic_root_id: 0,
            monoforum_peer_id: 0,
            msg_id,
            valid: session_id != 0 && peer_id != 0,
        }
    }
}

/// In-flight request tracked by UNManager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UNInFlightRequest {
    pub identifier: String,
    pub parsed: UNNotificationParsed,
    pub remove_when_done: bool,
}

/// Interactive notification action type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UNNotificationAction {
    DefaultClick,
    InlineReply(String),
    MarkAsRead,
    Custom(String),
    Dismiss,
}

/// Dispatched macOS notification record
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacNotificationPayload {
    pub identifier: String,
    pub title: String,
    pub subtitle: String,
    pub message: String,
    pub sound: bool,
    pub peer_id: u64,
    pub msg_id: i64,
    pub deep_link: String,
}

/// Modern macOS 10.14+ UNUserNotificationCenter Manager & osascript Dispatcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsManagerMacUnMac {
    pub manager_id: u64,
    pub modern_notifications_enabled: bool,
    pub in_flight_requests: HashMap<String, UNInFlightRequest>,
    pub replied_messages: Vec<(UNNotificationParsed, String)>,
    pub read_messages: Vec<UNNotificationParsed>,
    pub presentation_options: u32,
    /// When true, records requests without spawning osascript child processes (for CI/tests)
    pub mock_mode: bool,
    /// Dispatched notification history for automated verification assertions
    pub dispatched_history: Vec<MacNotificationPayload>,
}

impl NotificationsManagerMacUnMac {
    pub const PRESENTATION_BANNER: u32 = 1 << 0;
    pub const PRESENTATION_LIST: u32 = 1 << 1;
    pub const PRESENTATION_SOUND: u32 = 1 << 2;

    pub fn new(manager_id: u64) -> Self {
        Self {
            manager_id,
            modern_notifications_enabled: true,
            in_flight_requests: HashMap::new(),
            replied_messages: Vec::new(),
            read_messages: Vec::new(),
            presentation_options: Self::PRESENTATION_BANNER
                | Self::PRESENTATION_LIST
                | Self::PRESENTATION_SOUND,
            mock_mode: false,
            dispatched_history: Vec::new(),
        }
    }

    pub fn with_mock_mode(mut self, mock: bool) -> Self {
        self.mock_mode = mock;
        self
    }

    pub fn use_un_manager(&self, macos_version_major: u32) -> bool {
        self.modern_notifications_enabled && macos_version_major >= 14
    }

    /// Formats AppleScript string escaping special characters and stripping NUL / control bytes
    pub fn escape_applescript_string(input: &str) -> String {
        let mut out = String::with_capacity(input.len() + 8);
        for ch in input.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' | '\r' => out.push(' '),
                '\t' => out.push('\t'),
                c if (c as u32) < 0x20 || c == '\x7f' => {}
                c => out.push(c),
            }
        }
        out
    }

    /// Builds the `display notification` AppleScript command
    pub fn build_applescript_command(
        title: &str,
        subtitle: &str,
        message: &str,
        sound: bool,
    ) -> String {
        let esc_title = Self::escape_applescript_string(title);
        let esc_sub = Self::escape_applescript_string(subtitle);
        let esc_msg = Self::escape_applescript_string(message);

        let mut script = format!(
            "display notification \"{}\" with title \"{}\"",
            esc_msg, esc_title
        );
        if !esc_sub.is_empty() {
            script.push_str(&format!(" subtitle \"{}\"", esc_sub));
        }
        if sound {
            script.push_str(" sound name \"default\"");
        }
        script
    }

    /// Dispatches authentic macOS system notification and tracks in-flight request
    pub fn show_system_notification(
        &mut self,
        title: &str,
        subtitle: &str,
        message: &str,
        sound: bool,
        parsed: UNNotificationParsed,
    ) -> Result<String> {
        let identifier = format!("tg_notif_{}_{}", parsed.peer_id, parsed.msg_id);
        let deep_link = format!("tg://chat?id={}", parsed.peer_id);

        let payload = MacNotificationPayload {
            identifier: identifier.clone(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            message: message.to_string(),
            sound,
            peer_id: parsed.peer_id,
            msg_id: parsed.msg_id,
            deep_link,
        };

        if !self.mock_mode && cfg!(target_os = "macos") {
            let script = Self::build_applescript_command(title, subtitle, message, sound);
            let mut child = std::process::Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .spawn()
                .map_err(|e| {
                    gpui_telegram_core::error::TelegramError::Internal(format!(
                        "Failed to spawn osascript notification: {e}"
                    ))
                })?;
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        }

        self.in_flight_requests.insert(
            identifier.clone(),
            UNInFlightRequest {
                identifier: identifier.clone(),
                parsed,
                remove_when_done: false,
            },
        );
        self.dispatched_history.push(payload);

        Ok(identifier)
    }

    /// Handles notification click action, returning the deep-link URL
    pub fn handle_notification_click(&mut self, identifier: &str) -> Option<String> {
        if let Some(req) = self.in_flight_requests.remove(identifier) {
            Some(format!("tg://chat?id={}", req.parsed.peer_id))
        } else {
            None
        }
    }

    pub fn schedule_notification(
        &mut self,
        identifier: impl Into<String>,
        parsed: UNNotificationParsed,
    ) -> Result<String> {
        let id = identifier.into();
        self.in_flight_requests.insert(
            id.clone(),
            UNInFlightRequest {
                identifier: id.clone(),
                parsed,
                remove_when_done: false,
            },
        );
        Ok(id)
    }

    pub fn handle_notification_action(
        &mut self,
        identifier: &str,
        action: UNNotificationAction,
    ) -> Result<bool> {
        if let Some(req) = self.in_flight_requests.remove(identifier) {
            match action {
                UNNotificationAction::InlineReply(text) => {
                    self.replied_messages.push((req.parsed, text));
                }
                UNNotificationAction::MarkAsRead => {
                    self.read_messages.push(req.parsed);
                }
                _ => {}
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn clear_from_item(&mut self, msg_id: i64) -> Result<usize> {
        let prev = self.in_flight_requests.len();
        self.in_flight_requests.retain(|_, r| r.parsed.msg_id != msg_id);
        Ok(prev - self.in_flight_requests.len())
    }

    pub fn clear_from_peer(&mut self, peer_id: u64) -> Result<usize> {
        let prev = self.in_flight_requests.len();
        self.in_flight_requests
            .retain(|_, r| r.parsed.peer_id != peer_id);
        Ok(prev - self.in_flight_requests.len())
    }

    pub fn clear_from_session(&mut self, session_id: u64) -> Result<usize> {
        let prev = self.in_flight_requests.len();
        self.in_flight_requests
            .retain(|_, r| r.parsed.session_id != session_id);
        Ok(prev - self.in_flight_requests.len())
    }

    pub fn clear_all_fast(&mut self) -> Result<()> {
        self.in_flight_requests.clear();
        Ok(())
    }
}

impl Default for NotificationsManagerMacUnMac {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_un_manager_dispatch_and_reply() {
        let mut un = NotificationsManagerMacUnMac::new(100).with_mock_mode(true);
        assert!(un.use_un_manager(14));
        assert!(!un.use_un_manager(13));

        let parsed = UNNotificationParsed::new(1, 42, 999);
        un.schedule_notification("req-999", parsed.clone()).unwrap();
        assert_eq!(un.in_flight_requests.len(), 1);

        un.handle_notification_action(
            "req-999",
            UNNotificationAction::InlineReply("Got it, thanks!".into()),
        )
        .unwrap();
        assert_eq!(un.in_flight_requests.len(), 0);
        assert_eq!(un.replied_messages.len(), 1);
        assert_eq!(un.replied_messages[0].1, "Got it, thanks!");
    }

    #[test]
    fn test_show_system_notification_and_click() {
        let mut un = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
        let parsed = UNNotificationParsed::new(1, 1001, 555);
        let id = un
            .show_system_notification("Alice", "Direct Message", "Hello world!", true, parsed)
            .unwrap();

        assert_eq!(id, "tg_notif_1001_555");
        assert_eq!(un.dispatched_history.len(), 1);
        assert_eq!(un.dispatched_history[0].deep_link, "tg://chat?id=1001");

        let link = un.handle_notification_click(&id);
        assert_eq!(link, Some("tg://chat?id=1001".to_string()));
        assert!(un.in_flight_requests.is_empty());
    }

    #[test]
    fn test_escape_applescript_string() {
        let input = "Hello \"World\"\nLine 2 \\ test";
        let escaped = NotificationsManagerMacUnMac::escape_applescript_string(input);
        assert_eq!(escaped, "Hello \\\"World\\\" Line 2 \\\\ test");
    }
}
