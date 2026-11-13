//! Target Symbol: IvMarkdownButtonRowMarkdown
//! Module 13: Instant View Action Button Row Layout & Shimmer Glare
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MIN_PRIMARY_LABEL_CONTRAST: f32 = 4.5;
pub const LOADING_GLARE_OPACITY: f32 = 0.35;
pub const LOADING_GLARE_DURATION_MS: f32 = 1400.0;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionButtonKind {
    Url(String),
    Callback(Vec<u8>),
    SwitchInline(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionButtonData {
    pub button_id: u64,
    pub label: String,
    pub kind: ActionButtonKind,
    pub is_loading: bool,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownButtonRowMarkdown {
    pub row_index: usize,
    pub buttons: Vec<ActionButtonData>,
    pub glare_elapsed_ms: f32,
}

impl Default for IvMarkdownButtonRowMarkdown {
    fn default() -> Self {
        Self {
            row_index: 0,
            buttons: Vec::new(),
            glare_elapsed_ms: 0.0,
        }
    }
}

impl IvMarkdownButtonRowMarkdown {
    pub fn new(row_index: usize) -> Self {
        Self {
            row_index,
            buttons: Vec::new(),
            glare_elapsed_ms: 0.0,
        }
    }

    pub fn add_url_button(&mut self, button_id: u64, label: impl Into<String>, url: impl Into<String>) {
        self.buttons.push(ActionButtonData {
            button_id,
            label: label.into(),
            kind: ActionButtonKind::Url(url.into()),
            is_loading: false,
        });
    }

    pub fn add_callback_button(&mut self, button_id: u64, label: impl Into<String>, data: Vec<u8>) {
        self.buttons.push(ActionButtonData {
            button_id,
            label: label.into(),
            kind: ActionButtonKind::Callback(data),
            is_loading: false,
        });
    }

    pub fn set_loading(&mut self, button_id: u64, is_loading: bool) {
        if let Some(btn) = self.buttons.iter_mut().find(|b| b.button_id == button_id) {
            btn.is_loading = is_loading;
        }
    }

    pub fn advance_glare(&mut self, delta_ms: f32) {
        self.glare_elapsed_ms = (self.glare_elapsed_ms + delta_ms) % LOADING_GLARE_DURATION_MS;
    }

    pub fn glare_phase(&self) -> f32 {
        self.glare_elapsed_ms / LOADING_GLARE_DURATION_MS
    }
}

impl Render for IvMarkdownButtonRowMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut row_el = div().flex().flex_row().gap_2().w_full().my_2();

        for btn in &self.buttons {
            let label_str = btn.label.clone();
            let loading = btn.is_loading;

            row_el = row_el.child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .bg(if loading { rgb(0x2a3644) } else { rgb(0x2b5278) })
                    .text_color(rgb(0xffffff))
                    .text_sm()
                    .cursor_pointer()
                    .child(label_str),
            );
        }

        row_el
    }
}
