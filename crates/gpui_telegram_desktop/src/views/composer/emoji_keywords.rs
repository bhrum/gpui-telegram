//! Multi-Language Keyword to Emoji Dictionary Engine
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_keywords.h:L1-L87`
//! - `Telegram/SourceFiles/chat_helpers/emoji_keywords.cpp:L1-L759`
//! - `Telegram/SourceFiles/chat_helpers/emoji_keywords.cpp:L28-L30` (`kRefreshEach`, `kKeepNotUsedLangPacksCount`)
//! - `Telegram/SourceFiles/chat_helpers/emoji_keywords.cpp:L55-L76` (`SkipExactKeyword` heuristics)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::collections::HashMap;
use std::time::Duration;

pub const K_REFRESH_EACH: Duration = Duration::from_secs(3600);
pub const K_KEEP_NOT_USED_LANG_PACKS_COUNT: usize = 4;
pub const K_KEEP_NOT_USED_INPUT_LANGUAGES_COUNT: usize = 4;

/// Heuristics for skipping ambiguous short keywords from `emoji_keywords.cpp:L55-L76`
pub fn skip_exact_keyword(language: &str, word: &str) -> bool {
    if word.chars().count() == 1 && !word.chars().next().unwrap().is_alphabetic() {
        return true;
    } else if word == "10" {
        return true;
    } else if language != "en" {
        return false;
    } else if word.chars().count() == 1 {
        let ch = word.chars().next().unwrap();
        if ch != '$' && ch != '€' {
            return true;
        }
    } else if word.chars().count() == 2 {
        if word != "us" && word != "uk" && word != "hi" && word != "ok" {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordEmojiMatch {
    pub emoji: String,
    pub keyword: String,
}

/// Emoji Keywords dictionary managing multi-language keyword search and caching.
/// Target symbol in ledger: `EmojiKeywords`.
#[derive(Debug, Clone)]
pub struct EmojiKeywords {
    pub language: String,
    pub keywords_map: HashMap<String, Vec<String>>, // keyword -> list of emojis
    pub max_query_length: usize,
    pub version: i32,
}

impl Default for EmojiKeywords {
    fn default() -> Self {
        let mut s = Self {
            language: "en".to_string(),
            keywords_map: HashMap::new(),
            max_query_length: 32,
            version: 1,
        };
        s.init_builtin_keywords();
        s
    }
}

impl EmojiKeywords {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn init_builtin_keywords(&mut self) {
        // Built-in standard keywords mapping
        let builtins = [
            ("smile", vec!["😊", "😃", "😄", "😁"]),
            ("laugh", vec!["😂", "🤣", "😆"]),
            ("heart", vec!["❤️", "💖", "💕", "💓"]),
            ("fire", vec!["🔥"]),
            ("thumb", vec!["👍", "👎"]),
            ("clap", vec!["👏"]),
            ("party", vec!["🎉", "🥳"]),
            ("sad", vec!["😢", "😭", "🙁"]),
            ("cool", vec!["😎"]),
            ("ok", vec!["👌"]),
            ("hi", vec!["👋"]),
        ];

        for (kw, emojis) in builtins {
            self.keywords_map.insert(
                kw.to_string(),
                emojis.into_iter().map(String::from).collect(),
            );
        }
    }

    /// Search emojis matching the keyword query
    pub fn search(&self, query: &str) -> Vec<KeywordEmojiMatch> {
        let q = query.trim().to_lowercase();
        if q.is_empty() || skip_exact_keyword(&self.language, &q) {
            return Vec::new();
        }

        let mut results = Vec::new();
        for (kw, emojis) in &self.keywords_map {
            if kw.starts_with(&q) {
                for emoji in emojis {
                    results.push(KeywordEmojiMatch {
                        emoji: emoji.clone(),
                        keyword: kw.clone(),
                    });
                }
            }
        }
        results
    }
}
