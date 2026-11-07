//! Target Symbol: IvMarkdownParseFinalizeMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_parse_finalize.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct HeadingAnchor {
    pub text: String,
    pub slug: String,
    pub ordinal: usize,
}

#[derive(Debug, Clone, Default)]
pub struct FootnoteEntry {
    pub tag: String,
    pub ordinal: usize,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct FinalizedDocumentSemantics {
    pub anchors: Vec<HeadingAnchor>,
    pub footnotes: Vec<FootnoteEntry>,
    pub local_fragments: HashSet<String>,
    pub display_math_blocks_normalized: usize,
}

pub fn normalize_display_math_blocks(raw_blocks: &mut [String]) -> usize {
    let mut normalized = 0;
    for block in raw_blocks.iter_mut() {
        if block.trim().starts_with("$$") && block.trim().ends_with("$$") {
            normalized += 1;
            *block = block.trim().to_string();
        }
    }
    normalized
}

pub fn assign_heading_anchors(headings: &[String]) -> Vec<HeadingAnchor> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut anchors = Vec::with_capacity(headings.len());

    for (idx, heading) in headings.iter().enumerate() {
        let base_slug: String = heading
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect();
        let trimmed_slug = base_slug.trim_matches('-').to_string();
        let slug = if trimmed_slug.is_empty() {
            format!("section-{}", idx + 1)
        } else {
            let count = counts.entry(trimmed_slug.clone()).or_insert(0);
            *count += 1;
            if *count == 1 {
                trimmed_slug
            } else {
                format!("{}-{}", trimmed_slug, *count - 1)
            }
        };

        anchors.push(HeadingAnchor {
            text: heading.clone(),
            slug,
            ordinal: idx + 1,
        });
    }
    anchors
}

pub fn finalize_document_semantics(
    headings: &[String],
    raw_footnotes: &[String],
) -> FinalizedDocumentSemantics {
    let anchors = assign_heading_anchors(headings);
    let mut local_fragments = HashSet::new();

    for a in &anchors {
        local_fragments.insert(format!("#{}", a.slug));
    }

    let mut footnotes = Vec::with_capacity(raw_footnotes.len());
    for (idx, fn_text) in raw_footnotes.iter().enumerate() {
        let ordinal = idx + 1;
        let tag = format!("fn-{}", ordinal);
        local_fragments.insert(format!("#{}", tag));
        footnotes.push(FootnoteEntry {
            tag,
            ordinal,
            content: fn_text.clone(),
        });
    }

    FinalizedDocumentSemantics {
        anchors,
        footnotes,
        local_fragments,
        display_math_blocks_normalized: 0,
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownParseFinalizeMarkdown {
    pub semantics: FinalizedDocumentSemantics,
    pub total_finalizations: usize,
}

impl Default for IvMarkdownParseFinalizeMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownParseFinalizeMarkdown {
    pub fn new() -> Self {
        Self {
            semantics: FinalizedDocumentSemantics::default(),
            total_finalizations: 0,
        }
    }

    pub fn finalize(&mut self, headings: &[String], footnotes: &[String]) {
        self.total_finalizations += 1;
        self.semantics = finalize_document_semantics(headings, footnotes);
    }

    pub fn validate_local_fragment(&self, fragment: &str) -> bool {
        self.semantics.local_fragments.contains(fragment)
    }
}

impl Render for IvMarkdownParseFinalizeMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let headings = self.semantics.anchors.len();
        let fns = self.semantics.footnotes.len();
        let frags = self.semantics.local_fragments.len();

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
                    .child("Markdown Finalize & Anchor Engine"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Total Runs: {}", self.total_finalizations))
                    .child(format!("Headings: {}", headings))
                    .child(format!("Footnotes: {}", fns))
                    .child(format!("Anchors: {}", frags)),
            )
    }
}
