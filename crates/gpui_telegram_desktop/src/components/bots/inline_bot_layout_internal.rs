//! InlineBotLayoutInternal: Gallery mosaic and vertical list geometry calculator.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.h:L30-L80`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.cpp:L20-L90`
#![allow(dead_code)]

use gpui::*;

/// Minimum allowed aspect ratio for gallery item (width / height).
pub const MIN_ASPECT_RATIO: f32 = 0.5;

/// Maximum allowed aspect ratio for gallery item (width / height).
pub const MAX_ASPECT_RATIO: f32 = 2.5;

/// Target row height for mosaic gallery items in pixels.
pub const TARGET_ROW_HEIGHT: f32 = 100.0;

/// Item kind for inline layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineLayoutKind {
    Photo,
    Gif,
    File,
    Article,
}

/// Calculated geometry for a laid out inline item.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Layout item measurement inputs.
#[derive(Debug, Clone)]
pub struct LayoutItemInput {
    pub id: String,
    pub kind: InlineLayoutKind,
    pub raw_width: f32,
    pub raw_height: f32,
}

impl LayoutItemInput {
    pub fn aspect_ratio(&self) -> f32 {
        if self.raw_height > 0.0 {
            (self.raw_width / self.raw_height).clamp(MIN_ASPECT_RATIO, MAX_ASPECT_RATIO)
        } else {
            1.0
        }
    }
}

/// Authentic InlineBotLayoutInternal state.
pub struct InlineBotLayoutInternal {
    pub container_width: f32,
    pub spacing: f32,
    pub computed_rects: Vec<(String, LayoutRect)>,
    pub total_height: f32,
    pub updated_at: i64,
}

impl InlineBotLayoutInternal {
    pub fn new(container_width: f32) -> Self {
        Self {
            container_width,
            spacing: 4.0,
            computed_rects: Vec::new(),
            total_height: 0.0,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn compute_gallery_layout(&mut self, items: &[LayoutItemInput]) {
        self.computed_rects.clear();
        if items.is_empty() {
            self.total_height = 0.0;
            return;
        }

        let mut current_y = 0.0;
        let mut row_start = 0;

        while row_start < items.len() {
            let mut row_end = row_start;
            let mut row_aspect_sum = 0.0;

            // Greedily accumulate items until row width is filled at target height
            while row_end < items.len() {
                row_aspect_sum += items[row_end].aspect_ratio();
                row_end += 1;
                let potential_width = row_aspect_sum * TARGET_ROW_HEIGHT
                    + ((row_end - row_start - 1) as f32) * self.spacing;
                if potential_width >= self.container_width || row_end - row_start >= 4 {
                    break;
                }
            }

            let count = row_end - row_start;
            let total_spacing = ((count - 1) as f32) * self.spacing;
            let available_width = (self.container_width - total_spacing).max(10.0);
            let row_height = (available_width / row_aspect_sum).clamp(60.0, 160.0);

            let mut current_x = 0.0;
            for item in &items[row_start..row_end] {
                let w = item.aspect_ratio() * row_height;
                self.computed_rects.push((
                    item.id.clone(),
                    LayoutRect {
                        x: current_x,
                        y: current_y,
                        width: w,
                        height: row_height,
                    },
                ));
                current_x += w + self.spacing;
            }

            current_y += row_height + self.spacing;
            row_start = row_end;
        }

        self.total_height = current_y;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for InlineBotLayoutInternal {
    fn default() -> Self {
        Self::new(320.0)
    }
}

impl Render for InlineBotLayoutInternal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(self.total_height))
            .bg(rgb(0xFFFFFF))
    }
}
