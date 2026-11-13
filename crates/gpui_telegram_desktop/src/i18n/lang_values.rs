//! Target Symbol: LangValues
//! Telegram Desktop 1:1 Submodule: `LangValues`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_values.h`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use std::collections::HashMap;
use crate::i18n::lang_tag::LangTag;

/// Each plural entry occupies a contiguous block of 6 keys in the string table.
/// Upstream: `inline constexpr auto kPluralCount = 6;`
pub const K_PLURAL_COUNT: usize = 6;

/// Reactive translation value store and multi-tag replacement accumulator.
/// Upstream: `Lang::details::Producer` & `Lang::details::ReplaceUnwrap`
#[derive(Debug, Clone, Default)]
pub struct LangValues {
    dense_keys: Vec<Option<String>>,
    named_overrides: HashMap<String, String>,
}

impl LangValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(keys_count: usize) -> Self {
        Self {
            dense_keys: vec![None; keys_count],
            named_overrides: HashMap::new(),
        }
    }

    pub fn set_key(&mut self, key_id: u16, value: impl Into<String>) {
        let idx = key_id as usize;
        if idx >= self.dense_keys.len() {
            self.dense_keys.resize(idx + 1, None);
        }
        self.dense_keys[idx] = Some(value.into());
    }

    pub fn get_key(&self, key_id: u16) -> Option<&str> {
        self.dense_keys.get(key_id as usize)?.as_deref()
    }

    pub fn set_named_override(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.named_overrides.insert(key.into(), value.into());
    }

    pub fn get_named_override(&self, key: &str) -> Option<&str> {
        self.named_overrides.get(key).map(|s| s.as_str())
    }

    /// Resolves plural string by base key and CLDR count.
    /// Selects `base + shift` (where shift in 0..5), cascading to shift 5 (Other) if missing.
    pub fn get_plural(&self, base_key: u16, lang_code: &str, count: f64) -> Option<String> {
        let plural = LangTag::plural(lang_code, count);
        let target_key = base_key.saturating_add(plural.key_shift as u16);

        // Try exact shift key, then fallback to 'other' shift (5), then base key (0)
        let raw_template = self
            .get_key(target_key)
            .or_else(|| self.get_key(base_key + 5))
            .or_else(|| self.get_key(base_key))?;

        // Replace tag with count
        Some(LangTag::replace_tag(raw_template, 0x01, &plural.replacement))
    }

    /// Resolves and chains multiple tag replacements on a template string.
    pub fn replace_tags(template: &str, replacements: &[(u16, &str)]) -> String {
        let mut result = template.to_string();
        for &(tag, val) in replacements {
            result = LangTag::replace_tag(&result, tag, val);
        }
        result
    }
}
