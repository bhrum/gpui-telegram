//! Target Symbol: LangTag
//! Telegram Desktop 1:1 Submodule: `LangTag`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_tag.h`, `lang_tag.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

pub const K_TEXT_COMMAND: u8 = 0x10;
pub const K_TEXT_COMMAND_LANG_TAG: u8 = 0x20;
pub const K_TAG_REPLACEMENT_SIZE: usize = 4;

pub const K_SHIFT_ZERO: usize = 0;
pub const K_SHIFT_ONE: usize = 1;
pub const K_SHIFT_TWO: usize = 2;
pub const K_SHIFT_FEW: usize = 3;
pub const K_SHIFT_MANY: usize = 4;
pub const K_SHIFT_OTHER: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortenedCount {
    pub number: i64,
    pub string: String,
    pub shortened: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluralResult {
    pub key_shift: usize,
    pub replacement: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangTag;

impl LangTag {
    /// Formats large integer counts into compact strings (e.g. 1.2K, 3.4M, 1B).
    /// Upstream: `FormatCountToShort`
    pub fn format_count_to_short(number: i64, only_k: bool) -> ShortenedCount {
        let abs_num = number.abs();
        if !only_k && abs_num >= 1_000_000_000 {
            let val = (number as f64) / 1_000_000_000.0;
            ShortenedCount {
                number,
                string: Self::format_short_float(val, "B"),
                shortened: true,
            }
        } else if !only_k && abs_num >= 1_000_000 {
            let val = (number as f64) / 1_000_000.0;
            ShortenedCount {
                number,
                string: Self::format_short_float(val, "M"),
                shortened: true,
            }
        } else if abs_num >= 1_000 {
            let val = (number as f64) / 1_000.0;
            ShortenedCount {
                number,
                string: Self::format_short_float(val, "K"),
                shortened: true,
            }
        } else {
            ShortenedCount {
                number,
                string: number.to_string(),
                shortened: false,
            }
        }
    }

    fn format_short_float(val: f64, suffix: &str) -> String {
        let rounded_10 = (val * 10.0).round() / 10.0;
        if (rounded_10 - rounded_10.floor()).abs() < 0.05 {
            format!("{}{suffix}", rounded_10 as i64)
        } else {
            format!("{:.1}{suffix}", rounded_10)
        }
    }

    /// Formats integer with thousands grouping separator (e.g. 1,234,567).
    /// Upstream: `FormatCountDecimal`
    pub fn format_count_decimal(number: i64) -> String {
        let s = number.abs().to_string();
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        for (i, &ch) in chars.iter().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(ch);
        }

        if number < 0 {
            format!("-{result}")
        } else {
            result
        }
    }

    /// Finds replacement placeholder position in string.
    /// Upstream: `FindTagReplacementPosition`
    pub fn find_tag_replacement_position(original: &str, tag: u16) -> Option<usize> {
        let tag_char = tag as u8;
        let bytes = original.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() {
            if bytes[i] == K_TEXT_COMMAND_LANG_TAG && bytes[i + 1] == tag_char {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// Evaluates CLDR plural rule and returns appropriate key shift (0..5).
    /// Upstream: `Plural` & `ChoosePlural*`
    pub fn plural(lang_code: &str, count: f64) -> PluralResult {
        let int_val = count.abs() as i64;
        let is_int = (count - (int_val as f64)).abs() < 1e-9;
        let shift = Self::choose_plural_shift(lang_code, count, int_val, is_int);

        PluralResult {
            key_shift: shift,
            replacement: if is_int {
                Self::format_count_decimal(int_val)
            } else {
                format!("{:.2}", count)
            },
        }
    }

    fn choose_plural_shift(lang_code: &str, count: f64, i: i64, is_int: bool) -> usize {
        let base = lang_code.split('-').next().unwrap_or(lang_code).to_ascii_lowercase();
        match base.as_str() {
            // Russian, Ukrainian, Belarusian
            "ru" | "uk" | "be" => {
                if !is_int {
                    return K_SHIFT_OTHER;
                }
                let mod10 = i % 10;
                let mod100 = i % 100;
                if mod10 == 1 && mod100 != 11 {
                    K_SHIFT_ONE
                } else if (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) {
                    K_SHIFT_FEW
                } else if mod10 == 0 || (5..=9).contains(&mod10) || (11..=14).contains(&mod100) {
                    K_SHIFT_MANY
                } else {
                    K_SHIFT_OTHER
                }
            }
            // Arabic
            "ar" => {
                if i == 0 {
                    K_SHIFT_ZERO
                } else if i == 1 {
                    K_SHIFT_ONE
                } else if i == 2 {
                    K_SHIFT_TWO
                } else if (3..=10).contains(&(i % 100)) {
                    K_SHIFT_FEW
                } else if (11..=99).contains(&(i % 100)) {
                    K_SHIFT_MANY
                } else {
                    K_SHIFT_OTHER
                }
            }
            // Polish
            "pl" => {
                if !is_int {
                    return K_SHIFT_OTHER;
                }
                if i == 1 {
                    K_SHIFT_ONE
                } else {
                    let mod10 = i % 10;
                    let mod100 = i % 100;
                    if (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) {
                        K_SHIFT_FEW
                    } else {
                        K_SHIFT_MANY
                    }
                }
            }
            // French, Brazilian Portuguese (0 and 1 are singular)
            "fr" | "pt_br" => {
                if i == 0 || i == 1 {
                    K_SHIFT_ONE
                } else {
                    K_SHIFT_OTHER
                }
            }
            // English, German, Spanish, Italian, and general Germanic/Romance
            _ => {
                if is_int && i == 1 {
                    K_SHIFT_ONE
                } else {
                    K_SHIFT_OTHER
                }
            }
        }
    }

    /// Replaces placeholder tag with replacement text.
    /// Upstream: `ReplaceTag<QString>::Call`
    pub fn replace_tag(original: &str, tag: u16, replacement: &str) -> String {
        if let Some(pos) = Self::find_tag_replacement_position(original, tag) {
            let mut result = String::with_capacity(original.len() + replacement.len());
            result.push_str(&original[..pos]);
            result.push_str(replacement);
            let end_pos = pos + 2; // placeholder token is 2 bytes (\x20 + tag)
            if end_pos < original.len() {
                result.push_str(&original[end_pos..]);
            }
            result
        } else {
            original.to_string()
        }
    }
}
