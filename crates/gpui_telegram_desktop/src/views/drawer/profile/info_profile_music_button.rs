//! Info Profile Music Button component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_music_button.cpp:L1-L186`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_music_button.h:L1-L67`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Profile button displaying currently playing audio/music tracks in audio channels or profiles.
#[derive(Debug, Clone)]
pub struct InfoProfileMusicButtonProfile {
    pub track_title: String,
    pub performer: String,
    pub is_playing: bool,
    pub duration_seconds: u32,
}

impl InfoProfileMusicButtonProfile {
    pub fn new(title: impl Into<String>, performer: impl Into<String>, duration_seconds: u32) -> Self {
        Self {
            track_title: title.into(),
            performer: performer.into(),
            is_playing: false,
            duration_seconds,
        }
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }
}

impl Default for InfoProfileMusicButtonProfile {
    fn default() -> Self {
        Self::new("Track", "Artist", 0)
    }
}

impl Render for InfoProfileMusicButtonProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon = if self.is_playing { "⏸" } else { "▶" };
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .rounded_md()
            .bg(rgb(0x202b36))
            .child(div().text_sm().child(icon))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(div().text_sm().font_weight(FontWeight::BOLD).child(self.track_title.clone()))
                    .child(div().text_xs().text_color(rgb(0x708499)).child(self.performer.clone()))
            )
    }
}
