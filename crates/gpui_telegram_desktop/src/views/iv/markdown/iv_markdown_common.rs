//! Target Symbol: IvMarkdownCommonMarkdown
//! Module 13: Instant View Common Media Block Contracts & Geometry Clamping
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_MEDIA_WIDTH: f32 = 1200.0;
pub const MIN_MEDIA_ASPECT: f32 = 0.25;
pub const MAX_MEDIA_ASPECT: f32 = 4.00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaBlockType {
    #[default]
    Photo,
    Document,
    Video,
    Audio,
    Collage,
    ChannelPost,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaDimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownCommonMarkdown {
    pub max_width: f32,
    pub min_aspect: f32,
    pub max_aspect: f32,
}

impl Default for IvMarkdownCommonMarkdown {
    fn default() -> Self {
        Self {
            max_width: MAX_MEDIA_WIDTH,
            min_aspect: MIN_MEDIA_ASPECT,
            max_aspect: MAX_MEDIA_ASPECT,
        }
    }
}

impl IvMarkdownCommonMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clamp_dimensions(&self, width: f32, height: f32) -> MediaDimensions {
        let safe_w = width.max(1.0);
        let safe_h = height.max(1.0);
        let raw_aspect = safe_w / safe_h;
        let clamped_aspect = raw_aspect.clamp(self.min_aspect, self.max_aspect);

        let target_w = safe_w.min(self.max_width);
        let target_h = target_w / clamped_aspect;

        MediaDimensions {
            width: target_w,
            height: target_h,
        }
    }

    pub fn resolve_aspect_ratio(&self, width: f32, height: f32) -> f32 {
        if height <= 0.0 {
            1.0
        } else {
            (width / height).clamp(self.min_aspect, self.max_aspect)
        }
    }
}

impl Render for IvMarkdownCommonMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Common Media Contracts: max_w {:.0}px", self.max_width))
    }
}
