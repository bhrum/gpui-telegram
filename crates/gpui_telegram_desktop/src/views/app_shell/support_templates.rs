//! SupportTemplates: Categorized answer template database and search index for support
//! Upstream: Telegram/SourceFiles/window/support_templates.h, support_templates.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    General,
    AccountRecovery,
    Security,
    Billing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateItem {
    pub id: u32,
    pub category: TemplateCategory,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SupportTemplates {
    pub templates: Vec<TemplateItem>,
}

impl SupportTemplates {
    pub fn new() -> Self {
        Self {
            templates: vec![
                TemplateItem {
                    id: 1,
                    category: TemplateCategory::General,
                    title: "Welcome".to_string(),
                    content: "Hello! Thank you for contacting Telegram Support.".to_string(),
                },
                TemplateItem {
                    id: 2,
                    category: TemplateCategory::AccountRecovery,
                    title: "SMS Code Delivery".to_string(),
                    content: "If you cannot receive the login SMS code, please check if you have an active Telegram session on another device.".to_string(),
                },
                TemplateItem {
                    id: 3,
                    category: TemplateCategory::Security,
                    title: "Passcode Forgotten".to_string(),
                    content: "If you forgot your local passcode, you can reinstall the application to reset it.".to_string(),
                },
            ],
        }
    }

    pub fn search(&self, query: &str) -> Vec<&TemplateItem> {
        let q = query.to_lowercase();
        self.templates
            .iter()
            .filter(|t| t.title.to_lowercase().contains(&q) || t.content.to_lowercase().contains(&q))
            .collect()
    }

    pub fn by_category(&self, category: TemplateCategory) -> Vec<&TemplateItem> {
        self.templates
            .iter()
            .filter(|t| t.category == category)
            .collect()
    }
}

impl Default for SupportTemplates {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SupportTemplates {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SupportTemplates: {} entries", self.templates.len()))
    }
}
