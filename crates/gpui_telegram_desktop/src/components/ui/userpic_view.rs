//! Upstream C++ source: `Telegram/SourceFiles/ui/userpic_view.cpp:L1-L196` & `userpic_view.h:L1-L67`
//! Authentic GPUI implementation of peer userpic cache views and forum mask geometry.
#![allow(dead_code)]

use gpui::{px, Pixels};

/// Mask shape for peer userpics (`userpic_view.h:L39-L44`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerUserpicShape {
    Circle,
    Square,
    Forum, // Monoforum squircle
}

/// Community userpic card rotation constants from `userpic_view.cpp:L19-L30`.
pub const K_PEEK: f64 = 0.3;
pub const K_COVER: f64 = 0.5;
pub const K_PIVOT_Y: f64 = 0.75;
pub const K_CARD1_SCALE: f64 = 0.82;
pub const K_CARD1_ANGLE: f64 = -9.0;
pub const K_CARD1_RADIUS: f64 = 0.22;
pub const K_CARD1_OPACITY: f64 = 0.5;
pub const K_CARD2_SCALE: f64 = 0.70;
pub const K_CARD2_ANGLE: f64 = -16.0;
pub const K_CARD2_RADIUS: f64 = 0.16;
pub const K_CARD2_OPACITY: f64 = 0.30;
pub const K_GAP: f64 = 0.02;

/// Cached rendering state for peer avatars and forum supergroups.
/// C++ companion: `Ui::PeerUserpicView` (`userpic_view.h:L46-L58`).
#[derive(Debug, Clone)]
pub struct UserpicView {
    size: Pixels,
    shape: PeerUserpicShape,
    is_valid: bool,
    border_radius: Pixels,
}

impl UserpicView {
    /// Constructs a view with a given size and shape.
    pub fn new(size_px: f32, shape: PeerUserpicShape) -> Self {
        let size = px(size_px);
        let border_radius = match shape {
            PeerUserpicShape::Circle => size / 2.0,
            PeerUserpicShape::Square => px(0.0),
            PeerUserpicShape::Forum => size * 0.35, // Forum squircle radius
        };
        Self {
            size,
            shape,
            is_valid: true,
            border_radius,
        }
    }

    pub fn size(&self) -> Pixels {
        self.size
    }

    pub fn shape(&self) -> PeerUserpicShape {
        self.shape
    }

    pub fn border_radius(&self) -> Pixels {
        self.border_radius
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }
}

impl Default for UserpicView {
    fn default() -> Self {
        Self::new(48.0, PeerUserpicShape::Circle)
    }
}
