//! Authentic GPUI implementation of Telegram Saved Messages Categorized Sublists Widget.
//! Source: `Telegram/SourceFiles/info/saved/info_saved_sublists_widget.cpp:L1-L195`, `info_saved_sublists_widget.h:L1-L72`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SavedCategoryKind {
    Media,
    Files,
    Audio,
    Links,
    Voice,
    Polls,
    Gifs,
}

impl SavedCategoryKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Media => "Media",
            Self::Files => "Files",
            Self::Audio => "Audio files",
            Self::Links => "Shared links",
            Self::Voice => "Voice messages",
            Self::Polls => "Polls",
            Self::Gifs => "GIFs",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Media => "🖼",
            Self::Files => "📄",
            Self::Audio => "🎵",
            Self::Links => "🔗",
            Self::Voice => "🎙",
            Self::Polls => "📊",
            Self::Gifs => "👾",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedCategoryRow {
    pub kind: SavedCategoryKind,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct InfoSavedSublistsWidgetSaved {
    pub peer_id: u64,
    pub categories: Vec<SavedCategoryRow>,
    pub selected_kind: Option<SavedCategoryKind>,
}

impl InfoSavedSublistsWidgetSaved {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            categories: vec![
                SavedCategoryRow { kind: SavedCategoryKind::Media, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Files, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Audio, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Links, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Voice, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Polls, count: 0 },
                SavedCategoryRow { kind: SavedCategoryKind::Gifs, count: 0 },
            ],
            selected_kind: None,
        }
    }

    pub fn update_count(&mut self, kind: SavedCategoryKind, count: usize) {
        if let Some(cat) = self.categories.iter_mut().find(|c| c.kind == kind) {
            cat.count = count;
        }
    }

    pub fn select_category(&mut self, kind: Option<SavedCategoryKind>) {
        self.selected_kind = kind;
    }
}

impl Default for InfoSavedSublistsWidgetSaved {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoSavedSublistsWidgetSaved {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            .gap(px(2.0))
            .children(
                self.categories.iter().map(|cat| {
                    let is_active = self.selected_kind == Some(cat.kind);
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px(px(12.0))
                        .py(px(10.0))
                        .rounded(px(6.0))
                        .bg(if is_active { rgb(0x242f3d) } else { rgb(0x17212b) })
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(12.0))
                                .child(div().text_base().child(cat.kind.icon()))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(cat.kind.label())
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x707579))
                                .child(format!("{}", cat.count))
                        )
                })
            )
    }
}
