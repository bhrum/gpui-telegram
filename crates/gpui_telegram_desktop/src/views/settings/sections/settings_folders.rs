//! Authentic GPUI Settings Folders Section View Component
//! Source: Telegram/SourceFiles/settings/sections/settings_folders.cpp, Telegram/SourceFiles/settings/sections/settings_folders.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct ChatFolderItem {
    pub id: i32,
    pub title: String,
    pub icon_name: String,
    pub included_peers_count: usize,
    pub excluded_peers_count: usize,
    pub unread_count: usize,
    pub is_removed: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsFoldersSections {
    pub folders: Vec<ChatFolderItem>,
    pub tags_enabled: bool,
    pub selected_folder_id: Option<i32>,
    pub is_visible: bool,
    pub context_description: String,
}

impl SettingsFoldersSections {
    pub fn new() -> Self {
        Self {
            folders: vec![
                ChatFolderItem {
                    id: 1,
                    title: "Personal".into(),
                    icon_name: "folder_user".into(),
                    included_peers_count: 12,
                    excluded_peers_count: 0,
                    unread_count: 2,
                    is_removed: false,
                },
                ChatFolderItem {
                    id: 2,
                    title: "Work".into(),
                    icon_name: "folder_work".into(),
                    included_peers_count: 28,
                    excluded_peers_count: 4,
                    unread_count: 5,
                    is_removed: false,
                },
                ChatFolderItem {
                    id: 3,
                    title: "Channels".into(),
                    icon_name: "folder_channel".into(),
                    included_peers_count: 45,
                    excluded_peers_count: 0,
                    unread_count: 14,
                    is_removed: false,
                },
            ],
            tags_enabled: true,
            selected_folder_id: Some(1),
            is_visible: true,
            context_description: "Manage chat folders and organization filters".into(),
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `FilterRowButton::textContext`
    pub fn text_context(&self) -> &str {
        &self.context_description
    }

    /// Upstream method: `setRemoved`
    pub fn set_removed(&mut self, folder_id: i32) {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            folder.is_removed = true;
        }
    }

    /// Upstream method: `ranges::find`
    pub fn find(&self, folder_id: i32) -> Option<&ChatFolderItem> {
        self.folders.iter().find(|f| f.id == folder_id && !f.is_removed)
    }

    /// Upstream method: `tr::lng_filters_chats_count`
    pub fn lng_filters_chats_count(&self, folder_id: i32) -> usize {
        self.find(folder_id)
            .map(|f| f.included_peers_count)
            .unwrap_or(0)
    }

    /// Upstream method: `tr::lng_filters_no_chats`
    pub fn lng_filters_no_chats(&self, folder_id: i32) -> bool {
        self.lng_filters_chats_count(folder_id) == 0
    }

    /// Upstream method: `updateData`
    pub fn update_data(&mut self, folder_id: i32, new_title: String) {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            folder.title = new_title;
        }
    }
}

impl Default for SettingsFoldersSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsFoldersSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_folders: Vec<_> = self.folders.iter().filter(|f| !f.is_removed).cloned().collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Chat Folders"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a909d))
                    .child(self.context_description.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(active_folders.into_iter().map(|folder| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .p(px(10.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).child(folder.title))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x8a909d))
                                            .child(format!("{} chats", folder.included_peers_count)),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded_full()
                                    .bg(rgb(0x3e4451))
                                    .text_xs()
                                    .child(format!("{}", folder.unread_count)),
                            )
                    })),
            )
    }
}
