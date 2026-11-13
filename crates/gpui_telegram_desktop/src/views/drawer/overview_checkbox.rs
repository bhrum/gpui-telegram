//! Overview Round Checkbox Component
//! Upstream: Telegram/SourceFiles/overview/overview_checkbox.h, Telegram/SourceFiles/overview/overview_checkbox.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct OverviewCheckbox {
    pub is_checked: bool,
    pub is_pressed: bool,
    pub radius: f32,
    pub stroke_width: f32,
    pub animation_progress: f32,
}

impl Default for OverviewCheckbox {
    fn default() -> Self {
        Self {
            is_checked: false,
            is_pressed: false,
            radius: 11.0,
            stroke_width: 2.0,
            animation_progress: 0.0,
        }
    }
}

impl OverviewCheckbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
        self.animation_progress = if checked { 1.0 } else { 0.0 };
    }

    pub fn toggle(&mut self) -> bool {
        self.set_checked(!self.is_checked);
        self.is_checked
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
    }

    pub fn tick_animation(&mut self, delta_factor: f32) {
        let target = if self.is_checked { 1.0 } else { 0.0 };
        let diff = target - self.animation_progress;
        if diff.abs() > 0.05 {
            self.animation_progress += diff * 0.25 * delta_factor.clamp(0.1, 2.0);
        } else {
            self.animation_progress = target;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overview_checkbox_toggle() {
        let mut check = OverviewCheckbox::new();
        assert!(!check.is_checked);
        assert_eq!(check.animation_progress, 0.0);

        assert!(check.toggle());
        assert!(check.is_checked);
        assert_eq!(check.animation_progress, 1.0);

        assert!(!check.toggle());
        assert!(!check.is_checked);
        assert_eq!(check.animation_progress, 0.0);
    }
}
