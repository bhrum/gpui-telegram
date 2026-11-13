//! Recent Post Statistics Item Card
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_recent_message.cpp:L1-L358`
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_recent_message.h:L1-L73`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct RecentMessageItem {
    pub msg_id: i32,
    pub date: String,
    pub snippet: String,
    pub views: usize,
    pub shares: usize,
    pub public_forwards: usize,
    pub reactions_count: usize,
    pub has_media: bool,
}

pub fn format_metric_count(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct InfoStatisticsRecentMessageStatistics {
    pub messages: Vec<RecentMessageItem>,
    pub is_visible: bool,
}

impl InfoStatisticsRecentMessageStatistics {
    pub fn new() -> Self {
        let mut s = Self {
            messages: Vec::new(),
            is_visible: true,
        };
        s.init_sample_messages();
        s
    }

    pub fn init_sample_messages(&mut self) {
        self.messages = vec![
            RecentMessageItem {
                msg_id: 1042,
                date: "Nov 6".into(),
                snippet: "Major update released: Introducing Telegram Mini Apps 2.0 with full-screen mode...".into(),
                views: 45_200,
                shares: 3_820,
                public_forwards: 412,
                reactions_count: 5_120,
                has_media: true,
            },
            RecentMessageItem {
                msg_id: 1039,
                date: "Nov 4".into(),
                snippet: "Check out our latest development blog post on WebRTC voice and video architecture.".into(),
                views: 28_400,
                shares: 1_490,
                public_forwards: 185,
                reactions_count: 2_940,
                has_media: false,
            },
        ];
    }

    pub fn add_message(&mut self, item: RecentMessageItem) {
        self.messages.push(item);
    }
}

impl Default for InfoStatisticsRecentMessageStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStatisticsRecentMessageStatistics {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Recent Posts"),
            )
            .children(self.messages.iter().map(|msg| {
                let views_str = format_metric_count(msg.views);
                let shares_str = format_metric_count(msg.shares);

                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0x1e2c3a))
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_xs().text_color(rgb(0x8e8e93)).child(msg.date.clone()))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(format!("👁 {}", views_str)))
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(format!("↗ {}", shares_str))),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .line_clamp(2)
                            .child(msg.snippet.clone()),
                    )
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_metric_count() {
        assert_eq!(format_metric_count(500), "500");
        assert_eq!(format_metric_count(1500), "1.5K");
        assert_eq!(format_metric_count(2_400_000), "2.4M");
    }
}
