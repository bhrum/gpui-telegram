//! GPUI Telegram History Admin Log Filter Dialog/Sheet
//!
//! Citations:
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.h:L20-L32`
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.cpp:L17-L84` (FilterValueLabels category groupings)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::admin_log::{
    build_admin_log_filter_sections, AdminLogFilterFlags, AdminLogFilterSection,
    AdminLogFilterValue,
};

/// Upstream `history_admin_log_filter.h:L20-L32`: Admin Log Event Type Filter Selector
#[derive(Debug, Clone)]
pub struct HistoryAdminLogFilterAdminLog {
    pub is_channel: bool,
    pub sections: Vec<AdminLogFilterSection>,
    pub selected_flags: AdminLogFilterFlags,
    pub is_open: bool,
}

impl HistoryAdminLogFilterAdminLog {
    pub fn new(is_channel: bool) -> Self {
        let sections = build_admin_log_filter_sections(is_channel);
        Self {
            is_channel,
            sections,
            selected_flags: AdminLogFilterFlags::all(),
            is_open: false,
        }
    }

    pub fn toggle_flag(&mut self, flag: u32) {
        if self.selected_flags.contains(flag) {
            self.selected_flags.remove(flag);
        } else {
            self.selected_flags.insert(flag);
        }
    }

    pub fn select_all(&mut self) {
        self.selected_flags = AdminLogFilterFlags::all();
    }

    pub fn clear_all(&mut self) {
        self.selected_flags = AdminLogFilterFlags::new(0);
    }

    pub fn filter_value(&self) -> AdminLogFilterValue {
        AdminLogFilterValue {
            flags: Some(self.selected_flags),
            admin_user_ids: None,
        }
    }

    pub fn render_view(&self) -> impl IntoElement {
        let mut list = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .gap(px(12.0));

        for section in &self.sections {
            let mut sec_div = div().flex().flex_col().gap(px(6.0)).child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(13.0))
                    .text_color(rgb(0x2AABEE))
                    .child(section.title.clone()),
            );

            for entry in &section.entries {
                let is_checked = self.selected_flags.intersects(entry.flags);
                sec_div = sec_div.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .py(px(4.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(0x222222))
                                .child(entry.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(if is_checked {
                                    rgb(0x2AABEE)
                                } else {
                                    rgb(0xCCCCCC)
                                })
                                .child(if is_checked { "☑" } else { "☐" }),
                        ),
                );
            }
            list = list.child(sec_div);
        }

        list
    }
}

impl Default for HistoryAdminLogFilterAdminLog {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Render for HistoryAdminLogFilterAdminLog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut list = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .gap(px(12.0));

        for section in &self.sections {
            let mut sec_div = div().flex().flex_col().gap(px(6.0)).child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(13.0))
                    .text_color(rgb(0x2AABEE))
                    .child(section.title.clone()),
            );

            for entry in &section.entries {
                let is_checked = self.selected_flags.intersects(entry.flags);
                sec_div = sec_div.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .py(px(4.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(0x222222))
                                .child(entry.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(if is_checked {
                                    rgb(0x2AABEE)
                                } else {
                                    rgb(0xCCCCCC)
                                })
                                .child(if is_checked { "☑" } else { "☐" }),
                        ),
                );
            }
            list = list.child(sec_div);
        }

        list
    }
}
