//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_preview.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_preview.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::f32::consts::SQRT_2;

pub const K_FRAME_RADIUS_PERCENT: f32 = 0.25;
pub const K_ATTEMPTS_TO_DRAW_FIRST_FRAME: i32 = 3000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
}

#[derive(Debug, Clone)]
pub struct InfoUserpicEmojiBuilderPreviewUserpic {
    pub size: f32,
    pub is_forum: bool,
    pub colors: Vec<u32>,
    pub previous_colors: Option<Vec<u32>>,
    pub animation_progress: f32,
    pub document_id: Option<u64>,
}

impl InfoUserpicEmojiBuilderPreviewUserpic {
    pub fn new(size: f32, is_forum: bool) -> Self {
        Self {
            size,
            is_forum,
            colors: vec![0x20e2cdff, 0x4d8dffff],
            previous_colors: None,
            animation_progress: 1.0,
            document_id: None,
        }
    }

    /// Upstream mathematical formula: `_emojiSize = SafeRound(_size / M_SQRT2)`
    pub fn compute_emoji_size(size: f32) -> f32 {
        (size / SQRT_2).round()
    }

    /// Upstream frame geometry: `_frameGeometry(Rect(Size(_size)) - Margins((_size - _emojiSize) / 2))`
    pub fn compute_frame_geometry(&self) -> FrameGeometry {
        let emoji_size = Self::compute_emoji_size(self.size);
        let margin = (self.size - emoji_size) / 2.0;
        let corner_radius = emoji_size * K_FRAME_RADIUS_PERCENT;

        FrameGeometry {
            x: margin,
            y: margin,
            width: emoji_size,
            height: emoji_size,
            corner_radius,
        }
    }

    pub fn set_document_id(&mut self, id: u64) {
        self.document_id = Some(id);
    }

    pub fn set_gradient_colors(&mut self, colors: Vec<u32>) {
        if self.colors != colors {
            self.previous_colors = Some(self.colors.clone());
            self.colors = colors;
            self.animation_progress = 0.0;
        }
    }

    pub fn update_animation(&mut self, delta_progress: f32) {
        self.animation_progress = (self.animation_progress + delta_progress).min(1.0);
        if self.animation_progress >= 1.0 {
            self.previous_colors = None;
        }
    }
}

impl Default for InfoUserpicEmojiBuilderPreviewUserpic {
    fn default() -> Self {
        Self::new(100.0, false)
    }
}

impl Render for InfoUserpicEmojiBuilderPreviewUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let frame = self.compute_frame_geometry();
        let primary_color = *self.colors.first().unwrap_or(&0x20e2cdff);
        let secondary_color = *self.colors.last().unwrap_or(&0x4d8dffff);
        let radius = if self.is_forum { px(self.size * 0.28) } else { px(self.size / 2.0) };

        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(radius)
            .bg(rgba(primary_color))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(frame.width))
                    .h(px(frame.height))
                    .rounded(px(frame.corner_radius))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child(if self.document_id.is_some() { "🎭" } else { "😀" }),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_size_sqrt2_formula() {
        let size = 100.0;
        let emoji_size = InfoUserpicEmojiBuilderPreviewUserpic::compute_emoji_size(size);
        // 100 / sqrt(2) ≈ 70.71 -> rounded to 71
        assert_eq!(emoji_size, 71.0);

        let preview = InfoUserpicEmojiBuilderPreviewUserpic::new(100.0, false);
        let frame = preview.compute_frame_geometry();
        assert_eq!(frame.width, 71.0);
        assert_eq!(frame.height, 71.0);
        assert_eq!(frame.x, (100.0 - 71.0) / 2.0); // 14.5
        assert_eq!(frame.corner_radius, 71.0 * 0.25);
    }

    #[test]
    fn test_gradient_transition_progress() {
        let mut preview = InfoUserpicEmojiBuilderPreviewUserpic::new(100.0, false);
        let initial_colors = preview.colors.clone();

        preview.set_gradient_colors(vec![0xff1122ff, 0x334455ff]);
        assert_eq!(preview.previous_colors, Some(initial_colors));
        assert_eq!(preview.animation_progress, 0.0);

        preview.update_animation(0.5);
        assert_eq!(preview.animation_progress, 0.5);
        assert!(preview.previous_colors.is_some());

        preview.update_animation(0.6);
        assert_eq!(preview.animation_progress, 1.0);
        assert!(preview.previous_colors.is_none());
    }
}
