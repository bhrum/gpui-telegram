//! Authentic GPUI History View Photo Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_photo.h:L1-L183`
//! - `Telegram/SourceFiles/history/view/media/history_view_photo.cpp:L1-L1330`
//!
//! Constants & Bounds:
//! - `kStoryWidth = 720`, `kStoryHeight = 1280` (9:16 aspect ratio) (`history_view_photo.cpp:L53-L54`)
//! - Photo thumbnail aspect bounding: min size 100px, max size 420px (`history_view_photo.cpp:L343-L352`)

use crate::views::stage::media::{PhotoAspectRatio, PhotoMediaState, STORY_HEIGHT, STORY_WIDTH};

pub const K_STORY_WIDTH: u32 = 720;
pub const K_STORY_HEIGHT: u32 = 1280;

/// Visual presentation component for photo attachments in the history transcript.
/// Upstream: `HistoryView::Photo` (`history_view_photo.h:L30-L160`)
#[derive(Debug, Clone)]
pub struct HistoryViewPhotoViewMedia {
    pub state: PhotoMediaState,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl HistoryViewPhotoViewMedia {
    pub fn new(photo_id: u64, width: u32, height: u32) -> Self {
        Self {
            state: PhotoMediaState::new(photo_id, width, height),
            is_selected: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn new_story(photo_id: u64) -> Self {
        Self::new(photo_id, STORY_WIDTH, STORY_HEIGHT)
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.state.caption = Some(caption.into());
        self
    }

    pub fn with_paid_stars(mut self, stars: u64) -> Self {
        self.state.paid_stars = Some(stars);
        self.state.is_spoiler = true;
        self
    }

    pub fn reveal_spoiler(&mut self) {
        self.state.is_revealed = true;
    }
}
