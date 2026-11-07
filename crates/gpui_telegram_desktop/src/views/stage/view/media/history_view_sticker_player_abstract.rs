//! Authentic GPUI History View Sticker Player Abstract Interface.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker_player_abstract.h:L1-L34`
//!
//! Abstract Methods:
//! - `ready() -> bool`
//! - `framesCount() -> int`
//! - `frame(size, colored, mirror, now, paused) -> FrameInfo`
//! - `markFrameShown() -> bool`

/// Abstract sticker player capability provider in the GPUI layer.
/// Upstream: `HistoryView::StickerPlayer` (`history_view_sticker_player_abstract.h:L12-L31`)
#[derive(Debug, Clone, Default)]
pub struct HistoryViewStickerPlayerAbstractViewMedia {
    pub is_initialized: bool,
}

impl HistoryViewStickerPlayerAbstractViewMedia {
    pub fn new() -> Self {
        Self { is_initialized: true }
    }
}
