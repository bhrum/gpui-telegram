//! SectionMemento: State capture and restoration token for back-navigation stacks
//! Upstream: Telegram/SourceFiles/window/section_memento.h

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMemento {
    pub section_type: String,
    pub scroll_offset: f32,
    pub highlight_msg_id: Option<i64>,
    pub draft_text: Option<String>,
    pub cursor_position: usize,
    pub timestamp: i64,
}

impl SectionMemento {
    pub fn new(section_type: impl Into<String>) -> Self {
        Self {
            section_type: section_type.into(),
            scroll_offset: 0.0,
            highlight_msg_id: None,
            draft_text: None,
            cursor_position: 0,
            timestamp: 0,
        }
    }

    pub fn with_scroll(mut self, offset: f32) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn with_highlight(mut self, msg_id: i64) -> Self {
        self.highlight_msg_id = Some(msg_id);
        self
    }

    pub fn with_draft(mut self, draft: impl Into<String>, cursor: usize) -> Self {
        self.draft_text = Some(draft.into());
        self.cursor_position = cursor;
        self
    }
}

impl Default for SectionMemento {
    fn default() -> Self {
        Self::new("default")
    }
}

impl Render for SectionMemento {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("Memento for [{}], scroll: {:.1}", self.section_type, self.scroll_offset))
    }
}
