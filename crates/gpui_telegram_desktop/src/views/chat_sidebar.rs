//! 288px Chat Sidebar view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.3).
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_widget.cpp:L112-230`
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp:L116-260, L5469-5650`
//! - `Telegram/SourceFiles/dialogs/dialogs.style:L14-220`
//!
//! Sub-components:
//! 1. Universal Search Header (48px, rounded 16px search input, Cmd+K shortcut, menu button).
//! 2. Suggestions Banner (37px height, priority queue active suggestion).
//! 3. Stories Strip (horizontal carousel, 3 small preview thumbnails).
//! 4. Top Frequently Contacted Peers Strip.
//! 5. Folder Tabs Strip (36px, horizontal chips: All, Personal, Work, Bots, Unread, with 2px accent underline and unread count badges).
//! 6. Virtualized Dialog List:
//!    - Row height: exactly 62px (`dialogsRowHeight: 62px`).
//!    - Userpic avatar (46px diameter) at point(10px, 8px).
//!    - Peer name (14px bold) at nameLeft: 68px.
//!    - Timestamp (12px muted) right-aligned.
//!    - Preview message (13px regular) at textLeft: 68px.
//!    - Delivery status icons: Sending (clock), Sent (single check), Read (double check).
//!    - Unread count pill badge (height 19px, background #40a7e3 / #6b808d).
//!    - Mute icon and Pinned chat pin icon.
//!    - Typing indicator ("Alice is typing...").
//!
//! State mutations/actions live in `views::sidebar::sidebar_state_ops` and
//! GPUI `Render` lives in `views::sidebar::sidebar_main_render`.

use gpui::{Context, Entity, Pixels, Point};
use serde::{Deserialize, Serialize};

