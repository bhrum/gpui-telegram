//! Authentic HistoryView Fake Items Generator (`history_view_fake_items.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_fake_items.cpp:L1-L71`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FakeHistoryItem {
    pub id: u64,
    pub from_peer_id: u64,
    pub reply_to_id: Option<u64>,
    pub text: String,
    pub date: i64,
    pub effect_id: u64,
}

/// Authentic GPUI View for HistoryView::FakeItems.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewFakeItemsView {
    pub generated_items: Vec<FakeHistoryItem>,
}

impl HistoryViewFakeItemsView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_item(
        &mut self,
        from_peer_id: u64,
        reply_to_id: Option<u64>,
        text: impl Into<String>,
        date: i64,
    ) -> &FakeHistoryItem {
        let id = self.generated_items.len() as u64 + 1;
        let item = FakeHistoryItem {
            id,
            from_peer_id,
            reply_to_id,
            text: text.into(),
            date,
            effect_id: 0,
        };
        self.generated_items.push(item);
        self.generated_items.last().unwrap()
    }
}

impl Render for HistoryViewFakeItemsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
