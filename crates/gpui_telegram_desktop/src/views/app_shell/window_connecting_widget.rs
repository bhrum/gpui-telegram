//! WindowConnectingWidget: Top status bar showing connection progress, updates, or proxy state
//! Upstream: Telegram/SourceFiles/window/window_connecting_widget.h, window_connecting_widget.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Updating,
    WaitingForNetwork,
    ProxyConnecting,
}

#[derive(Debug, Clone)]
pub struct WindowConnectingWidget {
    pub status: ConnectionStatus,
    pub is_visible: bool,
    pub proxy_host: Option<String>,
}

impl WindowConnectingWidget {
    pub fn new() -> Self {
        Self {
            status: ConnectionStatus::Connected,
            is_visible: false,
            proxy_host: None,
        }
    }

    pub fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
        self.is_visible = status != ConnectionStatus::Connected;
    }

    pub fn set_proxy_host(&mut self, host: Option<String>) {
        self.proxy_host = host;
    }

    pub fn status_text(&self) -> &'static str {
        match self.status {
            ConnectionStatus::Connected => "Connected",
            ConnectionStatus::Connecting => "Connecting...",
            ConnectionStatus::Updating => "Updating...",
            ConnectionStatus::WaitingForNetwork => "Waiting for network...",
            ConnectionStatus::ProxyConnecting => "Connecting through proxy...",
        }
    }
}

impl Default for WindowConnectingWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowConnectingWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .h(if self.is_visible { px(28.0) } else { px(0.0) })
            .w_full()
            .bg(rgb(0x2B5278))
            .text_color(rgb(0xFFFFFF))
            .text_xs()
            .child(self.status_text())
    }
}
