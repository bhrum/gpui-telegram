//! Shared Statistics State and Snapshot Structures
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_common.h:L1-L24`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChannelStatisticsOverview {
    pub period_start_unix: u64,
    pub period_end_unix: u64,
    pub followers_count: usize,
    pub followers_growth: i32,
    pub enabled_notifications_percent: f32,
    pub views_per_post: usize,
    pub shares_per_post: usize,
    pub reactions_per_post: usize,
}

impl Default for ChannelStatisticsOverview {
    fn default() -> Self {
        Self {
            period_start_unix: 1761900000,
            period_end_unix: 1762504800,
            followers_count: 142_850,
            followers_growth: 3_240,
            enabled_notifications_percent: 24.5,
            views_per_post: 18_420,
            shares_per_post: 1_210,
            reactions_per_post: 2_890,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoStatisticsCommonStatistics {
    pub overview: ChannelStatisticsOverview,
    pub scroll_top: f32,
    pub recent_posts_expanded: bool,
    pub cached_post_previews: HashMap<i32, String>,
    pub is_visible: bool,
}

impl InfoStatisticsCommonStatistics {
    pub fn new() -> Self {
        Self {
            overview: ChannelStatisticsOverview::default(),
            scroll_top: 0.0,
            recent_posts_expanded: false,
            cached_post_previews: HashMap::new(),
            is_visible: true,
        }
    }

    pub fn set_scroll_position(&mut self, top: f32) {
        self.scroll_top = top;
    }

    pub fn toggle_recent_posts_expanded(&mut self) {
        self.recent_posts_expanded = !self.recent_posts_expanded;
    }

    pub fn cache_preview(&mut self, msg_id: i32, thumbnail_uri: String) {
        self.cached_post_previews.insert(msg_id, thumbnail_uri);
    }
}

impl Default for InfoStatisticsCommonStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStatisticsCommonStatistics {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let ov = &self.overview;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_3()
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
                            .child("Overview"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x43a047))
                            .child(format!("+{} this week", ov.followers_growth)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        div()
                            .flex_1()
                            .p(px(12.0))
                            .rounded_md()
                            .bg(rgb(0x1e2c3a))
                            .child(div().text_xs().text_color(rgb(0x8e8e93)).child("Subscribers"))
                            .child(div().text_lg().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("{}", ov.followers_count))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(12.0))
                            .rounded_md()
                            .bg(rgb(0x1e2c3a))
                            .child(div().text_xs().text_color(rgb(0x8e8e93)).child("Notifications"))
                            .child(div().text_lg().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("{:.1}%", ov.enabled_notifications_percent))),
                    ),
            )
    }
}
