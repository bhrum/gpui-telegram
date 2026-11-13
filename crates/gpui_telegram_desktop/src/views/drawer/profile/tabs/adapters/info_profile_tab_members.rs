//! Info Profile Tab Members Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_members.cpp:L1-L151`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_members.h:L1-L20`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Member roster entry for groups and channels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberRosterItem {
    pub user_id: i64,
    pub name: String,
    pub is_online: bool,
    pub status_text: String,
    pub is_admin: bool,
    pub is_owner: bool,
    pub rank_custom: Option<String>,
    pub can_promote: bool,
    pub can_kick: bool,
}

/// Tab adapter bridging the profile tabs host with channel or group member rosters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabMembersProfileTabsAdapters {
    pub channel_id: i64,
    pub members_count: usize,
    pub online_members_count: usize,
    pub search_enabled: bool,
    pub is_active: bool,
    pub search_query: String,
    pub members: Vec<MemberRosterItem>,
}

impl Default for InfoProfileTabMembersProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0)
    }
}

impl InfoProfileTabMembersProfileTabsAdapters {
    /// Upstream `kEnableSearchMembersAfterCount = 20` (`info_profile_tab_members.cpp:L21`).
    pub const ENABLE_SEARCH_MEMBERS_AFTER: usize = 20;

    pub fn new(channel_id: i64) -> Self {
        Self {
            channel_id,
            members_count: 0,
            online_members_count: 0,
            search_enabled: false,
            is_active: false,
            search_query: String::new(),
            members: Vec::new(),
        }
    }

    pub fn update_counts(&mut self, total: usize, online: usize) {
        self.members_count = total;
        self.online_members_count = online;
        self.search_enabled = total >= Self::ENABLE_SEARCH_MEMBERS_AFTER;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn add_member(&mut self, member: MemberRosterItem) {
        self.members.push(member);
        self.sync_counts_from_roster();
    }

    pub fn sync_counts_from_roster(&mut self) {
        let total = self.members.len();
        let online = self.members.iter().filter(|m| m.is_online).count();
        self.update_counts(total, online);
    }

    pub fn kick_member(&mut self, user_id: i64) -> bool {
        let prev_len = self.members.len();
        self.members.retain(|m| m.user_id != user_id);
        let removed = self.members.len() < prev_len;
        if removed {
            self.sync_counts_from_roster();
        }
        removed
    }

    pub fn promote_member(&mut self, user_id: i64, rank: Option<String>) -> bool {
        if let Some(m) = self.members.iter_mut().find(|m| m.user_id == user_id) {
            m.is_admin = true;
            m.rank_custom = rank;
            true
        } else {
            false
        }
    }

    pub fn filtered_members(&self) -> Vec<&MemberRosterItem> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.members.iter().collect()
        } else {
            self.members
                .iter()
                .filter(|m| m.name.to_lowercase().contains(&q))
                .collect()
        }
    }
}

impl Render for InfoProfileTabMembersProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let filtered = self.filtered_members();
        let is_empty = filtered.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(4.0))
                    .py(px(4.0))
                    .border_b_1()
                    .border_color(rgb(0xEBEEF2))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x707579))
                            .child(format!("{} members ({} online)", self.members_count, self.online_members_count))
                    )
                    .children(if self.search_enabled {
                        Some(div().text_xs().text_color(rgb(0x2AABEE)).child("🔍 Search enabled"))
                    } else {
                        None
                    })
            )
            .child(
                if is_empty {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(px(140.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No members found")
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(filtered.into_iter().map(|member| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .p(px(6.0))
                                .rounded_sm()
                                .hover(|s| s.bg(rgb(0xF5F7FA)))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .relative()
                                                .size(px(36.0))
                                                .rounded_full()
                                                .bg(rgb(0x40A7E3))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(rgb(0xFFFFFF))
                                                .font_weight(FontWeight::BOLD)
                                                .text_sm()
                                                .child(member.name.chars().next().unwrap_or('?').to_string())
                                                .children(if member.is_online {
                                                    Some(div().absolute().bottom_0().right_0().size(px(10.0)).rounded_full().bg(rgb(0x4FAE4E)).border_2().border_color(rgb(0xFFFFFF)))
                                                } else {
                                                    None
                                                })
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(rgb(0x222222))
                                                                .child(member.name.clone())
                                                        )
                                                        .children(if member.is_owner {
                                                            Some(div().px(px(4.0)).rounded_xs().bg(rgb(0xFEF2D6)).text_xs().text_color(rgb(0xB78103)).child("owner"))
                                                        } else if member.is_admin {
                                                            let rank = member.rank_custom.clone().unwrap_or_else(|| "admin".to_string());
                                                            Some(div().px(px(4.0)).rounded_xs().bg(rgb(0xEEF5FE)).text_xs().text_color(rgb(0x2AABEE)).child(rank))
                                                        } else {
                                                            None
                                                        })
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x707579))
                                                        .child(member.status_text.clone())
                                                )
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(6.0))
                                        .children(if member.can_promote && !member.is_admin {
                                            Some(div().px(px(8.0)).py(px(2.0)).rounded_xs().bg(rgb(0xF0F2F5)).text_xs().text_color(rgb(0x2AABEE)).child("Promote"))
                                        } else {
                                            None
                                        })
                                        .children(if member.can_kick {
                                            Some(div().px(px(8.0)).py(px(2.0)).rounded_xs().bg(rgb(0xFDEAEA)).text_xs().text_color(rgb(0xE05353)).child("Remove"))
                                        } else {
                                            None
                                        })
                                )
                        }))
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_members_adapter_management() {
        let mut adapter = InfoProfileTabMembersProfileTabsAdapters::new(6006);
        assert_eq!(adapter.channel_id, 6006);
        assert_eq!(adapter.members_count, 0);
        assert!(!adapter.search_enabled);

        adapter.add_member(MemberRosterItem {
            user_id: 1,
            name: "Founder".into(),
            is_online: true,
            status_text: "online".into(),
            is_admin: true,
            is_owner: true,
            rank_custom: None,
            can_promote: false,
            can_kick: false,
        });
        adapter.add_member(MemberRosterItem {
            user_id: 2,
            name: "Member 2".into(),
            is_online: false,
            status_text: "last seen yesterday".into(),
            is_admin: false,
            is_owner: false,
            rank_custom: None,
            can_promote: true,
            can_kick: true,
        });

        assert_eq!(adapter.members_count, 2);
        assert_eq!(adapter.online_members_count, 1);

        assert!(adapter.promote_member(2, Some("Moderator".into())));
        assert!(adapter.members[1].is_admin);
        assert_eq!(adapter.members[1].rank_custom.as_deref(), Some("Moderator"));

        assert!(adapter.kick_member(2));
        assert_eq!(adapter.members_count, 1);
        assert_eq!(adapter.members[0].user_id, 1);

        adapter.update_counts(25, 10);
        assert!(adapter.search_enabled);
    }
}
