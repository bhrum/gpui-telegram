//! GPUI Telegram History Admin Log Section View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_section.h:L36-L100`
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_section.cpp:L1-L678`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use crate::views::stage::admin_log::history_admin_log_filter::HistoryAdminLogFilterAdminLog;
use crate::views::stage::admin_log::history_admin_log_inner::HistoryAdminLogInnerAdminLog;

/// Upstream `history_admin_log_section.h:L36-L100`: Full Screen Admin Log View Section
#[derive(Debug, Clone)]
pub struct HistoryAdminLogSectionAdminLog {
    pub channel_id: i64,
    pub title: String,
    pub filter: HistoryAdminLogFilterAdminLog,
    pub inner: HistoryAdminLogInnerAdminLog,
    pub scroll_down_is_shown: bool,
}

impl HistoryAdminLogSectionAdminLog {
    pub fn new(channel_id: i64, is_channel: bool) -> Self {
        Self {
            channel_id,
            title: "Recent Actions".to_string(),
            filter: HistoryAdminLogFilterAdminLog::new(is_channel),
            inner: HistoryAdminLogInnerAdminLog::new(channel_id),
            scroll_down_is_shown: false,
        }
    }

    pub fn toggle_filter(&mut self) {
        self.filter.is_open = !self.filter.is_open;
    }

    pub fn on_scroll(&mut self, scroll_offset_y: f32) {
        self.scroll_down_is_shown = scroll_offset_y > 200.0;
    }
}

impl Default for HistoryAdminLogSectionAdminLog {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for HistoryAdminLogSectionAdminLog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xF4F4F5))
            .child(
                div()
                    .flex()
                    .h(px(52.0))
                    .px(px(16.0))
                    .items_center()
                    .justify_between()
                    .bg(rgb(0xFFFFFF))
                    .border_b_1()
                    .border_color(rgb(0xE0E0E0))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(16.0))
                            .text_color(rgb(0x222222))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .text_color(rgb(0xFFFFFF))
                            .rounded(px(6.0))
                            .text_size(px(13.0))
                            .cursor_pointer()
                            .child("Filter"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        if self.filter.is_open {
                            self.filter.render_view().into_any_element()
                        } else {
                            self.inner.render_view().into_any_element()
                        }
                    ),
            )
    }
}
