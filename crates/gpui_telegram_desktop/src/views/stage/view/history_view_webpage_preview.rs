//! History View Webpage Preview View Component (`history_view_webpage_preview`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_webpage_preview.h` (L1-L24)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_webpage_preview.cpp` (L1-L92)
//!
//! Extracts title, description, and preview imagery from web page metadata for rich URL cards
//! in chat history, enforcing upstream Telegram hierarchy and square-center crop heuristics.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Title and description pair extracted from webpage metadata.
/// Upstream: `history_view_webpage_preview.h:L12-L15` (`struct WebPageText`)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WebPageText {
    pub title: String,
    pub description: String,
}

/// Upstream extraction algorithm prioritizing siteName, title, description, author, and filename/url.
/// Upstream: `history_view_webpage_preview.cpp:L15-L57` (`TitleAndDescriptionFromWebPage`)
pub fn title_and_description_from_web_page(
    site_name: &str,
    title: &str,
    description: &str,
    author: &str,
    filename: &str,
    url: &str,
) -> WebPageText {
    let filename_or_url = if !filename.is_empty() { filename } else { url };
    let author_or_filename = if !author.is_empty() { author } else { filename_or_url };
    let description_or_author = if !description.is_empty() { description } else { author_or_filename };

    let (res_title, res_desc) = if site_name.is_empty() {
        if title.is_empty() {
            if description.is_empty() {
                (author.to_string(), filename_or_url.to_string())
            } else {
                (description.to_string(), author_or_filename.to_string())
            }
        } else {
            (title.to_string(), description_or_author.to_string())
        }
    } else {
        let desc = if title.is_empty() {
            description_or_author
        } else {
            title
        };
        (site_name.to_string(), desc.to_string())
    };

    WebPageText {
        title: res_title,
        description: res_desc,
    }
}

/// Computes crop bounds for non-square preview thumbnails to center them into a 1:1 box.
/// Upstream: `history_view_webpage_preview.cpp:L77-L86`
#[inline]
pub fn compute_thumbnail_crop_offset(width: u32, height: u32) -> (u32, u32, u32, u32) {
    if width == height {
        (0, 0, width, height)
    } else if width > height {
        ((width - height) / 2, 0, height, height)
    } else {
        (0, (height - width) / 2, width, width)
    }
}

/// Authentic GPUI View for webpage previews embedded in message bubbles.
/// Upstream: `history_view_webpage_preview.h:L17-L22`
#[derive(Debug, Clone)]
pub struct HistoryViewWebpagePreviewView {
    pub url: String,
    pub site_name: String,
    pub title: String,
    pub description: String,
    pub thumb_width: u32,
    pub thumb_height: u32,
    pub has_thumbnail: bool,
    pub is_selected: bool,
}

impl HistoryViewWebpagePreviewView {
    /// Constructs a webpage preview card.
    pub fn new(url: impl Into<String>) -> Self {
        let u = url.into();
        Self {
            url: u.clone(),
            site_name: String::new(),
            title: String::new(),
            description: String::new(),
            thumb_width: 0,
            thumb_height: 0,
            has_thumbnail: false,
            is_selected: false,
        }
    }

    /// Populates preview texts according to Telegram's resolution rules.
    /// Upstream: `history_view_webpage_preview.cpp:L15-L57`
    pub fn populate_from_metadata(
        &mut self,
        site_name: &str,
        title: &str,
        description: &str,
        author: &str,
        filename: &str,
    ) {
        self.site_name = site_name.to_string();
        let extracted = title_and_description_from_web_page(
            site_name,
            title,
            description,
            author,
            filename,
            &self.url,
        );
        self.title = extracted.title;
        self.description = extracted.description;
    }

    /// Sets thumbnail dimensions.
    pub fn set_thumbnail_dimensions(&mut self, width: u32, height: u32) {
        self.thumb_width = width;
        self.thumb_height = height;
        self.has_thumbnail = width > 0 && height > 0;
    }

    /// Calculates thumbnail center crop offset.
    pub fn thumbnail_crop(&self) -> (u32, u32, u32, u32) {
        compute_thumbnail_crop_offset(self.thumb_width, self.thumb_height)
    }
}

impl Default for HistoryViewWebpagePreviewView {
    fn default() -> Self {
        Self::new("https://telegram.org")
    }
}

impl Render for HistoryViewWebpagePreviewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let has_thumb = self.has_thumbnail;

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(rgb(0xF5F7FA))
            .border_l_2()
            .border_color(rgb(0x27A6E5))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x27A6E5))
                                    .child(if self.site_name.is_empty() { self.title.clone() } else { self.site_name.clone() })
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x222222))
                                    .child(self.title.clone())
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x666666))
                                    .child(self.description.clone())
                            )
                    )
                    .children(if has_thumb {
                        Some(
                            div()
                                .w(px(60.0))
                                .h(px(60.0))
                                .rounded(px(4.0))
                                .bg(rgb(0xDDDDDD))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(18.0))
                                .child("🌐")
                        )
                    } else {
                        None
                    })
            )
    }
}
