//! macOS Window Overlay Traffic Light Controls and Hit-Testing
//! Upstream Source: Telegram/SourceFiles/platform/mac/overlay_widget_mac.h, Telegram/SourceFiles/platform/mac/overlay_widget_mac.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// System title controls matching macOS traffic lights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacTitleControl {
    Close,
    Minimize,
    Maximize,
}

/// Bounding rectangle for hit-testing and traffic light button placement.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ControlGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ControlGeometry {
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

/// Helper and GPUI view for overlay window titlebar traffic light controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayWidgetMacMac {
    pub is_fullscreen: bool,
    pub master_opacity: f32,
    pub top_notch_skip: i32,
    pub native_movable: bool,
    pub hovered_control: Option<MacTitleControl>,
    pub close_rect: ControlGeometry,
    pub minimize_rect: ControlGeometry,
    pub maximize_rect: ControlGeometry,
}

impl OverlayWidgetMacMac {
    pub fn new() -> Self {
        Self {
            is_fullscreen: false,
            master_opacity: 1.0,
            top_notch_skip: 0,
            native_movable: true,
            hovered_control: None,
            close_rect: ControlGeometry { x: 12.0, y: 12.0, width: 12.0, height: 12.0 },
            minimize_rect: ControlGeometry { x: 32.0, y: 12.0, width: 12.0, height: 12.0 },
            maximize_rect: ControlGeometry { x: 52.0, y: 12.0, width: 12.0, height: 12.0 },
        }
    }

    pub fn skip_title_hit_test(&self, x: f32, y: f32) -> bool {
        self.close_rect.contains(x, y)
            || self.minimize_rect.contains(x, y)
            || self.maximize_rect.contains(x, y)
    }

    pub fn update_hover(&mut self, x: f32, y: f32) {
        if self.close_rect.contains(x, y) {
            self.hovered_control = Some(MacTitleControl::Close);
        } else if self.minimize_rect.contains(x, y) {
            self.hovered_control = Some(MacTitleControl::Minimize);
        } else if self.maximize_rect.contains(x, y) {
            self.hovered_control = Some(MacTitleControl::Maximize);
        } else {
            self.hovered_control = None;
        }
        self.native_movable = self.hovered_control.is_none();
    }

    pub fn before_show(&mut self, fullscreen: bool) -> Result<()> {
        self.is_fullscreen = fullscreen;
        self.hovered_control = None;
        self.native_movable = true;
        Ok(())
    }

    pub fn after_show(&mut self, fullscreen: bool) -> Result<()> {
        self.is_fullscreen = fullscreen;
        Ok(())
    }

    pub fn set_controls_opacity(&mut self, opacity: f32) {
        self.master_opacity = opacity.clamp(0.0, 1.0);
    }
}

impl Default for OverlayWidgetMacMac {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for OverlayWidgetMacMac {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let opacity = self.master_opacity;
        let is_fs = self.is_fullscreen;

        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .p_3()
            .opacity(opacity)
            .child(
                // Close button (red)
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(rgb(0xfc605c))
                    .border_1()
                    .border_color(rgb(0xde403b)),
            )
            .child(
                // Minimize button (yellow)
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(rgb(0xfec041))
                    .border_1()
                    .border_color(rgb(0xdd9819)),
            )
            .child(
                // Maximize button (green)
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(rgb(0x34c84a))
                    .border_1()
                    .border_color(rgb(0x27ae38)),
            );

        if is_fs {
            root = root.child(div().text_xs().text_color(rgb(0x8e8e93)).child("Fullscreen"));
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_hit_test_and_movable_state() {
        let mut overlay = OverlayWidgetMacMac::new();
        assert!(overlay.skip_title_hit_test(14.0, 14.0));
        assert!(overlay.skip_title_hit_test(34.0, 14.0));
        assert!(overlay.skip_title_hit_test(54.0, 14.0));
        assert!(!overlay.skip_title_hit_test(100.0, 14.0));

        overlay.update_hover(14.0, 14.0);
        assert_eq!(overlay.hovered_control, Some(MacTitleControl::Close));
        assert_eq!(overlay.native_movable, false);

        overlay.update_hover(200.0, 200.0);
        assert_eq!(overlay.hovered_control, None);
        assert_eq!(overlay.native_movable, true);
    }
}
