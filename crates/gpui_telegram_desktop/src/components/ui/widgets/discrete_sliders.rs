//! Authentic GPUI Discrete Stepped Slider Component.
//! Reference: Telegram/SourceFiles/ui/widgets/discrete_sliders.cpp:L15-L260
//! Reference: Telegram/SourceFiles/ui/widgets/discrete_sliders.h:L28-L173

#![allow(dead_code, unused_imports, unused_variables)]

/// Default transition animation duration for discrete active pill sliding.
pub const K_DISCRETE_ANIM_DURATION_MS: u64 = 150;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscreteSection {
    pub label: String,
    pub left: f32,
    pub width: f32,
    pub content_width: f32,
}

impl DiscreteSection {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            left: 0.0,
            width: 0.0,
            content_width: 0.0,
        }
    }
}

/// Target symbol for discrete settings sliders with segmented pill indicator.
#[derive(Debug, Clone)]
pub struct DiscreteSlidersWidgets {
    pub sections: Vec<DiscreteSection>,
    pub active_index: usize,
    pub selected_index: usize,
    pub snap_to_label: bool,
    pub select_on_press: bool,
    pub pill_left: f32,
    pub pill_width: f32,
    pub is_animating: bool,
    pub pressed_index: Option<usize>,
}

impl Default for DiscreteSlidersWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscreteSlidersWidgets {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            active_index: 0,
            selected_index: 0,
            snap_to_label: true,
            select_on_press: true,
            pill_left: 0.0,
            pill_width: 0.0,
            is_animating: false,
            pressed_index: None,
        }
    }

    pub fn set_sections(&mut self, labels: &[&str], total_width: f32) {
        if labels.is_empty() {
            return;
        }
        let count = labels.len() as f32;
        let sec_width = total_width / count;
        self.sections = labels
            .iter()
            .enumerate()
            .map(|(i, &l)| {
                let content_w = l.len() as f32 * 8.0 + 16.0;
                DiscreteSection {
                    label: l.to_string(),
                    left: i as f32 * sec_width,
                    width: sec_width,
                    content_width: content_w,
                }
            })
            .collect();

        if self.active_index >= self.sections.len() {
            self.active_index = 0;
        }
        self.selected_index = self.active_index;
        self.update_pill_immediate();
    }

    /// Finds section index from mouse click coordinate.
    /// Reference: `discrete_sliders.cpp:L252-L260`
    pub fn get_index_from_position(&self, x: f32) -> usize {
        if self.sections.is_empty() {
            return 0;
        }
        for (i, sec) in self.sections.iter().enumerate() {
            if x < sec.left + sec.width {
                return i;
            }
        }
        self.sections.len() - 1
    }

    /// Calculates final active range for pill indicator.
    /// Reference: `discrete_sliders.cpp:L162-L173`
    pub fn get_final_active_range(&self, index: usize) -> (f32, f32) {
        if let Some(sec) = self.sections.get(index) {
            let width = if self.snap_to_label {
                sec.width.min(sec.content_width)
            } else {
                sec.width
            };
            let left = sec.left + ((sec.width - width) / 2.0);
            (left, width)
        } else {
            (0.0, 0.0)
        }
    }

    pub fn set_active_section(&mut self, index: usize) {
        if index < self.sections.len() {
            self.active_index = index;
            self.selected_index = index;
            let (target_left, target_width) = self.get_final_active_range(index);
            self.pill_left = target_left;
            self.pill_width = target_width;
        }
    }

    pub fn update_pill_immediate(&mut self) {
        let (left, width) = self.get_final_active_range(self.active_index);
        self.pill_left = left;
        self.pill_width = width;
    }

    /// Handles mouse press event on discrete slider.
    /// Reference: `discrete_sliders.cpp:L200-L208`
    pub fn on_mouse_press(&mut self, x: f32) -> usize {
        let index = self.get_index_from_position(x);
        self.pressed_index = Some(index);
        if self.select_on_press {
            self.set_active_section(index);
        }
        index
    }

    /// Handles mouse release event on discrete slider.
    /// Reference: `discrete_sliders.cpp:L218-L233`
    pub fn on_mouse_release(&mut self, x: f32) -> Option<usize> {
        let pressed = self.pressed_index.take()?;
        let index = self.get_index_from_position(x);
        if self.select_on_press || index == pressed {
            self.set_active_section(index);
            Some(index)
        } else {
            None
        }
    }
}
