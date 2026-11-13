//! NotificationsManager: Core notification dispatching, muting, sound triggers, and badge counter
//! Upstream: Telegram/SourceFiles/window/notifications_manager.h, notifications_manager.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationItem {
    pub id: u64,
    pub peer_id: u64,
    pub title: String,
    pub body: String,
    pub play_sound: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct NotificationsManager {
    pub notifications: Vec<NotificationItem>,
    pub sounds_enabled: bool,
    pub previews_enabled: bool,
    pub total_badge_count: i32,
    next_id: u64,
}

impl NotificationsManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            sounds_enabled: true,
            previews_enabled: true,
            total_badge_count: 0,
            next_id: 1,
        }
    }

    pub fn schedule_notification(
        &mut self,
        peer_id: u64,
        title: impl Into<String>,
        body: impl Into<String>,
        sound: bool,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.notifications.push(NotificationItem {
            id,
            peer_id,
            title: title.into(),
            body: body.into(),
            play_sound: sound && self.sounds_enabled,
            timestamp: 0,
        });
        self.total_badge_count += 1;
        id
    }

    pub fn dismiss(&mut self, notification_id: u64) {
        self.notifications.retain(|n| n.id != notification_id);
    }

    pub fn clear_for_peer(&mut self, peer_id: u64) {
        let before = self.notifications.len();
        self.notifications.retain(|n| n.peer_id != peer_id);
        let removed = before - self.notifications.len();
        self.total_badge_count = self.total_badge_count.saturating_sub(removed as i32);
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
        self.total_badge_count = 0;
    }
}

impl Default for NotificationsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NotificationsManager {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("Notifications: active={}", self.notifications.len()))
    }
}
