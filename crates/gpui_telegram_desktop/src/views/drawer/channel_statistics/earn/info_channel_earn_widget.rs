//! Channel Monetization Drawer Screen Widget
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_widget.cpp:L1-L122`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_widget.h:L1-L76`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_channel_earn_list::InfoChannelEarnListChannelStatisticsEarn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelEarnTab {
    Ton,
    Stars,
}

#[derive(Debug, Clone)]
pub struct InfoChannelEarnWidgetChannelStatisticsEarn {
    pub channel_id: u64,
    pub title: String,
    pub active_tab: ChannelEarnTab,
    pub list: InfoChannelEarnListChannelStatisticsEarn,
    pub is_visible: bool,
}

impl InfoChannelEarnWidgetChannelStatisticsEarn {
    pub fn new(channel_id: u64, title: impl Into<String>) -> Self {
        Self {
            channel_id,
            title: title.into(),
            active_tab: ChannelEarnTab::Ton,
            list: InfoChannelEarnListChannelStatisticsEarn::new(),
            is_visible: true,
        }
    }

    pub fn set_tab(&mut self, tab: ChannelEarnTab) {
        self.active_tab = tab;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoChannelEarnWidgetChannelStatisticsEarn {
    fn default() -> Self {
        Self::new(0, "Channel Monetization")
    }
}

impl Render for InfoChannelEarnWidgetChannelStatisticsEarn {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_ton = self.active_tab == ChannelEarnTab::Ton;

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
                    .flex()
                    .border_b_1()
                    .border_color(rgb(0x242f3d))
                    .child(
                        div()
                            .flex_1()
                            .py(px(8.0))
                            .text_center()
                            .font_weight(if is_ton { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .text_color(if is_ton { rgb(0x0088cc) } else { rgb(0x8e8e93) })
                            .border_b_2()
                            .border_color(if is_ton { rgb(0x0088cc) } else { rgb(0x00000000) })
                            .child("TON Revenue"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .py(px(8.0))
                            .text_center()
                            .font_weight(if !is_ton { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .text_color(if !is_ton { rgb(0xffa800) } else { rgb(0x8e8e93) })
                            .border_b_2()
                            .border_color(if !is_ton { rgb(0xffa800) } else { rgb(0x00000000) })
                            .child("Telegram Stars"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(cx.new(|_| self.list.clone())),
            )
    }
}
