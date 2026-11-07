//! SpellcheckerCommon: Spellchecking dictionary manifest and locale configuration.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/spellchecker_common.h:L20-L45`
//! - `Telegram/SourceFiles/chat_helpers/spellchecker_common.cpp:L42-L95`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashSet;

/// Supported dictionary extensions (upstream: `kDictExtensions = { "dic", "aff" }`).
pub const DICT_EXTENSIONS: &[&str] = &["dic", "aff"];

/// Telegram public channel hosting dictionary blobs (upstream: `kDictionariesManifestChannel = "tdhbcfiles"`).
pub const MANIFEST_CHANNEL: &str = "tdhbcfiles";

/// Telegram post ID with dictionaries manifest JSON (upstream: `kDictionariesManifestPostId = 2949`).
pub const MANIFEST_POST_ID: i32 = 2949;

/// Pack Language With Country code (upstream: `spellchecker_common.cpp:L53-L58`).
pub fn encode_lwc(language: i32, country: i32, is_default_country: bool) -> i32 {
    if is_default_country {
        language
    } else {
        (language * 1000) + country
    }
}

/// Parse location string formatted as "channel#postId" (upstream: `spellchecker_common.cpp:L80-L90`).
pub fn parse_manifest_location(text: &str) -> Option<(String, i32)> {
    let mut parts = text.split('#');
    let channel = parts.next()?;
    let post_str = parts.next()?;
    let post_id = post_str.parse::<i32>().ok()?;
    if !channel.is_empty() && post_id > 0 {
        Some((channel.to_string(), post_id))
    } else {
        None
    }
}

/// Dictionary entry representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictionaryEntry {
    pub lang_id: i32,
    pub title: String,
    pub size_bytes: i64,
    pub is_downloaded: bool,
}

/// Authentic SpellcheckerCommon state.
pub struct SpellcheckerCommon {
    pub is_enabled: bool,
    pub active_lang_ids: HashSet<i32>,
    pub dictionaries: Vec<DictionaryEntry>,
    pub updated_at: i64,
}

impl SpellcheckerCommon {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            active_lang_ids: HashSet::new(),
            dictionaries: Vec::new(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn add_dictionary(&mut self, dict: DictionaryEntry) {
        self.dictionaries.push(dict);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn enable_language(&mut self, lang_id: i32) {
        self.active_lang_ids.insert(lang_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn disable_language(&mut self, lang_id: i32) {
        self.active_lang_ids.remove(&lang_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for SpellcheckerCommon {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SpellcheckerCommon {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.active_lang_ids.len();

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .p(px(6.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x444444))
                    .child(format!("Spellcheck active ({} langs)", active_count))
            )
    }
}
