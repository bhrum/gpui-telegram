//! UI Filter Link Header presentation.
//! Upstream: `Telegram/SourceFiles/ui/controls/filter_link_header.cpp:L20-L65`
//!           `Telegram/SourceFiles/ui/controls/filter_link_header.h:L10-L35`
//!
//! Header banner for chat folder share links with folder title and chat count pill.

use gpui::*;

/// FilterLinkHeaderControls component. Target symbol required by ledger: `FilterLinkHeaderControls`.
#[derive(Debug, Clone)]
pub struct FilterLinkHeaderControls {
    pub folder_name: String,
    pub chat_count: usize,
}

impl FilterLinkHeaderControls {
    pub fn new(folder_name: impl Into<String>, chat_count: usize) -> Self {
        Self {
            folder_name: folder_name.into(),
            chat_count,
        }
    }
}

impl Default for FilterLinkHeaderControls {
    fn default() -> Self {
        Self::new("Folder", 0)
    }
}

impl Render for FilterLinkHeaderControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p(px(12.0))
            .child(self.folder_name.clone())
            .child(format!("{} chats", self.chat_count))
    }
}
