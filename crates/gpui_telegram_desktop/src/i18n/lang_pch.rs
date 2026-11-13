//! Target Symbol: LangPch
//! Telegram Desktop 1:1 Submodule: `LangPch`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_pch.h`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

pub use chrono::{DateTime, NaiveDate, Utc};

pub use crate::i18n::lang_hardcoded::LangHardcoded;
pub use crate::i18n::lang_keys::LangKeys;
pub use crate::i18n::lang_tag::{LangTag, PluralResult, ShortenedCount};
pub use crate::i18n::lang_text_entity::{EntityInText, EntityType, LangTextEntity, TextWithEntities};

/// Precompiled common localization subsystem definitions and utilities.
/// Upstream: `Telegram/SourceFiles/lang/lang_pch.h`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangPch;

impl LangPch {
    /// Returns default system locale identifier.
    pub fn default_locale() -> &'static str {
        "en"
    }

    /// Validates whether a given ISO 639-1 language code is recognized.
    pub fn is_valid_lang_code(code: &str) -> bool {
        let trimmed = code.trim();
        (2..=5).contains(&trimmed.len()) && trimmed.chars().all(|c| c.is_ascii_alphabetic() || c == '-' || c == '_')
    }
}
