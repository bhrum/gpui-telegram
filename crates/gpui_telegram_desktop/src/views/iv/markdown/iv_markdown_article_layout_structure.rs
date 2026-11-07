//! Target Symbol: IvMarkdownArticleLayoutStructureMarkdown
//! Module 13: Instant View Structural Width Constraints & Column Distribution
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const QUOTE_COLLAPSED_LINES: usize = 3;
pub const TABLE_MIN_COL_WIDTH: f32 = 48.0;
pub const MAX_BLOCK_DEPTH: usize = 16;

#[derive(Debug, Clone, PartialEq)]
pub struct WidthAnalysisNode {
    pub block_id: u64,
    pub min_width: f32,
    pub max_width: f32,
    pub nesting_depth: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDistribution {
    pub column_widths: Vec<f32>,
    pub total_width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteCollapseState {
    pub is_collapsed: bool,
    pub total_lines: usize,
    pub line_limit: usize,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticleLayoutStructureMarkdown {
    pub viewport_width: f32,
    pub analysis_nodes: Vec<WidthAnalysisNode>,
    pub max_depth: usize,
}

impl Default for IvMarkdownArticleLayoutStructureMarkdown {
    fn default() -> Self {
        Self {
            viewport_width: 680.0,
            analysis_nodes: Vec::new(),
            max_depth: MAX_BLOCK_DEPTH,
        }
    }
}

impl IvMarkdownArticleLayoutStructureMarkdown {
    pub fn new(viewport_width: f32) -> Self {
        Self {
            viewport_width: viewport_width.max(200.0),
            analysis_nodes: Vec::new(),
            max_depth: MAX_BLOCK_DEPTH,
        }
    }

    pub fn add_analysis_node(&mut self, block_id: u64, min_width: f32, max_width: f32, depth: usize) {
        let clamped_depth = depth.min(self.max_depth);
        self.analysis_nodes.push(WidthAnalysisNode {
            block_id,
            min_width: min_width.max(0.0),
            max_width: max_width.max(min_width),
            nesting_depth: clamped_depth,
        });
    }

    pub fn distribute_table_columns(&self, col_proportions: &[f32], available_width: f32) -> ColumnDistribution {
        let num_cols = col_proportions.len();
        if num_cols == 0 {
            return ColumnDistribution {
                column_widths: Vec::new(),
                total_width: 0.0,
            };
        }

        let total_weight: f32 = col_proportions.iter().copied().sum();
        let safe_weight = if total_weight <= 0.0 { 1.0 } else { total_weight };

        let mut widths = Vec::with_capacity(num_cols);
        let mut total = 0.0;
        for &weight in col_proportions {
            let col_w = (available_width * (weight / safe_weight)).max(TABLE_MIN_COL_WIDTH);
            widths.push(col_w);
            total += col_w;
        }

        ColumnDistribution {
            column_widths: widths,
            total_width: total,
        }
    }

    pub fn should_collapse_quote(&self, line_count: usize) -> bool {
        line_count > QUOTE_COLLAPSED_LINES
    }

    pub fn new_quote_collapse_state(&self, line_count: usize) -> QuoteCollapseState {
        let should_collapse = self.should_collapse_quote(line_count);
        QuoteCollapseState {
            is_collapsed: should_collapse,
            total_lines: line_count,
            line_limit: QUOTE_COLLAPSED_LINES,
        }
    }
}

impl Render for IvMarkdownArticleLayoutStructureMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let nodes_count = self.analysis_nodes.len();
        div()
            .flex()
            .p_2()
            .text_xs()
            .text_color(rgb(0x8a9aa8))
            .child(format!("Structure Layout: {} analyzed nodes (width {:.0}px)", nodes_count, self.viewport_width))
    }
}
