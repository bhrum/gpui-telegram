//! Authentic GPUI History View Document Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_document.h:L1-L201`
//! - `Telegram/SourceFiles/history/view/media/history_view_document.cpp:L1-L2124`
//!
//! Constants & Logic:
//! - `kAudioVoiceMsgUpdateView = 100ms` (`history_view_document.cpp:L51`)
//! - `kVoiceBlobMinorScale = 0.88`, `kVoiceBlobMajorScale = 0.85` (`history_view_document.cpp:L56-L57`)
//! - `kVoiceBlobLevelDuration = 265.0ms`, `kVoiceBlobIdleLevel = 0.45`, `kVoiceBlobAlpha = 0.25` (`history_view_document.cpp:L53-L58`)
//! - `VoicePlaybackBlobs`: 6-segment minor blob and 8-segment major blob (`history_view_document.cpp:L60-L81`)

use crate::views::stage::media::{DocumentMediaState, DocumentMediaType, MediaFileStatus};

pub const K_VOICE_BLOB_MINOR_SCALE: f32 = 0.88;
pub const K_VOICE_BLOB_MAJOR_SCALE: f32 = 0.85;
pub const K_VOICE_BLOB_LEVEL_DURATION_MS: f32 = 265.0;
pub const K_VOICE_BLOB_IDLE_LEVEL: f32 = 0.45;
pub const K_VOICE_BLOB_ALPHA: f32 = 0.25;

/// View component for document attachments and voice/audio bubbles in the history transcript.
/// Upstream: `HistoryView::Document` (`history_view_document.h:L28-L200`)
#[derive(Debug, Clone)]
pub struct HistoryViewDocumentViewMedia {
    pub state: DocumentMediaState,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl HistoryViewDocumentViewMedia {
    pub fn new(file_name: impl Into<String>, file_size: u64) -> Self {
        Self {
            state: DocumentMediaState::new(file_name, file_size),
            is_selected: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn with_audio_metadata(mut self, performer: impl Into<String>, title: impl Into<String>, duration: u32) -> Self {
        self.state.performer = Some(performer.into());
        self.state.title = Some(title.into());
        self.state.media_type = DocumentMediaType::AudioMusic;
        self.state.duration_seconds = Some(duration);
        self
    }

    pub fn with_voice(mut self, duration: u32, is_round: bool) -> Self {
        self.state.media_type = if is_round {
            DocumentMediaType::RoundVideo
        } else {
            DocumentMediaType::VoiceMessage
        };
        self.state.duration_seconds = Some(duration);
        self
    }

    pub fn set_status(&mut self, status: MediaFileStatus) {
        self.state.status = status;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
}
