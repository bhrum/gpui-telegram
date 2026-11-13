//! Authentic GPUI Implementation: Text Block Layout & Geometry
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Reference: `Telegram/SourceFiles/tests/test_text.cpp:L300-L356`

/// Text block classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBlockKind {
    Text,
    Emoji,
    CustomEmoji,
    Newline,
    Skip,
}

/// A contiguous visual span of formatted text or inline object.
#[derive(Debug, Clone, PartialEq)]
pub struct TextBlock {
    pub kind: TextBlockKind,
    pub text: String,
    pub offset: usize,
    pub length: usize,
    pub width: f32,
    pub height: f32,
    pub entity_data: Option<String>,
}

impl TextBlock {
    pub fn new_text(text: impl Into<String>, offset: usize, width: f32, height: f32) -> Self {
        let text = text.into();
        let length = text.len();
        Self {
            kind: TextBlockKind::Text,
            text,
            offset,
            length,
            width,
            height,
            entity_data: None,
        }
    }

    pub fn new_emoji(glyph: impl Into<String>, offset: usize, size: f32) -> Self {
        let text = glyph.into();
        let length = text.len();
        Self {
            kind: TextBlockKind::Emoji,
            text,
            offset,
            length,
            width: size,
            height: size,
            entity_data: None,
        }
    }

    pub fn new_custom_emoji(entity_data: impl Into<String>, offset: usize, size: f32) -> Self {
        let data = entity_data.into();
        Self {
            kind: TextBlockKind::CustomEmoji,
            text: "\u{FFFC}".to_string(), // Object replacement char
            offset,
            length: 1,
            width: size,
            height: size,
            entity_data: Some(data),
        }
    }

    pub fn new_newline(offset: usize, line_height: f32) -> Self {
        Self {
            kind: TextBlockKind::Newline,
            text: "\n".to_string(),
            offset,
            length: 1,
            width: 0.0,
            height: line_height,
            entity_data: None,
        }
    }

    pub fn is_newline(&self) -> bool {
        self.kind == TextBlockKind::Newline
    }
}
