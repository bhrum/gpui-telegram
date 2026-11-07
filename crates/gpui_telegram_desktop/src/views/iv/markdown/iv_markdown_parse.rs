//! Target Symbol: IvMarkdownParseMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_parse.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct MarkdownParseLimits {
    pub max_source_bytes: usize,
    pub max_tree_depth: usize,
    pub max_ast_nodes: usize,
}

impl Default for MarkdownParseLimits {
    fn default() -> Self {
        Self {
            max_source_bytes: 512 * 1024,
            max_tree_depth: 32,
            max_ast_nodes: 4096,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValidatedMarkdownSource {
    pub source: String,
    pub byte_count: usize,
    pub has_bom: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownSourceValidationResult {
    pub is_valid: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct IvParsedDocument {
    pub source_bytes: usize,
    pub node_count: usize,
    pub heading_count: usize,
    pub paragraph_count: usize,
    pub formula_count: usize,
}

pub fn parse_markdown_for_iv(
    source: &[u8],
    limits: Option<MarkdownParseLimits>,
) -> Result<IvParsedDocument, String> {
    let limits = limits.unwrap_or_default();
    if source.len() > limits.max_source_bytes {
        return Err(format!(
            "Markdown source size {} exceeds limit of {} bytes",
            source.len(),
            limits.max_source_bytes
        ));
    }

    let text = match std::str::from_utf8(source) {
        Ok(t) => t,
        Err(e) => return Err(format!("Invalid UTF-8 source: {}", e)),
    };

    let mut doc = IvParsedDocument {
        source_bytes: source.len(),
        node_count: 0,
        heading_count: 0,
        paragraph_count: 0,
        formula_count: 0,
    };

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        doc.node_count += 1;
        if trimmed.starts_with('#') {
            doc.heading_count += 1;
        } else if trimmed.starts_with("$$") || trimmed.ends_with("$$") {
            doc.formula_count += 1;
        } else {
            doc.paragraph_count += 1;
        }

        if doc.node_count > limits.max_ast_nodes {
            return Err("AST node count limit exceeded during parse".to_string());
        }
    }

    Ok(doc)
}

#[derive(Debug, Clone)]
pub struct IvMarkdownParseMarkdown {
    pub last_parsed_doc: Option<IvParsedDocument>,
    pub parse_limits: MarkdownParseLimits,
    pub total_parses: usize,
}

impl Default for IvMarkdownParseMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownParseMarkdown {
    pub fn new() -> Self {
        Self {
            last_parsed_doc: None,
            parse_limits: MarkdownParseLimits::default(),
            total_parses: 0,
        }
    }

    pub fn parse(&mut self, source: &[u8]) -> Result<IvParsedDocument, String> {
        self.total_parses += 1;
        let doc = parse_markdown_for_iv(source, Some(self.parse_limits.clone()))?;
        self.last_parsed_doc = Some(doc.clone());
        Ok(doc)
    }
}

impl Render for IvMarkdownParseMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (bytes, nodes, headings, paras) = self
            .last_parsed_doc
            .as_ref()
            .map(|d| (d.source_bytes, d.node_count, d.heading_count, d.paragraph_count))
            .unwrap_or((0, 0, 0, 0));

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
                    .child("Instant View Markdown Parser"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Total Parses: {}", self.total_parses))
                    .child(format!("Source: {} bytes", bytes))
                    .child(format!("Nodes: {}", nodes))
                    .child(format!("Headings: {}", headings))
                    .child(format!("Paragraphs: {}", paras)),
            )
    }
}
