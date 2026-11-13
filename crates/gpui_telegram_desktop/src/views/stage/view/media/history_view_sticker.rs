//! Authentic GPUI History View Sticker Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker.h:L1-L158`
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp:L1-L693`
//!
//! Constants & Multipliers (`history_view_sticker.cpp:L44-L49`):
//! - `kMaxSizeFixed = 512`
//! - `kMaxEmojiSizeFixed = 256`
//! - `kPremiumMultiplier = 1.49` (1 + 0.245 * 2)
//! - `kEmojiMultiplier = 3`
//! - `kMessageEffectMultiplier = 2`

use crate::views::stage::media::{StickerFormat, StickerPlaybackState, MAX_SIZE_FIXED, PREMIUM_MULTIPLIER};

pub const K_MAX_SIZE_FIXED: u32 = 512;
pub const K_MAX_EMOJI_SIZE_FIXED: u32 = 256;
pub const K_PREMIUM_MULTIPLIER: f32 = 1.49;
pub const K_EMOJI_MULTIPLIER: u32 = 3;
pub const K_MESSAGE_EFFECT_MULTIPLIER: u32 = 2;

/// View component for individual animated or static stickers in history transcript.
/// Upstream: `HistoryView::Sticker` (`history_view_sticker.h:L33-L150`)
#[derive(Debug, Clone)]
pub struct HistoryViewStickerViewMedia {
    pub state: StickerPlaybackState,
    pub is_hovered: bool,
    pub is_selected: bool,
}

impl HistoryViewStickerViewMedia {
    pub fn new(document_id: u64, format: StickerFormat) -> Self {
        Self {
            state: StickerPlaybackState::new(document_id, format),
            is_hovered: false,
            is_selected: false,
        }
    }

    pub fn with_dice(mut self, emoji: impl Into<String>, value: i32) -> Self {
        self.state.dice_info = Some((emoji.into(), value));
        self
    }

    pub fn with_premium(mut self, is_premium: bool) -> Self {
        self.state.is_premium = is_premium;
        self
    }

    pub fn at_the_end(&self) -> bool {
        self.state.at_the_end()
    }

    pub fn compute_size(&self) -> f32 {
        self.state.compute_size()
    }
}
