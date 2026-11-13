//! Upstream Source: Telegram/SourceFiles/settings/business/settings_chat_intro.cpp, Telegram/SourceFiles/settings/business/settings_chat_intro.h
//! Subsystem: settings | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_INTRO_TITLE_LIMIT: usize = 32;
pub const K_INTRO_DESCRIPTION_LIMIT: usize = 70;

#[derive(Debug, Clone)]
pub struct SettingsChatIntroBusiness {
    pub title_text: String,
    pub description_text: String,
    pub sticker_id: Option<u64>,
    pub is_sticker_panel_open: bool,
    pub status_message: Option<String>,
}

impl SettingsChatIntroBusiness {
    pub fn new() -> Self {
        Self {
            title_text: String::new(),
            description_text: String::new(),
            sticker_id: None,
            is_sticker_panel_open: false,
            status_message: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.title_text.is_empty() && self.description_text.is_empty() && self.sticker_id.is_none()
    }

    pub fn set_title(&mut self, text: String) {
        if text.chars().count() <= K_INTRO_TITLE_LIMIT {
            self.title_text = text;
        }
    }

    pub fn set_description(&mut self, text: String) {
        if text.chars().count() <= K_INTRO_DESCRIPTION_LIMIT {
            self.description_text = text;
        }
    }

    pub fn choose_sticker(&mut self, doc_id: u64) {
        self.sticker_id = Some(doc_id);
        self.is_sticker_panel_open = false;
    }

    pub fn clear_sticker(&mut self) {
        self.sticker_id = None;
    }

    pub fn toggle_sticker_panel(&mut self) {
        self.is_sticker_panel_open = !self.is_sticker_panel_open;
    }

    pub fn reset(&mut self) {
        self.title_text.clear();
        self.description_text.clear();
        self.sticker_id = None;
        self.status_message = Some("Intro reset".into());
    }

    pub fn save(&mut self) {
        self.status_message = Some("Intro saved".into());
    }
}

impl Default for SettingsChatIntroBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsChatIntroBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.is_empty();
        let title_len = self.title_text.chars().count();
        let desc_len = self.description_text.chars().count();
        let has_custom_sticker = self.sticker_id.is_some();
        let panel_open = self.is_sticker_panel_open;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Interactive Preview Wrap
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p(px(20.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(8.0))
                    .child(
                        div()
                            .size(px(64.0))
                            .rounded(px(32.0))
                            .bg(rgb(0x232e3c))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_2xl()
                            .child(if has_custom_sticker { "✨" } else { "👋" }),
                    )
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(if self.title_text.is_empty() { "Greeting Title".into() } else { self.title_text.clone() }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8292a1))
                            .child(if self.description_text.is_empty() { "Write a brief welcome message for your clients...".into() } else { self.description_text.clone() }),
                    ),
            )
            // Title Input block
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(4.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child("Title"))
                            .child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("{}/{}", title_len, K_INTRO_TITLE_LIMIT))),
                    )
                    .child(div().text_sm().text_color(rgb(0xffffff)).child(if self.title_text.is_empty() { "Enter title...".into() } else { self.title_text.clone() })),
            )
            // Description Input block
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(4.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child("Message"))
                            .child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("{}/{}", desc_len, K_INTRO_DESCRIPTION_LIMIT))),
                    )
                    .child(div().text_sm().text_color(rgb(0xffffff)).child(if self.description_text.is_empty() { "Enter message...".into() } else { self.description_text.clone() })),
            )
            // Sticker Selector Button
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                        this.toggle_sticker_panel();
                        cx.notify();
                    }))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Choose Sticker"))
                    .child(div().text_xs().text_color(rgb(0x5288c1)).child(if has_custom_sticker { "Custom Sticker Selected" } else { "Random Hello Sticker" })),
            )
            // Sticker panel tray
            .when(panel_open, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(8.0))
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x232e3c))
                        .child(
                            div().cursor_pointer().text_lg().child("👋").on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.choose_sticker(101);
                                cx.notify();
                            })),
                        )
                        .child(
                            div().cursor_pointer().text_lg().child("🌟").on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.choose_sticker(102);
                                cx.notify();
                            })),
                        )
                        .child(
                            div().cursor_pointer().text_lg().child("💼").on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.choose_sticker(103);
                                cx.notify();
                            })),
                        ),
                )
            })
            // Reset Button
            .when(!is_empty, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x281717))
                        .hover(|s| s.bg(rgb(0x3d1f1f)))
                        .cursor_pointer()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xe05555))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                            this.reset();
                            cx.notify();
                        }))
                        .child("Reset Chat Intro"),
                )
            })
            .into_any_element()
    }
}
