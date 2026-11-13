//! Genuine History View Contact Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_contact.h:L25-L99`
//! - `Telegram/SourceFiles/history/view/media/history_view_contact.cpp:L39-L100`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::location::RichContactCard;

#[derive(Debug, Clone)]
pub struct HistoryViewContactViewMedia {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_id: Option<u64>,
    pub vcard_raw: Option<String>,
}

impl HistoryViewContactViewMedia {
    pub fn new(first_name: impl Into<String>, phone: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: String::new(),
            phone_number: phone.into(),
            user_id: None,
            vcard_raw: None,
        }
    }

    pub fn with_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = last_name.into();
        self
    }

    pub fn full_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

impl Default for HistoryViewContactViewMedia {
    fn default() -> Self {
        Self::new("Telegram", "+1234567890")
    }
}

impl Render for HistoryViewContactViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichContactCard::new(&self.first_name, &self.phone_number);
        card.last_name = self.last_name.clone();
        card.user_id = self.user_id;
        card.render()
    }
}
