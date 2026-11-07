//! Target Symbol: IvMarkdownArticleScrollForwarderMarkdown
//! Module 13: Instant View Nested Scroll Forwarder & Gesture Dispatcher
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const SCROLL_FORWARD_THRESHOLD: f32 = 2.0;
pub const SCROLL_DAMPING_FACTOR: f32 = 0.85;

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollForwarderEvent {
    pub delta_x: f32,
    pub delta_y: f32,
    pub is_precise: bool,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleScrollForwarderMarkdown {
    pub threshold: f32,
    pub damping: f32,
    pub accumulated_delta_y: f32,
    pub forwarded_events_count: usize,
}

impl Default for IvMarkdownArticleScrollForwarderMarkdown {
    fn default() -> Self {
        Self {
            threshold: SCROLL_FORWARD_THRESHOLD,
            damping: SCROLL_DAMPING_FACTOR,
            accumulated_delta_y: 0.0,
            forwarded_events_count: 0,
        }
    }
}

impl IvMarkdownArticleScrollForwarderMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_damping(damping: f32) -> Self {
        Self {
            damping: damping.clamp(0.1, 1.0),
            ..Default::default()
        }
    }

    pub fn forward_wheel(&mut self, dx: f32, dy: f32, is_precise: bool) -> Option<f32> {
        self.accumulated_delta_y += dy * self.damping;
        if self.accumulated_delta_y.abs() >= self.threshold {
            let out_dy = self.accumulated_delta_y;
            self.accumulated_delta_y = 0.0;
            self.forwarded_events_count += 1;
            Some(out_dy)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_delta_y = 0.0;
    }
}

impl Render for IvMarkdownArticleScrollForwarderMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.forwarded_events_count;
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Scroll Forwarder: {} events forwarded", count))
    }
}
