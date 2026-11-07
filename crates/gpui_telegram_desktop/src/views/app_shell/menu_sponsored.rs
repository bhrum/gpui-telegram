//! MenuSponsored: Context actions for sponsored channel messages (About ads, Report, Hide with Premium)
//! Upstream: Telegram/SourceFiles/menu/menu_sponsored.h, menu_sponsored.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuSponsored {
    pub message_id: i64,
    pub is_premium: bool,
}

impl MenuSponsored {
    pub fn new(message_id: i64, is_premium: bool) -> Self {
        Self { message_id, is_premium }
    }

    pub fn menu_actions(&self) -> Vec<&'static str> {
        let mut actions = vec!["What are sponsored messages?", "Report this ad"];
        if !self.is_premium {
            actions.push("Hide ads with Telegram Premium");
        }
        actions
    }
}

impl Default for MenuSponsored {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for MenuSponsored {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SponsoredMenu for msg {}", self.message_id))
    }
}
