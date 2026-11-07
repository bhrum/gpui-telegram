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

pub struct ChatSidebar {
    pub search_query: String,
    pub active_folder: String,
    pub active_chat_id: Option<u64>,
    pub dialogs: Vec<DialogItemModel>,
    pub folders: Vec<(String, u32)>,
    pub search_focused: bool,
    pub story_clicked_peer: Option<String>,

    // Authentic submodules wired per Mandates #17 & #18
    pub widget: DialogsWidget,
    pub inner_widget: DialogsInnerWidget,
    pub stories: DialogsStoriesListUi,
    pub top_peers: TopPeersStripUi,
    pub suggestions: DialogsSuggestionsUi,
    pub suggestion_manager: SuggestionManager,
    pub style: DialogsStyleSheet,
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
            search_query: String::new(),
            active_folder: "All".to_string(),
            active_chat_id: Some(1),
            dialogs,
            folders,
            search_focused: false,
            story_clicked_peer: None,
            widget: DialogsWidget::new(),
            inner_widget,
            stories,
            top_peers,
            suggestions,
            suggestion_manager,
            style: DialogsStyleSheet::new(),
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        let q = query.into();
        self.search_query = q.clone();
        self.widget.on_search_text_changed(q);
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

    pub fn selected_dialog(&self) -> Option<&DialogItemModel> {
        self.active_chat_id
            .and_then(|id| self.dialogs.iter().find(|d| d.id == id))
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
        let dialog_list: Vec<DialogItemModel> = self.filtered_dialogs().into_iter().cloned().collect();
        let folders_list = self.folders.clone();
        let stories_list: Vec<StoryElement> = self.stories.elements.iter().take(3).cloned().collect();
        let has_suggestion = self.suggestions.is_visible;
        let suggestion_item = self.suggestions.current_item.clone();
        let stories_count = self.stories.collapsed_preview_count();
        let top_peers_list = self.top_peers.entries.clone();

        div()
            .w(px(288.0))
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x17212B))
            .border_r_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            // 1. Universal Search Header (Height 48px)
            .child(
                div()
                    .h(px(48.0))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(10.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Menu / Hamburger icon (resets filter to All)
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(rgb(0x8A9AA8))
                            .hover(|s| s.text_color(rgb(0xFFFFFF)))
                            .text_size(px(16.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.select_folder("All");
                                this.set_search_query("");
                                this.search_focused = false;
                                cx.notify();
                            }))
                            .child("☰"),
                    )
                    // Search Input field (rounded 16px, background #242F3D)
                    .child(
                        div()
                            .cursor_pointer()
                            .flex_1()
                            .h(px(32.0))
                            .bg(rgb(0x242F3D))
                            .border_1()
                            .border_color(if is_search_focused { rgb(0x2AABEE) } else { rgba(0x00000000) })
                            .rounded(px(16.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.search_focused = true;
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
                                            .text_size(px(13.0))
                                            .child(if query.is_empty() {
                                                if is_search_focused {
                                                    "Type to filter chats...".to_string()
                                                } else {
                                                    "Search (Cmd+K)".to_string()
                                                }
                                            } else {
                                                query.clone()
                                            }),
                                    ),
                            )
                            .when(!query.is_empty(), |el| {
                                el.child(
                                    div()
                                        .cursor_pointer()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                            this.set_search_query("");
                                            cx.notify();
                                        }))
                                        .child("✕"),
                                )
                            }),
                    ),
            )
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
            // 5. Folder Tabs Strip (Height 36px)
            .child(
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
            // 6. Virtualized Dialog Rows (Each row exactly 62px height)
            .child(
                div()
                    .id("sidebar-dialogs-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .children(dialog_list.into_iter().map(|d| {
                        let dialog_id = d.id;
                        let is_selected = active_id == Some(dialog_id);
                        let bg_color = if is_selected {
                            rgb(0x2B5278) // Upstream Night active selection
                        } else {
                            rgb(0x17212B) // Upstream Night row bg
                        };

                        div()
                            .cursor_pointer()
                            .h(px(62.0)) // Upstream exact metric: dialogsRowHeight: 62px
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
                                    .w(px(46.0)) // dialogsPhotoSize: 46px
                                    .h(px(46.0))
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
                                                                .h(px(19.0)) // dialogsUnreadHeight: 19px
                                                                .min_w(px(19.0))
                                                                .px(px(5.0)) // dialogsUnreadPadding: 5px
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
                    })),
            )
    }
}
