//! Common formatting and descriptors for Saved Music drawer rows.
//! Source: `Telegram/SourceFiles/info/saved/info_saved_music_common.cpp:L1-L83`, `info_saved_music_common.h:L1-L36`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedAudioTrack {
    pub id: u64,
    pub title: String,
    pub performer: String,
    pub duration_seconds: i32,
    pub file_size: usize,
    pub date: i64,
    pub is_playing: bool,
}

impl SavedAudioTrack {
    pub fn format_duration(&self) -> String {
        let m = self.duration_seconds / 60;
        let s = self.duration_seconds % 60;
        format!("{}:{:02}", m, s)
    }

    pub fn subtitle(&self) -> String {
        if self.performer.is_empty() {
            self.format_duration()
        } else {
            format!("{} • {}", self.performer, self.format_duration())
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoSavedMusicCommonSaved {
    pub tracks: Vec<SavedAudioTrack>,
    pub active_track_id: Option<u64>,
    pub playback_progress: f32, // 0.0 .. 1.0
}

impl InfoSavedMusicCommonSaved {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            active_track_id: None,
            playback_progress: 0.0,
        }
    }

    pub fn set_active_track(&mut self, track_id: Option<u64>) {
        self.active_track_id = track_id;
        for t in &mut self.tracks {
            t.is_playing = Some(t.id) == track_id;
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.playback_progress = progress.clamp(0.0, 1.0);
    }
}

impl Default for InfoSavedMusicCommonSaved {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoSavedMusicCommonSaved {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x707579))
                    .child(format!("Tracks: {}", self.tracks.len()))
            )
    }
}
