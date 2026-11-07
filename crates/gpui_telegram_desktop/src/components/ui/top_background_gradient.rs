//! Upstream C++ source: `Telegram/SourceFiles/ui/top_background_gradient.cpp:L1-L283` & `top_background_gradient.h:L1-L79`
//! Authentic GPUI implementation of top header background gradient and pattern points.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels, Point};

/// Single background decoration pattern point from `top_background_gradient.h:L23-L29`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PatternPoint {
    pub position: Point<Pixels>,
    pub scale: f32,
    pub alpha: f32,
    pub angle_rad: f32,
}

/// Dynamic top bar background gradient with decorative pattern matrix.
/// C++ companion: `Ui::TopBackgroundGradient` (`top_background_gradient.cpp:L24-L283`).
#[derive(Debug, Clone)]
pub struct TopBackgroundGradient {
    gradient_top: Hsla,
    gradient_bottom: Hsla,
    pattern_color: Hsla,
    points: Vec<PatternPoint>,
    height: Pixels,
}

impl TopBackgroundGradient {
    /// Creates a top gradient with specific color bounds.
    pub fn new(top: Hsla, bottom: Hsla, pattern: Hsla, height: f32) -> Self {
        Self {
            gradient_top: top,
            gradient_bottom: bottom,
            pattern_color: pattern,
            points: Vec::new(),
            height: px(height),
        }
    }

    /// Generates standard decorative points layout across the header bounds.
    pub fn generate_default_pattern(&mut self, width: Pixels) {
        self.points.clear();
        let step_x = px(64.0);
        let step_y = px(48.0);
        let mut y = px(12.0);
        while y < self.height {
            let mut x = px(16.0);
            while x < width {
                self.points.push(PatternPoint {
                    position: Point { x, y },
                    scale: 1.0,
                    alpha: 0.15,
                    angle_rad: 0.0,
                });
                x += step_x;
            }
            y += step_y;
        }
    }

    pub fn points(&self) -> &[PatternPoint] {
        &self.points
    }

    pub fn gradient_colors(&self) -> (Hsla, Hsla) {
        (self.gradient_top, self.gradient_bottom)
    }

    pub fn height(&self) -> Pixels {
        self.height
    }
}

impl Default for TopBackgroundGradient {
    fn default() -> Self {
        Self::new(
            hsla(210.0, 0.85, 0.55, 1.0),
            hsla(215.0, 0.90, 0.45, 1.0),
            hsla(0.0, 0.0, 1.0, 0.20),
            120.0,
        )
    }
}
