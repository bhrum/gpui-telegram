//! macOS TouchBar Audio Player Controller
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Audio playback status for macOS TouchBar audio player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacAudioPlaybackState {
    Stopped,
    Playing,
    Paused,
}

/// Hardware audio player touchbar controller and GPUI Render element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarAudioMacTouchbar {
    pub state: MacAudioPlaybackState,
    pub title: String,
    pub artist: String,
    pub position_ms: u64,
    pub duration_ms: u64,
    pub close_requested: bool,
    pub customization_id: String,
}

impl MacTouchbarAudioMacTouchbar {
    pub const CUSTOMIZATION_ID: &'static str = "telegram.touchbar";
    pub const ITEM_PLAY: &'static str = "telegram.touchbar.play";
    pub const ITEM_PREV: &'static str = "telegram.touchbar.previousItem";
    pub const ITEM_NEXT: &'static str = "telegram.touchbar.nextItem";
    pub const ITEM_SEEKBAR: &'static str = "telegram.touchbar.seekbar";
    pub const ITEM_CURRENT_POS: &'static str = "telegram.touchbar.currentPosition";
    pub const ITEM_CLOSE: &'static str = "telegram.touchbar.closePlayer";

    pub fn new() -> Self {
        Self {
            state: MacAudioPlaybackState::Stopped,
            title: String::new(),
            artist: String::new(),
            position_ms: 0,
            duration_ms: 0,
            close_requested: false,
            customization_id: Self::CUSTOMIZATION_ID.into(),
        }
    }

    pub fn start_playback(&mut self, title: impl Into<String>, artist: impl Into<String>, duration_ms: u64) -> Result<()> {
        self.title = title.into();
        self.artist = artist.into();
        self.duration_ms = duration_ms;
        self.position_ms = 0;
        self.state = MacAudioPlaybackState::Playing;
        self.close_requested = false;
        Ok(())
    }

    pub fn toggle_play_pause(&mut self) -> Result<MacAudioPlaybackState> {
        self.state = match self.state {
            MacAudioPlaybackState::Playing => MacAudioPlaybackState::Paused,
            MacAudioPlaybackState::Paused | MacAudioPlaybackState::Stopped => MacAudioPlaybackState::Playing,
        };
        Ok(self.state)
    }

    pub fn seek_to_ratio(&mut self, ratio: f64) -> Result<u64> {
        let clamped = ratio.clamp(0.0, 1.0);
        self.position_ms = (self.duration_ms as f64 * clamped) as u64;
        Ok(self.position_ms)
    }

    pub fn request_close(&mut self) -> Result<()> {
        self.state = MacAudioPlaybackState::Stopped;
        self.close_requested = true;
        Ok(())
    }

    pub fn progress_ratio(&self) -> f64 {
        if self.duration_ms == 0 {
            0.0
        } else {
            (self.position_ms as f64 / self.duration_ms as f64).clamp(0.0, 1.0)
        }
    }
}

impl Default for MacTouchbarAudioMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacTouchbarAudioMacTouchbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_playing = self.state == MacAudioPlaybackState::Playing;
        let play_icon = if is_playing { "⏸" } else { "▶" };
        let ratio = self.progress_ratio() as f32;

        div()
            .h(px(30.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_2()
            .bg(rgb(0x1a1a1a))
            .rounded_md()
            // Previous button
            .child(div().px_2().py_1().bg(rgb(0x2f2f2f)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child("⏮"))
            // Play / Pause button
            .child(div().px_2().py_1().bg(rgb(0x2f2f2f)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child(play_icon))
            // Next button
            .child(div().px_2().py_1().bg(rgb(0x2f2f2f)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child("⏭"))
            // Progress Bar (Seekbar)
            .child(
                div()
                    .flex_1()
                    .h(px(6.0))
                    .bg(rgb(0x3a3a3a))
                    .rounded_full()
                    .relative()
                    .child(
                        div()
                            .h_full()
                            .w(px(ratio * 120.0))
                            .bg(rgb(0x40a7e3))
                            .rounded_full(),
                    ),
            )
            // Close player button
            .child(div().px_2().py_1().bg(rgb(0x2f2f2f)).rounded_sm().text_xs().text_color(rgb(0xffffff)).child("✕"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touchbar_audio_player() {
        let mut audio = MacTouchbarAudioMacTouchbar::new();
        audio.start_playback("Song 1", "Artist", 200_000).unwrap();
        assert_eq!(audio.state, MacAudioPlaybackState::Playing);

        audio.seek_to_ratio(0.25).unwrap();
        assert_eq!(audio.position_ms, 50_000);
        assert!((audio.progress_ratio() - 0.25).abs() < 1e-6);

        audio.toggle_play_pause().unwrap();
        assert_eq!(audio.state, MacAudioPlaybackState::Paused);

        audio.request_close().unwrap();
        assert!(audio.close_requested);
        assert_eq!(audio.state, MacAudioPlaybackState::Stopped);
    }
}
