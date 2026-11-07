//! Target Symbol: IvMarkdownViewMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_view.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_ZOOM_STEP: i32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownPreviewScrollMode {
    #[default]
    Instant,
    Animated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverscrollType {
    #[default]
    None,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownArticleSearchMatch {
    pub paragraph: usize,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownArticleSearchSource {
    pub query: String,
    pub matches: Vec<MarkdownArticleSearchMatch>,
}

#[derive(Debug, Clone, Default)]
pub struct PendingEmbedState {
    pub embed_url: String,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RpWidget {
    pub width: f32,
    pub height: f32,
    pub visible: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownPreviewRoot {
    pub scroll_y: f32,
    pub zoom_percent: i32,
    pub active_anchor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownViewMarkdown {
    pub root: MarkdownPreviewRoot,
    pub search: MarkdownArticleSearchSource,
    pub scroll_mode: MarkdownPreviewScrollMode,
    pub overscroll: OverscrollType,
}

impl Default for IvMarkdownViewMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownViewMarkdown {
    pub fn new() -> Self {
        Self {
            root: MarkdownPreviewRoot {
                scroll_y: 0.0,
                zoom_percent: 100,
                active_anchor: None,
            },
            search: MarkdownArticleSearchSource::default(),
            scroll_mode: MarkdownPreviewScrollMode::Instant,
            overscroll: OverscrollType::None,
        }
    }

    pub fn zoom_in(&mut self) {
        self.root.zoom_percent = (self.root.zoom_percent + K_ZOOM_STEP).min(300);
    }

    pub fn zoom_out(&mut self) {
        self.root.zoom_percent = (self.root.zoom_percent - K_ZOOM_STEP).max(50);
    }

    pub fn scroll_to_anchor(&mut self, anchor: &str, mode: MarkdownPreviewScrollMode) {
        self.root.active_anchor = Some(anchor.to_string());
        self.scroll_mode = mode;
    }

    pub fn set_search_matches(&mut self, query: String, matches: Vec<MarkdownArticleSearchMatch>) {
        self.search = MarkdownArticleSearchSource { query, matches };
    }
}

impl Render for IvMarkdownViewMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let zoom = self.root.zoom_percent;
        let scroll = self.root.scroll_y;
        let matches = self.search.matches.len();
        let anchor = self.root.active_anchor.as_deref().unwrap_or("None");

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
                    .child("Instant View Markdown Article Root"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Zoom: {}%", zoom))
                    .child(format!("Scroll Y: {:.0}px", scroll))
                    .child(format!("Active Anchor: {}", anchor))
                    .child(format!("Search Matches: {}", matches)),
            )
    }
}
