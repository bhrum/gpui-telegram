//! History View No Forwards Request Service Media (`history_view_no_forwards_request`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.h` (L1-L25)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.cpp` (L1-L137)
//!
//! Renders the service message media card displayed when content protection / no-forwards
//! is requested by an administrator or peer in a dialogue or supergroup.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Bullet point diameter for service feature lists.
/// Upstream: `history_view_no_forwards_request.cpp:L68` (`st::mediaUnreadSize`)
pub const K_BULLET_DIAMETER: f32 = 4.0;

/// Bullet indent skip for service feature lists.
/// Upstream: `history_view_no_forwards_request.cpp:L48` (`st::historyGroupAboutBulletSkip`)
pub const K_BULLET_SKIP: f32 = 8.0;

/// Individual feature restriction bullet item.
/// Upstream: `history_view_no_forwards_request.cpp:L24-L76` (`class BulletPointPart`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoForwardsFeatureBullet {
    pub text: String,
    pub is_last_item: bool,
}

/// Upstream standard restriction bullet points.
/// Upstream: `history_view_no_forwards_request.cpp:L118-L133`
pub fn default_restriction_features() -> Vec<NoForwardsFeatureBullet> {
    vec![
        NoForwardsFeatureBullet {
            text: "Forwarding messages".to_string(),
            is_last_item: false,
        },
        NoForwardsFeatureBullet {
            text: "Saving media to gallery or files".to_string(),
            is_last_item: false,
        },
        NoForwardsFeatureBullet {
            text: "Copying text and media links".to_string(),
            is_last_item: true,
        },
    ]
}

/// Authentic GPUI View for No Forwards Request service bubble.
/// Upstream: `history_view_no_forwards_request.h:L18-L24`, `history_view_no_forwards_request.cpp:L80-L135`
#[derive(Debug, Clone)]
pub struct HistoryViewNoForwardsRequestViewMedia {
    pub is_from_self: bool,
    pub sender_name: String,
    pub title_notice: String,
    pub bullet_points: Vec<NoForwardsFeatureBullet>,
    pub is_dismissed: bool,
}

impl HistoryViewNoForwardsRequestViewMedia {
    /// Creates a new no-forwards service card.
    /// Upstream: `history_view_no_forwards_request.cpp:L86-L116`
    pub fn new(is_from_self: bool, sender_name: impl Into<String>) -> Self {
        let name = sender_name.into();
        let title_notice = if is_from_self {
            "You requested to restrict saving and forwarding content in this chat.".to_string()
        } else {
            format!("{} requested to restrict saving and forwarding content in this chat.", name)
        };

        Self {
            is_from_self,
            sender_name: name,
            title_notice,
            bullet_points: default_restriction_features(),
            is_dismissed: false,
        }
    }

    /// Bullet point count.
    pub fn features_count(&self) -> usize {
        self.bullet_points.len()
    }
}

impl Default for HistoryViewNoForwardsRequestViewMedia {
    fn default() -> Self {
        Self::new(true, "You")
    }
}

impl Render for HistoryViewNoForwardsRequestViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .py(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .max_w(px(380.0))
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xE8EEF3))
                    .child(
                        // Header title with lock icon
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .mb(px(6.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("🔒")
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x333333))
                                    .child(self.title_notice.clone())
                            )
                    )
                    // Bullet list of restricted features
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .pl(px(16.0))
                            .children(
                                self.bullet_points.iter().map(|b| {
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(6.0))
                                        .child(
                                            div()
                                                .w(px(K_BULLET_DIAMETER))
                                                .h(px(K_BULLET_DIAMETER))
                                                .rounded_full()
                                                .bg(rgb(0x555555))
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(0x555555))
                                                .child(b.text.clone())
                                        )
                                })
                            )
                    )
            )
    }
}
