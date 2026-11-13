//! Target Symbol: IvMarkdownPrepareMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_PREPARED_BLOCKS: usize = 1000;
pub const MAX_INLINE_TEXT_OBJECTS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreparedLinkKind {
    #[default]
    Anchor,
    External,
    TgInternal,
}

#[derive(Debug, Clone, Default)]
pub struct PreparedLink {
    pub url: String,
    pub kind: PreparedLinkKind,
    pub is_secure: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InlineTextObjectKind {
    #[default]
    Formula,
    Image,
    Button,
    CustomEmoji,
}

#[derive(Debug, Clone, Default)]
pub struct InlineTextObjectFormulaData {
    pub tex_source: String,
    pub width: i32,
    pub height: i32,
    pub ascent: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InlineTextObjectIvImageData {
    pub image_url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InlineTextObjectButtonData {
    pub button_text: String,
    pub target_url: String,
}

#[derive(Debug, Clone, Default)]
pub struct InlineTextObjectEntity {
    pub kind: InlineTextObjectKind,
    pub offset: usize,
    pub length: usize,
    pub formula_data: Option<InlineTextObjectFormulaData>,
    pub image_data: Option<InlineTextObjectIvImageData>,
    pub button_data: Option<InlineTextObjectButtonData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreparedTableCellVerticalAlignment {
    #[default]
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Default)]
pub struct RichMessageLimits {
    pub max_blocks: usize,
    pub max_table_cells: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Markdown;

#[derive(Debug, Clone, Default)]
pub struct RichPage;

pub fn heading_level_label(level: usize) -> &'static str {
    match level {
        1 => "Heading 1",
        2 => "Heading 2",
        3 => "Heading 3",
        4 => "Heading 4",
        5 => "Heading 5",
        6 => "Heading 6",
        _ => "Heading",
    }
}

pub fn apply_soft_prepared_block_limit(blocks: &mut Vec<String>, limit: usize) -> bool {
    if blocks.len() > limit {
        blocks.truncate(limit);
        blocks.push("... [Article truncated due to length limits] ...".to_string());
        true
    } else {
        false
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareMarkdown {
    pub prepared_blocks_count: usize,
    pub inline_objects_count: usize,
    pub is_truncated: bool,
}

impl Default for IvMarkdownPrepareMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareMarkdown {
    pub fn new() -> Self {
        Self {
            prepared_blocks_count: 0,
            inline_objects_count: 0,
            is_truncated: false,
        }
    }

    pub fn prepare_synchronously(&mut self, blocks: &mut Vec<String>) {
        self.is_truncated = apply_soft_prepared_block_limit(blocks, MAX_PREPARED_BLOCKS);
        self.prepared_blocks_count = blocks.len();
    }
}

impl Render for IvMarkdownPrepareMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.prepared_blocks_count;
        let objects = self.inline_objects_count;
        let truncated = self.is_truncated;

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
                    .child("Instant View Document Preparation Engine"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Prepared Blocks: {}", count))
                    .child(format!("Inline Objects: {}", objects))
                    .child(format!("Truncated: {}", truncated)),
            )
    }
}
