//! Authentic GPUI Telegram theme style sheet for Dialogs Widget and Header Controls.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_widget.style:L10-67`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use gpui::{px, rgb, Pixels, Point, Rgba};

/// Metrics and styling for the top search header, lock button, and auxiliary buttons.
#[derive(Debug, Clone)]
pub struct DialogsWidgetStyleSheet {
    pub search_header_height: Pixels,    // 48px
    pub search_input_height: Pixels,     // 36px
    pub search_input_radius: Pixels,     // 18px (rounded)
    pub search_debounce_ms: u64,         // 900ms (kSearchRequestDelay)

    // Lock button (dialogsLock: width 36px, height 38px)
    pub lock_width: Pixels,
    pub lock_height: Pixels,
    pub lock_icon_position: Point<Pixels>, // point(-1px, -1px)

    // Narrow search toggle (dialogsSearchForNarrowFilters)
    pub narrow_search_icon_position: Point<Pixels>, // point(4px, 4px)

    // Load more button (dialogsLoadMoreButton: height 36px, textTop 9px)
    pub load_more_height: Pixels,
    pub load_more_text_top: Pixels,

    // Install update badge
    pub install_update_size: Pixels,     // 20px
    pub install_update_icon_skip: Pixels,// 7px
    pub install_update_inner_margin: Pixels, // 5px

    // Community request bubble (dialogsCommunityRequestsBubble)
    pub community_requests_height: Pixels,      // 44px
    pub community_requests_radius: Pixels,      // 11px
    pub community_requests_icon_margin: Pixels, // 9px
    pub community_requests_text_skip: Pixels,   // 48px
    pub community_requests_right_inset: Pixels, // 9px

    // Color tokens
    pub header_bg: Rgba,
    pub search_bg: Rgba,
    pub search_text_fg: Rgba,
    pub search_placeholder_fg: Rgba,
    pub button_active_bg: Rgba,
    pub button_active_fg: Rgba,
    pub menu_icon_fg: Rgba,
}

impl Default for DialogsWidgetStyleSheet {
    fn default() -> Self {
        Self {
            search_header_height: px(48.0),
            search_input_height: px(36.0),
            search_input_radius: px(18.0),
            search_debounce_ms: 900,

            lock_width: px(36.0),
            lock_height: px(38.0),
            lock_icon_position: Point { x: px(-1.0), y: px(-1.0) },

            narrow_search_icon_position: Point { x: px(4.0), y: px(4.0) },

            load_more_height: px(36.0),
            load_more_text_top: px(9.0),

            install_update_size: px(20.0),
            install_update_icon_skip: px(7.0),
            install_update_inner_margin: px(5.0),

            community_requests_height: px(44.0),
            community_requests_radius: px(11.0),
            community_requests_icon_margin: px(9.0),
            community_requests_text_skip: px(48.0),
            community_requests_right_inset: px(9.0),

            header_bg: rgb(0xFFFFFF),
            search_bg: rgb(0xF1F3F4),
            search_text_fg: rgb(0x222222),
            search_placeholder_fg: rgb(0xA2ABB2),
            button_active_bg: rgb(0x40A7E3),
            button_active_fg: rgb(0xFFFFFF),
            menu_icon_fg: rgb(0x707579),
        }
    }
}

impl DialogsWidgetStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
