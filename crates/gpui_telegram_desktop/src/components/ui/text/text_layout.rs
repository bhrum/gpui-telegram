//! Authentic GPUI Implementation: Core UiTextString Rich Text Layout Engine
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Reference: `Telegram/SourceFiles/tests/test_text.cpp:L72-L98`, `L131-L165`

use gpui::*;
use super::text_word::{tokenize_words, wrap_words_into_lines, TextLine};
use super::text_entity::{TextEntitySpan, TextEntityType};
use super::text_parser::{parse_text_entities, ParsedRichText};
use super::text_options::{TextParseOptions, item_text_default_options};

/// Selection span range within text string.
/// Citing `Telegram/SourceFiles/tests/test_text.cpp:L75`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextSelection {
    pub from: usize,
    pub to: usize,
}

impl TextSelection {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn is_empty(&self) -> bool {
        self.from >= self.to
    }

    pub fn contains(&self, index: usize) -> bool {
        index >= self.from && index < self.to
    }
}

/// The core rich text layout model matching `Ui::Text::String`.
/// Citing `Telegram/SourceFiles/tests/test_text.cpp:L72-L98`
#[derive(Debug, Clone)]
pub struct UiTextString {
    pub raw_text: String,
    pub parsed: ParsedRichText,
    pub lines: Vec<TextLine>,
    pub max_width: f32,
    pub min_height: f32,
    pub line_height: f32,
    pub selection: Option<TextSelection>,
}

impl UiTextString {
    pub fn new(text: &str, available_width: f32, font_size: f32) -> Self {
        let options = item_text_default_options();
        Self::with_options(text, available_width, font_size, &options)
    }

    pub fn with_options(
        text: &str,
        available_width: f32,
        font_size: f32,
        options: &TextParseOptions,
    ) -> Self {
        let parsed = parse_text_entities(text, options);
        let line_height = font_size * 1.35;
        let avg_char_width = font_size * 0.55;
        let space_width = font_size * 0.35;

        let words = tokenize_words(&parsed.text, avg_char_width);
        let lines = wrap_words_into_lines(&words, available_width, line_height, space_width);
        let max_w = lines.iter().map(|l| l.width).fold(0.0_f32, f32::max);
        let min_h = line_height;

        Self {
            raw_text: text.to_string(),
            parsed,
            lines,
            max_width: max_w,
            min_height: min_h,
            line_height,
            selection: None,
        }
    }

    pub fn count_height(&self, _available_width: f32) -> f32 {
        (self.lines.len() as f32 * self.line_height).max(self.min_height)
    }

    pub fn set_selection(&mut self, selection: Option<TextSelection>) {
        self.selection = selection;
    }

    pub fn entity_at_offset(&self, offset: usize) -> Option<&TextEntitySpan> {
        self.parsed.entities.iter().find(|e| {
            offset >= e.offset && offset < e.end_offset()
        })
    }
}

/// Authentic GPUI Render View for rich laid-out message text.
#[derive(Debug, Clone)]
pub struct UiTextLayoutView {
    pub text_model: UiTextString,
    pub default_color: gpui::Rgba,
    pub link_color: gpui::Rgba,
}

impl UiTextLayoutView {
    pub fn new(text_model: UiTextString) -> Self {
        Self {
            text_model,
            default_color: rgb(0x111111),
            link_color: rgb(0x2AABEE),
        }
    }
}

impl Render for UiTextLayoutView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text_model = &self.text_model;
        let default_color = self.default_color;
        let link_color = self.link_color;

        div()
            .flex()
            .flex_col()
            .children(text_model.lines.iter().map(|line| {
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .h(px(line.height))
                    .items_center()
                    .children(line.words.iter().map(|w| {
                        let entity = text_model.entity_at_offset(w.offset);
                        let is_link = entity.map(|e| e.entity_type == TextEntityType::Url).unwrap_or(false);
                        let is_bold = entity.map(|e| e.entity_type == TextEntityType::Bold).unwrap_or(false);
                        let is_code = entity.map(|e| e.entity_type == TextEntityType::Code).unwrap_or(false);
                        let is_spoiler = entity.map(|e| e.entity_type == TextEntityType::Spoiler).unwrap_or(false);

                        let color = if is_link { link_color } else { default_color };
                        let mut span_el = div().text_color(color);
                        if is_bold {
                            span_el = span_el.font_weight(FontWeight::BOLD);
                        }
                        if is_code {
                            span_el = span_el.font_family(".AppleSystemUIFontMonospaced").bg(rgb(0xF0F0F0)).rounded(px(2.0)).px(px(2.0));
                        }
                        if is_spoiler {
                            span_el = span_el.bg(rgb(0xCCCCCC)).text_color(rgb(0xCCCCCC)).rounded(px(2.0));
                        }
                        span_el = span_el.child(w.text.clone());

                        if w.has_trailing_space {
                            span_el = span_el.mr(px(4.0));
                        }
                        span_el
                    }))
            }))
    }
}
