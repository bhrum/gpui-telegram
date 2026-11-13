//! Persistent per-session user preferences and UI state
//! Upstream: Telegram/SourceFiles/main/main_session_settings.h, main_session_settings.cpp
//! Subsystem: main | Feature ID: TCG-N02 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Active tab in the composer selector drawer (Emoji, Stickers, GIFs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectorTab {
    #[default]
    Emoji = 0,
    Stickers = 1,
    Gifs = 2,
}

/// Automatic download configuration for photos, videos, and files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoDownloadSettings {
    pub photo: bool,
    pub video: bool,
    pub file: bool,
    pub max_file_size_mb: i64,
}

impl Default for AutoDownloadSettings {
    fn default() -> Self {
        Self {
            photo: true,
            video: true,
            file: false,
            max_file_size_mb: 10,
        }
    }
}

/// Per-session configuration settings persisted to local storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MainSessionSettings {
    pub support_switch: i32,
    pub support_fix_chats_order: bool,
    pub support_templates_autocomplete: bool,
    pub support_chats_time_slice: i32,
    pub support_all_search_results: bool,
    pub support_all_silent: bool,
    pub selector_tab: SelectorTab,
    pub group_stickers_section_hidden: HashSet<u64>,
    pub group_emoji_section_hidden: HashSet<u64>,
    pub archive_collapsed: bool,
    pub archive_in_main_menu: bool,
    pub dialogs_filters_enabled: bool,
    pub auto_download: AutoDownloadSettings,
    pub hidden_pinned_messages: HashMap<u64, i64>,
    pub photo_editor_hint_shown: u32,
    pub disable_sharing_box_shown: u32,
}

impl Default for MainSessionSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl MainSessionSettings {
    /// Constructs default session settings.
    pub fn new() -> Self {
        Self {
            support_switch: 0,
            support_fix_chats_order: false,
            support_templates_autocomplete: true,
            support_chats_time_slice: 86400,
            support_all_search_results: false,
            support_all_silent: false,
            selector_tab: SelectorTab::Emoji,
            group_stickers_section_hidden: HashSet::new(),
            group_emoji_section_hidden: HashSet::new(),
            archive_collapsed: false,
            archive_in_main_menu: false,
            dialogs_filters_enabled: true,
            auto_download: AutoDownloadSettings::default(),
            hidden_pinned_messages: HashMap::new(),
            photo_editor_hint_shown: 0,
            disable_sharing_box_shown: 0,
        }
    }

    /// Serializes session settings into a JSON byte buffer.
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }

    /// Deserializes and merges session settings from a serialized byte slice.
    pub fn add_from_serialized(&mut self, serialized: &[u8]) -> Result<(), serde_json::Error> {
        let loaded: MainSessionSettings = serde_json::from_slice(serialized)?;
        *self = loaded;
        Ok(())
    }

    // Support flags
    pub fn set_support_switch(&mut self, value: i32) {
        self.support_switch = value;
    }

    pub fn set_support_fix_chats_order(&mut self, fix: bool) {
        self.support_fix_chats_order = fix;
    }

    pub fn set_support_templates_autocomplete(&mut self, enabled: bool) {
        self.support_templates_autocomplete = enabled;
    }

    pub fn set_support_chats_time_slice(&mut self, slice: i32) {
        self.support_chats_time_slice = slice;
    }

    pub fn set_support_all_search_results(&mut self, all: bool) {
        self.support_all_search_results = all;
    }

    pub fn set_support_all_silent(&mut self, enabled: bool) {
        self.support_all_silent = enabled;
    }

    // Selector tabs
    pub fn selector_tab(&self) -> SelectorTab {
        self.selector_tab
    }

    pub fn set_selector_tab(&mut self, tab: SelectorTab) {
        self.selector_tab = tab;
    }

    // Hidden sticker & emoji sections per peer
    pub fn set_group_stickers_section_hidden(&mut self, peer_id: u64) {
        self.group_stickers_section_hidden.insert(peer_id);
    }

    pub fn is_group_stickers_section_hidden(&self, peer_id: u64) -> bool {
        self.group_stickers_section_hidden.contains(&peer_id)
    }

    pub fn remove_group_stickers_section_hidden(&mut self, peer_id: u64) {
        self.group_stickers_section_hidden.remove(&peer_id);
    }

    pub fn set_group_emoji_section_hidden(&mut self, peer_id: u64) {
        self.group_emoji_section_hidden.insert(peer_id);
    }

    pub fn is_group_emoji_section_hidden(&self, peer_id: u64) -> bool {
        self.group_emoji_section_hidden.contains(&peer_id)
    }

    pub fn remove_group_emoji_section_hidden(&mut self, peer_id: u64) {
        self.group_emoji_section_hidden.remove(&peer_id);
    }

    // Archive display
    pub fn archive_collapsed(&self) -> bool {
        self.archive_collapsed
    }

    pub fn set_archive_collapsed(&mut self, collapsed: bool) {
        self.archive_collapsed = collapsed;
    }

    pub fn archive_in_main_menu(&self) -> bool {
        self.archive_in_main_menu
    }

    pub fn set_archive_in_main_menu(&mut self, in_main_menu: bool) {
        self.archive_in_main_menu = in_main_menu;
    }

    // Hidden pinned messages
    pub fn hidden_pinned_message_id(&self, peer_id: u64) -> Option<i64> {
        self.hidden_pinned_messages.get(&peer_id).copied()
    }

    pub fn set_hidden_pinned_message_id(&mut self, peer_id: u64, msg_id: i64) {
        self.hidden_pinned_messages.insert(peer_id, msg_id);
    }

    // Hints
    pub fn increment_photo_editor_hint_shown(&mut self) {
        self.photo_editor_hint_shown = self.photo_editor_hint_shown.saturating_add(1);
    }

    pub fn increment_disable_sharing_box_shown(&mut self) {
        self.disable_sharing_box_shown = self.disable_sharing_box_shown.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_settings_serialization() {
        let mut settings = MainSessionSettings::new();
        settings.set_archive_collapsed(true);
        settings.set_selector_tab(SelectorTab::Stickers);
        settings.set_group_stickers_section_hidden(123456);

        let data = settings.serialize();
        assert!(!data.is_empty());

        let mut loaded = MainSessionSettings::new();
        assert!(loaded.add_from_serialized(&data).is_ok());
        assert!(loaded.archive_collapsed());
        assert_eq!(loaded.selector_tab(), SelectorTab::Stickers);
        assert!(loaded.is_group_stickers_section_hidden(123456));
    }
}
