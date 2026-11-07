//! Giveaway Participant Type Selection Row
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_type_row.cpp:L1-L194`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_type_row.h:L1-L70`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GiveawayParticipantType {
    RandomMembers,
    SpecificUsers,
    PrepaidVouchers,
}

#[derive(Debug, Clone)]
pub struct GiveawayTypeRowChannelStatisticsBoostsGiveaway {
    pub participant_type: GiveawayParticipantType,
    pub is_selected: bool,
    pub title: String,
    pub description: String,
    pub prepaid_boosts: Option<usize>,
    pub is_visible: bool,
}

impl GiveawayTypeRowChannelStatisticsBoostsGiveaway {
    pub fn new(participant_type: GiveawayParticipantType, is_selected: bool) -> Self {
        let (title, description) = match participant_type {
            GiveawayParticipantType::RandomMembers => (
                "All Subscribers".to_string(),
                "Randomly select winners from channel subscribers".to_string(),
            ),
            GiveawayParticipantType::SpecificUsers => (
                "Specific Users".to_string(),
                "Only selected subscribers can win".to_string(),
            ),
            GiveawayParticipantType::PrepaidVouchers => (
                "Prepaid Giveaway".to_string(),
                "Use prepaid Telegram Premium vouchers".to_string(),
            ),
        };

        Self {
            participant_type,
            is_selected,
            title,
            description,
            prepaid_boosts: None,
            is_visible: true,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    pub fn set_prepaid_boosts(&mut self, boosts: usize) {
        self.prepaid_boosts = Some(boosts);
    }
}

impl Default for GiveawayTypeRowChannelStatisticsBoostsGiveaway {
    fn default() -> Self {
        Self::new(GiveawayParticipantType::RandomMembers, true)
    }
}

impl Render for GiveawayTypeRowChannelStatisticsBoostsGiveaway {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.is_selected;

        div()
            .flex()
            .items_center()
            .justify_between()
            .p(px(12.0))
            .rounded_lg()
            .bg(if is_selected { rgb(0x233140) } else { rgb(0x1e2733) })
            .border_1()
            .border_color(if is_selected { rgb(0x0088cc) } else { rgb(0x242f3d) })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded_full()
                            .border_2()
                            .border_color(if is_selected { rgb(0x0088cc) } else { rgb(0x6c7883) })
                            .child(
                                if is_selected {
                                    div()
                                        .w(px(10.0))
                                        .h(px(10.0))
                                        .rounded_full()
                                        .bg(rgb(0x0088cc))
                                } else {
                                    div()
                                },
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(self.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8e8e93))
                                    .child(self.description.clone()),
                            ),
                    ),
            )
            .child(
                if let Some(boosts) = self.prepaid_boosts {
                    div()
                        .px(px(8.0))
                        .py(px(2.0))
                        .rounded_full()
                        .bg(rgb(0x9b51e0))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child(format!("+{} boosts", boosts))
                } else {
                    div()
                },
            )
    }
}
