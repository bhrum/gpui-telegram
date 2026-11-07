//! Authentic GPUI Implementation: Isolated Emoji Detection & Sizing Rules
//!
//! Subsystem: `ui/text` | Feature: `TCG-N15`

/// Isolated emoji display scale configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsolatedEmojiConfig {
    pub count: usize,
    pub scale: f32,
    pub pixel_size: f32,
}

/// Checks if a unicode scalar represents an emoji character.
pub fn is_emoji_char(ch: char) -> bool {
    let u = ch as u32;
    // Standard Unicode emoji blocks:
    (0x1F600..=0x1F64F).contains(&u) // Emoticons
        || (0x1F300..=0x1F5FF).contains(&u) // Misc Symbols and Pictographs
        || (0x1F680..=0x1F6FF).contains(&u) // Transport and Map
        || (0x1F700..=0x1F77F).contains(&u) // Alchemical
        || (0x1F780..=0x1F7FF).contains(&u) // Geometric Shapes Extended
        || (0x1F800..=0x1F8FF).contains(&u) // Supplemental Arrows-C
        || (0x1F900..=0x1F9FF).contains(&u) // Supplemental Symbols and Pictographs
        || (0x1FA00..=0x1FA6F).contains(&u) // Chess Symbols
        || (0x1FA70..=0x1FAFF).contains(&u) // Symbols and Pictographs Extended-A
        || (0x2600..=0x26FF).contains(&u)   // Misc symbols
        || (0x2700..=0x27BF).contains(&u)   // Dingbats
        || (0x2B00..=0x2BFF).contains(&u)   // Misc Symbols & Arrows (e.g. ⭐ \u{2B50})
        || (0x2300..=0x23FF).contains(&u)   // Misc Technical
        || (0x25A0..=0x25FF).contains(&u)   // Geometric Shapes
        || (0x2934..=0x2935).contains(&u)
        || (0x3297..=0x3299).contains(&u)
        || (0x3030..=0x303D).contains(&u)
        || (0x203C..=0x2049).contains(&u)
        || u == 0xFE0F                      // Variation selector 16
        || u == 0x200D                      // Zero-width joiner
}

/// Evaluates if message text consists entirely of 1 to 3 isolated emoji characters.
/// If true, returns the upscale configuration (3x for single emoji, 2x for 2-3 emoji).
pub fn check_isolated_emoji(text: &str, base_font_size: f32) -> Option<IsolatedEmojiConfig> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut emoji_count = 0;
    for ch in trimmed.chars() {
        if ch == '\u{FE0F}' || ch == '\u{200D}' {
            // Modifiers/joiners don't increment top-level emoji count
            continue;
        }
        if is_emoji_char(ch) {
            emoji_count += 1;
        } else if !ch.is_whitespace() {
            // Contains regular textual character -> not isolated emoji
            return None;
        }
    }

    if emoji_count == 1 {
        Some(IsolatedEmojiConfig {
            count: 1,
            scale: 3.0,
            pixel_size: base_font_size * 3.0,
        })
    } else if (2..=3).contains(&emoji_count) {
        Some(IsolatedEmojiConfig {
            count: emoji_count,
            scale: 2.0,
            pixel_size: base_font_size * 2.0,
        })
    } else {
        None
    }
}
