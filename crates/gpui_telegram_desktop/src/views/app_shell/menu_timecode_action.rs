//! MenuTimecodeAction: Timestamp link copy action for audio and video media items
//! Upstream: Telegram/SourceFiles/menu/menu_timecode_action.h, menu_timecode_action.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuTimecodeAction {
    pub timecode_secs: u32,
    pub base_url: String,
}

impl MenuTimecodeAction {
    pub fn new(timecode_secs: u32, base_url: impl Into<String>) -> Self {
        Self {
            timecode_secs,
            base_url: base_url.into(),
        }
    }

    pub fn formatted_timecode(&self) -> String {
        let mins = self.timecode_secs / 60;
        let secs = self.timecode_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn timecode_link(&self) -> String {
        format!("{}?t={}", self.base_url, self.timecode_secs)
    }
}

impl Default for MenuTimecodeAction {
    fn default() -> Self {
        Self::new(0, "https://t.me/example/1")
    }
}

impl Render for MenuTimecodeAction {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(format!("Copy link at {}", self.formatted_timecode()))
    }
}
