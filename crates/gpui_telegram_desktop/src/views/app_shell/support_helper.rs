//! SupportHelper: Volunteer support console utilities, peer diagnostics, and internal notes
//! Upstream: Telegram/SourceFiles/window/support_helper.h, support_helper.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct SupportHelper {
    pub current_user_id: Option<u64>,
    pub internal_notes: Vec<String>,
    pub is_agent_active: bool,
}

impl SupportHelper {
    pub fn new() -> Self {
        Self {
            current_user_id: None,
            internal_notes: Vec::new(),
            is_agent_active: true,
        }
    }

    pub fn inspect_user(&mut self, user_id: u64) {
        self.current_user_id = Some(user_id);
        self.internal_notes.clear();
    }

    pub fn append_note(&mut self, note: impl Into<String>) {
        self.internal_notes.push(note.into());
    }

    pub fn notes(&self) -> &[String] {
        &self.internal_notes
    }
}

impl Default for SupportHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SupportHelper {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SupportHelper: user={:?}, notes={}", self.current_user_id, self.internal_notes.len()))
    }
}
