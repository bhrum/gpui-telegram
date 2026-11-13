//! Target Symbol: IvMarkdownParseConvertMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_parse_convert.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_CORE_TYPES: usize = 14;
pub const K_MASKED_TYPES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceRange {
    pub start_offset: usize,
    pub end_offset: usize,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone)]
pub struct CmarkBudget {
    pub remaining_bytes: usize,
    pub max_depth: usize,
    pub current_depth: usize,
}

impl Default for CmarkBudget {
    fn default() -> Self {
        Self {
            remaining_bytes: 1024 * 1024,
            max_depth: 32,
            current_depth: 0,
        }
    }
}

impl CmarkBudget {
    pub fn enter_scope(&mut self) -> Result<(), &'static str> {
        if self.current_depth >= self.max_depth {
            return Err("Max AST depth exceeded");
        }
        self.current_depth += 1;
        Ok(())
    }

    pub fn leave_scope(&mut self) {
        self.current_depth = self.current_depth.saturating_sub(1);
    }

    pub fn charge_bytes(&mut self, bytes: usize) -> Result<(), &'static str> {
        if bytes > self.remaining_bytes {
            return Err("AST memory budget exhausted");
        }
        self.remaining_bytes -= bytes;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedDetailsBlock {
    pub summary: String,
    pub content: Vec<String>,
    pub is_open: bool,
    pub range: SourceRange,
}

#[derive(Debug, Clone, Default)]
pub struct BudgetedParseResult {
    pub node_count: usize,
    pub max_observed_depth: usize,
    pub warnings: Vec<String>,
    pub details_blocks: Vec<ParsedDetailsBlock>,
}

#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub name: String,
    pub range: SourceRange,
}

#[derive(Debug, Clone, Default)]
pub struct ParserDeleter;

#[derive(Debug, Clone, Default)]
pub struct NodeDeleter;

#[derive(Debug, Clone, Default)]
pub struct ParserState {
    pub is_running: bool,
    pub has_errors: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ParserPointer {
    pub ptr_val: usize,
}

#[derive(Debug, Clone, Default)]
pub struct NodePointer {
    pub ptr_val: usize,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownParseConvertMarkdown {
    pub budget: CmarkBudget,
    pub last_result: Option<BudgetedParseResult>,
    pub total_conversions: usize,
}

impl Default for IvMarkdownParseConvertMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownParseConvertMarkdown {
    pub fn new() -> Self {
        Self {
            budget: CmarkBudget::default(),
            last_result: None,
            total_conversions: 0,
        }
    }

    pub fn run_budgeted_cmark_parse(&mut self, source: &str) -> Result<BudgetedParseResult, String> {
        self.total_conversions += 1;
        let mut budget = self.budget.clone();
        budget.charge_bytes(source.len()).map_err(|e| e.to_string())?;

        let mut res = BudgetedParseResult {
            node_count: 0,
            max_observed_depth: 0,
            warnings: Vec::new(),
            details_blocks: Vec::new(),
        };

        let mut in_details = false;
        let mut current_details = ParsedDetailsBlock::default();

        for (idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            res.node_count += 1;

            if trimmed.starts_with("<details>") {
                in_details = true;
                current_details = ParsedDetailsBlock {
                    summary: "Details".to_string(),
                    content: Vec::new(),
                    is_open: false,
                    range: SourceRange {
                        start_offset: 0,
                        end_offset: 0,
                        line: idx as u32 + 1,
                        column: 1,
                    },
                };
            } else if trimmed.starts_with("</details>") && in_details {
                in_details = false;
                res.details_blocks.push(std::mem::take(&mut current_details));
            } else if in_details {
                if trimmed.starts_with("<summary>") && trimmed.ends_with("</summary>") {
                    current_details.summary = trimmed
                        .trim_start_matches("<summary>")
                        .trim_end_matches("</summary>")
                        .to_string();
                } else {
                    current_details.content.push(line.to_string());
                }
            }
        }

        self.last_result = Some(res.clone());
        Ok(res)
    }
}

impl Render for IvMarkdownParseConvertMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (nodes, details) = self
            .last_result
            .as_ref()
            .map(|r| (r.node_count, r.details_blocks.len()))
            .unwrap_or((0, 0));

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
                    .child("Cmark Conversion Pipeline"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Total Runs: {}", self.total_conversions))
                    .child(format!("Nodes: {}", nodes))
                    .child(format!("Details Blocks: {}", details))
                    .child(format!("Budget Remaining: {} bytes", self.budget.remaining_bytes)),
            )
    }
}
