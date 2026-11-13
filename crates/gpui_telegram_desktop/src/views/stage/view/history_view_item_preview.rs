//! History View Item Preview View Component (`history_view_item_preview`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_item_preview.h` (L1-L55)
//!
//! Handles message previews for replied-to messages, forwarded citations, story replies,
//! pinned previews, and media thumbnails with spoiler overlays.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Icon attached to the item preview header.
/// Upstream: `history_view_item_preview.h:L29-L33` (`enum class Icon`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemPreviewIcon {
    #[default]
    None,
    ForwardedMessage,
    ReplyToStory,
}

/// Represents a thumbnail image inside an item preview with spoiler/shape flags.
/// Upstream: `history_view_item_preview.h:L12-L26` (`struct ItemPreviewImage`)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemPreviewImage {
    pub raw_data: Vec<u8>,
    pub cache_key: u64,
}

impl ItemPreviewImage {
    /// Creates a new preview image with an optional spoiler mask.
    pub fn new(data: Vec<u8>, cache_key: u64) -> Self {
        Self {
            raw_data: data,
            cache_key,
        }
    }

    /// Whether the preview image should be obscured by a spoiler animation.
    /// Upstream: `history_view_item_preview.h:L16-L18` (`hasSpoiler() const { return (cacheKey & 1); }`)
    pub fn has_spoiler(&self) -> bool {
        (self.cache_key & 1) != 0
    }

    /// Whether the preview image should be clipped into an ellipse / round circle.
    /// Upstream: `history_view_item_preview.h:L19-L21` (`isEllipse() const { return (cacheKey & 2); }`)
    pub fn is_ellipse(&self) -> bool {
        (self.cache_key & 2) != 0
    }

    /// Whether the image contains valid pixel data.
    /// Upstream: `history_view_item_preview.h:L23-L25` (`explicit operator bool() const`)
    pub fn has_data(&self) -> bool {
        !self.raw_data.is_empty()
    }
}

/// Options controlling preview generation from history items.
/// Upstream: `history_view_item_preview.h:L42-L53` (`struct ToPreviewOptions`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToPreviewOptions {
    pub search_query: String,
    pub hide_sender: bool,
    pub hide_caption: bool,
    pub ignore_message_text: bool,
    pub generate_images: bool,
    pub ignore_group: bool,
    pub ignore_topic: bool,
    pub spoiler_login_code: bool,
    pub translated: bool,
}

impl Default for ToPreviewOptions {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            hide_sender: false,
            hide_caption: false,
            ignore_message_text: false,
            generate_images: true,
            ignore_group: false,
            ignore_topic: true,
            spoiler_login_code: false,
            translated: false,
        }
    }
}

/// Authentic GPUI View for item preview cards in history quotes/replies.
/// Upstream: `history_view_item_preview.h:L28-L40` (`struct ItemPreview`)
#[derive(Debug, Clone)]
pub struct HistoryViewItemPreviewView {
    pub message_id: i64,
    pub sender_name: String,
    pub preview_text: String,
    pub icon: ItemPreviewIcon,
    pub images: Vec<ItemPreviewImage>,
    pub arrow_in_text_pos: i32,
    pub images_in_text_pos: i32,
    pub is_active_preview: bool,
}

impl HistoryViewItemPreviewView {
    /// Constructs a new message preview item.
    pub fn new(message_id: i64, sender: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            message_id,
            sender_name: sender.into(),
            preview_text: text.into(),
            icon: ItemPreviewIcon::None,
            images: Vec::new(),
            arrow_in_text_pos: -1,
            images_in_text_pos: 0,
            is_active_preview: true,
        }
    }

    /// Sets the preview icon (forward arrow, story reply).
    pub fn with_icon(mut self, icon: ItemPreviewIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Adds a thumbnail image to the preview.
    pub fn add_image(&mut self, image: ItemPreviewImage) {
        self.images.push(image);
    }

    /// Checks if any image in the preview has a spoiler.
    pub fn has_any_spoiler(&self) -> bool {
        self.images.iter().any(|img| img.has_spoiler())
    }
}

impl Default for HistoryViewItemPreviewView {
    fn default() -> Self {
        Self::new(0, "Telegram", "Message Preview")
    }
}

impl Render for HistoryViewItemPreviewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let has_thumb = !self.images.is_empty();
        let spoiler = self.has_any_spoiler();

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .w_full()
            .p(px(6.0))
            .rounded(px(4.0))
            .bg(rgb(0xF4F6F8))
            .border_l_2()
            .border_color(rgb(0x27A6E5))
            .child(
                div()
                    .flex_1()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                match self.icon {
                                    ItemPreviewIcon::ForwardedMessage => "↗",
                                    ItemPreviewIcon::ReplyToStory => "⭕",
                                    ItemPreviewIcon::None => "",
                                }
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x27A6E5))
                                    .child(self.sender_name.clone())
                            )
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x333333))
                            .child(self.preview_text.clone())
                    )
            )
            .children(if has_thumb {
                Some(
                    div()
                        .w(px(36.0))
                        .h(px(36.0))
                        .rounded(px(4.0))
                        .bg(if spoiler { rgb(0x444444) } else { rgb(0xCCCCCC) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .child(if spoiler { "✦" } else { "🖼" })
                )
            } else {
                None
            })
    }
}
