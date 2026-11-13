//! Channel Boost Level Progress and Boosters List
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_inner_widget.cpp:L1-L575`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_inner_widget.h:L1-L63`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct BoosterMember {
    pub user_id: u64,
    pub name: String,
    pub boost_count: usize,
    pub expires_date: String,
    pub is_gift: bool,
}

#[derive(Debug, Clone)]
pub struct ChannelBoostStatus {
    pub level: usize,
    pub current_boosts: usize,
    pub next_level_boosts: usize,
    pub boost_link: String,
    pub boosters: Vec<BoosterMember>,
}

impl Default for ChannelBoostStatus {
    fn default() -> Self {
        Self {
            level: 4,
            current_boosts: 18,
            next_level_boosts: 24,
            boost_link: "t.me/boost/telegram".into(),
            boosters: vec![
                BoosterMember {
                    user_id: 201,
                    name: "Alex".into(),
                    boost_count: 4,
                    expires_date: "Dec 12, 2026".into(),
                    is_gift: true,
                },
                BoosterMember {
                    user_id: 202,
                    name: "Elena".into(),
                    boost_count: 2,
                    expires_date: "Jan 15, 2027".into(),
                    is_gift: false,
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoBoostsInnerWidgetChannelStatisticsBoosts {
    pub status: ChannelBoostStatus,
    pub is_visible: bool,
}

impl InfoBoostsInnerWidgetChannelStatisticsBoosts {
    pub fn new() -> Self {
        Self {
            status: ChannelBoostStatus::default(),
            is_visible: true,
        }
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.status.next_level_boosts == 0 {
            1.0
        } else {
            (self.status.current_boosts as f32 / self.status.next_level_boosts as f32).min(1.0)
        }
    }

    pub fn boosts_needed_for_next_level(&self) -> usize {
        self.status.next_level_boosts.saturating_sub(self.status.current_boosts)
    }

    pub fn perks_for_level(level: usize) -> &'static [&'static str] {
        match level {
            0 => &[],
            1 => &["Channel Stories"],
            2 => &["Channel Stories", "Custom Reactions"],
            3 => &["Channel Stories", "Custom Reactions", "Link Styles"],
            4..=7 => &["Channel Stories", "Custom Reactions", "Link Styles", "Custom Channel Wallpaper"],
            _ => &["Channel Stories", "Custom Reactions", "Link Styles", "Custom Channel Wallpaper", "Custom Emoji Pack"],
        }
    }
}

impl Default for InfoBoostsInnerWidgetChannelStatisticsBoosts {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoBoostsInnerWidgetChannelStatisticsBoosts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let level = self.status.level;
        let current = self.status.current_boosts;
        let next = self.status.next_level_boosts;
        let needed = self.boosts_needed_for_next_level();
        let ratio = self.progress_ratio();
        let perks = Self::perks_for_level(level);

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_lg()
                    .bg(rgb(0x1e2c3a))
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(format!("Level {}", level)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x9b51e0))
                                    .child(format!("{}/{} boosts", current, next)),
                            ),
                    )
                    .child(
                        div()
                            .h(px(6.0))
                            .rounded_full()
                            .bg(rgb(0x242f3d))
                            .child(
                                div()
                                    .h_full()
                                    .w(px(260.0 * ratio))
                                    .rounded_full()
                                    .bg(rgb(0x9b51e0)),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child(format!("{} more boosts to Level {}", needed, level + 1)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(10.0))
                    .rounded_md()
                    .bg(rgb(0x0088cc))
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Share Boost Link"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Active Boosters"),
                    )
                    .children(self.status.boosters.iter().map(|b| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .py(px(8.0))
                            .border_b_1()
                            .border_color(rgb(0x242f3d))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child(b.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x8e8e93))
                                            .child(format!("Expires {}", b.expires_date)),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded_full()
                                    .bg(rgb(0x9b51e0))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(format!("+{}", b.boost_count)),
                            )
                    })),
            )
    }
}
