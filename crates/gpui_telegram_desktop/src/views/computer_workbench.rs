//! Remote Computer Workbench Overlay (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.6, CAP-04).
//!
//! Sub-components:
//! 1. Remote Display Stream Canvas (1080p WebRTC/VNC stream simulation).
//! 2. Control Toolbar (Mouse capture, Keyboard pass-through, Clipboard sync, Quality mode, Screenshot).
//! 3. Host Connection Metrics (IP, Latency, 60fps, E2EE Tunnel).

use gpui::*;
use serde::{Deserialize, Serialize};

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
        }
    }

    pub fn toggle_mouse_capture(&mut self) {
        self.is_mouse_captured = !self.is_mouse_captured;
    }

    pub fn toggle_keyboard_capture(&mut self) {
        self.is_keyboard_captured = !self.is_keyboard_captured;
    }

    pub fn toggle_clipboard_sync(&mut self) {
        self.is_clipboard_synced = !self.is_clipboard_synced;
    }

    pub fn toggle_quality(&mut self) {
        self.is_quality_hd = !self.is_quality_hd;
    }

    fn render_toggle_button(label: &str, is_active: bool) -> Div {
        div()
            .cursor_pointer()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(if is_active { rgba(0x2AABEE33) } else { rgba(0x00000033) })
            .border_1()
            .border_color(if is_active { rgb(0x2AABEE) } else { rgba(0x00000000) })
            .text_size(px(11.0))
            .font_weight(FontWeight::BOLD)
            .text_color(if is_active { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
            .child(label.to_string())
    }
}

impl Default for ComputerWorkbench {
    fn default() -> Self {
        Self::new("Dev-Workstation-01".to_string())
    }
}

impl Render for ComputerWorkbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dev = self.device.clone();
        let mouse_cap = self.is_mouse_captured;
        let kb_cap = self.is_keyboard_captured;
        let clip_sync = self.is_clipboard_synced;
        let quality_hd = self.is_quality_hd;

        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x14181E))
            .flex()
            .flex_col()
            // 1. Top Control Bar (height 48px)
            .child(
                div()
                    .h(px(48.0))
                    .w_full()
                    .bg(rgb(0x1B2129))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0x0E1216))
                    // Host Info & Connection Badge
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(div().text_size(px(16.0)).child("🖥️"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child(dev.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .child(format!("{} • {} • {}ms • {}fps", dev.os_name, dev.ip_address, dev.latency_ms, dev.fps)),
                                    ),
                            ),
                    )
                    // Control Action Toggles (Mouse, Keyboard, Clipboard, Quality, Disconnect)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            // Mouse Capture Toggle
                            .child(
                                Self::render_toggle_button("🖱️ Mouse", mouse_cap)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_mouse_capture();
                                        cx.notify();
                                    })),
                            )
                            // Keyboard Capture Toggle
                            .child(
                                Self::render_toggle_button("⌨️ Keys", kb_cap)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_keyboard_capture();
                                        cx.notify();
                                    })),
                            )
                            // Clipboard Sync Toggle
                            .child(
                                Self::render_toggle_button("📋 Clip", clip_sync)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_clipboard_sync();
                                        cx.notify();
                                    })),
                            )
                            // Quality HD Toggle
                            .child(
                                Self::render_toggle_button(if quality_hd { "HD 60" } else { "SD Fast" }, quality_hd)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_quality();
                                        cx.notify();
                                    })),
                            )
                            // Disconnect Button
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0xE53935))
                                    .hover(|s| s.bg(rgb(0xC62828)))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.should_close = true;
                                        cx.notify();
                                    }))
                                    .child("Disconnect"),
                            ),
                    ),
            )
            // 2. Remote Screen Canvas Viewport
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .bg(rgb(0x0A0D10))
                    .p(px(12.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    // Screen Mock Frame (Aspect 16:9)
                    .child(
                        div()
                            .relative()
                            .w(px(720.0))
                            .h(px(405.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1E242C))
                            .border_1()
                            .border_color(rgb(0x2A3542))
                            .overflow_hidden()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .p(px(16.0))
                            // OS Desktop Mock Content
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(div().text_size(px(12.0)).text_color(rgb(0x8A9AA8)).child("Ubuntu Desktop - Terminal Running"))
                                    .child(div().text_size(px(11.0)).text_color(rgb(0x4CAF50)).child("🔒 E2EE Direct Tunnel Active")),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(div().text_size(px(32.0)).child("🖥️"))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child(format!("Remote Desktop Feed ({})", dev.resolution)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child("Press Cmd+Esc to release mouse capture"),
                            ),
                    ),
            )
    }
}
