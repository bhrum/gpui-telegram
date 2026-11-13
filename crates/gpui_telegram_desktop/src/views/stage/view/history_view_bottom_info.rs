//! Authentic HistoryView Bottom Info Component (`history_view_bottom_info.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_bottom_info.cpp:L40-L774`

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::bottom_info::{
    format_views_count, schedule_period_text, BottomInfoData, BottomInfoFlags, BottomInfoView,
};

/// Authentic GPUI View for HistoryView::BottomInfo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewBottomInfoView {
    pub info: BottomInfoData,
}

impl HistoryViewBottomInfoView {
    pub fn new() -> Self {
        Self {
            info: BottomInfoData::default(),
        }
    }

    pub fn with_date(date: i64, is_outgoing: bool) -> Self {
        let mut flags = BottomInfoFlags::new();
        if is_outgoing {
            flags.insert(BottomInfoFlags::OUT_LAYOUT);
        }
        Self {
            info: BottomInfoData {
                date,
                edited_date: None,
                author: None,
                flags,
                views: None,
                replies: None,
                forwards_count: None,
            },
        }
    }

    pub fn schedule_text(&self, period: i64) -> &'static str {
        schedule_period_text(period)
    }

    pub fn formatted_views(&self, views: i32) -> String {
        format_views_count(views)
    }
}

impl Default for HistoryViewBottomInfoView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewBottomInfoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.info.render())
    }
}
