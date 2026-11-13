//! Authentic GPUI Middle Click Auto-Scroll HUD Controller.
//! Reference: Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.cpp:L17-L60
//! Reference: Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.h:L18-L57

#![allow(dead_code, unused_imports, unused_variables)]

/// Timer tick delay in milliseconds: 15ms.
/// Reference: `middle_click_autoscroll.cpp:L17`
pub const K_AUTOSCROLL_DELAY_MS: u64 = 15;

/// Threshold for holding vs toggling middle mouse button: 220ms.
/// Reference: `middle_click_autoscroll.cpp:L18`
pub const K_HOLD_TO_TOGGLE_THRESHOLD_MS: u64 = 220;

/// Deadzone radius in pixels within which no scrolling occurs: 6px.
/// Reference: `middle_click_autoscroll.cpp:L20-L22`
pub const DEADZONE_RADIUS: f32 = 6.0;

/// Arrow layout metrics.
/// Reference: `middle_click_autoscroll.cpp:L45-L60`
pub const ARROW_OUTER_STROKE: f32 = 1.5;
pub const ARROW_CENTER_SKIP: f32 = 10.5;
pub const ARROW_GAP: f32 = 5.0;
pub const ARROW_HEAD_HALF_WIDTH: f32 = 4.5;

/// Target symbol managing middle-click HUD overlay and directional autoscrolling velocities.
#[derive(Debug, Clone)]
pub struct MiddleClickAutoscrollWidgets {
    pub is_active: bool,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub start_time_ms: u64,
}

impl Default for MiddleClickAutoscrollWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl MiddleClickAutoscrollWidgets {
    pub fn new() -> Self {
        Self {
            is_active: false,
            anchor_x: 0.0,
            anchor_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            start_time_ms: 0,
        }
    }

    /// Activates middle-click autoscroll anchored at `(x, y)`.
    pub fn start(&mut self, x: f32, y: f32, now_ms: u64) {
        self.is_active = true;
        self.anchor_x = x;
        self.anchor_y = y;
        self.current_x = x;
        self.current_y = y;
        self.start_time_ms = now_ms;
    }

    pub fn update_pointer(&mut self, x: f32, y: f32) {
        if self.is_active {
            self.current_x = x;
            self.current_y = y;
        }
    }

    /// Calculates directional delta outside deadzone.
    pub fn velocity(&self) -> (f32, f32) {
        if !self.is_active {
            return (0.0, 0.0);
        }
        let dx = self.current_x - self.anchor_x;
        let dy = self.current_y - self.anchor_y;

        let vx = if dx.abs() > DEADZONE_RADIUS {
            dx - (dx.signum() * DEADZONE_RADIUS)
        } else {
            0.0
        };
        let vy = if dy.abs() > DEADZONE_RADIUS {
            dy - (dy.signum() * DEADZONE_RADIUS)
        } else {
            0.0
        };
        (vx * 0.1, vy * 0.1)
    }

    /// Determines if release event should stop autoscroll (if held longer than threshold).
    pub fn on_mouse_release(&mut self, now_ms: u64) -> bool {
        if !self.is_active {
            return false;
        }
        let held_duration = now_ms.saturating_sub(self.start_time_ms);
        if held_duration > K_HOLD_TO_TOGGLE_THRESHOLD_MS {
            self.stop();
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) {
        self.is_active = false;
        self.anchor_x = 0.0;
        self.anchor_y = 0.0;
        self.current_x = 0.0;
        self.current_y = 0.0;
        self.start_time_ms = 0;
    }
}
