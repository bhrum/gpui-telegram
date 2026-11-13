//! macOS TouchBar Rich Text Formatter Popover Item
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.h, Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Text formatting commands matching macOS TouchBar popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum MacFormatCommand {
    Bold = 0x010,
    Italic = 0x011,
    Underline = 0x012,
    StrikeOut = 0x013,
    Blockquote = 0x014,
    Monospace = 0x015,
    Clear = 0x016,
    Link = 0x017,
}

impl MacFormatCommand {
    pub fn label(self) -> &'static str {
        match self {
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::Underline => "Underline",
            Self::StrikeOut => "Strike",
            Self::Blockquote => "Quote",
            Self::Monospace => "Mono",
            Self::Clear => "Clear",
            Self::Link => "Link",
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            Self::Bold => "Ctrl+B",
            Self::Italic => "Ctrl+I",
            Self::Underline => "Ctrl+U",
            Self::StrikeOut => "Ctrl+Shift+X",
            Self::Blockquote => "Ctrl+Shift+.",
            Self::Monospace => "Ctrl+Shift+M",
            Self::Clear => "Ctrl+Shift+N",
            Self::Link => "Ctrl+K",
        }
    }
}

/// Rich text format popover item and GPUI Render component for macOS TouchBar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacFormatterItemMacTouchbarItems {
    pub is_popover_open: bool,
    pub selected_command: Option<MacFormatCommand>,
    pub dispatched_commands: Vec<MacFormatCommand>,
}

impl MacFormatterItemMacTouchbarItems {
    pub const ALL_COMMANDS: [MacFormatCommand; 8] = [
        MacFormatCommand::Bold,
        MacFormatCommand::Italic,
        MacFormatCommand::Underline,
        MacFormatCommand::StrikeOut,
        MacFormatCommand::Blockquote,
        MacFormatCommand::Monospace,
        MacFormatCommand::Clear,
        MacFormatCommand::Link,
    ];

    pub fn new() -> Self {
        Self {
            is_popover_open: false,
            selected_command: None,
            dispatched_commands: Vec::new(),
        }
    }

    pub fn open_popover(&mut self) -> Result<()> {
        self.is_popover_open = true;
        Ok(())
    }

    pub fn dismiss_popover(&mut self) -> Result<()> {
        self.is_popover_open = false;
        Ok(())
    }

    pub fn dispatch_command(&mut self, cmd: MacFormatCommand) -> Result<()> {
        self.selected_command = Some(cmd);
        self.dispatched_commands.push(cmd);
        self.is_popover_open = false;
        Ok(())
    }
}

impl Default for MacFormatterItemMacTouchbarItems {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacFormatterItemMacTouchbarItems {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let open = self.is_popover_open;
        let mut root = div()
            .h(px(30.0))
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px_2()
            .bg(rgb(0x2d2d2d))
            .rounded_md()
            .child(
                div()
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(if open { rgb(0x4a4a4a) } else { rgb(0x3a3a3a) })
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child("BIU"),
            );

        if open {
            root = root.children(Self::ALL_COMMANDS.iter().map(|&cmd| {
                div()
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(rgb(0x383838))
                    .hover(|s| s.bg(rgb(0x505050)))
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(cmd.label())
            }));
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_commands() {
        let mut item = MacFormatterItemMacTouchbarItems::new();
        assert!(!item.is_popover_open);

        item.open_popover().unwrap();
        assert!(item.is_popover_open);

        item.dispatch_command(MacFormatCommand::Bold).unwrap();
        assert!(!item.is_popover_open);
        assert_eq!(item.selected_command, Some(MacFormatCommand::Bold));
        assert_eq!(item.dispatched_commands.len(), 1);
    }
}
