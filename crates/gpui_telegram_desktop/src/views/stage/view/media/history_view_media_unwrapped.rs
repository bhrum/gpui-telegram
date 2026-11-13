//! Authentic GPUI History View Media Unwrapped Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.h:L1-L184`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.cpp:L1-L786`
//!
//! Unwrapped Invariants (`history_view_media_unwrapped.h:L87-L98`):
//! - `needsBubble() = false`: Media rendered directly on background without message bubble container
//! - `unwrapped() = true`: Standalone geometry
//! - `drawsOwnEphemeralBadge() = true`: Direct ephemeral fire/countdown overlay
//! - `customInfoLayout() = true`: Time and status info rendered outside standard bubble tail

/// Media component for unwrapped visual items (stickers, custom emoji, dice).
/// Upstream: `HistoryView::UnwrappedMedia` (`history_view_media_unwrapped.h:L24-L140`)
#[derive(Debug, Clone)]
pub struct HistoryViewMediaUnwrappedViewMedia {
    pub width: f32,
    pub height: f32,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub always_show_out_timestamp: bool,
}

impl Default for HistoryViewMediaUnwrappedViewMedia {
    fn default() -> Self {
        Self {
            width: 180.0,
            height: 180.0,
            is_selected: false,
            is_hovered: false,
            always_show_out_timestamp: false,
        }
    }
}

impl HistoryViewMediaUnwrappedViewMedia {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            is_selected: false,
            is_hovered: false,
            always_show_out_timestamp: false,
        }
    }

    pub fn needs_bubble(&self) -> bool {
        false
    }

    pub fn is_unwrapped(&self) -> bool {
        true
    }

    pub fn draws_own_ephemeral_badge(&self) -> bool {
        true
    }
}
