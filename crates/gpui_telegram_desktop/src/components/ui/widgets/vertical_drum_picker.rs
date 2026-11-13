//! Authentic GPUI Cylindrical Vertical Drum Picker Component.
//! Reference: Telegram/SourceFiles/ui/widgets/vertical_drum_picker.cpp:L18-L60
//! Reference: Telegram/SourceFiles/ui/widgets/vertical_drum_picker.h:L20-L116

#![allow(dead_code, unused_imports, unused_variables)]

/// Boundary snap threshold close to integer index: 0.99.
/// Reference: `vertical_drum_picker.cpp:L18`
pub const K_ALMOST_INDEX: f32 = 0.99;

/// Minimum vertical perspective scaling factor at extreme drum edges: 0.2.
/// Reference: `vertical_drum_picker.cpp:L19`
pub const K_MIN_Y_SCALE: f32 = 0.2;

#[derive(Debug, Clone, PartialEq)]
pub struct DrumItem {
    pub id: i32,
    pub text: String,
}

/// Target symbol for 3D perspective vertical scroll wheel picker.
#[derive(Debug, Clone)]
pub struct VerticalDrumPickerWidgets {
    pub items: Vec<DrumItem>,
    pub selected_index: usize,
    pub current_offset: f32,
    pub target_offset: f32,
    pub item_height: f32,
    pub visible_items: usize,
    pub is_animating: bool,
}

impl Default for VerticalDrumPickerWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl VerticalDrumPickerWidgets {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            current_offset: 0.0,
            target_offset: 0.0,
            item_height: 36.0,
            visible_items: 5,
            is_animating: false,
        }
    }

    pub fn set_items(&mut self, items: Vec<DrumItem>) {
        self.items = items;
        if self.selected_index >= self.items.len() && !self.items.is_empty() {
            self.selected_index = 0;
            self.current_offset = 0.0;
            self.target_offset = 0.0;
        }
    }

    pub fn scroll_to_index(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = index;
            self.target_offset = index as f32;
            self.is_animating = true;
        }
    }

    /// Evaluates 3D cylindrical drum perspective scaling factor:
    /// `yScale = 0.2 + (1.0 - 0.2) * (1 - |distanceFromCenter|)^3`.
    /// Reference: `vertical_drum_picker.cpp:L40-L42`
    pub fn compute_perspective_scale(distance_from_center: f32) -> f32 {
        let rev_progress = (1.0 - distance_from_center.abs()).clamp(0.0, 1.0);
        let ease = rev_progress * rev_progress * rev_progress;
        K_MIN_Y_SCALE + (1.0 - K_MIN_Y_SCALE) * ease
    }

    /// Advances scrolling inertia physics towards target index offset.
    pub fn step_animation(&mut self, delta_factor: f32) -> bool {
        if self.is_animating {
            let diff = self.target_offset - self.current_offset;
            if diff.abs() < 0.01 {
                self.current_offset = self.target_offset;
                self.is_animating = false;
            } else {
                self.current_offset += diff * delta_factor.clamp(0.05, 0.4);
            }
            true
        } else {
            false
        }
    }
}
