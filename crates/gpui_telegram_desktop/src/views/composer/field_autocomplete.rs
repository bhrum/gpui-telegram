//! Field Autocomplete for Mentions (@), Hashtags (#), Bot Commands (/), and Stickers
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.h:L1-L222`
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L1-L2163`
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L66` (`kEphemeralHintHoverDelay`)
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L404-L417` (Trigger parsing)
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L1530-L1562` (`moveSel` navigation math)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::time::Duration;

pub const K_EPHEMERAL_HINT_HOVER_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteType {
    Mentions,
    Hashtags,
    BotCommands,
    Stickers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteChooseMethod {
    ByEnter,
    ByTab,
    ByClick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MentionRow {
    pub user_id: u64,
    pub username: String,
    pub name: String,
    pub is_bot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashtagRow {
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotCommandRow {
    pub command: String,
    pub description: String,
    pub bot_user_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerRow {
    pub document_id: u64,
    pub emoji: String,
}

/// Field Autocomplete controller managing popups for `@user`, `#hashtag`, `/command` and stickers.
/// Target symbol in ledger: `FieldAutocomplete`.
#[derive(Debug, Clone)]
pub struct FieldAutocomplete {
    pub is_open: bool,
    pub autocomplete_type: AutocompleteType,
    pub filter: String,
    pub selected_index: i32,
    pub stickers_per_row: usize,

    pub mention_rows: Vec<MentionRow>,
    pub hashtag_rows: Vec<HashtagRow>,
    pub bot_command_rows: Vec<BotCommandRow>,
    pub sticker_rows: Vec<StickerRow>,
}

impl Default for FieldAutocomplete {
    fn default() -> Self {
        Self {
            is_open: false,
            autocomplete_type: AutocompleteType::Mentions,
            filter: String::new(),
            selected_index: -1,
            stickers_per_row: 5,
            mention_rows: Vec::new(),
            hashtag_rows: Vec::new(),
            bot_command_rows: Vec::new(),
            sticker_rows: Vec::new(),
        }
    }
}

impl FieldAutocomplete {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row_count(&self) -> usize {
        match self.autocomplete_type {
            AutocompleteType::Mentions => self.mention_rows.len(),
            AutocompleteType::Hashtags => self.hashtag_rows.len(),
            AutocompleteType::BotCommands => self.bot_command_rows.len(),
            AutocompleteType::Stickers => self.sticker_rows.len(),
        }
    }

    /// Process input query from composer field. Directly maps `field_autocomplete.cpp:L404-L417`.
    pub fn show_filtered(&mut self, query: &str) {
        if query.is_empty() {
            self.hide();
            return;
        }

        let first = query.chars().next().unwrap_or('\0');
        let (kind, plain) = match first {
            '@' => (AutocompleteType::Mentions, &query[1..]),
            '#' => (AutocompleteType::Hashtags, &query[1..]),
            '/' => (AutocompleteType::BotCommands, &query[1..]),
            _ => {
                self.hide();
                return;
            }
        };

        self.autocomplete_type = kind;
        self.filter = plain.to_lowercase();
        self.is_open = true;
        self.selected_index = if self.row_count() > 0 { 0 } else { -1 };
    }

    pub fn hide(&mut self) {
        self.is_open = false;
        self.filter.clear();
        self.selected_index = -1;
    }

    /// Keyboard selection navigation math from `field_autocomplete.cpp:L1530-L1562`.
    pub fn move_selection(&mut self, key: &str) -> bool {
        let max_sel = self.row_count() as i32;
        if max_sel == 0 {
            return false;
        }

        let mut direction = match key {
            "Up" | "ArrowUp" => -1,
            "Down" | "ArrowDown" => 1,
            _ => 0,
        };

        if self.autocomplete_type == AutocompleteType::Stickers {
            if key == "Left" || key == "ArrowLeft" {
                direction = -1;
            } else if key == "Right" || key == "ArrowRight" {
                direction = 1;
            } else {
                direction *= self.stickers_per_row as i32;
            }
        }

        if direction == 0 {
            return false;
        }

        if self.selected_index < 0 || self.selected_index >= max_sel {
            if direction < -1 {
                let spr = self.stickers_per_row as i32;
                self.selected_index = ((max_sel - 1) / spr) * spr;
            } else if direction < 0 {
                self.selected_index = max_sel - 1;
            } else {
                self.selected_index = 0;
            }
            return self.selected_index >= 0 && self.selected_index < max_sel;
        }

        let next = self.selected_index + direction;
        if next < 0 || next >= max_sel {
            // Wrap or unselect
            self.selected_index = if next < 0 { max_sel - 1 } else { 0 };
        } else {
            self.selected_index = next;
        }

        true
    }

    /// Select chosen element at current index
    pub fn choose_selected(&self, _method: AutocompleteChooseMethod) -> Option<String> {
        if self.selected_index < 0 || self.selected_index as usize >= self.row_count() {
            return None;
        }

        let idx = self.selected_index as usize;
        match self.autocomplete_type {
            AutocompleteType::Mentions => {
                let row = &self.mention_rows[idx];
                Some(if !row.username.is_empty() {
                    format!("@{}", row.username)
                } else {
                    row.name.clone()
                })
            }
            AutocompleteType::Hashtags => {
                Some(format!("#{}", self.hashtag_rows[idx].tag))
            }
            AutocompleteType::BotCommands => {
                Some(format!("/{}", self.bot_command_rows[idx].command))
            }
            AutocompleteType::Stickers => {
                Some(format!("sticker:{}", self.sticker_rows[idx].document_id))
            }
        }
    }
}
