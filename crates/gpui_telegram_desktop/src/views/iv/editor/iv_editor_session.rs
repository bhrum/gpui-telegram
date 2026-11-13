//! Instant View Editor Session Orchestrator (`iv_editor_session.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Scope of the compose editor (inline thread or detached dialog).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComposeScope {
    #[default]
    Thread,
    Detached,
}

/// Delivery schedule policy when publishing from the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubmitPolicy {
    #[default]
    Immediate,
    Schedule,
}

/// Options configured when launching an Instant View compose window.
#[derive(Debug, Clone, Default)]
pub struct ComposeBoxOptions {
    pub scope: ComposeScope,
    pub submit_policy: SubmitPolicy,
    pub welcome_templates: bool,
}

/// Unique window identifier linking an editor to a specific peer/thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EditorWindowKey {
    pub peer_id: u64,
    pub topic_root_id: i32,
}

/// Registered active editor window.
#[derive(Debug, Clone)]
pub struct EditorWindowRecord {
    pub key: EditorWindowKey,
    pub title: String,
    pub is_compose: bool,
    pub created_at: i64,
}

/// Session-wide coordinator tracking open IV editor dialogs and feature gates.
#[derive(Debug, Clone)]
pub struct IvEditorSessionEditor {
    pub session_id: u64,
    pub is_premium: bool,
    pub stars_per_message: i32,
    pub open_windows: Vec<EditorWindowRecord>,
    pub is_active: bool,
}

impl Default for IvEditorSessionEditor {
    fn default() -> Self {
        Self {
            session_id: 1,
            is_premium: false,
            stars_per_message: 0,
            open_windows: Vec::new(),
            is_active: false,
        }
    }
}

impl IvEditorSessionEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_session(session_id: u64, is_premium: bool) -> Self {
        Self {
            session_id,
            is_premium,
            stars_per_message: 0,
            open_windows: Vec::new(),
            is_active: false,
        }
    }

    pub fn set_premium(&mut self, premium: bool) {
        self.is_premium = premium;
    }

    pub fn can_author_rich_messages(&self) -> bool {
        self.is_premium
    }

    pub fn premium_emoji_forbidden(&self, _peer_id: u64, is_custom: bool) -> bool {
        is_custom && !self.is_premium
    }

    pub fn is_compose_box_open(&self, peer_id: u64, topic_root_id: i32) -> bool {
        let key = EditorWindowKey { peer_id, topic_root_id };
        self.open_windows.iter().any(|w| w.key == key)
    }

    pub fn open_compose_box(&mut self, peer_id: u64, topic_root_id: i32, title: impl Into<String>) {
        let key = EditorWindowKey { peer_id, topic_root_id };
        if !self.is_compose_box_open(peer_id, topic_root_id) {
            self.open_windows.push(EditorWindowRecord {
                key,
                title: title.into(),
                is_compose: true,
                created_at: chrono::Utc::now().timestamp(),
            });
        }
    }

    pub fn close_window(&mut self, peer_id: u64, topic_root_id: i32) -> bool {
        let key = EditorWindowKey { peer_id, topic_root_id };
        if let Some(pos) = self.open_windows.iter().position(|w| w.key == key) {
            self.open_windows.remove(pos);
            true
        } else {
            false
        }
    }

    /// Synchronously cleans up all open windows on shutdown.
    /// Equivalent to C++ CloseAllWindows().
    pub fn close_all_windows(&mut self) {
        self.open_windows.clear();
    }
}

impl Render for IvEditorSessionEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Instant View Session #{}", self.session_id))
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Premium: {} | Open editor windows: {} | Stars per message: {}",
                        self.is_premium, self.open_windows.len(), self.stars_per_message
                    ))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.open_windows.iter().map(|w| {
                        div()
                            .text_size(px(11.0))
                            .child(format!("• {} (Peer ID: {})", w.title, w.key.peer_id))
                    }))
            )
    }
}
