//! Target Symbol: LangFileParser
//! Telegram Desktop 1:1 Submodule: `LangFileParser`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_file_parser.h`, `lang_file_parser.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use std::collections::HashMap;

/// Maximum allowable size for a custom `.strings` file: 1 MB.
/// Upstream: `constexpr auto kLangFileLimit = 1024 * 1024;`
pub const K_LANG_FILE_LIMIT: usize = 1024 * 1024;

/// Parser for Telegram Desktop `.strings` localization files.
/// Matches upstream class `Lang::FileParser`.
#[derive(Debug, Clone, Default)]
pub struct LangFileParser {
    result: HashMap<String, String>,
    errors_list: Vec<String>,
    warnings_list: Vec<String>,
    cached_errors: String,
    cached_warnings: String,
}

impl LangFileParser {
    pub fn new() -> Self {
        Self::default()
    }

    /// Strips single-line `//` and multi-line `/* ... */` comments from raw file content.
    /// Upstream: `base::parse::stripComments`
    pub fn strip_comments(input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '/' {
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        // Skip single-line comment
                        chars.next();
                        for line_ch in chars.by_ref() {
                            if line_ch == '\n' {
                                output.push('\n');
                                break;
                            }
                        }
                        continue;
                    } else if next == '*' {
                        // Skip block comment
                        chars.next();
                        let mut prev_star = false;
                        for block_ch in chars.by_ref() {
                            if prev_star && block_ch == '/' {
                                break;
                            }
                            prev_star = block_ch == '*';
                        }
                        continue;
                    }
                }
            } else if ch == '"' {
                // String literal: preserve slashes inside strings
                output.push(ch);
                let mut escaped = false;
                for str_ch in chars.by_ref() {
                    output.push(str_ch);
                    if escaped {
                        escaped = false;
                    } else if str_ch == '\\' {
                        escaped = true;
                    } else if str_ch == '"' {
                        break;
                    }
                }
                continue;
            }
            output.push(ch);
        }

        output
    }

    /// Parse translation content into key-value pairs.
    /// Upstream: `FileParser::parse()`
    pub fn parse_content(&mut self, raw_content: &str) -> bool {
        if raw_content.len() > K_LANG_FILE_LIMIT {
            self.errors_list.push(format!(
                "Lang file exceeds maximum size limit of {} bytes",
                K_LANG_FILE_LIMIT
            ));
            return false;
        }

        let stripped = Self::strip_comments(raw_content);
        let mut chars = stripped.chars().peekable();

        while chars.peek().is_some() {
            Self::skip_whitespace(&mut chars);
            if chars.peek().is_none() {
                break;
            }

            match self.read_key_value(&mut chars) {
                Ok(Some((key, value))) => {
                    self.result.insert(key, value);
                }
                Ok(None) => break,
                Err(err) => {
                    self.errors_list.push(err);
                    return false;
                }
            }
        }

        true
    }

    fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || c == ';' {
                chars.next();
            } else {
                break;
            }
        }
    }

    /// Reads `"key" = "value";`
    fn read_key_value(
        &mut self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<Option<(String, String)>, String> {
        Self::skip_whitespace(chars);
        if chars.peek().is_none() {
            return Ok(None);
        }

        // Expected opening quote for key
        if chars.next() != Some('"') {
            return Err("Expected quote before key name!".to_string());
        }

        let mut key = String::new();
        while let Some(&c) = chars.peek() {
            if (c >= 'a' && c <= 'z')
                || (c >= 'A' && c <= 'Z')
                || (c >= '0' && c <= '9')
                || c == '_'
                || c == '#'
            {
                key.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        if chars.next() != Some('"') {
            return Err(format!("Expected quote after key name '{key}'!"));
        }

        Self::skip_whitespace(chars);

        if chars.next() != Some('=') {
            return Err(format!("'=' expected after key '{key}'!"));
        }

        Self::skip_whitespace(chars);

        if chars.next() != Some('"') {
            return Err(format!("Expected quote before string value for key '{key}'!"));
        }

        let mut value = String::new();
        let mut escaped = false;
        let mut terminated = false;

        while let Some(c) = chars.next() {
            if escaped {
                match c {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    other => {
                        value.push('\\');
                        value.push(other);
                    }
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                terminated = true;
                break;
            } else {
                value.push(c);
            }
        }

        if !terminated {
            return Err(format!("Unterminated string value for key '{key}'!"));
        }

        // Optional semicolon
        Self::skip_whitespace(chars);

        Ok(Some((key, value)))
    }

    pub fn errors(&mut self) -> &str {
        if self.cached_errors.is_empty() && !self.errors_list.is_empty() {
            self.cached_errors = self.errors_list.join("\n");
        }
        &self.cached_errors
    }

    pub fn warnings(&mut self) -> &str {
        if self.cached_warnings.is_empty() && !self.warnings_list.is_empty() {
            self.cached_warnings = self.warnings_list.join("\n");
        }
        &self.cached_warnings
    }

    pub fn found(&self) -> &HashMap<String, String> {
        &self.result
    }
}
