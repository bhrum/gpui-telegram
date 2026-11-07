//! Voice note player component for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvVoiceNotePlayer {
    pub is_playing: bool,
    pub progress: f32,
    pub duration_secs: f32,
    pub speed_2x: bool,
}

impl IvVoiceNotePlayer {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            progress: 0.0,
            duration_secs: 45.0,
            speed_2x: false,
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn toggle_speed(&mut self) {
        self.speed_2x = !self.speed_2x;
    }
}

impl Render for IvVoiceNotePlayer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = if self.is_playing { "Playing" } else { "Paused" };
        let speed = if self.speed_2x { "2x" } else { "1x" };

        div()
            .flex()
            .items_center()
            .justify_between()
            .p_2()
            .bg(rgb(0x22262a))
            .text_color(rgb(0xffffff))
            .rounded_md()
            .child(div().font_weight(FontWeight::SEMIBOLD).child("Voice Note"))
            .child(div().text_xs().child(format!("{} | Speed: {}", status, speed)))
    }
}
