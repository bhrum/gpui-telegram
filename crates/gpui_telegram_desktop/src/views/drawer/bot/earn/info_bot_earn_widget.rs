//! Info Bot Earn Widget GPUI Component
//! Upstream: Telegram/SourceFiles/info/bot/earn/info_bot_earn_widget.h, info_bot_earn_widget.cpp
//! Exact container widget, title provider (`lng_bot_earn_title`), and shadow visibility.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_bot_earn_list::InfoBotEarnListBotEarn;

#[derive(Debug, Clone)]
pub struct InfoBotEarnWidgetBotEarn {
    pub peer_id: u64,
    pub title: String,
    pub is_visible: bool,
    pub list: InfoBotEarnListBotEarn,
    pub shadow_visible: bool,
}

impl InfoBotEarnWidgetBotEarn {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            title: "Bot Earnings".to_string(),
            is_visible: true,
            list: InfoBotEarnListBotEarn::new(peer_id),
            shadow_visible: true,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn desired_shadow_visibility(&self) -> bool {
        self.shadow_visible
    }

    pub fn show_finished(&mut self) {
        // Trigger initial API load upon transition completion
        self.list.is_loaded = true;
    }

    pub fn set_internal_state(&mut self, available: i64, current: i64, overall: i64) {
        self.list.available_balance_stars = available;
        self.list.current_balance_stars = current;
        self.list.overall_revenue_stars = overall;
        self.list.is_loaded = true;
    }
}

impl Default for InfoBotEarnWidgetBotEarn {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoBotEarnWidgetBotEarn {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_clone = self.title.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(54.0))
                    .px(px(16.0))
                    .border_b_1()
                    .border_color(rgb(0xE4E4E7))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .child(title_clone)
                    )
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.list.clone().into_element())
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_earn_widget_lifecycle() {
        let mut widget = InfoBotEarnWidgetBotEarn::new(2002);
        assert_eq!(widget.peer_id, 2002);
        assert_eq!(widget.title, "Bot Earnings");
        assert!(widget.desired_shadow_visibility());

        widget.set_internal_state(500, 100, 600);
        assert_eq!(widget.list.available_balance_stars, 500);
        assert!(widget.list.is_loaded);
    }
}
