//! GPUI Telegram History Admin Log Filter Value Model
//!
//! Citations:
//! - `Telegram/SourceFiles/history/admin_log/history_admin_log_filter_value.h:L10-L54`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::admin_log::{AdminLogFilterFlags, AdminLogFilterValue};

/// Upstream `history_admin_log_filter_value.h:L12-L44`: Admin Log Filter Value Wrapper
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HistoryAdminLogFilterValueAdminLog {
    pub inner: AdminLogFilterValue,
}

impl HistoryAdminLogFilterValueAdminLog {
    pub fn new() -> Self {
        Self {
            inner: AdminLogFilterValue::all(),
        }
    }

    pub fn with_flags(mut self, flags: AdminLogFilterFlags) -> Self {
        self.inner.flags = Some(flags);
        self
    }

    pub fn with_admins(mut self, admin_ids: Vec<i64>) -> Self {
        self.inner.admin_user_ids = Some(admin_ids);
        self
    }

    pub fn matches_event(&self, event_flag: u32, admin_id: i64) -> bool {
        self.inner.matches_event(event_flag, admin_id)
    }
}

impl Render for HistoryAdminLogFilterValueAdminLog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
