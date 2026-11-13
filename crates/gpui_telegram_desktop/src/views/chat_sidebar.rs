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

use crate::theme::dialogs::DialogsStyleSheet;
use crate::views::sidebar::dialogs_inner_widget::DialogsInnerWidget;
use crate::views::sidebar::dialogs_widget::DialogsWidget;
use crate::views::sidebar::suggestions::suggestion_manager::{SuggestionItem, SuggestionManager, SuggestionPriority};
use crate::views::sidebar::ui::dialogs_stories_list::{DialogsStoriesListUi, StoryElement};
use crate::views::sidebar::ui::dialogs_suggestions::DialogsSuggestionsUi;
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
    ("bot", "🤖 AI Bot"),
    ("unread", "🔵 未读"),
    ("pinned", "📌 置顶"),
    ("media", "🖼️ 媒体"),
    ("links", "🔗 链接"),
];

pub struct ChatSidebar {
    pub width: f32,
    pub is_resizing: bool,
    pub search_query: String,
    pub active_folder: String,
    pub active_chat_id: Option<u64>,
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

    // Authentic submodules wired per Mandates #17 & #18
    pub widget: DialogsWidget,
    pub inner_widget: DialogsInnerWidget,
    pub stories: DialogsStoriesListUi,
    pub top_peers: TopPeersStripUi,
    pub suggestions: DialogsSuggestionsUi,
    pub suggestion_manager: SuggestionManager,
    pub style: DialogsStyleSheet,
    pub drafts: std::collections::HashMap<u64, String>,
    pub archived_ids: Vec<u64>,
}

impl ChatSidebar {
    pub fn new() -> Self {
        let folders = vec![
            ("All".to_string(), 8),
            ("Personal".to_string(), 3),
            ("Work".to_string(), 5),
            ("Bots".to_string(), 0),
            ("Unread".to_string(), 8),
        ];

        let dialogs = vec![
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
                avatar_initials: "🔖".to_string(),
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
                avatar_initials: "🤖".to_string(),
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
                avatar_initials: "📢".to_string(),
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

        // Initialize authentic stories carousel
        let mut stories = DialogsStoriesListUi::new();
        stories.set_elements(vec![
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
                avatar_initials: "🤖".to_string(),
                avatar_color: 0x1ABC9C,
                story_count: 3,
                unread_count: 0,
                has_video_stream: false,
            },
        ]);

        // Initialize authentic top peers strip
        let mut top_peers = TopPeersStripUi::new();
        top_peers.set_entries(vec![
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
                avatar_initials: "🤖".to_string(),
                avatar_color: 0x1ABC9C,
                badge: 1,
                is_unread: true,
                is_muted: false,
                is_online: true,
            },
        ]);

        // Initialize suggestion manager with a sample priority notice
        let mut suggestion_manager = SuggestionManager::new();
        suggestion_manager.add_suggestion(SuggestionItem {
            priority: SuggestionPriority::PremiumGrace,
            key: SuggestionManager::KEY_PREMIUM_GRACE.to_string(),
            title: "Premium Subscription Paused".to_string(),
            description: "Update payment method to keep Telegram Premium active.".to_string(),
            button_text: Some("Renew".to_string()),
            action_url: Some("https://t.me/premiumbot?start=status".to_string()),
            can_close: true,
        });

        let mut suggestions = DialogsSuggestionsUi::new();
        suggestions.update_from_manager(&suggestion_manager);

        let mut inner_widget = DialogsInnerWidget::new();
        inner_widget.total_rows = dialogs.len();

        Self {
            width: DEFAULT_WIDTH,
            is_resizing: false,
            search_query: String::new(),
            active_folder: "All".to_string(),
            active_chat_id: Some(1),
            dialogs,
            folders,
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
            widget: DialogsWidget::new(),
            inner_widget,
            stories,
            top_peers,
            suggestions,
            suggestion_manager,
            style: DialogsStyleSheet::new(),
            drafts: std::collections::HashMap::new(),
            archived_ids: Vec::new(),
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(MIN_WIDTH, MAX_WIDTH);
    }

