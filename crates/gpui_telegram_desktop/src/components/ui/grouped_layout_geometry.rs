//! Upstream C++ source: `Telegram/SourceFiles/ui/grouped_layout_geometry.cpp:L1-L28` & `grouped_layout_geometry.h:L1-L20`
//! Authentic GPUI implementation of grouped album layout geometry and bounding boxes.
#![allow(dead_code)]

use gpui::{Bounds, Pixels, Point, Size};

/// Represents an individual cell placement within an album grid.
/// C++ companion: `Ui::GroupedLayoutGeometry` (`grouped_layout_geometry.cpp:L12-L28`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupedLayoutGeometry {
    pub bounds: Bounds<Pixels>,
    pub sides: u8, // bitmask of attached sides (Left=1, Top=2, Right=4, Bottom=8)
}

impl GroupedLayoutGeometry {
    pub fn new(x: Pixels, y: Pixels, width: Pixels, height: Pixels, sides: u8) -> Self {
        Self {
            bounds: Bounds {
                origin: Point { x, y },
                size: Size { width, height },
            },
            sides,
        }
    }

    pub fn is_top_left(&self) -> bool {
        (self.sides & 0x03) == 0x03
    }

    pub fn is_bottom_right(&self) -> bool {
        (self.sides & 0x0C) == 0x0C
    }
}

impl Default for GroupedLayoutGeometry {
    fn default() -> Self {
        use gpui::px;
        Self::new(px(0.0), px(0.0), px(120.0), px(120.0), 0)
    }
}
