//! macOS TouchBar Main Host Component (Pinned Chats, Formatter, Emoji Picker)
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Active panel mode for TouchBar Main strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacMainPanelMode {
    PinnedChats,
    TextFormatPopover,
    EmojiStickerPopover,
}

/// TouchBar Main item aggregation and GPUI Render container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarMainMacTouchbar {
    pub active_mode: MacMainPanelMode,
    pub has_active_chat: bool,
    pub markdown_enabled: bool,
    pub template_identifiers: Vec<String>,
}

impl MacTouchbarMainMacTouchbar {
    pub const ITEM_PINNED_PANEL: &'static str = "pinnedPanel";
    pub const ITEM_POPOVER_INPUT: &'static str = "popoverInput";
    pub const ITEM_POPOVER_PICKER: &'static str = "pickerButtons";

    pub fn new() -> Self {
        Self {
            active_mode: MacMainPanelMode::PinnedChats,
            has_active_chat: false,
            markdown_enabled: true,
            template_identifiers: vec![
                Self::ITEM_PINNED_PANEL.into(),
                Self::ITEM_POPOVER_INPUT.into(),
                Self::ITEM_POPOVER_PICKER.into(),
            ],
        }
    }

    pub fn set_active_chat(&mut self, has_chat: bool) -> Result<()> {
        self.has_active_chat = has_chat;
        Ok(())
    }

    pub fn set_markdown_enabled(&mut self, enabled: bool) -> Result<()> {
        self.markdown_enabled = enabled;
        Ok(())
    }

    pub fn switch_panel_mode(&mut self, mode: MacMainPanelMode) -> Result<()> {
        self.active_mode = mode;
        Ok(())
    }

    pub fn active_item_identifiers(&self) -> Vec<&'static str> {
        let mut items = vec![Self::ITEM_PINNED_PANEL];
        if self.markdown_enabled {
            items.push(Self::ITEM_POPOVER_INPUT);
        } else if self.has_active_chat {
            items.push(Self::ITEM_POPOVER_PICKER);
        }
        items
    }
}

impl Default for MacTouchbarMainMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacTouchbarMainMacTouchbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mode = self.active_mode;

        div()
            .h(px(30.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_2()
            .bg(rgb(0x1e1e1e))
            .rounded_md()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .bg(if mode == MacMainPanelMode::PinnedChats { rgb(0x40a7e3) } else { rgb(0x2d2d2d) })
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Pinned"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .bg(if mode == MacMainPanelMode::TextFormatPopover { rgb(0x40a7e3) } else { rgb(0x2d2d2d) })
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Format"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .bg(if mode == MacMainPanelMode::EmojiStickerPopover { rgb(0x40a7e3) } else { rgb(0x2d2d2d) })
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Stickers"),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touchbar_main_items_and_modes() {
        let mut main = MacTouchbarMainMacTouchbar::new();
        assert_eq!(main.active_item_identifiers(), vec!["pinnedPanel", "popoverInput"]);

        main.set_markdown_enabled(false).unwrap();
        main.set_active_chat(true).unwrap();
        assert_eq!(main.active_item_identifiers(), vec!["pinnedPanel", "pickerButtons"]);

        main.switch_panel_mode(MacMainPanelMode::EmojiStickerPopover).unwrap();
        assert_eq!(main.active_mode, MacMainPanelMode::EmojiStickerPopover);
    }
}
