//! WindowSessionController: Master router for active user session, section stacks, and chat routing
//! Upstream: Telegram/SourceFiles/window/window_session_controller.h, window_session_controller.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationSection {
    DialogsList,
    History { peer_id: u64, msg_id: Option<i64> },
    TopicHistory { channel_id: u64, topic_id: i64 },
    PeerInfo { peer_id: u64 },
    Settings { section: String },
    Folder { folder_id: i32 },
    Forum { forum_id: u64 },
    MediaViewer { peer_id: u64, msg_id: i64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationEntry {
    pub section: NavigationSection,
    pub scroll_top: f32,
    pub draft_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WindowSessionController {
    pub session_id: String,
    pub active_peer_id: Option<u64>,
    pub active_folder_id: Option<i32>,
    pub active_topic_id: Option<i64>,
    pub search_query: Option<String>,
    pub search_in_chat_id: Option<u64>,
    pub navigation_stack: Vec<NavigationEntry>,
    pub current_section: NavigationSection,
    pub connecting_bottom_skip: i32,
    pub is_primary_window: bool,
    pub tabbed_selector_open: bool,
}

impl WindowSessionController {
    pub fn new() -> Self {
        Self::with_session("default_session".to_string())
    }

    pub fn with_session(session_id: String) -> Self {
        Self {
            session_id,
            active_peer_id: None,
            active_folder_id: None,
            active_topic_id: None,
            search_query: None,
            search_in_chat_id: None,
            navigation_stack: Vec::new(),
            current_section: NavigationSection::DialogsList,
            connecting_bottom_skip: 0,
            is_primary_window: true,
            tabbed_selector_open: false,
        }
    }

    pub fn show_peer_history(&mut self, peer_id: u64, msg_id: Option<i64>) {
        let entry = NavigationEntry {
            section: self.current_section.clone(),
            scroll_top: 0.0,
            draft_text: None,
        };
        self.navigation_stack.push(entry);
        self.active_peer_id = Some(peer_id);
        self.current_section = NavigationSection::History { peer_id, msg_id };
    }

    pub fn show_topic(&mut self, channel_id: u64, topic_id: i64) {
        let entry = NavigationEntry {
            section: self.current_section.clone(),
            scroll_top: 0.0,
            draft_text: None,
        };
        self.navigation_stack.push(entry);
        self.active_topic_id = Some(topic_id);
        self.current_section = NavigationSection::TopicHistory { channel_id, topic_id };
    }

    pub fn show_peer_info(&mut self, peer_id: u64) {
        let entry = NavigationEntry {
            section: self.current_section.clone(),
            scroll_top: 0.0,
            draft_text: None,
        };
        self.navigation_stack.push(entry);
        self.current_section = NavigationSection::PeerInfo { peer_id };
    }

    pub fn show_settings(&mut self, section: impl Into<String>) {
        let entry = NavigationEntry {
            section: self.current_section.clone(),
            scroll_top: 0.0,
            draft_text: None,
        };
        self.navigation_stack.push(entry);
        self.current_section = NavigationSection::Settings { section: section.into() };
    }

    pub fn navigate_back(&mut self) -> bool {
        if let Some(prev) = self.navigation_stack.pop() {
            match &prev.section {
                NavigationSection::History { peer_id, .. } => {
                    self.active_peer_id = Some(*peer_id);
                }
                NavigationSection::TopicHistory { topic_id, .. } => {
                    self.active_topic_id = Some(*topic_id);
                }
                NavigationSection::DialogsList => {
                    self.active_peer_id = None;
                    self.active_topic_id = None;
                }
                _ => {}
            }
            self.current_section = prev.section;
            true
        } else {
            false
        }
    }

    pub fn clear_section_stack(&mut self) {
        self.navigation_stack.clear();
        self.current_section = NavigationSection::DialogsList;
        self.active_peer_id = None;
        self.active_topic_id = None;
    }

    pub fn open_folder(&mut self, folder_id: i32) {
        self.active_folder_id = Some(folder_id);
        self.current_section = NavigationSection::Folder { folder_id };
    }

    pub fn close_folder(&mut self) {
        self.active_folder_id = None;
        self.current_section = NavigationSection::DialogsList;
    }

    pub fn search_in_chat(&mut self, chat_id: u64, query: Option<String>) {
        self.search_in_chat_id = Some(chat_id);
        self.search_query = query;
    }

    pub fn clear_search(&mut self) {
        self.search_query = None;
        self.search_in_chat_id = None;
    }

    pub fn set_connecting_bottom_skip(&mut self, skip: i32) {
        self.connecting_bottom_skip = skip;
    }

    pub fn set_active_chat_entry(&mut self, peer_id: u64) {
        self.active_peer_id = Some(peer_id);
    }

    pub fn active_chat_current(&self) -> Option<u64> {
        self.active_peer_id
    }

    pub fn toggle_tabbed_selector(&mut self) {
        self.tabbed_selector_open = !self.tabbed_selector_open;
    }
}

impl Default for WindowSessionController {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSessionController {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let section_title = match &self.current_section {
            NavigationSection::DialogsList => "Dialogs".to_string(),
            NavigationSection::History { peer_id, .. } => format!("Chat History [{}]", peer_id),
            NavigationSection::TopicHistory { channel_id, topic_id } => {
                format!("Topic [{}:{}]", channel_id, topic_id)
            }
            NavigationSection::PeerInfo { peer_id } => format!("Profile Info [{}]", peer_id),
            NavigationSection::Settings { section } => format!("Settings: {}", section),
            NavigationSection::Folder { folder_id } => format!("Folder {}", folder_id),
            NavigationSection::Forum { forum_id } => format!("Forum {}", forum_id),
            NavigationSection::MediaViewer { peer_id, msg_id } => {
                format!("Media Viewer [{}:{}]", peer_id, msg_id)
            }
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0E1621))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(40.0))
                    .px(px(16.0))
                    .bg(rgb(0x17212B))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(section_title)
                    )
                    .children((self.navigation_stack.len() > 0).then(|| {
                        div().text_sm().text_color(rgb(0x5288C1)).child("← Back")
                    }))
            )
            .child(
                div()
                    .flex_1()
                    .p(px(16.0))
                    .text_color(rgb(0x82868A))
                    .child(format!("Session: {}", self.session_id))
            )
    }
}
