//! Similar Channels Recommendations Drawer Screen
//! Upstream: `Telegram/SourceFiles/info/similar_peers/info_similar_peers_widget.cpp:L1-L533`
//! Upstream: `Telegram/SourceFiles/info/similar_peers/info_similar_peers_widget.h:L1-L77`
//! Subsystem: info | Feature ID: TCG-N11 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SimilarChannelItem {
    pub peer_id: u64,
    pub title: String,
    pub subscriber_count: usize,
    pub is_subscribed: bool,
}

#[derive(Debug, Clone)]
pub struct InfoSimilarPeersWidgetSimilarPeers {
    pub channel_id: u64,
    pub title: String,
    pub channels: Vec<SimilarChannelItem>,
    pub unlock_boosts_needed: Option<usize>,
    pub is_visible: bool,
}

impl InfoSimilarPeersWidgetSimilarPeers {
    pub fn new(channel_id: u64) -> Self {
        let mut s = Self {
            channel_id,
            title: "Similar Channels".into(),
            channels: Vec::new(),
            unlock_boosts_needed: None,
            is_visible: true,
        };
        s.init_sample_similar_channels();
        s
    }

    pub fn init_sample_similar_channels(&mut self) {
        self.channels = vec![
            SimilarChannelItem {
                peer_id: 501,
                title: "Rust Language Community".into(),
                subscriber_count: 84_200,
                is_subscribed: false,
            },
            SimilarChannelItem {
                peer_id: 502,
                title: "Systems Programming Weekly".into(),
                subscriber_count: 52_100,
                is_subscribed: true,
            },
            SimilarChannelItem {
                peer_id: 503,
                title: "GPUI & Zed Developments".into(),
                subscriber_count: 36_900,
                is_subscribed: false,
            },
        ];
    }

    pub fn toggle_subscription(&mut self, peer_id: u64) {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.peer_id == peer_id) {
            ch.is_subscribed = !ch.is_subscribed;
        }
    }

    pub fn set_unlock_boosts_needed(&mut self, boosts: usize) {
        self.unlock_boosts_needed = Some(boosts);
    }
}

impl Default for InfoSimilarPeersWidgetSimilarPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoSimilarPeersWidgetSimilarPeers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let unlock_needed = self.unlock_boosts_needed;

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
                if let Some(boosts) = unlock_needed {
                    div()
                        .m(px(16.0))
                        .p(px(12.0))
                        .rounded_lg()
                        .bg(rgb(0x242f3d))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0xffa800))
                                .child(format!("⚡ Channel requires {} more boosts to view similar channels", boosts)),
                        )
                } else {
                    div()
                },
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .gap_2()
                    .children(self.channels.iter().map(|ch| {
                        let is_sub = ch.is_subscribed;
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(10.0))
                            .rounded_md()
                            .bg(rgb(0x1e2c3a))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child(ch.title.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x8e8e93))
                                            .child(format!("{} subscribers", ch.subscriber_count)),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded_full()
                                    .bg(if is_sub { rgb(0x242f3d) } else { rgb(0x0088cc) })
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(if is_sub { "Subscribed" } else { "Subscribe" }),
                            )
                    })),
            )
    }
}
