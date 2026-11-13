//! Target Symbol: LangTextEntity
//! Telegram Desktop 1:1 Submodule: `LangTextEntity`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_text_entity.h`, `lang_text_entity.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::i18n::lang_tag::{LangTag, K_TAG_REPLACEMENT_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Bold,
    Semibold,
    Italic,
    Underline,
    StrikeOut,
    Code,
    Pre,
    CustomUrl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityInText {
    pub entity_type: EntityType,
    pub offset: usize,
    pub length: usize,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TextWithEntities {
    pub text: String,
    pub entities: Vec<EntityInText>,
}

impl TextWithEntities {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            entities: Vec::new(),
        }
    }

    pub fn with_entity(mut self, entity: EntityInText) -> Self {
        self.entities.push(entity);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangTextEntity;

impl LangTextEntity {
    /// Replaces placeholder tag in rich formatted text while adjusting entity offset coordinates.
    /// Upstream: `ReplaceTag<TextWithEntities>::Replace`
    pub fn replace_tag(
        original: TextWithEntities,
        tag: u16,
        replacement: &TextWithEntities,
    ) -> TextWithEntities {
        let pos = match LangTag::find_tag_replacement_position(&original.text, tag) {
            Some(p) => p,
            None => return original,
        };

        let tag_token_len = 2; // placeholder tag token (\x20 + tag_byte)
        let mut new_text = String::with_capacity(original.text.len() + replacement.text.len());
        new_text.push_str(&original.text[..pos]);
        new_text.push_str(&replacement.text);
        if pos + tag_token_len < original.text.len() {
            new_text.push_str(&original.text[pos + tag_token_len..]);
        }

        let mut new_entities = Vec::new();
        let delta = (replacement.text.len() as isize) - (tag_token_len as isize);

        // Adjust original entities
        for entity in original.entities {
            if entity.offset >= pos + tag_token_len {
                // Entity is strictly after replacement
                let new_offset = ((entity.offset as isize) + delta).max(0) as usize;
                new_entities.push(EntityInText {
                    offset: new_offset,
                    ..entity
                });
            } else if entity.offset + entity.length <= pos {
                // Entity is strictly before replacement
                new_entities.push(entity);
            } else {
                // Entity overlaps replacement point: expand span
                let new_len = ((entity.length as isize) + delta).max(1) as usize;
                new_entities.push(EntityInText {
                    length: new_len,
                    ..entity
                });
            }
        }

        // Add spliced replacement entities
        for rentity in &replacement.entities {
            new_entities.push(EntityInText {
                entity_type: rentity.entity_type,
                offset: pos + rentity.offset,
                length: rentity.length,
                data: rentity.data.clone(),
            });
        }

        new_entities.sort_by_key(|e| e.offset);

        TextWithEntities {
            text: new_text,
            entities: new_entities,
        }
    }

    /// Wraps text in bold entity.
    /// Upstream: `tr::bold`
    pub fn bold(text: impl Into<String>) -> TextWithEntities {
        let s = text.into();
        let len = s.len();
        TextWithEntities {
            text: s,
            entities: vec![EntityInText {
                entity_type: EntityType::Bold,
                offset: 0,
                length: len,
                data: String::new(),
            }],
        }
    }

    /// Wraps text in italic entity.
    /// Upstream: `tr::italic`
    pub fn italic(text: impl Into<String>) -> TextWithEntities {
        let s = text.into();
        let len = s.len();
        TextWithEntities {
            text: s,
            entities: vec![EntityInText {
                entity_type: EntityType::Italic,
                offset: 0,
                length: len,
                data: String::new(),
            }],
        }
    }

    /// Wraps text in custom link entity.
    /// Upstream: `tr::link` / `tr::url`
    pub fn link(text: impl Into<String>, url: impl Into<String>) -> TextWithEntities {
        let s = text.into();
        let len = s.len();
        TextWithEntities {
            text: s,
            entities: vec![EntityInText {
                entity_type: EntityType::CustomUrl,
                offset: 0,
                length: len,
                data: url.into(),
            }],
        }
    }

    /// Converts text to uppercase while preserving entities.
    /// Upstream: `tr::upper`
    pub fn upper(mut input: TextWithEntities) -> TextWithEntities {
        input.text = input.text.to_uppercase();
        input
    }
}
