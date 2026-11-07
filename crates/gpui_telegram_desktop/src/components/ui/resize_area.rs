//! Upstream C++ source: `Telegram/SourceFiles/ui/resize_area.h:L1-L78`
//! Authentic GPUI implementation of panel and drawer edge resizing handles.
#![allow(dead_code)]

use gpui::{px, Pixels};

/// Resize drag edge orientation from `resize_area.h:L18-L24`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    Left,
    Right,
    Top,
    Bottom,
}

/// Drag handle controller for adjustable sidebar and drawer widths.
/// C++ companion: `Ui::ResizeArea` (`resize_area.h:L14-L78`).
#[derive(Debug, Clone)]
pub struct ResizeArea {
    edge: ResizeEdge,
    current_width: Pixels,
    min_width: Pixels,
    max_width: Pixels,
    is_dragging: bool,
}

impl ResizeArea {
    pub fn new(edge: ResizeEdge, initial_width: f32, min_width: f32, max_width: f32) -> Self {
        Self {
            edge,
            current_width: px(initial_width),
            min_width: px(min_width),
            max_width: px(max_width),
            is_dragging: false,
        }
    }

    pub fn start_drag(&mut self) {
        self.is_dragging = true;
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }

    pub fn apply_delta(&mut self, delta: Pixels) {
        if !self.is_dragging {
            return;
        }
        let new_w = match self.edge {
            ResizeEdge::Right => self.current_width + delta,
            ResizeEdge::Left => self.current_width - delta,
            _ => self.current_width,
        };
        self.current_width = new_w.clamp(self.min_width, self.max_width);
    }

    pub fn current_width(&self) -> Pixels {
        self.current_width
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}

impl Default for ResizeArea {
    fn default() -> Self {
        Self::new(ResizeEdge::Right, 260.0, 200.0, 480.0)
    }
}
