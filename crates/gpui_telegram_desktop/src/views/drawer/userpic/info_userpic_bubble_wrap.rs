//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_bubble_wrap.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_bubble_wrap.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_BUBBLE_PADDING: f32 = 5.0;
pub const K_HORIZONTAL_OFFSET: f32 = 1.0;
pub const K_VERTICAL_OFFSET: f32 = 2.0;
pub const K_OPACITY_STEP1: f32 = 0.2;
pub const K_OPACITY_STEP2: f32 = 0.4;
pub const K_BUBBLE_RADIUS_SMALL: f32 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BubbleRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BubbleRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Upstream logic: `BubbleWrapInnerRect`
    pub fn inner_rect(&self) -> Self {
        Self {
            x: self.x + K_BUBBLE_PADDING,
            y: self.y + K_BUBBLE_PADDING,
            width: (self.width - K_BUBBLE_PADDING * 2.0).max(0.0),
            height: (self.height - K_BUBBLE_PADDING * 2.0).max(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoUserpicBubbleWrapUserpic {
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
    pub show_shadow: bool,
}

impl InfoUserpicBubbleWrapUserpic {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            corner_radius: K_BUBBLE_RADIUS_SMALL,
            show_shadow: true,
        }
    }

    pub fn inner_rect(&self) -> BubbleRect {
        BubbleRect::new(0.0, 0.0, self.width, self.height).inner_rect()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

impl Default for InfoUserpicBubbleWrapUserpic {
    fn default() -> Self {
        Self::new(356.0, 120.0)
    }
}

impl Render for InfoUserpicBubbleWrapUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let inner = self.inner_rect();

        div()
            .flex()
            .flex_col()
            .w(px(self.width))
            .h(px(self.height))
            .p(px(K_BUBBLE_PADDING))
            .child(
                div()
                    .size_full()
                    .bg(rgb(0x17212b))
                    .rounded(px(self.corner_radius))
                    .border_1()
                    .border_color(rgb(0x242f3d))
                    .shadow_md()
                    .p(px(8.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child("Userpic Bubble Content"),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_wrap_inner_rect_math() {
        let wrap = InfoUserpicBubbleWrapUserpic::new(356.0, 216.0);
        let inner = wrap.inner_rect();

        assert_eq!(inner.x, 5.0);
        assert_eq!(inner.y, 5.0);
        assert_eq!(inner.width, 346.0);
        assert_eq!(inner.height, 206.0);
    }

    #[test]
    fn test_bubble_wrap_clamping() {
        let mut wrap = InfoUserpicBubbleWrapUserpic::new(8.0, 8.0);
        let inner = wrap.inner_rect();
        assert_eq!(inner.width, 0.0);
        assert_eq!(inner.height, 0.0);

        wrap.resize(100.0, 50.0);
        assert_eq!(wrap.inner_rect().width, 90.0);
        assert_eq!(wrap.inner_rect().height, 40.0);
    }
}
