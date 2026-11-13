//! Instant View LaTeX Math Formula Editor Dialog (`iv_editor_math_box.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Standard sample LaTeX formulas provided as quick presets in the Telegram UI.
pub const FORMULA_SAMPLES: &[&str] = &[
    "e^{i\\pi}=-1",
    "x^n+y^n=z^n",
    "\\sin^2\\alpha+\\cos^2\\alpha=1",
    "x_{1,2}=\\frac{-b\\pm\\sqrt{b^2-4ac}}{2a}",
];

/// Validates LaTeX mathematical syntax for balanced delimiters and braces.
pub fn validate_latex_math(source: &str) -> Result<(), &'static str> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err("Formula cannot be empty");
    }

    let mut brace_depth: i32 = 0;
    for ch in trimmed.chars() {
        match ch {
            '{' => brace_depth += 1,
            '}' => {
                brace_depth -= 1;
                if brace_depth < 0 {
                    return Err("Unmatched closing brace '}'");
                }
            }
            _ => {}
        }
    }

    if brace_depth != 0 {
        return Err("Unclosed opening brace '{'");
    }
    Ok(())
}

/// Committed formula payload from the editor box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathBoxResult {
    pub source: String,
    pub separate_line: bool,
}

/// Modal dialog controller for writing and previewing LaTeX formulas.
#[derive(Debug, Clone)]
pub struct IvEditorMathBoxEditor {
    pub source: String,
    pub separate_line: bool,
    pub editing_existing: bool,
    pub is_open: bool,
    pub title: String,
    pub error_message: Option<String>,
    pub committed_result: Option<MathBoxResult>,
}

impl Default for IvEditorMathBoxEditor {
    fn default() -> Self {
        Self {
            source: FORMULA_SAMPLES[0].to_string(),
            separate_line: false,
            editing_existing: false,
            is_open: true,
            title: "Edit Formula".to_string(),
            error_message: None,
            committed_result: None,
        }
    }
}

impl IvEditorMathBoxEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_source(mut self, source: impl Into<String>, separate_line: bool) -> Self {
        self.source = source.into();
        self.separate_line = separate_line;
        self
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = source.into();
        self.error_message = None;
    }

    pub fn set_separate_line(&mut self, separate: bool) {
        self.separate_line = separate;
    }

    pub fn apply_sample(&mut self, index: usize) {
        if let Some(&sample) = FORMULA_SAMPLES.get(index) {
            self.source = sample.to_string();
            self.error_message = None;
        }
    }

    pub fn commit(&mut self) -> bool {
        match validate_latex_math(&self.source) {
            Ok(()) => {
                self.committed_result = Some(MathBoxResult {
                    source: self.source.clone(),
                    separate_line: self.separate_line,
                });
                self.is_open = false;
                true
            }
            Err(err) => {
                self.error_message = Some(err.to_string());
                false
            }
        }
    }
}

impl Render for IvEditorMathBoxEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(16.0))
                    .child(self.title.clone())
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(format!("LaTeX Source: {}", self.source))
                    .child(format!("Separate line (display mode): {}", self.separate_line))
            )
            .children(self.error_message.as_ref().map(|err| {
                div()
                    .mt(px(4.0))
                    .text_color(rgb(0xE53935))
                    .text_size(px(12.0))
                    .child(err.clone())
            }))
            .child(
                div()
                    .mt(px(12.0))
                    .p(px(12.0))
                    .bg(rgb(0xF8F9FA))
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(rgb(0xEEEEEE))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child("Formula Preview (MicroTeX)")
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .font_family("serif")
                            .text_size(px(16.0))
                            .text_center()
                            .text_color(rgb(0x111111))
                            .child(format!("$$ {} $$", self.source))
                    )
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x666666))
                            .child("Presets: ")
                    )
                    .children(FORMULA_SAMPLES.iter().enumerate().map(|(idx, sample)| {
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x2AABEE))
                            .child(format!("[#{}]", idx + 1))
                    }))
            )
    }
}
