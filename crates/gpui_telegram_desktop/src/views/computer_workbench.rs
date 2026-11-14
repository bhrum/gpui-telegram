use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostDeviceSpec {
    pub name: String,
    pub ip_address: String,
    pub os_name: String,
    pub resolution: String,
    pub latency_ms: u32,
    pub fps: u32,
    pub is_tunnel_encrypted: bool,
}

pub struct ComputerWorkbench {
    pub device: HostDeviceSpec,
    pub is_mouse_captured: bool,
    pub is_keyboard_captured: bool,
    pub is_clipboard_synced: bool,
    pub is_quality_hd: bool,
    pub is_connected: bool,
    pub should_close: bool,
    pub frame_source: Option<String>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_action: Option<String>,
}

impl ComputerWorkbench {
    pub fn new(device_name: String) -> Self {
        Self {
            device: HostDeviceSpec {
                name: device_name,
                ip_address: "10.0.4.12".to_string(),
                os_name: "Ubuntu 24.04 LTS (x86_64)".to_string(),
                resolution: "1920x1080 @ 60Hz".to_string(),
                latency_ms: 14,
                fps: 60,
                is_tunnel_encrypted: true,
            },
            is_mouse_captured: true,
            is_keyboard_captured: true,
            is_clipboard_synced: true,
            is_quality_hd: true,
            is_connected: true,
            should_close: false,
            frame_source: None,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".to_string(),
            pending_action: None,
        }
    }

    pub fn new_live(device_name: String, palette: TdesktopThemePalette, language_id: &str) -> Self {
        Self {
            device: HostDeviceSpec {
                name: device_name,
                ip_address: String::new(),
                os_name: String::new(),
                resolution: String::new(),
                latency_ms: 0,
                fps: 0,
                is_tunnel_encrypted: true,
            },
            is_mouse_captured: true,
            is_keyboard_captured: true,
            is_clipboard_synced: true,
            is_quality_hd: true,
            is_connected: false,
            should_close: false,
            frame_source: None,
            palette,
            language_id: language_id.to_string(),
            pending_action: None,
        }
    }

    pub fn sync_lease(&mut self, device: HostDeviceSpec, frame_source: Option<String>, connected: bool) {
        self.device = device;
        self.frame_source = frame_source;
        self.is_connected = connected;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn take_action(&mut self) -> Option<String> { self.pending_action.take() }
    pub fn toggle_mouse_capture(&mut self) { self.is_mouse_captured = !self.is_mouse_captured; }
    pub fn toggle_keyboard_capture(&mut self) { self.is_keyboard_captured = !self.is_keyboard_captured; }
    pub fn toggle_clipboard_sync(&mut self) { self.is_clipboard_synced = !self.is_clipboard_synced; }
    pub fn toggle_quality(&mut self) { self.is_quality_hd = !self.is_quality_hd; }

    fn toggle_chip(label: &str, active: bool, pal: &TdesktopThemePalette) -> Div {
        div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0))
            .bg(if active { TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.18) } else { rgb(pal.input_bg).into() })
            .border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
            .text_xs().font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(if active { pal.accent } else { pal.text_secondary }))
            .child(label.to_string())
    }
}

impl Default for ComputerWorkbench {
    fn default() -> Self { Self::new("Dev-Workstation-01".to_string()) }
}

impl Render for ComputerWorkbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let dev = self.device.clone();
        let metrics = if dev.ip_address.is_empty() && dev.latency_ms == 0 {
            if self.is_connected { "Encrypted Stream Ready".to_string() } else { tr_str(&lang, "等待远程会话连接", "Waiting for remote desktop lease...").to_string() }
        } else {
            format!("{} · {} · {}ms · {}fps", dev.os_name, dev.ip_address, dev.latency_ms, dev.fps)
        };

        div().size_full().bg(rgb(pal.window_bg)).flex().flex_col()
            .child(
                div().h(px(48.0)).flex_shrink_0().w_full().bg(rgb(pal.top_bar_bg)).px(px(16.0)).flex().items_center().justify_between().border_b_1().border_color(rgb(pal.border_fg))
                    .child(div().flex().items_center().gap(px(10.0))
                        .child(tg_icon(IconName::ComputerDesktop, 18.0, pal.accent))
                        .child(div().flex().flex_col()
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(dev.name.clone()))
                            .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(metrics))))
                    .child(div().flex().items_center().gap(px(6.0))
                        .child(Self::toggle_chip("Mouse", self.is_mouse_captured, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_mouse_capture(); cx.notify(); })))
                        .child(Self::toggle_chip("Keys", self.is_keyboard_captured, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_keyboard_capture(); cx.notify(); })))
                        .child(Self::toggle_chip("Clip", self.is_clipboard_synced, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_clipboard_sync(); cx.notify(); })))
                        .child(Self::toggle_chip(if self.is_quality_hd { "HD 60" } else { "SD Fast" }, self.is_quality_hd, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_quality(); cx.notify(); })))
                        .child(div().cursor_pointer().px(px(10.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.danger)).text_xs().font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.pending_action = Some("disconnect".into()); this.should_close = true; cx.notify(); }))
                            .child(tr_str(&lang, "断开", "Disconnect")))),
            )
            .child(div().flex_1().w_full().flex().items_center().justify_center().p(px(16.0)).child(if let Some(ref src) = self.frame_source {
                div().size_full().rounded(px(10.0)).overflow_hidden().border_1().border_color(rgb(pal.border_fg)).child(img(src.clone()).size_full().object_fit(ObjectFit::Contain)).into_any_element()
            } else {
                div().size_full().rounded(px(12.0)).bg(rgb(pal.card_bg)).border_1().border_color(rgb(pal.border_fg)).flex().flex_col().items_center().justify_center().gap(px(10.0))
                    .child(tg_icon(IconName::ComputerDesktop, 44.0, pal.icon_fg))
                    .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child(if self.is_connected { format!("{} ({})", dev.name, dev.resolution) } else { tr_str(&lang, "当前未连接到远程桌面流", "No active remote desktop stream").to_string() }))
                    .when(dev.is_tunnel_encrypted, |el| el.child(div().flex().flex_row().items_center().gap(px(6.0)).child(tg_icon(IconName::Lock, 12.0, pal.success)).child(div().text_xs().text_color(rgb(pal.text_secondary)).child("E2EE Tunnel Ready"))))
                    .into_any_element()
            }))
    }
}