use crate::theme::dialogs::DialogsStyleSheet;
use crate::theme::TdesktopThemePalette;
use crate::views::nav_rail::{NavDestination, NavRail};
use crate::views::sidebar::dialogs_inner_widget::DialogsInnerWidget;
use crate::views::sidebar::dialogs_row_view::SidebarContextAction;
use crate::views::sidebar::dialogs_search_tags::DialogsSearchTags;
use crate::views::sidebar::dialogs_widget::DialogsWidget;
use crate::views::sidebar::suggestions::suggestion_manager::SuggestionManager;
use crate::views::sidebar::ui::chat_search_in::ChatSearchInUi;
use crate::views::sidebar::ui::dialogs_message_view::DialogMessageMediaBadge;
use crate::views::sidebar::ui::dialogs_stories_list::DialogsStoriesListUi;
use crate::views::sidebar::ui::dialogs_suggestions::DialogsSuggestionsUi;
use crate::views::sidebar::ui::dialogs_topics_view::DialogsTopicsViewUi;
use crate::views::sidebar::ui::top_peers_strip::TopPeersStripUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogDeliveryStatus {
    Sending,
    Sent,
    Read,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarCreationModal {
    AddContact,
    CreateGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddContactKind {
    AiBot,
    Friend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreationInputField {
    PrimaryName,
    SecondaryHandle,
    TertiaryPromptOrPhone,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BotGrokAvatarVisual {
    pub agent_id: String,
    pub shape: String,
    pub color: String,
    pub execution_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingAddContactRequest {
    pub local_dialog_id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub is_bot: bool,
    pub system_prompt: Option<String>,
    pub avatar_shape: Option<String>,
    pub avatar_color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingCreateGroupRequest {
    pub local_dialog_id: u64,
    pub title: String,
    pub member_ids: Vec<u64>,
    pub member_usernames: Vec<String>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategorizedSearchResult {
    pub chat_id: u64,
    pub message_id: u64,
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub timestamp: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub is_bot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SidebarCatalogAppItem {
    pub bot_id: u64,
    pub bot_username: String,
    pub title: String,
    pub short_name: String,
    pub description: String,
    pub category: String,
    pub icon_initials: String,
    pub avatar_color: u32,
    pub installs_count: u64,
    pub mau_count: u64,
    pub is_verified: bool,
    pub is_installed: bool,
    pub is_mine: bool,
    pub webview_url: String,
    pub companion_chat_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogItemModel {
    pub id: u64,
    pub peer_name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub last_message: String,
    pub timestamp: String,
    pub unread_count: u32,
    pub is_muted: bool,
    pub is_pinned: bool,
    pub is_online: bool,
    pub typing_indicator: Option<String>,
    pub delivery_status: DialogDeliveryStatus,
    pub folder: String,
}

pub const DEFAULT_WIDTH: f32 = 288.0;
pub const DEFAULT_SIDEBAR_WIDTH: f32 = DEFAULT_WIDTH;
pub const COLLAPSED_WIDTH: f32 = 64.0;
pub const COLLAPSED_SIDEBAR_WIDTH: f32 = COLLAPSED_WIDTH;
pub const MIN_WIDTH: f32 = 240.0;
pub const MIN_SIDEBAR_WIDTH: f32 = MIN_WIDTH;
pub const MAX_WIDTH: f32 = 360.0;
pub const MAX_SIDEBAR_WIDTH: f32 = MAX_WIDTH;
pub const ROW_HEIGHT: f32 = 62.0;
pub const DIALOG_ROW_HEIGHT: f32 = ROW_HEIGHT;
pub const AVATAR_SIZE: f32 = 46.0;
pub const DIALOG_AVATAR_SIZE: f32 = AVATAR_SIZE;
pub const UNREAD_HEIGHT: f32 = 19.0;
pub const DIALOG_UNREAD_HEIGHT: f32 = UNREAD_HEIGHT;
pub const DIALOG_UNREAD_PADDING: f32 = 5.0;
pub const SEARCH_HEIGHT: f32 = 35.0;
pub const SEARCH_BAR_HEIGHT: f32 = SEARCH_HEIGHT;
pub const SEARCH_RADIUS: f32 = 18.0;
pub const SEARCH_BAR_RADIUS: f32 = SEARCH_RADIUS;
pub const SEARCH_HEADER_HEIGHT: f32 = 48.0;
pub const DIALOG_NAME_LEFT: f32 = 68.0;

pub const TELEGRAM_SEARCH_CATEGORIES: &[(&str, &str)] = &[
    ("Chats", "聊天"),
    ("Channels", "频道"),
    ("Apps", "应用"),
    ("Posts", "贴文"),
    ("Media", "图片"),
    ("Downloads", "视频"),
    ("Links", "链接"),
    ("Files", "文件"),
    ("Music", "音乐"),
    ("Voice", "语音"),
];

pub const TELEGRAM_SEARCH_TAGS: &[(&str, &str)] = &[
    ("all", "全部"),
    ("bot", "AI Bot"),
    ("unread", "未读"),
    ("pinned", "置顶"),
    ("media", "媒体"),
    ("links", "链接"),
];

pub struct ChatSidebar {
    pub width: f32,
    pub is_resizing: bool,
    pub resize_drag_start_x: Option<f32>,
    pub resize_start_width: f32,
    pub search_query: String,
    pub active_folder: String,
    pub active_chat_id: Option<u64>,
    pub selection_revision: u64,
    pub dialogs: Vec<DialogItemModel>,
    pub folders: Vec<(String, u32)>,
    pub search_focused: bool,
    pub story_clicked_peer: Option<String>,

    // `+` button next to search box & Add Contact (Create AI Bot / Add Friend) / Create Group / Contacts modal state
    pub is_plus_menu_open: bool,
    pub is_contacts_modal_open: bool,
    pub contacts_search_query: String,
    pub active_creation_modal: Option<SidebarCreationModal>,
    pub contact_kind: AddContactKind,
    pub active_creation_field: CreationInputField,
    pub creation_primary_input: String,
    pub creation_secondary_input: String,
    pub creation_tertiary_input: String,
    pub creation_avatar_shape: String,
    pub creation_avatar_color: String,
    pub creation_use_dynamic_avatar: bool,
    pub pending_add_contact_request: Option<PendingAddContactRequest>,
    pub pending_create_group_request: Option<PendingCreateGroupRequest>,
    pub pending_open_new_channel: bool,
    pub next_local_dialog_id: u64,

    // Telegram 1:1 Categorized Search state when search box is clicked
    pub active_search_category: String,
    pub active_search_tag: String,
    pub remote_search_results: Vec<CategorizedSearchResult>,
    pub catalog_mini_apps: Vec<SidebarCatalogAppItem>,
    pub recent_mini_apps: Vec<SidebarCatalogAppItem>,
    pub pending_open_botfather: bool,
    pub pending_install_open_miniapp: Option<SidebarCatalogAppItem>,
    pub pending_open_miniapp_only: Option<SidebarCatalogAppItem>,
    pub pending_open_bot_profile: Option<String>,
    pub bot_main_app_uris: std::collections::HashMap<u64, String>,
    pub pending_launch_dialog_miniapp: Option<(u64, String)>,
    pub pending_open_search_hit: Option<CategorizedSearchResult>,
    pub search_request_seq: u64,
    pub search_in_ui: ChatSearchInUi,
    pub search_tags_ui: DialogsSearchTags,

    // Authentic submodules & visual parity state
    pub widget: DialogsWidget,
    pub inner_widget: DialogsInnerWidget,
    pub stories: DialogsStoriesListUi,
    pub top_peers: TopPeersStripUi,
    pub suggestions: DialogsSuggestionsUi,
    pub suggestion_manager: SuggestionManager,
    pub style: DialogsStyleSheet,
    pub drafts: std::collections::HashMap<u64, String>,
    pub archived_ids: Vec<u64>,
    pub is_archived_expanded: bool,
    pub unread_marked_ids: std::collections::HashSet<u64>,
    pub verified_ids: std::collections::HashSet<u64>,
    pub premium_ids: std::collections::HashSet<u64>,
    pub bot_ids: std::collections::HashSet<u64>,
    pub bot_fabushi_visuals: std::collections::HashMap<u64, (String, u32)>,
    pub bot_grok_avatars: std::collections::HashMap<u64, BotGrokAvatarVisual>,
    pub contact_chat_ids: std::collections::HashSet<u64>,
    pub group_chat_ids: std::collections::HashSet<u64>,
    pub channel_chat_ids: std::collections::HashSet<u64>,
    pub custom_folders: Vec<gpui_telegram_proto::contracts::chats::ChatFolderDto>,
    pub folder_tags_enabled: bool,
    pub include_muted_in_badge: bool,
    pub badge_count_messages: bool,
    pub saved_messages_id: Option<u64>,
    pub media_badges: std::collections::HashMap<u64, DialogMessageMediaBadge>,
    pub forum_topics: std::collections::HashMap<u64, DialogsTopicsViewUi>,
    pub expanded_forum_id: Option<u64>,
    pub selected_topic_id: Option<(u64, u64)>,
    pub context_menu: Option<(u64, Point<Pixels>)>,
    pub pending_context_action: Option<SidebarContextAction>,
    pub open_main_menu_requested: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub is_collapsed: bool,
    pub collapse_state_revision: u64,
    pub is_nav_more_expanded: bool,
    pub is_account_popover_open: bool,
    pub hovered_rail_item: Option<&'static str>,
    pub hovered_rail_dialog_id: Option<u64>,
    pub nav_rail: Option<Entity<NavRail>>,
    pub standalone_active_destination: NavDestination,
}

impl ChatSidebar {
    pub fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            is_resizing: false,
            resize_drag_start_x: None,
            resize_start_width: DEFAULT_WIDTH,
            search_query: String::new(),
            active_folder: "All".to_string(),
            active_chat_id: None,
            selection_revision: 0,
            dialogs: Vec::new(),
            folders: vec![("All".to_string(), 0)],
            search_focused: false,
            story_clicked_peer: None,
            is_plus_menu_open: false,
            is_contacts_modal_open: false,
            contacts_search_query: String::new(),
            active_creation_modal: None,
            contact_kind: AddContactKind::AiBot,
            active_creation_field: CreationInputField::PrimaryName,
            creation_primary_input: String::new(),
            creation_secondary_input: String::new(),
            creation_tertiary_input: String::new(),
            creation_avatar_shape: "blob".to_string(),
            creation_avatar_color: "blue".to_string(),
            creation_use_dynamic_avatar: true,
            pending_add_contact_request: None,
            pending_create_group_request: None,
            pending_open_new_channel: false,
            next_local_dialog_id: 8000,
            active_search_category: "Chats".to_string(),
            active_search_tag: "all".to_string(),
            remote_search_results: Vec::new(),
            catalog_mini_apps: Vec::new(),
            recent_mini_apps: Vec::new(),
            pending_open_botfather: false,
            pending_install_open_miniapp: None,
            pending_open_miniapp_only: None,
            pending_open_bot_profile: None,
            bot_main_app_uris: std::collections::HashMap::new(),
            pending_launch_dialog_miniapp: None,
            pending_open_search_hit: None,
            search_request_seq: 0,
            search_in_ui: ChatSearchInUi::new(),
            search_tags_ui: DialogsSearchTags::new(),
            widget: DialogsWidget::new(),
            inner_widget: DialogsInnerWidget::new(),
            stories: DialogsStoriesListUi::new(),
            top_peers: TopPeersStripUi::new(),
            suggestions: DialogsSuggestionsUi::new(),
            suggestion_manager: SuggestionManager::new(),
            style: DialogsStyleSheet::new(),
            drafts: std::collections::HashMap::new(),
            archived_ids: Vec::new(),
            is_archived_expanded: false,
            unread_marked_ids: std::collections::HashSet::new(),
            verified_ids: std::collections::HashSet::new(),
            premium_ids: std::collections::HashSet::new(),
            bot_ids: std::collections::HashSet::new(),
            bot_fabushi_visuals: std::collections::HashMap::new(),
            bot_grok_avatars: std::collections::HashMap::new(),
            contact_chat_ids: std::collections::HashSet::new(),
            group_chat_ids: std::collections::HashSet::new(),
            channel_chat_ids: std::collections::HashSet::new(),
            custom_folders: Vec::new(),
            folder_tags_enabled: false,
            include_muted_in_badge: false,
            badge_count_messages: true,
            saved_messages_id: None,
            media_badges: std::collections::HashMap::new(),
            forum_topics: std::collections::HashMap::new(),
            expanded_forum_id: None,
            selected_topic_id: None,
            context_menu: None,
            pending_context_action: None,
            open_main_menu_requested: false,
            palette: TdesktopThemePalette::default(),
            language_id: "zh-CN".to_string(),
            is_collapsed: false,
            collapse_state_revision: 0,
            is_nav_more_expanded: false,
            is_account_popover_open: false,
            hovered_rail_item: None,
            hovered_rail_dialog_id: None,
            nav_rail: None,
            standalone_active_destination: NavDestination::Chats,
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(MIN_WIDTH, MAX_WIDTH);
    }

    pub fn reset_width(&mut self) {
        self.width = DEFAULT_WIDTH;
    }

    pub fn attach_nav_rail(&mut self, nav_rail: Entity<NavRail>, cx: &mut Context<Self>) {
        self.nav_rail = Some(nav_rail.clone());
        cx.observe(&nav_rail, |_, _, cx| cx.notify()).detach();
    }

    pub fn effective_width(&self) -> f32 {
        if self.is_collapsed {
            COLLAPSED_WIDTH
        } else {
            self.width
        }
    }

    pub fn set_collapsed(&mut self, collapsed: bool) {
        self.is_collapsed = collapsed;
        self.hovered_rail_item = None;
        self.hovered_rail_dialog_id = None;
        self.is_account_popover_open = false;
        if collapsed {
            self.is_plus_menu_open = false;
            self.context_menu = None;
            self.is_resizing = false;
        }
    }

    pub fn toggle_collapsed_from_user(&mut self) {
        self.set_collapsed(!self.is_collapsed);
        self.collapse_state_revision = self.collapse_state_revision.wrapping_add(1);
    }

    pub(crate) fn expand_from_user_if_collapsed(&mut self) {
        if self.is_collapsed {
            self.set_collapsed(false);
            self.collapse_state_revision = self.collapse_state_revision.wrapping_add(1);
        }
    }

    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str) {
        self.is_resizing = false;
        self.resize_drag_start_x = None;
        self.folders = vec![("All".to_string(), 0)];
        self.dialogs.clear();
        self.search_query.clear();
        self.search_focused = false;
        self.story_clicked_peer = None;
        self.is_plus_menu_open = false;
        self.is_account_popover_open = false;
        self.hovered_rail_dialog_id = None;
        self.is_contacts_modal_open = false;
        self.active_creation_modal = None;
        self.remote_search_results.clear();
        self.catalog_mini_apps.clear();
        self.recent_mini_apps.clear();
        self.pending_open_botfather = false;
        self.pending_install_open_miniapp = None;
        self.pending_open_miniapp_only = None;
        self.pending_open_bot_profile = None;
        self.bot_main_app_uris.clear();
        self.pending_launch_dialog_miniapp = None;
        self.pending_open_search_hit = None;
        self.search_in_ui.clear_scope();
        self.search_tags_ui.clear_selection();
        self.widget = DialogsWidget::new();
        self.inner_widget = DialogsInnerWidget::new();
        self.stories = DialogsStoriesListUi::new();
        self.top_peers = TopPeersStripUi::new();
        self.suggestion_manager = SuggestionManager::new();
        self.suggestions = DialogsSuggestionsUi::new();
        self.drafts.clear();
        self.archived_ids.clear();
        self.is_archived_expanded = false;
        self.unread_marked_ids.clear();
        self.verified_ids.clear();
        self.premium_ids.clear();
        self.bot_ids.clear();
        self.saved_messages_id = None;
        self.media_badges.clear();
        self.forum_topics.clear();
        self.expanded_forum_id = None;
        self.selected_topic_id = None;
        self.context_menu = None;
        self.pending_context_action = None;
        self.active_folder = "All".to_string();
        self.active_chat_id = None;
    }

    pub fn open_contacts_modal(&mut self) {
        self.is_plus_menu_open = false;
        self.is_account_popover_open = false;
        self.search_focused = false;
        self.is_contacts_modal_open = true;
        self.contacts_search_query.clear();
    }

    pub fn close_contacts_modal(&mut self) {
        self.is_contacts_modal_open = false;
        self.contacts_search_query.clear();
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        let q = query.into();
        self.search_query = q.clone();
        self.widget.on_search_text_changed(q);
        self.search_request_seq = self.search_request_seq.wrapping_add(1);
    }

    pub fn focus_search(&mut self) {
        self.expand_from_user_if_collapsed();
        self.search_focused = true;
        self.is_plus_menu_open = false;
        self.is_account_popover_open = false;
        self.search_request_seq = self.search_request_seq.wrapping_add(1);
    }

    pub fn exit_search(&mut self) {
        self.search_focused = false;
        self.search_query.clear();
        self.active_search_category = "Chats".to_string();
        self.active_search_tag = "all".to_string();
        self.widget.on_search_text_changed(String::new());
    }

    pub fn select_search_category(&mut self, category: impl Into<String>) {
        self.active_search_category = category.into();
        self.search_request_seq = self.search_request_seq.wrapping_add(1);
    }

    pub fn open_apps_center(&mut self) {
        self.focus_search();
        self.search_query.clear();
        self.active_search_category = "Apps".to_string();
        self.search_request_seq = self.search_request_seq.wrapping_add(1);
    }

    pub fn set_catalog_mini_apps(&mut self, apps: Vec<SidebarCatalogAppItem>) {
        self.catalog_mini_apps = apps;
    }

    pub fn set_recent_mini_apps(&mut self, apps: Vec<SidebarCatalogAppItem>) {
        self.recent_mini_apps = apps;
    }

    pub fn set_bot_main_app_uris(&mut self, uris: std::collections::HashMap<u64, String>) {
        self.bot_main_app_uris = uris;
    }

    pub fn select_search_tag(&mut self, tag: impl Into<String>) {
        let t = tag.into();
        if self.active_search_tag == t {
            self.active_search_tag = "all".to_string();
            self.search_tags_ui.clear_selection();
        } else {
            self.active_search_tag = t.clone();
            self.search_tags_ui.toggle_tag(&t);
        }
        self.search_request_seq = self.search_request_seq.wrapping_add(1);
    }

    pub fn toggle_plus_menu(&mut self) {
        self.is_account_popover_open = false;
        if self.is_collapsed {
            self.expand_from_user_if_collapsed();
            self.is_plus_menu_open = true;
            self.search_focused = false;
            return;
        }
        self.is_plus_menu_open = !self.is_plus_menu_open;
        if self.is_plus_menu_open {
            self.search_focused = false;
        }
    }

    pub fn select_folder(&mut self, folder: impl Into<String>) {
        self.active_folder = folder.into();
    }

    pub fn select_dialog(&mut self, id: u64) {
        if let Some(idx) = self.dialogs.iter().position(|d| d.id == id) {
            self.active_chat_id = Some(id);
            self.inner_widget.selected_index = Some(idx);
            self.unread_marked_ids.remove(&id);
            self.mark_as_read(id);
        }
    }

    pub fn select_dialog_from_user(&mut self, id: u64) {
        self.select_dialog(id);
        if self.active_chat_id == Some(id) {
            self.selection_revision = self.selection_revision.wrapping_add(1);
        }
    }

    pub fn clear_selection(&mut self) {
        self.active_chat_id = None;
        self.inner_widget.selected_index = None;
    }

    pub fn set_typing(&mut self, id: u64, typing: Option<String>) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.typing_indicator = typing;
        }
    }

    pub fn set_unread(&mut self, id: u64, unread: u32) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.unread_count = unread;
        }
    }

    pub fn mark_as_read(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.unread_count = 0;
            d.delivery_status = DialogDeliveryStatus::Read;
        }
    }

    pub fn selected_dialog(&self) -> Option<&DialogItemModel> {
        self.active_chat_id
            .and_then(|id| self.dialogs.iter().find(|d| d.id == id))
    }

    pub fn set_draft(&mut self, id: u64, draft_text: String) {
        if draft_text.trim().is_empty() {
            self.drafts.remove(&id);
        } else {
            self.drafts.insert(id, draft_text);
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang_id: impl Into<String>) {
        self.language_id = lang_id.into();
    }

    pub fn take_open_main_menu_requested(&mut self) -> bool {
        let was = self.open_main_menu_requested;
        self.open_main_menu_requested = false;
        was
    }

    pub fn take_pending_context_action(&mut self) -> Option<SidebarContextAction> {
        self.pending_context_action.take()
    }

    pub fn take_pending_open_new_channel(&mut self) -> bool {
        std::mem::take(&mut self.pending_open_new_channel)
    }

    pub fn set_forum_topics(&mut self, topics_ui: DialogsTopicsViewUi) {
        self.forum_topics.insert(topics_ui.forum_id, topics_ui);
    }

    pub fn toggle_forum_topics(&mut self, forum_id: u64) {
        if self.expanded_forum_id == Some(forum_id) {
            self.expanded_forum_id = None;
        } else if self.forum_topics.contains_key(&forum_id) {
            self.expanded_forum_id = Some(forum_id);
        }
    }
}

impl Default for ChatSidebar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[std::prelude::v1::test]
    fn selection_revision_increments_only_on_user_selection() {
        let mut sb = ChatSidebar::new();
        assert_eq!(sb.selection_revision, 0);

        sb.dialogs.push(DialogItemModel {
            id: 42,
            peer_name: "Alice".to_string(),
            avatar_initials: "AL".to_string(),
            avatar_color: 0x2AABEE,
            last_message: String::new(),
            timestamp: String::new(),
            unread_count: 1,
            is_muted: false,
            is_pinned: false,
            is_online: false,
            typing_indicator: None,
            delivery_status: DialogDeliveryStatus::Read,
            folder: "All".to_string(),
        });

        // Programmatic/sync selection and non-selection mutations must not bump revision
        sb.select_dialog(42);
        sb.set_search_query("al");
        sb.select_folder("All");
        sb.set_typing(42, Some("typing".to_string()));
        sb.set_unread(42, 3);
        sb.recompute_folder_badges();
        assert_eq!(sb.selection_revision, 0);

        // Selecting an unknown dialog does not bump revision
        sb.select_dialog_from_user(999);
        assert_eq!(sb.selection_revision, 0);

        // Explicit user click increments revision even when re-clicking active dialog
        sb.select_dialog_from_user(42);
        assert_eq!(sb.selection_revision, 1);
        sb.select_dialog_from_user(42);
        assert_eq!(sb.selection_revision, 2);
    }

    #[std::prelude::v1::test]
    fn unified_sidebar_collapse_and_expand_geometry_and_revisions() {
        let mut sb = ChatSidebar::new();
        assert!(!sb.is_collapsed);
        assert_eq!(sb.collapse_state_revision, 0);
        assert!((sb.effective_width() - DEFAULT_WIDTH).abs() < f32::EPSILON);

        // Programmatic collapse does not bump user collapse revision
        sb.set_collapsed(true);
        assert!(sb.is_collapsed);
        assert_eq!(sb.collapse_state_revision, 0);
        assert!((sb.effective_width() - COLLAPSED_WIDTH).abs() < f32::EPSILON);

        // User toggle expands and bumps revision
        sb.toggle_collapsed_from_user();
        assert!(!sb.is_collapsed);
        assert_eq!(sb.collapse_state_revision, 1);
        assert!((sb.effective_width() - DEFAULT_WIDTH).abs() < f32::EPSILON);

        // Focusing search from collapsed state auto-expands and bumps revision
        sb.toggle_collapsed_from_user();
        assert!(sb.is_collapsed);
        assert_eq!(sb.collapse_state_revision, 2);
        sb.focus_search();
        assert!(!sb.is_collapsed);
        assert!(sb.search_focused);
        assert_eq!(sb.collapse_state_revision, 3);

        // Opening compose menu from collapsed state auto-expands and bumps revision
        sb.toggle_collapsed_from_user();
        assert!(sb.is_collapsed);
        assert_eq!(sb.collapse_state_revision, 4);
        sb.toggle_plus_menu();
        assert!(!sb.is_collapsed);
        assert!(sb.is_plus_menu_open);
        assert_eq!(sb.collapse_state_revision, 5);
    }

    #[std::prelude::v1::test]
    fn collapsed_rail_dialog_selection_keeps_rail_collapsed_and_popover_toggles() {
        let mut sb = ChatSidebar::new();
        sb.dialogs.push(DialogItemModel {
            id: 77,
            peer_name: "Big Decision".to_string(),
            avatar_initials: "BD".to_string(),
            avatar_color: 0x5092E6,
            last_message: "hi".to_string(),
            timestamp: "now".to_string(),
            unread_count: 2,
            is_muted: false,
            is_pinned: false,
            is_online: true,
            typing_indicator: None,
            delivery_status: DialogDeliveryStatus::Read,
            folder: "All".to_string(),
        });
        sb.set_collapsed(true);
        assert!(sb.is_collapsed);

        // Selecting a dialog on the collapsed rail updates active chat without expanding
        sb.select_dialog_from_user(77);
        assert_eq!(sb.active_chat_id, Some(77));
        assert!(sb.is_collapsed);

        // Account popover toggles cleanly without requesting legacy MainMenuDrawer
        assert!(!sb.is_account_popover_open);
        sb.is_account_popover_open = !sb.is_account_popover_open;
        assert!(sb.is_account_popover_open);
        assert!(!sb.open_main_menu_requested);
        sb.set_collapsed(false);
        assert!(!sb.is_account_popover_open);
    }
}
