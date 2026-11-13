//! macOS TouchBar Fullscreen & Media Viewer Controller
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.mm
//! Subsystem: platform | Feature ID: TCG-N10 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Media view touchbar controller and GPUI Render element for fullscreen/video overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarMediaViewMacTouchbar {
    pub is_playing: bool,
    pub rotation_degrees: i32,
    pub is_fullscreen: bool,
    pub is_pip_active: bool,
    pub position_ms: u64,
    pub duration_ms: u64,
}

impl MacTouchbarMediaViewMacTouchbar {
    pub const ITEM_PLAY_PAUSE: &'static str = "button.playPause";
    pub const ITEM_ROTATE: &'static str = "button.rotate";
    pub const ITEM_FULLSCREEN: &'static str = "button.fullscreen";
    pub const ITEM_PIP: &'static str = "button.pip";
    pub const ITEM_TRACK: &'static str = "trackPosition";
    pub const ITEM_SEEK: &'static str = "seekBar";

    pub fn new() -> Self {
        Self {
            is_playing: false,
            rotation_degrees: 0,
            is_fullscreen: false,
            is_pip_active: false,
            position_ms: 0,
            duration_ms: 0,
        }
    }

    pub fn toggle_play_pause(&mut self) -> Result<bool> {
        self.is_playing = !self.is_playing;
        Ok(self.is_playing)
    }

    pub fn rotate_counter_clockwise(&mut self) -> Result<i32> {
        self.rotation_degrees = (self.rotation_degrees + 270) % 360;
        Ok(self.rotation_degrees)
    }

    pub fn toggle_fullscreen(&mut self) -> Result<bool> {
        self.is_fullscreen = !self.is_fullscreen;
        Ok(self.is_fullscreen)
    }

    pub fn toggle_pip(&mut self) -> Result<bool> {
        self.is_pip_active = !self.is_pip_active;
        Ok(self.is_pip_active)
    }

    pub fn update_track_state(&mut self, pos_ms: u64, dur_ms: u64, playing: bool) {
        self.position_ms = pos_ms;
        self.duration_ms = dur_ms;
        self.is_playing = playing;
    }
}

impl Default for MacTouchbarMediaViewMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacTouchbarMediaViewMacTouchbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let play_icon = if self.is_playing { "⏸" } else { "▶" };
        let fs_icon = if self.is_fullscreen { "⤦" } else { "⤢" };
        let rot = format!("↺ {}°", self.rotation_degrees);

        div()
            .h(px(30.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_2()
            .bg(rgb(0x181818))
            .rounded_md()
            // Play/Pause button
            .child(div().px_2().py_1().bg(rgb(0x303030)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child(play_icon))
            // Rotate button
            .child(div().px_2().py_1().bg(rgb(0x303030)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child(rot))
            // Fullscreen toggle button
            .child(div().px_2().py_1().bg(rgb(0x303030)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child(fs_icon))
            // PiP toggle button
            .child(div().px_2().py_1().bg(rgb(0x303030)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child("PiP"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_view_touchbar_controls() {
        let mut mv = MacTouchbarMediaViewMacTouchbar::new();
        assert!(!mv.is_playing);

        assert!(mv.toggle_play_pause().unwrap());
        assert!(mv.is_playing);

        assert_eq!(mv.rotate_counter_clockwise().unwrap(), 270);
        assert_eq!(mv.rotate_counter_clockwise().unwrap(), 180);

        assert!(mv.toggle_fullscreen().unwrap());
        assert!(mv.is_fullscreen);

        assert!(mv.toggle_pip().unwrap());
        assert!(mv.is_pip_active);
    }
}
