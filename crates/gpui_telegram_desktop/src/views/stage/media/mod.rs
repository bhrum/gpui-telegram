//! History Media Views (Slice 5: Media Basic & Slice 6: Media Rich).
//! Subsystem: history (`SPEC-06-HISTORY`)

// --- Slice 5: Media Basic ---
pub mod document;
pub mod photo;
pub mod gif;
pub mod sticker;
pub mod grouped;
pub mod spoiler;

pub use document::*;
pub use photo::*;
pub use gif::*;
pub use sticker::*;
pub use grouped::*;
pub use spoiler::*;

// --- Slice 6: Media Rich ---
pub mod poll;
pub mod web_page;
pub mod invoice;
pub mod transfer;
pub mod todo;
pub mod gift;
pub mod location;

pub use poll::*;
pub use web_page::*;
pub use invoice::*;
pub use transfer::*;
pub use todo::*;
pub use gift::*;
pub use location::*;

/// Position of media within a containing message bubble.
/// Upstream: `HistoryView::MediaInBubbleState` (`history_view_media.h:L57-L62`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaInBubbleState {
    #[default]
    None,
    Top,
    Middle,
    Bottom,
}

/// Border rounding for media corners within a message bubble.
/// Upstream: `Ui::BubbleRounding` (`ui/chat/message_bubble.h`)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaBubbleRounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Default for MediaBubbleRounding {
    fn default() -> Self {
        Self {
            top_left: 12.0,
            top_right: 12.0,
            bottom_right: 12.0,
            bottom_left: 12.0,
        }
    }
}

impl MediaBubbleRounding {
    pub const NONE: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub const FULL: Self = Self {
        top_left: 16.0,
        top_right: 16.0,
        bottom_right: 16.0,
        bottom_left: 16.0,
    };
}

/// Bitmask describing which outer sides of a grouped layout an item touches.
/// Upstream: `RectPart` / `RectParts` (`ui/rect_part.h`, `history_view_media_grouped.cpp:L52-L57`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RectParts {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl RectParts {
    pub const ALL: Self = Self {
        top: true,
        bottom: true,
        left: true,
        right: true,
    };

    pub const NONE: Self = Self {
        top: false,
        bottom: false,
        left: false,
        right: false,
    };
}

/// Media download and playback progress status.
/// Upstream: `HistoryView::File::_statusSize` (`history_view_file.h:L58-L67`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaFileStatus {
    NotDownloaded,
    Downloading { loaded_bytes: u64, total_bytes: u64 },
    Downloaded,
    Failed,
    Played { seconds: u32, duration: u32 },
}

impl MediaFileStatus {
    pub const STATUS_NOT_DOWNLOADED: i64 = -16; // 0xFFFFFFF0LL
    pub const STATUS_ALREADY_DOWNLOADED: i64 = -15; // 0xFFFFFFF1LL
    pub const STATUS_FAILED: i64 = -14; // 0xFFFFFFF2LL

    pub fn to_status_size(&self) -> i64 {
        match self {
            Self::NotDownloaded => Self::STATUS_NOT_DOWNLOADED,
            Self::Downloaded => Self::STATUS_ALREADY_DOWNLOADED,
            Self::Failed => Self::STATUS_FAILED,
            Self::Downloading { loaded_bytes, .. } => *loaded_bytes as i64,
            Self::Played { seconds, .. } => -((*seconds as i64) + 1),
        }
    }

    pub fn from_status_size(code: i64, total_bytes: u64, duration: u32) -> Self {
        if code == Self::STATUS_NOT_DOWNLOADED {
            Self::NotDownloaded
        } else if code == Self::STATUS_ALREADY_DOWNLOADED {
            Self::Downloaded
        } else if code == Self::STATUS_FAILED {
            Self::Failed
        } else if code >= 0 {
            Self::Downloading {
                loaded_bytes: code as u64,
                total_bytes,
            }
        } else {
            let seconds = ((-code) - 1).max(0) as u32;
            Self::Played { seconds, duration }
        }
    }
}
