//! GPUI Telegram History Message Text Selection Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_message_selection.h:L15-L198`

#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::BTreeSet;
use gpui::*;
use gpui_telegram_client::history::selection::{
    MessageSelection, MessageSelectionFlatEndpoint, MessageSelectionKind, TextSelection,
};

/// Upstream `history_message_selection.h:L93-L198`: Stage Message Selection State & Range Tracker
#[derive(Debug, Clone, Default)]
pub struct HistoryMessageSelection {
    pub inner: MessageSelection,
    pub is_active: bool,
    pub selected_message_ids: BTreeSet<u64>,
}

impl HistoryMessageSelection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_range(&mut self, anchor_symbol: u16, focus_symbol: u16, after_symbol: bool) {
        let anchor = MessageSelectionFlatEndpoint::new(anchor_symbol, false);
        let focus = MessageSelectionFlatEndpoint::new(focus_symbol, after_symbol);
        self.inner = MessageSelection::from_endpoints(anchor, focus);
        self.is_active = !self.inner.is_empty() || !self.selected_message_ids.is_empty();
    }

    pub fn toggle_message(&mut self, msg_id: u64) {
        if self.selected_message_ids.contains(&msg_id) {
            self.selected_message_ids.remove(&msg_id);
        } else {
            self.selected_message_ids.insert(msg_id);
        }
        self.is_active = !self.selected_message_ids.is_empty() || !self.inner.is_empty();
    }

    pub fn is_selected(&self, msg_id: u64) -> bool {
        self.selected_message_ids.contains(&msg_id)
    }

    pub fn clear(&mut self) {
        self.inner = MessageSelection::empty();
        self.selected_message_ids.clear();
        self.is_active = false;
    }

    pub fn contains_symbol(&self, symbol: u16) -> bool {
        let ep = MessageSelectionFlatEndpoint::new(symbol, false);
        self.inner.contains(&ep)
    }
}

impl Render for HistoryMessageSelection {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
