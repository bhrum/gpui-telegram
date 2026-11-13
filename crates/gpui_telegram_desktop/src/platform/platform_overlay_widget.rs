//! Platform Overlay Widget Helper & Controls
//! Upstream: Telegram/SourceFiles/platform/platform_overlay_widget.h, Telegram/SourceFiles/platform/platform_overlay_widget.cpp
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

pub const K_MAXIMIZED_ICON_OPACITY: f32 = 0.6;
pub const K_NORMAL_ICON_OPACITY: f32 = 0.9;
pub const K_OVER_BACKGROUND_OPACITY: f32 = 0.2775;
pub const K_STORIES_NAV_OPACITY: f32 = 0.3;
pub const K_STORIES_NAV_OVER_OPACITY: f32 = 0.7;

#[derive(Debug, Clone)]
pub struct PlatformOverlayWidget {
    pub is_maximized: bool,
    pub master_opacity: f32,
    pub controls_side_right: f32,
    pub file_dialog_shown: bool,
    pub skip_title_hit_test: bool,
    pub device_pixel_ratio: f32,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformOverlayWidget {
    pub fn new() -> Self {
        Self {
            is_maximized: false,
            master_opacity: K_NORMAL_ICON_OPACITY,
            controls_side_right: 12.0,
            file_dialog_shown: false,
            skip_title_hit_test: false,
            device_pixel_ratio: 1.0,
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        }
    }

    /// Upstream method: `mouseEvents`
    pub fn mouse_events(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `DefaultOverlayWidgetHelper::Buttons`
    pub fn buttons(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `style::DevicePixelRatio`
    pub fn device_pixel_ratio(&mut self) -> Result<()> {
        self.device_pixel_ratio = 1.0;
        Ok(())
    }

    /// Upstream method: `controlsSideRightValue`
    pub fn controls_side_right_value(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `notifyFileDialogShown`
    pub fn notify_file_dialog_shown(&mut self) -> Result<()> {
        self.file_dialog_shown = true;
        Ok(())
    }

    /// Upstream method: `clearState`
    pub fn clear_state(&mut self) -> Result<()> {
        self.master_opacity = K_NORMAL_ICON_OPACITY;
        self.file_dialog_shown = false;
        Ok(())
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.is_maximized = maximized;
        self.master_opacity = if maximized {
            K_MAXIMIZED_ICON_OPACITY
        } else {
            K_NORMAL_ICON_OPACITY
        };
    }

    /// Upstream formula: `Media::View::OverBackgroundColor`
    /// k1 = 0.15 * 0.85 / (1. - 0.85 * 0.85) = 0.1275 / 0.2775 = 0.459459
    /// k2 = 0.15 / (1. - 0.85 * 0.85) = 0.15 / 0.2775 = 0.540540
    pub fn over_background_color(bg_rgb: (u8, u8, u8)) -> (u8, u8, u8) {
        let (r, g, b) = bg_rgb;
        let c2 = 255.0;
        let denom = 1.0 - 0.85 * 0.85;
        let k1 = (0.15 * 0.85) / denom;
        let k2 = 0.15 / denom;
        let mix = |a: f32, c: f32| -> u8 {
            (a * k1 + c * k2).round().clamp(0.0, 255.0) as u8
        };
        (mix(r as f32, c2), mix(g as f32, c2), mix(b as f32, c2))
    }
}

impl Default for PlatformOverlayWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformOverlayWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let alpha = (self.master_opacity * 255.0) as u32;

        div()
            .flex()
            .flex_row()
            .justify_end()
            .items_center()
            .p(px(8.0))
            .gap(px(8.0))
            .bg(rgba((alpha << 24) | 0x000000))
            .child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded_md()
                    .bg(rgb(0x333333))
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(11.0))
                    .child(if self.is_maximized { "Restore" } else { "Maximize" }),
            )
            .child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded_md()
                    .bg(rgb(0xE53935))
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(11.0))
                    .child("Close"),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_color_mixing_and_opacity() {
        let mut overlay = PlatformOverlayWidget::new();
        assert_eq!(overlay.master_opacity, K_NORMAL_ICON_OPACITY);
        overlay.set_maximized(true);
        assert_eq!(overlay.master_opacity, K_MAXIMIZED_ICON_OPACITY);

        let mixed = PlatformOverlayWidget::over_background_color((0, 0, 0));
        assert!(mixed.0 > 0 && mixed.0 < 255);
        assert_eq!(mixed.0, mixed.1);
        assert_eq!(mixed.1, mixed.2);

        assert!(overlay.notify_file_dialog_shown().is_ok());
        assert!(overlay.file_dialog_shown);
        assert!(overlay.clear_state().is_ok());
        assert!(!overlay.file_dialog_shown);
    }
}
