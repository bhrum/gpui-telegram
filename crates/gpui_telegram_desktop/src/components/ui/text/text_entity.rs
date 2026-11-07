//! Authentic GPUI Implementation: Text Entity Formatting & Range Overlap Resolution
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Reference: `Telegram/SourceFiles/tests/test_text.cpp:L38-L47`

use serde::{Deserialize, Serialize};

/// Upstream EntityType variants supported in Telegram messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextEntityType {
    Bold,
    Semibold,
    Italic,
    Underline,
    StrikeOut,
    Code,
    Pre,
    Blockquote,
    Spoiler,
    Url,
    CustomUrl,
    Mention,
    Hashtag,
    BotCommand,
    CustomEmoji,
}

/// An entity span within a text string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEntitySpan {
    pub entity_type: TextEntityType,
    pub offset: usize,
    pub length: usize,
    pub data: Option<String>,
}

impl TextEntitySpan {
    pub fn new(entity_type: TextEntityType, offset: usize, length: usize) -> Self {
        Self {
            entity_type,
            offset,
            length,
            data: None,
        }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn end_offset(&self) -> usize {
        self.offset + self.length
    }
}

/// Resolves overlapping entity ranges according to Telegram nesting rules.
/// Entities are ordered by offset ascending; ties broken by length descending.
/// Any invalid partial-overlap cross is truncated or rejected.
pub fn resolve_entity_overlaps(entities: &[TextEntitySpan]) -> Vec<TextEntitySpan> {
    let mut sorted = entities.to_vec();
    sorted.sort_by(|a, b| {
        a.offset
            .cmp(&b.offset)
            .then_with(|| b.length.cmp(&a.length))
    });

    let mut result: Vec<TextEntitySpan> = Vec::new();
    for entity in sorted {
        if entity.length == 0 {
            continue;
        }
        // Verify non-conflicting boundary against active parent stack
        let mut valid = true;
        for existing in &result {
            let existing_end = existing.end_offset();
            let entity_end = entity.end_offset();

            // Partial overlap condition: starts inside existing but extends beyond existing
            if entity.offset > existing.offset
                && entity.offset < existing_end
                && entity_end > existing_end
            {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(entity);
        }
    }
    result
}

/// Checks if an entity list contains a specific entity type.
/// Citing `Telegram/SourceFiles/tests/test_text.cpp:L38-L47`
pub fn has_entity_type(entities: &[TextEntitySpan], entity_type: TextEntityType) -> bool {
    entities.iter().any(|e| e.entity_type == entity_type)
}
