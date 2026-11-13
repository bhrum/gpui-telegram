//! Upstream C++ source: `Telegram/SourceFiles/ui/ui_pch.h:L1-L45`
//! Authentic GPUI implementation of common UI precompiled headers and layout aliases.
#![allow(dead_code)]

use gpui::{px, Pixels, Point, Size};

/// Common geometric and layout aliases shared across UI primitives.
/// C++ companion: `ui_pch.h:L1-L45`.
#[derive(Debug, Clone, Copy, Default)]
pub struct UiPch;

impl UiPch {
    /// Helper creating a Size<Pixels> from width and height floats.
    pub fn size(w: f32, h: f32) -> Size<Pixels> {
        Size {
            width: px(w),
            height: px(h),
        }
    }

    /// Helper creating a Point<Pixels> from x and y floats.
    pub fn point(x: f32, y: f32) -> Point<Pixels> {
        Point {
            x: px(x),
            y: px(y),
        }
    }
}
