//! Target Symbol: IvMarkdownHistoryViewMediaMarkdown
//! Module 13: Instant View Slideshow Media Gallery & Gesture Transitions
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const SLIDESHOW_SLIDE_DURATION_MS: f32 = 280.0;
pub const SLIDESHOW_COMMIT_RATIO: f32 = 0.5;
pub const SLIDESHOW_WHEEL_STEP: f32 = 40.0;
pub const SLIDESHOW_COMMIT_VELOCITY: f32 = 500.0;

#[derive(Debug, Clone, PartialEq)]
pub struct SlideshowMediaItem {
    pub media_id: u64,
    pub source_url: String,
    pub caption: String,
    pub aspect_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownHistoryViewMediaMarkdown {
    pub items: Vec<SlideshowMediaItem>,
    pub current_index: usize,
    pub drag_offset_x: f32,
    pub is_dragging: bool,
}

impl Default for IvMarkdownHistoryViewMediaMarkdown {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            current_index: 0,
            drag_offset_x: 0.0,
            is_dragging: false,
        }
    }
}

impl IvMarkdownHistoryViewMediaMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_media_item(&mut self, media_id: u64, url: impl Into<String>, caption: impl Into<String>, aspect_ratio: f32) {
        self.items.push(SlideshowMediaItem {
            media_id,
            source_url: url.into(),
            caption: caption.into(),
            aspect_ratio: aspect_ratio.max(0.1),
        });
    }

    pub fn next_slide(&mut self) -> bool {
        if self.current_index + 1 < self.items.len() {
            self.current_index += 1;
            self.drag_offset_x = 0.0;
            true
        } else {
            false
        }
    }

    pub fn previous_slide(&mut self) -> bool {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.drag_offset_x = 0.0;
            true
        } else {
            false
        }
    }

    pub fn on_drag(&mut self, dx: f32) {
        self.is_dragging = true;
        self.drag_offset_x += dx;
    }

    pub fn commit_drag(&mut self, viewport_width: f32, velocity_x: f32) {
        self.is_dragging = false;
        let threshold = viewport_width * SLIDESHOW_COMMIT_RATIO;

        if velocity_x < -SLIDESHOW_COMMIT_VELOCITY || self.drag_offset_x < -threshold {
            self.next_slide();
        } else if velocity_x > SLIDESHOW_COMMIT_VELOCITY || self.drag_offset_x > threshold {
            self.previous_slide();
        } else {
            self.drag_offset_x = 0.0;
        }
    }

    pub fn current_item(&self) -> Option<&SlideshowMediaItem> {
        self.items.get(self.current_index)
    }
}

impl Render for IvMarkdownHistoryViewMediaMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let total = self.items.len();
        let cur = if total > 0 { self.current_index + 1 } else { 0 };
        let counter_str = format!("{} / {}", cur, total);

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .p_2()
            .bg(rgb(0x1a1e23))
            .rounded_md()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a9aa8))
                    .child(format!("Slideshow: {}", counter_str)),
            )
    }
}
