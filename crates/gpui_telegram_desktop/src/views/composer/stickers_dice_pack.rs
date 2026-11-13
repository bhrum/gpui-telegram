//! StickersDicePack: Telegram animated dice, darts, slot machine, and sports game stickers.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_dice_pack.h:L22-L60`
//! - `Telegram/SourceFiles/chat_helpers/stickers_dice_pack.cpp:L23-L90`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashMap;

/// Standard dice string 🎲 (upstream: `DicePacks::kDiceString`).
pub const DICE_STRING: &str = "\u{1F3B2}";

/// Dart string 🎯 (upstream: `DicePacks::kDartString`).
pub const DART_STRING: &str = "\u{1F3AF}";

/// Slot machine string 🎰 (upstream: `DicePacks::kSlotString`).
pub const SLOT_STRING: &str = "\u{1F3B0}";

/// Football string ⚽ (upstream: `DicePacks::kFballString`).
pub const FBALL_STRING: &str = "\u{26BD}";

/// Basketball string 🏀 (upstream: `DicePacks::kBballString`).
pub const BBALL_STRING: &str = "\u{1F3C0}";

/// Party popper celebration string 🎉 (upstream: `DicePacks::kPartyPopper`).
pub const PARTY_POPPER: &str = "\u{1F389}";

/// Game classification for animated dice emoji.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiceGameKind {
    Dice,
    Dart,
    Slot,
    Football,
    Basketball,
    PartyPopper,
}

impl DiceGameKind {
    pub fn from_emoji(emoji: &str) -> Option<Self> {
        match emoji {
            DICE_STRING => Some(Self::Dice),
            DART_STRING => Some(Self::Dart),
            SLOT_STRING => Some(Self::Slot),
            FBALL_STRING => Some(Self::Football),
            BBALL_STRING => Some(Self::Basketball),
            PARTY_POPPER => Some(Self::PartyPopper),
            _ => None,
        }
    }

    pub fn max_value(&self) -> u32 {
        match self {
            Self::Slot => 64, // 4 * 4 * 4 slot wheel combinations
            Self::PartyPopper => 1,
            _ => 6, // Standard 1-6 outcomes
        }
    }
}

/// Check if emoji is slot machine (upstream: `DicePacks::IsSlot`).
pub fn is_slot(emoji: &str) -> bool {
    emoji == SLOT_STRING
}

/// Parse dice sticker pack index from emoji character (upstream: `stickers_dice_pack.cpp:L88-L89`).
pub fn parse_dice_index(ch: char) -> Option<i32> {
    if ch == '#' {
        Some(0)
    } else if ('1'..='6').contains(&ch) {
        Some((ch as i32) - ('1' as i32) + 1)
    } else {
        None
    }
}

/// Authentic StickersDicePack state.
pub struct StickersDicePack {
    pub current_emoji: String,
    pub current_value: u32,
    pub document_map: HashMap<i32, u64>,
    pub is_rolling: bool,
    pub updated_at: i64,
}

impl StickersDicePack {
    pub fn new(emoji: impl Into<String>) -> Self {
        let emoji_str = emoji.into();
        Self {
            current_emoji: emoji_str,
            current_value: 1,
            document_map: HashMap::new(),
            is_rolling: false,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn set_value(&mut self, val: u32) {
        self.current_value = val;
        self.is_rolling = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn start_roll(&mut self) {
        self.is_rolling = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn bind_document(&mut self, value: i32, document_id: u64) {
        self.document_map.insert(value, document_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn lookup(&self, value: i32) -> Option<u64> {
        self.document_map.get(&value).copied()
    }
}

impl Default for StickersDicePack {
    fn default() -> Self {
        Self::new(DICE_STRING)
    }
}

impl Render for StickersDicePack {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(72.0))
            .h(px(72.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(8.0))
            .bg(rgb(0xF5F5F5))
            .child(
                div()
                    .text_size(px(32.0))
                    .child(self.current_emoji.clone())
            )
    }
}
