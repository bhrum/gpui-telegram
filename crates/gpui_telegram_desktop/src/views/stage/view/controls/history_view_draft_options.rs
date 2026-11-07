//! Draft options dialog controller for tuning reply quotes, forward attribution, and webpage link previews.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_draft_options.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_draft_options.h`

use crate::views::composer::controls::{
    DraftOptionsFsm, DraftOptionsView, DraftSection, ForwardOptions, ReplyDraftOptions, WebPageDraftOptions,
};
use gpui::*;
use serde::{Deserialize, Serialize};

pub struct HistoryViewDraftOptionsViewControls {
    pub view: DraftOptionsView,
}

impl HistoryViewDraftOptionsViewControls {
    pub fn new() -> Self {
        Self {
            view: DraftOptionsView::new(),
        }
    }

    pub fn open(&mut self, section: DraftSection) {
        self.view.open(section);
    }

    pub fn close(&mut self) {
        self.view.close();
    }
}

impl Default for HistoryViewDraftOptionsViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewDraftOptionsViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self.view.is_open;
        let section_name = match self.view.fsm.active_section {
            DraftSection::Reply => "Reply",
            DraftSection::Forward => "Forward",
            DraftSection::Link => "Link Preview",
        };

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(rgb(0x1e2c3a))
            .rounded_md()
            .text_xs()
            .text_color(rgb(0xffffff))
            .child(format!("Draft options ({}): {}", section_name, if is_open { "Open" } else { "Closed" }))
    }
}
