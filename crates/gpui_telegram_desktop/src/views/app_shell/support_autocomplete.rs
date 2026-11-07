//! SupportAutocomplete: Support ticket canned response macro autocompletion engine
//! Upstream: Telegram/SourceFiles/window/support_autocomplete.h, support_autocomplete.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportMacro {
    pub tag: String,
    pub title: String,
    pub template_text: String,
}

#[derive(Debug, Clone)]
pub struct SupportAutocomplete {
    pub macros: Vec<SupportMacro>,
    pub current_query: String,
}

impl SupportAutocomplete {
    pub fn new() -> Self {
        Self {
            macros: vec![
                SupportMacro {
                    tag: "#welcome".to_string(),
                    title: "Welcome Greeting".to_string(),
                    template_text: "Hello! How can I assist you today with Telegram?".to_string(),
                },
                SupportMacro {
                    tag: "#reset_2fa".to_string(),
                    title: "Two-Step Verification Reset".to_string(),
                    template_text: "To reset your two-step verification password, please follow the steps in Settings > Privacy.".to_string(),
                },
                SupportMacro {
                    tag: "#phone".to_string(),
                    title: "Phone Number Change".to_string(),
                    template_text: "You can change your phone number directly in Settings > Edit Profile > Change Number.".to_string(),
                },
            ],
            current_query: String::new(),
        }
    }

    pub fn query_matches(&self, query: &str) -> Vec<&SupportMacro> {
        let q = query.to_lowercase();
        self.macros
            .iter()
            .filter(|m| m.tag.to_lowercase().contains(&q) || m.title.to_lowercase().contains(&q))
            .collect()
    }

    pub fn add_macro(&mut self, tag: impl Into<String>, title: impl Into<String>, text: impl Into<String>) {
        self.macros.push(SupportMacro {
            tag: tag.into(),
            title: title.into(),
            template_text: text.into(),
        });
    }
}

impl Default for SupportAutocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SupportAutocomplete {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SupportAutocomplete: {} macros loaded", self.macros.len()))
    }
}
