//! Floating Quick Emoji and Reaction Picker Overlay
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.h:L1-L112`
//! - `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.cpp:L1-L790`
//! - `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.style:L1-L45`
//!
//! Subsystem: chat_helpers, Feature: TCG-N15

use gpui::{px, Pixels};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerOverlayMode {
    CompactStrip, // 40px quick reaction strip
    ExpandedGrid, // 220px full emoji picker grid
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickerCell {
    pub emoji: String,
    pub is_custom: bool,
    pub custom_id: Option<u64>,
}

/// Floating overlay positioned directly above composer for one-click emoji/reaction insertion.
/// Target symbol in ledger: `EmojiPickerOverlay`.
#[derive(Debug, Clone)]
pub struct EmojiPickerOverlay {
    pub is_open: bool,
    pub mode: PickerOverlayMode,
    pub recent_emojis: Vec<PickerCell>,
    pub grid_emojis: Vec<PickerCell>,
    pub hovered_index: Option<usize>,
    pub selected_index: Option<usize>,

    // Metric tokens from emoji_picker_overlay.style
    pub strip_height: Pixels,
    pub expanded_height: Pixels,
    pub item_size: Pixels,
    pub item_skip: Pixels,
}

impl Default for EmojiPickerOverlay {
    fn default() -> Self {
        let defaults = vec!["👍", "❤️", "🔥", "🎉", "👏", "😂", "😮", "😢"]
            .into_iter()
            .map(|e| PickerCell {
                emoji: e.to_string(),
                is_custom: false,
                custom_id: None,
            })
            .collect();

        Self {
            is_open: false,
            mode: PickerOverlayMode::CompactStrip,
            recent_emojis: defaults,
            grid_emojis: Vec::new(),
            hovered_index: None,
            selected_index: None,
            strip_height: px(40.0),
            expanded_height: px(220.0),
            item_size: px(30.0),
            item_skip: px(4.0),
        }
    }
}

impl EmojiPickerOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_expanded(&mut self) {
        self.mode = match self.mode {
            PickerOverlayMode::CompactStrip => PickerOverlayMode::ExpandedGrid,
            PickerOverlayMode::ExpandedGrid => PickerOverlayMode::CompactStrip,
        };
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.hovered_index = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.hovered_index = None;
    }

    pub fn current_height(&self) -> Pixels {
        match self.mode {
            PickerOverlayMode::CompactStrip => self.strip_height,
            PickerOverlayMode::ExpandedGrid => self.expanded_height,
        }
    }

    pub fn select_cell(&mut self, index: usize) -> Option<PickerCell> {
        let cells = match self.mode {
            PickerOverlayMode::CompactStrip => &self.recent_emojis,
            PickerOverlayMode::ExpandedGrid => &self.grid_emojis,
        };

        if index < cells.len() {
            self.selected_index = Some(index);
            Some(cells[index].clone())
        } else {
            None
        }
    }
}
