//! Target Symbol: IvMarkdownArticleTextMarkdown
//! Module 13: Instant View Formatted Text Span Rendering & Entity Styling
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const FORMULA_EXACT_METRIC_SCALE: f32 = 1000.0;
pub const INLINE_BUTTON_LABEL_SPAN_CUTS_MAX: usize = 16;

#[derive(Debug, Clone, PartialEq)]
pub enum TextEntityKind {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
    Link(String),
    Spoiler,
    Formula(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextEntitySpan {
    pub start: usize,
    pub length: usize,
    pub kind: TextEntityKind,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleTextMarkdown {
    pub raw_text: String,
    pub entities: Vec<TextEntitySpan>,
    pub is_rtl: bool,
}

impl Default for IvMarkdownArticleTextMarkdown {
    fn default() -> Self {
        Self {
            raw_text: String::new(),
            entities: Vec::new(),
            is_rtl: false,
        }
    }
}

impl IvMarkdownArticleTextMarkdown {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            raw_text: text.into(),
            entities: Vec::new(),
            is_rtl: false,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.raw_text = text.into();
    }

    pub fn add_entity(&mut self, start: usize, length: usize, kind: TextEntityKind) {
        self.entities.push(TextEntitySpan { start, length, kind });
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
    }

    pub fn set_rtl(&mut self, is_rtl: bool) {
        self.is_rtl = is_rtl;
    }
}

impl Render for IvMarkdownArticleTextMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.raw_text.clone();
        let entity_count = self.entities.len();

        div()
            .flex()
            .flex_col()
            .py_1()
            .child(
                div()
                    .text_sm()
                    .line_height(relative(1.5))
                    .text_color(rgb(0xd0d5dc))
                    .child(text),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x606c78))
                    .child(format!("Entities: {}", entity_count)),
            )
    }
}
