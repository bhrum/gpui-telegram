//! Authentic GPUI Implementation: Song & Media Document Name Formatter
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Source: `Telegram/SourceFiles/ui/text/format_song_document_name.cpp:L22-L88`
//! Upstream Header: `Telegram/SourceFiles/ui/text/format_song_document_name.h:L16-L25`

use gpui::*;
use super::format_song_name::SongNameFormatter;

/// Document media type classification.
/// Citing `Telegram/SourceFiles/ui/text/format_song_document_name.cpp:L32-L45`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentMediaType {
    AudioFile,
    VideoFile,
    VoiceMessage,
    VideoMessage,
    Sticker,
}

/// Upstream `FormatDownloadsName` representation.
/// Citing `Telegram/SourceFiles/ui/text/format_song_document_name.cpp:L31-L46`
pub fn format_downloads_name(
    media_type: DocumentMediaType,
    sticker_alt: Option<&str>,
    song_formatter: Option<&SongNameFormatter>,
) -> (String, bool) {
    match media_type {
        DocumentMediaType::VideoFile => ("Video".to_string(), true),
        DocumentMediaType::VoiceMessage => ("Voice message".to_string(), true),
        DocumentMediaType::VideoMessage => ("Video message".to_string(), true),
        DocumentMediaType::Sticker => {
            if let Some(alt) = sticker_alt {
                if !alt.is_empty() {
                    return (format!("Sticker {}", alt), true);
                }
            }
            ("Sticker".to_string(), true)
        }
        DocumentMediaType::AudioFile => {
            if let Some(formatter) = song_formatter {
                (formatter.to_string_formatted(), false)
            } else {
                ("Audio track".to_string(), true)
            }
        }
    }
}

/// Upstream `FormatVoiceName` relative date resolver and song name constructor.
/// Citing `Telegram/SourceFiles/ui/text/format_song_document_name.cpp:L48-L88`
pub fn format_voice_name(
    sender_name: &str,
    is_out: bool,
    is_post: bool,
    time_str: &str,
    days_ago: i64,
    date_str: &str,
) -> SongNameFormatter {
    let name = if is_out && !is_post {
        "You"
    } else if sender_name.is_empty() {
        "Unknown"
    } else {
        sender_name
    };

    let date_label = if days_ago == 0 {
        format!("Today at {}", time_str)
    } else if days_ago == 1 {
        format!("Yesterday at {}", time_str)
    } else {
        format!("{} at {}", date_str, time_str)
    };

    SongNameFormatter::new("", &date_label, name).with_no_dash(true)
}

/// Authentic GPUI Render View for document file headers.
/// Registered in Ledger as `FormatSongDocumentNameText`.
#[derive(Debug, Clone)]
pub struct FormatSongDocumentNameText {
    pub display_name: String,
    pub is_bold: bool,
    pub media_type: DocumentMediaType,
    pub size_text: String,
}

impl FormatSongDocumentNameText {
    pub fn new(display_name: impl Into<String>, is_bold: bool, media_type: DocumentMediaType) -> Self {
        Self {
            display_name: display_name.into(),
            is_bold,
            media_type,
            size_text: String::new(),
        }
    }

    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.size_text = size.into();
        self
    }
}

impl Default for FormatSongDocumentNameText {
    fn default() -> Self {
        Self::new("Untitled Document", false, DocumentMediaType::AudioFile)
    }
}

impl Render for FormatSongDocumentNameText {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let type_badge = match self.media_type {
            DocumentMediaType::AudioFile => "🎵",
            DocumentMediaType::VideoFile => "🎬",
            DocumentMediaType::VoiceMessage => "🎙️",
            DocumentMediaType::VideoMessage => "📹",
            DocumentMediaType::Sticker => "🎨",
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(16.0))
                    .child(type_badge)
            )
            .child({
                let mut col = div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(if self.is_bold {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(rgb(0x111111))
                            .child(self.display_name.clone())
                    );
                if !self.size_text.is_empty() {
                    col = col.child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(self.size_text.clone())
                    );
                }
                col
            })
    }
}
