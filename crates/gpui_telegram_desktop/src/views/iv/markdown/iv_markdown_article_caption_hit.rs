//! Target Symbol: IvMarkdownArticleCaptionHitMarkdown
//! Module 13: Instant View Article Caption Hit Testing & Link Routing
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const CAPTION_PADDING: f32 = 6.0;
pub const CAPTION_HIT_TOLERANCE: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaptionTargetKind {
    #[default]
    Photo,
    Document,
    Video,
    Audio,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaptionHitResult {
    None,
    Link(String),
    Hashtag(String),
    Mention(String),
    MediaOpen,
}

#[derive(Debug, Clone)]
pub struct CaptionHitBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub payload: CaptionHitResult,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleCaptionHitMarkdown {
    pub target_kind: CaptionTargetKind,
    pub caption_text: String,
    pub hit_boxes: Vec<CaptionHitBox>,
    pub hovered_result: CaptionHitResult,
}

impl Default for IvMarkdownArticleCaptionHitMarkdown {
    fn default() -> Self {
        Self {
            target_kind: CaptionTargetKind::Photo,
            caption_text: String::new(),
            hit_boxes: Vec::new(),
            hovered_result: CaptionHitResult::None,
        }
    }
}

impl IvMarkdownArticleCaptionHitMarkdown {
    pub fn new(target_kind: CaptionTargetKind) -> Self {
        Self {
            target_kind,
            caption_text: String::new(),
            hit_boxes: Vec::new(),
            hovered_result: CaptionHitResult::None,
        }
    }

    pub fn set_caption(&mut self, text: impl Into<String>) {
        self.caption_text = text.into();
    }

    pub fn add_hit_box(&mut self, x: f32, y: f32, width: f32, height: f32, payload: CaptionHitResult) {
        self.hit_boxes.push(CaptionHitBox {
            x,
            y,
            width,
            height,
            payload,
        });
    }

    pub fn clear_hit_boxes(&mut self) {
        self.hit_boxes.clear();
        self.hovered_result = CaptionHitResult::None;
    }

    pub fn hit_test(&self, px: f32, py: f32) -> CaptionHitResult {
        for box_item in &self.hit_boxes {
            let left = box_item.x - CAPTION_HIT_TOLERANCE;
            let right = box_item.x + box_item.width + CAPTION_HIT_TOLERANCE;
            let top = box_item.y - CAPTION_HIT_TOLERANCE;
            let bottom = box_item.y + box_item.height + CAPTION_HIT_TOLERANCE;

            if px >= left && px <= right && py >= top && py <= bottom {
                return box_item.payload.clone();
            }
        }
        CaptionHitResult::None
    }

    pub fn on_mouse_move(&mut self, px: f32, py: f32) {
        self.hovered_result = self.hit_test(px, py);
    }
}

impl Render for IvMarkdownArticleCaptionHitMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.caption_text.clone();
        let is_hovered = self.hovered_result != CaptionHitResult::None;

        div()
            .flex()
            .flex_col()
            .p_2()
            .text_xs()
            .text_color(if is_hovered { rgb(0x40a7e3) } else { rgb(0x8a9aa8) })
            .child(text)
    }
}
