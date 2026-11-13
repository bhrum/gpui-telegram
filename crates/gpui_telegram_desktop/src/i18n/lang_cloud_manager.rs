//! Target Symbol: LangCloudManager
//! Telegram Desktop 1:1 Submodule: `LangCloudManager`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_cloud_manager.h`, `lang_cloud_manager.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use std::collections::HashMap;
use crate::i18n::lang_instance::{LangInstance, Language};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudLangPackLanguage {
    pub lang_code: String,
    pub name: String,
    pub native_name: String,
    pub plural_code: String,
    pub base_lang_code: String,
    pub strings_count: usize,
    pub translated_count: usize,
    pub translations_url: String,
    pub is_official: bool,
}

impl CloudLangPackLanguage {
    /// Computes percentage of completed translations.
    /// Upstream: `_percent(data.vtranslated_count().v * 100 / data.vstrings_count().v)`
    pub fn translation_percent(&self) -> usize {
        if self.strings_count == 0 {
            0
        } else {
            (self.translated_count * 100) / self.strings_count
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageSwitchDisposition {
    ReadyImmediate,
    WarningIncomplete { percent: usize, edit_link: String },
    NotReady { percent: usize, edit_link: String },
}

/// Cloud language pack synchronization and switching manager.
/// Matches upstream class `Lang::CloudManager`.
#[derive(Debug, Clone, Default)]
pub struct LangCloudManager {
    languages: Vec<CloudLangPackLanguage>,
    suggested_language: Option<String>,
    current_pack_version: i32,
    base_pack_version: i32,
}

impl LangCloudManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_suggested_language(&mut self, code: impl Into<String>) {
        self.suggested_language = Some(code.into());
    }

    pub fn suggested_language(&self) -> Option<&str> {
        self.suggested_language.as_deref()
    }

    pub fn set_languages(&mut self, langs: Vec<CloudLangPackLanguage>) {
        self.languages = langs;
    }

    pub fn languages(&self) -> &[CloudLangPackLanguage] {
        &self.languages
    }

    pub fn set_current_versions(&mut self, version: i32, base_version: i32) {
        self.current_pack_version = version;
        self.base_pack_version = base_version;
    }

    /// Evaluates translation completeness warning dialog requirements.
    /// Upstream: `CloudManager::switchWithWarning`
    pub fn evaluate_switch_warning(&self, lang: &CloudLangPackLanguage) -> LanguageSwitchDisposition {
        let percent = lang.translation_percent();
        if percent < 50 {
            LanguageSwitchDisposition::NotReady {
                percent,
                edit_link: lang.translations_url.clone(),
            }
        } else if percent < 100 {
            LanguageSwitchDisposition::WarningIncomplete {
                percent,
                edit_link: lang.translations_url.clone(),
            }
        } else {
            LanguageSwitchDisposition::ReadyImmediate
        }
    }

    /// Executes language switch into target `LangInstance`.
    /// Upstream: `CloudManager::switchToLanguage`
    pub fn switch_to_language(&mut self, instance: &mut LangInstance, lang: &CloudLangPackLanguage) {
        instance.switch_to_id(Language {
            id: lang.lang_code.clone(),
            plural_id: if lang.plural_code.is_empty() {
                lang.lang_code.clone()
            } else {
                lang.plural_code.clone()
            },
            base_id: lang.base_lang_code.clone(),
            name: lang.name.clone(),
            native_name: lang.native_name.clone(),
        });
    }

    /// Resets language to English default.
    /// Upstream: `CloudManager::resetToDefault`
    pub fn reset_to_default(&mut self, instance: &mut LangInstance) {
        instance.switch_to_id(Language {
            id: "en".to_string(),
            plural_id: "en".to_string(),
            base_id: String::new(),
            name: "English".to_string(),
            native_name: "English".to_string(),
        });
    }

    /// Applies cloud language difference deltas to target instance.
    /// Upstream: `CloudManager::applyLangPackDifference`
    pub fn apply_difference(
        &mut self,
        instance: &mut LangInstance,
        new_version: i32,
        strings: HashMap<String, String>,
    ) {
        instance.apply_difference(new_version, strings);
        self.current_pack_version = new_version;
    }
}
