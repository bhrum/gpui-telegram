//! Upstream Source: Telegram/SourceFiles/settings/business/settings_quick_replies.cpp, Telegram/SourceFiles/settings/business/settings_quick_replies.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_SHORTCUT_LIMIT: usize = 32;
pub const K_QUICK_REPLIES_LIMIT: usize = 100;

#[derive(Debug, Clone)]
pub struct QuickReplyItem {
    pub id: u64,
    pub name: String,
    pub messages_count: usize,
}

pub fn is_valid_shortcut_name(name: &str) -> bool {
    let count = name.chars().count();
    if count == 0 || count > K_SHORTCUT_LIMIT {
        return false;
    }
    for ch in name.chars() {
        let is_letter_or_num = ch.is_alphanumeric();
        let is_allowed_punct = ch == '_' || ch == '\u{200c}' || ch == '\u{00b7}';
        let is_sinhala = ('\u{0d80}'..='\u{0dff}').contains(&ch);
        if !is_letter_or_num && !is_allowed_punct && !is_sinhala {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
pub struct SettingsQuickRepliesBusiness {
    pub shortcuts: Vec<QuickReplyItem>,
    pub is_editing: bool,
    pub new_name_input: String,
    pub error_text: Option<String>,
    pub status_message: Option<String>,
}

impl SettingsQuickRepliesBusiness {
    pub fn new() -> Self {
        Self {
            shortcuts: vec![
                QuickReplyItem {
                    id: 1,
                    name: "thanks".into(),
                    messages_count: 2,
                },
                QuickReplyItem {
                    id: 2,
                    name: "pricing".into(),
                    messages_count: 1,
                },
            ],
            is_editing: false,
            new_name_input: String::new(),
            error_text: None,
            status_message: None,
        }
    }

    pub fn can_add(&self) -> bool {
        self.shortcuts.len() < K_QUICK_REPLIES_LIMIT
    }

    pub fn set_input(&mut self, text: String) {
        self.new_name_input = text;
        self.error_text = None;
    }

    pub fn add_shortcut(&mut self) -> Result<u64, &'static str> {
        let name = self.new_name_input.trim().to_string();
        if !is_valid_shortcut_name(&name) {
            self.error_text = Some("Invalid shortcut name".into());
            return Err("INVALID_NAME");
        }
        if !self.can_add() {
            self.error_text = Some("Quick replies limit reached".into());
            return Err("LIMIT_REACHED");
        }
        let id = self.shortcuts.len() as u64 + 1;
        self.shortcuts.push(QuickReplyItem {
            id,
            name: name.to_string(),
            messages_count: 0,
        });
        self.new_name_input.clear();
        self.is_editing = false;
        self.status_message = Some(format!("Shortcut /{} created", name));
        Ok(id)
    }

    pub fn remove_shortcut(&mut self, id: u64) {
        self.shortcuts.retain(|s| s.id != id);
        self.status_message = Some("Shortcut deleted".into());
    }

    pub fn toggle_modal(&mut self) {
        self.is_editing = !self.is_editing;
        self.new_name_input.clear();
        self.error_text = None;
    }
}

impl Default for SettingsQuickRepliesBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsQuickRepliesBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.shortcuts.len();
        let is_editing = self.is_editing;
        let err = self.error_text.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header Info
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Quick Replies"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Create shortcuts for frequently sent messages and answers.")),
            )
            // Add shortcut button
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x5288c1))
                    .hover(|s| s.bg(rgb(0x4277b0)))
                    .cursor_pointer()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xffffff))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                        this.toggle_modal();
                        cx.notify();
                    }))
                    .child(format!("Add Shortcut ({}/{})", count, K_QUICK_REPLIES_LIMIT)),
            )
            // Add Shortcut dialog wrap
            .when(is_editing, |container| {
                container.child(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x232e3c))
                        .gap(px(8.0))
                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("New Shortcut Name"))
                        .when_some(err, |c, e| c.child(div().text_xs().text_color(rgb(0xe05555)).child(e)))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.set_input("hello".into());
                                            let _ = this.add_shortcut();
                                            cx.notify();
                                        }))
                                        .child("+ /hello"),
                                )
                                .child(
                                    div()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.set_input("hours".into());
                                            let _ = this.add_shortcut();
                                            cx.notify();
                                        }))
                                        .child("+ /hours"),
                                ),
                        ),
                )
            })
            // Shortcuts list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.shortcuts.iter().map(|item| {
                        let id = item.id;
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
                                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("/{}", item.name)))
                                    .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("{} messages", item.messages_count))),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(rgb(0xe05555))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                        this.remove_shortcut(id);
                                        cx.notify();
                                    }))
                                    .child("Delete"),
                            )
                    })),
            )
            .into_any_element()
    }
}
