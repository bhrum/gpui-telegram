//! Target Symbol: IvMarkdownSlideshowChromeMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_slideshow_chrome.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_INACTIVE_DOT_OPACITY: f32 = 0.5;

#[derive(Debug, Clone, Default)]
pub struct MarkdownGroupedMedia {
    pub items_count: usize,
    pub current_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SlideshowNavRects {
    pub prev_x: f32,
    pub prev_y: f32,
    pub next_x: f32,
    pub next_y: f32,
    pub button_size: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SlideshowDotsGeometry {
    pub dot_radius: f32,
    pub dot_spacing: f32,
    pub total_width: f32,
    pub total_height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SlideshowDotsBackdrop {
    pub width: f32,
    pub height: f32,
    pub opacity: f32,
}

pub fn compute_slideshow_nav_rects(width: f32, height: f32, button_size: f32) -> SlideshowNavRects {
    let padding = 16.0;
    let y = (height - button_size) / 2.0;
    SlideshowNavRects {
        prev_x: padding,
        prev_y: y,
        next_x: width - button_size - padding,
        next_y: y,
        button_size,
    }
}

pub fn compute_slideshow_dots(items_count: usize, dot_radius: f32, dot_spacing: f32) -> SlideshowDotsGeometry {
    let diameter = dot_radius * 2.0;
    let total_width = if items_count > 0 {
        (items_count as f32 * diameter) + ((items_count - 1) as f32 * dot_spacing)
    } else {
        0.0
    };
    SlideshowDotsGeometry {
        dot_radius,
        dot_spacing,
        total_width,
        total_height: diameter + 8.0,
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownSlideshowChromeMarkdown {
    pub media: MarkdownGroupedMedia,
    pub nav_rects: SlideshowNavRects,
    pub dots_geo: SlideshowDotsGeometry,
}

impl Default for IvMarkdownSlideshowChromeMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownSlideshowChromeMarkdown {
    pub fn new() -> Self {
        Self {
            media: MarkdownGroupedMedia {
                items_count: 5,
                current_index: 0,
            },
            nav_rects: compute_slideshow_nav_rects(600.0, 400.0, 40.0),
            dots_geo: compute_slideshow_dots(5, 4.0, 6.0),
        }
    }

    pub fn next(&mut self) {
        if self.media.items_count > 0 {
            self.media.current_index = (self.media.current_index + 1) % self.media.items_count;
        }
    }

    pub fn prev(&mut self) {
        if self.media.items_count > 0 {
            if self.media.current_index == 0 {
                self.media.current_index = self.media.items_count - 1;
            } else {
                self.media.current_index -= 1;
            }
        }
    }
}

impl Render for IvMarkdownSlideshowChromeMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let current = self.media.current_index;
        let count = self.media.items_count;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1e2124))
            .text_color(rgb(0xf0f0f0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Slideshow Chrome"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Current Index: {} / {}", current + 1, count))
                    .child(format!("Dots Width: {:.1} px", self.dots_geo.total_width)),
            )
    }
}
