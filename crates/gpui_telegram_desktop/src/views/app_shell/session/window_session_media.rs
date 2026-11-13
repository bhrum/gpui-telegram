//! WindowSessionMediaSession: Media viewer and floating player integration for the active window session
//! Upstream: Telegram/SourceFiles/window/session/

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaPlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

#[derive(Debug, Clone)]
pub struct WindowSessionMediaSession {
    pub current_media_id: Option<u64>,
    pub playback_state: MediaPlaybackState,
    pub float_player_active: bool,
    pub fullscreen_active: bool,
    pub position_ms: u64,
    pub duration_ms: u64,
}

impl WindowSessionMediaSession {
    pub fn new() -> Self {
        Self {
            current_media_id: None,
            playback_state: MediaPlaybackState::Stopped,
            float_player_active: false,
            fullscreen_active: false,
            position_ms: 0,
            duration_ms: 0,
        }
    }

    pub fn open_media(&mut self, media_id: u64, duration_ms: u64) {
        self.current_media_id = Some(media_id);
        self.playback_state = MediaPlaybackState::Playing;
        self.position_ms = 0;
        self.duration_ms = duration_ms;
    }

    pub fn close_media(&mut self) {
        self.current_media_id = None;
        self.playback_state = MediaPlaybackState::Stopped;
        self.fullscreen_active = false;
    }

    pub fn toggle_playback(&mut self) {
        self.playback_state = match self.playback_state {
            MediaPlaybackState::Playing => MediaPlaybackState::Paused,
            MediaPlaybackState::Paused => MediaPlaybackState::Playing,
            other => other,
        };
    }

    pub fn set_float_player(&mut self, active: bool) {
        self.float_player_active = active;
    }

    pub fn set_fullscreen(&mut self, active: bool) {
        self.fullscreen_active = active;
    }
}

impl Default for WindowSessionMediaSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSessionMediaSession {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!(
                "SessionMedia: state={:?}, media={:?}, float={}",
                self.playback_state, self.current_media_id, self.float_player_active
            ))
    }
}
