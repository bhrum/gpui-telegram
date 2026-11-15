use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::{ComputerNodeDto, ComputerNodeKind, ComputerRunPolicy};

use crate::components::ui::TextInputController;
use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkbenchTab {
    #[default]
    DesktopScreen,
    RemoteTerminal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalLogEntry {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub cwd: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkbenchCommand {
    RefreshScreen,
    SendGuiClick { x: i32, y: i32 },
    SendGuiText { text: String },
    RunShell { command: String, cwd: Option<String> },
    ListFiles { path: String },
    ReadRemoteFile { path: String },
}

/// Decodes an inline `data:image/<fmt>;base64,...` frame URL into a cached GPUI `Image`.
pub fn decode_data_url_image(data_url: &str) -> Option<Arc<gpui::Image>> {
    let rest = data_url.strip_prefix("data:image/")?;
    let (meta, b64) = rest.split_once(',')?;
    let meta_lower = meta.to_ascii_lowercase();
    if !meta_lower.contains(";base64") {
        return None;
    }
    let fmt = if meta_lower.starts_with("png") {
        gpui::ImageFormat::Png
    } else if meta_lower.starts_with("jpeg") || meta_lower.starts_with("jpg") {
        gpui::ImageFormat::Jpeg
    } else if meta_lower.starts_with("webp") {
        gpui::ImageFormat::Webp
    } else if meta_lower.starts_with("gif") {
        gpui::ImageFormat::Gif
    } else {
        gpui::ImageFormat::Png
    };
    let bytes = decode_base64_bytes(b64)?;
    if bytes.is_empty() {
        return None;
    }
    Some(Arc::new(gpui::Image::from_bytes(fmt, bytes)))
}

fn decode_base64_bytes(input: &str) -> Option<Vec<u8>> {
    fn val(b: u8) -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' | b'-' => Some(62),
            b'/' | b'_' => Some(63),
            _ => None,
        }
    }
    let mut cleaned = Vec::with_capacity(input.len());
    for &b in input.as_bytes() {
        if b.is_ascii_whitespace() {
            continue;
        }
        cleaned.push(b);
    }
    if cleaned.is_empty() || cleaned.len() % 4 == 1 {
        return None;
    }
    let mut out = Vec::with_capacity((cleaned.len() * 3) / 4);
    let mut i = 0;
    while i < cleaned.len() {
        let a = val(cleaned[i])?;
        let b = val(*cleaned.get(i + 1)?)?;
        let c_byte = *cleaned.get(i + 2).unwrap_or(&b'=');
        let d_byte = *cleaned.get(i + 3).unwrap_or(&b'=');
        out.push((a << 2) | (b >> 4));
        if c_byte != b'=' {
            let c = val(c_byte)?;
            out.push((b << 4) | (c >> 2));
            if d_byte != b'=' {
                let d = val(d_byte)?;
                out.push((c << 6) | d);
            }
        }
        i += 4;
    }
    Some(out)
}

