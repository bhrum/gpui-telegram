//! Authentic GPUI History View Video Message Seek Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.h:L1-L61`
//! - `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.cpp:L1-L193`
//!
//! Constants & Durations (`history_view_video_message_seek.cpp:L19-L26`):
//! - `kShowDuration = 220ms`
//! - `kHideDuration = 150ms`
//! - `kGrabDuration = 150ms`
//! - `kGhostDuration = 150ms`
//! - `kTrackOpacity = 0.2`
//! - `kShadowInnerStop = 0.7`
//! - `kGhostJump = 0.9`

pub const K_SHOW_DURATION_MS: u64 = 220;
pub const K_HIDE_DURATION_MS: u64 = 150;
pub const K_GRAB_DURATION_MS: u64 = 150;
pub const K_GHOST_DURATION_MS: u64 = 150;
pub const K_TRACK_OPACITY: f32 = 0.2;
pub const K_SHADOW_INNER_STOP: f32 = 0.7;
pub const K_GHOST_JUMP: f32 = 0.9;

/// Circular seeker and scrubber for round video messages.
/// Upstream: `HistoryView::VideoMessageSeek` (`history_view_video_message_seek.h:L20-L58`)
#[derive(Debug, Clone)]
pub struct HistoryViewVideoMessageSeekViewMedia {
    pub progress: f32,
    pub ghost_progress: f32,
    pub is_grabbed: bool,
    pub is_shown: bool,
}

impl Default for HistoryViewVideoMessageSeekViewMedia {
    fn default() -> Self {
        Self {
            progress: 0.0,
            ghost_progress: 0.0,
            is_grabbed: false,
            is_shown: false,
        }
    }
}

impl HistoryViewVideoMessageSeekViewMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn set_dragged_progress(&mut self, progress: f32) {
        let p = progress.clamp(0.0, 1.0);
        if (p - self.progress).abs() > K_GHOST_JUMP {
            self.ghost_progress = self.progress;
        }
        self.progress = p;
    }

    pub fn set_grabbed(&mut self, grabbed: bool) {
        self.is_grabbed = grabbed;
    }
}
