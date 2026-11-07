//! macOS Legacy NSUserNotificationCenter Native Notifications Manager
//! Upstream Source: Telegram/SourceFiles/platform/mac/notifications_manager_mac.h, Telegram/SourceFiles/platform/mac/notifications_manager_mac.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Asynchronous notification clearing tasks matching upstream `ClearTask`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacClearTask {
    ClearFromItem { msg_id: i64 },
    ClearFromTopic { context_id: u64, topic_id: i64 },
    ClearFromSublist { context_id: u64, sublist_id: i64 },
    ClearFromHistory { peer_id: u64 },
    ClearFromSession { session_id: u64 },
    ClearAll,
    ClearFinish,
}

/// Notification payload for native macOS notification center.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacNotificationInfo {
    pub notification_id: u64,
    pub session_id: u64,
    pub peer_id: u64,
    pub topic_id: Option<i64>,
    pub msg_id: i64,
    pub title: String,
    pub subtitle: String,
    pub message: String,
    pub sound_name: Option<String>,
    pub userpic_path: Option<String>,
}

/// Legacy macOS NSUserNotificationCenter Manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsManagerMacMac {
    pub manager_id: u64,
    pub delivered_notifications: HashMap<u64, MacNotificationInfo>,
    pub clearing_queue: VecDeque<MacClearTask>,
    pub dnd_enabled: bool,
    pub last_settings_query_ms: u64,
    pub sound_cache_dir: String,
}

impl NotificationsManagerMacMac {
    pub const QUERY_SETTINGS_EACH_MS: u64 = 1000;
    pub const CACHE_EXPIRATION_WEEKS: u32 = 5;

    pub fn new(manager_id: u64) -> Self {
        Self {
            manager_id,
            delivered_notifications: HashMap::new(),
            clearing_queue: VecDeque::new(),
            dnd_enabled: false,
            last_settings_query_ms: 0,
            sound_cache_dir: "~/Library/Sounds".into(),
        }
    }

    pub fn show_native_notification(&mut self, info: MacNotificationInfo) -> Result<u64> {
        let id = info.notification_id;
        self.delivered_notifications.insert(id, info);
        Ok(id)
    }

    pub fn clear_from_item(&mut self, msg_id: i64) -> Result<()> {
        self.clearing_queue.push_back(MacClearTask::ClearFromItem { msg_id });
        self.delivered_notifications.retain(|_, n| n.msg_id != msg_id);
        Ok(())
    }

    pub fn clear_from_history(&mut self, peer_id: u64) -> Result<()> {
        self.clearing_queue.push_back(MacClearTask::ClearFromHistory { peer_id });
        self.delivered_notifications.retain(|_, n| n.peer_id != peer_id);
        Ok(())
    }

    pub fn clear_from_session(&mut self, session_id: u64) -> Result<()> {
        self.clearing_queue.push_back(MacClearTask::ClearFromSession { session_id });
        self.delivered_notifications.retain(|_, n| n.session_id != session_id);
        Ok(())
    }

    pub fn clear_all_fast(&mut self) -> Result<()> {
        self.clearing_queue.push_back(MacClearTask::ClearAll);
        self.delivered_notifications.clear();
        Ok(())
    }

    pub fn process_clearing_tasks(&mut self) -> usize {
        let count = self.clearing_queue.len();
        self.clearing_queue.clear();
        count
    }

    pub fn query_dnd_state(&mut self, now_ms: u64, system_dnd: bool) -> bool {
        if self.last_settings_query_ms > 0
            && now_ms <= self.last_settings_query_ms + Self::QUERY_SETTINGS_EACH_MS
        {
            return self.dnd_enabled;
        }
        self.last_settings_query_ms = now_ms;
        self.dnd_enabled = system_dnd;
        self.dnd_enabled
    }

    pub fn account_name_separator(&self) -> &'static str {
        " -> "
    }

    pub fn do_skip_toast(&self) -> bool {
        false
    }
}

impl Default for NotificationsManagerMacMac {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_notification_lifecycle() {
        let mut mgr = NotificationsManagerMacMac::new(42);

        let n1 = MacNotificationInfo {
            notification_id: 101,
            session_id: 1,
            peer_id: 2001,
            topic_id: None,
            msg_id: 555,
            title: "Alice".into(),
            subtitle: "".into(),
            message: "Hello".into(),
            sound_name: None,
            userpic_path: None,
        };
        mgr.show_native_notification(n1).unwrap();
        assert_eq!(mgr.delivered_notifications.len(), 1);

        mgr.clear_from_item(555).unwrap();
        assert_eq!(mgr.delivered_notifications.len(), 0);
        assert_eq!(mgr.process_clearing_tasks(), 1);

        assert_eq!(mgr.query_dnd_state(100, true), true);
        // within 1000ms throttled
        assert_eq!(mgr.query_dnd_state(500, false), true);
        // after 1000ms updated
        assert_eq!(mgr.query_dnd_state(1200, false), false);
    }
}
