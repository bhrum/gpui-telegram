//! Authentic GPUI History View File Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_file.h:L1-L119`
//! - `Telegram/SourceFiles/history/view/media/history_view_file.cpp:L1-L181`
//!
//! Constants & Status Size Encoding (`history_view_file.h:L58-L67`):
//! - `>= 0`: Download / upload in progress (`loaded_bytes`)
//! - `< 0`: Played audio string (`-(seconds + 1)`)
//! - `0xFFFFFFF0LL` (-16): Not yet downloaded file
//! - `0xFFFFFFF1LL` (-15): Already downloaded file
//! - `0xFFFFFFF2LL` (-14): Failed to download or upload file
//! - `duration = -1`: No duration, `duration = -2`: GIF duration

use crate::views::stage::media::MediaFileStatus;

pub const STATUS_CODE_NOT_DOWNLOADED: i64 = -16;
pub const STATUS_CODE_ALREADY_DOWNLOADED: i64 = -15;
pub const STATUS_CODE_FAILED: i64 = -14;

/// Base file state handler tracking download progress and local caching state.
/// Upstream: `HistoryView::File` (`history_view_file.h:L18-L100`)
#[derive(Debug, Clone)]
pub struct HistoryViewFileViewMedia {
    pub status_size: i64,
    pub total_size: u64,
    pub duration: i32,
    pub real_duration: i32,
    pub status_text: String,
}

impl HistoryViewFileViewMedia {
    pub fn new(total_size: u64) -> Self {
        Self {
            status_size: STATUS_CODE_NOT_DOWNLOADED,
            total_size,
            duration: -1,
            real_duration: 0,
            status_text: String::new(),
        }
    }

    /// Set status size with upstream logic from `history_view_file.cpp:L67-L95`
    pub fn set_status_size(&mut self, new_size: i64, full_size: i64, duration: i32, real_duration: i32) {
        self.status_size = new_size;
        self.total_size = full_size.max(0) as u64;
        self.duration = duration;
        self.real_duration = real_duration;

        self.status_text = if new_size == STATUS_CODE_NOT_DOWNLOADED {
            format!("{} B", self.total_size)
        } else if new_size == STATUS_CODE_ALREADY_DOWNLOADED {
            format!("Downloaded ({} B)", self.total_size)
        } else if new_size == STATUS_CODE_FAILED {
            "Failed".to_string()
        } else if new_size >= 0 {
            format!("{}/{} B", new_size, self.total_size)
        } else {
            let seconds = ((-new_size) - 1).max(0);
            format!("{:02}:{:02}", seconds / 60, seconds % 60)
        };
    }

    pub fn to_file_status(&self) -> MediaFileStatus {
        MediaFileStatus::from_status_size(self.status_size, self.total_size, self.duration.max(0) as u32)
    }
}
