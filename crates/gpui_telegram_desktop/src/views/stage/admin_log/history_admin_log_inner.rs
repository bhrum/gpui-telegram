//! GPUI Telegram History Admin Log Inner Widget & Event Scroller
//!
//! Citations:
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.h:L61-L100`
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.cpp:L89-L103` (Pagination and grouping constants)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::admin_log::{
    AdminLogDeleteGroup, K_DELETE_GROUP_TIME_WINDOW_SECS, K_EVENTS_FIRST_PAGE,
    K_EVENTS_PER_PAGE, K_MAX_CHANNEL_ADMINS, K_PRELOAD_UP_DISPLAY_ITEMS_BUFFER,
};

/// Upstream `history_admin_log_inner.h:L61-L100`: Inner Administrative Log Virtualized List
#[derive(Debug, Clone)]
pub struct HistoryAdminLogInnerAdminLog {
    pub channel_id: i64,
    pub loaded_event_count: usize,
    pub delete_groups: Vec<AdminLogDeleteGroup>,
    pub loaded_at_top: bool,
    pub loaded_at_bottom: bool,
    pub is_loading: bool,
}

impl HistoryAdminLogInnerAdminLog {
    pub fn new(channel_id: i64) -> Self {
        Self {
            channel_id,
            loaded_event_count: 0,
            delete_groups: Vec::new(),
            loaded_at_top: false,
            loaded_at_bottom: true,
            is_loading: false,
        }
    }

    /// Upstream `kEventsPerPage = 100`: Paginates events
    pub fn load_next_page(&mut self, new_events_count: usize) {
        self.loaded_event_count = self.loaded_event_count.saturating_add(new_events_count);
        if new_events_count < K_EVENTS_PER_PAGE {
            self.loaded_at_top = true;
        }
    }

    /// Upstream `kDeleteGroupTimeWindowSeconds = 5`: Coalesces consecutive message deletions
    pub fn record_message_deleted(
        &mut self,
        event_id: u64,
        admin_id: i64,
        date: i64,
        msg_id: i64,
    ) {
        if let Some(last_group) = self.delete_groups.last_mut() {
            if last_group.try_coalesce(admin_id, date, msg_id) {
                return;
            }
        }
        self.delete_groups.push(AdminLogDeleteGroup::new(
            event_id, admin_id, date, msg_id,
        ));
    }

    pub fn render_view(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x888888))
                    .child(format!(
                        "Admin log: {} events loaded across {} delete groups",
                        self.loaded_event_count,
                        self.delete_groups.len()
                    )),
            )
    }
}

impl Default for HistoryAdminLogInnerAdminLog {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for HistoryAdminLogInnerAdminLog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x888888))
                    .child(format!(
                        "Admin log: {} events loaded across {} delete groups",
                        self.loaded_event_count,
                        self.delete_groups.len()
                    )),
            )
    }
}
