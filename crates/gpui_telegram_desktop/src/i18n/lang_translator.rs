//! Target Symbol: LangTranslator
//! Telegram Desktop 1:1 Submodule: `LangTranslator`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_translator.h`, `lang_translator.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

/// Translates native OS platform menus and dialog commands to Telegram localized strings.
/// Matches upstream class `Lang::Translator : public QTranslator`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangTranslator;

impl LangTranslator {
    /// Translates OS menu items according to context and platform.
    /// Upstream: `Translator::translate(context, sourceText, disambiguation, n)`
    pub fn translate(context: &str, source_text: &str, app_name: &str) -> Option<String> {
        match context {
            "QMenuBar" => match source_text {
                "Services" => Some("Services".to_string()),
                "Hide %1" => Some(format!("Hide {app_name}")),
                "Hide Others" => Some("Hide Others".to_string()),
                "Show All" => Some("Show All".to_string()),
                "Preferences..." => Some("Preferences...".to_string()),
                "Quit %1" => Some(format!("Quit {app_name}")),
                "About %1" => Some(format!("About {app_name}")),
                _ => None,
            },
            "QCocoaMenuItem" => match source_text {
                "Cut" => Some("Cut".to_string()),
                "Copy" => Some("Copy".to_string()),
                "Paste" => Some("Paste".to_string()),
                "Select All" => Some("Select All".to_string()),
                _ => None,
            },
            "QCocoaMenuBar" => match source_text {
                "Edit" => Some("Edit".to_string()),
                _ => None,
            },
            "QWidgetTextControl" | "QLineEdit" => match source_text {
                "&Undo" | "Undo" => Some("Undo".to_string()),
                "&Redo" | "Redo" => Some("Redo".to_string()),
                "Cu&t" | "Cut" => Some("Cut".to_string()),
                "&Copy" | "Copy" => Some("Copy".to_string()),
                "&Paste" | "Paste" => Some("Paste".to_string()),
                "Delete" => Some("Delete".to_string()),
                "Select All" => Some("Select All".to_string()),
                _ => None,
            },
            "QUnicodeControlCharacterMenu" => match source_text {
                "Insert Unicode control character" => {
                    Some("Insert Unicode control character".to_string())
                }
                _ => None,
            },
            _ => None,
        }
    }
}
