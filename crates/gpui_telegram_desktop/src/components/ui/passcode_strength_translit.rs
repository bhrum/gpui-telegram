//! Upstream C++ source: `Telegram/SourceFiles/ui/passcode_strength_translit.cpp:L1-L52` & `passcode_strength_translit.h:L1-L23`
//! Authentic GPUI implementation of Cyrillic case folding and Latin transliteration.
#![allow(dead_code)]

const K_BASIC_LOWER_FIRST: u32 = 0x0430;
const K_BASIC_LOWER: [&str; 48] = [
    "a", "b", "v", "g", "d", "e", "zh", "z",
    "i", "y", "k", "l", "m", "n", "o", "p",
    "r", "s", "t", "u", "f", "kh", "ts", "ch",
    "sh", "sch", "", "y", "", "e", "yu", "ya",
    "e", "yo", "dj", "g", "ye", "dz", "i", "yi",
    "j", "lj", "nj", "c", "k", "i", "w", "dz",
];

/// Cyrillic transliteration and normalization utilities for passcode security scoring.
/// C++ companion: `Ui::PasscodeStrengthTranslit` (`passcode_strength_translit.cpp:L25-L52`).
#[derive(Debug, Clone, Copy, Default)]
pub struct PasscodeStrengthTranslit;

impl PasscodeStrengthTranslit {
    /// Checks if a unicode code point is uppercase Cyrillic (`passcode_strength_translit.cpp:L37-L39`).
    pub fn is_uppercase_cyrillic(point: char) -> bool {
        let cp = point as u32;
        (cp >= 0x0400 && cp <= 0x042F) || (cp == 0x0490)
    }

    /// Folds Cyrillic characters to lowercase (`passcode_strength_translit.cpp:L41-L50`).
    pub fn lowercase_cyrillic(point: char) -> char {
        let cp = point as u32;
        let lowered = if cp >= 0x0400 && cp <= 0x040F {
            cp + 0x50
        } else if cp >= 0x0410 && cp <= 0x042F {
            cp + 0x20
        } else if cp == 0x0490 {
            0x0491
        } else {
            cp
        };
        char::from_u32(lowered).unwrap_or(point)
    }

    /// Transliterates lowercase Cyrillic characters to Latin representation (`passcode_strength_translit.cpp:L52-L61`).
    pub fn transliterate_cyrillic(point: char) -> Option<&'static str> {
        let cp = point as u32;
        if cp >= K_BASIC_LOWER_FIRST && (cp - K_BASIC_LOWER_FIRST) < K_BASIC_LOWER.len() as u32 {
            Some(K_BASIC_LOWER[(cp - K_BASIC_LOWER_FIRST) as usize])
        } else if cp == 0x0491 {
            Some("g")
        } else {
            None
        }
    }

    /// Transliterates an entire string from Cyrillic to Latin.
    pub fn transliterate_string(text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for c in text.chars() {
            let lowered = Self::lowercase_cyrillic(c);
            if let Some(trans) = Self::transliterate_cyrillic(lowered) {
                result.push_str(trans);
            } else {
                result.push(c);
            }
        }
        result
    }
}
