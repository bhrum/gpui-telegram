//! History View Double-Click Quick Action View
//! Faithful upstream C++ extraction:
//! - `history_view_quick_action.cpp:L1-L19`
//! - `history_view_quick_action.h:L10-L20`

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DoubleClickQuickAction {
    #[default]
    Reply,
    React,
    None,
}

#[derive(Debug, Clone)]
pub struct HistoryViewQuickActionView {
    pub current_action: DoubleClickQuickAction,
    pub favorite_reaction: String,
}

impl HistoryViewQuickActionView {
    pub fn new() -> Self {
        Self {
            current_action: DoubleClickQuickAction::Reply,
            favorite_reaction: "❤️".to_string(),
        }
    }

    pub fn set_action(&mut self, action: DoubleClickQuickAction) {
        self.current_action = action;
    }

    pub fn set_favorite_reaction(&mut self, reaction: String) {
        self.favorite_reaction = reaction;
    }

    pub fn handle_double_click(&self, _message_id: u64) -> Option<String> {
        match self.current_action {
            DoubleClickQuickAction::Reply => Some("reply".to_string()),
            DoubleClickQuickAction::React => Some(format!("react:{}", self.favorite_reaction)),
            DoubleClickQuickAction::None => None,
        }
    }
}

impl Default for HistoryViewQuickActionView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewQuickActionView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .text_size(px(12.0))
            .text_color(rgb(0x666666))
            .child(format!("Quick action: {:?}", self.current_action))
    }
}
