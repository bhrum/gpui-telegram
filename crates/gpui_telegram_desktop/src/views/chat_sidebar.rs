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

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use crate::i18n::tr_str;
use crate::theme::dialogs::DialogsStyleSheet;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::sidebar::dialogs_inner_widget::DialogsInnerWidget;
use crate::views::sidebar::dialogs_overlays::{
    render_dialog_context_menu, render_folder_tabs_strip, render_forum_topics_subtree,
};
use crate::views::sidebar::dialogs_renderer::{
    render_archived_header_row, render_plus_dropdown, render_sidebar_header,
    render_stories_strip, render_suggestion_bar,
};
use crate::views::sidebar::dialogs_row_view::{
    render_dialog_row_item, SidebarContextAction, SidebarDialogMeta,
};
use crate::views::sidebar::dialogs_search_tags::DialogsSearchTags;
use crate::views::sidebar::dialogs_widget::DialogsWidget;
use crate::views::sidebar::sidebar_search_view::{
    render_search_panel, render_search_results_list,
};
use crate::views::sidebar::suggestions::suggestion_manager::{
    SuggestionItem, SuggestionManager, SuggestionPriority,
};
use crate::views::sidebar::ui::chat_search_in::ChatSearchInUi;
use crate::views::sidebar::ui::dialogs_message_view::DialogMessageMediaBadge;
use crate::views::sidebar::ui::dialogs_stories_list::{DialogsStoriesListUi, StoryElement};
use crate::views::sidebar::ui::dialogs_suggestions::DialogsSuggestionsUi;
use crate::views::sidebar::ui::dialogs_topics_view::DialogsTopicsViewUi;
use crate::views::sidebar::ui::top_peers_strip::{TopPeerItem, TopPeersStripUi};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingAddContactRequest {
    pub local_dialog_id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub is_bot: bool,
    pub system_prompt: Option<String>,
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
    pub pending_add_contact_request: Option<PendingAddContactRequest>,
    pub pending_create_group_request: Option<PendingCreateGroupRequest>,
    pub next_local_dialog_id: u64,

    // Telegram 1:1 Categorized Search state when search box is clicked
    pub active_search_category: String,
    pub active_search_tag: String,
    pub remote_search_results: Vec<CategorizedSearchResult>,
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
            pending_add_contact_request: None,
            pending_create_group_request: None,
            next_local_dialog_id: 8000,
            active_search_category: "Chats".to_string(),
            active_search_tag: "all".to_string(),
            remote_search_results: Vec::new(),
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
        }
    }

    pub fn new_demo() -> Self {
        let mut sidebar = Self::new();
        sidebar.folders = vec![
            ("All".to_string(), 8),
            ("Personal".to_string(), 3),
            ("Work".to_string(), 5),
            ("Bots".to_string(), 0),
            ("Unread".to_string(), 8),
        ];
        sidebar.dialogs = vec![
            DialogItemModel {
                id: 1,
                peer_name: "Alice Smith".to_string(),
                avatar_initials: "AS".to_string(),
                avatar_color: 0x2AABEE,
                last_message: "Are we still on for the design review today?".to_string(),
                timestamp: "14:32".to_string(),
                unread_count: 2,
                is_muted: false,
                is_pinned: true,
                is_online: true,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Read,
                folder: "Personal".to_string(),
            },
            DialogItemModel {
                id: 2,
                peer_name: "Rust Core Devs".to_string(),
                avatar_initials: "RC".to_string(),
                avatar_color: 0x9B59B6,
                last_message: "Bob: SumTree virtual list benchmark passed with 0 lag!".to_string(),
                timestamp: "14:15".to_string(),
                unread_count: 5,
                is_muted: false,
                is_pinned: true,
                is_online: false,
                typing_indicator: Some("Bob is typing...".to_string()),
                delivery_status: DialogDeliveryStatus::Read,
                folder: "Work".to_string(),
            },
            DialogItemModel {
                id: 3,
                peer_name: "Saved Messages".to_string(),
                avatar_initials: "SM".to_string(),
                avatar_color: 0x3498DB,
                last_message: "Voice note backup and deployment notes".to_string(),
                timestamp: "12:05".to_string(),
                unread_count: 0,
                is_muted: true,
                is_pinned: true,
                is_online: false,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Read,
                folder: "All".to_string(),
            },
            DialogItemModel {
                id: 4,
                peer_name: "Autonomous Agent Bot".to_string(),
                avatar_initials: "AB".to_string(),
                avatar_color: 0x1ABC9C,
                last_message: "Task Center: Background build finished with status 0".to_string(),
                timestamp: "10:48".to_string(),
                unread_count: 1,
                is_muted: false,
                is_pinned: false,
                is_online: true,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Sent,
                folder: "Bots".to_string(),
            },
            DialogItemModel {
                id: 5,
                peer_name: "Telegram Announcements".to_string(),
                avatar_initials: "TA".to_string(),
                avatar_color: 0xE67E22,
                last_message: "Version 1.0 is here! Featuring Talk Mode and Remote Computer control.".to_string(),
                timestamp: "Yesterday".to_string(),
                unread_count: 0,
                is_muted: true,
                is_pinned: false,
                is_online: false,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Read,
                folder: "All".to_string(),
            },
        ];
        sidebar.saved_messages_id = Some(3);
        sidebar.bot_ids.insert(4);
        sidebar.verified_ids.insert(5);
        sidebar.stories.set_elements(vec![
            StoryElement {
                id: 1,
                peer_name: "Alice Smith".to_string(),
                avatar_initials: "AS".to_string(),
                avatar_color: 0x2AABEE,
                story_count: 2,
                unread_count: 1,
                has_video_stream: false,
            },
            StoryElement {
                id: 2,
                peer_name: "Rust Core Devs".to_string(),
                avatar_initials: "RC".to_string(),
                avatar_color: 0x9B59B6,
                story_count: 1,
                unread_count: 1,
                has_video_stream: true,
            },
            StoryElement {
                id: 4,
                peer_name: "Autonomous Agent Bot".to_string(),
                avatar_initials: "AB".to_string(),
                avatar_color: 0x1ABC9C,
                story_count: 3,
                unread_count: 0,
                has_video_stream: false,
            },
        ]);
        sidebar.top_peers.set_entries(vec![
            TopPeerItem {
                id: 1,
                name: "Alice".to_string(),
                avatar_initials: "AS".to_string(),
                avatar_color: 0x2AABEE,
                badge: 2,
                is_unread: true,
                is_muted: false,
                is_online: true,
            },
            TopPeerItem {
                id: 2,
                name: "Rust Devs".to_string(),
                avatar_initials: "RC".to_string(),
                avatar_color: 0x9B59B6,
                badge: 5,
                is_unread: true,
                is_muted: false,
                is_online: false,
            },
            TopPeerItem {
                id: 4,
                name: "Agent Bot".to_string(),
                avatar_initials: "AB".to_string(),
                avatar_color: 0x1ABC9C,
                badge: 1,
                is_unread: true,
                is_muted: false,
                is_online: true,
            },
        ]);
        sidebar.suggestion_manager.add_suggestion(SuggestionItem {
            priority: SuggestionPriority::PremiumGrace,
            key: SuggestionManager::KEY_PREMIUM_GRACE.to_string(),
            title: "Premium Subscription Paused".to_string(),
            description: "Update payment method to keep Telegram Premium active.".to_string(),
            button_text: Some("Renew".to_string()),
            action_url: Some("https://t.me/premiumbot?start=status".to_string()),
            can_close: true,
        });
        sidebar.suggestions.update_from_manager(&sidebar.suggestion_manager);
        sidebar.inner_widget.total_rows = sidebar.dialogs.len();
        sidebar.inner_widget.selected_index = Some(0);
        sidebar.active_chat_id = Some(1);
        sidebar
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(MIN_WIDTH, MAX_WIDTH);
    }

    pub fn reset_width(&mut self) {
        self.width = DEFAULT_WIDTH;
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
        self.is_contacts_modal_open = false;
        self.active_creation_modal = None;
        self.remote_search_results.clear();
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

    pub fn recompute_folder_badges(&mut self) {
        let mut all_unread = 0u32;
        let mut folder_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for d in &self.dialogs {
            all_unread = all_unread.saturating_add(d.unread_count);
            if !d.folder.is_empty() && d.folder != "All" && d.folder != "Unread" {
                let entry = folder_counts.entry(d.folder.clone()).or_insert(0);
                *entry = entry.saturating_add(d.unread_count);
            }
        }
        let mut next_folders = vec![("All".to_string(), all_unread)];
        for (existing_name, _) in &self.folders {
            if existing_name != "All" && existing_name != "Unread" {
                let cnt = folder_counts.remove(existing_name).unwrap_or(0);
                next_folders.push((existing_name.clone(), cnt));
            }
        }
        let mut extra: Vec<(String, u32)> = folder_counts.into_iter().collect();
        extra.sort_by(|a, b| a.0.cmp(&b.0));
        next_folders.extend(extra);
        if all_unread > 0 || self.folders.iter().any(|(n, _)| n == "Unread") {
            next_folders.push(("Unread".to_string(), all_unread));
        }
        self.folders = next_folders;
        self.inner_widget.total_rows = self.dialogs.len();
    }

    pub fn open_contacts_modal(&mut self) {
        self.is_plus_menu_open = false;
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
        self.search_focused = true;
        self.is_plus_menu_open = false;
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
        self.is_plus_menu_open = !self.is_plus_menu_open;
        if self.is_plus_menu_open {
            self.search_focused = false;
        }
    }

    pub fn open_add_contact_modal(&mut self, kind: AddContactKind) {
        self.is_plus_menu_open = false;
        self.is_contacts_modal_open = false;
        self.search_focused = false;
        self.active_creation_modal = Some(SidebarCreationModal::AddContact);
        self.contact_kind = kind;
        self.active_creation_field = CreationInputField::PrimaryName;
        self.creation_primary_input.clear();
        self.creation_secondary_input.clear();
        self.creation_tertiary_input.clear();
    }

    pub fn open_create_group_modal(&mut self) {
        self.is_plus_menu_open = false;
        self.search_focused = false;
        self.active_creation_modal = Some(SidebarCreationModal::CreateGroup);
        self.active_creation_field = CreationInputField::PrimaryName;
        self.creation_primary_input.clear();
        self.creation_secondary_input.clear();
        self.creation_tertiary_input.clear();
    }

    pub fn close_creation_modal(&mut self) {
        self.active_creation_modal = None;
        self.is_plus_menu_open = false;
        self.creation_primary_input.clear();
        self.creation_secondary_input.clear();
        self.creation_tertiary_input.clear();
    }

    pub fn active_creation_field_text(&self) -> String {
        match self.active_creation_field {
            CreationInputField::PrimaryName => self.creation_primary_input.clone(),
            CreationInputField::SecondaryHandle => self.creation_secondary_input.clone(),
            CreationInputField::TertiaryPromptOrPhone => self.creation_tertiary_input.clone(),
        }
    }

    pub fn push_creation_text(&mut self, text: &str) {
        match self.active_creation_field {
            CreationInputField::PrimaryName => self.creation_primary_input.push_str(text),
            CreationInputField::SecondaryHandle => self.creation_secondary_input.push_str(text),
            CreationInputField::TertiaryPromptOrPhone => self.creation_tertiary_input.push_str(text),
        }
    }

    pub fn backspace_creation_text(&mut self) {
        match self.active_creation_field {
            CreationInputField::PrimaryName => {
                self.creation_primary_input.pop();
            }
            CreationInputField::SecondaryHandle => {
                self.creation_secondary_input.pop();
            }
            CreationInputField::TertiaryPromptOrPhone => {
                self.creation_tertiary_input.pop();
            }
        }
    }

    pub fn clear_creation_text(&mut self) {
        match self.active_creation_field {
            CreationInputField::PrimaryName => self.creation_primary_input.clear(),
            CreationInputField::SecondaryHandle => self.creation_secondary_input.clear(),
            CreationInputField::TertiaryPromptOrPhone => self.creation_tertiary_input.clear(),
        }
    }

    pub fn cycle_creation_field(&mut self) {
        self.active_creation_field = match self.active_creation_field {
            CreationInputField::PrimaryName => CreationInputField::SecondaryHandle,
            CreationInputField::SecondaryHandle => CreationInputField::TertiaryPromptOrPhone,
            CreationInputField::TertiaryPromptOrPhone => CreationInputField::PrimaryName,
        };
    }

    /// Commits the current Add Contact (`AiBot` or `Friend`) or Create Group modal,
    /// immediately inserting an interactive dialog into `self.dialogs`, selecting it,
    /// and queueing a backend persistence request (`pending_add_contact_request` or `pending_create_group_request`).
    pub fn submit_creation_modal(&mut self) -> Option<u64> {
        let modal = self.active_creation_modal?;
        let local_id = self.next_local_dialog_id;
        self.next_local_dialog_id += 1;

        match modal {
            SidebarCreationModal::AddContact => {
                let is_bot = self.contact_kind == AddContactKind::AiBot;
                let name = if self.creation_primary_input.trim().is_empty() {
                    if is_bot {
                        "智能助手 AI Bot".to_string()
                    } else {
                        "新好友 Contact".to_string()
                    }
                } else {
                    self.creation_primary_input.trim().to_string()
                };

                let handle = {
                    let raw = self.creation_secondary_input.trim().trim_start_matches('@');
                    if raw.is_empty() {
                        if is_bot {
                            "custom_ai_bot".to_string()
                        } else {
                            "friend_user".to_string()
                        }
                    } else if is_bot && !raw.ends_with("bot") {
                        format!("{}_bot", raw)
                    } else {
                        raw.to_string()
                    }
                };

                let tertiary = {
                    let raw = self.creation_tertiary_input.trim();
                    if raw.is_empty() {
                        None
                    } else {
                        Some(raw.to_string())
                    }
                };

                let lang = self.language_id.as_str();
                let (avatar_initials, avatar_color, preview_msg, folder) = if is_bot {
                    let default_prompt = tr_str(
                        lang,
                        "通用智能助手，支持问答、代码与翻译",
                        "General assistant for Q&A, code, and translation",
                    );
                    let prompt_preview = tertiary.as_deref().unwrap_or(default_prompt);
                    let mut initials: String = name.chars().take(2).collect::<String>().to_uppercase();
                    if initials.is_empty() {
                        initials = "BOT".to_string();
                    }
                    (
                        initials,
                        0x1ABC9C,
                        format!("AI Bot (@{}) • {}", handle, prompt_preview),
                        "Bots".to_string(),
                    )
                } else {
                    let initials = name
                        .chars()
                        .take(2)
                        .collect::<String>()
                        .to_uppercase();
                    (
                        if initials.is_empty() { "FR".to_string() } else { initials },
                        0x2AABEE,
                        if lang.eq_ignore_ascii_case("en") {
                            format!("Added @{} to contacts", handle)
                        } else {
                            format!("已添加好友 @{}，开始聊天吧", handle)
                        },
                        "Personal".to_string(),
                    )
                };

                self.dialogs.insert(
                    0,
                    DialogItemModel {
                        id: local_id,
                        peer_name: name.clone(),
                        avatar_initials,
                        avatar_color,
                        last_message: preview_msg,
                        timestamp: "Just now".to_string(),
                        unread_count: 1,
                        is_muted: false,
                        is_pinned: false,
                        is_online: true,
                        typing_indicator: None,
                        delivery_status: DialogDeliveryStatus::Read,
                        folder,
                    },
                );
                self.inner_widget.total_rows = self.dialogs.len();
                self.inner_widget.selected_index = Some(0);
                self.active_chat_id = Some(local_id);
                self.selection_revision = self.selection_revision.wrapping_add(1);
                if is_bot {
                    self.bot_ids.insert(local_id);
                }

                self.pending_add_contact_request = Some(PendingAddContactRequest {
                    local_dialog_id: local_id,
                    first_name: name,
                    last_name: None,
                    username: Some(handle),
                    phone: if is_bot { None } else { tertiary.clone() },
                    is_bot,
                    system_prompt: if is_bot { tertiary } else { None },
                });
            }
            SidebarCreationModal::CreateGroup => {
                let lang = self.language_id.as_str();
                let title = if self.creation_primary_input.trim().is_empty() {
                    tr_str(lang, "新建 Telegram 群组", "New Telegram Group").to_string()
                } else {
                    self.creation_primary_input.trim().to_string()
                };

                let mut member_ids: Vec<u64> = Vec::new();
                let mut member_usernames: Vec<String> = Vec::new();
                for token in self
                    .creation_secondary_input
                    .split(|c: char| c == ',' || c == ' ' || c == ';')
                {
                    let cleaned = token.trim().trim_start_matches('@');
                    if cleaned.is_empty() {
                        continue;
                    }
                    if let Ok(uid) = cleaned.parse::<u64>() {
                        if !member_ids.contains(&uid) {
                            member_ids.push(uid);
                        }
                    } else {
                        let handle = cleaned.to_string();
                        if !member_usernames.contains(&handle) {
                            member_usernames.push(handle);
                        }
                    }
                }

                let about = {
                    let raw = self.creation_tertiary_input.trim();
                    if raw.is_empty() {
                        None
                    } else {
                        Some(raw.to_string())
                    }
                };

                let initials = title
                    .chars()
                    .take(2)
                    .collect::<String>()
                    .to_uppercase();

                let preview_msg = if let Some(ref desc) = about {
                    if lang.eq_ignore_ascii_case("en") {
                        format!("Group \"{}\" • {}", title, desc)
                    } else {
                        format!("群组「{}」• {}", title, desc)
                    }
                } else if lang.eq_ignore_ascii_case("en") {
                    format!("Group \"{}\" created", title)
                } else {
                    format!("群组「{}」已创建", title)
                };

                self.dialogs.insert(
                    0,
                    DialogItemModel {
                        id: local_id,
                        peer_name: title.clone(),
                        avatar_initials: if initials.is_empty() { "GR".to_string() } else { initials },
                        avatar_color: 0x9B59B6,
                        last_message: preview_msg,
                        timestamp: "Just now".to_string(),
                        unread_count: 0,
                        is_muted: false,
                        is_pinned: false,
                        is_online: true,
                        typing_indicator: None,
                        delivery_status: DialogDeliveryStatus::Read,
                        folder: "Work".to_string(),
                    },
                );
                self.inner_widget.total_rows = self.dialogs.len();
                self.inner_widget.selected_index = Some(0);
                self.active_chat_id = Some(local_id);
                self.selection_revision = self.selection_revision.wrapping_add(1);

                self.pending_create_group_request = Some(PendingCreateGroupRequest {
                    local_dialog_id: local_id,
                    title,
                    member_ids,
                    member_usernames,
                    about,
                });
            }
        }

        self.close_creation_modal();
        Some(local_id)
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

    pub fn filtered_dialogs(&self) -> Vec<&DialogItemModel> {
        let q = self.search_query.to_lowercase();
        self.dialogs
            .iter()
            .filter(|d| {
                let matches_folder = if self.active_folder == "All" {
                    true
                } else if self.active_folder == "Unread" {
                    d.unread_count > 0
                } else {
                    d.folder == self.active_folder
                };

                let matches_search = if q.is_empty() {
                    true
                } else {
                    d.peer_name.to_lowercase().contains(&q)
                        || d.last_message.to_lowercase().contains(&q)
                };

                matches_folder && matches_search
            })
            .collect()
    }

    /// Computes Telegram 1:1 categorized search results combining local dialogs/messages
    /// and remote server search results for the active search category & tag.
    pub fn categorized_search_results(&self) -> Vec<CategorizedSearchResult> {
        let q = self.search_query.trim().to_lowercase();
        let cat = self.active_search_category.to_lowercase();
        let tag = self.active_search_tag.as_str();

        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for d in &self.dialogs {
            let is_bot = d.folder == "Bots"
                || d.peer_name.to_lowercase().contains("bot")
                || d.avatar_initials == "🤖";
            let is_channel = d.peer_name.to_lowercase().contains("announcement")
                || d.peer_name.to_lowercase().contains("channel")
                || d.peer_name.contains("频道");
            let last_lower = d.last_message.to_lowercase();
            let has_link = last_lower.contains("http://")
                || last_lower.contains("https://")
                || last_lower.contains("t.me/")
                || last_lower.contains("🔗");
            let has_media = last_lower.contains("[media]")
                || last_lower.contains("photo")
                || last_lower.contains("video")
                || last_lower.contains("🖼️");
            let has_file = last_lower.contains(".pdf")
                || last_lower.contains(".zip")
                || last_lower.contains(".rs")
                || last_lower.contains("file")
                || last_lower.contains("📄");
            let has_music = last_lower.contains(".mp3")
                || last_lower.contains("music")
                || last_lower.contains("🎵");
            let has_voice = last_lower.contains("voice")
                || last_lower.contains("语音")
                || last_lower.contains("🎤");

            let item_cat = match cat.as_str() {
                "channels" => {
                    if is_channel || d.folder == "Work" {
                        "Channels"
                    } else {
                        continue;
                    }
                }
                "apps" => {
                    if is_bot {
                        "Apps"
                    } else {
                        continue;
                    }
                }
                "posts" => {
                    if !is_bot {
                        "Posts"
                    } else {
                        continue;
                    }
                }
                "media" => {
                    if has_media {
                        "Media"
                    } else {
                        continue;
                    }
                }
                "downloads" => {
                    if has_file || has_media || last_lower.contains("download") {
                        "Downloads"
                    } else {
                        continue;
                    }
                }
                "links" => {
                    if has_link {
                        "Links"
                    } else {
                        continue;
                    }
                }
                "files" => {
                    if has_file {
                        "Files"
                    } else {
                        continue;
                    }
                }
                "music" => {
                    if has_music {
                        "Music"
                    } else {
                        continue;
                    }
                }
                "voice" => {
                    if has_voice {
                        "Voice"
                    } else {
                        continue;
                    }
                }
                _ => "Chats",
            };

            let tag_ok = match tag {
                "bot" => is_bot,
                "unread" => d.unread_count > 0,
                "pinned" => d.is_pinned,
                "media" => has_media || item_cat == "Media",
                "links" => has_link || item_cat == "Links",
                _ => true,
            };
            if !tag_ok {
                continue;
            }

            let query_ok = q.is_empty()
                || d.peer_name.to_lowercase().contains(&q)
                || last_lower.contains(&q);
            if !query_ok {
                continue;
            }

            let lang = self.language_id.as_str();
            let subtitle = match cat.as_str() {
                "media" => format!("{} • {}", tr_str(lang, "共享照片与视频", "Shared Photos & Videos"), d.last_message),
                "downloads" => format!("{} • {}", tr_str(lang, "已下载文件", "Downloads"), d.last_message),
                "links" => format!("{} (https://t.me/{}) • {}", tr_str(lang, "共享链接", "Shared Link"), d.id, d.last_message),
                "files" => format!("{} • {}", tr_str(lang, "共享文档与源码", "Shared Files"), d.last_message),
                "music" => format!("{} • {}", tr_str(lang, "音频播放列表", "Audio Playlist"), d.last_message),
                "voice" => format!("{} • {}", tr_str(lang, "语音消息记录", "Voice Messages"), d.last_message),
                "posts" => format!("{} • {}", tr_str(lang, "频道/群组帖子", "Public Posts"), d.last_message),
                "apps" => format!("{} • {}", tr_str(lang, "Telegram AI Bot / 小程序", "Telegram AI Bot / Mini App"), d.last_message),
                "channels" => format!("{} • {}", tr_str(lang, "频道与群组", "Channels & Groups"), d.last_message),
                _ => d.last_message.clone(),
            };

            if seen.insert((d.id, 0)) {
                out.push(CategorizedSearchResult {
                    chat_id: d.id,
                    message_id: 0,
                    title: d.peer_name.clone(),
                    subtitle,
                    category: item_cat.to_string(),
                    timestamp: d.timestamp.clone(),
                    avatar_initials: d.avatar_initials.clone(),
                    avatar_color: d.avatar_color,
                    is_bot,
                });
            }
        }

        for r in &self.remote_search_results {
            let r_cat = r.category.to_lowercase();
            let cat_ok = cat == "chats"
                || r_cat == cat
                || (cat == "apps" && r.is_bot)
                || (cat == "downloads"
                    && (r_cat == "downloads" || r_cat == "files" || r_cat == "media"))
                || (cat == "channels" && r_cat == "channels");
            let sub_lower = r.subtitle.to_lowercase();
            let tag_ok = match tag {
                "bot" => r.is_bot,
                "media" => {
                    r_cat == "media"
                        || sub_lower.contains("[media]")
                        || sub_lower.contains("photo")
                        || sub_lower.contains("video")
                }
                "links" => {
                    r_cat == "links"
                        || sub_lower.contains("http://")
                        || sub_lower.contains("https://")
                        || sub_lower.contains("t.me/")
                }
                _ => true,
            };
            let query_ok = q.is_empty()
                || r.title.to_lowercase().contains(&q)
                || sub_lower.contains(&q);
            if cat_ok && tag_ok && query_ok && seen.insert((r.chat_id, r.message_id)) {
                out.push(r.clone());
            }
        }

        out
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

    pub fn toggle_archive_dialog(&mut self, id: u64) {
        if let Some(pos) = self.archived_ids.iter().position(|&x| x == id) {
            self.archived_ids.remove(pos);
        } else {
            self.archived_ids.push(id);
        }
    }

    pub fn toggle_mute_dialog(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.is_muted = !d.is_muted;
        }
    }

    pub fn toggle_pin_dialog(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.is_pinned = !d.is_pinned;
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

    pub fn toggle_unread_mark(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            if d.unread_count > 0 {
                d.unread_count = 0;
                self.unread_marked_ids.remove(&id);
            } else if !self.unread_marked_ids.remove(&id) {
                self.unread_marked_ids.insert(id);
            }
        }
        self.recompute_folder_badges();
    }

    pub fn clear_history(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.last_message.clear();
            d.unread_count = 0;
            d.typing_indicator = None;
        }
        self.drafts.remove(&id);
        self.unread_marked_ids.remove(&id);
        self.recompute_folder_badges();
    }

    pub fn delete_dialog(&mut self, id: u64) {
        self.dialogs.retain(|d| d.id != id);
        self.archived_ids.retain(|&x| x != id);
        self.drafts.remove(&id);
        self.unread_marked_ids.remove(&id);
        self.forum_topics.remove(&id);
        if self.active_chat_id == Some(id) {
            self.clear_selection();
        } else if let Some(sel) = self.active_chat_id {
            self.inner_widget.selected_index = self.dialogs.iter().position(|d| d.id == sel);
        }
        self.recompute_folder_badges();
    }

    pub fn apply_context_action(&mut self, action: SidebarContextAction) {
        match action {
            SidebarContextAction::TogglePin(id) => self.toggle_pin_dialog(id),
            SidebarContextAction::ToggleMute(id) => self.toggle_mute_dialog(id),
            SidebarContextAction::ToggleArchive(id) => self.toggle_archive_dialog(id),
            SidebarContextAction::ToggleUnread(id) => self.toggle_unread_mark(id),
            SidebarContextAction::ClearHistory(id) => self.clear_history(id),
            SidebarContextAction::DeleteChat(id) => self.delete_dialog(id),
        }
        self.context_menu = None;
        self.pending_context_action = Some(action);
    }

    pub fn dialog_meta(&self, item: &DialogItemModel) -> SidebarDialogMeta {
        let lower = item.peer_name.to_lowercase();
        let topics = self.forum_topics.get(&item.id).cloned();
        SidebarDialogMeta {
            is_verified: self.verified_ids.contains(&item.id),
            is_premium: self.premium_ids.contains(&item.id),
            is_bot: self.bot_ids.contains(&item.id) || item.folder == "Bots" || lower.ends_with("bot"),
            is_saved_messages: self.saved_messages_id == Some(item.id)
                || item.peer_name == "Saved Messages"
                || item.peer_name == "已保存消息",
            is_unread_marked: self.unread_marked_ids.contains(&item.id),
            is_archived: self.archived_ids.contains(&item.id),
            draft_text: self.drafts.get(&item.id).cloned(),
            media_badge: self.media_badges.get(&item.id).copied().unwrap_or(DialogMessageMediaBadge::None),
            is_topics_expanded: self.expanded_forum_id == Some(item.id),
            topics,
        }
    }
}

