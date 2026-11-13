//! Authentic HistoryView Cursor State Component (`history_view_cursor_state.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_cursor_state.cpp:L1-L115`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PointState {
    #[default]
    Outside,
    Inside,
    GroupPart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorState {
    #[default]
    None,
    Text,
    Date,
    Enlarge,
    Forwarded,
    FromPhoto,
    LogAdminService,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextState {
    pub item_id: u64,
    pub cursor: CursorState,
    pub symbol: u16,
    pub after_symbol: bool,
    pub over_message_text: bool,
    pub custom_tooltip: Option<String>,
}

impl TextState {
    pub fn new(item_id: u64) -> Self {
        Self {
            item_id,
            cursor: CursorState::None,
            symbol: 0,
            after_symbol: false,
            over_message_text: false,
            custom_tooltip: None,
        }
    }

    pub fn set_position(&mut self, symbol: u16, after_symbol: bool) {
        self.symbol = symbol;
        self.after_symbol = after_symbol;
    }

    pub fn add_offset(&mut self, offset: u16) {
        self.symbol = self.symbol.saturating_add(offset);
    }
}

/// Authentic GPUI View for HistoryView::CursorState.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewCursorStateView {
    pub point_state: PointState,
    pub text_state: TextState,
}

impl HistoryViewCursorStateView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_item(item_id: u64) -> Self {
        Self {
            point_state: PointState::Inside,
            text_state: TextState::new(item_id),
        }
    }
}

impl Render for HistoryViewCursorStateView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