/// Renders an editable field's committed text, IME preedit underline, and caret indicator.
pub fn render_ime_text_preview(
    committed: &str,
    preedit: Option<&str>,
    placeholder: &str,
    focused: bool,
    masked: bool,
    text_color: u32,
    pal: TdesktopThemePalette,
) -> AnyElement {
    let display_committed = if masked {
        "•".repeat(committed.chars().count())
    } else {
        committed.to_string()
    };
    let pre = preedit.filter(|s| !s.is_empty());
    let is_empty = display_committed.is_empty() && pre.is_none();

    let mut row = div()
        .w_full()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(1.0));

    if is_empty {
        row = row.child(
            div()
                .text_size(px(12.5))
                .text_color(rgb(pal.text_secondary))
                .child(placeholder.to_string()),
        );
    } else {
        if !display_committed.is_empty() {
            row = row.child(
                div()
                    .text_size(px(12.5))
                    .text_color(rgb(text_color))
                    .child(display_committed),
            );
        }
        if let Some(p) = pre {
            row = row.child(
                div()
                    .px(px(2.0))
                    .rounded(px(3.0))
                    .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.18))
                    .border_b_2()
                    .border_color(rgb(pal.accent))
                    .text_size(px(12.5))
                    .text_color(rgb(text_color))
                    .child(p.to_string()),
            );
        }
    }

    if focused {
        row = row.child(
            div()
                .w(px(1.5))
                .h(px(14.0))
                .rounded(px(1.0))
                .bg(rgb(pal.accent)),
        );
    }
    row.into_any_element()
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

    // Extended multi-device & cloud computer workbench state
    pub node_id: Option<String>,
    pub node_kind: ComputerNodeKind,
    pub run_policy: ComputerRunPolicy,
    pub active_tab: WorkbenchTab,
    pub terminal_input: Option<Entity<TextInputController>>,
    pub terminal_draft: String,
    pub terminal_focused: bool,
    pub terminal_history: Vec<TerminalLogEntry>,
    pub remote_cwd: String,
    pub remote_files: Vec<String>,
    pub file_preview: Option<(String, String)>,
    pub status_banner: Option<String>,
    pub pending_command: Option<WorkbenchCommand>,
    cached_frame_src: Option<String>,
    cached_frame_image: Option<Arc<gpui::Image>>,
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
            node_id: None,
            node_kind: ComputerNodeKind::LocalDevice,
            run_policy: ComputerRunPolicy::AskEveryTime,
            active_tab: WorkbenchTab::DesktopScreen,
            terminal_input: None,
            terminal_draft: String::new(),
            terminal_focused: false,
            terminal_history: Vec::new(),
            remote_cwd: "~".to_string(),
            remote_files: Vec::new(),
            file_preview: None,
            status_banner: None,
            pending_command: None,
            cached_frame_src: None,
            cached_frame_image: None,
        }
    }

    pub fn new_live(device_name: String, palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new(device_name);
        s.device.ip_address = String::new();
        s.device.os_name = String::new();
        s.device.resolution = String::new();
        s.device.latency_ms = 0;
        s.device.fps = 0;
        s.is_connected = false;
        s.palette = palette;
        s.language_id = language_id.to_string();
        s
    }

    pub fn ensure_terminal_input(&mut self, cx: &mut Context<Self>) {
        if self.terminal_input.is_none() {
            self.terminal_input = Some(cx.new(|_| TextInputController::new()));
        }
    }

    pub fn sync_lease(
        &mut self,
        device: HostDeviceSpec,
        frame_source: Option<String>,
        connected: bool,
    ) {
        self.device = device;
        self.update_frame_url(frame_source);
        self.is_connected = connected;
    }

    pub fn set_target_node(&mut self, node: &ComputerNodeDto) {
        self.node_id = Some(node.node_id.clone());
        self.node_kind = node.kind;
        self.run_policy = node.run_policy;
        self.is_connected = node.online;
        let ip_or_host = node
            .host
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&node.ip_address)
            .to_string();
        self.device = HostDeviceSpec {
            name: node.display_name.clone(),
            ip_address: ip_or_host,
            os_name: node.os_name.clone(),
            resolution: if !node.resolution.is_empty() {
                node.resolution.clone()
            } else if node.latest_frame_data_url.is_some() {
                "1280x800 @ Live".to_string()
            } else {
                "Remote Control".to_string()
            },
            latency_ms: if node.online { 18 } else { 0 },
            fps: if node.online { 30 } else { 0 },
            is_tunnel_encrypted: true,
        };
        if let Some(wd) = node.workdir.as_ref().filter(|s| !s.trim().is_empty()) {
            self.remote_cwd = wd.clone();
        }
        self.update_frame_url(node.latest_frame_data_url.clone());
    }

    pub fn refresh_target_node_status(&mut self, node: &ComputerNodeDto) {
        self.run_policy = node.run_policy;
        self.is_connected = node.online;
        self.device.name = node.display_name.clone();
        self.device.ip_address = node
            .host
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&node.ip_address)
            .to_string();
        if node.latest_frame_data_url.is_some() {
            self.update_frame_url(node.latest_frame_data_url.clone());
        }
    }

    pub fn update_frame_url(&mut self, url: Option<String>) {
        if self.cached_frame_src != url {
            self.cached_frame_image = url.as_deref().and_then(decode_data_url_image);
            self.cached_frame_src = url.clone();
        }
        self.frame_source = url;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }
    pub fn set_language(&mut self, language_id: impl Into<String>) {
        self.language_id = language_id.into();
    }
    pub fn take_action(&mut self) -> Option<String> {
        self.pending_action.take()
    }
    pub fn take_command(&mut self) -> Option<WorkbenchCommand> {
        self.pending_command.take()
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

    pub fn push_terminal_output(
        &mut self,
        command: String,
        stdout: String,
        stderr: String,
        exit_code: i32,
        new_cwd: Option<String>,
    ) {
        if let Some(cwd) = new_cwd.filter(|s| !s.trim().is_empty()) {
            self.remote_cwd = cwd;
        }
        self.terminal_history.push(TerminalLogEntry {
            command,
            stdout,
            stderr,
            exit_code,
            cwd: self.remote_cwd.clone(),
        });
        if self.terminal_history.len() > 80 {
            let excess = self.terminal_history.len() - 80;
            self.terminal_history.drain(0..excess);
        }
    }

    pub fn set_remote_files(&mut self, cwd: String, files: Vec<String>) {
        if !cwd.trim().is_empty() {
            self.remote_cwd = cwd;
        }
        self.remote_files = files;
    }

    pub fn set_file_preview(&mut self, path: String, content: String) {
        self.file_preview = Some((path, content));
    }

    pub fn submit_terminal_draft(&mut self) -> bool {
        let cmd = self.terminal_draft.trim().to_string();
        if cmd.is_empty() {
            return false;
        }
        self.terminal_draft.clear();
        let cwd = Some(self.remote_cwd.clone()).filter(|s| !s.is_empty());
        self.pending_command = Some(WorkbenchCommand::RunShell { command: cmd, cwd });
        self.pending_action = Some("command_enqueued".to_string());
        true
    }

    pub fn is_terminal_input_active(&self) -> bool {
        self.active_tab == WorkbenchTab::RemoteTerminal && self.terminal_focused
    }

    pub fn terminal_text_and_preedit(&self, cx: &App) -> (String, Option<String>) {
        if let Some(ctrl) = self.terminal_input.as_ref() {
            return ctrl.read(cx).text_and_preedit(self.terminal_draft.clone());
        }
        (self.terminal_draft.clone(), None)
    }

    pub fn clear_terminal_input(&mut self, cx: &mut Context<Self>) {
        self.terminal_draft.clear();
        if let Some(ctrl) = self.terminal_input.as_ref() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        cx.notify();
    }

    pub fn push_terminal_text(&mut self, text: &str, cx: &mut Context<Self>) {
        if let Some(ctrl) = self.terminal_input.as_ref() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        self.terminal_draft.push_str(text);
        cx.notify();
    }

    pub fn handle_terminal_non_char_key(
        &mut self,
        key: &str,
        is_alt: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_terminal_input_active() {
            return false;
        }
        match key {
            "backspace" | "delete" => {
                let changed = if is_alt {
                    crate::components::ui::delete_prev_word(&mut self.terminal_draft)
                } else {
                    crate::components::ui::pop_last_char(&mut self.terminal_draft)
                };
                if changed {
                    cx.notify();
                }
                true
            }
            "enter" => {
                if self.submit_terminal_draft() {
                    cx.notify();
                }
                true
            }
            _ => false,
        }
    }

    pub fn replace_and_mark_terminal_input(
        &mut self,
        new_text: &str,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        self.ensure_terminal_input(cx);
        let commit = self
            .terminal_input
            .as_ref()
            .and_then(|c| c.update(cx, |ctrl, _| ctrl.handle_replace_and_mark(new_text)));
        if let Some(text) = commit.as_ref() {
            self.terminal_draft.push_str(text);
        }
        cx.notify();
        commit
    }

    fn toggle_chip(label: &str, active: bool, pal: &TdesktopThemePalette) -> Div {
        div()
            .cursor_pointer()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(if active {
                TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.18)
            } else {
                rgb(pal.input_bg).into()
            })
            .border_1()
            .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(if active {
                pal.accent
            } else {
                pal.text_secondary
            }))
            .child(label.to_string())
    }

    fn render_screen_pane(
        &self,
        pal: &TdesktopThemePalette,
        lang: &str,
        dev: &HostDeviceSpec,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let preview = if let Some(ref arc_img) = self.cached_frame_image {
            img(arc_img.clone())
                .size_full()
                .object_fit(ObjectFit::Contain)
                .into_any_element()
        } else if let Some(src) = self.frame_source.as_ref().filter(|s| !s.starts_with("data:")) {
            img(src.clone())
                .size_full()
                .object_fit(ObjectFit::Contain)
                .into_any_element()
        } else {
            div()
                .size_full()
                .rounded(px(10.0))
                .bg(rgb(pal.history_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(10.0))
                .child(tg_icon(IconName::ComputerDesktop, 44.0, pal.icon_fg))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(if self.is_connected {
                            tr_str(
                                lang,
                                "点击画布即可控制鼠标，或点击下方捕获最新桌面画面",
                                "Click the viewport to send mouse events or refresh live frame",
                            )
                        } else {
                            tr_str(
                                lang,
                                "远程设备当前离线或待连接",
                                "Remote computer is offline or awaiting connection",
                            )
                        }),
                )
                .when(dev.is_tunnel_encrypted, |el| {
                    el.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(tg_icon(IconName::Lock, 12.0, pal.success))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(tr_str(
                                        lang,
                                        "安全控制通道已就绪",
                                        "Encrypted Control Channel Ready",
                                    )),
                            ),
                    )
                })
                .into_any_element()
        };

        div()
            .flex_1()
            .min_h(px(0.0))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .id("workbench-desktop-viewport")
                    .cursor_pointer()
                    .flex_1()
                    .min_h(px(0.0))
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .bg(rgb(pal.history_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, cx| {
                            let x = f32::from(ev.position.x).max(0.0) as i32;
                            let y = f32::from(ev.position.y).max(0.0) as i32;
                            this.terminal_focused = false;
                            this.status_banner = Some(format!(
                                "{} ({x}, {y})",
                                tr_str(&this.language_id, "已发送远程点击", "Sent GUI click")
                            ));
                            this.pending_command = Some(WorkbenchCommand::SendGuiClick { x, y });
                            this.pending_action = Some(format!("gui_click:{x}:{y}"));
                            cx.notify();
                        }),
                    )
                    .child(preview),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(10.0))
                    .py(px(8.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(Self::quick_cmd_btn(
                                "wb-refresh-btn",
                                tr_str(lang, "刷新画面", "Refresh Screen"),
                                true,
                                pal,
                                cx.listener(|this, _, _, cx| {
                                    this.pending_command = Some(WorkbenchCommand::RefreshScreen);
                                    this.status_banner = Some(
                                        tr_str(
                                            &this.language_id,
                                            "正在抓取远程桌面画面...",
                                            "Capturing remote screen...",
                                        )
                                        .to_string(),
                                    );
                                    cx.notify();
                                }),
                            ))
                            .child(Self::quick_cmd_btn(
                                "wb-open-term-btn",
                                tr_str(lang, "打开终端", "Terminal"),
                                false,
                                pal,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = WorkbenchTab::RemoteTerminal;
                                    this.terminal_focused = true;
                                    cx.notify();
                                }),
                            ))
                            .child(Self::quick_cmd_btn(
                                "wb-list-files-btn",
                                tr_str(lang, "浏览文件", "Files"),
                                false,
                                pal,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = WorkbenchTab::RemoteTerminal;
                                    this.pending_command = Some(WorkbenchCommand::ListFiles {
                                        path: this.remote_cwd.clone(),
                                    });
                                    cx.notify();
                                }),
                            )),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(pal.text_secondary))
                            .child(format!(
                                "{} · {}",
                                tr_str(lang, "策略", "Policy"),
                                if lang.starts_with("zh") {
                                    self.run_policy.label_zh()
                                } else {
                                    self.run_policy.label_en()
                                }
                            )),
                    ),
            )
            .into_any_element()
    }

    fn render_terminal_pane(
        &self,
        pal: &TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (draft_text, draft_preedit) = self
            .terminal_input
            .as_ref()
            .map(|c| c.read(cx).text_and_preedit(self.terminal_draft.clone()))
            .unwrap_or_else(|| (self.terminal_draft.clone(), None));
        let is_zh = lang.starts_with("zh");
        let term_bg = match pal.mode {
            TdesktopThemeMode::Day => 0x111827,
            _ => pal.window_bg,
        };

        let mut logs_col = div().flex().flex_col().gap(px(8.0));
        if self.terminal_history.is_empty() {
            logs_col = logs_col.child(
                div()
                    .text_xs()
                    .text_color(rgb(0x94A3B8))
                    .child(if is_zh {
                        "在下方输入 Shell 命令以在远程计算机执行（策略为“每次询问”时会弹出授权确认）。"
                    } else {
                        "Enter a shell command below to run on this remote computer."
                    }),
            );
        } else {
            for entry in &self.terminal_history {
                logs_col = logs_col.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(3.0))
                        .pb(px(6.0))
                        .border_b_1()
                        .border_color(rgb(0x1E293B))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x38BDF8))
                                        .child(format!("{} $ {}", entry.cwd, entry.command)),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(rgb(if entry.exit_code == 0 {
                                            0x4ADE80
                                        } else {
                                            0xF87171
                                        }))
                                        .child(format!("exit {}", entry.exit_code)),
                                ),
                        )
                        .when(!entry.stdout.is_empty(), |col| {
                            col.child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xE2E8F0))
                                    .child(entry.stdout.clone()),
                            )
                        })
                        .when(!entry.stderr.is_empty(), |col| {
                            col.child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xFCA5A5))
                                    .child(entry.stderr.clone()),
                            )
                        }),
                );
            }
        }

        let mut files_col = div().flex().flex_col().gap(px(4.0));
        files_col = files_col.child(
            div()
                .id("wb-parent-dir")
                .cursor_pointer()
                .px(px(8.0))
                .py(px(5.0))
                .rounded(px(6.0))
                .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                .text_xs()
                .text_color(rgb(pal.accent))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        let next = parent_path(&this.remote_cwd);
                        this.pending_command = Some(WorkbenchCommand::ListFiles { path: next });
                        cx.notify();
                    }),
                )
                .child(".. (上级目录)"),
        );
        if self.remote_files.is_empty() {
            files_col = files_col.child(
                div()
                    .px(px(8.0))
                    .py(px(10.0))
                    .text_xs()
                    .text_color(rgb(pal.text_secondary))
                    .child(if is_zh {
                        "点击“刷新目录”加载远程文件列表"
                    } else {
                        "Click Refresh to load remote directory"
                    }),
            );
        } else {
            for (idx, item) in self.remote_files.iter().enumerate() {
                let item_name = item.clone();
                let is_dir = item_name.ends_with('/');
                files_col = files_col.child(
                    div()
                        .id(("wb-file-item", idx))
                        .cursor_pointer()
                        .px(px(8.0))
                        .py(px(5.0))
                        .rounded(px(6.0))
                        .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(6.0))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                let clean = item_name.trim_end_matches('/');
                                let joined = join_remote_path(&this.remote_cwd, clean);
                                if is_dir {
                                    this.pending_command =
                                        Some(WorkbenchCommand::ListFiles { path: joined });
                                } else {
                                    this.pending_command =
                                        Some(WorkbenchCommand::ReadRemoteFile { path: joined });
                                }
                                cx.notify();
                            }),
                        )
                        .child(tg_icon(
                            if is_dir {
                                IconName::FolderAll
                            } else {
                                IconName::FileDoc
                            },
                            13.0,
                            if is_dir { pal.accent } else { pal.icon_fg },
                        ))
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(pal.text_primary))
                                .truncate()
                                .child(item.clone()),
                        ),
                );
            }
        }

        div()
            .flex_1()
            .min_h(px(0.0))
            .flex()
            .flex_row()
            .gap(px(10.0))
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .rounded(px(12.0))
                    .bg(rgb(term_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .border_b_1()
                            .border_color(rgb(0x1E293B))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(tg_icon(IconName::TerminalMcp, 14.0, 0x38BDF8))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xE2E8F0))
                                            .child(format!("cwd: {}", self.remote_cwd)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(6.0))
                                    .child(Self::quick_cmd_btn(
                                        "wb-quick-uname",
                                        "uname -a",
                                        false,
                                        pal,
                                        cx.listener(|this, _, _, cx| {
                                            this.pending_command =
                                                Some(WorkbenchCommand::RunShell {
                                                    command: "uname -a".to_string(),
                                                    cwd: Some(this.remote_cwd.clone()),
                                                });
                                            cx.notify();
                                        }),
                                    ))
                                    .child(Self::quick_cmd_btn(
                                        "wb-quick-ls",
                                        "ls -la",
                                        false,
                                        pal,
                                        cx.listener(|this, _, _, cx| {
                                            this.pending_command =
                                                Some(WorkbenchCommand::RunShell {
                                                    command: "ls -la".to_string(),
                                                    cwd: Some(this.remote_cwd.clone()),
                                                });
                                            cx.notify();
                                        }),
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .id("wb-term-history")
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
                            .p(px(12.0))
                            .child(logs_col),
                    )
                    .child(
                        div()
                            .p(px(10.0))
                            .border_t_1()
                            .border_color(rgb(0x1E293B))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("wb-term-input-box")
                                    .cursor_text()
                                    .flex_1()
                                    .min_h(px(34.0))
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0x0F172A))
                                    .border_1()
                                    .border_color(rgb(if self.terminal_focused {
                                        pal.accent
                                    } else {
                                        0x334155
                                    }))
                                    .flex()
                                    .items_center()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.ensure_terminal_input(cx);
                                            this.terminal_focused = true;
                                            cx.notify();
                                        }),
                                    )
                                    .child(render_ime_text_preview(
                                        &draft_text,
                                        draft_preedit.as_deref(),
                                        tr_str(
                                            lang,
                                            "输入远程 Shell 命令并回车...",
                                            "Enter remote shell command...",
                                        ),
                                        self.terminal_focused,
                                        false,
                                        0xF8FAFC,
                                        *pal,
                                    )),
                            )
                            .child(Self::quick_cmd_btn(
                                "wb-run-shell-btn",
                                tr_str(lang, "执行", "Run"),
                                true,
                                pal,
                                cx.listener(|this, _, _, cx| {
                                    if this.submit_terminal_draft() {
                                        cx.notify();
                                    }
                                }),
                            )),
                    ),
            )
            .child(
                div()
                    .w(px(270.0))
                    .flex_shrink_0()
                    .h_full()
                    .rounded(px(12.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .border_b_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(tr_str(lang, "远程文件", "Remote Files")),
                            )
                            .child(Self::quick_cmd_btn(
                                "wb-refresh-files-btn",
                                tr_str(lang, "刷新目录", "Refresh"),
                                false,
                                pal,
                                cx.listener(|this, _, _, cx| {
                                    this.pending_command = Some(WorkbenchCommand::ListFiles {
                                        path: this.remote_cwd.clone(),
                                    });
                                    cx.notify();
                                }),
                            )),
                    )
                    .child(
                        div()
                            .id("wb-files-scroll")
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
                            .p(px(8.0))
                            .child(files_col),
                    )
                    .when_some(self.file_preview.as_ref(), |panel, (fname, content)| {
                        panel.child(
                            div()
                                .max_h(px(160.0))
                                .border_t_1()
                                .border_color(rgb(pal.border_fg))
                                .p(px(8.0))
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.accent))
                                        .truncate()
                                        .child(fname.clone()),
                                )
                                .child(
                                    div()
                                        .id("wb-file-preview-scroll")
                                        .flex_1()
                                        .overflow_y_scroll()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(content.clone()),
                                ),
                        )
                    }),
            )
            .into_any_element()
    }

    fn quick_cmd_btn(
        id: &'static str,
        label: &str,
        primary: bool,
        pal: &TdesktopThemePalette,
        on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .cursor_pointer()
            .px(px(10.0))
            .py(px(5.0))
            .rounded(px(7.0))
            .bg(rgb(if primary { pal.accent } else { pal.input_bg }))
            .border_1()
            .border_color(rgb(if primary { pal.accent } else { pal.border_fg }))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(if primary {
                pal.active_button_fg
            } else {
                pal.text_primary
            }))
            .on_mouse_down(MouseButton::Left, on_click)
            .child(label.to_string())
    }
}

