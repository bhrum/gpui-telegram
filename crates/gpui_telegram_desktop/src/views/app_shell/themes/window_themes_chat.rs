//! WindowThemesChatThemes: Per-chat custom themes and cloud wallpaper synchronization
//! Upstream: Telegram/SourceFiles/window/themes/window_themes_chat.h, window_themes_chat.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemesChatThemes {
    pub peer_id: u64,
    pub emoticon: Option<String>,
    pub wallpaper_slug: Option<String>,
}

impl WindowThemesChatThemes {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            emoticon: None,
            wallpaper_slug: None,
        }
    }

    pub fn set_theme(&mut self, emoticon: impl Into<String>, slug: Option<String>) {
        self.emoticon = Some(emoticon.into());
        self.wallpaper_slug = slug;
    }

    pub fn reset(&mut self) {
        self.emoticon = None;
        self.wallpaper_slug = None;
    }
}

impl Default for WindowThemesChatThemes {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for WindowThemesChatThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("ChatTheme [peer={}]: emoticon={:?}", self.peer_id, self.emoticon))
    }
}
