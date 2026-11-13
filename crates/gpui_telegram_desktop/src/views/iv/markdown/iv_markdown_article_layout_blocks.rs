//! Target Symbol: IvMarkdownArticleLayoutBlocksMarkdown
//! Module 13: Instant View Article Layout Blocks Metrics & Table Cell Geometry
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const CODE_TAB_COLUMNS: usize = 4;
pub const CODE_TRAILING_GUARD: usize = 1;
pub const READABLE_TEXT_COLUMNS: usize = 80;

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellGeometry {
    pub col_index: usize,
    pub row_index: usize,
    pub col_span: usize,
    pub row_span: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRowGeometry {
    pub row_index: usize,
    pub y: f32,
    pub height: f32,
    pub cells: Vec<TableCellGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMetrics {
    pub block_id: u64,
    pub natural_width: f32,
    pub natural_height: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleLayoutBlocksMarkdown {
    pub content_width: f32,
    pub block_metrics: Vec<BlockMetrics>,
    pub table_rows: Vec<TableRowGeometry>,
    pub total_layout_height: f32,
}

impl Default for IvMarkdownArticleLayoutBlocksMarkdown {
    fn default() -> Self {
        Self {
            content_width: 680.0,
            block_metrics: Vec::new(),
            table_rows: Vec::new(),
            total_layout_height: 0.0,
        }
    }
}

impl IvMarkdownArticleLayoutBlocksMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content_width(width: f32) -> Self {
        Self {
            content_width: width.max(100.0),
            ..Default::default()
        }
    }

    pub fn add_block_metric(&mut self, block_id: u64, natural_width: f32, natural_height: f32, margin_top: f32, margin_bottom: f32) {
        let metric = BlockMetrics {
            block_id,
            natural_width,
            natural_height,
            margin_top,
            margin_bottom,
        };
        self.block_metrics.push(metric);
        self.recompute_total_height();
    }

    pub fn clear_metrics(&mut self) {
        self.block_metrics.clear();
        self.table_rows.clear();
        self.total_layout_height = 0.0;
    }

    pub fn recompute_total_height(&mut self) {
        let mut cur_y = 0.0;
        for m in &self.block_metrics {
            cur_y += m.margin_top + m.natural_height + m.margin_bottom;
        }
        self.total_layout_height = cur_y;
    }

    pub fn add_table_row(&mut self, row_index: usize, height: f32, cells: Vec<TableCellGeometry>) {
        let y = self.table_rows.last().map_or(0.0, |r| r.y + r.height);
        self.table_rows.push(TableRowGeometry {
            row_index,
            y,
            height,
            cells,
        });
    }
}

impl Render for IvMarkdownArticleLayoutBlocksMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.block_metrics.len();
        let height_str = format!("{:.1}px", self.total_layout_height);

        div()
            .flex()
            .flex_col()
            .p_2()
            .text_xs()
            .text_color(rgb(0x8a9aa8))
            .child(format!("Layout: {} blocks, total height {}", count, height_str))
    }
}
