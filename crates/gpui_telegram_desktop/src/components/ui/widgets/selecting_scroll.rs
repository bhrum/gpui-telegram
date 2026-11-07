//! Authentic GPUI Selecting Scroll Auto-Scroll Helper.
//! Reference: Telegram/SourceFiles/ui/widgets/selecting_scroll.cpp:L20-L60
//! Reference: Telegram/SourceFiles/ui/widgets/selecting_scroll.h:L10-L18

#![allow(dead_code, unused_imports, unused_variables)]

/// Minimum interval between speed updates: 15ms.
/// Reference: `selecting_scroll.cpp:L20`
pub const K_SPEED_INTERVAL_MS: u64 = 15;

/// Maximum auto-scrolling speed limit in pixels: 37px per step.
/// Reference: `selecting_scroll.cpp:L21`
pub const K_MAX_SPEED: f32 = 37.0;

/// Maximum single frame interval cap: 64ms.
/// Reference: `selecting_scroll.cpp:L22`
pub const K_MAX_FRAME_INTERVAL_MS: u64 = 64;

/// Computes distance outside visible boundary rectangle.
/// Reference: `selecting_scroll.cpp:L28-L36`
pub fn delta_outside(top: f32, bottom: f32, y: f32) -> f32 {
    if y < top {
        y - top
    } else if y > bottom {
        y - bottom
    } else {
        0.0
    }
}

/// Computes auto-scrolling velocity given boundary delta:
/// `speed = (delta > 0) ? min(delta * 3 / 20 + 1, 37) : max(delta * 3 / 20 - 1, -37)`.
/// Reference: `selecting_scroll.cpp:L38-L42`
pub fn speed_for_delta(delta: f32) -> f32 {
    if delta > 0.0 {
        ((delta * 3.0 / 20.0) + 1.0).min(K_MAX_SPEED)
    } else if delta < 0.0 {
        ((delta * 3.0 / 20.0) - 1.0).max(-K_MAX_SPEED)
    } else {
        0.0
    }
}

/// Target symbol managing automated scrolling while dragging selection outside viewport.
#[derive(Debug, Clone)]
pub struct SelectingScrollWidgets {
    pub is_active: bool,
    pub delta: f32,
    pub accumulated_pixels: f32,
    pub last_frame_ms: u64,
}

impl Default for SelectingScrollWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectingScrollWidgets {
    pub fn new() -> Self {
        Self {
            is_active: false,
            delta: 0.0,
            accumulated_pixels: 0.0,
            last_frame_ms: 0,
        }
    }

    /// Evaluates pointer Y coordinate against visible bounds [top, bottom].
    pub fn update_cursor(&mut self, top: f32, bottom: f32, cursor_y: f32) -> f32 {
        self.delta = delta_outside(top, bottom, cursor_y);
        self.is_active = self.delta != 0.0;
        speed_for_delta(self.delta)
    }

    /// Steps scrolling frame and returns scroll displacement in integer pixels.
    /// Reference: `selecting_scroll.cpp:L57-L60`
    pub fn step_frame(&mut self, now_ms: u64) -> i32 {
        if !self.is_active || self.delta == 0.0 {
            self.delta = 0.0;
            self.accumulated_pixels = 0.0;
            return 0;
        }

        let speed = speed_for_delta(self.delta);
        let elapsed = if self.last_frame_ms == 0 {
            K_SPEED_INTERVAL_MS
        } else {
            (now_ms.saturating_sub(self.last_frame_ms)).min(K_MAX_FRAME_INTERVAL_MS)
        };
        self.last_frame_ms = now_ms;

        let frame_scroll = speed * (elapsed as f32 / K_SPEED_INTERVAL_MS as f32);
        self.accumulated_pixels += frame_scroll;

        let int_pixels = self.accumulated_pixels as i32;
        self.accumulated_pixels -= int_pixels as f32;
        int_pixels
    }

    pub fn stop(&mut self) {
        self.is_active = false;
        self.delta = 0.0;
        self.accumulated_pixels = 0.0;
        self.last_frame_ms = 0;
    }
}
