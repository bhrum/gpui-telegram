//! InlineBotResult: Parser and data model for inline bot query results.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_result.h:L40-L100`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_result.cpp:L35-L85`
#![allow(dead_code)]

use gpui::*;

/// Mime type for video thumbnail (upstream: `kVideoThumbMime = "video/mp4"`).
pub const K_VIDEO_THUMB_MIME: &str = "video/mp4";

/// Result type classification (upstream: `InlineBots::Result::Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InlineResultType {
    #[default]
    Unknown,
    Photo,
    Video,
    Audio,
    Sticker,
    File,
    Gif,
    Article,
    Contact,
    Venue,
    Geo,
    Game,
}

/// Convert upstream type string to enum (upstream: `inline_bot_result.cpp:L63-L76`).
pub fn parse_result_type(s: &str) -> InlineResultType {
    match s {
        "photo" => InlineResultType::Photo,
        "video" => InlineResultType::Video,
        "audio" | "voice" => InlineResultType::Audio,
        "sticker" => InlineResultType::Sticker,
        "file" => InlineResultType::File,
        "gif" => InlineResultType::Gif,
        "article" => InlineResultType::Article,
        "contact" => InlineResultType::Contact,
        "venue" => InlineResultType::Venue,
        "geo" => InlineResultType::Geo,
        "game" => InlineResultType::Game,
        _ => InlineResultType::Unknown,
    }
}

/// Authentic InlineBotResult model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineBotResult {
    pub id: String,
    pub query_id: u64,
    pub result_type: InlineResultType,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub thumb_url: Option<String>,
    pub content_url: Option<String>,
    pub document_id: Option<u64>,
    pub photo_id: Option<u64>,
    pub updated_at: i64,
}

impl InlineBotResult {
    pub fn new(id: impl Into<String>, query_id: u64, result_type: InlineResultType) -> Self {
        Self {
            id: id.into(),
            query_id,
            result_type,
            title: None,
            description: None,
            url: None,
            thumb_url: None,
            content_url: None,
            document_id: None,
            photo_id: None,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_gallery_type(&self) -> bool {
        matches!(
            self.result_type,
            InlineResultType::Photo | InlineResultType::Gif | InlineResultType::Video
        )
    }

    pub fn has_thumb(&self) -> bool {
        self.thumb_url.is_some() || self.photo_id.is_some() || self.document_id.is_some()
    }
}

impl Default for InlineBotResult {
    fn default() -> Self {
        Self::new("default_result", 0, InlineResultType::Article)
    }
}

impl Render for InlineBotResult {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_text = self.title.as_deref().unwrap_or("Untitled");
        let desc_text = self.description.as_deref().unwrap_or("");

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .child(title_text.to_string())
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x666666))
                    .child(desc_text.to_string())
            )
    }
}
