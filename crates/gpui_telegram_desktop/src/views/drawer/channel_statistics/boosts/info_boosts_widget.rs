//! Channel Boosts Outer Drawer Widget
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_widget.cpp:L1-L118`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_widget.h:L1-L67`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_boosts_inner_widget::InfoBoostsInnerWidgetChannelStatisticsBoosts;

#[derive(Debug, Clone)]
pub struct InfoBoostsWidgetChannelStatisticsBoosts {
    pub channel_id: u64,
    pub title: String,
    pub inner: InfoBoostsInnerWidgetChannelStatisticsBoosts,
    pub is_visible: bool,
}

impl InfoBoostsWidgetChannelStatisticsBoosts {
    pub fn new(channel_id: u64, title: impl Into<String>) -> Self {
        Self {
            channel_id,
            title: title.into(),
            inner: InfoBoostsInnerWidgetChannelStatisticsBoosts::new(),
            is_visible: true,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoBoostsWidgetChannelStatisticsBoosts {
    fn default() -> Self {
        Self::new(0, "Channel Boosts")
    }
}

impl Render for InfoBoostsWidgetChannelStatisticsBoosts {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(cx.new(|_| self.inner.clone())),
            )
    }
}
