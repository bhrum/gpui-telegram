//! Windows Overlay Widget Implementation Hooks
//! Upstream: Telegram/SourceFiles/platform/win/overlay_widget_win.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct OverlayWidgetWinWin {
    pub border_padding: f32,
    pub window_handle_tracked: bool,
    pub custom_title_controls: bool,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl OverlayWidgetWinWin {
    pub fn new() -> Self {
        Self {
            border_padding: 8.0,
            window_handle_tracked: false,
            custom_title_controls: true,
            is_supported: true,
            is_enabled: true,
            platform_name: "WindowsOverlayWidget".into(),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.window_handle_tracked = true;
        Ok(())
    }

    pub fn set_border_padding(&mut self, padding: f32) {
        self.border_padding = padding;
    }
}

impl Default for OverlayWidgetWinWin {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for OverlayWidgetWinWin {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(self.border_padding))
            .gap(px(4.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_md()
            .child(
                div()
                    .text_size(px(11.0))
                    .child(format!("WinOverlay: CustomControls={}, Tracked={}", self.custom_title_controls, self.window_handle_tracked)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_widget_win_lifecycle() {
        let mut widget = OverlayWidgetWinWin::new();
        assert!(!widget.window_handle_tracked);
        assert!(widget.initialize().is_ok());
        assert!(widget.window_handle_tracked);
        widget.set_border_padding(12.0);
        assert_eq!(widget.border_padding, 12.0);
    }
}
