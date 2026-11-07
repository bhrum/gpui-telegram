//! Target Symbol: LangInstance
//! Telegram Desktop 1:1 Submodule: `LangInstance`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_instance.h`, `lang_instance.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use std::collections::HashMap;
use crate::i18n::lang_file_parser::LangFileParser;

pub const K_SERIALIZE_VERSION_TAG: &str = "#new";
pub const K_SERIALIZE_VERSION: i32 = 1;
pub const K_CLOUD_LANG_PACK_NAME: &str = "tdesktop";
pub const K_CUSTOM_LANGUAGE: &str = "#custom";
pub const K_LANG_VALUES_LIMIT: usize = 20000;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Language {
    pub id: String,
    pub plural_id: String,
    pub base_id: String,
    pub name: String,
    pub native_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pack {
    None,
    Current,
    Base,
}

/// Active language instance maintaining the loaded string dictionary, versioning, and base fallback.
/// Matches upstream class `Lang::Instance`.
#[derive(Debug, Clone)]
pub struct LangInstance {
    id: String,
    plural_id: String,
    base_id: String,
    name: String,
    native_name: String,
    version: i32,
    is_custom: bool,
    values: HashMap<String, String>,
    base: Option<Box<LangInstance>>,
}

impl Default for LangInstance {
    fn default() -> Self {
        Self {
            id: "en".to_string(),
            plural_id: "en".to_string(),
            base_id: String::new(),
            name: "English".to_string(),
            native_name: "English".to_string(),
            version: 0,
            is_custom: false,
            values: HashMap::new(),
            base: None,
        }
    }
}

impl LangInstance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn native_name(&self) -> &str {
        &self.native_name
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn is_custom(&self) -> bool {
        self.is_custom
    }

    /// Switches active language to specified metadata.
    /// Upstream: `Instance::switchToId`
    pub fn switch_to_id(&mut self, language: Language) {
        self.id = language.id;
        self.plural_id = language.plural_id;
        self.base_id = language.base_id;
        self.name = language.name;
        self.native_name = language.native_name;
        self.is_custom = false;
        self.values.clear();
        self.version = 0;
    }

    /// Loads custom `.strings` file content.
    /// Upstream: `Instance::switchToCustomFile`
    pub fn switch_to_custom_content(&mut self, content: &str) -> bool {
        let mut parser = LangFileParser::new();
        if !parser.parse_content(content) {
            return false;
        }

        self.id = K_CUSTOM_LANGUAGE.to_string();
        self.name = "Custom".to_string();
        self.native_name = "Custom".to_string();
        self.is_custom = true;
        self.version = 0;
        self.values = parser.found().clone();
        true
    }

    /// Look up string by key, cascading to base fallback if present.
    /// Upstream: `Instance::getValue`
    pub fn get_value(&self, key: &str) -> Option<&str> {
        if let Some(val) = self.values.get(key) {
            return Some(val.as_str());
        }
        if let Some(ref base) = self.base {
            return base.get_value(key);
        }
        None
    }

    /// Applies cloud language pack delta difference.
    /// Upstream: `Instance::applyDifference`
    pub fn apply_difference(&mut self, new_version: i32, deltas: HashMap<String, String>) {
        if new_version >= self.version {
            self.version = new_version;
            for (k, v) in deltas {
                if v.is_empty() {
                    self.values.remove(&k);
                } else if self.values.len() < K_LANG_VALUES_LIMIT {
                    self.values.insert(k, v);
                }
            }
        }
    }

    /// Serializes instance state to binary buffer with version tag.
    /// Upstream: `Instance::serialize`
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(K_SERIALIZE_VERSION_TAG.as_bytes());
        buffer.push(b'\n');
        buffer.extend_from_slice(self.id.as_bytes());
        buffer.push(b'\n');
        buffer.extend_from_slice(self.version.to_string().as_bytes());
        buffer.push(b'\n');
        buffer
    }

    /// Deserializes instance state.
    /// Upstream: `Instance::fillFromSerialized`
    pub fn fill_from_serialized(&mut self, data: &[u8]) -> bool {
        let text = match std::str::from_utf8(data) {
            Ok(t) => t,
            Err(_) => return false,
        };
        let mut lines = text.lines();
        if lines.next() != Some(K_SERIALIZE_VERSION_TAG) {
            return false;
        }
        if let Some(id_line) = lines.next() {
            self.id = id_line.to_string();
        }
        if let Some(ver_line) = lines.next() {
            self.version = ver_line.parse().unwrap_or(0);
        }
        true
    }
}
