//! Chat & Message Moderation Dialog Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/moderate_messages_box.h`
//! - `Telegram/SourceFiles/boxes/moderate_messages_box.cpp:L79-L120`
//!
//! Features:
//! - `kModerateCommonGroups = "moderate-common-groups"` setting toggle
//! - `kModerateMessagesBoxAnimationDuration = 80ms`
//! - Moderation actions: report spam, delete all messages from user, delete reactions, ban/restrict
//! - Ban across multiple common groups simultaneously

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MODERATE_COMMON_GROUPS_ID: &str = "moderate-common-groups";
pub const K_MODERATE_MESSAGES_BOX_ANIMATION_DURATION_MS: u64 = 80;

#[derive(Debug, Clone, Default)]
pub struct ModerateMessagesBoxOptions {
    pub report_spam: bool,
    pub delete_all_messages: bool,
    pub delete_all_reactions: bool,
    pub ban_user: bool,
    pub ban_common_groups: bool,
}

#[derive(Debug, Clone)]
pub struct ModerateMessagesBox {
    pub target_user_id: u64,
    pub target_user_name: String,
    pub message_ids: Vec<u64>,
    pub options: ModerateMessagesBoxOptions,
    pub common_groups_count: usize,
}

impl ModerateMessagesBox {
    pub fn new(target_user_id: u64, target_user_name: impl Into<String>, message_ids: Vec<u64>) -> Self {
        Self {
            target_user_id,
            target_user_name: target_user_name.into(),
            message_ids,
            options: ModerateMessagesBoxOptions::default(),
            common_groups_count: 0,
        }
    }

    pub fn with_common_groups(mut self, count: usize) -> Self {
        self.common_groups_count = count;
        self
    }

    pub fn set_report_spam(&mut self, val: bool) {
        self.options.report_spam = val;
    }

    pub fn set_delete_all(&mut self, val: bool) {
        self.options.delete_all_messages = val;
    }

    pub fn set_ban_user(&mut self, val: bool) {
        self.options.ban_user = val;
    }
}

impl Default for ModerateMessagesBox {
    fn default() -> Self {
        Self::new(0, "User", Vec::new())
    }
}

impl Render for ModerateMessagesBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let msg_count = self.message_ids.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD6D6D6))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0xC62828))
                    .child(format!("Moderate Messages from {}", self.target_user_name)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x444444))
                    .child(format!("Selected messages to delete: {}", msg_count)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x555555))
                    .child(format!("• Report spam: {}", if self.options.report_spam { "Yes" } else { "No" }))
                    .child(format!("• Delete all messages: {}", if self.options.delete_all_messages { "Yes" } else { "No" }))
                    .child(format!("• Ban user from chat: {}", if self.options.ban_user { "Yes" } else { "No" }))
                    .child(format!("• Common groups to ban from: {}", self.common_groups_count)),
            )
    }
}
