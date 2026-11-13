//! Authentic GPUI implementation of Telegram Desktop's Connection & Proxy Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/connection_box.h:L37-L108` and `connection_box.cpp:L70-L195`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Save settings delay from `connection_box.cpp:L70`.
pub const K_SAVE_SETTINGS_DELAYED_TIMEOUT_MS: u64 = 1000;

/// Proxy protocol type mirroring upstream `MTP::ProxyData::Type` (`connection_box.h:L39-L41`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProxyType {
    #[default]
    None,
    Socks5,
    Mtproto,
    Web,
}

/// Operational state of a proxy connection mirroring `ProxiesBoxController::ItemState` (`connection_box.h:L59-L67`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProxyItemState {
    #[default]
    NotTested,
    Checking,
    Online,
    Available,
    Unavailable,
}

/// Proxy connection descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyConfig {
    pub id: u32,
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub secret: Option<String>,
    pub ping_ms: Option<u32>,
    pub state: ProxyItemState,
    pub is_enabled: bool,
}

/// Encodes proxy configuration into standard Telegram `tg://` deep-link per `connection_box.cpp:L118-L154`.
pub fn proxy_to_tg_link(proxy: &ProxyConfig) -> String {
    match proxy.proxy_type {
        ProxyType::Socks5 => {
            let mut link = format!("tg://socks?server={}&port={}", proxy.host, proxy.port);
            if let Some(ref u) = proxy.user {
                link.push_str(&format!("&user={}", u));
            }
            if let Some(ref p) = proxy.password {
                link.push_str(&format!("&pass={}", p));
            }
            link
        }
        ProxyType::Mtproto | ProxyType::Web => {
            let mut link = format!("tg://proxy?server={}&port={}", proxy.host, proxy.port);
            if let Some(ref s) = proxy.secret {
                link.push_str(&format!("&secret={}", s));
            }
            link
        }
        ProxyType::None => String::new(),
    }
}

/// ConnectionBox view component.
/// Cites upstream `ProxiesBoxController` (`connection_box.cpp:L42-L108`).
#[derive(Clone)]
pub struct ConnectionBox {
    pub proxies: Vec<ProxyConfig>,
    pub selected_proxy_id: Option<u32>,
    pub use_proxy_for_calls: bool,
    pub try_ipv6: bool,
    pub auto_rotate_timeout_secs: u32,
    pub is_closed: bool,
}

impl ConnectionBox {
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            selected_proxy_id: None,
            use_proxy_for_calls: false,
            try_ipv6: true,
            auto_rotate_timeout_secs: 300,
            is_closed: false,
        }
    }

    pub fn add_proxy(&mut self, proxy: ProxyConfig) {
        self.proxies.push(proxy);
    }

    pub fn select_proxy(&mut self, id: u32) {
        self.selected_proxy_id = Some(id);
        for p in &mut self.proxies {
            p.is_enabled = p.id == id;
        }
    }

    pub fn remove_proxy(&mut self, id: u32) {
        self.proxies.retain(|p| p.id != id);
        if self.selected_proxy_id == Some(id) {
            self.selected_proxy_id = None;
        }
    }
}

impl Default for ConnectionBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ConnectionBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("connection_box_card")
            .w(px(420.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child("Proxy Settings"),
            );

        // Proxy list
        let mut list = div().flex().flex_col().gap(px(8.0));
        if self.proxies.is_empty() {
            list = list.child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x888888))
                    .child("No proxies configured. Using direct connection."),
            );
        } else {
            for proxy in &self.proxies {
                let ping_text = proxy
                    .ping_ms
                    .map(|ms| format!("{} ms", ms))
                    .unwrap_or_else(|| "checking...".to_string());

                let ping_color = match proxy.state {
                    ProxyItemState::Online | ProxyItemState::Available => rgb(0x4CAF50),
                    ProxyItemState::Checking => rgb(0xFFA000),
                    _ => rgb(0xE53935),
                };

                let row = div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(if proxy.is_enabled {
                        rgba(0x5288C114)
                    } else {
                        rgb(0xF9F9F9)
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x333333))
                                    .child(format!("{}:{}", proxy.host, proxy.port)),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x707579))
                                    .child(format!("{:?}", proxy.proxy_type)),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(ping_color)
                            .child(ping_text),
                    );

                list = list.child(row);
            }
        }
        card = card.child(list);

        // Footer buttons
        let btn_row = div()
            .flex()
            .justify_end()
            .gap(px(8.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x5288C1))
                    .child("Done"),
            );

        card.child(btn_row).into_any_element()
    }
}
