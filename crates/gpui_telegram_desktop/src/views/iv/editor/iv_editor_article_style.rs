//! Instant View Article Markdown Style & Line Metrics (`iv_editor_article_style.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Line layout description for calculating natural visual widths in an article.
#[derive(Debug, Clone, Default)]
pub struct TextBlockLayout {
    pub text: String,
    pub line_widths: Vec<f32>,
}

/// Simulated text document containing blocks for visual line width evaluation.
#[derive(Debug, Clone, Default)]
pub struct ArticleTextDocument {
    pub blocks: Vec<TextBlockLayout>,
    pub text_width: Option<f32>,
}

impl ArticleTextDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_block(mut self, text: impl Into<String>, line_widths: Vec<f32>) -> Self {
        self.blocks.push(TextBlockLayout {
            text: text.into(),
            line_widths,
        });
        self
    }

    pub fn set_text_width(&mut self, width: f32) {
        self.text_width = Some(width.max(1.0));
    }
}

/// Configuration for markdown typography and line metric enforcement.
#[derive(Debug, Clone)]
pub struct EditorMarkdownStyle {
    pub line_metrics_enabled: bool,
    pub body_font_size: Pixels,
    pub heading1_font_size: Pixels,
    pub heading2_font_size: Pixels,
    pub heading3_font_size: Pixels,
    pub code_font_size: Pixels,
    pub footer_font_size: Pixels,
    pub line_height_factor: f32,
}

impl Default for EditorMarkdownStyle {
    fn default() -> Self {
        Self {
            line_metrics_enabled: true,
            body_font_size: px(15.0),
            heading1_font_size: px(24.0),
            heading2_font_size: px(20.0),
            heading3_font_size: px(17.0),
            code_font_size: px(13.0),
            footer_font_size: px(12.0),
            line_height_factor: 1.4,
        }
    }
}

/// Primary editor style helper and metrics calculator.
#[derive(Debug, Clone)]
pub struct IvEditorArticleStyleEditor {
    pub style: EditorMarkdownStyle,
    pub title: String,
    pub is_active: bool,
    pub body_padding: (Pixels, Pixels),
    pub last_computed_width: f32,
}

impl Default for IvEditorArticleStyleEditor {
    fn default() -> Self {
        Self {
            style: EditorMarkdownStyle::default(),
            title: "Instant View Article Style".to_string(),
            is_active: false,
            body_padding: (px(0.0), px(8.0)),
            last_computed_width: 0.0,
        }
    }
}

impl IvEditorArticleStyleEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn create_editor_markdown_style(&mut self) -> EditorMarkdownStyle {
        self.style = EditorMarkdownStyle::default();
        self.style.clone()
    }

    pub fn enable_q_text_edit_line_metrics(&mut self) -> bool {
        self.style.line_metrics_enabled = true;
        true
    }

    /// Computes max visual line width across all document blocks.
    /// Equivalent to C++ MaxVisualLineWidth(document).
    pub fn max_visual_line_width(&mut self, document: &ArticleTextDocument) -> f32 {
        let mut max_width: f32 = 0.0;
        for block in &document.blocks {
            for &w in &block.line_widths {
                if w > max_width {
                    max_width = w;
                }
            }
        }
        self.last_computed_width = max_width.ceil().max(0.0);
        self.last_computed_width
    }

    /// Clones document with width constraint and calculates line width.
    /// Equivalent to C++ MaxVisualLineWidthForWidth(document, width).
    pub fn max_visual_line_width_for_width(
        &mut self,
        document: &ArticleTextDocument,
        width: f32,
    ) -> f32 {
        let clamped_width = width.max(1.0);
        let mut clone = document.clone();
        clone.set_text_width(clamped_width);
        // Constrain widths by wrapping
        for block in &mut clone.blocks {
            for w in &mut block.line_widths {
                if *w > clamped_width {
                    *w = clamped_width;
                }
            }
        }
        self.max_visual_line_width(&clone)
    }
}

impl Render for IvEditorArticleStyleEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg = if self.is_active {
            rgb(0x2AABEE)
        } else {
            rgb(0xFFFFFF)
        };
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(bg)
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child(self.title.clone())
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Line metrics: {}, body size: {:?}, last width: {:.1}px",
                        if self.style.line_metrics_enabled { "enabled" } else { "disabled" },
                        self.style.body_font_size,
                        self.last_computed_width
                    ))
            )
    }
}
