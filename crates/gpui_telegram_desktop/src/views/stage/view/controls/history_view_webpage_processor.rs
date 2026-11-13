//! Webpage preview link extraction and resolver processor.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebPageParsedData {
    pub url: String,
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub has_preview_photo: bool,
}

pub struct HistoryViewWebpageProcessorViewControls {
    pub parsed: Option<WebPageParsedData>,
    pub parsed_links: Vec<String>,
    pub is_disabled: bool,
}

impl HistoryViewWebpageProcessorViewControls {
    pub fn new() -> Self {
        Self {
            parsed: None,
            parsed_links: Vec::new(),
            is_disabled: false,
        }
    }

    /// Extract links and create preview metadata
    /// Upstream: `WebpageProcessor::checkPreview` (`history_view_webpage_processor.cpp:L200-240`)
    pub fn parse_input_text(&mut self, text: &str) {
        if self.is_disabled {
            return;
        }

        let links: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.starts_with("http://") || w.starts_with("https://") || w.starts_with("t.me/"))
            .map(|s| s.to_string())
            .collect();

        if let Some(first_link) = links.first().cloned() {
            self.parsed = Some(WebPageParsedData {
                url: first_link.clone(),
                title: format!("Preview of {}", first_link),
                description: "Link summary preview content".to_string(),
                site_name: "Webpage".to_string(),
                has_preview_photo: true,
            });
        } else {
            self.parsed = None;
        }
        self.parsed_links = links;
    }

    pub fn clear(&mut self) {
        self.parsed = None;
        self.parsed_links.clear();
    }
}

impl Default for HistoryViewWebpageProcessorViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewWebpageProcessorViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.parsed.is_none() {
            return div().into_any_element();
        }

        let preview = self.parsed.as_ref().unwrap();
        let title = preview.title.clone();
        let desc = preview.description.clone();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .px_3()
            .py_1p5()
            .bg(rgb(0x18222d))
            .border_l_2()
            .border_color(rgb(0x2481cc))
            .text_xs()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x2481cc))
                    .child(title),
            )
            .child(
                div()
                    .text_color(rgb(0x8e8e93))
                    .child(desc),
            )
            .into_any_element()
    }
}
