//! Video avatar userpic animated playback component.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_video_userpic.cpp:L1-175`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsVideoUserpicUi {
    pub peer_id: u64,
    pub is_playing: bool,
    pub frame_index: u32,
}

impl DialogsVideoUserpicUi {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            is_playing: false,
            frame_index: 0,
        }
    }
}
