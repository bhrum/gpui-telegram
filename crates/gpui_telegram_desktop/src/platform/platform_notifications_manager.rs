//! Platform Notifications Manager Abstraction
//! Upstream: Telegram/SourceFiles/platform/platform_notifications_manager.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct PlatformNotificationsManager {
    pub native_supported: bool,
    pub sound_enabled: bool,
    pub flash_bounce_enabled: bool,
    pub volume_supported: bool,
    pub enforced: bool,
    pub by_default: bool,
    pub unread_notifications_count: usize,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformNotificationsManager {
    pub fn new() -> Self {
        Self {
            native_supported: true,
            sound_enabled: true,
            flash_bounce_enabled: true,
            volume_supported: true,
            enforced: false,
            by_default: true,
            unread_notifications_count: 0,
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        }
    }

    /// Upstream method: `Create`
    pub fn create(&mut self) -> Result<()> {
        self.is_enabled = true;
        Ok(())
    }

    /// Upstream method: `SkipToastForCustom`
    pub fn skip_toast_for_custom(&self) -> bool {
        false
    }

    /// Upstream method: `MaybePlaySoundForCustom`
    pub fn maybe_play_sound_for_custom<F: FnOnce()>(&self, play_sound: F) {
        if self.sound_enabled {
            play_sound();
        }
    }

    /// Upstream method: `MaybeFlashBounceForCustom`
    pub fn maybe_flash_bounce_for_custom<F: FnOnce()>(&self, flash_bounce: F) {
        if self.flash_bounce_enabled {
            flash_bounce();
        }
    }

    /// Upstream method: `WaitForInputForCustom`
    pub fn wait_for_input_for_custom(&self) -> bool {
        false
    }

    /// Upstream method: `Supported`
    pub fn supported(&self) -> bool {
        self.native_supported
    }

    /// Upstream method: `Enforced`
    pub fn enforced(&self) -> bool {
        self.enforced
    }

    /// Upstream method: `ByDefault`
    pub fn by_default(&self) -> bool {
        self.by_default
    }

    /// Upstream method: `VolumeSupported`
    pub fn volume_supported(&self) -> bool {
        self.volume_supported
    }

    pub fn push_notification(&mut self) {
        self.unread_notifications_count += 1;
    }

    pub fn clear_notifications(&mut self) {
        self.unread_notifications_count = 0;
    }
}

impl Default for PlatformNotificationsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformNotificationsManager {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(8.0))
            .gap(px(6.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_md()
            .child(
                div()
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded_full()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(10.0))
                    .child(self.unread_notifications_count.to_string()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .child(format!("Notifications ({}) — Sound: {}", self.platform_name, self.sound_enabled)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications_manager_logic() {
        let mut mgr = PlatformNotificationsManager::new();
        assert!(mgr.supported());
        assert!(!mgr.enforced());
        assert!(mgr.by_default());
        assert!(!mgr.skip_toast_for_custom());

        let mut sound_played = false;
        mgr.maybe_play_sound_for_custom(|| sound_played = true);
        assert!(sound_played);

        mgr.push_notification();
        assert_eq!(mgr.unread_notifications_count, 1);
        mgr.clear_notifications();
        assert_eq!(mgr.unread_notifications_count, 0);
    }
}
