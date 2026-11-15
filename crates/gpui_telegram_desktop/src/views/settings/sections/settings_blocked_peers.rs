// Source: Telegram/SourceFiles/settings/sections/settings_blocked_peers.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_empty_state,
    settings_section_header,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedPeerItem {
    pub user_id: u64,
    pub name: String,
    pub blocked_date: String,
}

#[derive(Debug, Clone)]
pub struct SettingsBlockedPeersSections {
    pub blocked_list: Vec<BlockedPeerItem>,
    pub search_query: String,
    pub is_loading: bool,
    pub pending_unblock_user_id: Option<u64>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsBlockedPeersSections {
    pub fn new() -> Self {
        Self {
            blocked_list: Vec::new(),
            search_query: String::new(),
            is_loading: false,
            pending_unblock_user_id: None,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self { Self::new() }
    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }

    pub fn block_peer(&mut self, user_id: u64, name: impl Into<String>, date: impl Into<String>) {
        if !self.blocked_list.iter().any(|p| p.user_id == user_id) {
            self.blocked_list.push(BlockedPeerItem { user_id, name: name.into(), blocked_date: date.into() });
        }
    }

    pub fn unblock_peer(&mut self, user_id: u64) -> bool {
        let initial_len = self.blocked_list.len();
        self.blocked_list.retain(|p| p.user_id != user_id);
        let removed = self.blocked_list.len() < initial_len;
        if removed {
            self.pending_unblock_user_id = Some(user_id);
        }
        removed
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn filtered_peers(&self) -> Vec<&BlockedPeerItem> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.blocked_list.iter().collect()
        } else {
            self.blocked_list.iter().filter(|p| p.name.to_lowercase().contains(&q)).collect()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.blocked_list.is_empty()
    }
}

impl Default for SettingsBlockedPeersSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsBlockedPeersSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;
        let peers: Vec<BlockedPeerItem> = self.filtered_peers().into_iter().cloned().collect();

        let count_desc = if lang.starts_with("zh") {
            format!("{} 位用户", peers.len())
        } else {
            format!("{} users", peers.len())
        };

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Close, tr_str(lang, "已屏蔽用户", "Blocked Users"), Some(&count_desc), pal))
                .map(|el| {
                    if peers.is_empty() {
                        el.child(settings_empty_state(IconName::ContactsUsers, tr_str(lang, "暂无屏蔽用户", "No blocked users yet"), pal))
                    } else {
                        el.children(peers.into_iter().map(|peer| {
                            let uid = peer.user_id;
                            settings_action_row(IconName::ContactsUsers, &peer.name, tr_str(lang, "取消屏蔽", "Unblock"), pal, true, move |this| {
                                this.unblock_peer(uid);
                            }, cx)
                        }))
                    }
                }))
            .into_any_element()
    }
}
