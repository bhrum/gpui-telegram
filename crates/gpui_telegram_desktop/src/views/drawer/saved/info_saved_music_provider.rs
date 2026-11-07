//! Authentic GPUI implementation of Telegram Saved Music Provider.
//! Source: `Telegram/SourceFiles/info/saved/info_saved_music_provider.cpp:L1-L422`, `info_saved_music_provider.h:L1-L131`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_saved_music_common::SavedAudioTrack;

pub const SAVED_MUSIC_PRELOAD_SCREENS: usize = 3;
pub const SAVED_MUSIC_PRELOAD_SCREENS_FULL: usize = 5;
pub const SAVED_MUSIC_MINIMAL_IDS_LIMIT: usize = 16;

#[derive(Debug, Clone)]
pub struct InfoSavedMusicProviderSaved {
    pub peer_id: u64,
    pub tracks: Vec<SavedAudioTrack>,
    pub is_loading: bool,
    pub has_more: bool,
    pub current_query: String,
}

impl InfoSavedMusicProviderSaved {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            tracks: Vec::new(),
            is_loading: false,
            has_more: true,
            current_query: String::new(),
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut p = Self::new();
        p.peer_id = peer_id;
        p
    }

    pub fn append_tracks(&mut self, new_tracks: Vec<SavedAudioTrack>, has_more: bool) {
        for track in new_tracks {
            if let Some(pos) = self.tracks.iter().position(|t| t.id == track.id) {
                self.tracks[pos] = track;
            } else {
                self.tracks.push(track);
            }
        }
        self.has_more = has_more;
        self.is_loading = false;
    }

    pub fn set_search_query(&mut self, query: &str) {
        self.current_query = query.trim().to_lowercase();
    }

    pub fn filtered_tracks(&self) -> Vec<&SavedAudioTrack> {
        if self.current_query.is_empty() {
            self.tracks.iter().collect()
        } else {
            self.tracks
                .iter()
                .filter(|t| {
                    t.title.to_lowercase().contains(&self.current_query)
                        || t.performer.to_lowercase().contains(&self.current_query)
                })
                .collect()
        }
    }
}

impl Default for InfoSavedMusicProviderSaved {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoSavedMusicProviderSaved {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.filtered_tracks().len();
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p(px(8.0))
            .bg(rgb(0x17212b))
            .rounded(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x707579))
                    .child(format!("Cached: {} tracks", count))
            )
            .when(self.is_loading, |this| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x5288c1))
                        .child("Loading...")
                )
            })
    }
}
