//! Target Symbol: IvEditorWindowEditor
//! Module 13: Instant View Editor Host Window Container
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowState {
    #[default]
    Normal,
    Minimized,
    Maximized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalLayerKind {
    LinkEditor { url: String, text: String },
    MathFormulaEditor { formula: String, is_block: bool },
    MediaCaptionEditor { caption: String },
    ConfirmDiscardChanges,
}

#[derive(Debug, Clone)]
pub struct ModalLayer {
    pub title: String,
    pub kind: ModalLayerKind,
    pub is_dismissible: bool,
}

#[derive(Debug, Clone)]
pub struct IvEditorWindowEditor {
    pub title: String,
    pub state: WindowState,
    pub active_layer: Option<ModalLayer>,
    pub is_focused: bool,
    pub is_ime_active: bool,
    pub close_requested: bool,
    pub width: f32,
    pub height: f32,
}

impl Default for IvEditorWindowEditor {
    fn default() -> Self {
        Self {
            title: "Instant View Article Editor".to_string(),
            state: WindowState::Normal,
            active_layer: None,
            is_focused: true,
            is_ime_active: false,
            close_requested: false,
            width: 900.0,
            height: 700.0,
        }
    }
}

impl IvEditorWindowEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_modal(&mut self, title: impl Into<String>, kind: ModalLayerKind) {
        self.active_layer = Some(ModalLayer {
            title: title.into(),
            kind,
            is_dismissible: true,
        });
    }

    pub fn hide_modal(&mut self) {
        self.active_layer = None;
    }

    pub fn is_modal_shown(&self) -> bool {
        self.active_layer.is_some()
    }

    pub fn request_close(&mut self) {
        if self.is_modal_shown() {
            self.hide_modal();
        } else {
            self.close_requested = true;
        }
    }

    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
    }

    pub fn restore(&mut self) {
        self.state = WindowState::Normal;
    }

    pub fn on_ime_start(&mut self) {
        self.is_ime_active = true;
    }

    pub fn on_ime_end(&mut self) {
        self.is_ime_active = false;
    }
}

impl Render for IvEditorWindowEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut window_frame = div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x13171D))
            .rounded(px(10.0))
            .border_1()
            .border_color(rgb(0x2E3542))
            .shadow_xl();

        // Titlebar
        let titlebar = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(px(36.0))
            .px(px(14.0))
            .bg(rgb(0x1B2028))
            .border_b_1()
            .border_color(rgb(0x282F3B))
            // Traffic lights
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .items_center()
                    .child(div().w(px(12.0)).h(px(12.0)).rounded(px(6.0)).bg(rgb(0xED6A5E))) // Close
                    .child(div().w(px(12.0)).h(px(12.0)).rounded(px(6.0)).bg(rgb(0xF5BF4F))) // Minimize
                    .child(div().w(px(12.0)).h(px(12.0)).rounded(px(6.0)).bg(rgb(0x61C554))), // Zoom
            )
            // Title
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xCCD2DE))
                    .child(self.title.clone()),
            )
            // Trailing spacer
            .child(div().w(px(52.0)));

        window_frame = window_frame.child(titlebar);

        // Body content
        let body = div()
            .flex()
            .flex_1()
            .p(px(16.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(0x8A92A0))
                    .child("Editor workspace active. Document synced."),
            );

        window_frame = window_frame.child(body);

        // Modal overlay if active
        if let Some(layer) = &self.active_layer {
            let modal_box = div()
                .absolute()
                .inset_0()
                .bg(rgba(0x000000AA))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w(px(400.0))
                        .bg(rgb(0x222834))
                        .rounded(px(8.0))
                        .p(px(20.0))
                        .shadow_2xl()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child(layer.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(0x9EA8B6))
                                .child("Modal content configuration active."),
                        ),
                );

            window_frame = window_frame.child(modal_box);
        }

        window_frame
    }
}
