//! Authentic GPUI Sliding Tabs Transition Container.
//! Reference: Telegram/SourceFiles/ui/widgets/sliding_tabs.cpp:L17-L80
//! Reference: Telegram/SourceFiles/ui/widgets/sliding_tabs.h:L18-L45

#![allow(dead_code, unused_imports, unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlideTransitionType {
    #[default]
    Animated,
    Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideDirection {
    Left,
    Right,
}

/// Target symbol for animated tab-switch container.
#[derive(Debug, Clone)]
pub struct SlidingTabsWidgets {
    pub count: usize,
    pub shown_index: usize,
    pub previous_index: usize,
    pub is_animating: bool,
    pub animation_progress: f32,
    pub slide_direction: Option<SlideDirection>,
}

impl Default for SlidingTabsWidgets {
    fn default() -> Self {
        Self::new(1)
    }
}

impl SlidingTabsWidgets {
    pub fn new(count: usize) -> Self {
        Self {
            count: count.max(1),
            shown_index: 0,
            previous_index: 0,
            is_animating: false,
            animation_progress: 1.0,
            slide_direction: None,
        }
    }

    /// Switches tab with sliding animation or instant switch.
    /// Reference: `sliding_tabs.cpp:L44-L60`
    pub fn show_tab(&mut self, index: usize, transition: SlideTransitionType) {
        if index >= self.count || index == self.shown_index {
            return;
        }
        self.previous_index = self.shown_index;
        self.shown_index = index;

        // Slide left if switching to a lower index, else right
        self.slide_direction = Some(if index < self.previous_index {
            SlideDirection::Left
        } else {
            SlideDirection::Right
        });

        match transition {
            SlideTransitionType::Instant => {
                self.is_animating = false;
                self.animation_progress = 1.0;
            }
            SlideTransitionType::Animated => {
                self.is_animating = true;
                self.animation_progress = 0.0;
            }
        }
    }

    /// Advances animation frame progress.
    pub fn step_animation(&mut self, delta: f32) -> bool {
        if self.is_animating {
            self.animation_progress = (self.animation_progress + delta).min(1.0);
            if self.animation_progress >= 1.0 {
                self.is_animating = false;
                self.slide_direction = None;
            }
            true
        } else {
            false
        }
    }
}
