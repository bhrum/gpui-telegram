//! Target Symbol: LangNumbersAnimation
//! Telegram Desktop 1:1 Submodule: `LangNumbersAnimation`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_numbers_animation.h`, `lang_numbers_animation.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::i18n::lang_tag::LangTag;

/// Representation of an i18n string containing an animated numeric section (e.g. member count, reactions).
/// Upstream: `Ui::StringWithNumbers`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StringWithNumbers {
    pub text: String,
    pub offset: usize,
    pub length: usize,
}

impl StringWithNumbers {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            offset: 0,
            length: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangNumbersAnimation;

impl LangNumbersAnimation {
    /// Replaces placeholder tag in an animated number string, tracking the numeric substring range.
    /// Upstream: `ReplaceTag<Ui::StringWithNumbers>::Call`
    pub fn replace_tag(
        mut original: StringWithNumbers,
        tag: u16,
        replacement: &StringWithNumbers,
    ) -> StringWithNumbers {
        if let Some(pos) = LangTag::find_tag_replacement_position(&original.text, tag) {
            original.offset = pos;
            let tag_token_len = 2; // placeholder tag token (\x20 + tag_byte)
            let mut new_text = String::with_capacity(original.text.len() + replacement.text.len());
            new_text.push_str(&original.text[..pos]);
            new_text.push_str(&replacement.text);
            if pos + tag_token_len < original.text.len() {
                new_text.push_str(&original.text[pos + tag_token_len..]);
            }
            original.text = new_text;
            original.length = replacement.text.len();
        }
        original
    }

    /// Helper to wrap formatted numeric counter for slot-machine animation.
    pub fn wrap_count(count_str: impl Into<String>) -> StringWithNumbers {
        let s = count_str.into();
        let len = s.len();
        StringWithNumbers {
            text: s,
            offset: 0,
            length: len,
        }
    }
}
