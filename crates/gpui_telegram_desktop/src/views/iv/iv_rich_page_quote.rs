//! Target Symbol: IvRichPageQuote
//! Module 13: Instant View Rich Page Quote Indexing & Range Model
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const RICH_QUOTE_MAX_TEXT_DEPTH: usize = 64;
pub const MAX_QUOTE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RichQuotePoint {
    pub node_index: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RichQuoteRange {
    pub start: RichQuotePoint,
    pub end: RichQuotePoint,
}

impl RichQuoteRange {
    pub fn new(start: RichQuotePoint, end: RichQuotePoint) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start.node_index == self.end.node_index && self.start.offset == self.end.offset
    }
}

#[derive(Debug, Clone)]
pub struct IvRichPageQuote {
    pub page_id: u64,
    pub quote_text: String,
    pub range: Option<RichQuoteRange>,
    pub author: Option<String>,
    pub is_valid: bool,
}

impl Default for IvRichPageQuote {
    fn default() -> Self {
        Self {
            page_id: 1,
            quote_text: "Instant View articles allow readers to extract quoted excerpts directly into chat replies.".to_string(),
            range: Some(RichQuoteRange {
                start: RichQuotePoint { node_index: 0, offset: 0 },
                end: RichQuotePoint { node_index: 0, offset: 92 },
            }),
            author: Some("Telegram".to_string()),
            is_valid: true,
        }
    }
}

impl IvRichPageQuote {
    pub fn new(page_id: u64, text: impl Into<String>) -> Self {
        let t = text.into();
        let len = t.len();
        Self {
            page_id,
            quote_text: t,
            range: Some(RichQuoteRange {
                start: RichQuotePoint { node_index: 0, offset: 0 },
                end: RichQuotePoint { node_index: 0, offset: len },
            }),
            author: None,
            is_valid: len > 0 && len <= MAX_QUOTE_SIZE,
        }
    }

    pub fn word_count(&self) -> usize {
        self.quote_text.split_whitespace().count()
    }

    pub fn splits_formula(&self, text: &str, position: usize) -> bool {
        let mut in_formula = false;
        for (i, c) in text.char_indices() {
            if c == '$' {
                in_formula = !in_formula;
            }
            if i >= position {
                break;
            }
        }
        in_formula
    }
}

impl Render for IvRichPageQuote {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(12.0))
            .bg(rgb(0x1F242D))
            .rounded(px(8.0))
            .border_l_4()
            .border_color(rgb(0x2AABEE))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child(if let Some(auth) = &self.author {
                                format!("Quote from {}", auth)
                            } else {
                                "Quote Selection".to_string()
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x6E7684))
                            .child(format!("{} words", self.word_count())),
                    ),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xD2D7E0))
                    .italic()
                    .child(format!("\"{}\"", self.quote_text)),
            )
    }
}
