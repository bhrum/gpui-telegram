//! Asynchronous Profile Preloader for Short Info Dialog
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/prepare_short_info_box.h:L1-L68`
//! - `Telegram/SourceFiles/boxes/peers/prepare_short_info_box.cpp:L1-L571`
//! - `prepare_short_info_box.cpp:L40-L150` (Async full info resolution)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `PrepareShortInfoBoxPeers`.
#[derive(Debug, Clone)]
pub struct PrepareShortInfoBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub is_loading: bool,
    pub load_error: Option<String>,
}

impl Default for PrepareShortInfoBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl PrepareShortInfoBoxPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(340.0),
            height: px(280.0),
            peer_id,
            is_loading: true,
            load_error: None,
        }
    }

    pub fn complete_load(&mut self) {
        self.is_loading = false;
        self.load_error = None;
    }

    pub fn fail_load(&mut self, err: String) {
        self.is_loading = false;
        self.load_error = Some(err);
    }
}
