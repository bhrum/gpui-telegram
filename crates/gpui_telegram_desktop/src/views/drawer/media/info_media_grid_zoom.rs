//! Info Media Grid Zoom controller
//! Upstream: `Telegram/SourceFiles/info/media/info_media_grid_zoom.cpp:L1-L252`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_grid_zoom.h:L1-L55`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// Controller managing pinch-to-zoom and step-zoom levels for the media thumbnail grid.
/// Supports smooth transitions between 1 and 6 column layouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoMediaGridZoomMedia {
    pub current_columns: usize,
    pub min_columns: usize,
    pub max_columns: usize,
    pub zoom_progress: f32,
    pub is_animating: bool,
}

impl InfoMediaGridZoomMedia {
    pub const STEP_MIN: usize = 1;
    pub const STEP_MAX: usize = 6;
    pub const DEFAULT_COLUMNS: usize = 3;
    pub const PINCH_STEP: f32 = 0.05;
    pub const FADE_DURATION_MS: u64 = 150;

    pub fn new() -> Self {
        Self {
            current_columns: Self::DEFAULT_COLUMNS,
            min_columns: Self::STEP_MIN,
            max_columns: Self::STEP_MAX,
            zoom_progress: 0.0,
            is_animating: false,
        }
    }

    pub fn zoom_in(&mut self) -> bool {
        if self.current_columns > self.min_columns {
            self.current_columns -= 1;
            true
        } else {
            false
        }
    }

    pub fn zoom_out(&mut self) -> bool {
        if self.current_columns < self.max_columns {
            self.current_columns += 1;
            true
        } else {
            false
        }
    }

    pub fn set_columns(&mut self, columns: usize) {
        self.current_columns = columns.clamp(self.min_columns, self.max_columns);
    }

    pub fn item_size(&self, available_width: f32, spacing: f32) -> f32 {
        let cols = self.current_columns.max(1) as f32;
        let total_spacing = (cols - 1.0) * spacing;
        ((available_width - total_spacing) / cols).max(24.0)
    }
}

impl Default for InfoMediaGridZoomMedia {
    fn default() -> Self {
        Self::new()
    }
}
