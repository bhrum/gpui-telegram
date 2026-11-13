//! Authentic GPUI Implementation: Text Word Wrapping & Line-Breaking Heuristics
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`

/// A word token with measured geometry and trailing whitespace flag.
#[derive(Debug, Clone, PartialEq)]
pub struct TextWord {
    pub text: String,
    pub offset: usize,
    pub length: usize,
    pub width: f32,
    pub has_trailing_space: bool,
    pub is_newline: bool,
}

/// A laid-out line composed of word tokens.
#[derive(Debug, Clone, PartialEq)]
pub struct TextLine {
    pub words: Vec<TextWord>,
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
}

impl TextLine {
    pub fn new(y_offset: f32, height: f32) -> Self {
        Self {
            words: Vec::new(),
            width: 0.0,
            height,
            y_offset,
        }
    }

    pub fn text(&self) -> String {
        let mut s = String::new();
        for (i, w) in self.words.iter().enumerate() {
            s.push_str(&w.text);
            if w.has_trailing_space && i + 1 < self.words.len() {
                s.push(' ');
            }
        }
        s
    }
}

/// Splits input text into word tokens using character metrics.
pub fn tokenize_words(text: &str, avg_char_width: f32) -> Vec<TextWord> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut word_offset = 0;
    let mut in_word = false;

    for (byte_idx, ch) in text.char_indices() {
        if ch == '\n' {
            if in_word {
                let len = current_word.len();
                let width = (current_word.chars().count() as f32) * avg_char_width;
                words.push(TextWord {
                    text: std::mem::take(&mut current_word),
                    offset: word_offset,
                    length: len,
                    width,
                    has_trailing_space: false,
                    is_newline: false,
                });
                in_word = false;
            }
            words.push(TextWord {
                text: "\n".to_string(),
                offset: byte_idx,
                length: 1,
                width: 0.0,
                has_trailing_space: false,
                is_newline: true,
            });
        } else if ch.is_whitespace() {
            if in_word {
                let len = current_word.len();
                let width = (current_word.chars().count() as f32) * avg_char_width;
                words.push(TextWord {
                    text: std::mem::take(&mut current_word),
                    offset: word_offset,
                    length: len,
                    width,
                    has_trailing_space: true,
                    is_newline: false,
                });
                in_word = false;
            }
        } else {
            if !in_word {
                word_offset = byte_idx;
                in_word = true;
            }
            current_word.push(ch);
        }
    }

    if in_word {
        let len = current_word.len();
        let width = (current_word.chars().count() as f32) * avg_char_width;
        words.push(TextWord {
            text: current_word,
            offset: word_offset,
            length: len,
            width,
            has_trailing_space: false,
            is_newline: false,
        });
    }

    words
}

/// Wraps word tokens into lines according to available width and line height.
pub fn wrap_words_into_lines(
    words: &[TextWord],
    available_width: f32,
    line_height: f32,
    space_width: f32,
) -> Vec<TextLine> {
    let mut lines = Vec::new();
    let mut current_line = TextLine::new(0.0, line_height);
    let mut current_y = 0.0;

    for word in words {
        if word.is_newline {
            lines.push(current_line);
            current_y += line_height;
            current_line = TextLine::new(current_y, line_height);
            continue;
        }

        let added_width = if current_line.words.is_empty() {
            word.width
        } else {
            space_width + word.width
        };

        if !current_line.words.is_empty() && current_line.width + added_width > available_width {
            lines.push(current_line);
            current_y += line_height;
            current_line = TextLine::new(current_y, line_height);
            current_line.width = word.width;
            current_line.words.push(word.clone());
        } else {
            current_line.width += added_width;
            current_line.words.push(word.clone());
        }
    }

    if !current_line.words.is_empty() || lines.is_empty() {
        lines.push(current_line);
    }

    lines
}