impl Default for ChatSidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ChatSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = self.palette.clone();
        let lang_id = self.language_id.clone();
        let active_folder = self.active_folder.clone();
        let active_id = self.active_chat_id;
        let query = self.search_query.clone();
        let is_search_focused = self.search_focused;
        let is_plus_open = self.is_plus_menu_open;
        let creation_modal = self.active_creation_modal;
        let contact_kind = self.contact_kind;
        let active_field = self.active_creation_field;
        let primary_val = self.creation_primary_input.clone();
        let secondary_val = self.creation_secondary_input.clone();
        let tertiary_val = self.creation_tertiary_input.clone();
        let active_search_cat = self.active_search_category.clone();
        let active_search_tag = self.active_search_tag.clone();
        let is_narrow = self.width < 252.0;

        let categorized_results = if is_search_focused {
            self.categorized_search_results()
        } else {
            Vec::new()
        };

        let header = render_sidebar_header(
            &query,
            is_search_focused,
            is_plus_open,
            creation_modal.is_some(),
            &palette,
            &lang_id,
            cx,
            |this, cx| {
                if this.search_focused {
                    this.select_folder("All");
                    this.exit_search();
                    this.close_creation_modal();
                } else {
                    this.open_main_menu_requested = true;
                }
                cx.notify();
            },
            |this, cx| {
                this.focus_search();
                cx.notify();
            },
            |this, cx| {
                if !this.search_query.is_empty() {
                    this.set_search_query("");
                } else {
                    this.exit_search();
                }
                cx.notify();
            },
            |this, cx| {
                this.toggle_plus_menu();
                cx.notify();
            },
        );

        let plus_dropdown = if is_plus_open {
            Some(render_plus_dropdown(
                &palette,
                &lang_id,
                cx,
                |this, kind, cx| {
                    this.open_add_contact_modal(kind);
                    cx.notify();
                },
                |this, cx| {
                    this.open_create_group_modal();
                    cx.notify();
                },
                |this, cx| {
                    this.open_contacts_modal();
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let search_panel = if is_search_focused {
            let peers_list = self.top_peers.entries.clone();
            let scope_ui = self.search_in_ui.clone();
            Some(render_search_panel(
                &active_search_cat,
                &active_search_tag,
                &peers_list,
                &scope_ui,
                &palette,
                &lang_id,
                cx,
                |this, cid, cx| {
                    this.select_search_category(cid);
                    cx.notify();
                },
                |this, tid, cx| {
                    this.select_search_tag(tid);
                    cx.notify();
                },
                |this, pid, cx| {
                    this.search_focused = false;
                    this.select_dialog_from_user(pid);
                    cx.notify();
                },
                |this, cx| {
                    this.search_in_ui.clear_scope();
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let suggestion_bar = {
            let sug_ui = self.suggestions.clone();
            render_suggestion_bar(&sug_ui, &palette, cx, |this, cx| {
                this.suggestions.dismiss_current(&mut this.suggestion_manager);
                cx.notify();
            })
        };

        let stories_strip = {
            let stories_ui = self.stories.clone();
            render_stories_strip(
                &stories_ui,
                &palette,
                &lang_id,
                cx,
                |this, cx| {
                    this.stories.toggle_expanded();
                    cx.notify();
                },
                |this, sid, peer, cx| {
                    this.story_clicked_peer = Some(peer);
                    this.stories.active_story_id = Some(sid);
                    this.select_dialog_from_user(sid);
                    cx.notify();
                },
            )
        };

        let folder_tabs = if !is_search_focused {
            let folders_list = self.folders.clone();
            Some(render_folder_tabs_strip(
                &folders_list,
                &active_folder,
                &palette,
                cx,
                |this, fname, cx| {
                    this.select_folder(fname);
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let scroll_children: Vec<AnyElement> = if is_search_focused {
            render_search_results_list(
                &query,
                categorized_results,
                &palette,
                &lang_id,
                cx,
                |this, cid, cx| {
                    this.search_focused = false;
                    this.select_dialog_from_user(cid);
                    cx.notify();
                },
            )
        } else {
            let is_default_all = active_folder == "All" && query.trim().is_empty();
            let filtered: Vec<DialogItemModel> =
                self.filtered_dialogs().into_iter().cloned().collect();
            let mut archived: Vec<DialogItemModel> = Vec::new();
            let mut pinned: Vec<DialogItemModel> = Vec::new();
            let mut regular: Vec<DialogItemModel> = Vec::new();
            for item in filtered {
                if is_default_all && self.archived_ids.contains(&item.id) {
                    archived.push(item);
                } else if item.is_pinned {
                    pinned.push(item);
                } else {
                    regular.push(item);
                }
            }
            let archived_with_meta: Vec<(DialogItemModel, SidebarDialogMeta)> = archived
                .iter()
                .map(|d| (d.clone(), self.dialog_meta(d)))
                .collect();
            let main_with_meta: Vec<(DialogItemModel, SidebarDialogMeta)> = pinned
                .into_iter()
                .chain(regular.into_iter())
                .map(|d| (d.clone(), self.dialog_meta(&d)))
                .collect();

            let mut list: Vec<AnyElement> = Vec::new();
            if let Some(arch_header) = render_archived_header_row(
                &archived,
                self.is_archived_expanded,
                &palette,
                &lang_id,
                cx,
                |this, cx| {
                    this.is_archived_expanded = !this.is_archived_expanded;
                    cx.notify();
                },
            ) {
                list.push(arch_header);
            }
            if self.is_archived_expanded {
                for (idx, (item, meta)) in archived_with_meta.iter().enumerate() {
                    let did = item.id;
                    let is_sel = active_id == Some(did);
                    list.push(render_dialog_row_item(
                        item,
                        idx,
                        is_sel,
                        is_narrow,
                        meta,
                        &palette,
                        &lang_id,
                        cx,
                        move |this, cx| {
                            this.context_menu = None;
                            this.search_focused = false;
                            this.select_dialog_from_user(did);
                            cx.notify();
                        },
                        move |this, pos, cx| {
                            this.context_menu = Some((did, pos));
                            cx.notify();
                        },
                        move |this, cx| {
                            this.toggle_forum_topics(did);
                            cx.notify();
                        },
                    ));
                }
            }

            let selected_topic = self.selected_topic_id;
            for (idx, (item, meta)) in main_with_meta.iter().enumerate() {
                let did = item.id;
                let is_sel = active_id == Some(did);
                list.push(render_dialog_row_item(
                    item,
                    idx,
                    is_sel,
                    is_narrow,
                    meta,
                    &palette,
                    &lang_id,
                    cx,
                    move |this, cx| {
                        this.context_menu = None;
                        this.search_focused = false;
                        this.select_dialog_from_user(did);
                        cx.notify();
                    },
                    move |this, pos, cx| {
                        this.context_menu = Some((did, pos));
                        cx.notify();
                    },
                    move |this, cx| {
                        this.toggle_forum_topics(did);
                        cx.notify();
                    },
                ));
                if meta.is_topics_expanded {
                    if let Some(ref topics_ui) = meta.topics {
                        list.push(render_forum_topics_subtree(
                            topics_ui,
                            selected_topic,
                            &palette,
                            cx,
                            |this, fid, tid, cx| {
                                this.selected_topic_id = Some((fid, tid));
                                this.select_dialog_from_user(fid);
                                cx.notify();
                            },
                        ));
                    }
                }
            }

            if list.is_empty() {
                list.push(
                    div()
                        .id("sidebar-empty-dialogs-state")
                        .w_full()
                        .py(px(40.0))
                        .px(px(16.0))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(10.0))
                        .child(tg_icon(IconName::FolderChat, 32.0, palette.text_secondary))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.text_primary))
                                .child(tr_str(&lang_id, "暂无对话", "No chats yet")),
                        )
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(palette.text_secondary))
                                .child(tr_str(
                                    &lang_id,
                                    "点击右上角图标添加联系人或创建群组",
                                    "Start a chat or connect your workspace",
                                )),
                        )
                        .into_any_element(),
                );
            }
            list
        };

        let ctx_menu_overlay = if let Some((ctx_id, anchor)) = self.context_menu {
            let (is_pinned, is_muted, has_unread) = self
                .dialogs
                .iter()
                .find(|d| d.id == ctx_id)
                .map(|d| (d.is_pinned, d.is_muted, d.unread_count > 0 || self.unread_marked_ids.contains(&ctx_id)))
                .unwrap_or((false, false, false));
            let is_archived = self.archived_ids.contains(&ctx_id);
            Some(render_dialog_context_menu(
                ctx_id,
                anchor,
                is_pinned,
                is_muted,
                is_archived,
                has_unread,
                &palette,
                &lang_id,
                cx,
                |this, action, cx| {
                    this.apply_context_action(action);
                    cx.notify();
                },
                |this, cx| {
                    this.context_menu = None;
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let accent = rgb(palette.accent);
        div()
            .relative()
            .w(px(self.width))
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(palette.dialogs_bg))
            .border_r_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .flex_col()
            .on_mouse_move(cx.listener(|this, ev: &MouseMoveEvent, _w, cx| {
                if this.is_resizing {
                    if let Some(start_x) = this.resize_drag_start_x {
                        let dx = f32::from(ev.position.x) - start_x;
                        this.set_width(this.resize_start_width + dx);
                        cx.notify();
                    }
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _ev: &MouseUpEvent, _w, cx| {
                if this.is_resizing {
                    this.is_resizing = false;
                    this.resize_drag_start_x = None;
                    cx.notify();
                }
            }))
            .child(
                div()
                    .id("sidebar-resize-handle")
                    .absolute()
                    .top_0()
                    .right_0()
                    .w(px(4.0))
                    .h_full()
                    .cursor_col_resize()
                    .hover(move |s| s.bg(accent))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, ev: &MouseDownEvent, _w, cx| {
                        this.is_resizing = true;
                        this.resize_drag_start_x = Some(f32::from(ev.position.x));
                        this.resize_start_width = this.width;
                        cx.notify();
                    })),
            )
            .child(header)
            .when_some(plus_dropdown, |el, panel| el.child(panel))
            .when_some(creation_modal, |el, modal_kind| {
                el.child(self.render_creation_modal_panel(
                    modal_kind,
                    contact_kind,
                    active_field,
                    &primary_val,
                    &secondary_val,
                    &tertiary_val,
                    &palette,
                    &lang_id,
                    cx,
                ))
            })
            .when_some(search_panel, |el, panel| el.child(panel))
            .when_some(suggestion_bar, |el, bar| el.child(bar))
            .when_some(stories_strip, |el, strip| el.child(strip))
            .when_some(folder_tabs, |el, tabs| el.child(tabs))
            .child(
                div()
                    .id("sidebar-dialogs-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .children(scroll_children),
            )
            .when_some(ctx_menu_overlay, |el, overlay| el.child(overlay))
    }
}

impl ChatSidebar {
    fn render_creation_modal_panel(
        &self,
        modal_kind: SidebarCreationModal,
        contact_kind: AddContactKind,
        active_field: CreationInputField,
        primary_val: &str,
        secondary_val: &str,
        tertiary_val: &str,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_add_contact = modal_kind == SidebarCreationModal::AddContact;
        let is_ai_bot = contact_kind == AddContactKind::AiBot;

        let primary_label = if is_add_contact {
            if is_ai_bot { tr_str(lang_id, "AI Bot 名称", "Bot Name") } else { tr_str(lang_id, "好友昵称 / 姓名", "Friend Name") }
        } else {
            tr_str(lang_id, "群组名称 (Group Title)", "Group Title")
        };
        let secondary_label = if is_add_contact {
            if is_ai_bot { tr_str(lang_id, "Bot 用户名 (@bot)", "Bot Handle (@bot)") } else { tr_str(lang_id, "手机号 / @用户名", "Phone or @username") }
        } else {
            tr_str(lang_id, "群成员 (@username 逗号分隔)", "Members (@username, comma separated)")
        };
        let tertiary_label = if is_add_contact {
            if is_ai_bot { tr_str(lang_id, "AI 系统提示词 (Prompt)", "System Prompt") } else { tr_str(lang_id, "备注信息", "Bio / Notes") }
        } else {
            tr_str(lang_id, "群组简介 (Description)", "Description")
        };

        let field_box = |label: &'static str, val: &str, is_active: bool| {
            div()
                .cursor_pointer()
                .w_full()
                .px(px(8.0))
                .py(px(5.0))
                .rounded(px(6.0))
                .bg(rgb(palette.input_bg))
                .border_1()
                .border_color(if is_active { rgb(palette.accent) } else { rgb(palette.border_fg) })
                .child(div().text_size(px(10.0)).text_color(rgb(palette.text_secondary)).child(label))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(palette.text_primary))
                        .child(if is_active { format!("{}|", val) } else { val.to_string() }),
                )
        };

        let hover_text = rgb(palette.text_primary);
        let accent_hover = rgb(palette.accent_hover);
        div()
            .id("sidebar-creation-modal-panel")
            .w_full()
            .px(px(10.0))
            .py(px(10.0))
            .bg(rgb(palette.menu_bg))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.text_primary))
                            .child(if is_add_contact {
                                tr_str(lang_id, "添加联系人", "Add Contact")
                            } else {
                                tr_str(lang_id, "创建群组", "New Group")
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .hover(move |s| s.text_color(hover_text))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                                this.close_creation_modal();
                                cx.notify();
                            }))
                            .child(tg_icon(IconName::Close, 13.0, palette.icon_fg)),
                    ),
            )
            .when(is_add_contact, |el| {
                el.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .id("add-contact-tab-aibot")
                                .cursor_pointer()
                                .flex_1()
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(if is_ai_bot { rgb(palette.accent) } else { rgb(palette.composer_bg) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap(px(4.0))
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(if is_ai_bot { rgb(palette.active_button_fg) } else { rgb(palette.text_primary) })
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                                    this.contact_kind = AddContactKind::AiBot;
                                    cx.notify();
                                }))
                                .child(tg_icon(IconName::BotRobot, 13.0, if is_ai_bot { palette.active_button_fg } else { palette.icon_fg }))
                                .child(tr_str(lang_id, "创建 AI Bot", "AI Bot")),
                        )
                        .child(
                            div()
                                .id("add-contact-tab-friend")
                                .cursor_pointer()
                                .flex_1()
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(if !is_ai_bot { rgb(palette.accent) } else { rgb(palette.composer_bg) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap(px(4.0))
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(if !is_ai_bot { rgb(palette.active_button_fg) } else { rgb(palette.text_primary) })
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                                    this.contact_kind = AddContactKind::Friend;
                                    cx.notify();
                                }))
                                .child(tg_icon(IconName::ContactsUsers, 13.0, if !is_ai_bot { palette.active_button_fg } else { palette.icon_fg }))
                                .child(tr_str(lang_id, "添加好友", "Friend")),
                        ),
                )
            })
            .child(
                field_box(primary_label, primary_val, active_field == CreationInputField::PrimaryName)
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                        this.active_creation_field = CreationInputField::PrimaryName;
                        cx.notify();
                    })),
            )
            .child(
                field_box(secondary_label, secondary_val, active_field == CreationInputField::SecondaryHandle)
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                        this.active_creation_field = CreationInputField::SecondaryHandle;
                        cx.notify();
                    })),
            )
            .child(
                field_box(tertiary_label, tertiary_val, active_field == CreationInputField::TertiaryPromptOrPhone)
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                        this.active_creation_field = CreationInputField::TertiaryPromptOrPhone;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(palette.composer_bg))
                            .text_size(px(11.5))
                            .text_color(rgb(palette.text_secondary))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                                this.close_creation_modal();
                                cx.notify();
                            }))
                            .child(tr_str(lang_id, "取消", "Cancel")),
                    )
                    .child(
                        div()
                            .id("creation-modal-submit-btn")
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(palette.accent))
                            .hover(move |s| s.bg(accent_hover))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.active_button_fg))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _ev, _w, cx| {
                                this.submit_creation_modal();
                                cx.notify();
                            }))
                            .child(if is_add_contact {
                                if is_ai_bot { tr_str(lang_id, "创建 AI Bot", "Create Bot") } else { tr_str(lang_id, "添加好友", "Add Contact") }
                            } else {
                                tr_str(lang_id, "创建群组", "Create Group")
                            }),
                    ),
            )
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
}
