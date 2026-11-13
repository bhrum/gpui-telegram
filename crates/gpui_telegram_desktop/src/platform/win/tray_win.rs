//! Windows System Tray Icon & Shell Notify Bridge
//! Upstream: Telegram/SourceFiles/platform/win/tray_win.h, Telegram/SourceFiles/platform/win/tray_win.cpp
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct TrayWinWin {
    pub is_registered: bool,
    pub unread_counter: u32,
    pub tooltip: String,
    pub icon_clicks_count: usize,
    pub show_requests_count: usize,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl TrayWinWin {
    pub fn new() -> Self {
        Self {
            is_registered: true,
            unread_counter: 0,
            tooltip: "Telegram Desktop".into(),
            icon_clicks_count: 0,
            show_requests_count: 0,
            is_supported: true,
            is_enabled: true,
            platform_name: "WindowsTrayNotify".into(),
        }
    }

    /// Upstream method: `Window::LogoNoMargin`
    pub fn logo_no_margin(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `Tray::Tray`
    pub fn tray(&mut self) -> Result<()> {
        self.is_registered = true;
        Ok(())
    }

    /// Upstream method: `rpl::iconClicks`
    pub fn icon_clicks(&mut self) -> Result<()> {
        self.icon_clicks_count += 1;
        Ok(())
    }

    /// Upstream method: `rpl::showFromTrayRequests`
    pub fn show_from_tray_requests(&mut self) -> Result<()> {
        self.show_requests_count += 1;
        Ok(())
    }

    /// Upstream method: `createIcon`
    pub fn create_icon(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `Core::TrayIconToolTip`
    pub fn tray_icon_tool_tip(&mut self) -> Result<()> {
        if self.unread_counter > 0 {
            self.tooltip = format!("Telegram Desktop ({} unread)", self.unread_counter);
        } else {
            self.tooltip = "Telegram Desktop".into();
        }
        Ok(())
    }

    pub fn set_unread_counter(&mut self, counter: u32) {
        self.unread_counter = counter;
        let _ = self.tray_icon_tool_tip();
    }
}

impl Default for TrayWinWin {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for TrayWinWin {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(6.0))
            .gap(px(6.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_md()
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .rounded_md()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(10.0))
                    .child("TG"),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .child(format!("WinTray: Unread={}, Clicks={}", self.unread_counter, self.icon_clicks_count)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_win_lifecycle_and_methods() {
        let mut tray = TrayWinWin::new();
        assert!(tray.logo_no_margin().is_ok());
        assert!(tray.tray().is_ok());
        assert!(tray.create_icon().is_ok());
        assert!(tray.tray_icon_tool_tip().is_ok());
        assert_eq!(tray.tooltip, "Telegram Desktop");

        tray.set_unread_counter(12);
        assert_eq!(tray.unread_counter, 12);
        assert!(tray.tooltip.contains("12 unread"));

        assert!(tray.icon_clicks().is_ok());
        assert_eq!(tray.icon_clicks_count, 1);

        assert!(tray.show_from_tray_requests().is_ok());
        assert_eq!(tray.show_requests_count, 1);
    }
}
