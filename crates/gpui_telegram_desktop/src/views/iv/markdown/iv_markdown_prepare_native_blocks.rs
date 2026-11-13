//! Target Symbol: IvMarkdownPrepareNativeBlocksMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_native_blocks.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub struct RomanPart {
    pub value: u32,
    pub symbol: &'static str,
}

pub const K_ROMAN_PARTS: [RomanPart; 13] = [
    RomanPart { value: 1000, symbol: "M" },
    RomanPart { value: 900, symbol: "CM" },
    RomanPart { value: 500, symbol: "D" },
    RomanPart { value: 400, symbol: "CD" },
    RomanPart { value: 100, symbol: "C" },
    RomanPart { value: 90, symbol: "XC" },
    RomanPart { value: 50, symbol: "L" },
    RomanPart { value: 40, symbol: "XL" },
    RomanPart { value: 10, symbol: "X" },
    RomanPart { value: 9, symbol: "IX" },
    RomanPart { value: 5, symbol: "V" },
    RomanPart { value: 4, symbol: "IV" },
    RomanPart { value: 1, symbol: "I" },
];

pub fn to_roman(mut num: u32) -> String {
    let mut res = String::new();
    for part in &K_ROMAN_PARTS {
        while num >= part.value {
            res.push_str(part.symbol);
            num -= part.value;
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RichPageBlockKind {
    #[default]
    Paragraph,
    Heading,
    List,
    Table,
    Image,
    Video,
    Audio,
    Embed,
    Divider,
    Map,
    Collage,
}

#[derive(Debug, Clone, Default)]
pub struct RichPageTableCell {
    pub text: String,
    pub col_span: usize,
    pub row_span: usize,
}

#[derive(Debug, Clone, Default)]
pub struct RichPageTableRow {
    pub cells: Vec<RichPageTableCell>,
}

#[derive(Debug, Clone, Default)]
pub struct RichPageListItem {
    pub index: u32,
    pub marker: String,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct RichPageRelatedArticle {
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct RichPageBlock {
    pub kind: RichPageBlockKind,
    pub text: String,
    pub rows: Vec<RichPageTableRow>,
    pub items: Vec<RichPageListItem>,
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvDepthContext {
    pub depth: usize,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvOrderedMarkerFormatter {
    pub is_roman: bool,
}

impl NativeIvOrderedMarkerFormatter {
    pub fn format_marker(&self, index: u32) -> String {
        if self.is_roman {
            format!("{}.", to_roman(index))
        } else {
            format!("{}.", index)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvTableOccupancyRow {
    pub occupied: Vec<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvTableOccupancyGrid {
    pub rows: Vec<NativeIvTableOccupancyRow>,
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvPreparedLeafFormulaRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareNativeBlocksMarkdown {
    pub lowered_blocks: Vec<RichPageBlock>,
    pub total_lowered: usize,
}

impl Default for IvMarkdownPrepareNativeBlocksMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareNativeBlocksMarkdown {
    pub fn new() -> Self {
        Self {
            lowered_blocks: Vec::new(),
            total_lowered: 0,
        }
    }

    pub fn lower_block(&mut self, block: RichPageBlock) {
        self.total_lowered += 1;
        self.lowered_blocks.push(block);
    }
}

impl Render for IvMarkdownPrepareNativeBlocksMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.lowered_blocks.len();
        let total = self.total_lowered;

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
                    .child("Native Instant View Block Lowering Engine"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Active Blocks: {}", count))
                    .child(format!("Total Lowered: {}", total)),
            )
    }
}
