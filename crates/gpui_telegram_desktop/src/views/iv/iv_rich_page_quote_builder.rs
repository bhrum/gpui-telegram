//! Target Symbol: IvRichPageQuoteBuilder
//! Module 13: Instant View Reply Quote Fragment Synthesis & Cutter Pipeline
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_FRAGMENTS: usize = 16;
pub const MAX_LENGTH: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteFragment {
    pub text: String,
    pub is_code: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltRichQuote {
    pub fragments: Vec<QuoteFragment>,
    pub offset: usize,
    pub overflown: bool,
}

#[derive(Debug, Clone)]
pub struct IvRichPageQuoteBuilder {
    pub length_limit: usize,
    pub max_fragments: usize,
    pub last_built_quote: Option<BuiltRichQuote>,
}

impl Default for IvRichPageQuoteBuilder {
    fn default() -> Self {
        Self {
            length_limit: MAX_LENGTH,
            max_fragments: MAX_FRAGMENTS,
            last_built_quote: None,
        }
    }
}

impl IvRichPageQuoteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_quote(&mut self, source: &str, length_limit: usize) -> BuiltRichQuote {
        let limit = length_limit.min(MAX_LENGTH);
        let trimmed = source.trim();

        if trimmed.len() <= limit {
            let quote = BuiltRichQuote {
                fragments: vec![QuoteFragment {
                    text: trimmed.to_string(),
                    is_code: false,
                }],
                offset: 0,
                overflown: false,
            };
            self.last_built_quote = Some(quote.clone());
            return quote;
        }

        // Needs truncation
        let budget = limit.saturating_sub(3); // space for "..."
        let mut char_count = 0;
        let mut clipped_str = String::with_capacity(limit);

        for ch in trimmed.chars() {
            if char_count + ch.len_utf8() > budget {
                break;
            }
            clipped_str.push(ch);
            char_count += ch.len_utf8();
        }

        // Snap to last word boundary if possible
        if let Some(last_space) = clipped_str.rfind(' ') {
            if last_space > budget / 2 {
                clipped_str.truncate(last_space);
            }
        }
        clipped_str.push_str("...");

        let quote = BuiltRichQuote {
            fragments: vec![QuoteFragment {
                text: clipped_str,
                is_code: false,
            }],
            offset: 0,
            overflown: true,
        };
        self.last_built_quote = Some(quote.clone());
        quote
    }
}

impl Render for IvRichPageQuoteBuilder {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (content, overflown, frag_count) = match &self.last_built_quote {
            Some(q) => {
                let first = q.fragments.first().map(|f| f.text.as_str()).unwrap_or("");
                (first.to_string(), q.overflown, q.fragments.len())
            }
            None => ("No quote built".to_string(), false, 0),
        };

        div()
            .flex()
            .flex_col()
            .p(px(10.0))
            .bg(rgb(0x1F232B))
            .rounded(px(6.0))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xCCD2DE))
                            .child("Synthesized Reply Quote"),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(if overflown { rgb(0xF5BF4F) } else { rgb(0x61C554) })
                            .child(if overflown { "TRUNCATED" } else { "EXACT" }),
                    ),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xD8DEE9))
                    .child(content),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x6E7787))
                    .child(format!("Fragments: {} | Limit: {} chars", frag_count, self.length_limit)),
            )
    }
}
