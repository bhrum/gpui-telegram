//! Target Symbol: IvMarkdownArticleSelectionMarkdown
//! Module 13: Instant View Cross-Block Selection & Clipboard Formatting
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const PHOTO_COPY_LABEL: &str = "[Photo]";
pub const TAG_IV_MATH: &str = "formula";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectableSegmentKind {
    #[default]
    Text,
    Table,
    Code,
    Math,
    Media,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectableSegment {
    pub block_id: u64,
    pub kind: SelectableSegmentKind,
    pub start_offset: usize,
    pub end_offset: usize,
    pub text_slice: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableSelectionCell {
    pub row: usize,
    pub col: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleSelectionMarkdown {
    pub is_selecting: bool,
    pub segments: Vec<SelectableSegment>,
    pub table_cells: Vec<TableSelectionCell>,
}

impl Default for IvMarkdownArticleSelectionMarkdown {
    fn default() -> Self {
        Self {
            is_selecting: false,
            segments: Vec::new(),
            table_cells: Vec::new(),
        }
    }
}

impl IvMarkdownArticleSelectionMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_selection(&mut self) {
        self.is_selecting = true;
        self.segments.clear();
        self.table_cells.clear();
    }

    pub fn end_selection(&mut self) {
        self.is_selecting = false;
    }

    pub fn add_segment(&mut self, block_id: u64, kind: SelectableSegmentKind, start: usize, end: usize, text: impl Into<String>) {
        self.segments.push(SelectableSegment {
            block_id,
            kind,
            start_offset: start,
            end_offset: end,
            text_slice: text.into(),
        });
    }

    pub fn add_table_cell(&mut self, row: usize, col: usize, content: impl Into<String>) {
        self.table_cells.push(TableSelectionCell {
            row,
            col,
            content: content.into(),
        });
    }

    pub fn format_clipboard_text(&self) -> String {
        if !self.table_cells.is_empty() {
            let mut out = String::new();
            let mut cur_row = usize::MAX;
            for cell in &self.table_cells {
                if cell.row != cur_row {
                    if cur_row != usize::MAX {
                        out.push('\n');
                    }
                    cur_row = cell.row;
                } else {
                    out.push('\t');
                }
                out.push_str(&cell.content);
            }
            return out;
        }

        let mut out = String::new();
        for (i, seg) in self.segments.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            match seg.kind {
                SelectableSegmentKind::Media => out.push_str(PHOTO_COPY_LABEL),
                SelectableSegmentKind::Math => {
                    out.push('$');
                    out.push_str(&seg.text_slice);
                    out.push('$');
                }
                _ => out.push_str(&seg.text_slice),
            }
        }
        out
    }

    pub fn clear(&mut self) {
        self.is_selecting = false;
        self.segments.clear();
        self.table_cells.clear();
    }
}

impl Render for IvMarkdownArticleSelectionMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.segments.len();
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Selection: {} segments active", count))
    }
}
