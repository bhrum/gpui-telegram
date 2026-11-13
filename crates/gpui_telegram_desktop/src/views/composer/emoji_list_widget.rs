//! Standard and Custom Emoji Grid with Categories and Skin Tone Picker
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_list_widget.h:L1-L620`
//! - `Telegram/SourceFiles/chat_helpers/emoji_list_widget.cpp:L1-L4348`
//! - `Telegram/SourceFiles/chat_helpers/emoji_list_widget.h:L69` (`kEmojiSectionCount = 8`)
//! - `Telegram/SourceFiles/chat_helpers/emoji_list_widget.cpp:L50-L55` (`kCollapsedRows`, `kColorPickerDelay`, `kSearchRequestDelay`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::time::Duration;

pub const K_EMOJI_SECTION_COUNT: usize = 8;
pub const K_COLLAPSED_ROWS: usize = 3;
pub const K_COLOR_PICKER_DELAY: Duration = Duration::from_millis(500);
pub const K_SEARCH_REQUEST_DELAY: Duration = Duration::from_millis(400);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmojiSection {
    Recent = 0,
    People = 1,
    Nature = 2,
    Food = 3,
    Activity = 4,
    Travel = 5,
    Objects = 6,
    Symbols = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinToneModifier {
    Default = 0,
    Light = 1,
    MediumLight = 2,
    Medium = 3,
    MediumDark = 4,
    Dark = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiEntry {
    pub glyph: String,
    pub section: EmojiSection,
    pub keywords: Vec<String>,
    pub supports_skin_tones: bool,
    pub active_skin_tone: SkinToneModifier,
    pub is_custom: bool,
    pub custom_document_id: Option<u64>,
}

/// Emoji List Widget managing categorized emoji sections, search, and skin tone selection.
/// Target symbol in ledger: `EmojiListWidget`.
#[derive(Debug, Clone)]
pub struct EmojiListWidget {
    pub active_section: EmojiSection,
    pub entries: Vec<EmojiEntry>,
    pub search_query: String,
    pub filtered_entries: Vec<usize>, // indices into entries
    pub selected_index: Option<usize>,
    pub skin_picker_open_for_index: Option<usize>,
}

impl Default for EmojiListWidget {
    fn default() -> Self {
        let mut w = Self {
            active_section: EmojiSection::Recent,
            entries: Vec::new(),
            search_query: String::new(),
            filtered_entries: Vec::new(),
            selected_index: None,
            skin_picker_open_for_index: None,
        };
        w.init_standard_emojis();
        w
    }
}

impl EmojiListWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_standard_emojis(&mut self) {
        let sample_emojis = [
            ("👍", EmojiSection::Recent, true),
            ("❤️", EmojiSection::Recent, false),
            ("😊", EmojiSection::People, false),
            ("👋", EmojiSection::People, true),
            ("🐶", EmojiSection::Nature, false),
            ("🍕", EmojiSection::Food, false),
            ("⚽", EmojiSection::Activity, false),
            ("✈️", EmojiSection::Travel, false),
            ("💡", EmojiSection::Objects, false),
            ("⭐", EmojiSection::Symbols, false),
        ];

        for (glyph, section, supports_skin) in sample_emojis {
            self.entries.push(EmojiEntry {
                glyph: glyph.to_string(),
                section,
                keywords: Vec::new(),
                supports_skin_tones: supports_skin,
                active_skin_tone: SkinToneModifier::Default,
                is_custom: false,
                custom_document_id: None,
            });
        }
        self.filtered_entries = (0..self.entries.len()).collect();
    }

    pub fn switch_section(&mut self, section: EmojiSection) {
        self.active_section = section;
        self.search_query.clear();
        self.filtered_entries = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.section == section)
            .map(|(i, _)| i)
            .collect();
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = query.trim().to_lowercase();
        if self.search_query.is_empty() {
            self.switch_section(self.active_section);
            return;
        }

        self.filtered_entries = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                e.glyph.contains(&self.search_query)
                    || e.keywords.iter().any(|k| k.starts_with(&self.search_query))
            })
            .map(|(i, _)| i)
            .collect();
    }

    pub fn set_skin_tone(&mut self, entry_index: usize, tone: SkinToneModifier) {
        if let Some(entry) = self.entries.get_mut(entry_index) {
            if entry.supports_skin_tones {
                entry.active_skin_tone = tone;
            }
        }
        self.skin_picker_open_for_index = None;
    }
}