fn parent_path(p: &str) -> String {
    let trimmed = p.trim_end_matches('/');
    if trimmed.is_empty() || trimmed == "~" {
        return "~".to_string();
    }
    match trimmed.rsplit_once('/') {
        Some(("", _)) => "/".to_string(),
        Some((parent, _)) => parent.to_string(),
        None => "~".to_string(),
    }
}

fn join_remote_path(base: &str, child: &str) -> String {
    if child.starts_with('/') {
        return child.to_string();
    }
    let clean_base = base.trim_end_matches('/');
    if clean_base.is_empty() {
        child.to_string()
    } else {
        format!("{clean_base}/{child}")
    }
}

impl Default for ComputerWorkbench {
    fn default() -> Self {
        Self::new("Dev-Workstation-01".to_string())
    }
}

impl Render for ComputerWorkbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_terminal_input(cx);
        let pal = self.palette;
        let lang = self.language_id.clone();
        let dev = self.device.clone();
        let metrics = if dev.ip_address.is_empty() && dev.latency_ms == 0 {
            if self.is_connected {
                tr_str(&lang, "已连接", "Connected").to_string()
            } else {
                tr_str(&lang, "离线", "Offline").to_string()
            }
        } else {
            format!(
                "{} • {}ms • {} FPS",
                dev.ip_address, dev.latency_ms, dev.fps
            )
        };
        let is_desktop = self.active_tab == WorkbenchTab::DesktopScreen;

        div()
            .w(px(920.0))
            .h(px(620.0))
            .max_w_full()
            .max_h_full()
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .rounded(px(16.0))
            .shadow_xl()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(
                div()
                    .px(px(16.0))
                    .py(px(11.0))
                    .bg(rgb(pal.top_bar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(10.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(tg_icon(IconName::ComputerDesktop, 18.0, pal.accent))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(dev.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .px(px(6.0))
                                                    .py(px(1.0))
                                                    .rounded(px(999.0))
                                                    .bg(rgb(pal.input_bg))
                                                    .text_size(px(10.5))
                                                    .text_color(rgb(if self.is_connected {
                                                        pal.success
                                                    } else {
                                                        pal.text_secondary
                                                    }))
                                                    .child(if self.node_kind == ComputerNodeKind::CloudServer {
                                                        tr_str(&lang, "云电脑", "Cloud")
                                                    } else {
                                                        tr_str(&lang, "本地", "Local")
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .child(metrics),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(Self::quick_cmd_btn(
                                "wb-tab-desktop",
                                tr_str(&lang, "远程桌面", "Desktop"),
                                is_desktop,
                                &pal,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = WorkbenchTab::DesktopScreen;
                                    this.terminal_focused = false;
                                    cx.notify();
                                }),
                            ))
                            .child(Self::quick_cmd_btn(
                                "wb-tab-terminal",
                                tr_str(&lang, "远程终端与文件", "Terminal & Files"),
                                !is_desktop,
                                &pal,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = WorkbenchTab::RemoteTerminal;
                                    this.terminal_focused = true;
                                    cx.notify();
                                }),
                            ))
                            .child(
                                Self::toggle_chip(
                                    tr_str(&lang, "鼠标", "Mouse"),
                                    self.is_mouse_captured,
                                    &pal,
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.toggle_mouse_capture();
                                        cx.notify();
                                    }),
                                ),
                            )
                            .child(
                                Self::toggle_chip(
                                    tr_str(&lang, "键盘", "Keys"),
                                    self.is_keyboard_captured,
                                    &pal,
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.toggle_keyboard_capture();
                                        cx.notify();
                                    }),
                                ),
                            )
                            .child(
                                Self::toggle_chip(
                                    tr_str(&lang, "剪贴板", "Clip"),
                                    self.is_clipboard_synced,
                                    &pal,
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.toggle_clipboard_sync();
                                        cx.notify();
                                    }),
                                ),
                            )
                            .child(
                                Self::toggle_chip("HD", self.is_quality_hd, &pal).on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.toggle_quality();
                                        cx.notify();
                                    }),
                                ),
                            )
                            .child(
                                div()
                                    .id("wb-disconnect-btn")
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.danger))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.should_close = true;
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(&lang, "断开", "Disconnect")),
                            ),
                    ),
            )
            .when_some(self.status_banner.as_ref(), |col, msg| {
                col.child(
                    div()
                        .px(px(16.0))
                        .py(px(6.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.12))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .text_xs()
                        .text_color(rgb(pal.text_primary))
                        .child(msg.clone()),
                )
            })
            .child(div().flex_1().min_h(px(0.0)).p(px(14.0)).flex().child(
                if is_desktop {
                    self.render_screen_pane(&pal, &lang, &dev, cx)
                } else {
                    self.render_terminal_pane(&pal, &lang, cx)
                },
            ))
    }
}