    pub fn reset_width(&mut self) {
        self.width = DEFAULT_WIDTH;
    }

    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str) {
        self.folders = vec![
            ("All".to_string(), 0),
            ("Personal".to_string(), 0),
            ("Work".to_string(), 0),
            ("Bots".to_string(), 0),
            ("Unread".to_string(), 0),
        ];

        self.dialogs = vec![DialogItemModel {
            id: 4,
            peer_name: "Autonomous Agent Bot".to_string(),
            avatar_initials: "🤖".to_string(),
            avatar_color: 0x1ABC9C,
            last_message: "Cloud AI Agent • Ready for prompts".to_string(),
            timestamp: "Online".to_string(),
            unread_count: 0,
            is_muted: false,
            is_pinned: true,
            is_online: true,
            typing_indicator: None,
            delivery_status: DialogDeliveryStatus::Read,
            folder: "Bots".to_string(),
        }];

        self.stories.set_elements(vec![]);
        self.top_peers.set_entries(vec![]);
        self.suggestions.is_visible = false;
        self.suggestions.current_item = None;
        self.inner_widget.total_rows = self.dialogs.len();
        self.active_folder = "All".to_string();
        self.active_chat_id = Some(4);
    }

    pub fn recompute_folder_badges(&mut self) {
        let mut all_unread = 0u32;
        let mut personal_unread = 0u32;
        let mut work_unread = 0u32;
        let mut bots_unread = 0u32;

        for d in &self.dialogs {
            all_unread = all_unread.saturating_add(d.unread_count);
            match d.folder.as_str() {
                "Personal" => personal_unread = personal_unread.saturating_add(d.unread_count),
                "Work" => work_unread = work_unread.saturating_add(d.unread_count),
                "Bots" => bots_unread = bots_unread.saturating_add(d.unread_count),
                _ => {}
            }
        }

        self.folders = vec![
            ("All".to_string(), all_unread),
            ("Personal".to_string(), personal_unread),
            ("Work".to_string(), work_unread),
            ("Bots".to_string(), bots_unread),
            ("Unread".to_string(), all_unread),
        ];
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
        } else {
            self.active_search_tag = t;
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

                let (avatar_initials, avatar_color, preview_msg, folder) = if is_bot {
                    let prompt_preview = tertiary
                        .as_deref()
                        .unwrap_or("通用智能助手，支持问答、代码与翻译");
                    (
                        "🤖".to_string(),
                        0x1ABC9C,
                        format!("🤖 AI Bot (@{}) • {}", handle, prompt_preview),
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
                        format!("👋 已添加好友 @{}，开始聊天吧！", handle),
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
                self.active_chat_id = Some(local_id);

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
                let title = if self.creation_primary_input.trim().is_empty() {
                    "新建 Telegram 群组".to_string()
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
                    format!("👥 群组「{}」• {}", title, desc)
                } else {
                    format!("👥 群组「{}」已创建", title)
                };

                self.dialogs.insert(
                    0,
                    DialogItemModel {
                        id: local_id,
                        peer_name: title.clone(),
                        avatar_initials: if initials.is_empty() { "👥".to_string() } else { initials },
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
                self.active_chat_id = Some(local_id);

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
        self.active_chat_id = Some(id);
        self.mark_as_read(id);
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

            let subtitle = match cat.as_str() {
                "media" => format!("🖼️ 共享照片与视频 • {}", d.last_message),
                "downloads" => format!("⬇️ 已下载文件 • {}", d.last_message),
                "links" => format!("🔗 共享链接 (https://t.me/{}) • {}", d.id, d.last_message),
                "files" => format!("📄 共享文档与源码 • {}", d.last_message),
                "music" => format!("🎵 音频播放列表 • {}", d.last_message),
                "voice" => format!("🎤 语音消息记录 • {}", d.last_message),
                "posts" => format!("📝 频道/群组帖子 • {}", d.last_message),
                "apps" => format!("🤖 Telegram AI Bot / 小程序 • {}", d.last_message),
                "channels" => format!("📢 频道与群组 • {}", d.last_message),
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
}

impl Default for ChatSidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ChatSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        let categorized_results = if is_search_focused {
            self.categorized_search_results()
        } else {
            Vec::new()
        };
        let dialog_list: Vec<DialogItemModel> = self.filtered_dialogs().into_iter().cloned().collect();
        let folders_list = self.folders.clone();
        let stories_list: Vec<StoryElement> = self.stories.elements.iter().take(3).cloned().collect();
        let has_suggestion = self.suggestions.is_visible;
        let suggestion_item = self.suggestions.current_item.clone();
        let stories_count = self.stories.collapsed_preview_count();
        let top_peers_list = if self.top_peers.entries.is_empty() {
            self.dialogs
                .iter()
                .take(4)
                .map(|d| TopPeerItem {
                    id: d.id,
                    name: d.peer_name.chars().take(8).collect(),
                    avatar_initials: d.avatar_initials.clone(),
                    avatar_color: d.avatar_color,
                    badge: d.unread_count,
                    is_unread: d.unread_count > 0,
                    is_muted: d.is_muted,
                    is_online: d.is_online,
                })
                .collect()
        } else {
            self.top_peers.entries.clone()
        };

        div()
            .relative()
            .w(px(self.width))
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x17212B))
            .border_r_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            // Resize drag handle (width 4px, absolute at right edge)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .w(px(4.0))
                    .h_full()
                    .cursor_col_resize()
                    .hover(|s| s.bg(rgb(0x2AABEE))),
            )
            // 1. Universal Search Header (Height 48px) with Search Input + `+` Button
            .child(
                div()
                    .h(px(SEARCH_HEADER_HEIGHT))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(10.0))
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    // Menu / Hamburger icon or Back arrow when in Search Mode
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(rgb(0x8A9AA8))
                            .hover(|s| s.text_color(rgb(0xFFFFFF)))
                            .text_size(px(16.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.select_folder("All");
                                this.exit_search();
                                this.close_creation_modal();
                                cx.notify();
                            }))
                            .child(if is_search_focused { "←" } else { "☰" }),
                    )
                    // Search Input field (height 35px, rounded 18px, background #242F3D)
                    .child(
                        div()
                            .id("sidebar-search-box")
                            .cursor_pointer()
                            .flex_1()
                            .h(px(SEARCH_HEIGHT))
                            .bg(rgb(0x242F3D))
                            .border_1()
                            .border_color(if is_search_focused { rgb(0x2AABEE) } else { rgba(0x00000000) })
                            .rounded(px(SEARCH_RADIUS))
                            .px(px(10.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.focus_search();
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_color(if is_search_focused { rgb(0x2AABEE) } else { rgb(0x6B808D) })
                                            .text_size(px(12.0))
                                            .child("🔍"),
                                    )
                                    .child(
                                        div()
                                            .text_color(if query.is_empty() { rgb(0x6B808D) } else { rgb(0xFFFFFF) })
                                            .text_size(px(12.5))
                                            .child(if query.is_empty() {
                                                if is_search_focused {
                                                    "| 搜索".to_string()
                                                } else {
                                                    "搜索".to_string()
                                                }
                                            } else if is_search_focused {
                                                format!("{}|", query)
                                            } else {
                                                query.clone()
                                            }),
                                    ),
                            )
                            .when(is_search_focused || !query.is_empty(), |el| {
                                el.child(
                                    div()
                                        .id("sidebar-search-clear-btn")
                                        .cursor_pointer()
                                        .px(px(4.0))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                            cx.stop_propagation();
                                            if !this.search_query.is_empty() {
                                                this.set_search_query("");
                                            } else {
                                                this.exit_search();
                                            }
                                            cx.notify();
                                        }))
                                        .child("✕"),
                                )
                            }),
                    )
                    // `+` Button right next to Search Box (Add Contact: AI Bot / Friend, Create Group)
                    .child(
                        div()
                            .id("sidebar-plus-action-btn")
                            .cursor_pointer()
                            .w(px(SEARCH_HEIGHT))
                            .h(px(SEARCH_HEIGHT))
                            .flex_shrink_0()
                            .rounded_full()
                            .bg(if is_plus_open || creation_modal.is_some() {
                                rgb(0x2AABEE)
                            } else {
                                rgb(0x242F3D)
                            })
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_plus_menu();
                                cx.notify();
                            }))
                            .child("+"),
                    ),
            )
            // 1b. `+` Creation Menu Dropdown (Add Contact -> AI Bot / Friend, Create Group, Contacts Modal)
            .when(is_plus_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(10.0))
                        .py(px(8.0))
                        .bg(rgb(0x202B36))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .id("plus-menu-add-contact")
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(7.0))
                                .rounded(px(8.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2B5278)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.open_add_contact_modal(AddContactKind::AiBot);
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(div().text_size(px(14.0)).child("👤"))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(12.5))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0xFFFFFF))
                                                        .child("添加联系人 (Add Contact)"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(10.5))
                                                        .text_color(rgb(0x8A9AA8))
                                                        .child("支持创建专属 AI Bot 或添加好友"),
                                                ),
                                        ),
                                )
                                .child(div().text_size(px(11.0)).text_color(rgb(0x2AABEE)).child("→")),
                        )
                        .child(
                            div()
                                .id("plus-menu-create-group")
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(7.0))
                                .rounded(px(8.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2B5278)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.open_create_group_modal();
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(div().text_size(px(14.0)).child("👥"))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(12.5))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0xFFFFFF))
                                                        .child("创建群组 (Create Group)"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(10.5))
                                                        .text_color(rgb(0x8A9AA8))
                                                        .child("创建多人或 AI 协同群聊"),
                                                ),
                                        ),
                                )
                                .child(div().text_size(px(11.0)).text_color(rgb(0x2AABEE)).child("→")),
                        )
                        .child(
                            div()
                                .id("plus-menu-contacts-modal")
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(7.0))
                                .rounded(px(8.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2B5278)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.open_contacts_modal();
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(div().text_size(px(14.0)).child("📒"))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(12.5))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0xFFFFFF))
                                                        .child("联系人列表 (Contacts)"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(10.5))
                                                        .text_color(rgb(0x8A9AA8))
                                                        .child("打开 Telegram 联系人弹窗"),
                                                ),
                                        ),
                                )
                                .child(div().text_size(px(11.0)).text_color(rgb(0x2AABEE)).child("→")),
                        ),
                )
            })
            // 1c. Interactive Creation Modal Panel (Add Contact: Create AI Bot / Add Friend, or Create Group)
            .when_some(creation_modal, |el, modal_kind| {
                el.child(self.render_creation_modal_panel(
                    modal_kind,
                    contact_kind,
                    active_field,
                    &primary_val,
                    &secondary_val,
                    &tertiary_val,
                    cx,
                ))
            })
            // 1d. Telegram 1:1 Categorized Search Panel when Search Box is clicked (`is_search_focused`)
            .when(is_search_focused, |el| {
                el.child(self.render_categorized_search_header(
                    &active_search_cat,
                    &active_search_tag,
                    &top_peers_list,
                    cx,
                ))
            })
            // 2. Suggestions Banner (Height 37px, dialogsImportantBarHeight: 37px)
            .when(has_suggestion && suggestion_item.is_some(), |el| {
                let item = suggestion_item.unwrap();
                el.child(
                    div()
                        .h(px(37.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(10.0))
                        .bg(rgb(0x242F3D))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().text_size(px(12.0)).child("⭐"))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .child(item.title),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .when(item.button_text.is_some(), |btn| {
                                    btn.child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(rgb(0x5288C1))
                                            .hover(|s| s.bg(rgb(0x2AABEE)))
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                                this.suggestions.is_visible = false;
                                                cx.notify();
                                            }))
                                            .child(item.button_text.unwrap()),
                                    )
                                })
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                            this.suggestions.is_visible = false;
                                            cx.notify();
                                        }))
                                        .child("✕"),
                                ),
                        ),
                )
            })
            // 3. Stories Horizontal Strip (preview thumbnails if available)
            .when(stories_count > 0, |el| {
                el.child(
                    div()
                        .h(px(44.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .child(
                            div()
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x8A9AA8))
                                .child("STORIES"),
                        )
                        .children(stories_list.into_iter().map(|s| {
                            let story_id = s.id;
                            let peer_name = s.peer_name.clone();
                            div()
                                .cursor_pointer()
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded_full()
                                .bg(rgb(s.avatar_color))
                                .border_2()
                                .border_color(if s.unread_count > 0 { rgb(0x5288C1) } else { rgb(0x6B808D) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.story_clicked_peer = Some(peer_name.clone());
                                    this.select_dialog(story_id);
                                    cx.notify();
                                }))
                                .child(s.avatar_initials.clone())
                        })),
                )
            })
            // 4. Top Frequently Contacted Peers Strip (if in search mode or at top)
            .when(!query.is_empty() && !top_peers_list.is_empty(), |el| {
                el.child(
                    div()
                        .h(px(52.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .children(top_peers_list.iter().map(|p| {
                            let peer_id = p.id;
                            div()
                                .cursor_pointer()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap(px(2.0))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.select_dialog(peer_id);
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .w(px(32.0))
                                        .h(px(32.0))
                                        .rounded_full()
                                        .bg(rgb(p.avatar_color))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .child(p.avatar_initials.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child(p.name.clone()),
                                )
                        })),
                )
            })
            // 5. Folder Tabs Strip (Height 36px, hidden when Categorized Search Mode is active)
            .when(!is_search_focused, |el| {
                el.child(
                    div()
                        .h(px(36.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(8.0))
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .children(folders_list.into_iter().map(|(folder, unread)| {
                            let is_active = folder == active_folder;
                            let folder_name = folder.clone();
                            div()
                                .cursor_pointer()
                                .relative()
                                .h_full()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .px(px(6.0))
                                .text_size(px(13.0))
                                .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::NORMAL })
                                .text_color(if is_active { rgb(0x5288C1) } else { rgb(0x8A9AA8) })
                                .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.select_folder(folder_name.clone());
                                    cx.notify();
                                }))
                                .child(folder)
                                .when(unread > 0, |el| {
                                    el.child(
                                        div()
                                            .h(px(16.0))
                                            .min_w(px(16.0))
                                            .px(px(4.0))
                                            .rounded_full()
                                            .bg(if is_active { rgb(0x5288C1) } else { rgb(0x242F3D) })
                                            .text_color(rgb(0xFFFFFF))
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(format!("{}", unread)),
                                    )
                                })
                                // Active Tab 2px Underline Indicator
                                .when(is_active, |el| {
                                    el.child(
                                        div()
                                            .absolute()
                                            .bottom_0()
                                            .left_0()
                                            .right_0()
                                            .h(px(2.0))
                                            .bg(rgb(0x5288C1)),
                                    )
                                })
                        })),
                )
            })
            // 6. Categorized Search Results OR Virtualized Dialog Rows
            .child(
                div()
                    .id("sidebar-dialogs-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .when(is_search_focused, |scroll| {
                        let show_recent_hint = query.is_empty() || categorized_results.is_empty();
                        scroll
                            .when(show_recent_hint, |s| {
                                s.child(
                                    div()
                                        .id("sidebar-search-empty-state")
                                        .w_full()
                                        .py(px(28.0))
                                        .px(px(16.0))
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(10.0))
                                        .border_b_1()
                                        .border_color(rgb(0x0E1621))
                                        .child(
                                            div()
                                                .text_size(px(36.0))
                                                .text_color(rgb(0x54687A))
                                                .child("🔍"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.5))
                                                .text_color(rgb(0x6B808D))
                                                .child("最近搜索结果"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.5))
                                                .text_color(rgb(0x6B808D))
                                                .child("将显示在此处。"),
                                        ),
                                )
                            })
                            .children(categorized_results.into_iter().map(|item| {
                            let chat_id = item.chat_id;
                            div()
                                .cursor_pointer()
                                .h(px(ROW_HEIGHT))
                                .flex_shrink_0()
                                .w_full()
                                .px(px(10.0))
                                .py(px(8.0))
                                .bg(rgb(0x17212B))
                                .hover(|s| s.bg(rgb(0x202B36)))
                                .border_b_1()
                                .border_color(rgb(0x0E1621))
                                .flex()
                                .items_center()
                                .gap(px(12.0))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.search_focused = false;
                                    this.select_dialog(chat_id);
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .w(px(42.0))
                                        .h(px(42.0))
                                        .rounded_full()
                                        .bg(rgb(item.avatar_color))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(rgb(0xFFFFFF))
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(14.0))
                                        .child(item.avatar_initials),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .child(
                                            div()
                                                .w_full()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(6.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.5))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(item.title),
                                                        )
                                                        .child(
                                                            div()
                                                                .px(px(5.0))
                                                                .py(px(1.0))
                                                                .rounded(px(4.0))
                                                                .bg(rgb(0x242F3D))
                                                                .text_size(px(10.0))
                                                                .text_color(rgb(0x2AABEE))
                                                                .child(item.category),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(rgb(0x8A9AA8))
                                                        .child(item.timestamp),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x8A9AA8))
                                                .child(item.subtitle),
                                        ),
                                )
                        }))
                    })
                    .when(!is_search_focused, |scroll| {
                        scroll.children(dialog_list.into_iter().map(|d| {
                            let dialog_id = d.id;
                            let is_selected = active_id == Some(dialog_id);
                            let bg_color = if is_selected {
                                rgb(0x2B5278) // Upstream Night active selection
                            } else {
                                rgb(0x17212B) // Upstream Night row bg
                            };

                            div()
                            .cursor_pointer()
                            .h(px(ROW_HEIGHT)) // Upstream exact metric: dialogsRowHeight: 62px
                            .flex_shrink_0()
                            .w_full()
                            .px(px(10.0))
                            .py(px(8.0)) // dialogsRowPadding: margins(10, 8, 10, 8)
                            .bg(bg_color)
                            .hover(move |s| if is_selected { s } else { s.bg(rgb(0x202B36)) })
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                this.search_focused = false;
                                this.select_dialog(dialog_id);
                                cx.notify();
                            }))
                            // Avatar (46px diameter)
                            .child(
                                div()
                                    .relative()
                                    .w(px(AVATAR_SIZE)) // dialogsPhotoSize: 46px
                                    .h(px(AVATAR_SIZE))
                                    .rounded_full()
                                    .bg(rgb(d.avatar_color))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xFFFFFF))
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(px(15.0))
                                    .child(d.avatar_initials.clone())
                                    .when(d.is_online, |el| {
                                        el.child(
                                            div()
                                                .absolute()
                                                .bottom_0()
                                                .right_0()
                                                .w(px(11.0))
                                                .h(px(11.0))
                                                .rounded_full()
                                                .bg(rgb(0x4CAF50))
                                                .border_2()
                                                .border_color(bg_color),
                                        )
                                    }),
                            )
                            // Content (Name, Timestamp, Preview, Badges)
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .flex_col()
                                    .justify_between()
                                    // Row 1: Name and Timestamp
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_size(px(14.0)) // nameFont
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(d.peer_name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0)) // dialogsDateFont
                                                    .text_color(rgb(0x8A9AA8))
                                                    .child(d.timestamp.clone()),
                                            ),
                                    )
                                    // Row 2: Preview snippet or typing indicator & badges
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                if let Some(ref typing) = d.typing_indicator {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(0x5288C1)) // Typing indicator accent
                                                        .child(typing.clone())
                                                } else {
                                                    div()
                                                        .text_size(px(13.0)) // dialogsTextFont
                                                        .text_color(rgb(0x8A9AA8))
                                                        .child(d.last_message.clone())
                                                },
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(4.0))
                                                    // Pin Icon
                                                    .when(d.is_pinned, |el| {
                                                        el.child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x6B808D))
                                                                .child("📌"),
                                                        )
                                                    })
                                                    // Mute Icon
                                                    .when(d.is_muted, |el| {
                                                        el.child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x6B808D))
                                                                .child("🔕"),
                                                        )
                                                    })
                                                    // Unread Count Pill Badge (Height 19px)
                                                    .when(d.unread_count > 0, |el| {
                                                        let badge_bg = if d.is_muted {
                                                            rgb(0x6B808D) // Upstream muted badge
                                                        } else {
                                                            rgb(0x5288C1) // Upstream active badge
                                                        };
                                                        el.child(
                                                            div()
                                                                .h(px(UNREAD_HEIGHT)) // dialogsUnreadHeight: 19px
                                                                .min_w(px(UNREAD_HEIGHT))
                                                                .px(px(DIALOG_UNREAD_PADDING)) // dialogsUnreadPadding: 5px
                                                                .rounded_full()
                                                                .bg(badge_bg)
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .text_size(px(11.0))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(format!("{}", d.unread_count)),
                                                        )
                                                    })
                                                    // Delivery Checkmarks when no unread badge
                                                    .when(d.unread_count == 0, |el| {
                                                        let check = match d.delivery_status {
                                                            DialogDeliveryStatus::Sending => "⏱️",
                                                            DialogDeliveryStatus::Sent => "✓",
                                                            DialogDeliveryStatus::Read => "✓✓",
                                                        };
                                                        el.child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x5288C1))
                                                                .child(check),
                                                        )
                                                    }),
                                            ),
                                    ),
                            )
                        }))
                    }),
            )
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
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_add_contact = modal_kind == SidebarCreationModal::AddContact;
        let is_ai_bot = contact_kind == AddContactKind::AiBot;

        let primary_label = if is_add_contact {
            if is_ai_bot { "AI Bot 名称" } else { "好友昵称 / 姓名" }
        } else {
            "群组名称 (Group Title)"
        };
        let secondary_label = if is_add_contact {
            if is_ai_bot { "Bot 用户名 (@bot)" } else { "手机号 / @用户名" }
        } else {
            "群成员 (@username 逗号分隔)"
        };
        let tertiary_label = if is_add_contact {
            if is_ai_bot { "AI 系统提示词 (Prompt)" } else { "备注信息" }
        } else {
            "群组简介 (Description)"
        };

        div()
            .id("sidebar-creation-modal-panel")
            .w_full()
            .px(px(10.0))
            .py(px(10.0))
            .bg(rgb(0x202B36))
            .border_b_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            .gap(px(8.0))
            // Header row with title & close button
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
                            .text_color(rgb(0xFFFFFF))
                            .child(if is_add_contact {
                                "添加联系人 (Add Contact)"
                            } else {
                                "创建群组 (New Group)"
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8A9AA8))
                            .hover(|s| s.text_color(rgb(0xFFFFFF)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.close_creation_modal();
                                cx.notify();
                            }))
                            .child("✕"),
                    ),
            )
            // Sub-mode switcher when in Add Contact mode: [创建 AI Bot] vs [添加好友]
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
                                .bg(if is_ai_bot { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.contact_kind = AddContactKind::AiBot;
                                    cx.notify();
                                }))
                                .child("🤖 创建 AI Bot"),
                        )
                        .child(
                            div()
                                .id("add-contact-tab-friend")
                                .cursor_pointer()
                                .flex_1()
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(if !is_ai_bot { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.contact_kind = AddContactKind::Friend;
                                    cx.notify();
                                }))
                                .child("👤 添加好友"),
                        ),
                )
            })
            // Primary Input Field
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .px(px(8.0))
                    .py(px(5.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(if active_field == CreationInputField::PrimaryName {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0x242F3D)
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_creation_field = CreationInputField::PrimaryName;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(primary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(if active_field == CreationInputField::PrimaryName {
                                format!("{}|", primary_val)
                            } else {
                                primary_val.to_string()
                            }),
                    ),
            )
            // Secondary Input Field
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .px(px(8.0))
                    .py(px(5.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(if active_field == CreationInputField::SecondaryHandle {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0x242F3D)
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_creation_field = CreationInputField::SecondaryHandle;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(secondary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(if active_field == CreationInputField::SecondaryHandle {
                                format!("{}|", secondary_val)
                            } else {
                                secondary_val.to_string()
                            }),
                    ),
            )
            // Tertiary Input Field
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .px(px(8.0))
                    .py(px(5.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(if active_field == CreationInputField::TertiaryPromptOrPhone {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0x242F3D)
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_creation_field = CreationInputField::TertiaryPromptOrPhone;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(tertiary_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(if active_field == CreationInputField::TertiaryPromptOrPhone {
                                format!("{}|", tertiary_val)
                            } else {
                                tertiary_val.to_string()
                            }),
                    ),
            )
            // Submit & Cancel action buttons
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
                            .bg(rgb(0x242F3D))
                            .text_size(px(11.5))
                            .text_color(rgb(0x8A9AA8))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.close_creation_modal();
                                cx.notify();
                            }))
                            .child("取消"),
                    )
                    .child(
                        div()
                            .id("creation-modal-submit-btn")
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x5288C1)))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.submit_creation_modal();
                                cx.notify();
                            }))
                            .child(if is_add_contact {
                                if is_ai_bot { "立即创建 AI Bot" } else { "立即添加好友" }
                            } else {
                                "立即创建群组"
                            }),
                    ),
            )
    }

    fn render_categorized_search_header(
        &self,
        active_search_cat: &str,
        active_search_tag: &str,
        top_peers_list: &[TopPeerItem],
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("sidebar-categorized-search-header")
            .w_full()
            .bg(rgb(0x17212B))
            .border_b_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            // 1. Horizontal Category Tab Bar directly under Search Box (`聊天 | 频道 | 应用 | 贴文 | 图片 | 视频 | 链接 | 文件 | 音乐 | 语音`)
            // Matches Telegram Desktop screenshot `media_1794178867127_7a80469d.jpg` with active blue bottom underline
            .child(
                div()
                    .id("sidebar-search-category-tabs")
                    .w_full()
                    .px(px(6.0))
                    .border_b_1()
                    .border_color(rgb(0x0E1621))
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(2.0))
                    .children(TELEGRAM_SEARCH_CATEGORIES.iter().map(|(cat_id, cat_label)| {
                        let is_active = active_search_cat == *cat_id;
                        let cid = (*cat_id).to_string();
                        div()
                            .cursor_pointer()
                            .relative()
                            .px(px(6.0))
                            .py(px(7.0))
                            .text_size(px(12.0))
                            .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .text_color(if is_active { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                            .hover(|s| s.text_color(rgb(0xFFFFFF)))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                this.select_search_category(cid.clone());
                                cx.notify();
                            }))
                            .child(*cat_label)
                            .when(is_active, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .bottom_0()
                                        .left_0()
                                        .right_0()
                                        .h(px(2.0))
                                        .bg(rgb(0x2AABEE)),
                                )
                            })
                    })),
            )
            // 2. "常用联系人" Section Header + Horizontal Circular Peer Avatars + Names
            .when(!top_peers_list.is_empty(), |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(12.0))
                        .pt(px(8.0))
                        .pb(px(8.0))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x8A9AA8))
                                .child("常用联系人"),
                        )
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .gap(px(12.0))
                                .children(top_peers_list.iter().map(|p| {
                                    let peer_id = p.id;
                                    let name = p.name.clone();
                                    let initials = p.avatar_initials.clone();
                                    let color = p.avatar_color;
                                    div()
                                        .cursor_pointer()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap(px(4.0))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            this.search_focused = false;
                                            this.select_dialog(peer_id);
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .w(px(40.0))
                                                .h(px(40.0))
                                                .rounded_full()
                                                .bg(rgb(color))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_size(px(13.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xFFFFFF))
                                                .child(initials),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(0xFFFFFF))
                                                .child(name),
                                        )
                                })),
                        ),
                )
            })
            // 3. Search Filter Tags Row (`DialogsSearchTags`)
            .child(
                div()
                    .w_full()
                    .px(px(8.0))
                    .py(px(5.0))
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(5.0))
                    .children(TELEGRAM_SEARCH_TAGS.iter().map(|(tag_id, tag_label)| {
                        let is_tag_active = active_search_tag == *tag_id;
                        let tag_str = (*tag_id).to_string();
                        div()
                            .cursor_pointer()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded_full()
                            .bg(if is_tag_active { rgb(0x5288C1) } else { rgb(0x202B36) })
                            .border_1()
                            .border_color(if is_tag_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                            .text_size(px(10.5))
                            .text_color(if is_tag_active { rgb(0xFFFFFF) } else { rgb(0x8A9AA8) })
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                this.select_search_tag(tag_str.clone());
                                cx.notify();
                            }))
                            .child(*tag_label)
                    })),
            )
    }
}
