//! Authentic GPUI Settings Websites Section Component
//! Source: Telegram/SourceFiles/settings/sections/settings_websites.cpp, Telegram/SourceFiles/settings/sections/settings_websites.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_SHORT_POLL_TIMEOUT_MS: u64 = 60_000;

#[derive(Debug, Clone)]
pub struct ConnectedWebsite {
    pub id: u64,
    pub domain: String,
    pub bot_name: String,
    pub browser: String,
    pub ip: String,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct SettingsWebsitesSections {
    pub websites: Vec<ConnectedWebsite>,
    pub poll_timeout_ms: u64,
    pub has_divider: bool,
    pub is_visible: bool,
}

impl SettingsWebsitesSections {
    pub fn new() -> Self {
        Self {
            websites: vec![
                ConnectedWebsite {
                    id: 501,
                    domain: "fragment.com".into(),
                    bot_name: "Fragment Auth Bot".into(),
                    browser: "Safari / macOS".into(),
                    ip: "192.168.1.50".into(),
                    location: "United States".into(),
                },
                ConnectedWebsite {
                    id: 502,
                    domain: "telegram.org".into(),
                    bot_name: "Telegram Login".into(),
                    browser: "Chrome / Windows".into(),
                    ip: "10.0.0.5".into(),
                    location: "United Kingdom".into(),
                },
            ],
            poll_timeout_ms: K_SHORT_POLL_TIMEOUT_MS,
            has_divider: true,
            is_visible: true,
        }
    }

    pub fn websites_id(&self) -> &'static str {
        "settings_websites"
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn margins(&self) -> (f32, f32, f32, f32) {
        (22.0, 10.0, 20.0, 5.0)
    }

    pub fn disconnect_all(&mut self) -> usize {
        let count = self.websites.len();
        self.websites.clear();
        count
    }

    pub fn disconnect(&mut self, id: u64) -> bool {
        let initial_len = self.websites.len();
        self.websites.retain(|w| w.id != id);
        self.websites.len() < initial_len
    }

    pub fn remove(&mut self, id: u64) {
        self.disconnect(id);
    }

    pub fn update(&mut self, site: ConnectedWebsite) {
        if let Some(existing) = self.websites.iter_mut().find(|w| w.id == site.id) {
            *existing = site;
        } else {
            self.websites.push(site);
        }
    }

    pub fn add_divider(&mut self) {
        self.has_divider = true;
    }

    pub fn lng_sessions_ip(&self, ip: &str) -> String {
        format!("IP: {}", ip)
    }

    pub fn lng_settings_disconnect(&mut self, id: u64) {
        self.disconnect(id);
    }
}

impl Default for SettingsWebsitesSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsWebsitesSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sites = self.websites.clone();
        let is_empty = sites.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(div().font_weight(FontWeight::BOLD).text_lg().child("Connected Websites"))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8a909d))
                                    .child("Websites you logged into with Telegram."),
                            ),
                    ),
            )
            .child(
                if is_empty {
                    div()
                        .p(px(16.0))
                        .rounded_md()
                        .bg(rgb(0x282c34))
                        .text_center()
                        .text_xs()
                        .text_color(rgb(0x8a909d))
                        .child("No connected websites")
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .children(sites.into_iter().map(|s| {
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .p(px(10.0))
                                .rounded_md()
                                .bg(rgb(0x282c34))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(div().font_weight(FontWeight::SEMIBOLD).child(s.domain))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x8a909d))
                                                .child(format!("{} | {}", s.browser, s.ip)),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xeb5757))
                                        .child("Disconnect"),
                                )
                        }))
                },
            )
    }
}
