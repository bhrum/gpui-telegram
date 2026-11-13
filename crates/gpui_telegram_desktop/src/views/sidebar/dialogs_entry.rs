//! Upstream:
//! - Telegram/SourceFiles/dialogs/dialogs_entry.h:L1-L240
//! - Telegram/SourceFiles/dialogs/dialogs_entry.cpp:L1-L532
//!
//! Anonymous Constants & Sorting Math:
//! - `FixedOnTopDialogPos(index) = 0xFFFFFFFFFFFF000FULL - index` (L44-46)
//! - `PinnedDialogPos(pinnedIndex) = 0xFFFFFFFF000000FFULL - pinnedIndex` (L48-50)
//! - `UnreadOnTopDialogPos(sortKeyByDate) = sortKeyByDate | 0x8000000000000000ULL` (L52-54)
//! - `DialogPosFromDate(date) = (uint64(date) << 32) | (++DialogsPosToTopShift)` (L37-42)
//! - `kArchiveFixOnTopIndex = 1`, `kTopPromotionFixOnTopIndex = 2` (entry.h:L141-142)
//! - `BadgesForUnread` (entry.cpp:L67-98)

use gpui::*;
use gpui_telegram_client::dialogs::{
    badges_for_unread, BadgesState, CountInBadge, DialogEntryData, IncludeInBadge, UnreadState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogsEntryState {
    #[default]
    Idle,
    Active,
    Hovered,
    Disabled,
}

/// Authentic GPUI Dialog Entry component.
/// Upstream: `Dialogs::Entry` (`dialogs_entry.h:L74-222`)
#[derive(Debug, Clone)]
pub struct DialogsEntry {
    pub data: DialogEntryData,
    pub state: DialogsEntryState,
    pub is_active: bool,
    pub is_hovered: bool,
}

impl DialogsEntry {
    pub fn new(id: i64, title: impl Into<String>) -> Self {
        Self {
            data: DialogEntryData::new(id, title),
            state: DialogsEntryState::Idle,
            is_active: false,
            is_hovered: false,
        }
    }

    pub fn with_time_id(mut self, time_id: i64) -> Self {
        self.data.time_id = time_id;
        self
    }

    pub fn with_unread_state(mut self, unread_state: UnreadState) -> Self {
        self.data.unread_state = unread_state;
        self
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.state = if active {
            DialogsEntryState::Active
        } else if self.is_hovered {
            DialogsEntryState::Hovered
        } else {
            DialogsEntryState::Idle
        };
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
        if !self.is_active {
            self.state = if hovered {
                DialogsEntryState::Hovered
            } else {
                DialogsEntryState::Idle
            };
        }
    }

    pub fn compute_sort_position(&self, unread_on_top: bool) -> u64 {
        self.data.compute_sort_position(unread_on_top)
    }

    pub fn badges_state(
        &self,
        count: CountInBadge,
        include: IncludeInBadge,
        count_unread_messages: bool,
        include_muted_counter: bool,
    ) -> BadgesState {
        badges_for_unread(
            &self.data.unread_state,
            count,
            include,
            count_unread_messages,
            include_muted_counter,
        )
    }
}

impl Default for DialogsEntry {
    fn default() -> Self {
        Self::new(0, "Dialog")
    }
}

impl Render for DialogsEntry {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (bg, title_color, sub_color) = if self.is_active {
            (rgb(0x2AABEE), rgb(0xFFFFFF), rgb(0xEEF8FF))
        } else if self.is_hovered {
            (rgb(0xF4F4F5), rgb(0x222222), rgb(0x707579))
        } else {
            (rgb(0xFFFFFF), rgb(0x222222), rgb(0x707579))
        };

        let badge = self.badges_state(
            CountInBadge::Default,
            IncludeInBadge::Default,
            false,
            false,
        );

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(62.0))
            .px(px(10.0))
            .py(px(8.0))
            .bg(bg)
            .child(
                // 46px circular avatar placeholder
                div()
                    .w(px(46.0))
                    .h(px(46.0))
                    .rounded_full()
                    .bg(rgb(0x5682A3))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(16.0))
                    .child(self.data.title.chars().next().unwrap_or('?').to_string())
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .pl(px(12.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_size(px(14.0))
                                    .text_color(title_color)
                                    .child(self.data.title.clone())
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(sub_color)
                                    .child(if self.data.time_id > 0 {
                                        let dt = chrono::DateTime::from_timestamp(self.data.time_id, 0)
                                            .unwrap_or_else(|| chrono::Utc::now());
                                        dt.format("%H:%M").to_string()
                                    } else {
                                        String::new()
                                    })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .pt(px(4.0))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(sub_color)
                                    .child(if self.data.is_forum { "Forum" } else { "Message" })
                            )
                            .child(
                                if badge.unread_counter > 0 {
                                    let badge_bg = if badge.unread_muted {
                                        rgb(0xC4C9CC)
                                    } else {
                                        rgb(0x2AABEE)
                                    };
                                    div()
                                        .h(px(19.0))
                                        .px(px(6.0))
                                        .rounded_full()
                                        .bg(badge_bg)
                                        .text_color(rgb(0xFFFFFF))
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(11.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(badge.unread_counter.to_string())
                                } else {
                                    div()
                                }
                            )
                    )
            )
    }
}
