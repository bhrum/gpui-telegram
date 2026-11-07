//! Authentic GPUI Interface & Translation Language Picker Component.
//! Reference: Telegram/SourceFiles/ui/boxes/choose_language_box.cpp:L25-L383
//! Reference: Telegram/SourceFiles/ui/boxes/choose_language_box.h:L20-L36

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    pub id: String,
    pub english_name: String,
    pub native_name: String,
    pub is_selected: bool,
}

/// Target symbol for language chooser box.
/// Reference: `choose_language_box.h:L25-L36`
#[derive(Debug, Clone)]
pub struct ChooseLanguageBoxBoxes {
    pub languages: Vec<LanguageEntry>,
    pub search_query: String,
    pub is_multiselect: bool,
    pub selected_ids: Vec<String>,
}

impl Default for ChooseLanguageBoxBoxes {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ChooseLanguageBoxBoxes {
    pub fn new(multiselect: bool) -> Self {
        let defaults = vec![
            LanguageEntry {
                id: "en".to_string(),
                english_name: "English".to_string(),
                native_name: "English".to_string(),
                is_selected: true,
            },
            LanguageEntry {
                id: "ru".to_string(),
                english_name: "Russian".to_string(),
                native_name: "Русский".to_string(),
                is_selected: false,
            },
            LanguageEntry {
                id: "zh".to_string(),
                english_name: "Chinese".to_string(),
                native_name: "中文".to_string(),
                is_selected: false,
            },
            LanguageEntry {
                id: "es".to_string(),
                english_name: "Spanish".to_string(),
                native_name: "Español".to_string(),
                is_selected: false,
            },
            LanguageEntry {
                id: "de".to_string(),
                english_name: "German".to_string(),
                native_name: "Deutsch".to_string(),
                is_selected: false,
            },
        ];
        Self {
            languages: defaults,
            search_query: String::new(),
            is_multiselect: multiselect,
            selected_ids: vec!["en".to_string()],
        }
    }

    /// Filters languages by query against English name, native name, or language code.
    /// Reference: `choose_language_box.cpp:L160-L180`
    pub fn filtered_languages(&self) -> Vec<&LanguageEntry> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.languages.iter().collect()
        } else {
            self.languages
                .iter()
                .filter(|l| {
                    l.english_name.to_lowercase().contains(&q)
                        || l.native_name.to_lowercase().contains(&q)
                        || l.id.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn toggle_language(&mut self, id: &str) {
        if self.is_multiselect {
            if let Some(pos) = self.selected_ids.iter().position(|x| x == id) {
                self.selected_ids.remove(pos);
            } else {
                self.selected_ids.push(id.to_string());
            }
        } else {
            self.selected_ids = vec![id.to_string()];
        }
        for l in &mut self.languages {
            l.is_selected = self.selected_ids.contains(&l.id);
        }
    }
}
