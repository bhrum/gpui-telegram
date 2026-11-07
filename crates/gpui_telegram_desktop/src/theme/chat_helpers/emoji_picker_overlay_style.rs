//! Emoji Picker Overlay Theme Style Sheet
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.style:L1-L45`
//!
//! Subsystem: chat_helpers, Feature: TCG-N15

use gpui::{px, rgb, rgba, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct EmojiPickerOverlayStyleSheet {
    /// `stickersEmojiPickerExpandedRadius: 20px;` (emoji_picker_overlay.style:L10)
    pub expanded_radius: Pixels,
    /// `stickersEmojiPickerPadding: margins(12px, 8px, 12px, 0px);` (L12)
    pub padding_horizontal: Pixels,
    pub padding_top: Pixels,
    pub padding_bottom: Pixels,
    /// `stickersEmojiPickerItemSize: 30px;` (L13)
    pub item_size: Pixels,
    /// `stickersEmojiPickerItemSkip: 4px;` (L14)
    pub item_skip: Pixels,
    /// `stickersEmojiPickerStripHeight: 40px;` (L15)
    pub strip_height: Pixels,
    /// `stickersEmojiPickerSearchSkip: 6px;` (L16)
    pub search_skip: Pixels,
    /// `stickersEmojiPickerExpandedHeight: 220px;` (L17)
    pub expanded_height: Pixels,
    /// `stickersEmojiPickerExpandSize: 24px;` (L39)
    pub expand_button_size: Pixels,

    // Colors & Shadow
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub subtext_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub ripple_bg: Rgba,
    pub shadow_blur: Pixels,
    pub shadow_offset_y: Pixels,
    pub shadow_opacity: f32,
}

impl Default for EmojiPickerOverlayStyleSheet {
    fn default() -> Self {
        Self {
            expanded_radius: px(20.0),
            padding_horizontal: px(12.0),
            padding_top: px(8.0),
            padding_bottom: px(0.0),
            item_size: px(30.0),
            item_skip: px(4.0),
            strip_height: px(40.0),
            search_skip: px(6.0),
            expanded_height: px(220.0),
            expand_button_size: px(24.0),

            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            subtext_color: rgb(0x707579),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
            ripple_bg: rgba(0x00000010),
            shadow_blur: px(20.0),
            shadow_offset_y: px(6.0),
            shadow_opacity: 0.22,
        }
    }
}

impl EmojiPickerOverlayStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate total strip item horizontal advance: `item_size + item_skip`
    pub fn item_advance(&self) -> Pixels {
        self.item_size + self.item_skip
    }
}
