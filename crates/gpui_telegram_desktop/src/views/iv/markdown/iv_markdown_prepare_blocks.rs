//! Target Symbol: IvMarkdownPrepareBlocksMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_blocks.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreparedBlockType {
    #[default]
    Paragraph,
    Heading,
    List,
    Table,
    CodeBlock,
    Blockquote,
    Divider,
    Details,
}

#[derive(Debug, Clone, Default)]
pub struct PreparedBlockData {
    pub block_type: PreparedBlockType,
    pub content: String,
    pub level: usize,
    pub is_tight: bool,
}

pub fn prepare_table_cell_text(raw_text: &str) -> String {
    raw_text.trim().replace('\n', " ")
}

pub fn prepare_fallback_blocks(raw_lines: &[String]) -> Vec<PreparedBlockData> {
    raw_lines
        .iter()
        .map(|line| PreparedBlockData {
            block_type: PreparedBlockType::Paragraph,
            content: line.clone(),
            level: 0,
            is_tight: false,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareBlocksMarkdown {
    pub blocks: Vec<PreparedBlockData>,
    pub total_blocks_built: usize,
}

impl Default for IvMarkdownPrepareBlocksMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareBlocksMarkdown {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            total_blocks_built: 0,
        }
    }

    pub fn append_prepared(&mut self, block: PreparedBlockData) {
        self.total_blocks_built += 1;
        self.blocks.push(block);
    }

    pub fn collect_footnote_definitions(&self) -> Vec<String> {
        self.blocks
            .iter()
            .filter(|b| b.content.starts_with("[^") && b.content.contains("]:"))
            .map(|b| b.content.clone())
            .collect()
    }
}

impl Render for IvMarkdownPrepareBlocksMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.blocks.len();
        let total = self.total_blocks_built;

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
                    .child("Markdown Prepared Blocks Compiler"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Active Blocks: {}", count))
                    .child(format!("Total Built: {}", total)),
            )
    }
}
