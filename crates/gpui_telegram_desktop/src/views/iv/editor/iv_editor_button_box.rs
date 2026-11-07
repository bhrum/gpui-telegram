//! Instant View Button Block Editor Dialog (`iv_editor_button_box.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use gpui::prelude::FluentBuilder;

/// Instant View interactive button action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkupButtonType {
    #[default]
    Url,
    Callback,
    SwitchInline,
    Buy,
}

/// Instant View button color theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkupButtonColor {
    #[default]
    Normal,
    Primary,
    Dangerous,
}

/// Rich button entity configuration.
#[derive(Debug, Clone, Default)]
pub struct RichButtonEditData {
    pub button_text: String,
    pub url: String,
    pub payload: Vec<u8>,
    pub button_type: MarkupButtonType,
    pub color: MarkupButtonColor,
}

/// Arguments passed to the rich button editing modal.
#[derive(Debug, Clone, Default)]
pub struct RichButtonEditBoxArgs {
    pub data: RichButtonEditData,
    pub separate_line: bool,
    pub editing_existing: bool,
}

/// Committed result after button edit dialog closes.
#[derive(Debug, Clone, Default)]
pub struct RichButtonEditResult {
    pub data: RichButtonEditData,
    pub separate_line: bool,
}

/// Validates target URL format for article buttons.
pub fn validate_button_url(url: &str) -> Result<String, &'static str> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err("URL cannot be empty");
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("tg://") {
        Ok(trimmed.to_string())
    } else {
        Err("URL must start with https://, http:// or tg://")
    }
}

/// Interactive button modal dialog controller.
#[derive(Debug, Clone)]
pub struct IvEditorButtonBoxEditor {
    pub args: RichButtonEditBoxArgs,
    pub is_open: bool,
    pub title: String,
    pub error_message: Option<String>,
    pub committed_result: Option<RichButtonEditResult>,
}

impl Default for IvEditorButtonBoxEditor {
    fn default() -> Self {
        Self {
            args: RichButtonEditBoxArgs::default(),
            is_open: true,
            title: "Edit Button".to_string(),
            error_message: None,
            committed_result: None,
        }
    }
}

impl IvEditorButtonBoxEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_args(mut self, args: RichButtonEditBoxArgs) -> Self {
        self.args = args;
        self
    }

    pub fn set_label(&mut self, text: impl Into<String>) {
        self.args.data.button_text = text.into();
    }

    pub fn set_button_text(&mut self, text: impl Into<String>) {
        self.args.data.button_text = text.into();
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        let u = url.into();
        self.args.data.url = u;
        self.error_message = None;
    }

    pub fn set_button_type(&mut self, button_type: MarkupButtonType) {
        self.args.data.button_type = button_type;
    }

    pub fn set_button_color(&mut self, color: MarkupButtonColor) {
        self.args.data.color = color;
    }

    pub fn set_separate_line(&mut self, separate: bool) {
        self.args.separate_line = separate;
    }

    pub fn commit(&mut self) -> bool {
        if self.args.data.button_text.trim().is_empty() {
            self.error_message = Some("Button text cannot be empty".to_string());
            return false;
        }

        if self.args.data.button_type == MarkupButtonType::Url {
            match validate_button_url(&self.args.data.url) {
                Ok(valid_url) => {
                    self.args.data.url = valid_url;
                }
                Err(err) => {
                    self.error_message = Some(err.to_string());
                    return false;
                }
            }
        }

        self.committed_result = Some(RichButtonEditResult {
            data: self.args.data.clone(),
            separate_line: self.args.separate_line,
        });
        self.is_open = false;
        true
    }
}

impl Render for IvEditorButtonBoxEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let button_color = match self.args.data.color {
            MarkupButtonColor::Primary => rgb(0x2AABEE),
            MarkupButtonColor::Dangerous => rgb(0xE53935),
            MarkupButtonColor::Normal => rgb(0x555555),
        };

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
                    .child(format!("Text: {}", self.args.data.button_text))
                    .child(format!("URL: {}", self.args.data.url))
                    .child(format!("Type: {:?}", self.args.data.button_type))
                    .child(format!("Separate line: {}", self.args.separate_line))
            )
            .children(self.error_message.as_ref().map(|err| {
                div()
                    .mt(px(8.0))
                    .text_color(rgb(0xE53935))
                    .text_size(px(12.0))
                    .child(err.clone())
            }))
            .child(
                div()
                    .mt(px(12.0))
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(button_color)
                    .text_color(rgb(0xFFFFFF))
                    .text_center()
                    .child(if self.args.data.button_text.is_empty() {
                        "Button Preview".to_string()
                    } else {
                        self.args.data.button_text.clone()
                    })
            )
    }
}
