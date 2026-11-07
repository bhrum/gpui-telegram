//! Info Media Empty Widget
//! Upstream: `Telegram/SourceFiles/info/media/info_media_empty_widget.cpp:L1-L135`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_empty_widget.h:L1-L45`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use super::info_media_common::SharedMediaType;

/// Empty state widget displaying centered icon and contextual description
/// when no shared media items exist in the selected category.
#[derive(Debug, Clone)]
pub struct InfoMediaEmptyWidgetMedia {
    pub media_type: SharedMediaType,
    pub is_search_results: bool,
    pub custom_message: Option<String>,
}

impl InfoMediaEmptyWidgetMedia {
    pub fn new(media_type: SharedMediaType) -> Self {
        Self {
            media_type,
            is_search_results: false,
            custom_message: None,
        }
    }

    pub fn for_search(media_type: SharedMediaType) -> Self {
        Self {
            media_type,
            is_search_results: true,
            custom_message: None,
        }
    }

    pub fn primary_text(&self) -> &'static str {
        if self.is_search_results {
            return "No matching media found";
        }
        match self.media_type {
            SharedMediaType::PhotoVideo => "No photos or videos here yet",
            SharedMediaType::Files => "No files shared in this chat",
            SharedMediaType::SharedLinks => "No shared links found",
            SharedMediaType::AudioMusic => "No music tracks here",
            SharedMediaType::VoiceMessages => "No voice messages",
            SharedMediaType::RoundVideos => "No video messages",
            SharedMediaType::Gifs => "No GIFs shared yet",
        }
    }

    pub fn subtitle_text(&self) -> &'static str {
        if self.is_search_results {
            return "Try different keywords or filters";
        }
        "Media shared in this chat will be displayed here"
    }
}

impl Default for InfoMediaEmptyWidgetMedia {
    fn default() -> Self {
        Self::new(SharedMediaType::PhotoVideo)
    }
}

impl Render for InfoMediaEmptyWidgetMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.custom_message.clone().unwrap_or_else(|| self.primary_text().to_string());
        let subtitle = self.subtitle_text();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                div()
                    .w(px(64.0))
                    .h(px(64.0))
                    .rounded_full()
                    .bg(rgb(0x232e3c))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0x5288c1))
                    .text_xl()
                    .child("📁")
            )
            .child(
                div()
                    .mt(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(title)
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_sm()
                    .text_color(rgb(0x708499))
                    .child(subtitle)
            )
    }
}
