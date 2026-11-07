//! MenuMute: Submenu providing standard notification mute durations (1h, 8h, 2d, Forever)
//! Upstream: Telegram/SourceFiles/menu/menu_mute.h, menu_mute.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuteDuration {
    Hours1,
    Hours8,
    Days2,
    Forever,
    Unmute,
}

impl MuteDuration {
    pub fn to_seconds(self) -> u32 {
        match self {
            MuteDuration::Hours1 => 3600,
            MuteDuration::Hours8 => 8 * 3600,
            MuteDuration::Days2 => 2 * 86400,
            MuteDuration::Forever => u32::MAX,
            MuteDuration::Unmute => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuMute {
    pub peer_id: u64,
    pub is_muted: bool,
}

impl MenuMute {
    pub fn new(peer_id: u64, is_muted: bool) -> Self {
        Self { peer_id, is_muted }
    }

    pub fn options(&self) -> Vec<MuteDuration> {
        if self.is_muted {
            vec![MuteDuration::Unmute]
        } else {
            vec![
                MuteDuration::Hours1,
                MuteDuration::Hours8,
                MuteDuration::Days2,
                MuteDuration::Forever,
            ]
        }
    }
}

impl Default for MenuMute {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for MenuMute {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(if self.is_muted { "Unmute Notifications" } else { "Mute Notifications..." })
    }
}
