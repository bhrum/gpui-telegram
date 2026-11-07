//! Target Symbol: IvData
//! Module 13: Instant View WebPage Metadata Container
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub fn site_name_from_url(url: &str) -> String {
    let trimmed = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let domain = trimmed.split(['/', '?', '#']).next().unwrap_or(trimmed);
    if domain.is_empty() {
        return "Unknown Site".to_string();
    }
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() >= 2 {
        let name = parts[parts.len() - 2];
        let mut chars = name.chars();
        if let Some(first) = chars.next() {
            return format!("{}{}", first.to_uppercase(), chars.as_str());
        }
    }
    domain.to_string()
}

#[derive(Debug, Clone)]
pub struct IvData {
    pub page_id: u64,
    pub hash: i32,
    pub url: String,
    pub name: String,
    pub partial: bool,
    pub is_cached: bool,
}

impl Default for IvData {
    fn default() -> Self {
        Self {
            page_id: 0,
            hash: 0,
            url: String::new(),
            name: String::new(),
            partial: false,
            is_cached: false,
        }
    }
}

impl IvData {
    pub fn new(page_id: u64, hash: i32, url: impl Into<String>, partial: bool) -> Self {
        let u = url.into();
        let name = site_name_from_url(&u);
        Self {
            page_id,
            hash,
            url: u,
            name,
            partial,
            is_cached: true,
        }
    }

    pub fn id(&self) -> String {
        format!("{}_{}", self.page_id, self.hash)
    }

    pub fn page_id(&self) -> u64 {
        self.page_id
    }

    pub fn hash(&self) -> i32 {
        self.hash
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn partial(&self) -> bool {
        self.partial
    }
}

impl Render for IvData {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(10.0))
            .bg(rgb(0x222730))
            .rounded(px(6.0))
            .border_1()
            .border_color(rgb(0x313845))
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(if self.name.is_empty() {
                                "Instant View Source".to_string()
                            } else {
                                self.name.clone()
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(if self.partial { rgb(0xF5BF4F) } else { rgb(0x61C554) })
                            .child(if self.partial { "PARTIAL" } else { "FULL" }),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x8D95A5))
                    .child(if self.url.is_empty() {
                        "No URL bound".to_string()
                    } else {
                        self.url.clone()
                    }),
            )
    }
}
