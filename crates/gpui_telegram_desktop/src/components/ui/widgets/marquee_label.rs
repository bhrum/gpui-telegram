//! Authentic GPUI Marquee Label Component with Edge Fade Ramps.
//! Reference: Telegram/SourceFiles/ui/widgets/marquee_label.cpp:L29-L120
//! Reference: Telegram/SourceFiles/ui/widgets/marquee_label.h:L35-L126

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{hsla, rgba, Hsla, Rgba};

/// Initial pause before beginning scrolling cycle: 500ms.
/// Reference: `marquee_label.cpp:L29`
pub const K_MARQUEE_DELAY_MS: u64 = 500;

/// Standard animation frame delta: 16ms (~60fps).
/// Reference: `marquee_label.cpp:L30`
pub const K_FRAME_DELTA_MS: u64 = 16;

/// Edge distance acceleration speed factor: 3.0.
/// Reference: `marquee_label.cpp:L31`
pub const K_EDGE_SCROLL_SPEED: f32 = 3.0;

/// Target symbol for continuously scrolling label when text overflows.
#[derive(Debug, Clone)]
pub struct MarqueeLabelWidgets {
    pub text: String,
    pub text_width: f32,
    pub available_width: f32,
    pub offset: f32,
    pub gap: f32,
    pub speed: f32,
    pub fade_ramp: f32,
    pub is_scrolling: bool,
    pub is_paused: bool,
    pub delay_remaining_ms: u64,
    pub text_color_override: Option<Rgba>,
    pub is_selectable: bool,
}

impl Default for MarqueeLabelWidgets {
    fn default() -> Self {
        Self::new("Marquee")
    }
}

impl MarqueeLabelWidgets {
    pub fn new(text: impl Into<String>) -> Self {
        let t = text.into();
        let tw = t.len() as f32 * 8.0;
        Self {
            text: t,
            text_width: tw,
            available_width: 200.0,
            offset: 0.0,
            gap: 16.0,
            speed: 40.0,
            fade_ramp: 12.0,
            is_scrolling: false,
            is_paused: false,
            delay_remaining_ms: K_MARQUEE_DELAY_MS,
            text_color_override: None,
            is_selectable: false,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.text_width = self.text.len() as f32 * 8.0;
        self.offset = 0.0;
        self.delay_remaining_ms = K_MARQUEE_DELAY_MS;
        self.refresh_scroll_state();
    }

    pub fn set_available_width(&mut self, width: f32) {
        self.available_width = width;
        self.refresh_scroll_state();
    }

    pub fn set_color_override(&mut self, color: Option<Rgba>) {
        self.text_color_override = color;
    }

    pub fn refresh_scroll_state(&mut self) {
        self.is_scrolling = self.text_width > self.available_width;
        if !self.is_scrolling {
            self.offset = 0.0;
        }
    }

    /// Advances scrolling offset using delta time.
    /// Reference: `marquee_label.cpp:L357-L373`
    pub fn step(&mut self, delta_seconds: f32) {
        if !self.is_scrolling || self.is_paused {
            return;
        }
        let delta_ms = (delta_seconds * 1000.0) as u64;
        if self.delay_remaining_ms > 0 {
            self.delay_remaining_ms = self.delay_remaining_ms.saturating_sub(delta_ms);
            return;
        }

        let total_cycle = self.text_width + self.gap;
        self.offset += self.speed * delta_seconds;
        if self.offset >= total_cycle {
            self.offset = 0.0;
            self.delay_remaining_ms = K_MARQUEE_DELAY_MS;
        }
    }

    /// Returns the opacity coefficient at edge ramps.
    /// Reference: `marquee_label.cpp:L400-L410`
    pub fn edge_fade_opacity(&self, x: f32) -> f32 {
        if x < self.fade_ramp {
            (x / self.fade_ramp).clamp(0.0, 1.0)
        } else if x > self.available_width - self.fade_ramp {
            ((self.available_width - x) / self.fade_ramp).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }
}
