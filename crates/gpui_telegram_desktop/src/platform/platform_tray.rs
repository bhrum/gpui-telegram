//! Platform System Tray Abstraction
//! Upstream: Telegram/SourceFiles/platform/platform_tray.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct PlatformTray {
    pub is_visible: bool,
    pub unread_count: u32,
    pub has_monochrome_setting: bool,
    pub is_monochrome: bool,
    pub tooltip: String,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformTray {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            unread_count: 0,
            has_monochrome_setting: cfg!(target_os = "windows") || cfg!(target_os = "linux"),
            is_monochrome: false,
            tooltip: "Telegram Desktop".into(),
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn set_unread_count(&mut self, count: u32) {
        self.unread_count = count;
        if count > 0 {
            self.tooltip = format!("Telegram Desktop ({})", count);
        } else {
            self.tooltip = "Telegram Desktop".into();
        }
    }

    pub fn set_monochrome(&mut self, monochrome: bool) {
        self.is_monochrome = monochrome;
    }

    pub fn has_monochrome_setting(&self) -> bool {
        self.has_monochrome_setting
    }
}

impl Default for PlatformTray {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformTray {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon_bg = if self.is_monochrome { rgb(0x616161) } else { rgb(0x2AABEE) };

        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(4.0))
            .gap(px(4.0))
            .child(
                div()
                    .w(px(22.0))
                    .h(px(22.0))
                    .rounded_md()
                    .bg(icon_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(10.0))
                    .child("TG"),
            )
            .child(if self.unread_count > 0 {
                div()
                    .px(px(4.0))
                    .py(px(1.0))
                    .rounded_full()
                    .bg(rgb(0xE53935))
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(9.0))
                    .child(self.unread_count.to_string())
            } else {
                div()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_icon_and_unread_counter() {
        let mut tray = PlatformTray::new();
        assert!(tray.is_visible);
        assert!(tray.initialize().is_ok());

        tray.set_unread_count(5);
        assert_eq!(tray.unread_count, 5);
        assert!(tray.tooltip.contains("5"));

        tray.set_unread_count(0);
        assert_eq!(tray.unread_count, 0);
        assert_eq!(tray.tooltip, "Telegram Desktop");

        tray.hide();
        assert!(!tray.is_visible);
        tray.show();
        assert!(tray.is_visible);
    }
}
