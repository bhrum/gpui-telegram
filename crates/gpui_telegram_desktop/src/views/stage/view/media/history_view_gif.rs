//! Authentic GPUI History View GIF Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_gif.h:L1-L305`
//! - `Telegram/SourceFiles/history/view/media/history_view_gif.cpp:L1-L2751`
//!
//! Constants & Geometry Limits:
//! - `kMaxGifForwardedBarLines = 4` (`history_view_gif.cpp:L75`)
//! - `kUseNonBlurredThreshold = 240` (`history_view_gif.cpp:L76`)
//! - `kMaxInlineArea = 1920 * 1080` (2,073,600 px) (`history_view_gif.cpp:L77`)
//! - `kMaxInstantViewInlineArea = 1920 * 1920` (3,686,400 px) (`history_view_gif.cpp:L78`)
//! - `kSeekPreviewInterval = 100ms` (`history_view_gif.cpp:L79`)

use crate::views::stage::media::{GifMediaState, VideoCornerStatusInfo};

pub const K_MAX_INLINE_AREA: u64 = 1920 * 1080;
pub const K_MAX_INSTANT_VIEW_INLINE_AREA: u64 = 1920 * 1920;
pub const K_SEEK_PREVIEW_INTERVAL_MS: u64 = 100;
pub const K_MAX_GIF_FORWARDED_BAR_LINES: usize = 4;
pub const K_USE_NON_BLURRED_THRESHOLD: u32 = 240;

/// View component for looping animations and streaming video messages in history.
/// Upstream: `HistoryView::Gif` (`history_view_gif.h:L57-L290`)
#[derive(Debug, Clone)]
pub struct HistoryViewGifViewMedia {
    pub state: GifMediaState,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub is_muted: bool,
}

impl HistoryViewGifViewMedia {
    pub fn new(document_id: u64, width: u32, height: u32, duration: u32) -> Self {
        Self {
            state: GifMediaState::new(document_id, width, height, duration),
            is_selected: false,
            is_hovered: false,
            is_muted: true,
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.state.caption = Some(caption.into());
        self
    }

    pub fn set_spoiler(&mut self, is_spoiler: bool) {
        self.state.is_spoiler = is_spoiler;
    }

    pub fn reveal_spoiler(&mut self) {
        self.state.is_revealed = true;
    }

    pub fn set_playback_progress(&mut self, progress: f32) {
        self.state.playback_progress = progress.clamp(0.0, 1.0);
    }

    pub fn toggle_sound(&mut self) {
        self.is_muted = !self.is_muted;
        self.state.corner_status.is_muted = self.is_muted;
    }
}
