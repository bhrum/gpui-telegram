//! Info Media Provider
//! Upstream: `Telegram/SourceFiles/info/media/info_media_provider.cpp:L1-L669`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_provider.h:L1-L133`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};
use super::info_media_common::SharedMediaType;

/// Query provider and pagination controller for fetching shared media from local storage and MTProto.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoMediaProviderMedia {
    pub peer_id: i64,
    pub topic_root_id: i32,
    pub media_type: SharedMediaType,
    pub has_more: bool,
    pub is_loading: bool,
    pub last_max_id: i64,
    pub loaded_count: usize,
}

impl InfoMediaProviderMedia {
    pub const MINIMAL_IDS_LIMIT: usize = 50;
    pub const PRELOADED_SCREENS_COUNT: usize = 3;

    pub fn new(peer_id: i64, media_type: SharedMediaType) -> Self {
        Self {
            peer_id,
            topic_root_id: 0,
            media_type,
            has_more: true,
            is_loading: false,
            last_max_id: 0,
            loaded_count: 0,
        }
    }

    pub fn for_topic(peer_id: i64, topic_root_id: i32, media_type: SharedMediaType) -> Self {
        Self {
            peer_id,
            topic_root_id,
            media_type,
            has_more: true,
            is_loading: false,
            last_max_id: 0,
            loaded_count: 0,
        }
    }

    pub fn start_loading(&mut self) -> bool {
        if self.is_loading || !self.has_more {
            return false;
        }
        self.is_loading = true;
        true
    }

    pub fn finish_loading(&mut self, new_items_count: usize, last_id: i64, has_more: bool) {
        self.is_loading = false;
        self.loaded_count += new_items_count;
        self.last_max_id = last_id;
        self.has_more = has_more;
    }

    pub fn reset(&mut self) {
        self.has_more = true;
        self.is_loading = false;
        self.last_max_id = 0;
        self.loaded_count = 0;
    }
}

impl Default for InfoMediaProviderMedia {
    fn default() -> Self {
        Self::new(0, SharedMediaType::PhotoVideo)
    }
}
