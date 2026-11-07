//! Authentic HistoryView Summary Header Component (`history_view_summary_header.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_summary_header.cpp:L1-L320`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SummaryHeaderData {
    pub title: String,
    pub summary_text: String,
    pub is_expanded: bool,
    pub has_particles: bool,
}

/// Authentic GPUI View for HistoryView::SummaryHeader.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewSummaryHeaderView {
    pub data: SummaryHeaderData,
}

impl HistoryViewSummaryHeaderView {
    pub fn new() -> Self {
        Self {
            data: SummaryHeaderData {
                title: "Thread Summary".to_string(),
                summary_text: String::new(),
                is_expanded: false,
                has_particles: true,
            },
        }
    }

    pub fn with_summary(title: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            data: SummaryHeaderData {
                title: title.into(),
                summary_text: summary.into(),
                is_expanded: false,
                has_particles: true,
            },
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.data.is_expanded = !self.data.is_expanded;
    }
}

impl Render for HistoryViewSummaryHeaderView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_color = rgb(0x8A59E7); // AI Purple accent

        div()
            .flex()
            .flex_col()
            .w_full()
            .mb(px(8.0))
            .p(px(10.0))
            .bg(rgb(0xF5F0FF))
            .border_1()
            .border_color(rgb(0xE6DBFA))
            .rounded(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(div().text_size(px(14.0)).child("✨"))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(title_color)
                            .child(self.data.title.clone()),
                    ),
            )
            .child(
                div()
                    .mt(px(4.0))
                    .text_size(px(12.0))
                    .line_height(px(17.0))
                    .text_color(rgb(0x444444))
                    .child(self.data.summary_text.clone()),
            )
    }
}
