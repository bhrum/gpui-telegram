//! Authentic HistoryView Rich Quote View Component (`history_view_rich_quote.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_rich_quote.cpp:L30-L330`

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::rich_quote::{RichQuoteData, RichQuoteView};

/// Authentic GPUI View for HistoryView::RichQuote.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewRichQuoteView {
    pub quote: Option<RichQuoteData>,
}

impl HistoryViewRichQuoteView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_quote(text: impl Into<String>) -> Self {
        Self {
            quote: Some(RichQuoteData::new(text)),
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        if let Some(ref mut q) = self.quote {
            q.author_name = Some(author.into());
        }
        self
    }
}

impl Render for HistoryViewRichQuoteView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(ref quote) = self.quote {
            div().child(quote.render())
        } else {
            div()
        }
    }
}
