//! Target Symbol: IvMarkdownArticleQuoteMarkdown
//! Module 13: Instant View Quote Segments, Intervals & Visual Highlights
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_DISPLAY_POSITION: i32 = 0x7FFFFFFF;

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteInterval {
    pub quote_id: u64,
    pub start_pos: i32,
    pub end_pos: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteSegmentGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub has_accent_bar: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteHighlightTarget {
    pub quote_id: u64,
    pub is_highlighted: bool,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleQuoteMarkdown {
    pub quote_intervals: Vec<QuoteInterval>,
    pub segments: Vec<QuoteSegmentGeometry>,
    pub highlighted_id: Option<u64>,
}

impl Default for IvMarkdownArticleQuoteMarkdown {
    fn default() -> Self {
        Self {
            quote_intervals: Vec::new(),
            segments: Vec::new(),
            highlighted_id: None,
        }
    }
}

impl IvMarkdownArticleQuoteMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_interval(&mut self, quote_id: u64, start: i32, end: i32) {
        let safe_end = end.min(MAX_DISPLAY_POSITION);
        self.quote_intervals.push(QuoteInterval {
            quote_id,
            start_pos: start,
            end_pos: safe_end,
        });
    }

    pub fn add_segment(&mut self, x: f32, y: f32, width: f32, height: f32, has_accent_bar: bool) {
        self.segments.push(QuoteSegmentGeometry {
            x,
            y,
            width,
            height,
            has_accent_bar,
        });
    }

    pub fn set_highlight(&mut self, quote_id: u64) {
        self.highlighted_id = Some(quote_id);
    }

    pub fn clear_highlight(&mut self) {
        self.highlighted_id = None;
    }

    pub fn find_quote_at(&self, pos: i32) -> Option<u64> {
        for interval in &self.quote_intervals {
            if pos >= interval.start_pos && pos <= interval.end_pos {
                return Some(interval.quote_id);
            }
        }
        None
    }
}

impl Render for IvMarkdownArticleQuoteMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_hl = self.highlighted_id.is_some();
        let bar_color = if is_hl { rgb(0x40a7e3) } else { rgb(0x56606a) };

        div()
            .flex()
            .gap_2()
            .p_2()
            .child(
                div()
                    .w(px(3.0))
                    .rounded_full()
                    .bg(bar_color),
            )
            .child(
                div()
                    .text_xs()
                    .italic()
                    .text_color(rgb(0x9aa7b4))
                    .child(format!("Quote: {} intervals", self.quote_intervals.len())),
            )
    }
}
