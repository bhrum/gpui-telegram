//! Authentic GPUI Audio Volume & Level Meter Component.
//! Reference: Telegram/SourceFiles/ui/widgets/level_meter.cpp:L15-L48
//! Reference: Telegram/SourceFiles/ui/widgets/level_meter.h:L15-L33

#![allow(dead_code, unused_imports, unused_variables)]

/// Target symbol for segmented microphone input volume / audio playback level meter.
#[derive(Debug, Clone)]
pub struct LevelMeterWidgets {
    pub value: f32,
    pub line_count: usize,
    pub line_width: f32,
    pub line_spacing: f32,
    pub height: f32,
}

impl Default for LevelMeterWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelMeterWidgets {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            line_count: 8,
            line_width: 3.0,
            line_spacing: 2.0,
            height: 18.0,
        }
    }

    /// Sets the normalized audio level [0.0, 1.0].
    /// Reference: `level_meter.cpp:L20-L23`
    pub fn set_value(&mut self, val: f32) {
        self.value = val.clamp(0.0, 1.0);
    }

    /// Returns bar corner radius: `lineWidth / 2`.
    /// Reference: `level_meter.cpp:L33`
    pub fn bar_corner_radius(&self) -> f32 {
        self.line_width / 2.0
    }

    /// Checks if line segment at index `i` is active for current level value.
    /// Reference: `level_meter.cpp:L37-L40`
    pub fn is_line_active(&self, index: usize) -> bool {
        if self.line_count == 0 {
            return false;
        }
        let threshold = (index + 1) as f32 / self.line_count as f32;
        threshold <= self.value
    }

    /// Computes X coordinate for line segment `i`: `(lineWidth + lineSpacing) * i`.
    /// Reference: `level_meter.cpp:L42`
    pub fn line_x_offset(&self, index: usize) -> f32 {
        (self.line_width + self.line_spacing) * index as f32
    }

    /// Computes total natural widget width.
    pub fn natural_width(&self) -> f32 {
        if self.line_count == 0 {
            0.0
        } else {
            (self.line_width * self.line_count as f32)
                + (self.line_spacing * (self.line_count - 1) as f32)
        }
    }
}
