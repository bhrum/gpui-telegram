//! Authentic GPUI History View Video Status Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_video_status.h:L1-L47`
//! - `Telegram/SourceFiles/history/view/media/history_view_video_status.cpp:L1-L173`
//!
//! Corner Download & Status Overlay Layout (`history_view_video_status.cpp:L20-L59`):
//! - `VideoCornerDownloadRect`: computed status pill rectangle
//! - Streaming video download progress and sound mute state

use crate::views::stage::media::VideoCornerStatusInfo;

/// Overlay status and download controller for video thumbnails.
/// Upstream: `HistoryView::VideoCornerStatus` (`history_view_video_status.h:L21-L35`)
#[derive(Debug, Clone)]
pub struct HistoryViewVideoStatusViewMedia {
    pub status: VideoCornerStatusInfo,
    pub is_loading: bool,
    pub outer_width: i32,
}

impl Default for HistoryViewVideoStatusViewMedia {
    fn default() -> Self {
        Self {
            status: VideoCornerStatusInfo::default(),
            is_loading: false,
            outer_width: 320,
        }
    }
}

impl HistoryViewVideoStatusViewMedia {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            status: VideoCornerStatusInfo {
                text: text.into(),
                download_size: None,
                is_downloading: false,
                is_muted: true,
            },
            is_loading: false,
            outer_width: 320,
        }
    }

    pub fn set_downloading(&mut self, is_downloading: bool, size: impl Into<String>) {
        self.status.is_downloading = is_downloading;
        self.status.download_size = Some(size.into());
    }

    pub fn set_muted(&mut self, is_muted: bool) {
        self.status.is_muted = is_muted;
    }
}
