//! macOS Modern UNUserNotificationCenter Native Notifications Manager
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

/// Modern macOS 10.14+ UNUserNotificationCenter Manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsManagerMacUnMac {
    pub manager_id: u64,
    pub modern_notifications_enabled: bool,
    pub in_flight_requests: HashMap<String, UNInFlightRequest>,
    pub replied_messages: Vec<(UNNotificationParsed, String)>,
    pub read_messages: Vec<UNNotificationParsed>,
    pub presentation_options: u32,
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
        }
    }

    pub fn use_un_manager(&self, macos_version_major: u32) -> bool {
        self.modern_notifications_enabled && macos_version_major >= 14
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

    pub fn clear_from_session(&mut self, session_id: u64) -> Result<usize> {
        let prev = self.in_flight_requests.len();
        self.in_flight_requests.retain(|_, r| r.parsed.session_id != session_id);
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
        let mut un = NotificationsManagerMacUnMac::new(100);
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
}
