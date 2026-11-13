//! Info Profile Balance Pill component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_balance_pill.cpp:L1-L286`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_balance_pill.h:L1-L59`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Telegram Stars balance pill displayed on channel and bot profile headers.
#[derive(Debug, Clone)]
pub struct InfoProfileBalancePillProfile {
    pub balance: u64,
    pub is_loading: bool,
}

impl InfoProfileBalancePillProfile {
    pub fn new(balance: u64) -> Self {
        Self {
            balance,
            is_loading: false,
        }
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }
}

impl Default for InfoProfileBalancePillProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoProfileBalancePillProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = format!("⭐ {}", self.balance);

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded_full()
            .bg(rgb(0x3a2c18))
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0xf5a623))
            .child(text)
    }
}
