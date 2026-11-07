//! Authentic GPUI Telegram theme style sheet for Dialogs.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs.style:L14-220`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use gpui::{px, rgb, Pixels, Point, Rgba};

/// Metrics and styling for Dialogs row elements (`dialogs.style:L14-220`).
#[derive(Debug, Clone)]
pub struct DialogsStyleSheet {
    // Row dimensions
    pub default_row_height: Pixels,      // 62px (dialogsRowHeight: 62px)
    pub tagged_row_height: Pixels,       // 72px (taggedDialogRow.height: 72px)
    pub forum_row_height: Pixels,        // 80px (forumDialogRow.height: 80px)
    pub tagged_forum_row_height: Pixels, // 96px (taggedForumDialogRow.height: 96px)
    pub compact_row_height: Pixels,      // 54px compact mode

    // Padding and layout offsets
    pub padding_horizontal: Pixels,      // 10px
    pub padding_vertical: Pixels,        // 8px
    pub photo_size: Pixels,              // 46px (defaultDialogRow.photoSize: 46px)
    pub name_left: Pixels,               // 68px (defaultDialogRow.nameLeft: 68px)
    pub name_top: Pixels,                // 10px (defaultDialogRow.nameTop: 10px)
    pub text_left: Pixels,               // 68px (defaultDialogRow.textLeft: 68px)
    pub text_top: Pixels,                // 34px (defaultDialogRow.textTop: 34px)
    pub text_width_min: Pixels,          // 150px (dialogsTextWidthMin: 150px)
    pub date_skip: Pixels,               // 5px (dialogsDateSkip: 5px)

    // Forum topic metrics
    pub topics_skip: Pixels,             // 8px
    pub topics_skip_big: Pixels,         // 14px
    pub topics_height: Pixels,           // 21px
    pub tag_top: Pixels,                 // 52px (tagged) / 77px (tagged forum)

    // Badges & indicators
    pub unread_height: Pixels,           // 19px (dialogsUnreadHeight: 19px)
    pub unread_padding: Pixels,          // 5px (dialogsUnreadPadding: 5px)
    pub unread_font_size: Pixels,        // 12px bold (dialogsUnreadFont)
    pub online_badge_size: Pixels,       // 10px (dialogsOnlineBadgeSize: 10px)
    pub online_badge_stroke: Pixels,     // 2px
    pub online_badge_skip: Point<Pixels>,// (0px, 2px)
    pub online_badge_duration_ms: u32,   // 150ms
    pub call_badge_size: Pixels,         // 16px
    pub call_badge_skip: Point<Pixels>,  // (-3px, -3px)
    pub subscription_badge_size: Pixels, // 16px
    pub subscription_badge_skip: Point<Pixels>, // (-4px, -4px)
    pub subscription_outline_twice: Pixels, // 3px
    pub ttl_badge_size: Pixels,          // 20px
    pub important_bar_height: Pixels,    // 37px

    // Animation durations (ms)
    pub filter_slide_duration_ms: u32,       // 200ms
    pub filter_swipe_slide_duration_ms: u32, // 140ms

    // Color tokens
    pub bg_color: Rgba,
    pub bg_color_over: Rgba,
    pub bg_color_active: Rgba,
    pub text_fg: Rgba,
    pub text_fg_over: Rgba,
    pub name_fg: Rgba,
    pub unread_bg: Rgba,
    pub unread_bg_muted: Rgba,
    pub unread_fg: Rgba,
    pub date_fg: Rgba,
    pub online_badge_color: Rgba,
}

impl Default for DialogsStyleSheet {
    fn default() -> Self {
        Self {
            default_row_height: px(62.0),
            tagged_row_height: px(72.0),
            forum_row_height: px(80.0),
            tagged_forum_row_height: px(96.0),
            compact_row_height: px(54.0),

            padding_horizontal: px(10.0),
            padding_vertical: px(8.0),
            photo_size: px(46.0),
            name_left: px(68.0),
            name_top: px(10.0),
            text_left: px(68.0),
            text_top: px(34.0),
            text_width_min: px(150.0),
            date_skip: px(5.0),

            topics_skip: px(8.0),
            topics_skip_big: px(14.0),
            topics_height: px(21.0),
            tag_top: px(52.0),

            unread_height: px(19.0),
            unread_padding: px(5.0),
            unread_font_size: px(12.0),
            online_badge_size: px(10.0),
            online_badge_stroke: px(2.0),
            online_badge_skip: Point { x: px(0.0), y: px(2.0) },
            online_badge_duration_ms: 150,
            call_badge_size: px(16.0),
            call_badge_skip: Point { x: px(-3.0), y: px(-3.0) },
            subscription_badge_size: px(16.0),
            subscription_badge_skip: Point { x: px(-4.0), y: px(-4.0) },
            subscription_outline_twice: px(3.0),
            ttl_badge_size: px(20.0),
            important_bar_height: px(37.0),

            filter_slide_duration_ms: 200,
            filter_swipe_slide_duration_ms: 140,

            bg_color: rgb(0xFFFFFF),
            bg_color_over: rgb(0xF4F4F5),
            bg_color_active: rgb(0x40A7E3),
            text_fg: rgb(0x707579),
            text_fg_over: rgb(0x707579),
            name_fg: rgb(0x000000),
            unread_bg: rgb(0x40A7E3),
            unread_bg_muted: rgb(0xA2ABB2),
            unread_fg: rgb(0xFFFFFF),
            date_fg: rgb(0x999999),
            online_badge_color: rgb(0x0AC630),
        }
    }
}

impl DialogsStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes row height dynamically depending on tagged state and forum topics.
    pub fn compute_row_height(&self, has_tags: bool, is_forum: bool) -> Pixels {
        match (has_tags, is_forum) {
            (true, true) => self.tagged_forum_row_height,
            (false, true) => self.forum_row_height,
            (true, false) => self.tagged_row_height,
            (false, false) => self.default_row_height,
        }
    }
}
