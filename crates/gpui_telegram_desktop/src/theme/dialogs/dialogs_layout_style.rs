//! Authentic GPUI Telegram theme style sheet for Dialogs Layout Icons and Palettes.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_layout.style:L10-100`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use gpui::{px, rgb, Pixels, Point, Rgba};

/// Palettes and icon offsets for message status and badges within dialog rows.
#[derive(Debug, Clone)]
pub struct DialogsLayoutUiStyleSheet {
    // Icon offsets
    pub bot_icon_offset: Point<Pixels>,       // point(1px, 3px)
    pub forum_icon_offset: Point<Pixels>,     // point(1px, 4px)
    pub sending_icon_offset: Point<Pixels>,   // point(8px, 4px)
    pub lock_icon_offset: Point<Pixels>,      // point(4px, 0px)

    // Badge icon sizes
    pub poll_icon_size: Pixels,               // 18px
    pub mention_badge_size: Pixels,           // 13px
    pub reaction_badge_size: Pixels,          // 13px
    pub poll_badge_size: Pixels,              // 13px

    // Text palette colors for draft and taken states
    pub draft_link_fg: Rgba,
    pub draft_link_fg_over: Rgba,
    pub draft_link_fg_active: Rgba,
    pub sending_icon_fg: Rgba,
    pub verified_badge_fg: Rgba,
    pub verified_badge_fg_active: Rgba,
}

impl Default for DialogsLayoutUiStyleSheet {
    fn default() -> Self {
        Self {
            bot_icon_offset: Point { x: px(1.0), y: px(3.0) },
            forum_icon_offset: Point { x: px(1.0), y: px(4.0) },
            sending_icon_offset: Point { x: px(8.0), y: px(4.0) },
            lock_icon_offset: Point { x: px(4.0), y: px(0.0) },

            poll_icon_size: px(18.0),
            mention_badge_size: px(13.0),
            reaction_badge_size: px(13.0),
            poll_badge_size: px(13.0),

            draft_link_fg: rgb(0xDD3B4B),
            draft_link_fg_over: rgb(0xDD3B4B),
            draft_link_fg_active: rgb(0xFFFFFF),
            sending_icon_fg: rgb(0xA2ABB2),
            verified_badge_fg: rgb(0x2AABEE),
            verified_badge_fg_active: rgb(0xFFFFFF),
        }
    }
}

impl DialogsLayoutUiStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
