//! Upstream C++ source: `Telegram/SourceFiles/ui/item_text_options.cpp:L1-L70` & `item_text_options.h:L1-L28`
//! Authentic GPUI implementation of rich message text layout options and flags.
#![allow(dead_code)]

/// Flags controlling text layout, link highlighting, and spoiler rendering.
/// C++ companion: `Ui::ItemTextOptions` (`item_text_options.cpp:L17-L70`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemTextOptions {
    pub selectable: bool,
    pub allow_spoilers: bool,
    pub allow_custom_emoji: bool,
    pub allow_quotes: bool,
    pub monospace_as_code: bool,
}

impl ItemTextOptions {
    /// Standard message bubble options.
    pub fn for_history() -> Self {
        Self {
            selectable: true,
            allow_spoilers: true,
            allow_custom_emoji: true,
            allow_quotes: true,
            monospace_as_code: true,
        }
    }

    /// Read-only preview options.
    pub fn for_preview() -> Self {
        Self {
            selectable: false,
            allow_spoilers: true,
            allow_custom_emoji: false,
            allow_quotes: false,
            monospace_as_code: false,
        }
    }
}

impl Default for ItemTextOptions {
    fn default() -> Self {
        Self::for_history()
    }
}
