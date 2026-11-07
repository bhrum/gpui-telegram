//! Channel & Supergroup Statistics Outer Drawer Widget
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_widget.cpp:L1-L160`
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_widget.h:L1-L69`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_statistics_inner_widget::InfoStatisticsInnerWidgetStatistics;
use super::info_statistics_export::InfoStatisticsExportStatistics;

#[derive(Debug, Clone)]
pub struct InfoStatisticsWidgetStatistics {
    pub peer_id: u64,
    pub title: String,
    pub inner: InfoStatisticsInnerWidgetStatistics,
    pub export_ctrl: InfoStatisticsExportStatistics,
    pub show_export_dialog: bool,
    pub is_visible: bool,
}

impl InfoStatisticsWidgetStatistics {
    pub fn new(peer_id: u64, title: impl Into<String>) -> Self {
        let t = title.into();
        Self {
            peer_id,
            title: t.clone(),
            inner: InfoStatisticsInnerWidgetStatistics::new(),
            export_ctrl: InfoStatisticsExportStatistics::new(t),
            show_export_dialog: false,
            is_visible: true,
        }
    }

    pub fn toggle_export_dialog(&mut self) {
        self.show_export_dialog = !self.show_export_dialog;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoStatisticsWidgetStatistics {
    fn default() -> Self {
        Self::new(0, "Statistics")
    }
}

impl Render for InfoStatisticsWidgetStatistics {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let show_dialog = self.show_export_dialog;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x17212b))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .border_b_1()
                    .border_color(rgb(0x242f3d))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_md()
                            .bg(rgb(0x242f3d))
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x0088cc))
                            .child("Export XLSX"),
                    ),
            )
            .child(
                if show_dialog {
                    div()
                        .p(px(16.0))
                        .border_b_1()
                        .border_color(rgb(0x242f3d))
                        .child(cx.new(|_| self.export_ctrl.clone()))
                } else {
                    div()
                },
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(cx.new(|_| self.inner.clone())),
            )
    }
}
