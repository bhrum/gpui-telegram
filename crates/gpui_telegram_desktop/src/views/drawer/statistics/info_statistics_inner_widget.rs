//! Statistics Charts Container & Inner Metrics Layout
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_inner_widget.cpp:L1-L1025`
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_inner_widget.h:L1-L90`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_statistics_recent_message::InfoStatisticsRecentMessageStatistics;
use super::info_statistics_list_controllers::InfoStatisticsListControllersStatistics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticsPeriod {
    Day,
    Week,
    Month,
}

#[derive(Debug, Clone)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub struct InfoStatisticsInnerWidgetStatistics {
    pub period: StatisticsPeriod,
    pub views_by_hour: Vec<ChartDataPoint>,
    pub recent_messages: InfoStatisticsRecentMessageStatistics,
    pub list_controllers: InfoStatisticsListControllersStatistics,
    pub is_visible: bool,
}

impl InfoStatisticsInnerWidgetStatistics {
    pub fn new() -> Self {
        let mut s = Self {
            period: StatisticsPeriod::Week,
            views_by_hour: Vec::new(),
            recent_messages: InfoStatisticsRecentMessageStatistics::new(),
            list_controllers: InfoStatisticsListControllersStatistics::new(),
            is_visible: true,
        };
        s.init_chart_data();
        s
    }

    pub fn init_chart_data(&mut self) {
        self.views_by_hour = vec![
            ChartDataPoint { label: "00:00".into(), value: 1200 },
            ChartDataPoint { label: "04:00".into(), value: 450 },
            ChartDataPoint { label: "08:00".into(), value: 3400 },
            ChartDataPoint { label: "12:00".into(), value: 5800 },
            ChartDataPoint { label: "16:00".into(), value: 6200 },
            ChartDataPoint { label: "20:00".into(), value: 7100 },
        ];
    }

    pub fn set_period(&mut self, period: StatisticsPeriod) {
        self.period = period;
    }
}

impl Default for InfoStatisticsInnerWidgetStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStatisticsInnerWidgetStatistics {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_period = self.period;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Views by Hour"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .children([(StatisticsPeriod::Day, "Day"), (StatisticsPeriod::Week, "Week"), (StatisticsPeriod::Month, "Month")].into_iter().map(|(p, label)| {
                                let is_active = p == current_period;
                                div()
                                    .px(px(8.0))
                                    .py(px(3.0))
                                    .rounded_md()
                                    .bg(if is_active { rgb(0x0088cc) } else { rgb(0x1e2c3a) })
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(label)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_end()
                    .justify_between()
                    .h(px(90.0))
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0x1e2c3a))
                    .children(self.views_by_hour.iter().map(|pt| {
                        let bar_h = (pt.value as f32 / 7500.0 * 60.0).max(4.0);
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .w(px(18.0))
                                    .h(px(bar_h))
                                    .rounded_t_sm()
                                    .bg(rgb(0x0088cc)),
                            )
                            .child(div().text_xs().text_color(rgb(0x8e8e93)).child(pt.label.clone()))
                    })),
            )
            .child(
                div()
                    .flex_col()
                    .child(cx.new(|_| self.recent_messages.clone())),
            )
            .child(
                div()
                    .flex_col()
                    .child(cx.new(|_| self.list_controllers.clone())),
            )
    }
}
