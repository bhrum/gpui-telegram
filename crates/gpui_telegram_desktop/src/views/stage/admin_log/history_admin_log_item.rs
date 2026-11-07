//! GPUI Telegram History Admin Log Item Event Row
//!
//! Citations:
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_item.h:L21-L57`
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_item.cpp:L63-L82`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Upstream `history_admin_log_item.h:L28-L57`: Admin Log Event Item View Row
#[derive(Debug, Clone)]
pub struct HistoryAdminLogItemAdminLog {
    pub event_id: u64,
    pub admin_id: i64,
    pub date: i64,
    pub action_title: String,
    pub details: String,
}

impl HistoryAdminLogItemAdminLog {
    pub fn new(event_id: u64, admin_id: i64, date: i64, action_title: impl Into<String>) -> Self {
        Self {
            event_id,
            admin_id,
            date,
            action_title: action_title.into(),
            details: String::new(),
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = details.into();
        self
    }
}

impl Default for HistoryAdminLogItemAdminLog {
    fn default() -> Self {
        Self::new(0, 0, 0, "")
    }
}

impl Render for HistoryAdminLogItemAdminLog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(10.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xEEEEEE))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(13.0))
                            .text_color(rgb(0x222222))
                            .child(self.action_title.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Admin #{}", self.admin_id)),
                    ),
            )
            .child(
                div()
                    .mt(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x555555))
                    .child(self.details.clone()),
            )
    }
}
