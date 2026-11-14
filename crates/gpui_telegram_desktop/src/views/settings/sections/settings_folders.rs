use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_empty_state, settings_section_header,
    settings_subsection_title, settings_toggle_row,
};

/// Upstream counterpart: `Settings::Folders` (`settings_folders.cpp`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatFolderItem {
    pub id: i32,
    pub title: String,
    pub icon_name: String,
    pub included_peers_count: usize,
    pub excluded_peers_count: usize,
    pub unread_count: usize,
    pub is_removed: bool,
}

pub struct SettingsFoldersSections {
    pub folders: Vec<ChatFolderItem>,
    pub tags_enabled: bool,
    pub selected_folder_id: Option<i32>,
    pub is_visible: bool,
    pub context_description: String,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_create_folder: bool,
}

impl SettingsFoldersSections {
    pub fn new() -> Self {
        Self {
            folders: vec![
                ChatFolderItem { id: 1, title: "Unread".into(), icon_name: "folder_unread".into(), included_peers_count: 15, excluded_peers_count: 2, unread_count: 15, is_removed: false },
                ChatFolderItem { id: 2, title: "Work".into(), icon_name: "folder_work".into(), included_peers_count: 8, excluded_peers_count: 0, unread_count: 3, is_removed: false },
                ChatFolderItem { id: 3, title: "Personal".into(), icon_name: "folder_personal".into(), included_peers_count: 12, excluded_peers_count: 1, unread_count: 0, is_removed: false },
            ],
            tags_enabled: false,
            selected_folder_id: Some(1),
            is_visible: false,
            context_description: "Create folders for different groups of chats and quickly switch between them.".into(),
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_create_folder: false,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        Self {
            folders: Vec::new(),
            tags_enabled: false,
            selected_folder_id: None,
            is_visible: true,
            context_description: tr_str(language_id, "为不同聊天分组创建文件夹，并在它们之间快速切换。", "Create folders for different groups of chats and quickly switch between them.").to_string(),
            palette,
            language_id: language_id.to_string(),
            pending_create_folder: false,
        }
    }

    pub fn sync_folders(&mut self, folders: Vec<ChatFolderItem>) {
        self.folders = folders;
        if self.selected_folder_id.is_none() {
            self.selected_folder_id = self.folders.iter().find(|f| !f.is_removed).map(|f| f.id);
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn text_context(&self) -> &str { &self.context_description }

    pub fn set_removed(&mut self, folder_id: i32) {
        if let Some(f) = self.folders.iter_mut().find(|f| f.id == folder_id) { f.is_removed = true; }
    }

    pub fn find(&self, folder_id: i32) -> Option<&ChatFolderItem> {
        self.folders.iter().find(|f| f.id == folder_id && !f.is_removed)
    }

    pub fn lng_filters_chats_count(&self, folder_id: i32) -> usize {
        self.find(folder_id).map(|f| f.included_peers_count).unwrap_or(0)
    }

    pub fn lng_filters_no_chats(&self, folder_id: i32) -> bool { self.lng_filters_chats_count(folder_id) == 0 }

    pub fn update_data(&mut self, folder_id: i32, new_title: String) {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) { folder.title = new_title; }
    }
}

impl Default for SettingsFoldersSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsFoldersSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let active: Vec<ChatFolderItem> = self.folders.iter().filter(|f| !f.is_removed).cloned().collect();
        let sel = self.selected_folder_id;

        settings_container(pal.clone())
            .child(settings_section_header(IconName::FolderAll, tr_str(&lang, "聊天文件夹", "Chat Folders"), Some(&self.context_description), pal.clone()))
            .child(
                div().flex().flex_row().justify_between().items_center()
                    .child(settings_subsection_title(tr_str(&lang, "文件夹", "Folders"), pal.clone()))
                    .child(
                        div().px(px(12.0)).py(px(6.0)).rounded(px(8.0)).bg(rgb(pal.accent)).text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.active_button_fg)).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                let next_id = this.folders.iter().map(|f| f.id).max().unwrap_or(0) + 1;
                                this.folders.push(ChatFolderItem { id: next_id, title: format!("Folder {}", next_id), icon_name: "folder".into(), included_peers_count: 0, excluded_peers_count: 0, unread_count: 0, is_removed: false });
                                this.selected_folder_id = Some(next_id);
                                this.pending_create_folder = true;
                                cx.notify();
                            }))
                            .child(format!("+ {}", tr_str(&lang, "创建新文件夹", "Create Folder"))),
                    ),
            )
            .child(if active.is_empty() {
                settings_empty_state(IconName::FolderAll, tr_str(&lang, "尚未创建任何文件夹", "No custom folders configured yet."), pal.clone()).into_any_element()
            } else {
                settings_card(pal.clone()).children(active.into_iter().map(|folder| {
                    let fid = folder.id;
                    let is_sel = sel == Some(fid);
                    div().flex().flex_row().justify_between().items_center().px(px(10.0)).py(px(8.0)).rounded(px(8.0))
                        .bg(rgb(if is_sel { pal.dialogs_hover_bg } else { pal.card_bg })).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.selected_folder_id = Some(fid); cx.notify(); }))
                        .child(div().flex().flex_row().items_center().gap(px(10.0))
                            .child(tg_icon(IconName::FolderChat, 16.0, pal.accent))
                            .child(div().flex().flex_col()
                                .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child(folder.title))
                                .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(format!("{} chats", folder.included_peers_count)))))
                        .child(div().flex().flex_row().items_center().gap(px(8.0))
                            .when(folder.unread_count > 0, |el| el.child(div().px(px(7.0)).py(px(2.0)).rounded_full().bg(rgb(pal.muted_badge_bg)).text_xs().text_color(rgb(pal.text_primary)).child(format!("{}", folder.unread_count))))
                            .child(div().p(px(4.0)).rounded(px(6.0)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.set_removed(fid); cx.notify(); })).child(tg_icon(IconName::DeleteTrash, 14.0, pal.danger))))
                })).into_any_element()
            })
            .child(settings_card(pal.clone()).child(settings_toggle_row(IconName::FolderWork, tr_str(&lang, "显示文件夹标签", "Show Folder Tags"), Some("Display folder names on chat list items"), self.tags_enabled, pal, |this| this.tags_enabled = !this.tags_enabled, cx)))
    }
}
