//! State mutations, search filtering, creation modal submission, and action dispatch for [`ChatSidebar`].

use std::collections::{HashMap, HashSet};

use gpui::Context;

use crate::i18n::tr_str;
use crate::views::chat_sidebar::{
    AddContactKind, CategorizedSearchResult, ChatSidebar, CreationInputField, DialogDeliveryStatus,
    DialogItemModel, PendingAddContactRequest, PendingCreateGroupRequest, SidebarCreationModal,
};
use crate::views::nav_rail::{NavDestination, NavRail};
use crate::views::sidebar::dialogs_row_view::{SidebarContextAction, SidebarDialogMeta};
use crate::views::sidebar::gemini_unified_sidebar::UnifiedSidebarAction;
use crate::views::sidebar::suggestions::suggestion_manager::{
    SuggestionItem, SuggestionManager, SuggestionPriority,
};
use crate::views::sidebar::ui::dialogs_message_view::DialogMessageMediaBadge;
use crate::views::sidebar::ui::dialogs_stories_list::StoryElement;
use crate::views::sidebar::ui::top_peers_strip::TopPeerItem;

fn initials_or_fallback(label: &str, fallback: &str) -> String {
    let init: String = label.chars().take(2).collect::<String>().to_uppercase();
    if init.is_empty() {
        fallback.to_string()
    } else {
        init
    }
}

