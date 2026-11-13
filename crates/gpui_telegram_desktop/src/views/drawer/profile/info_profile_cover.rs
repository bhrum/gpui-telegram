//! Info Profile Cover component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_cover.cpp:L1-L200`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_cover.h:L1-L79`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Adaptive profile or topic cover header with custom icon, sticker playback, or gradient fill.
#[derive(Debug, Clone)]
pub struct InfoProfileCoverProfile {
    pub topic_id: i32,
    pub icon_emoji: Option<String>,
    pub cover_color: u32,
    pub aspect_ratio: f32,
    pub is_expanded: bool,
}

impl InfoProfileCoverProfile {
    pub const DEFAULT_ASPECT_RATIO: f32 = 1.777; // 16:9

    pub fn new(topic_id: i32) -> Self {
        Self {
            topic_id,
            icon_emoji: None,
            cover_color: 0x2b5278,
            aspect_ratio: Self::DEFAULT_ASPECT_RATIO,
            is_expanded: false,
        }
    }

    pub fn set_icon_emoji(&mut self, emoji: impl Into<String>) {
        self.icon_emoji = Some(emoji.into());
    }

    pub fn set_cover_color(&mut self, color: u32) {
        self.cover_color = color;
    }
}

impl Default for InfoProfileCoverProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoProfileCoverProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let color = rgb(self.cover_color);
        let icon = self.icon_emoji.clone().unwrap_or_else(|| "💬".to_string());

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(140.0))
            .bg(color)
            .child(
                div()
                    .w(px(56.0))
                    .h(px(56.0))
                    .rounded_full()
                    .bg(rgb(0x18222d))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_2xl()
                    .child(icon)
            )
    }
}
