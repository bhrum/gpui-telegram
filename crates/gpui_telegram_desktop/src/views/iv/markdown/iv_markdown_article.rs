//! Target Symbol: IvMarkdownArticleMarkdown
//! Module 13: Instant View Markdown Article Host & Layout Coordinator
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MIN_HIGHLIGHT_ALPHA: f32 = 0.12;
pub const HIGHLIGHT_FADE_DURATION_MS: f32 = 600.0;

#[derive(Debug, Clone, PartialEq)]
pub struct ArticleHighlightEntry {
    pub target_block_id: u64,
    pub alpha: f32,
    pub timestamp_ms: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollOwnerIdentity {
    pub owner_id: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ArticleSelectionPosition {
    pub block_index: usize,
    pub char_offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelatedArticleItem {
    pub url: String,
    pub title: String,
    pub site_name: String,
    pub preview_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleMarkdown {
    pub page_id: u64,
    pub site_name: String,
    pub article_title: String,
    pub author_name: String,
    pub published_date: String,
    pub scroll_owner: ScrollOwnerIdentity,
    pub highlights: Vec<ArticleHighlightEntry>,
    pub selection_range: Option<(ArticleSelectionPosition, ArticleSelectionPosition)>,
    pub related_articles: Vec<RelatedArticleItem>,
    pub scroll_offset_y: f32,
}

impl Default for IvMarkdownArticleMarkdown {
    fn default() -> Self {
        Self {
            page_id: 0,
            site_name: String::new(),
            article_title: String::new(),
            author_name: String::new(),
            published_date: String::new(),
            scroll_owner: ScrollOwnerIdentity::default(),
            highlights: Vec::new(),
            selection_range: None,
            related_articles: Vec::new(),
            scroll_offset_y: 0.0,
        }
    }
}

impl IvMarkdownArticleMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_article(page_id: u64, site_name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            page_id,
            site_name: site_name.into(),
            article_title: title.into(),
            ..Default::default()
        }
    }

    pub fn set_metadata(&mut self, author: impl Into<String>, date: impl Into<String>) {
        self.author_name = author.into();
        self.published_date = date.into();
    }

    pub fn trigger_block_highlight(&mut self, block_id: u64, now_ms: i64) {
        self.highlights.retain(|h| h.target_block_id != block_id);
        self.highlights.push(ArticleHighlightEntry {
            target_block_id: block_id,
            alpha: 1.0,
            timestamp_ms: now_ms,
        });
    }

    pub fn update_highlights(&mut self, now_ms: i64) {
        self.highlights.retain_mut(|h| {
            let elapsed = (now_ms - h.timestamp_ms).max(0) as f32;
            if elapsed >= HIGHLIGHT_FADE_DURATION_MS {
                false
            } else {
                h.alpha = 1.0 - (elapsed / HIGHLIGHT_FADE_DURATION_MS);
                h.alpha >= MIN_HIGHLIGHT_ALPHA
            }
        });
    }

    pub fn set_selection(&mut self, start: ArticleSelectionPosition, end: ArticleSelectionPosition) {
        self.selection_range = Some((start, end));
    }

    pub fn clear_selection(&mut self) {
        self.selection_range = None;
    }

    pub fn add_related_article(&mut self, item: RelatedArticleItem) {
        self.related_articles.push(item);
    }

    pub fn set_scroll_offset(&mut self, offset_y: f32) {
        self.scroll_offset_y = offset_y.max(0.0);
    }
}

impl Render for IvMarkdownArticleMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let site = self.site_name.clone();
        let title = self.article_title.clone();
        let author = self.author_name.clone();
        let date = self.published_date.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .bg(rgb(0x181a1f))
            .text_color(rgb(0xe0e4e8))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .mb_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x7c8a99))
                            .child(site),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child(title),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .text_xs()
                            .text_color(rgb(0x9aa7b4))
                            .child(author)
                            .child(date),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .id("article-content")
                    .child(div().text_sm().child("Article document loaded")),
            )
    }
}
