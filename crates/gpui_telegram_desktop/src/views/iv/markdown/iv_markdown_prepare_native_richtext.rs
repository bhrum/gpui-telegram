//! Target Symbol: IvMarkdownPrepareNativeRichtextMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_native_richtext.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_DEFAULT_MAP_WIDTH: i32 = 400;
pub const K_DEFAULT_MAP_HEIGHT: i32 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GeoPointLocation {
    pub lat: f64,
    pub lon: f64,
    pub access_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Default)]
pub struct PreparedIvRichText {
    pub text: String,
    pub alignment: Alignment,
    pub entities_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvRichTextContext {
    pub max_length: usize,
    pub default_alignment: Alignment,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareNativeRichtextMarkdown {
    pub rich_texts: Vec<PreparedIvRichText>,
    pub total_conversions: usize,
}

impl Default for IvMarkdownPrepareNativeRichtextMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareNativeRichtextMarkdown {
    pub fn new() -> Self {
        Self {
            rich_texts: Vec::new(),
            total_conversions: 0,
        }
    }

    pub fn convert_native_entities(&mut self, text: &str, alignment: Alignment) -> PreparedIvRichText {
        self.total_conversions += 1;
        let rt = PreparedIvRichText {
            text: text.to_string(),
            alignment,
            entities_count: 0,
        };
        self.rich_texts.push(rt.clone());
        rt
    }
}

impl Render for IvMarkdownPrepareNativeRichtextMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.rich_texts.len();
        let total = self.total_conversions;

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
                    .child("Native RichText Entity Converter"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Active Texts: {}", active))
                    .child(format!("Total Converted: {}", total)),
            )
    }
}
