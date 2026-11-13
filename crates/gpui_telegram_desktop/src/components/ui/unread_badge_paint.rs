//! Upstream C++ source: `Telegram/SourceFiles/ui/unread_badge_paint.cpp:L1-L193` & `unread_badge_paint.h:L1-L51`
//! Authentic GPUI implementation of unread badge size presets and capsule geometry.
#![allow(dead_code)]

use gpui::{px, Bounds, Pixels, Point, Size};

/// Contextual size presets for unread badges (`unread_badge_paint.h:L12-L21`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnreadBadgeSize {
    Dialogs,
    MainMenu,
    HistoryToDown,
    StickersPanel,
    StickersBox,
    TouchBar,
    ReactionInDialogs,
    PollInDialogs,
}

/// Geometry and styling rules for badge pills.
/// C++ companion: `Ui::PaintUnreadBadge` (`unread_badge_paint.cpp:L43-L191`).
#[derive(Debug, Clone)]
pub struct UnreadBadgePaint {
    preset: UnreadBadgeSize,
    height: Pixels,
    padding: Pixels,
}

impl UnreadBadgePaint {
    /// Constructs a painter for a specific size preset.
    pub fn new(preset: UnreadBadgeSize) -> Self {
        let (height, padding) = match preset {
            UnreadBadgeSize::Dialogs => (px(18.0), px(5.0)),
            UnreadBadgeSize::MainMenu => (px(16.0), px(4.0)),
            UnreadBadgeSize::HistoryToDown => (px(20.0), px(6.0)),
            UnreadBadgeSize::ReactionInDialogs | UnreadBadgeSize::PollInDialogs => (px(16.0), px(4.0)),
            _ => (px(18.0), px(5.0)),
        };

        Self {
            preset,
            height,
            padding,
        }
    }

    /// Computes the layout bounding box for a given text length and origin point.
    pub fn compute_bounds(&self, text_width: Pixels, origin: Point<Pixels>) -> Bounds<Pixels> {
        let min_width = self.height;
        let content_width = text_width + (self.padding * 2.0);
        let width = if content_width < min_width { min_width } else { content_width };

        Bounds {
            origin,
            size: Size {
                width,
                height: self.height,
            },
        }
    }

    pub fn corner_radius(&self) -> Pixels {
        self.height / 2.0
    }
}

impl Default for UnreadBadgePaint {
    fn default() -> Self {
        Self::new(UnreadBadgeSize::Dialogs)
    }
}
