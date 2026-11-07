//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_common.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_common.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_GRADIENT_ROTATION_DEGREES: i32 = 45;
pub const K_FORUM_USERPIC_RADIUS_MULTIPLIER: f32 = 0.28;

#[derive(Debug, Clone, PartialEq)]
pub struct EmojiBuilderResult {
    pub document_id: u64,
    pub colors: Vec<u32>,
    pub is_forum: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmojiBuilderStartData {
    pub document_id: u64,
    pub builder_color_index: usize,
    pub documents: Vec<u64>,
    pub gradient_editor_colors: Vec<u32>,
    pub is_forum: bool,
}

impl Default for EmojiBuilderStartData {
    fn default() -> Self {
        Self {
            document_id: 0,
            builder_color_index: 0,
            documents: Vec::new(),
            gradient_editor_colors: vec![0x20e2cdff, 0x4d8dffff],
            is_forum: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserpicShape {
    Circle,
    RoundForumRect,
    FullSquare,
}

#[derive(Debug, Clone)]
pub struct GradientSpec {
    pub width: f32,
    pub height: f32,
    pub colors: Vec<u32>,
    pub rotation_degrees: i32,
    pub shape: UserpicShape,
    pub corner_radius: f32,
}

impl GradientSpec {
    pub fn new(width: f32, height: f32, colors: Vec<u32>, shape: UserpicShape) -> Self {
        let corner_radius = match shape {
            UserpicShape::Circle => (width.min(height)) / 2.0,
            UserpicShape::RoundForumRect => (width.min(height)) * K_FORUM_USERPIC_RADIUS_MULTIPLIER,
            UserpicShape::FullSquare => 0.0,
        };

        Self {
            width,
            height,
            colors,
            rotation_degrees: K_GRADIENT_ROTATION_DEGREES,
            shape,
            corner_radius,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InfoUserpicEmojiBuilderCommonUserpic {
    pub default_rotation: i32,
    pub forum_multiplier: f32,
}

impl InfoUserpicEmojiBuilderCommonUserpic {
    pub fn new() -> Self {
        Self {
            default_rotation: K_GRADIENT_ROTATION_DEGREES,
            forum_multiplier: K_FORUM_USERPIC_RADIUS_MULTIPLIER,
        }
    }

    pub fn generate_spec(
        &self,
        width: f32,
        height: f32,
        colors: Vec<u32>,
        is_circle: bool,
        is_forum: bool,
    ) -> GradientSpec {
        let shape = if is_circle {
            UserpicShape::Circle
        } else if is_forum {
            UserpicShape::RoundForumRect
        } else {
            UserpicShape::FullSquare
        };
        GradientSpec::new(width, height, colors, shape)
    }
}

impl Render for InfoUserpicEmojiBuilderCommonUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .child(div().text_xs().text_color(rgb(0x707579)).child(format!("Gradient Angle: {}°", self.default_rotation)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_spec_generation_shapes() {
        let common = InfoUserpicEmojiBuilderCommonUserpic::new();
        assert_eq!(common.default_rotation, 45);

        // Circle shape
        let circle_spec = common.generate_spec(100.0, 100.0, vec![0xff0000ff], true, false);
        assert_eq!(circle_spec.shape, UserpicShape::Circle);
        assert_eq!(circle_spec.corner_radius, 50.0);
        assert_eq!(circle_spec.rotation_degrees, 45);

        // Forum rounded rect shape
        let forum_spec = common.generate_spec(100.0, 100.0, vec![0xff0000ff], false, true);
        assert_eq!(forum_spec.shape, UserpicShape::RoundForumRect);
        assert_eq!(forum_spec.corner_radius, 28.0); // 100.0 * 0.28
    }

    #[test]
    fn test_start_data_default_colors() {
        let data = EmojiBuilderStartData::default();
        assert_eq!(data.gradient_editor_colors.len(), 2);
        assert_eq!(data.builder_color_index, 0);
        assert!(!data.is_forum);
    }
}
