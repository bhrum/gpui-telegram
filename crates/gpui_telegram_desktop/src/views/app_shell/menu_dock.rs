//! MenuDock: OS dock and system tray integration menu (media playback, muting, window toggling)
//! Upstream: Telegram/SourceFiles/menu/menu_dock.h, menu_dock.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuDock {
    pub is_playing_audio: bool,
    pub notifications_enabled: bool,
}

impl MenuDock {
    pub fn new() -> Self {
        Self {
            is_playing_audio: false,
            notifications_enabled: true,
        }
    }

    pub fn toggle_player(&mut self) -> bool {
        self.is_playing_audio = !self.is_playing_audio;
        self.is_playing_audio
    }

    pub fn toggle_notifications(&mut self) -> bool {
        self.notifications_enabled = !self.notifications_enabled;
        self.notifications_enabled
    }
}

impl Default for MenuDock {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuDock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("DockMenu: playing={}, notifs={}", self.is_playing_audio, self.notifications_enabled))
    }
}
