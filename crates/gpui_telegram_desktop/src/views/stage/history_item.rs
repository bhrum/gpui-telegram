//! GPUI Telegram History Item Stage View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item.h:L89-L107` (HistoryItemCommonFields)
//! - `Telegram/SourceFiles/history/history_item.cpp:L97-L100` (kNotificationTextLimit, kPinnedMessageTextLimit)
//! - `Telegram/SourceFiles/history/history_item.cpp:L3367-L3372` (isTooOldForEdit)
//! - `Telegram/SourceFiles/history/history_item.cpp:L3374-L3386` (allowsEdit)
//! - `Telegram/SourceFiles/history/history_item.cpp:L3490-L3524` (canDeleteForEveryone)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::item::{
    HistoryItem as ClientHistoryItem, MessageFlags, PaidPostType,
    DEFAULT_EDIT_TIME_LIMIT_SECS, DEFAULT_REVOKE_TIME_LIMIT_SECS,
    K_NOTIFICATION_TEXT_LIMIT, K_PINNED_MESSAGE_TEXT_LIMIT,
};

/// Upstream `history_item.h:L121-L845`: GPUI Visual Model for a History Item in the Chat Stage
#[derive(Debug, Clone)]
pub struct HistoryItem {
    pub inner: ClientHistoryItem,
    pub is_selected: bool,
    pub is_hovered: bool,
    pub is_highlighted: bool,
    pub active_reaction: Option<String>,
}

impl HistoryItem {
    pub fn new(id: i64, peer_id: i64, from_id: i64, date: i64, text: impl Into<String>) -> Self {
        Self {
            inner: ClientHistoryItem::new(id, peer_id, from_id, date, text),
            is_selected: false,
            is_hovered: false,
            is_highlighted: false,
            active_reaction: None,
        }
    }

    pub fn with_flags(mut self, flags: MessageFlags) -> Self {
        self.inner.flags = flags;
        self
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn set_highlighted(&mut self, highlighted: bool) {
        self.is_highlighted = highlighted;
    }

    pub fn toggle_reaction(&mut self, reaction: impl Into<String>) {
        let r = reaction.into();
        if self.active_reaction.as_deref() == Some(&r) {
            self.active_reaction = None;
        } else {
            self.active_reaction = Some(r);
        }
    }

    pub fn allows_edit(&self, now: i64, edit_limit: i64, can_edit_indefinitely: bool) -> bool {
        self.inner.allows_edit(now, edit_limit, can_edit_indefinitely)
    }

    pub fn can_delete_for_everyone(
        &self,
        now: i64,
        is_self: bool,
        is_user: bool,
        is_channel: bool,
        is_bot: bool,
    ) -> bool {
        self.inner.can_delete_for_everyone(
            now,
            is_self,
            is_user,
            is_channel,
            is_bot,
            DEFAULT_REVOKE_TIME_LIMIT_SECS,
            DEFAULT_REVOKE_TIME_LIMIT_SECS,
            true,
        )
    }

    pub fn render_view(&self) -> impl IntoElement {
        let is_outgoing = self.inner.out();
        let bg_color = if self.is_selected {
            rgb(0xCCE5FF)
        } else if is_outgoing {
            rgb(0xEEFFDE) // Outgoing bubble green
        } else {
            rgb(0xFFFFFF) // Incoming bubble white
        };

        let bubble_row = if is_outgoing {
            div().flex().w_full().justify_end()
        } else {
            div().flex().w_full().justify_start()
        };

        bubble_row
            .py(px(4.0))
            .px(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .max_w(px(480.0))
                    .p(px(10.0))
                    .bg(bg_color)
                    .rounded(px(12.0))
                    .shadow_sm()
                    .border_1()
                    .border_color(if self.is_highlighted {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0xE0E0E0)
                    })
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgb(0x111111))
                            .child(self.inner.text.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .items_center()
                            .mt(px(4.0))
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(format!(
                                "{}{}",
                                if self.inner.hide_edited_badge() { "" } else if self.inner.edit_date.is_some() { "edited " } else { "" },
                                if is_outgoing { "✓✓" } else { "" }
                            )),
                    ),
            )
    }
}

impl Default for HistoryItem {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, "")
    }
}

impl Render for HistoryItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_outgoing = self.inner.out();
        let bg_color = if self.is_selected {
            rgb(0xCCE5FF)
        } else if is_outgoing {
            rgb(0xEEFFDE) // Outgoing bubble green
        } else {
            rgb(0xFFFFFF) // Incoming bubble white
        };

        let bubble_row = if is_outgoing {
            div().flex().w_full().justify_end()
        } else {
            div().flex().w_full().justify_start()
        };

        bubble_row
            .py(px(4.0))
            .px(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .max_w(px(480.0))
                    .p(px(10.0))
                    .bg(bg_color)
                    .rounded(px(12.0))
                    .shadow_sm()
                    .border_1()
                    .border_color(if self.is_highlighted {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0xE0E0E0)
                    })
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgb(0x111111))
                            .child(self.inner.text.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .items_center()
                            .mt(px(4.0))
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(format!(
                                "{}{}",
                                if self.inner.hide_edited_badge() { "" } else if self.inner.edit_date.is_some() { "edited " } else { "" },
                                if is_outgoing { "✓✓" } else { "" }
                            )),
                    ),
            )
    }
}
