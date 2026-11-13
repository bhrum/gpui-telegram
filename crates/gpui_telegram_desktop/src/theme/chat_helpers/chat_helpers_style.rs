//! Chat Helpers Theme Style Sheet
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/chat_helpers.style:L1-L1907`
//! - `Telegram/SourceFiles/chat_helpers/chat_helpers.style:L790-L835` (`defaultEmojiPan`)
//! - `Telegram/SourceFiles/chat_helpers/chat_helpers.style:L465-L530` (`autocomplete`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use gpui::{px, rgb, rgba, Pixels, Rgba};

/// Design tokens and pixel metrics for Telegram Desktop chat helpers,
/// including tabbed selector, emoji panel, autocomplete suggestions, and composer geometry.
#[derive(Debug, Clone)]
pub struct ChatHelpersStyleSheet {
    // --- Tabbed Panel & Emoji Pan Geometry ---
    /// `emojiPanWidth: 345px;` (chat_helpers.style:L774)
    pub emoji_pan_width: Pixels,
    /// `emojiPanMaxHeight: 640px;` (chat_helpers.style:L775)
    pub emoji_pan_max_height: Pixels,
    /// `emojiPanMinHeight: 278px;` (chat_helpers.style:L776)
    pub emoji_pan_min_height: Pixels,
    /// `emojiPanHeightRatio: 0.75;` (chat_helpers.style:L777)
    pub emoji_pan_height_ratio: f32,
    /// `emojiPanRadius: 8px;` (chat_helpers.style:L778)
    pub emoji_pan_radius: Pixels,

    // --- Panel Inner Metrics (chat_helpers.style:L795-L835) ---
    pub panel_margin_horiz: Pixels,
    pub panel_padding_top: Pixels,
    pub panel_padding_bottom: Pixels,
    pub panel_padding_horiz: Pixels,
    /// `desiredSize: 37px;`
    pub desired_item_size: Pixels,
    /// `header: 33px;`
    pub header_height: Pixels,
    pub header_left: Pixels,
    pub header_top: Pixels,
    /// `footer: 36px;`
    pub footer_height: Pixels,
    /// `iconSkip: 3px;`
    pub icon_skip: Pixels,
    /// `iconWidth: 30px;`
    pub icon_width: Pixels,
    /// `iconArea: 28px;`
    pub icon_area: Pixels,

    // --- Search Bar in Tabbed Selector ---
    pub search_pack_width: Pixels,
    pub search_margin_top: Pixels,
    pub search_margin_bottom: Pixels,
    pub search_margin_horiz: Pixels,

    // --- Composer Field Bounds (chat_helpers.style:L1200-L1250) ---
    /// `historyComposeFieldMaxHeight: 224px;`
    pub compose_field_max_height: Pixels,
    /// `historySendSize: size(44px, 46px);`
    pub send_button_width: Pixels,
    pub send_button_height: Pixels,
    pub send_button_padding: Pixels,
    pub document_margin: Pixels,

    // --- Autocomplete Metrics (chat_helpers.style:L465-L530) ---
    pub autocomplete_row_height: Pixels,
    pub autocomplete_search_padding_horiz: Pixels,
    pub autocomplete_search_padding_vert: Pixels,
    pub autocomplete_bottom_skip: Pixels,

    // --- Color Tokens ---
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub subtext_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub hover_bg: Rgba,
    pub active_bg: Rgba,
    pub categories_bg: Rgba,
    pub categories_hover_bg: Rgba,
    pub shadow_color: Rgba,
}

impl Default for ChatHelpersStyleSheet {
    fn default() -> Self {
        Self {
            emoji_pan_width: px(345.0),
            emoji_pan_max_height: px(640.0),
            emoji_pan_min_height: px(278.0),
            emoji_pan_height_ratio: 0.75,
            emoji_pan_radius: px(8.0),

            panel_margin_horiz: px(7.0),
            panel_padding_top: px(0.0),
            panel_padding_bottom: px(7.0),
            panel_padding_horiz: px(7.0),
            desired_item_size: px(37.0),
            header_height: px(33.0),
            header_left: px(14.0),
            header_top: px(10.0),
            footer_height: px(36.0),
            icon_skip: px(3.0),
            icon_width: px(30.0),
            icon_area: px(28.0),

            search_pack_width: px(70.0),
            search_margin_top: px(11.0),
            search_margin_bottom: px(5.0),
            search_margin_horiz: px(2.0),

            compose_field_max_height: px(224.0),
            send_button_width: px(44.0),
            send_button_height: px(46.0),
            send_button_padding: px(4.0),
            document_margin: px(4.0),

            autocomplete_row_height: px(36.0),
            autocomplete_search_padding_horiz: px(16.0),
            autocomplete_search_padding_vert: px(5.0),
            autocomplete_bottom_skip: px(0.0),

            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            subtext_color: rgb(0x707579),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
            hover_bg: rgba(0x0000000D),
            active_bg: rgba(0x2AABEE1A),
            categories_bg: rgb(0xF4F4F5),
            categories_hover_bg: rgba(0x00000014),
            shadow_color: rgba(0x00000033),
        }
    }
}

impl ChatHelpersStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate clamped panel content height based on available vertical space.
    /// Derived from `tabbed_panel.cpp:L194-L208`.
    pub fn clamp_panel_height(&self, available_height: f32) -> Pixels {
        let wanted = (available_height * self.emoji_pan_height_ratio).round();
        let min_h = f32::from(self.emoji_pan_min_height);
        let max_h = f32::from(self.emoji_pan_max_height);
        px(wanted.clamp(min_h, max_h))
    }

    /// Calculate min composer input height: `historySendSize.height() - 2 * historySendPadding`
    /// Derived from `message_field.cpp:L705-L706`.
    pub fn composer_min_height(&self) -> Pixels {
        self.send_button_height - (self.send_button_padding * 2.0)
    }
}
