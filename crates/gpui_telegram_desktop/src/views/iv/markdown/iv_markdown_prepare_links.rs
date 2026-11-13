//! Target Symbol: IvMarkdownPrepareLinksMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_links.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ExternalEntityLinkData {
    pub raw_url: String,
    pub clean_url: String,
    pub display_text: String,
    pub is_telegram: bool,
}

pub fn external_link_display_text(url: &str) -> String {
    let without_scheme = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("tg://");
    without_scheme.trim_end_matches('/').to_string()
}

pub fn external_entity_link_data(url: &str, custom_label: Option<&str>) -> ExternalEntityLinkData {
    let clean = url.trim().to_string();
    let is_tg = clean.starts_with("tg://") || clean.starts_with("https://t.me/");
    let display = custom_label
        .map(|s| s.to_string())
        .unwrap_or_else(|| external_link_display_text(&clean));

    ExternalEntityLinkData {
        raw_url: clean.clone(),
        clean_url: clean,
        display_text: display,
        is_telegram: is_tg,
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareLinksMarkdown {
    pub link_index: HashMap<String, ExternalEntityLinkData>,
    pub total_links_resolved: usize,
}

impl Default for IvMarkdownPrepareLinksMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareLinksMarkdown {
    pub fn new() -> Self {
        Self {
            link_index: HashMap::new(),
            total_links_resolved: 0,
        }
    }

    pub fn resolve_link(&mut self, url: &str, label: Option<&str>) -> ExternalEntityLinkData {
        self.total_links_resolved += 1;
        let data = external_entity_link_data(url, label);
        self.link_index.insert(data.raw_url.clone(), data.clone());
        data
    }
}

impl Render for IvMarkdownPrepareLinksMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let indexed = self.link_index.len();
        let total = self.total_links_resolved;

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
                    .child("Instant View Link Resolver"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Unique Indexed: {}", indexed))
                    .child(format!("Total Resolved: {}", total)),
            )
    }
}
