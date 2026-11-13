//! Target Symbol: IvMarkdownMathMarkdown
//! Module 13: Instant View LaTeX Math Syntax Scanner & Token Extractor
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathDelimiterKind {
    #[default]
    InlineSingleDollar,
    DisplayDoubleDollar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathToken {
    pub start_offset: usize,
    pub end_offset: usize,
    pub expression: String,
    pub delimiter: MathDelimiterKind,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownMathMarkdown {
    pub tokens: Vec<MathToken>,
    pub syntax_errors_count: usize,
}

impl Default for IvMarkdownMathMarkdown {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            syntax_errors_count: 0,
        }
    }
}

impl IvMarkdownMathMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan_source(&mut self, text: &str) {
        self.tokens.clear();
        self.syntax_errors_count = 0;

        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut idx = 0;

        while idx < len {
            if idx + 1 < len && bytes[idx] == b'$' && bytes[idx + 1] == b'$' {
                let start = idx;
                idx += 2;
                let expr_start = idx;
                let mut found_end = false;
                while idx + 1 < len {
                    if bytes[idx] == b'$' && bytes[idx + 1] == b'$' {
                        let expr = text[expr_start..idx].trim().to_string();
                        self.tokens.push(MathToken {
                            start_offset: start,
                            end_offset: idx + 2,
                            expression: expr,
                            delimiter: MathDelimiterKind::DisplayDoubleDollar,
                        });
                        idx += 2;
                        found_end = true;
                        break;
                    }
                    idx += 1;
                }
                if !found_end {
                    self.syntax_errors_count += 1;
                    break;
                }
            } else if bytes[idx] == b'$' {
                let start = idx;
                idx += 1;
                let expr_start = idx;
                let mut found_end = false;
                while idx < len {
                    if bytes[idx] == b'$' {
                        let expr = text[expr_start..idx].trim().to_string();
                        self.tokens.push(MathToken {
                            start_offset: start,
                            end_offset: idx + 1,
                            expression: expr,
                            delimiter: MathDelimiterKind::InlineSingleDollar,
                        });
                        idx += 1;
                        found_end = true;
                        break;
                    }
                    idx += 1;
                }
                if !found_end {
                    self.syntax_errors_count += 1;
                    break;
                }
            } else {
                idx += 1;
            }
        }
    }

    pub fn is_valid_expression(expr: &str) -> bool {
        let mut brace_count = 0i32;
        for c in expr.chars() {
            if c == '{' {
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
                if brace_count < 0 {
                    return false;
                }
            }
        }
        brace_count == 0
    }
}

impl Render for IvMarkdownMathMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.tokens.len();
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Math Scanner: {} formulas extracted", count))
    }
}
