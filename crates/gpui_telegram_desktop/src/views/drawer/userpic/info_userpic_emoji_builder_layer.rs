//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_layer.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_layer.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_INFO_DESIRED_WIDTH: f32 = 380.0;
pub const K_USERPIC_BUILDER_EMOJI_LAYER_MIN_HEIGHT: f32 = 496.0;
pub const K_BOX_RADIUS: f32 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct InfoUserpicEmojiBuilderLayerUserpic {
    pub content_width: f32,
    pub min_height: f32,
    pub corner_radius: f32,
    pub close_by_outside_click: bool,
}

impl InfoUserpicEmojiBuilderLayerUserpic {
    pub fn new() -> Self {
        Self {
            content_width: K_INFO_DESIRED_WIDTH,
            min_height: K_USERPIC_BUILDER_EMOJI_LAYER_MIN_HEIGHT,
            corner_radius: K_BOX_RADIUS,
            close_by_outside_click: false,
        }
    }

    /// Upstream logic: `parentResized`
    /// Computes centered bounds: `(parentSize - currentSize) / 2`
    pub fn compute_centered_bounds(&self, parent_width: f32, parent_height: f32) -> LayerBounds {
        let current_width = self.content_width.min(parent_width);
        let current_height = self.min_height.min(parent_height);
        let left = ((parent_width - current_width) / 2.0).max(0.0);
        let top = ((parent_height - current_height) / 2.0).max(0.0);

        LayerBounds {
            x: left,
            y: top,
            width: current_width,
            height: current_height,
        }
    }
}

impl Default for InfoUserpicEmojiBuilderLayerUserpic {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoUserpicEmojiBuilderLayerUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(self.content_width))
            .min_h(px(self.min_height))
            .bg(rgb(0x17212b))
            .rounded(px(self.corner_radius))
            .shadow_lg()
            .p(px(16.0))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .child("Userpic Builder Modal Layer"),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_centering_bounds() {
        let layer = InfoUserpicEmojiBuilderLayerUserpic::new();
        assert!(!layer.close_by_outside_click);

        let parent_w = 1200.0;
        let parent_h = 800.0;
        let bounds = layer.compute_centered_bounds(parent_w, parent_h);

        assert_eq!(bounds.width, K_INFO_DESIRED_WIDTH);
        assert_eq!(bounds.height, K_USERPIC_BUILDER_EMOJI_LAYER_MIN_HEIGHT);
        assert_eq!(bounds.x, (1200.0 - 380.0) / 2.0); // 410.0
        assert_eq!(bounds.y, (800.0 - 496.0) / 2.0); // 152.0
    }
}
