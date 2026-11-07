//! Authentic GPUI History View Media Spoiler Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h:L1-L40`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp:L1-L10`
//!
//! Spoiler Animation & Tag Structures:
//! - `MediaSpoiler`: reveal animation, revealed boolean (`history_view_media_spoiler.h:L19-L27`)
//! - `MediaSpoilerTag`: bitfields `price: 63, sensitive: 1`, colors (`history_view_media_spoiler.h:L29-L37`)

use crate::views::stage::media::{MediaSpoilerState, MediaPaidTag, MediaSensitiveTag};

/// State and animation holder for spoiler effects and paid/sensitive content tags.
/// Upstream: `HistoryView::MediaSpoiler` & `MediaSpoilerTag` (`history_view_media_spoiler.h:L19-L37`)
#[derive(Debug, Clone, Default)]
pub struct HistoryViewMediaSpoilerViewMedia {
    pub spoiler_state: MediaSpoilerState,
    pub paid_tag: Option<MediaPaidTag>,
    pub sensitive_tag: Option<MediaSensitiveTag>,
}

impl HistoryViewMediaSpoilerViewMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_paid_stars(mut self, stars: u64) -> Self {
        self.paid_tag = Some(MediaPaidTag { price: stars });
        self
    }

    pub fn with_sensitive(mut self, is_sensitive: bool) -> Self {
        self.sensitive_tag = Some(MediaSensitiveTag { is_sensitive });
        self
    }

    pub fn reveal(&mut self) {
        self.spoiler_state.is_revealed = true;
    }

    pub fn is_revealed(&self) -> bool {
        self.spoiler_state.is_revealed
    }
}
