//! Authentic GPUI theme stylesheet for ConnectionBox & Proxy configuration.
//! Upstream reference: `Telegram/SourceFiles/boxes/connection_box.style:L1-L89`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct ConnectionBoxStyleSheet {
    pub host_input_width: Pixels,
    pub port_input_width: Pixels,
    pub proxy_row_height: Pixels,
    pub ping_indicator_size: Pixels,
    pub box_padding: (Pixels, Pixels, Pixels, Pixels),
    pub host_port_spacing: Pixels,
    pub online_ping_fg: Rgba,
    pub offline_ping_fg: Rgba,
    pub checking_ping_fg: Rgba,
    pub text_fg: Rgba,
}

impl Default for ConnectionBoxStyleSheet {
    fn default() -> Self {
        Self {
            host_input_width: px(160.0),
            port_input_width: px(55.0),
            proxy_row_height: px(40.0),
            ping_indicator_size: px(8.0),
            box_padding: (px(24.0), px(16.0), px(24.0), px(16.0)),
            host_port_spacing: px(12.0),
            online_ping_fg: rgb(0x4FAE4E),
            offline_ping_fg: rgb(0xDF3F40),
            checking_ping_fg: rgb(0xF2994A),
            text_fg: rgb(0x222222),
        }
    }
}

impl ConnectionBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_widths(&self) -> (Pixels, Pixels) {
        (self.host_input_width, self.port_input_width)
    }
}
