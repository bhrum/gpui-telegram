//! macOS TouchBar Emoji & Sticker Scrubber Item
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.h, Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Item type selector matching TouchBar scrubber.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrubberMode {
    Emoji,
    Stickers,
}

/// An individual emoji or sticker entry displayed in the TouchBar scrubber.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrubberItemEntry {
    pub id: String,
    pub display_text: String,
    pub is_sticker: bool,
    pub document_id: Option<u64>,
}

/// Interactive Emoji & Sticker scrubber and GPUI Render component for TouchBar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacScrubberItemMacTouchbarItems {
    pub mode: ScrubberMode,
    pub is_popover_open: bool,
    pub items: Vec<ScrubberItemEntry>,
    pub selected_item: Option<ScrubberItemEntry>,
    pub ideal_icon_size: i32,
    pub segment_icon_size: i32,
    pub segment_size: i32,
    pub max_sticker_sets: i32,
}

impl MacScrubberItemMacTouchbarItems {
    pub const IDEAL_ICON_SIZE: i32 = 36;
    pub const SEGMENT_ICON_SIZE: i32 = 25;
    pub const SEGMENT_SIZE: i32 = 92;
    pub const MAX_STICKER_SETS: i32 = 5;

    pub fn new() -> Self {
        Self {
            mode: ScrubberMode::Emoji,
            is_popover_open: false,
            items: Vec::new(),
            selected_item: None,
            ideal_icon_size: Self::IDEAL_ICON_SIZE,
            segment_icon_size: Self::SEGMENT_ICON_SIZE,
            segment_size: Self::SEGMENT_SIZE,
            max_sticker_sets: Self::MAX_STICKER_SETS,
        }
    }

    pub fn set_mode(&mut self, mode: ScrubberMode) -> Result<()> {
        self.mode = mode;
        Ok(())
    }

    pub fn open_popover(&mut self) -> Result<()> {
        self.is_popover_open = true;
        Ok(())
    }

    pub fn dismiss_popover(&mut self) -> Result<()> {
        self.is_popover_open = false;
        Ok(())
    }

    pub fn add_item(&mut self, item: ScrubberItemEntry) -> Result<()> {
        self.items.push(item);
        Ok(())
    }

    pub fn select_and_send(&mut self, id: &str) -> Result<Option<ScrubberItemEntry>> {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            let item = self.items[pos].clone();
            self.selected_item = Some(item.clone());
            self.is_popover_open = false;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

impl Default for MacScrubberItemMacTouchbarItems {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacScrubberItemMacTouchbarItems {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let open = self.is_popover_open;
        let mode = self.mode;

        let mut root = div()
            .h(px(30.0))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_2()
            .bg(rgb(0x222222))
            .rounded_md()
            .child(
                div()
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(rgb(0x383838))
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(if mode == ScrubberMode::Emoji { "😊" } else { "⭐" }),
            );

        if open {
            root = root.children(self.items.iter().map(|item| {
                div()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded_sm()
                    .bg(rgb(0x2f2f2f))
                    .hover(|s| s.bg(rgb(0x454545)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(item.display_text.clone())
            }));
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrubber_items_and_selection() {
        let mut scrubber = MacScrubberItemMacTouchbarItems::new();
        assert_eq!(scrubber.ideal_icon_size, 36);

        scrubber
            .add_item(ScrubberItemEntry {
                id: "emoji_smile".into(),
                display_text: "😀".into(),
                is_sticker: false,
                document_id: None,
            })
            .unwrap();

        scrubber.open_popover().unwrap();
        assert!(scrubber.is_popover_open);

        let sent = scrubber.select_and_send("emoji_smile").unwrap();
        assert!(sent.is_some());
        assert!(!scrubber.is_popover_open);
        assert_eq!(scrubber.selected_item.unwrap().display_text, "😀");
    }
}
