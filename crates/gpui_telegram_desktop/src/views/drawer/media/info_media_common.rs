//! Info Media Common types and shared definitions
//! Upstream: `Telegram/SourceFiles/info/media/info_media_common.cpp:L1-L99`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_common.h:L1-L200`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// Upstream: `Storage::SharedMediaType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SharedMediaType {
    PhotoVideo = 0,
    Files = 1,
    SharedLinks = 2,
    AudioMusic = 3,
    VoiceMessages = 4,
    RoundVideos = 5,
    Gifs = 6,
}

impl SharedMediaType {
    pub fn all() -> [SharedMediaType; 6] {
        [
            SharedMediaType::PhotoVideo,
            SharedMediaType::Files,
            SharedMediaType::SharedLinks,
            SharedMediaType::AudioMusic,
            SharedMediaType::VoiceMessages,
            SharedMediaType::Gifs,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            SharedMediaType::PhotoVideo => "Media",
            SharedMediaType::Files => "Files",
            SharedMediaType::SharedLinks => "Links",
            SharedMediaType::AudioMusic => "Music",
            SharedMediaType::VoiceMessages => "Voice",
            SharedMediaType::RoundVideos => "Video Messages",
            SharedMediaType::Gifs => "GIFs",
        }
    }
}

/// Upstream: `Info::Media::ListItemSelectionData`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListItemSelectionData {
    pub message_id: i64,
    pub is_selected: bool,
    pub item_index: usize,
}

/// Upstream: `Info::Media::RowsScrollCache`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowsScrollCache {
    pub scroll_top: f32,
    pub visible_start_index: usize,
    pub visible_end_index: usize,
}

/// Core shared media configuration and state holder
/// Upstream: `Info::Media::Common`
#[derive(Debug, Clone)]
pub struct InfoMediaCommonMedia {
    pub preload_screens_count: f32,
    pub active_media_type: SharedMediaType,
    pub scroll_cache: RowsScrollCache,
    pub is_selection_mode: bool,
}

impl InfoMediaCommonMedia {
    pub const PRELOAD_IF_LESS_THAN_SCREENS: f32 = 2.0;

    pub fn new() -> Self {
        Self {
            preload_screens_count: Self::PRELOAD_IF_LESS_THAN_SCREENS,
            active_media_type: SharedMediaType::PhotoVideo,
            scroll_cache: RowsScrollCache {
                scroll_top: 0.0,
                visible_start_index: 0,
                visible_end_index: 0,
            },
            is_selection_mode: false,
        }
    }

    pub fn set_media_type(&mut self, media_type: SharedMediaType) {
        self.active_media_type = media_type;
        self.scroll_cache.scroll_top = 0.0;
    }

    pub fn toggle_selection_mode(&mut self) {
        self.is_selection_mode = !self.is_selection_mode;
    }

    pub fn should_preload(&self, current_screen: f32, total_screens: f32) -> bool {
        (total_screens - current_screen) < self.preload_screens_count
    }
}

impl Default for InfoMediaCommonMedia {
    fn default() -> Self {
        Self::new()
    }
}
