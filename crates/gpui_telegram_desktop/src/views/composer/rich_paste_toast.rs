//! RichPasteToast: Rich formatting clipboard paste detector and action toast.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/rich_paste_toast.h:L18-L60`
//! - `Telegram/SourceFiles/chat_helpers/rich_paste_toast.cpp:L32-L92`
#![allow(dead_code)]

use gpui::*;

/// Duration for the rich paste confirmation toast to stay on screen
/// (upstream: `kToastDuration = 9 * 1000ms`).
pub const K_TOAST_DURATION_MS: u64 = 9000;

/// Minimum cooldown interval between offering rich paste formatting toasts
/// (upstream: `kOfferTimeout = 30 * 1000ms`).
pub const K_OFFER_TIMEOUT_MS: u64 = 30000;

/// Formatting offer kind (upstream: `ChatHelpers::RichPasteOffer`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RichPasteOffer {
    #[default]
    Editor,
    Markdown,
    Field,
    Plain,
}

/// Rich text tag descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextTag {
    pub offset: usize,
    pub length: usize,
    pub id: String,
}

/// Text with formatting tags.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaggedText {
    pub text: String,
    pub tags: Vec<TextTag>,
}

/// Replace a substring while updating text tags with appropriate offset shifts
/// (upstream: `TextWithTagsReplaced`, `rich_paste_toast.cpp:L65-L92`).
pub fn replace_text_with_tags(
    original: &TaggedText,
    from: usize,
    till: usize,
    with: &TaggedText,
) -> TaggedText {
    let prefix = &original.text[..from.min(original.text.len())];
    let suffix = if till < original.text.len() {
        &original.text[till..]
    } else {
        ""
    };
    let new_text = format!("{}{}{}", prefix, with.text, suffix);

    let shift = (with.text.len() as isize) - ((till - from) as isize);
    let mut new_tags = Vec::new();

    // 1. Tags before 'from'
    for tag in &original.tags {
        if tag.offset < from {
            let end = (tag.offset + tag.length).min(from);
            new_tags.push(TextTag {
                offset: tag.offset,
                length: end.saturating_sub(tag.offset),
                id: tag.id.clone(),
            });
        }
    }

    // 2. Tags from inserted 'with'
    for tag in &with.tags {
        new_tags.push(TextTag {
            offset: from + tag.offset,
            length: tag.length,
            id: tag.id.clone(),
        });
    }

    // 3. Tags after 'till'
    for tag in &original.tags {
        let end = tag.offset + tag.length;
        if end > till {
            let start = tag.offset.max(till);
            let shifted_start = ((start as isize) + shift).max(0) as usize;
            new_tags.push(TextTag {
                offset: shifted_start,
                length: end.saturating_sub(start),
                id: tag.id.clone(),
            });
        }
    }

    TaggedText {
        text: new_text,
        tags: new_tags,
    }
}

/// Throttle limiter for showing rich paste toasts (upstream: `RichPasteOfferThrottle`).
#[derive(Debug, Clone, Default)]
pub struct RichPasteOfferThrottle {
    pub last_shown_ms: i64,
}

impl RichPasteOfferThrottle {
    pub fn take(&mut self, now_ms: i64) -> bool {
        if self.last_shown_ms == 0 || now_ms - self.last_shown_ms > (K_OFFER_TIMEOUT_MS as i64) {
            self.last_shown_ms = now_ms;
            true
        } else {
            false
        }
    }
}

/// Authentic RichPasteToast component state.
pub struct RichPasteToast {
    pub is_visible: bool,
    pub offer: RichPasteOffer,
    pub throttle: RichPasteOfferThrottle,
    pub updated_at: i64,
}

impl RichPasteToast {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            offer: RichPasteOffer::Editor,
            throttle: RichPasteOfferThrottle::default(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn show(&mut self, offer: RichPasteOffer, now_ms: i64) -> bool {
        if self.throttle.take(now_ms) {
            self.offer = offer;
            self.is_visible = true;
            self.updated_at = chrono::Utc::now().timestamp();
            true
        } else {
            false
        }
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for RichPasteToast {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for RichPasteToast {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p(px(10.0))
            .bg(rgb(0x282E33))
            .rounded(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("Paste with formatting? ({:?})", self.offer))
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child("Format")
                    )
            )
    }
}
