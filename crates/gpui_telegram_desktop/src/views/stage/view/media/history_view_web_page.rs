//! Genuine History View Web Page Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_web_page.h:L25-L229`
//! - `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp:L56-L200`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::web_page::{
    WebPageType, RichWebPageCard, FactcheckInfo,
    article_thumb_width, article_thumb_height, page_to_phrase,
    K_MAX_ORIGINAL_ENTRY_LINES, K_FACTCHECK_COLLAPSED_LINES,
    K_STICKER_SET_LINES, K_FACTCHECK_ABOUT_DURATION_MS,
};

#[derive(Debug, Clone)]
pub struct HistoryViewWebPageViewMedia {
    pub url: String,
    pub title: String,
    pub description: String,
    pub site_name: Option<String>,
    pub page_type: WebPageType,
    pub has_instant_view: bool,
    pub is_expanded: bool,
    pub factcheck: Option<FactcheckInfo>,
}

impl HistoryViewWebPageViewMedia {
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            description: String::new(),
            site_name: None,
            page_type: WebPageType::Article,
            has_instant_view: false,
            is_expanded: false,
            factcheck: None,
        }
    }

    pub fn with_site(mut self, site: impl Into<String>) -> Self {
        self.site_name = Some(site.into());
        self
    }

    pub fn with_iv(mut self, iv: bool) -> Self {
        self.has_instant_view = iv;
        self
    }

    pub fn set_factcheck(&mut self, text: impl Into<String>, country: impl Into<String>) {
        self.factcheck = Some(FactcheckInfo {
            text: text.into(),
            country: country.into(),
            is_expanded: false,
        });
    }

    pub fn toggle_factcheck(&mut self) {
        if let Some(fc) = self.factcheck.as_mut() {
            fc.is_expanded = !fc.is_expanded;
        }
    }
}

impl Default for HistoryViewWebPageViewMedia {
    fn default() -> Self {
        Self::new("https://telegram.org", "Telegram Messenger")
    }
}

impl Render for HistoryViewWebPageViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichWebPageCard::new(&self.url, &self.title);
        card.site_name = self.site_name.clone();
        card.description = self.description.clone();
        card.page_type = self.page_type;
        card.has_instant_view = self.has_instant_view;
        card.factcheck = self.factcheck.clone();
        card.render()
    }
}
