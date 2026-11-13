//! Upstream Source: Telegram/SourceFiles/settings/business/settings_chat_links.cpp, Telegram/SourceFiles/settings/business/settings_chat_links.h
//! Subsystem: settings | Feature ID: TCG-N11 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_CHANGES_DEBOUNCE_TIMEOUT_MS: u64 = 1000;
pub const K_BUSINESS_CHAT_LINKS_LIMIT: usize = 100;

#[derive(Debug, Clone)]
pub struct ChatLinkItem {
    pub link: String,
    pub title: String,
    pub message: String,
    pub clicks: u32,
}

#[derive(Debug, Clone)]
pub struct SettingsChatLinksBusiness {
    pub links: Vec<ChatLinkItem>,
    pub is_creating: bool,
    pub is_premium: bool,
    pub active_menu_link: Option<String>,
    pub status_message: Option<String>,
}

impl SettingsChatLinksBusiness {
    pub fn new() -> Self {
        Self {
            links: vec![
                ChatLinkItem {
                    link: "https://t.me/m/abc123XYZ".into(),
                    title: "Orders Inquiry".into(),
                    message: "Hi, I have a question about my purchase.".into(),
                    clicks: 14,
                },
            ],
            is_creating: false,
            is_premium: true,
            active_menu_link: None,
            status_message: None,
        }
    }

    pub fn can_create(&self) -> bool {
        self.links.len() < K_BUSINESS_CHAT_LINKS_LIMIT
    }

    pub fn create_link(&mut self, title: String, message: String) -> Result<String, &'static str> {
        if !self.can_create() {
            return Err("LIMIT_REACHED");
        }
        let link_slug = format!("https://t.me/m/{}", self.links.len() + 101);
        let item = ChatLinkItem {
            link: link_slug.clone(),
            title,
            message,
            clicks: 0,
        };
        self.links.insert(0, item);
        self.is_creating = false;
        self.status_message = Some("Link created".into());
        Ok(link_slug)
    }

    pub fn delete_link(&mut self, link: &str) {
        self.links.retain(|l| l.link != link);
        self.active_menu_link = None;
        self.status_message = Some("Link deleted".into());
    }

    pub fn toggle_menu(&mut self, link: String) {
        if self.active_menu_link.as_deref() == Some(&link) {
            self.active_menu_link = None;
        } else {
            self.active_menu_link = Some(link);
        }
    }

    pub fn copy_link(&mut self, link: &str) {
        self.status_message = Some(format!("Copied: {}", link));
        self.active_menu_link = None;
    }
}

impl Default for SettingsChatLinksBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsChatLinksBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.links.len();
        let can_create = self.can_create();
        let active_menu = self.active_menu_link.clone();

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
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Chat Links"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Share direct links with prefilled messages for your clients.")),
            )
            // Create New Link Button
            .when(can_create, |el| {
                el.child(
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
                            let _ = this.create_link("Support Link".into(), "Hello, need help.".into());
                            cx.notify();
                        }))
                        .child(format!("Create New Link ({}/{})", count, K_BUSINESS_CHAT_LINKS_LIMIT)),
                )
            })
            // Links list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.links.iter().map(|item| {
                        let link_str = item.link.clone();
                        let link_for_menu = item.link.clone();
                        let link_for_delete = item.link.clone();
                        let is_menu_open = active_menu.as_deref() == Some(&item.link);

                        div()
                            .flex()
                            .flex_col()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .gap(px(6.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(item.title.clone()))
                                            .child(div().text_xs().text_color(rgb(0x5288c1)).child(item.link.clone())),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("{} clicks", item.clicks)))
                                            .child(
                                                div()
                                                    .cursor_pointer()
                                                    .text_sm()
                                                    .text_color(rgb(0x5288c1))
                                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                                        this.toggle_menu(link_for_menu.clone());
                                                        cx.notify();
                                                    }))
                                                    .child("⋮"),
                                            ),
                                    ),
                            )
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("\"{}\"", item.message)))
                            .when(is_menu_open, |entry| {
                                entry.child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(12.0))
                                        .p(px(8.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x232e3c))
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                                    this.copy_link(&link_str);
                                                    cx.notify();
                                                }))
                                                .child("Copy"),
                                        )
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(rgb(0xe05555))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                                    this.delete_link(&link_for_delete);
                                                    cx.notify();
                                                }))
                                                .child("Delete"),
                                        ),
                                )
                            })
                    })),
            )
            .into_any_element()
    }
}
