//! Authentic GPUI Document & Audio/Voice Message View.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_document.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_document.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_file.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_file.cpp`

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::MediaFileStatus;

/// Voice playback organic blob animation constants.
/// Upstream: `history_view_document.cpp:L53-L81`
pub const K_VOICE_BLOB_MINOR_SCALE: f32 = 0.88;
pub const K_VOICE_BLOB_MAJOR_SCALE: f32 = 0.85;
pub const K_VOICE_BLOB_ALPHA: f32 = 0.25;
pub const K_VOICE_BLOB_LEVEL_DURATION_MS: f32 = 265.0;
pub const K_VOICE_BLOB_IDLE_LEVEL: f32 = 0.45;

pub struct VoicePlaybackBlobConfig {
    pub minor_scale: f32,
    pub major_scale: f32,
    pub alpha: f32,
    pub level_duration_ms: f32,
    pub idle_level: f32,
    pub minor_segments: usize,
    pub major_segments: usize,
}

impl Default for VoicePlaybackBlobConfig {
    fn default() -> Self {
        Self {
            minor_scale: 0.88, // kVoiceBlobMinorScale
            major_scale: 0.85, // kVoiceBlobMajorScale
            alpha: 0.25,       // kVoiceBlobAlpha
            level_duration_ms: 265.0, // 100.0 + 500.0 * 0.33
            idle_level: 0.45,  // kVoiceBlobIdleLevel
            minor_segments: 6,
            major_segments: 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentMediaType {
    GenericFile,
    AudioMusic,
    VoiceMessage,
    RoundVideo,
}

#[derive(Debug, Clone)]
pub struct DocumentMediaState {
    pub file_name: String,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub extension: String,
    pub file_size: u64,
    pub media_type: DocumentMediaType,
    pub status: MediaFileStatus,
    pub is_outgoing: bool,
    pub ttl_seconds: Option<u32>,
    pub duration_seconds: Option<u32>,
}

impl DocumentMediaState {
    pub fn new(file_name: impl Into<String>, file_size: u64) -> Self {
        let name = file_name.into();
        let ext = name.split('.').last().unwrap_or("").to_lowercase();
        Self {
            file_name: name,
            performer: None,
            title: None,
            extension: ext,
            file_size,
            media_type: DocumentMediaType::GenericFile,
            status: MediaFileStatus::NotDownloaded,
            is_outgoing: false,
            ttl_seconds: None,
            duration_seconds: None,
        }
    }

    pub fn formatted_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;
        if self.file_size >= GB {
            format!("{:.2} GB", self.file_size as f64 / GB as f64)
        } else if self.file_size >= MB {
            format!("{:.1} MB", self.file_size as f64 / MB as f64)
        } else if self.file_size >= KB {
            format!("{:.0} KB", self.file_size as f64 / KB as f64)
        } else {
            format!("{} B", self.file_size)
        }
    }

    pub fn status_text(&self) -> String {
        match &self.status {
            MediaFileStatus::NotDownloaded => self.formatted_size(),
            MediaFileStatus::Downloading { loaded_bytes, total_bytes } => {
                let percent = if *total_bytes > 0 {
                    (*loaded_bytes as f64 / *total_bytes as f64) * 100.0
                } else {
                    0.0
                };
                format!("{:.0}% of {}", percent, self.formatted_size())
            }
            MediaFileStatus::Downloaded => self.formatted_size(),
            MediaFileStatus::Failed => "Download failed".to_string(),
            MediaFileStatus::Played { seconds, duration } => {
                let s = *seconds;
                let d = *duration;
                format!("{:02}:{:02} / {:02}:{:02}", s / 60, s % 60, d / 60, d % 60)
            }
        }
    }
}

pub struct DocumentMediaView;

impl DocumentMediaView {
    pub fn render(state: &DocumentMediaState) -> impl IntoElement {
        let bg_color = if state.is_outgoing {
            rgb(0x2B5278)
        } else {
            rgb(0x182533)
        };
        let icon_bg = if state.is_outgoing {
            rgb(0x3B6FA0)
        } else {
            rgb(0x283848)
        };
        let accent_color = rgb(0x5288C1);

        let icon_char = match state.media_type {
            DocumentMediaType::GenericFile => "📄",
            DocumentMediaType::AudioMusic => "🎵",
            DocumentMediaType::VoiceMessage => "🎤",
            DocumentMediaType::RoundVideo => "📹",
        };

        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .p(px(8.0))
            .min_w(px(240.0))
            .max_w(px(360.0))
            .rounded(px(12.0))
            .bg(bg_color)
            // Left Action / Download Circular Button
            .child(
                div()
                    .relative()
                    .w(px(44.0))
                    .h(px(44.0))
                    .rounded_full()
                    .bg(icon_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(20.0))
                    .child(icon_char)
                    // Download progress ring overlay if downloading
                    .when(
                        matches!(state.status, MediaFileStatus::Downloading { .. }),
                        |el| {
                            el.border_2().border_color(accent_color)
                        },
                    ),
            )
            // Center File Info & Description
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .truncate()
                            .child(if let (Some(artist), Some(track)) = (&state.performer, &state.title) {
                                format!("{} - {}", artist, track)
                            } else {
                                state.file_name.clone()
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x7D9BB7))
                            .child(state.status_text()),
                    ),
            )
            // Right-aligned TTL self-destruct badge if configured
            .when_some(state.ttl_seconds, |el, ttl| {
                el.child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(6.0))
                        .bg(rgba(0xE0554588))
                        .text_size(px(10.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(format!("⏱ {}s", ttl)),
                )
            })
    }
}
