//! Authentic GPUI Typographic Font Chooser Component.
//! Reference: Telegram/SourceFiles/ui/boxes/choose_font_box.cpp:L40-L965
//! Reference: Telegram/SourceFiles/ui/boxes/choose_font_box.h:L12-L20

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Minimum horizontal width for live preview text: 120.
/// Reference: `choose_font_box.cpp:L45`
pub const K_MIN_TEXT_WIDTH: usize = 120;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFamilyEntry {
    pub family: String,
    pub is_monospace: bool,
    pub is_custom: bool,
}

/// Target symbol for font selection and preview modal box.
/// Reference: `choose_font_box.h:L12-L20`
#[derive(Debug, Clone)]
pub struct ChooseFontBoxBoxes {
    pub families: Vec<FontFamilyEntry>,
    pub selected_family: String,
    pub preview_text: String,
    pub font_size: f32,
    pub is_saved: bool,
}

impl Default for ChooseFontBoxBoxes {
    fn default() -> Self {
        Self::new("Inter")
    }
}

impl ChooseFontBoxBoxes {
    pub fn new(current_family: impl Into<String>) -> Self {
        let initial = current_family.into();
        Self {
            families: vec![
                FontFamilyEntry {
                    family: "Inter".to_string(),
                    is_monospace: false,
                    is_custom: false,
                },
                FontFamilyEntry {
                    family: "SF Pro Text".to_string(),
                    is_monospace: false,
                    is_custom: false,
                },
                FontFamilyEntry {
                    family: "JetBrains Mono".to_string(),
                    is_monospace: true,
                    is_custom: true,
                },
                FontFamilyEntry {
                    family: "Fira Code".to_string(),
                    is_monospace: true,
                    is_custom: true,
                },
            ],
            selected_family: initial,
            preview_text: "The quick brown fox jumps over the lazy dog 0123456789".to_string(),
            font_size: 13.0,
            is_saved: false,
        }
    }

    pub fn select_family(&mut self, family: &str) {
        if self.families.iter().any(|f| f.family == family) {
            self.selected_family = family.to_string();
        }
    }

    pub fn save(&mut self) -> String {
        self.is_saved = true;
        self.selected_family.clone()
    }
}
