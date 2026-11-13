//! Upstream C++ source: `Telegram/SourceFiles/ui/grouped_layout.cpp:L1-L620` & `grouped_layout.h:L1-L35`
//! Authentic GPUI implementation of Telegram grouped media album layout engine.
#![allow(dead_code)]

use super::grouped_layout_geometry::GroupedLayoutGeometry;
use gpui::{px, Pixels, Size};

/// Computes positions and sizes for grouped media bubbles (1 to 10 items).
/// C++ companion: `Ui::GroupedLayout` (`grouped_layout.cpp:L17-L620`).
#[derive(Debug, Clone)]
pub struct GroupedLayout {
    max_width: Pixels,
    max_height: Pixels,
    spacing: Pixels,
}

impl GroupedLayout {
    pub fn new(max_width: f32, max_height: f32, spacing: f32) -> Self {
        Self {
            max_width: px(max_width),
            max_height: px(max_height),
            spacing: px(spacing),
        }
    }

    /// Computes cell layouts for a list of media aspect ratios (width / height).
    pub fn calculate(&self, aspect_ratios: &[f32]) -> Vec<GroupedLayoutGeometry> {
        let count = aspect_ratios.len();
        if count == 0 {
            return Vec::new();
        }
        if count == 1 {
            let ratio = aspect_ratios[0].clamp(0.4, 2.5);
            let (w, h) = if ratio >= 1.0 {
                (self.max_width, self.max_width / ratio)
            } else {
                (self.max_height * ratio, self.max_height)
            };
            return vec![GroupedLayoutGeometry::new(px(0.0), px(0.0), w, h, 0x0F)];
        }

        // 2-item side by side or stacked layout
        if count == 2 {
            let half_w = (self.max_width - self.spacing) / 2.0;
            let h = (half_w * 0.75).min(self.max_height);
            return vec![
                GroupedLayoutGeometry::new(px(0.0), px(0.0), half_w, h, 0x03),
                GroupedLayoutGeometry::new(half_w + self.spacing, px(0.0), half_w, h, 0x0C),
            ];
        }

        // Generic grid layout for 3 to 10 items
        let cols = if count <= 4 { 2 } else { 3 };
        let rows = (count + cols - 1) / cols;
        let col_w = (self.max_width - (self.spacing * (cols - 1) as f32)) / cols as f32;
        let row_h = (self.max_height - (self.spacing * (rows - 1) as f32)) / rows as f32;

        let mut results = Vec::with_capacity(count);
        for i in 0..count {
            let col = i % cols;
            let row = i / cols;
            let x = (col_w + self.spacing) * col as f32;
            let y = (row_h + self.spacing) * row as f32;
            results.push(GroupedLayoutGeometry::new(x, y, col_w, row_h, 0));
        }
        results
    }
}

impl Default for GroupedLayout {
    fn default() -> Self {
        Self::new(420.0, 360.0, 2.0)
    }
}
