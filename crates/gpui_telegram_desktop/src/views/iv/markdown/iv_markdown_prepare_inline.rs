//! Target Symbol: IvMarkdownPrepareInlineMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_inline.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct InlineFormulaSource {
    pub tex: String,
    pub offset: usize,
}

#[derive(Debug, Clone, Default)]
pub struct InlineFormulaContext {
    pub base_font_size: f32,
    pub line_height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct RawInlineTag {
    pub tag: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Default)]
pub struct DecodedDisplaySpan {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_code: bool,
    pub is_spoiler: bool,
    pub link_url: Option<String>,
}

pub fn finalize_prepared_external_link(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("tg://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

pub fn append_text_with_inline_formulas(
    base_text: &str,
    formulas: &[InlineFormulaSource],
) -> Vec<DecodedDisplaySpan> {
    let mut spans = Vec::new();
    let mut cursor = 0;

    for f in formulas {
        if f.offset > cursor && f.offset <= base_text.len() {
            spans.push(DecodedDisplaySpan {
                text: base_text[cursor..f.offset].to_string(),
                is_bold: false,
                is_italic: false,
                is_code: false,
                is_spoiler: false,
                link_url: None,
            });
            cursor = f.offset;
        }
        spans.push(DecodedDisplaySpan {
            text: format!("${}$", f.tex),
            is_bold: false,
            is_italic: false,
            is_code: true,
            is_spoiler: false,
            link_url: None,
        });
    }

    if cursor < base_text.len() {
        spans.push(DecodedDisplaySpan {
            text: base_text[cursor..].to_string(),
            is_bold: false,
            is_italic: false,
            is_code: false,
            is_spoiler: false,
            link_url: None,
        });
    }

    spans
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareInlineMarkdown {
    pub spans: Vec<DecodedDisplaySpan>,
    pub total_spans_processed: usize,
}

impl Default for IvMarkdownPrepareInlineMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareInlineMarkdown {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            total_spans_processed: 0,
        }
    }

    pub fn append_inline(&mut self, span: DecodedDisplaySpan) {
        self.total_spans_processed += 1;
        self.spans.push(span);
    }
}

impl Render for IvMarkdownPrepareInlineMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.spans.len();
        let total = self.total_spans_processed;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1e2124))
            .text_color(rgb(0xf0f0f0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Inline Span Formatting Engine"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Active Spans: {}", active))
                    .child(format!("Total Processed: {}", total)),
            )
    }
}
