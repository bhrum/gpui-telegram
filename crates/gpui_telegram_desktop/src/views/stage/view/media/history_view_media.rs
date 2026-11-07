//! Authentic GPUI History View Media Base Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_media.h:L1-L473`
//! - `Telegram/SourceFiles/history/view/media/history_view_media.cpp:L1-L680`
//!
//! Constants & Timestamp Extraction:
//! - `kMaxYouTubeTimestampDuration = 360,000` (100 hours in seconds) (`history_view_media.cpp:L93`)
//! - `TimeFromMatch`: `value1 * 3600 + value2 * 60 + value3` (`history_view_media.cpp:L44-L61`)
//! - `TimestampLinkBase`: format `"internal:media_timestamp?base=doc{id}_{peer}_{msg}&t="` (`history_view_media.cpp:L74-L80`)

use crate::views::stage::media::{MediaBubbleRounding, MediaInBubbleState};

pub const K_MAX_YOUTUBE_TIMESTAMP_DURATION_SECS: i32 = 100 * 60 * 60; // 360,000s

/// Parse timestamp from (hours, minutes, seconds) parts.
/// Upstream: `TimeFromMatch` (`history_view_media.cpp:L44-L61`)
pub fn parse_timestamp_from_match(hours: Option<&str>, minutes: &str, seconds: &str) -> Option<i32> {
    let h = match hours {
        Some(s) if !s.is_empty() => s.parse::<i32>().ok()?,
        _ => 0,
    };
    let m = minutes.parse::<i32>().ok()?;
    let s = seconds.parse::<i32>().ok()?;

    if s < 60 && (hours.is_none() || m < 60) {
        Some(h * 3600 + m * 60 + s)
    } else {
        None
    }
}

/// Construct timestamp link base for media document context.
/// Upstream: `TimestampLinkBase` (`history_view_media.cpp:L74-L80`)
pub fn build_timestamp_link_base(doc_id: u64, peer_id: u64, msg_id: i32) -> String {
    format!("internal:media_timestamp?base=doc{}_{}_{}&t=", doc_id, peer_id, msg_id)
}

/// Base visual media view holder in the history transcript.
/// Upstream: `HistoryView::Media` (`history_view_media.h:L90-L460`)
#[derive(Debug, Clone)]
pub struct HistoryViewMediaViewMedia {
    pub in_bubble_state: MediaInBubbleState,
    pub bubble_rounding: MediaBubbleRounding,
    pub width: f32,
    pub height: f32,
    pub is_displayed: bool,
    pub needs_bubble: bool,
}

impl Default for HistoryViewMediaViewMedia {
    fn default() -> Self {
        Self {
            in_bubble_state: MediaInBubbleState::None,
            bubble_rounding: MediaBubbleRounding::default(),
            width: 0.0,
            height: 0.0,
            is_displayed: true,
            needs_bubble: true,
        }
    }
}

impl HistoryViewMediaViewMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn is_bubble_top(&self) -> bool {
        matches!(self.in_bubble_state, MediaInBubbleState::Top | MediaInBubbleState::None)
    }

    pub fn is_bubble_bottom(&self) -> bool {
        matches!(self.in_bubble_state, MediaInBubbleState::Bottom | MediaInBubbleState::None)
    }
}
