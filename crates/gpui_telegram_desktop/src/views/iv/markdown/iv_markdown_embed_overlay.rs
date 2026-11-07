//! Target Symbol: IvMarkdownEmbedOverlayMarkdown
//! Module 13: Instant View Embedded Iframe & Webview Overlay Coordinator
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const READY_REVEAL_DELAY_MS: u64 = 150;
pub const HTML_REQUEST_PREFIX: &str = "tg://iv-embed";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmbedLayoutMode {
    #[default]
    Inline,
    FullWidth,
    Responsive,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum EmbedState {
    #[default]
    Loading,
    Ready,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct IvMarkdownEmbedOverlayMarkdown {
    pub source_url: String,
    pub html_snippet: Option<String>,
    pub layout_mode: EmbedLayoutMode,
    pub state: EmbedState,
    pub width: f32,
    pub height: f32,
}

impl Default for IvMarkdownEmbedOverlayMarkdown {
    fn default() -> Self {
        Self {
            source_url: String::new(),
            html_snippet: None,
            layout_mode: EmbedLayoutMode::Inline,
            state: EmbedState::Loading,
            width: 480.0,
            height: 270.0,
        }
    }
}

impl IvMarkdownEmbedOverlayMarkdown {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            source_url: url.into(),
            ..Default::default()
        }
    }

    pub fn with_dimensions(mut self, width: f32, height: f32) -> Self {
        self.width = width.max(100.0);
        self.height = height.max(50.0);
        self
    }

    pub fn set_html(&mut self, html: impl Into<String>) {
        self.html_snippet = Some(html.into());
    }

    pub fn mark_ready(&mut self) {
        self.state = EmbedState::Ready;
    }

    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.state = EmbedState::Failed(error.into());
    }

    pub fn is_ready(&self) -> bool {
        self.state == EmbedState::Ready
    }
}

impl Render for IvMarkdownEmbedOverlayMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let height = self.height;
        let status_text = match &self.state {
            EmbedState::Loading => "Loading embed...".to_string(),
            EmbedState::Ready => "Interactive iframe ready".to_string(),
            EmbedState::Failed(err) => err.clone(),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(height))
            .bg(rgb(0x1e2329))
            .rounded_md()
            .text_xs()
            .text_color(rgb(0x8a9aa8))
            .child(status_text)
    }
}
