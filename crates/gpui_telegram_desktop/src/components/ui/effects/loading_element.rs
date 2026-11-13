//! Upstream: `Telegram/SourceFiles/ui/effects/loading_element.cpp:L23-L41`
//! Upstream: `Telegram/SourceFiles/ui/effects/loading_element.h:L18-L63`
//!
//! Reusable skeleton loading placeholders (lines, avatar circles, multiline text)
//! with standard rounded radius `thickness / 2.0` and line height spacing.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderShape {
    Line,
    Circle,
    RoundedRect,
}

#[derive(Debug, Clone)]
pub struct LoadingElementEffects {
    shape: PlaceholderShape,
    width: f32,
    thickness: f32,
    skip_margin: f32,
    pulse_alpha: f32,
}

impl LoadingElementEffects {
    pub fn new(shape: PlaceholderShape, width: f32, thickness: f32) -> Self {
        Self {
            shape,
            width: if width > 0.0 { width } else { 100.0 },
            thickness: if thickness > 0.0 { thickness } else { 12.0 },
            skip_margin: 4.0,
            pulse_alpha: 0.6,
        }
    }

    /// C++: `loading_element.cpp:L29-L31`: `height() = thickness + skip`
    pub fn total_height(&self) -> f32 {
        self.thickness + self.skip_margin
    }

    /// C++: `loading_element.cpp:L38`: `radius = thickness / 2.`
    pub fn corner_radius(&self) -> f32 {
        self.thickness / 2.0
    }

    pub fn set_pulse_alpha(&mut self, alpha: f32) {
        self.pulse_alpha = alpha.clamp(0.2, 1.0);
    }

    pub fn shape(&self) -> PlaceholderShape {
        self.shape
    }
}

impl Default for LoadingElementEffects {
    fn default() -> Self {
        Self::new(PlaceholderShape::Line, 120.0, 10.0)
    }
}

impl Render for LoadingElementEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let radius = self.corner_radius();
        let bg = rgba(0x2B527888);

        match self.shape {
            PlaceholderShape::Circle => div()
                .size(px(self.thickness))
                .rounded(px(self.thickness / 2.0))
                .bg(bg),
            _ => div()
                .w(px(self.width))
                .h(px(self.thickness))
                .rounded(px(radius))
                .bg(bg),
        }
    }
}
