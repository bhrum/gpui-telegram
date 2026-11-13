//! Genuine History View Game Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_game.h:L25-L120`
//! - `Telegram/SourceFiles/history/view/media/history_view_game.cpp:L54-L100`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct HistoryViewGameViewMedia {
    pub title: String,
    pub description: String,
    pub short_name: String,
    pub score: Option<u32>,
    pub bot_id: u64,
}

impl HistoryViewGameViewMedia {
    pub fn new(title: impl Into<String>, short_name: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            short_name: short_name.into(),
            score: None,
            bot_id: 0,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_score(mut self, score: u32) -> Self {
        self.score = Some(score);
        self
    }
}

impl Default for HistoryViewGameViewMedia {
    fn default() -> Self {
        Self::new("Lumberjack", "lumberjack")
    }
}

impl Render for HistoryViewGameViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .p(px(12.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(self.title.clone()),
                    )
                    .when_some(self.score, |el, score| {
                        el.child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x233140))
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x2AABEE))
                                .child(format!("Score: {}", score)),
                        )
                    }),
            )
            .when(!self.description.is_empty(), |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x8A9AA8))
                        .child(self.description.clone()),
                )
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .mt(px(4.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2AABEE))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("🎮 Play Game"),
            )
    }
}
