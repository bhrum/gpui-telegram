//! Authentic GPUI Continuous Slider Component.
//! Reference: Telegram/SourceFiles/ui/widgets/continuous_sliders.cpp:L18-L160
//! Reference: Telegram/SourceFiles/ui/widgets/continuous_sliders.h:L24-L200

#![allow(dead_code, unused_imports, unused_variables)]

/// Inactivity timeout before finishing wheel scrolling adjustments.
/// Reference: `continuous_sliders.cpp:L20`
pub const K_BY_WHEEL_FINISHED_TIMEOUT_MS: u64 = 1000;

/// Default wheel delta steps (120 deltas per step) and coefficient 1 / (120 * 10) = 1 / 1200.
/// Reference: `continuous_sliders.cpp:L121-L122`
pub const WHEEL_DELTA_STEP: f32 = 120.0;
pub const WHEEL_COEF: f32 = 1.0 / (WHEEL_DELTA_STEP * 10.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Target symbol for smooth floating-point media / audio seek sliders.
#[derive(Debug, Clone)]
pub struct ContinuousSlidersWidgets {
    pub direction: SliderDirection,
    pub is_disabled: bool,
    pub value: f32,
    pub received_till: f32,
    pub down_value: f32,
    pub is_mouse_down: bool,
    pub is_hovered: bool,
    pub fade_opacity: f32,
    pub move_by_wheel: bool,
    pub wheel_timer_active: bool,
    pub last_wheel_time_ms: u64,
}

impl Default for ContinuousSlidersWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl ContinuousSlidersWidgets {
    pub fn new() -> Self {
        Self {
            direction: SliderDirection::Horizontal,
            is_disabled: false,
            value: 0.0,
            received_till: 0.0,
            down_value: 0.0,
            is_mouse_down: false,
            is_hovered: false,
            fade_opacity: 1.0,
            move_by_wheel: true,
            wheel_timer_active: false,
            last_wheel_time_ms: 0,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    pub fn set_received_till(&mut self, received: f32) {
        self.received_till = received.clamp(0.0, 1.0);
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.is_disabled = disabled;
    }

    pub fn set_direction(&mut self, direction: SliderDirection) {
        self.direction = direction;
    }

    /// Computes seek value from mouse coordinate within track bounding box.
    /// Reference: `continuous_sliders.cpp:L84-L91`
    pub fn compute_value_from_pos(&self, pos: f32, track_start: f32, track_len: f32) -> f32 {
        if track_len <= 0.0 {
            return 0.0;
        }
        let ratio = if self.direction == SliderDirection::Horizontal {
            (pos - track_start) / track_len
        } else {
            1.0 - (pos - track_start) / track_len
        };
        ratio.clamp(0.0, 1.0)
    }

    /// Handles mouse press on slider track.
    /// Reference: `continuous_sliders.cpp:L93-L100`
    pub fn on_mouse_press(&mut self, pos: f32, track_start: f32, track_len: f32) -> f32 {
        if self.is_disabled {
            return self.value;
        }
        self.is_mouse_down = true;
        self.down_value = self.compute_value_from_pos(pos, track_start, track_len);
        self.value = self.down_value;
        self.value
    }

    /// Handles mouse drag while pressed.
    /// Reference: `continuous_sliders.cpp:L78-L82`
    pub fn on_mouse_move(&mut self, pos: f32, track_start: f32, track_len: f32) -> Option<f32> {
        if self.is_mouse_down && !self.is_disabled {
            self.down_value = self.compute_value_from_pos(pos, track_start, track_len);
            self.value = self.down_value;
            Some(self.value)
        } else {
            None
        }
    }

    /// Handles mouse release.
    /// Reference: `continuous_sliders.cpp:L102-L115`
    pub fn on_mouse_release(&mut self) -> f32 {
        self.is_mouse_down = false;
        self.value = self.down_value;
        self.value
    }

    /// Applies mouse wheel scroll delta with macOS / Windows direction adaptation.
    /// Reference: `continuous_sliders.cpp:L121-L141`
    pub fn on_wheel_delta(&mut self, delta_x: f32, delta_y: f32, is_mac: bool, now_ms: u64) -> f32 {
        if self.is_disabled || !self.move_by_wheel {
            return self.value;
        }
        let effective_delta = if is_mac {
            if delta_x.abs() > delta_y.abs() { -delta_x } else { delta_y }
        } else {
            if delta_x.abs() > delta_y.abs() { -delta_x } else { -delta_y }
        };

        self.value = (self.value + effective_delta * WHEEL_COEF).clamp(0.0, 1.0);
        self.wheel_timer_active = true;
        self.last_wheel_time_ms = now_ms;
        self.value
    }

    /// Checks if wheel debouncing interval has expired.
    pub fn check_wheel_timeout(&mut self, now_ms: u64) -> bool {
        if self.wheel_timer_active && now_ms >= self.last_wheel_time_ms + K_BY_WHEEL_FINISHED_TIMEOUT_MS {
            self.wheel_timer_active = false;
            true
        } else {
            false
        }
    }

    /// Steps the slider value by an increment (e.g. keyboard navigation).
    /// Reference: `continuous_sliders.cpp:L143-L160`
    pub fn step_by(&mut self, step: f32) -> f32 {
        if !self.is_disabled {
            self.value = (self.value + step).clamp(0.0, 1.0);
        }
        self.value
    }
}
