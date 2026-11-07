//! macOS Cocoa NSWindow Shell & Dock Tile Integration
//! Upstream: Telegram/SourceFiles/platform/mac/main_window_mac.mm, .h
//! Manages custom AppKit window titlebar, traffic light positioning, and Dock unread counters.

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowMacMac {
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
    pub dock_badge: String,
    pub custom_title_height: f32,
    pub window_title: String,
    pub is_fullscreen: bool,
    pub is_muted: bool,
    pub unread_count: u32,
}

impl MainWindowMacMac {
    pub fn new() -> Self {
        Self {
            is_supported: true,
            is_enabled: true,
            platform_name: "macOS Cocoa NSWindow".into(),
            dock_badge: String::new(),
            custom_title_height: 28.0,
            window_title: "Telegram".into(),
            is_fullscreen: false,
            is_muted: false,
            unread_count: 0,
        }
    }

    pub fn unread_counter_changed_hook(&mut self, count: u32, muted: bool) -> Result<()> {
        self.unread_count = count;
        self.is_muted = muted;
        self.update_dock_counter(count, muted)?;
        Ok(())
    }

    pub fn update_dock_counter(&mut self, count: u32, muted: bool) -> Result<()> {
        self.dock_badge = if count == 0 {
            String::new()
        } else if muted {
            "•".to_string()
        } else {
            count.to_string()
        };
        Ok(())
    }

    pub fn make_from_letters(&self, letters: &str) -> String {
        letters.chars().take(2).collect::<String>().to_uppercase()
    }

    pub fn set_window_badge(&mut self, badge: impl Into<String>) -> Result<()> {
        self.dock_badge = badge.into();
        Ok(())
    }

    pub fn global_menu_markdown_state(&self) -> bool {
        true
    }

    pub fn event_filter(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get_custom_title_height(&self) -> f32 {
        self.custom_title_height
    }

    pub fn set_custom_title_height(&mut self, h: f32) {
        self.custom_title_height = h.max(20.0);
    }

    pub fn hide_and_deactivate(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for MainWindowMacMac {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MainWindowMacMac {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let root = div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(self.custom_title_height))
            .px(px(12.0))
            .bg(rgb(0x18181B))
            .border_b_1()
            .border_color(rgb(0x27272A))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Cocoa traffic lights (Close, Minimize, Zoom)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().w(px(12.0)).h(px(12.0)).rounded_full().bg(rgb(0xFF5F56)))
                            .child(div().w(px(12.0)).h(px(12.0)).rounded_full().bg(rgb(0xFFBD2E)))
                            .child(div().w(px(12.0)).h(px(12.0)).rounded_full().bg(rgb(0x27C93F))),
                    )
                    .child(
                        div()
                            .ml(px(8.0))
                            .text_size(px(13.0))
                            .text_color(rgb(0xF4F4F5))
                            .child(self.window_title.clone()),
                    ),
            );
        let mut right_area = div()
            .flex()
            .items_center()
            .gap(px(8.0));

        if !self.dock_badge.is_empty() {
            right_area = right_area.child(
                div()
                    .px(px(6.0))
                    .py(px(1.0))
                    .rounded_full()
                    .bg(if self.is_muted { rgb(0x71717A) } else { rgb(0xEF4444) })
                    .text_size(px(10.0))
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("Dock: {}", self.dock_badge)),
            );
        }

        root.child(right_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_window_mac() {
        let mut win = MainWindowMacMac::new();
        assert_eq!(win.get_custom_title_height(), 28.0);
        assert_eq!(win.dock_badge, "");

        assert!(win.unread_counter_changed_hook(9, false).is_ok());
        assert_eq!(win.dock_badge, "9");

        assert!(win.unread_counter_changed_hook(9, true).is_ok());
        assert_eq!(win.dock_badge, "•");

        assert!(win.unread_counter_changed_hook(0, false).is_ok());
        assert_eq!(win.dock_badge, "");

        assert_eq!(win.make_from_letters("Telegram"), "TE");
        assert!(win.global_menu_markdown_state());
    }
}
