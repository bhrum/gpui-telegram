//! Genuine History View Suggest Decision Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.h:L15-L33`
//! - `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.cpp:L28-L58`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_FADED_OPACITY: f32 = 0.85;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionEmoji {
    Agreement,
    Calendar,
    Money,
    Hourglass,
    Reload,
    Decline,
    Discard,
    Warning,
}

impl DecisionEmoji {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Agreement => "🤝",
            Self::Calendar => "📅",
            Self::Money => "💰",
            Self::Hourglass => "⌛",
            Self::Reload => "🔄",
            Self::Decline => "❌",
            Self::Discard => "🚫",
            Self::Warning => "⚠️",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryViewSuggestDecisionViewMedia {
    pub proposal_text: String,
    pub emoji_type: DecisionEmoji,
    pub is_accepted: Option<bool>,
}

impl HistoryViewSuggestDecisionViewMedia {
    pub fn new(proposal_text: impl Into<String>, emoji_type: DecisionEmoji) -> Self {
        Self {
            proposal_text: proposal_text.into(),
            emoji_type,
            is_accepted: None,
        }
    }

    pub fn accept(&mut self) {
        self.is_accepted = Some(true);
    }

    pub fn decline(&mut self) {
        self.is_accepted = Some(false);
    }
}

impl Default for HistoryViewSuggestDecisionViewMedia {
    fn default() -> Self {
        Self::new("Change meeting date to Friday?", DecisionEmoji::Calendar)
    }
}

impl Render for HistoryViewSuggestDecisionViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .p(px(12.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().text_size(px(18.0)).child(self.emoji_type.as_str()))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(self.proposal_text.clone()),
                    ),
            )
            .when(self.is_accepted.is_none(), |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex_1()
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x4CAF50))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child("Accept"),
                        )
                        .child(
                            div()
                                .flex_1()
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x233140))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xE55B5B))
                                .child("Decline"),
                        ),
                )
            })
            .when_some(self.is_accepted, |el, accepted| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(if accepted { rgb(0x4CAF50) } else { rgb(0xE55B5B) })
                        .child(if accepted { "Accepted" } else { "Declined" }),
                )
            })
    }
}
