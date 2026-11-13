//! WindowNoPaidToken: Insufficient credits and Stars balance warning dialog
//! Upstream: Telegram/SourceFiles/window/window_no_paid_token.h, window_no_paid_token.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowNoPaidToken {
    pub is_visible: bool,
    pub bot_username: String,
    pub required_stars: u64,
    pub current_stars: u64,
}

impl WindowNoPaidToken {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            bot_username: String::new(),
            required_stars: 0,
            current_stars: 0,
        }
    }

    pub fn prompt_insufficient_stars(&mut self, bot: impl Into<String>, required: u64, current: u64) {
        self.bot_username = bot.into();
        self.required_stars = required;
        self.current_stars = current;
        self.is_visible = true;
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }

    pub fn stars_needed(&self) -> u64 {
        self.required_stars.saturating_sub(self.current_stars)
    }
}

impl Default for WindowNoPaidToken {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowNoPaidToken {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(20.0))
            .bg(rgb(0x17212B))
            .rounded(px(8.0))
            .child(div().font_weight(FontWeight::BOLD).text_color(rgb(0xFFD700)).child("Insufficient Stars Balance"))
            .child(format!(
                "Need {} more Telegram Stars to message @{}",
                self.stars_needed(),
                self.bot_username
            ))
    }
}
