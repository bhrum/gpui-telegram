//! Authentic GPUI Implementation: Song Name Formatter & Component
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Source: `Telegram/SourceFiles/ui/text/format_song_name.cpp:L14-L80`
//! Upstream Header: `Telegram/SourceFiles/ui/text/format_song_name.h:L12-L35`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Composed song title and performer pair.
/// Citing `Telegram/SourceFiles/ui/text/format_song_name.h:L14-L17`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ComposedName {
    pub title: String,
    pub performer: String,
}

/// Upstream `ComputeComposedName` implementation.
/// Citing `Telegram/SourceFiles/ui/text/format_song_name.cpp:L14-L37`
pub fn compute_composed_name(
    filename: &str,
    song_title: &str,
    song_performer: &str,
) -> ComposedName {
    let unknown = "Unknown Track";
    if song_title.is_empty() && song_performer.is_empty() {
        return ComposedName {
            title: if filename.is_empty() {
                unknown.to_string()
            } else {
                filename.to_string()
            },
            performer: String::new(),
        };
    }

    if song_performer.is_empty() {
        return ComposedName {
            title: song_title.to_string(),
            performer: String::new(),
        };
    }

    ComposedName {
        title: if song_title.is_empty() {
            unknown.to_string()
        } else {
            song_title.to_string()
        },
        performer: song_performer.to_string(),
    }
}

/// Formatter for audio track names with customizable dash separators and bold styling.
/// Citing `Telegram/SourceFiles/ui/text/format_song_name.cpp:L41-L80`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SongNameFormatter {
    composed: ComposedName,
    no_dash: bool,
}

impl SongNameFormatter {
    pub fn new(filename: &str, song_title: &str, song_performer: &str) -> Self {
        Self {
            composed: compute_composed_name(filename, song_title, song_performer),
            no_dash: false,
        }
    }

    pub fn composed_name(&self) -> &ComposedName {
        &self.composed
    }

    pub fn set_no_dash(&mut self, no_dash: bool) {
        self.no_dash = no_dash;
    }

    pub fn with_no_dash(mut self, no_dash: bool) -> Self {
        self.no_dash = no_dash;
        self
    }

    /// Formats the song name string according to upstream dash rules.
    /// Citing `Telegram/SourceFiles/ui/text/format_song_name.cpp:L52-L60`
    pub fn to_string_formatted(&self) -> String {
        let title = &self.composed.title;
        let performer = &self.composed.performer;

        let dash = if title.is_empty() || performer.is_empty() {
            ""
        } else if self.no_dash {
            " "
        } else {
            " \u{2013} " // Unicode en-dash '\xe2\x80\x93'
        };

        format!("{}{}{}", performer, dash, title)
    }

    /// Generates plain text and entity span for semibold performer highlighting.
    /// Citing `Telegram/SourceFiles/ui/text/format_song_name.cpp:L66-L80`
    pub fn text_with_semibold(&self, bold_only_performer: bool) -> (String, Option<(usize, usize)>) {
        let text = self.to_string_formatted();
        let entity = if !bold_only_performer || !self.composed.performer.is_empty() {
            let len = if self.composed.performer.is_empty() {
                text.len()
            } else {
                self.composed.performer.len()
            };
            Some((0, len))
        } else {
            None
        };
        (text, entity)
    }
}

/// Authentic GPUI Render View for formatted song display.
/// Registered in Ledger as `FormatSongNameText`.
#[derive(Debug, Clone)]
pub struct FormatSongNameText {
    pub formatter: SongNameFormatter,
    pub is_playing: bool,
    pub accent_color: gpui::Rgba,
}

impl FormatSongNameText {
    pub fn new(filename: &str, song_title: &str, song_performer: &str) -> Self {
        Self {
            formatter: SongNameFormatter::new(filename, song_title, song_performer),
            is_playing: false,
            accent_color: rgb(0x2AABEE),
        }
    }

    pub fn playing(mut self, playing: bool) -> Self {
        self.is_playing = playing;
        self
    }
}

impl Default for FormatSongNameText {
    fn default() -> Self {
        Self::new("", "", "")
    }
}

impl Render for FormatSongNameText {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (formatted_text, entity_range) = self.formatter.text_with_semibold(false);
        let performer_len = entity_range.map(|(_, l)| l).unwrap_or(0);
        let performer_part = &formatted_text[..performer_len.min(formatted_text.len())];
        let rest_part = &formatted_text[performer_len.min(formatted_text.len())..];

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded_full()
                    .bg(if self.is_playing {
                        self.accent_color
                    } else {
                        rgb(0xEEF2F6)
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(11.0))
                    .text_color(if self.is_playing {
                        rgb(0xFFFFFF)
                    } else {
                        rgb(0x555555)
                    })
                    .child(if self.is_playing { "⏸" } else { "▶" })
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .text_size(px(13.0))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x111111))
                            .child(performer_part.to_string())
                    )
                    .child(
                        div()
                            .text_color(rgb(0x444444))
                            .child(rest_part.to_string())
                    )
            )
    }
}
