//! Info Profile Tab Content interface
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_content.h:L1-L85`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfileTabKind {
    Media,
    Files,
    Links,
    Audio,
    Voice,
    Gifs,
    Stories,
    Members,
    SimilarChannels,
    Polls,
    SavedSublists,
    Gifts,
}

/// Abstract state holder for content rendered within a profile tab viewport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoProfileTabContentProfileTabs {
    pub tab_kind: ProfileTabKind,
    pub peer_id: i64,
    pub scroll_offset: f32,
    pub is_active: bool,
}

impl InfoProfileTabContentProfileTabs {
    pub fn new(tab_kind: ProfileTabKind, peer_id: i64) -> Self {
        Self {
            tab_kind,
            peer_id,
            scroll_offset: 0.0,
            is_active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_scroll_offset(&mut self, offset: f32) {
        self.scroll_offset = offset;
    }
}

impl Default for InfoProfileTabContentProfileTabs {
    fn default() -> Self {
        Self::new(ProfileTabKind::Media, 0)
    }
}
