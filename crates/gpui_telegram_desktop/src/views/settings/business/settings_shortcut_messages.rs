//! Upstream Source: Telegram/SourceFiles/settings/business/settings_shortcut_messages.cpp, Telegram/SourceFiles/settings/business/settings_shortcut_messages.h
//! Subsystem: settings | Feature ID: TCG-N45 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_QUICK_REPLY_MESSAGES_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct ShortcutMessageEntry {
    pub id: u64,
    pub text: String,
    pub media_type: Option<String>,
    pub is_selected: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsShortcutMessagesBusiness {
    pub shortcut_id: u64,
    pub shortcut_name: String,
    pub messages: Vec<ShortcutMessageEntry>,
    pub input_text: String,
    pub is_multi_select: bool,
    pub status_message: Option<String>,
}

impl SettingsShortcutMessagesBusiness {
    pub fn new() -> Self {
        Self {
            shortcut_id: 1,
            shortcut_name: "quick_response".into(),
            messages: vec![
                ShortcutMessageEntry {
                    id: 1,
                    text: "Thank you for contacting us! Our team will get back to you shortly.".into(),
                    media_type: None,
                    is_selected: false,
                },
                ShortcutMessageEntry {
                    id: 2,
                    text: "Here is our product catalogue: https://example.com/catalog".into(),
                    media_type: Some("document".into()),
                    is_selected: false,
                },
            ],
            input_text: String::new(),
            is_multi_select: false,
            status_message: None,
        }
    }

    pub fn can_send_more(&self) -> bool {
        self.messages.len() < K_QUICK_REPLY_MESSAGES_LIMIT
    }

    pub fn set_input(&mut self, text: String) {
        self.input_text = text;
    }

    pub fn send_message(&mut self) -> Result<u64, &'static str> {
        let trimmed = self.input_text.trim();
        if trimmed.is_empty() {
            return Err("EMPTY_TEXT");
        }
        if !self.can_send_more() {
            return Err("LIMIT_REACHED");
        }
        let id = self.messages.len() as u64 + 1;
        self.messages.push(ShortcutMessageEntry {
            id,
            text: trimmed.to_string(),
            media_type: None,
            is_selected: false,
        });
        self.input_text.clear();
        self.status_message = Some("Message added to shortcut".into());
        Ok(id)
    }

    pub fn toggle_select(&mut self, id: u64) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == id) {
            msg.is_selected = !msg.is_selected;
        }
        self.is_multi_select = self.messages.iter().any(|m| m.is_selected);
    }

    pub fn delete_selected(&mut self) {
        self.messages.retain(|m| !m.is_selected);
        self.is_multi_select = false;
        self.status_message = Some("Selected messages deleted".into());
    }

    pub fn clear_selection(&mut self) {
        for msg in &mut self.messages {
            msg.is_selected = false;
        }
        self.is_multi_select = false;
    }
}

impl Default for SettingsShortcutMessagesBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsShortcutMessagesBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.messages.len();
        let name = self.shortcut_name.clone();
        let is_multi = self.is_multi_select;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("/{}", name)))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("{}/{} messages", count, K_QUICK_REPLY_MESSAGES_LIMIT))),
                    )
                    .when(is_multi, |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0xe05555))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.delete_selected();
                                            cx.notify();
                                        }))
                                        .child("Delete Selected"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0x8292a1))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.clear_selection();
                                            cx.notify();
                                        }))
                                        .child("Cancel"),
                                ),
                        )
                    }),
            )
            // Message List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.messages.iter().map(|msg| {
                        let id = msg.id;
                        let selected = msg.is_selected;

                        div()
                            .flex()
                            .flex_row()
                            .items_start()
                            .justify_between()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(if selected { rgb(0x232e3c) } else { rgb(0x17212b) })
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.toggle_select(id);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().text_color(rgb(0xffffff)).child(msg.text.clone()))
                                    .when_some(msg.media_type.clone(), |el, mt| {
                                        el.child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("📎 Attachment: {}", mt)))
                                    }),
                            )
                            .child(div().text_xs().text_color(if selected { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if selected { "✓" } else { "○" }))
                    })),
            )
            // Bottom Compose bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(if self.input_text.is_empty() { "Write a shortcut message...".into() } else { self.input_text.clone() }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x5288c1))
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.set_input("We are glad to assist you with your request.".into());
                                let _ = this.send_message();
                                cx.notify();
                            }))
                            .child("Send"),
                    ),
            )
            .into_any_element()
    }
}
