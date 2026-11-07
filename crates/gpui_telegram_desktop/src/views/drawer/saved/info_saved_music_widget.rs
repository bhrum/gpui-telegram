//! Authentic GPUI implementation of Telegram Saved Music Drawer Tab Widget.
//! Source: `Telegram/SourceFiles/info/saved/info_saved_music_widget.cpp:L1-L353`, `info_saved_music_widget.h:L1-L79`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_saved_music_common::{InfoSavedMusicCommonSaved, SavedAudioTrack};
use super::info_saved_music_provider::InfoSavedMusicProviderSaved;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedMusicMemento {
    pub scroll_top: f32,
    pub playing_track_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct InfoSavedMusicWidgetSaved {
    pub peer_id: u64,
    pub provider: InfoSavedMusicProviderSaved,
    pub common: InfoSavedMusicCommonSaved,
    pub search_query: String,
    pub scroll_top: f32,
}

impl InfoSavedMusicWidgetSaved {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            provider: InfoSavedMusicProviderSaved::new(),
            common: InfoSavedMusicCommonSaved::new(),
            search_query: String::new(),
            scroll_top: 0.0,
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut w = Self::new();
        w.peer_id = peer_id;
        w.provider.peer_id = peer_id;
        w
    }

    pub fn toggle_play(&mut self, track_id: u64) {
        if self.common.active_track_id == Some(track_id) {
            self.common.set_active_track(None);
        } else {
            self.common.set_active_track(Some(track_id));
        }
    }

    pub fn set_search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.provider.set_search_query(query);
    }
}

impl Default for InfoSavedMusicWidgetSaved {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoSavedMusicWidgetSaved {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let tracks = self.provider.filtered_tracks();
        let is_empty = tracks.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            .gap(px(8.0))
            // Search Input Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(8.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x242f3d))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child("🔍 Search saved music...")
                    )
            )
            // Empty State
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(40.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No saved music found")
                )
            })
            // Audio rows list
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(2.0));
                for track in tracks {
                    let is_active = self.common.active_track_id == Some(track.id);
                    col = col.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_active { rgb(0x242f3d) } else { rgb(0x17212b) })
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .w(px(32.0))
                                            .h(px(32.0))
                                            .rounded_full()
                                            .bg(if is_active { rgb(0x5288c1) } else { rgb(0x2b5278) })
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xs()
                                            .text_color(rgb(0xffffff))
                                            .child(if is_active { "⏸" } else { "▶" })
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(track.title.clone())
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x707579))
                                                    .child(track.subtitle())
                                            )
                                    )
                            )
                    );
                }
                this.child(col)
            })
    }
}
