//! macOS TouchBar Specialized Control Elements (Sliders, Segmented Toggles)
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Dynamic state for audio/video slider and progress controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacTrackState {
    pub position_ms: u64,
    pub duration_ms: u64,
    pub is_playing: bool,
    pub frequency: i32,
}

impl MacTrackState {
    pub fn new(position_ms: u64, duration_ms: u64, is_playing: bool) -> Self {
        Self {
            position_ms,
            duration_ms,
            is_playing,
            frequency: 44100,
        }
    }

    pub fn progress_ratio(&self) -> f64 {
        if self.duration_ms == 0 {
            0.0
        } else {
            (self.position_ms as f64 / self.duration_ms as f64).clamp(0.0, 1.0)
        }
    }
}

/// Helper and controller for macOS TouchBar buttons, two-state toggles, and sliders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarControlsMacTouchbar {
    pub active_items_count: usize,
    pub slider_value: f64,
    pub two_state_active: bool,
}

impl MacTouchbarControlsMacTouchbar {
    pub fn new() -> Self {
        Self {
            active_items_count: 0,
            slider_value: 0.0,
            two_state_active: false,
        }
    }

    /// Formats elapsed and total duration in `MM:SS` format.
    pub fn format_duration_text(position_ms: u64, duration_ms: u64) -> String {
        let pos_sec = position_ms / 1000;
        let dur_sec = duration_ms / 1000;
        format!(
            "{:02}:{:02} / {:02}:{:02}",
            pos_sec / 60,
            pos_sec % 60,
            dur_sec / 60,
            dur_sec % 60
        )
    }

    pub fn handle_slider_seek(&mut self, ratio: f64) -> Result<f64> {
        self.slider_value = ratio.clamp(0.0, 1.0);
        Ok(self.slider_value)
    }

    pub fn toggle_two_state(&mut self) -> Result<bool> {
        self.two_state_active = !self.two_state_active;
        Ok(self.two_state_active)
    }

    pub fn set_two_state(&mut self, active: bool) {
        self.two_state_active = active;
    }
}

impl Default for MacTouchbarControlsMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touchbar_controls_and_formatting() {
        let mut controls = MacTouchbarControlsMacTouchbar::new();
        assert_eq!(controls.slider_value, 0.0);

        let formatted = MacTouchbarControlsMacTouchbar::format_duration_text(75000, 240000);
        assert_eq!(formatted, "01:15 / 04:00");

        controls.handle_slider_seek(0.5).unwrap();
        assert_eq!(controls.slider_value, 0.5);

        assert_eq!(controls.toggle_two_state().unwrap(), true);
        assert_eq!(controls.toggle_two_state().unwrap(), false);
    }
}
