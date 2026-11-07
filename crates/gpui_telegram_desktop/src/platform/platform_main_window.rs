//! Platform Main Window Abstraction
//! Upstream: Telegram/SourceFiles/platform/platform_main_window.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct PlatformMainWindow {
    pub width: f32,
    pub height: f32,
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub is_fullscreen: bool,
    pub is_active: bool,
    pub native_frame_shadow: bool,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformMainWindow {
    pub fn new() -> Self {
        Self {
            width: 1024.0,
            height: 768.0,
            is_maximized: false,
            is_minimized: false,
            is_fullscreen: false,
            is_active: true,
            native_frame_shadow: cfg!(target_os = "windows"),
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn set_geometry(&mut self, width: f32, height: f32) {
        self.width = width.max(400.0);
        self.height = height.max(300.0);
    }

    pub fn toggle_maximize(&mut self) {
        self.is_maximized = !self.is_maximized;
    }

    pub fn minimize(&mut self) {
        self.is_minimized = true;
    }

    pub fn restore(&mut self) {
        self.is_minimized = false;
        self.is_maximized = false;
        self.is_fullscreen = false;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_maximized(&self) -> bool {
        self.is_maximized
    }

    pub fn native_frame_shadow(&self) -> bool {
        self.native_frame_shadow
    }
}

impl Default for PlatformMainWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformMainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let border_color = if self.is_active { rgb(0x2AABEE) } else { rgb(0xCCCCCC) };

        div()
            .flex()
            .flex_col()
            .w(px(self.width.min(600.0)))
            .h(px(self.height.min(400.0)))
            .border_1()
            .border_color(border_color)
            .rounded_lg()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .px(px(12.0))
                    .py(px(6.0))
                    .bg(rgb(0xFAFAFA))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format!("Telegram ({})", self.platform_name)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(rgb(0xFFBD2E)))
                            .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(rgb(0x28C840)))
                            .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(rgb(0xFF5F56))),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .p(px(16.0))
                    .child(format!("Main Window: {}x{} (Maximized: {})", self.width, self.height, self.is_maximized)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_window_geometry_and_states() {
        let mut win = PlatformMainWindow::new();
        assert!(!win.is_maximized());
        win.set_geometry(1280.0, 800.0);
        assert_eq!(win.width, 1280.0);
        assert_eq!(win.height, 800.0);

        win.toggle_maximize();
        assert!(win.is_maximized());

        win.minimize();
        assert!(win.is_minimized);

        win.restore();
        assert!(!win.is_minimized);
        assert!(!win.is_maximized);
    }
}
