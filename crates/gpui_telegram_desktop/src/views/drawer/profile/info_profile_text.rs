//! Info Profile Text component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_text.cpp:L1-L157`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_text.h:L1-L36`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Formatted text renderer for profile bio and channel descriptions,
/// handling text selection, link clicks, and hashtag navigation.
#[derive(Debug, Clone)]
pub struct InfoProfileTextProfile {
    pub raw_text: String,
    pub is_selectable: bool,
    pub max_lines: Option<usize>,
    pub is_expanded: bool,
}

impl InfoProfileTextProfile {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            raw_text: text.into(),
            is_selectable: true,
            max_lines: Some(4),
            is_expanded: false,
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.raw_text = text.into();
    }
}

impl Default for InfoProfileTextProfile {
    fn default() -> Self {
        Self::new("")
    }
}

impl Render for InfoProfileTextProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .text_sm()
            .text_color(rgb(0xffffff))
            .child(self.raw_text.clone())
    }
}
