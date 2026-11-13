//! Video player component for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvVideoPlayer {
    pub is_playing: bool,
    pub current_time_secs: f32,
    pub total_duration_secs: f32,
    pub volume: f32,
    pub playback_rate: f32,
}

impl IvVideoPlayer {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            current_time_secs: 0.0,
            total_duration_secs: 120.0,
            volume: 1.0,
            playback_rate: 1.0,
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn seek_to(&mut self, seconds: f32) {
        self.current_time_secs = seconds.clamp(0.0, self.total_duration_secs);
    }

    pub fn set_playback_rate(&mut self, rate: f32) {
        self.playback_rate = rate.clamp(0.5, 2.0);
    }
}

impl Render for IvVideoPlayer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = if self.is_playing { "Playing" } else { "Paused" };
        let cur = self.current_time_secs as i32;
        let total = self.total_duration_secs as i32;

        div()
            .flex()
            .flex_col()
            .p_3()
            .bg(rgb(0x1a1a1a))
            .text_color(rgb(0xeeeeee))
            .rounded_md()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(div().font_weight(FontWeight::BOLD).child("Instant View Video Player"))
                    .child(div().text_sm().child(status)),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .mt_2()
                    .text_xs()
                    .child(format!("{:02}:{:02} / {:02}:{:02}", cur / 60, cur % 60, total / 60, total % 60))
                    .child(format!("Speed: {:.1}x", self.playback_rate)),
            )
    }
}
