use gpui::*;
use crate::app::updater::{DesktopUpdateSnapshot, CURRENT_APP_VERSION};
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_info_row,
    settings_section_header, settings_subsection_title, settings_toggle_row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyType {
    Direct,
    Http,
    Socks5,
    Mtproto,
}

pub struct SettingsAdvancedSections {
    pub proxy_type: ProxyType,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub opengl_enabled: bool,
    pub native_window_frame: bool,
    pub tray_icon_enabled: bool,
    pub ask_download_path: bool,
    pub download_path: String,
    pub auto_update_enabled: bool,
    pub is_visible: bool,
    pub local_storage_managed: bool,
    pub update_available: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_open_connection_box: bool,
    pub pending_open_local_storage: bool,
    pub pending_open_power_saving: bool,
    pub pending_open_export_box: bool,
    pub pending_prefs_sync: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_status_text: String,
    pub update_progress_pct: Option<u8>,
    pub is_checking_or_downloading: bool,
    pub pending_check_updates: bool,
    pub pending_apply_update: bool,
}

impl SettingsAdvancedSections {
    pub fn new() -> Self {
        Self {
            proxy_type: ProxyType::Direct,
            proxy_host: String::new(),
            proxy_port: 1080,
            opengl_enabled: true,
            native_window_frame: false,
            tray_icon_enabled: true,
            ask_download_path: false,
            download_path: "~/Downloads/Fabushi".into(),
            auto_update_enabled: true,
            is_visible: false,
            local_storage_managed: false,
            update_available: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_open_connection_box: false,
            pending_open_local_storage: false,
            pending_open_power_saving: false,
            pending_open_export_box: false,
            pending_prefs_sync: false,
            current_version: CURRENT_APP_VERSION.to_string(),
            latest_version: None,
            update_status_text: format!("Fabushi v{}", CURRENT_APP_VERSION),
            update_progress_pct: None,
            is_checking_or_downloading: false,
            pending_check_updates: false,
            pending_apply_update: false,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.palette = palette;
        s.language_id = language_id.into();
        s.is_visible = true;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn lng_settings_manage_local_storage(&mut self) { self.local_storage_managed = true; self.pending_open_local_storage = true; }
    pub fn has_update(&mut self) -> bool { self.update_available = self.auto_update_enabled; self.update_available }
    pub fn lng_connection_proxy(&mut self) { self.proxy_type = ProxyType::Socks5; }
    pub fn to_native_separators(&mut self) {
        if self.download_path.contains("C:/") { self.download_path = self.download_path.replace('/', "\\"); }
    }
    pub fn setup_connection_type(&mut self) { self.proxy_type = ProxyType::Direct; }
    pub fn lng_connection_auto(&mut self) { self.proxy_type = ProxyType::Direct; }

    /// Applies the latest updater state snapshot to this section view.
    pub fn sync_update_snapshot(&mut self, snapshot: &DesktopUpdateSnapshot) {
        self.current_version = snapshot.current_version.clone();
        self.latest_version = snapshot.latest_version.clone();
        self.update_status_text = snapshot.status_text.clone();
        self.update_progress_pct = snapshot.progress_pct;
        self.is_checking_or_downloading = snapshot.is_busy;
        self.update_available = snapshot.latest_version.is_some();
    }

    pub fn sync_proxy(&mut self, proxy_type_str: &str, host: &str, port: u16, enabled: bool) {
        let lower_kind = proxy_type_str.trim().to_ascii_lowercase();
        if !enabled || lower_kind == "direct" || lower_kind == "none" || host.trim().is_empty() {
            self.proxy_type = ProxyType::Direct;
            self.proxy_host.clear();
            self.proxy_port = 1080;
            return;
        }
        self.proxy_type = match lower_kind.as_str() {
            "http" | "https" | "web" => ProxyType::Http,
            "mtproto" => ProxyType::Mtproto,
            _ => ProxyType::Socks5,
        };
        self.proxy_host = host.trim().to_string();
        self.proxy_port = if port == 0 { 1080 } else { port };
    }
}

impl Default for SettingsAdvancedSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsAdvancedSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let conn_label = match self.proxy_type {
            ProxyType::Direct => tr_str(&lang, "直连 (默认)", "Default (Direct)"),
            ProxyType::Http => tr_str(&lang, "HTTP 代理", "HTTP Proxy"),
            ProxyType::Socks5 => tr_str(&lang, "SOCKS5 代理", "SOCKS5 Proxy"),
            ProxyType::Mtproto => tr_str(&lang, "MTProto 代理", "MTProto Proxy"),
        };
        let conn_detail = if self.proxy_type == ProxyType::Direct || self.proxy_host.is_empty() {
            conn_label.to_string()
        } else {
            format!("{} ({}:{})", conn_label, self.proxy_host, self.proxy_port)
        };

        let version_badge = format!("Fabushi v{}", self.current_version);
        let update_action_title = if self.is_checking_or_downloading {
            if let Some(pct) = self.update_progress_pct {
                if lang.starts_with("zh") {
                    format!("正在下载更新 ({pct}%)…")
                } else {
                    format!("Downloading Update ({pct}%)…")
                }
            } else {
                tr_str(&lang, "正在检查更新…", "Checking for Updates…").to_string()
            }
        } else if let Some(ref latest) = self.latest_version {
            if lang.starts_with("zh") {
                format!("一键更新至 Fabushi v{latest}")
            } else {
                format!("One-Click Update to v{latest}")
            }
        } else {
            tr_str(&lang, "检查更新", "Check for Updates").to_string()
        };
        let update_value_label = if self.is_checking_or_downloading {
            self.update_status_text.clone()
        } else if let Some(ref latest) = self.latest_version {
            if lang.starts_with("zh") {
                format!("立即安装 v{latest} 并重启")
            } else {
                format!("Install v{latest} & Restart")
            }
        } else {
            self.update_status_text.clone()
        };
        let can_apply_now = self.latest_version.is_some() && !self.is_checking_or_downloading;

        settings_container(pal.clone())
            .child(settings_section_header(IconName::SettingsGear, tr_str(&lang, "高级设置", "Advanced"), Some(tr_str(&lang, "版本更新、连接方式、缓存存储与硬件加速", "App updates, connection, storage, and hardware acceleration")), pal.clone()))
            .child(settings_subsection_title(tr_str(&lang, "版本与软件更新", "Version & App Updates"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(settings_info_row(tr_str(&lang, "当前版本", "Current Version"), &version_badge, pal.clone()))
                    .child(settings_action_row(
                        IconName::ClockSending,
                        &update_action_title,
                        &update_value_label,
                        pal.clone(),
                        false,
                        move |this| {
                            if this.is_checking_or_downloading {
                                return;
                            }
                            if can_apply_now {
                                this.pending_apply_update = true;
                            } else {
                                this.pending_check_updates = true;
                            }
                        },
                        cx,
                    ))
                    .child(settings_toggle_row(IconName::ClockSending, tr_str(&lang, "自动识别并提示更新", "Automatically Check for Updates"), None, self.auto_update_enabled, pal.clone(), |this| { this.auto_update_enabled = !this.auto_update_enabled; this.pending_prefs_sync = true; }, cx)),
            )
            .child(settings_subsection_title(tr_str(&lang, "数据与存储", "Data and Storage"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(settings_action_row(IconName::FileDoc, tr_str(&lang, "管理本地存储", "Manage Local Storage"), "", pal.clone(), false, |this| this.lng_settings_manage_local_storage(), cx))
                    .child(settings_action_row(IconName::NightMoon, tr_str(&lang, "省电模式", "Power Saving"), "", pal.clone(), false, |this| this.pending_open_power_saving = true, cx))
                    .child(settings_info_row(tr_str(&lang, "下载路径", "Download Path"), &self.download_path, pal.clone()))
                    .child(settings_toggle_row(IconName::Paperclip, tr_str(&lang, "每次下载询问保存路径", "Ask Download Path for Each File"), None, self.ask_download_path, pal.clone(), |this| { this.ask_download_path = !this.ask_download_path; this.pending_prefs_sync = true; }, cx))
                    .child(settings_action_row(IconName::Forward, tr_str(&lang, "导出聊天数据", "Export Chat Data"), "", pal.clone(), false, |this| this.pending_open_export_box = true, cx)),
            )
            .child(settings_subsection_title(tr_str(&lang, "连接与系统集成", "Connection & System Integration"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(settings_action_row(IconName::SettingsGear, tr_str(&lang, "连接类型", "Connection Type"), &conn_detail, pal.clone(), false, |this| {
                        this.pending_open_connection_box = true;
                    }, cx))
                    .child(settings_toggle_row(IconName::ComputerDesktop, tr_str(&lang, "硬件加速", "Hardware Acceleration (Metal / OpenGL)"), None, self.opengl_enabled, pal.clone(), |this| { this.opengl_enabled = !this.opengl_enabled; this.pending_prefs_sync = true; }, cx))
                    .child(settings_toggle_row(IconName::SidebarPanel, tr_str(&lang, "使用系统窗口边框", "Use System Window Frame"), None, self.native_window_frame, pal.clone(), |this| { this.native_window_frame = !this.native_window_frame; this.pending_prefs_sync = true; }, cx))
                    .child(settings_toggle_row(IconName::ComputerDesktop, tr_str(&lang, "显示托盘图标", "Show Tray Icon"), None, self.tray_icon_enabled, pal, |this| { this.tray_icon_enabled = !this.tray_icon_enabled; this.pending_prefs_sync = true; }, cx)),
            )
    }
}