fn non_empty_trimmed(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

impl ChatSidebar {
    /// Constructs a demo sidebar populated with sample dialogs, stories, top peers, and notices for preview/tests.
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

    pub fn dialog_unread_contribution(&self, d: &DialogItemModel) -> u32 {
        if !self.include_muted_in_badge && d.is_muted {
            return 0;
        }
        let has_unread = d.unread_count > 0 || self.unread_marked_ids.contains(&d.id);
        if !has_unread {
            return 0;
        }
        if self.badge_count_messages {
            d.unread_count.max(1)
        } else {
            1
        }
    }

    pub fn total_unread_badge(&self) -> u32 {
        self.dialogs
            .iter()
            .map(|d| self.dialog_unread_contribution(d))
            .sum()
    }

    pub fn set_custom_folders(
        &mut self,
        mut folders: Vec<gpui_telegram_proto::contracts::chats::ChatFolderDto>,
    ) {
        folders.sort_by_key(|f| (f.sort_order, f.id));
        let removed_custom_titles: HashSet<String> = self
            .custom_folders
            .iter()
            .map(|f| f.title.trim().to_string())
            .filter(|old_t| !folders.iter().any(|nf| nf.title.trim() == old_t))
            .collect();
        if !removed_custom_titles.is_empty() {
            self.folders.retain(|(name, _)| {
                name == "All"
                    || !removed_custom_titles.contains(name)
                    || self.dialogs.iter().any(|d| d.folder == *name)
            });
        }
        self.custom_folders = folders;
        self.recompute_folder_badges();
    }

    pub fn set_notification_badge_rules(
        &mut self,
        badge_count_messages: bool,
        include_muted_in_badge: bool,
    ) {
        self.badge_count_messages = badge_count_messages;
        self.include_muted_in_badge = include_muted_in_badge;
        self.recompute_folder_badges();
    }

    pub fn dialog_matches_custom_folder(
        &self,
        d: &DialogItemModel,
        f: &gpui_telegram_proto::contracts::chats::ChatFolderDto,
    ) -> bool {
        let cid = d.id as i64;
        if f.exclude_chats.contains(&cid) {
            return false;
        }
        if f.include_chats.contains(&cid) || f.pinned_chats.contains(&cid) {
            return true;
        }
        if f.exclude_muted && d.is_muted {
            return false;
        }
        if f.exclude_read && d.unread_count == 0 && !self.unread_marked_ids.contains(&d.id) {
            return false;
        }
        if f.exclude_archived && self.archived_ids.contains(&d.id) {
            return false;
        }
        let lower = d.peer_name.to_lowercase();
        let is_bot = self.bot_ids.contains(&d.id) || d.folder == "Bots" || lower.ends_with("bot");
        let is_channel = self.channel_chat_ids.contains(&d.id) || d.folder == "Channels";
        let is_group = !is_channel
            && (self.group_chat_ids.contains(&d.id) || d.folder == "Groups" || d.folder == "Work");
        let is_saved = self.saved_messages_id == Some(d.id)
            || d.peer_name == "Saved Messages"
            || d.peer_name == "已保存消息"
            || d.peer_name == "收藏夹";
        let is_direct = !is_bot && !is_channel && !is_group && !is_saved;
        let is_contact =
            is_direct && (self.contact_chat_ids.contains(&d.id) || d.folder == "Personal");
        let is_non_contact = is_direct && !is_contact;

        (f.include_bots && is_bot)
            || (f.include_channels && is_channel)
            || (f.include_groups && is_group)
            || (f.include_contacts && is_contact)
            || (f.include_non_contacts && is_non_contact)
            || d.folder == f.title
    }

    pub fn recompute_folder_badges(&mut self) {
        let mut all_unread = 0u32;
        let mut folder_counts: HashMap<String, u32> = HashMap::new();
        for d in &self.dialogs {
            let contrib = self.dialog_unread_contribution(d);
            all_unread = all_unread.saturating_add(contrib);
            if !d.folder.is_empty() && d.folder != "All" && d.folder != "Unread" {
                let entry = folder_counts.entry(d.folder.clone()).or_insert(0);
                *entry = entry.saturating_add(contrib);
            }
        }
        let mut next_folders = vec![("All".to_string(), all_unread)];
        let mut seen: HashSet<String> = HashSet::from(["All".to_string()]);

        for cf in &self.custom_folders {
            let title = cf.title.trim();
            if title.is_empty() || title == "All" || !seen.insert(title.to_string()) {
                continue;
            }
            folder_counts.remove(title);
            let badge: u32 = self
                .dialogs
                .iter()
                .filter(|d| self.dialog_matches_custom_folder(d, cf))
                .map(|d| self.dialog_unread_contribution(d))
                .sum();
            next_folders.push((title.to_string(), badge));
        }

        for (existing_name, _) in &self.folders {
            let is_builtin = matches!(
                existing_name.as_str(),
                "Personal" | "Work" | "Groups" | "Channels" | "Bots"
            );
            if is_builtin && seen.insert(existing_name.clone()) {
                let cnt = folder_counts.remove(existing_name).unwrap_or(0);
                next_folders.push((existing_name.clone(), cnt));
            }
        }
        let mut extra: Vec<(String, u32)> = folder_counts
            .into_iter()
            .filter(|(name, _)| seen.insert(name.clone()))
            .collect();
        extra.sort_by(|a, b| a.0.cmp(&b.0));
        next_folders.extend(extra);
        if all_unread > 0 || self.folders.iter().any(|(n, _)| n == "Unread") {
            if seen.insert("Unread".to_string()) {
                next_folders.push(("Unread".to_string(), all_unread));
            }
        }
        self.folders = next_folders;
        if self.active_folder != "All"
            && !self.folders.iter().any(|(n, _)| n == &self.active_folder)
        {
            self.active_folder = "All".to_string();
        }
        self.inner_widget.total_rows = self.dialogs.len();
    }

    fn reset_creation_inputs(&mut self, modal: SidebarCreationModal) {
        self.is_plus_menu_open = false;
        self.is_account_popover_open = false;
        self.search_focused = false;
        self.active_creation_modal = Some(modal);
        self.active_creation_field = CreationInputField::PrimaryName;
        self.creation_primary_input.clear();
        self.creation_secondary_input.clear();
        self.creation_tertiary_input.clear();
    }

    pub fn open_add_contact_modal(&mut self, kind: AddContactKind) {
        self.hovered_rail_dialog_id = None;
        self.is_contacts_modal_open = false;
        self.contact_kind = kind;
        self.reset_creation_inputs(SidebarCreationModal::AddContact);
    }

    pub fn open_create_group_modal(&mut self) {
        self.reset_creation_inputs(SidebarCreationModal::CreateGroup);
    }

    pub fn close_creation_modal(&mut self) {
        self.active_creation_modal = None;
        self.is_plus_menu_open = false;
        self.creation_primary_input.clear();
        self.creation_secondary_input.clear();
        self.creation_tertiary_input.clear();
    }

    fn creation_field_mut(&mut self) -> &mut String {
        match self.active_creation_field {
            CreationInputField::PrimaryName => &mut self.creation_primary_input,
            CreationInputField::SecondaryHandle => &mut self.creation_secondary_input,
            CreationInputField::TertiaryPromptOrPhone => &mut self.creation_tertiary_input,
        }
    }

    pub fn active_creation_field_text(&self) -> String {
        match self.active_creation_field {
            CreationInputField::PrimaryName => self.creation_primary_input.clone(),
            CreationInputField::SecondaryHandle => self.creation_secondary_input.clone(),
            CreationInputField::TertiaryPromptOrPhone => self.creation_tertiary_input.clone(),
        }
    }

    pub fn push_creation_text(&mut self, text: &str) {
        self.creation_field_mut().push_str(text);
    }

    pub fn backspace_creation_text(&mut self) {
        self.creation_field_mut().pop();
    }

    pub fn clear_creation_text(&mut self) {
        self.creation_field_mut().clear();
    }

    pub fn cycle_creation_field(&mut self) {
        self.active_creation_field = match self.active_creation_field {
            CreationInputField::PrimaryName => CreationInputField::SecondaryHandle,
            CreationInputField::SecondaryHandle => CreationInputField::TertiaryPromptOrPhone,
            CreationInputField::TertiaryPromptOrPhone => CreationInputField::PrimaryName,
        };
    }

    fn insert_created_dialog(&mut self, dialog: DialogItemModel) {
        let local_id = dialog.id;
        self.dialogs.insert(0, dialog);
        self.inner_widget.total_rows = self.dialogs.len();
        self.inner_widget.selected_index = Some(0);
        self.active_chat_id = Some(local_id);
        self.selection_revision = self.selection_revision.wrapping_add(1);
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
                let lang = self.language_id.as_str();
                let name = non_empty_trimmed(&self.creation_primary_input).unwrap_or_else(|| {
                    if is_bot {
                        tr_str(lang, "智能助手", "AI Assistant").to_string()
                    } else {
                        tr_str(lang, "新联系人", "New Contact").to_string()
                    }
                });

                let handle = {
                    let raw = self.creation_secondary_input.trim().trim_start_matches('@');
                    if raw.is_empty() {
                        if is_bot { "custom_ai_bot" } else { "friend_user" }.to_string()
                    } else if is_bot && !raw.ends_with("bot") {
                        format!("{}_bot", raw)
                    } else {
                        raw.to_string()
                    }
                };

                let tertiary = non_empty_trimmed(&self.creation_tertiary_input);
                let (avatar_initials, avatar_color, preview_msg, folder) = if is_bot {
                    let default_prompt = tr_str(
                        lang,
                        "通用智能助手，支持问答、代码与翻译",
                        "General assistant for Q&A, code, and translation",
                    );
                    let prompt_preview = tertiary.as_deref().unwrap_or(default_prompt);
                    (
                        initials_or_fallback(&name, "BOT"),
                        0x1ABC9C,
                        if lang.eq_ignore_ascii_case("en") {
                            format!("AI Bot (@{}) • {}", handle, prompt_preview)
                        } else {
                            format!("AI 助手 (@{}) • {}", handle, prompt_preview)
                        },
                        "Bots".to_string(),
                    )
                } else {
                    (
                        initials_or_fallback(&name, "FR"),
                        0x2AABEE,
                        if lang.eq_ignore_ascii_case("en") {
                            format!("Added @{} to contacts", handle)
                        } else {
                            format!("已添加好友 @{}，开始聊天吧", handle)
                        },
                        "Personal".to_string(),
                    )
                };
                let now_label = tr_str(lang, "刚刚", "Just now").to_string();

                self.insert_created_dialog(DialogItemModel {
                    id: local_id,
                    peer_name: name.clone(),
                    avatar_initials,
                    avatar_color,
                    last_message: preview_msg,
                    timestamp: now_label,
                    unread_count: 1,
                    is_muted: false,
                    is_pinned: false,
                    is_online: true,
                    typing_indicator: None,
                    delivery_status: DialogDeliveryStatus::Read,
                    folder,
                });
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
                let title = non_empty_trimmed(&self.creation_primary_input)
                    .unwrap_or_else(|| tr_str(lang, "新建群组", "New Group").to_string());

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

                let about = non_empty_trimmed(&self.creation_tertiary_input);
                let initials = initials_or_fallback(&title, "GR");
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

                let now_label = tr_str(lang, "刚刚", "Just now").to_string();
                self.insert_created_dialog(DialogItemModel {
                    id: local_id,
                    peer_name: title.clone(),
                    avatar_initials: initials,
                    avatar_color: 0x9B59B6,
                    last_message: preview_msg,
                    timestamp: now_label,
                    unread_count: 0,
                    is_muted: false,
                    is_pinned: false,
                    is_online: true,
                    typing_indicator: None,
                    delivery_status: DialogDeliveryStatus::Read,
                    folder: "Groups".to_string(),
                });

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

    pub fn filtered_dialogs(&self) -> Vec<&DialogItemModel> {
        let q = self.search_query.to_lowercase();
        let active_cf = self
            .custom_folders
            .iter()
            .find(|cf| cf.title == self.active_folder);
        let mut out: Vec<&DialogItemModel> = self
            .dialogs
            .iter()
            .filter(|d| {
                let matches_folder = if self.active_folder == "All" {
                    true
                } else if let Some(cf) = active_cf {
                    self.dialog_matches_custom_folder(d, cf)
                } else if self.active_folder == "Unread" {
                    d.unread_count > 0 || self.unread_marked_ids.contains(&d.id)
                } else if self.active_folder == "Bots" {
                    d.folder == "Bots" || self.bot_ids.contains(&d.id)
                } else if self.active_folder == "Groups" {
                    d.folder == "Groups" || self.group_chat_ids.contains(&d.id)
                } else if self.active_folder == "Channels" {
                    d.folder == "Channels" || self.channel_chat_ids.contains(&d.id)
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
            .collect();
        if let Some(cf) = active_cf {
            if !cf.pinned_chats.is_empty() {
                out.sort_by_key(|d| {
                    cf.pinned_chats
                        .iter()
                        .position(|&pid| pid == d.id as i64)
                        .unwrap_or(usize::MAX)
                });
            }
        }
        out
    }

    /// Computes Telegram 1:1 categorized search results combining local dialogs/messages
    /// and remote server search results for the active search category & tag.
    pub fn categorized_search_results(&self) -> Vec<CategorizedSearchResult> {
        let q = self.search_query.trim().to_lowercase();
        let cat = self.active_search_category.to_lowercase();
        let tag = self.active_search_tag.as_str();

        let mut out = Vec::new();
        let mut seen = HashSet::new();

        for d in &self.dialogs {
            let is_bot = d.folder == "Bots"
                || self.bot_ids.contains(&d.id)
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
                "channels" if is_channel || d.folder == "Work" || d.folder == "Groups" => "Channels",
                "apps" if is_bot => "Apps",
                "posts" if !is_bot => "Posts",
                "media" if has_media => "Media",
                "downloads" if has_file || has_media || last_lower.contains("download") => {
                    "Downloads"
                }
                "links" if has_link => "Links",
                "files" if has_file => "Files",
                "music" if has_music => "Music",
                "voice" if has_voice => "Voice",
                "channels" | "apps" | "posts" | "media" | "downloads" | "links" | "files"
                | "music" | "voice" => continue,
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
                "apps" => format!("{} • {}", tr_str(lang, "AI Bot / 小程序", "AI Bot / Mini App"), d.last_message),
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

    pub fn toggle_archive_dialog(&mut self, id: u64) {
        if let Some(pos) = self.archived_ids.iter().position(|&x| x == id) {
            self.archived_ids.remove(pos);
        } else {
            self.archived_ids.push(id);
        }
        self.recompute_folder_badges();
    }

    pub fn toggle_mute_dialog(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.is_muted = !d.is_muted;
        }
        self.recompute_folder_badges();
    }

    pub fn toggle_pin_dialog(&mut self, id: u64) {
        if let Some(d) = self.dialogs.iter_mut().find(|d| d.id == id) {
            d.is_pinned = !d.is_pinned;
        }
        self.recompute_folder_badges();
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
        let mut folder_tags = Vec::new();
        if self.folder_tags_enabled {
            for cf in &self.custom_folders {
                if !cf.title.trim().is_empty()
                    && cf.title != "All"
                    && self.dialog_matches_custom_folder(item, cf)
                {
                    folder_tags.push(cf.title.clone());
                    if folder_tags.len() >= 2 {
                        break;
                    }
                }
            }
            if folder_tags.is_empty() && !item.folder.is_empty() && item.folder != "All" {
                let localized = match item.folder.as_str() {
                    "Bots" => tr_str(&self.language_id, "机器人", "Bots").to_string(),
                    "Groups" => tr_str(&self.language_id, "群组", "Groups").to_string(),
                    "Channels" => tr_str(&self.language_id, "频道", "Channels").to_string(),
                    "Personal" => tr_str(&self.language_id, "私聊", "Personal").to_string(),
                    "Work" => tr_str(&self.language_id, "工作", "Work").to_string(),
                    other => other.to_string(),
                };
                folder_tags.push(localized);
            }
        }
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
            folder_tags,
        }
    }

    fn update_nav_rail(&mut self, cx: &mut Context<Self>, update: impl FnOnce(&mut NavRail)) {
        self.is_account_popover_open = false;
        if let Some(rail) = self.nav_rail.clone() {
            rail.update(cx, |r, r_cx| {
                update(r);
                r_cx.notify();
            });
        }
    }

    /// Dispatches actions from the Gemini-style unified collapsed rail and expanded sidebar chrome.
    pub fn handle_unified_action(
        &mut self,
        action: UnifiedSidebarAction,
        cx: &mut Context<Self>,
    ) {
        match action {
            UnifiedSidebarAction::ToggleCollapsed => {
                self.toggle_collapsed_from_user();
                cx.notify();
            }
            UnifiedSidebarAction::SelectDestination(dest) => {
                self.standalone_active_destination = dest;
                self.context_menu = None;
                if dest != NavDestination::Chats {
                    self.is_plus_menu_open = false;
                }
                self.update_nav_rail(cx, |r| r.select(dest));
                cx.notify();
            }
            UnifiedSidebarAction::SelectDialog(dialog_id) => {
                self.is_account_popover_open = false;
                self.is_plus_menu_open = false;
                self.context_menu = None;
                self.select_dialog_from_user(dialog_id);
                cx.notify();
            }
            UnifiedSidebarAction::OpenMainMenu => {
                self.is_account_popover_open = !self.is_account_popover_open;
                self.is_plus_menu_open = false;
                self.context_menu = None;
                cx.notify();
            }
            UnifiedSidebarAction::OpenAddFriend => {
                self.expand_from_user_if_collapsed();
                self.is_account_popover_open = false;
                self.open_add_contact_modal(AddContactKind::Friend);
                cx.notify();
            }
            UnifiedSidebarAction::ToggleTheme => {
                self.update_nav_rail(cx, |r| r.theme_toggle_requested = true);
                cx.notify();
            }
            UnifiedSidebarAction::Logout => {
                self.update_nav_rail(cx, |r| r.logout_requested = true);
                cx.notify();
            }
            UnifiedSidebarAction::FocusSearch => {
                self.focus_search();
                cx.notify();
            }
            UnifiedSidebarAction::ExitSearch => {
                self.select_folder("All");
                self.exit_search();
                self.close_creation_modal();
                cx.notify();
            }
            UnifiedSidebarAction::ClearSearch => {
                if !self.search_query.is_empty() {
                    self.set_search_query("");
                } else {
                    self.exit_search();
                }
                cx.notify();
            }
            UnifiedSidebarAction::TogglePlusMenu => {
                self.toggle_plus_menu();
                cx.notify();
            }
            UnifiedSidebarAction::ToggleNavMore => {
                self.is_nav_more_expanded = !self.is_nav_more_expanded;
                cx.notify();
            }
            UnifiedSidebarAction::OpenContacts => {
                self.expand_from_user_if_collapsed();
                self.open_contacts_modal();
                cx.notify();
            }
            UnifiedSidebarAction::SetHover(item) => {
                if self.hovered_rail_item != item {
                    self.hovered_rail_item = item;
                    cx.notify();
                }
            }
            UnifiedSidebarAction::SetHoverDialog(dialog_id) => {
                if self.hovered_rail_dialog_id != dialog_id {
                    self.hovered_rail_dialog_id = dialog_id;
                    cx.notify();
                }
            }
        }
    }
}
