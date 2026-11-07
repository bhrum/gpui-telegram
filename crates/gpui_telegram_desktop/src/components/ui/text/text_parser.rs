//! Authentic GPUI Implementation: Text & Markdown Entity Parser
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`

use super::text_entity::{TextEntitySpan, TextEntityType, resolve_entity_overlaps};
use super::text_options::*;

/// Parsed rich text containing clean string and validated entity spans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedRichText {
    pub text: String,
    pub entities: Vec<TextEntitySpan>,
}

/// Parses plain text and markdown formatting into clean text and entities.
pub fn parse_text_entities(raw_text: &str, options: &TextParseOptions) -> ParsedRichText {
    let mut clean_text = String::new();
    let mut entities = Vec::new();
    let chars: Vec<char> = raw_text.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        // Spoiler check: ||spoiler||
        if options.has_flag(TEXT_PARSE_MARKDOWN) && i + 1 < n && chars[i] == '|' && chars[i + 1] == '|' {
            if let Some(close_idx) = find_closing_marker(&chars, i + 2, "||") {
                let start_offset = clean_text.len();
                let inner: String = chars[i + 2..close_idx].iter().collect();
                clean_text.push_str(&inner);
                entities.push(TextEntitySpan::new(
                    TextEntityType::Spoiler,
                    start_offset,
                    inner.len(),
                ));
                i = close_idx + 2;
                continue;
            }
        }

        // Bold check: **bold**
        if options.has_flag(TEXT_PARSE_MARKDOWN) && i + 1 < n && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(close_idx) = find_closing_marker(&chars, i + 2, "**") {
                let start_offset = clean_text.len();
                let inner: String = chars[i + 2..close_idx].iter().collect();
                clean_text.push_str(&inner);
                entities.push(TextEntitySpan::new(
                    TextEntityType::Bold,
                    start_offset,
                    inner.len(),
                ));
                i = close_idx + 2;
                continue;
            }
        }

        // Monospace check: `code`
        if options.has_flag(TEXT_PARSE_MARKDOWN) && chars[i] == '`' {
            if let Some(close_idx) = find_closing_char(&chars, i + 1, '`') {
                let start_offset = clean_text.len();
                let inner: String = chars[i + 1..close_idx].iter().collect();
                clean_text.push_str(&inner);
                entities.push(TextEntitySpan::new(
                    TextEntityType::Code,
                    start_offset,
                    inner.len(),
                ));
                i = close_idx + 1;
                continue;
            }
        }

        // Mention check: @username
        if options.has_flag(TEXT_PARSE_MENTIONS) && chars[i] == '@' {
            let start_offset = clean_text.len();
            let mut j = i + 1;
            while j < n && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            if j > i + 1 {
                let token: String = chars[i..j].iter().collect();
                clean_text.push_str(&token);
                entities.push(TextEntitySpan::new(
                    TextEntityType::Mention,
                    start_offset,
                    token.len(),
                ));
                i = j;
                continue;
            }
        }

        // Hashtag check: #hashtag
        if options.has_flag(TEXT_PARSE_HASHTAGS) && chars[i] == '#' {
            let start_offset = clean_text.len();
            let mut j = i + 1;
            while j < n && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            if j > i + 1 {
                let token: String = chars[i..j].iter().collect();
                clean_text.push_str(&token);
                entities.push(TextEntitySpan::new(
                    TextEntityType::Hashtag,
                    start_offset,
                    token.len(),
                ));
                i = j;
                continue;
            }
        }

        // URL check: http:// or https://
        if options.has_flag(TEXT_PARSE_LINKS) && is_url_start(&chars, i) {
            let start_offset = clean_text.len();
            let mut j = i;
            while j < n && !chars[j].is_whitespace() && !matches!(chars[j], ')' | ']' | '}' | '>' | '"' | '\'') {
                j += 1;
            }
            let url: String = chars[i..j].iter().collect();
            clean_text.push_str(&url);
            entities.push(TextEntitySpan::new(
                TextEntityType::Url,
                start_offset,
                url.len(),
            ).with_data(url.clone()));
            i = j;
            continue;
        }

        clean_text.push(chars[i]);
        i += 1;
    }

    let resolved = resolve_entity_overlaps(&entities);
    ParsedRichText {
        text: clean_text,
        entities: resolved,
    }
}

fn find_closing_marker(chars: &[char], start: usize, marker: &str) -> Option<usize> {
    let m_chars: Vec<char> = marker.chars().collect();
    let m_len = m_chars.len();
    if start + m_len > chars.len() {
        return None;
    }
    for idx in start..=chars.len() - m_len {
        if chars[idx..idx + m_len] == m_chars[..] {
            return Some(idx);
        }
    }
    None
}

fn find_closing_char(chars: &[char], start: usize, marker: char) -> Option<usize> {
    for idx in start..chars.len() {
        if chars[idx] == marker {
            return Some(idx);
        }
    }
    None
}

fn is_url_start(chars: &[char], idx: usize) -> bool {
    let s: String = chars.iter().skip(idx).take(8).collect();
    s.starts_with("http://") || s.starts_with("https://")
}
