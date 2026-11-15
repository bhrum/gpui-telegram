//! Shared text input, UTF-16 offset conversion, word deletion, clipboard sanitization,
//! and IME preedit controller helpers used across GPUI desktop input views.

use std::ops::Range;

/// Returns the number of UTF-16 code units in `s`.
pub fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// Converts a UTF-16 code unit offset in `s` into the corresponding UTF-8 byte offset.
pub fn utf16_offset_to_utf8(s: &str, utf16_offset: usize) -> usize {
    let mut current_utf16 = 0;
    for (byte_idx, ch) in s.char_indices() {
        if current_utf16 >= utf16_offset {
            return byte_idx;
        }
        current_utf16 += ch.len_utf16();
    }
    s.len()
}

/// Removes the trailing Unicode scalar character from `s`, returning `true` if `s` changed.
pub fn pop_last_char(s: &mut String) -> bool {
    s.pop().is_some()
}

/// Deletes the previous word (skipping trailing whitespace, then contiguous word or punctuation
/// characters) from `s`, returning `true` if `s` changed.
pub fn delete_prev_word(s: &mut String) -> bool {
    if s.is_empty() {
        return false;
    }
    let orig_len = s.len();
    while s.chars().next_back().is_some_and(char::is_whitespace) {
        s.pop();
    }
    if let Some(last) = s.chars().next_back() {
        let is_word = |c: char| c.is_alphanumeric() || c == '_';
        if is_word(last) {
            while s.chars().next_back().is_some_and(is_word) {
                s.pop();
            }
        } else {
            while s
                .chars()
                .next_back()
                .is_some_and(|c| !c.is_whitespace() && !is_word(c))
            {
                s.pop();
            }
        }
    }
    s.len() != orig_len
}

/// Normalizes and sanitizes pasted or IME-committed text, stripping unsafe control characters
/// and optionally stripping or normalizing line breaks for single-line fields.
pub fn sanitize_paste_text(raw: &str, allow_newlines: bool) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\r' => {
                if allow_newlines {
                    if chars.peek() == Some(&'\n') {
                        let _ = chars.next();
                    }
                    out.push('\n');
                }
            }
            '\n' => {
                if allow_newlines {
                    out.push('\n');
                }
            }
            '\t' => {
                out.push(if allow_newlines { '\t' } else { ' ' });
            }
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// Tracks active IME marked composition text and post-commit Enter-key absorption state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TextInputController {
    pub ime_preedit: Option<String>,
    pub just_committed_ime: bool,
}

impl TextInputController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_preedit(&self) -> bool {
        self.ime_preedit.as_ref().is_some_and(|p| !p.is_empty())
    }

    pub fn clear_preedit(&mut self) {
        self.ime_preedit = None;
    }

    pub fn take_just_committed(&mut self) -> bool {
        std::mem::take(&mut self.just_committed_ime)
    }

    pub fn text_and_preedit(&self, committed: String) -> (String, Option<String>) {
        (committed, self.ime_preedit.clone())
    }

    pub fn selected_utf16_range(&self, committed: &str) -> Option<Range<usize>> {
        let total_utf16 = utf16_len(committed) + self.ime_preedit.as_deref().map_or(0, utf16_len);
        Some(total_utf16..total_utf16)
    }

    pub fn marked_utf16_range(&self, committed: &str) -> Option<Range<usize>> {
        let preedit = self.ime_preedit.as_deref().filter(|p| !p.is_empty())?;
        let start = utf16_len(committed);
        let end = start + utf16_len(preedit);
        Some(start..end)
    }

    /// Processes `replace_and_mark_text_in_range`: sets `ime_preedit` and returns `None` when
    /// `new_text` is non-empty, or commits and clears any prior preedit when `new_text` is empty.
    pub fn handle_replace_and_mark(&mut self, new_text: &str) -> Option<String> {
        if new_text.is_empty() {
            let committed = self.ime_preedit.take().filter(|p| !p.is_empty());
            if committed.is_some() {
                self.just_committed_ime = true;
            }
            committed
        } else {
            self.ime_preedit = Some(new_text.to_string());
            self.just_committed_ime = false;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf16_conversion_with_cjk_and_emoji() {
        let s = "hi你😀!";
        assert_eq!(utf16_len(s), 6);
        assert_eq!(utf16_offset_to_utf8(s, 0), 0);
        assert_eq!(utf16_offset_to_utf8(s, 2), 2);
        assert_eq!(utf16_offset_to_utf8(s, 3), 5);
        assert_eq!(utf16_offset_to_utf8(s, 5), 9);
        assert_eq!(utf16_offset_to_utf8(s, 99), s.len());
    }

    #[test]
    fn test_pop_and_delete_prev_word_and_sanitize() {
        let mut s = "hello world   ".to_string();
        assert!(delete_prev_word(&mut s));
        assert_eq!(s, "hello ");
        assert!(pop_last_char(&mut s));
        assert_eq!(s, "hello");
        assert!(delete_prev_word(&mut s));
        assert_eq!(s, "");
        assert!(!delete_prev_word(&mut s));

        assert_eq!(sanitize_paste_text("a\r\nb\0c\t", false), "abc ");
        assert_eq!(sanitize_paste_text("a\r\nb\rc\0\t", true), "a\nb\nc\t");
    }

    #[test]
    fn test_text_input_controller_ime_lifecycle() {
        let mut ctrl = TextInputController::default();
        assert!(!ctrl.has_preedit());
        assert_eq!(ctrl.selected_utf16_range("abc"), Some(3..3));
        assert_eq!(ctrl.marked_utf16_range("abc"), None);

        assert_eq!(ctrl.handle_replace_and_mark("nihao"), None);
        assert!(ctrl.has_preedit());
        assert_eq!(ctrl.selected_utf16_range("abc"), Some(8..8));
        assert_eq!(ctrl.marked_utf16_range("abc"), Some(3..8));

        assert_eq!(ctrl.handle_replace_and_mark(""), Some("nihao".to_string()));
        assert!(!ctrl.has_preedit());
        assert!(ctrl.take_just_committed());
        assert!(!ctrl.take_just_committed());
    }
}
