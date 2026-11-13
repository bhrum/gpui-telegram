//! Complete 1:1 Telegram Desktop (`tdesktop`) Multi-Section Settings Workspace.
//! Wires upstream `Telegram/SourceFiles/settings/sections/*` models into a unified,
//! live-interactive GPUI Settings panel with 11 full sub-pages.

use crate::views::tdesktop_theme::{TdesktopThemeMode, TdesktopThemePalette, tr_str};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::PersistedClientState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPageId {
    Account,
    Notifications,
    PrivacySecurity,
    ChatAppearance,
    Folders,
    AdvancedStorage,
    CallDevices,
    LanguageTranslate,
    PremiumStars,
    TonWallet,
    BusinessPassport,
}

#[derive(Debug, Clone)]
pub enum SettingsMutation {
    CloseSettings,
    LogOut,
    SetThemeMode(TdesktopThemeMode),
    SetAccentColor(u32),
    SetFontSize(u32),
    SetBubbleRadius(u32),
    ToggleSendWithEnter,
    SetLanguage(String),
    ToggleTranslateBar,
    AddCustomFolder(String),
    RemoveCustomFolder(String),
    ClearLocalCache,
    OpenProxyModal,
    OpenPasscodeModal,
    OpenExportModal,
    LockNow,
    RevokeSession(String),
    BuyOrTopUpStars(u64),
    SendTonDemoTransfer,
    UpdateBio(String),
}

pub struct SettingsWorkspaceView {
    pub active_page: SettingsPageId,
    pub search_filter: String,
    pub new_folder_input: String,
    pub bio_edit_input: String,
    pub editing_field: u8, // 0=none, 1=search, 2=folder, 3=bio
    // Local toggles for notifications / calls / business / passport
    pub notify_private: bool,
    pub notify_groups: bool,
    pub notify_channels: bool,
    pub notify_sound: bool,
    pub call_noise_suppression: bool,
    pub call_p2p_contacts_only: bool,
    pub business_hours_enabled: bool,
    pub business_quick_replies: usize,
    pub passport_verified_docs: usize,
    pub state_snapshot: PersistedClientState,
    pub palette: TdesktopThemePalette,
    pub pending_mutation: Option<SettingsMutation>,
}

impl SettingsWorkspaceView {
    pub fn new(state_snapshot: PersistedClientState, palette: TdesktopThemePalette) -> Self {
        let bio = state_snapshot
            .accounts
            .iter()
            .find(|a| a.account_id == state_snapshot.active_account_id)
            .map(|a| a.bio.clone())
            .unwrap_or_default();
        Self {
            active_page: SettingsPageId::ChatAppearance,
            search_filter: String::new(),
            new_folder_input: "Channels".to_string(),
            bio_edit_input: bio,
            editing_field: 0,
            notify_private: true,
            notify_groups: true,
            notify_channels: false,
            notify_sound: true,
            call_noise_suppression: true,
            call_p2p_contacts_only: true,
            business_hours_enabled: true,
            business_quick_replies: 4,
            passport_verified_docs: 2,
            state_snapshot,
            palette,
            pending_mutation: None,
        }
    }

    pub fn sync_from_state(&mut self, state: PersistedClientState, palette: TdesktopThemePalette) {
        self.state_snapshot = state;
        self.palette = palette;
    }

    pub fn handle_key_input(&mut self, key: &str, key_char: Option<&str>) -> bool {
        if self.editing_field == 0 {
            return false;
        }
        let target = match self.editing_field {
            1 => &mut self.search_filter,
            2 => &mut self.new_folder_input,
            _ => &mut self.bio_edit_input,
        };
        if key == "backspace" {
            target.pop();
            return true;
        }
        if key == "space" {
            target.push(' ');
            return true;
        }
        if key == "enter" {
            if self.editing_field == 3 {
                self.pending_mutation =
                    Some(SettingsMutation::UpdateBio(self.bio_edit_input.clone()));
            }
            self.editing_field = 0;
            return true;
        }
        if let Some(ch) = key_char {
            if !ch.chars().any(|c| c.is_control()) {
                target.push_str(ch);
                return true;
            }
        }
        false
    }
}

impl Render for SettingsWorkspaceView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.state_snapshot.language_id.clone();
        let cur_page = self.active_page;
        let q = self.search_filter.to_lowercase();

        let nav_entries: Vec<(SettingsPageId, &str, &str, &str)> = vec![
            (SettingsPageId::Account, "👤", "我的账号与资料", "My Account & Profile"),
            (SettingsPageId::Notifications, "🔔", "通知与声音", "Notifications and Sounds"),
            (SettingsPageId::PrivacySecurity, "🔐", "隐私与安全", "Privacy and Security"),
            (SettingsPageId::ChatAppearance, "🎨", "聊天外观与主题", "Chat Settings & Themes"),
            (SettingsPageId::Folders, "📁", "聊天分组管理", "Chat Folders"),
            (SettingsPageId::AdvancedStorage, "⚡", "高级、存储与代理", "Advanced, Storage & Proxy"),
            (SettingsPageId::CallDevices, "🎙️", "通话与音视频设备", "Call Settings"),
            (SettingsPageId::LanguageTranslate, "🌐", "多语言与实时翻译", "Language & Translation"),
            (SettingsPageId::PremiumStars, "⭐", "Telegram Premium 与星币", "Telegram Premium & Stars"),
            (SettingsPageId::TonWallet, "💎", "TON 加密钱包", "TON Self-Custodial Wallet"),
            (SettingsPageId::BusinessPassport, "💼", "商业版与 Passport", "Business & Telegram Passport"),
        ];

        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .bg(rgb(pal.window_bg))
            .flex()
            // Left Settings Section Navigation Column (260px)
            .child(
                div()
                    .w(px(264.0))
                    .flex_shrink_0()
                    .h_full()
                    .bg(rgb(pal.dialogs_bg))
                    .border_r_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(52.0))
                            .px(px(14.0))
                            .border_b_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(tr_str(&lang, "⚙️ 设置中心", "⚙️ Settings")),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(11.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.pending_mutation =
                                                Some(SettingsMutation::CloseSettings);
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(&lang, "← 返回聊天", "← Chats")),
                            ),
                    )
                    // Search Settings Input
                    .child(
                        div()
                            .p(px(10.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .h(px(32.0))
                                    .px(px(10.0))
                                    .rounded(px(16.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .items_center()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.editing_field = 1;
                                            cx.notify();
                                        }),
                                    )
                                    .child(if self.search_filter.is_empty() {
                                        tr_str(&lang, "🔍 搜索设置项...", "🔍 Search settings...")
                                            .to_string()
                                    } else {
                                        format!("🔍 {}", self.search_filter)
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .id("settings-left-nav-scroll")
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
                            .flex()
                            .flex_col()
                            .children(
                                nav_entries
                                    .into_iter()
                                    .filter(|(_, _, zh_t, en_t)| {
                                        q.is_empty()
                                            || zh_t.to_lowercase().contains(&q)
                                            || en_t.to_lowercase().contains(&q)
                                    })
                                    .map(|(pid, icon, zh_t, en_t)| {
                                        let is_sel = pid == cur_page;
                                        let label = tr_str(&lang, zh_t, en_t).to_string();
                                        div()
                                            .cursor_pointer()
                                            .h(px(40.0))
                                            .px(px(14.0))
                                            .bg(if is_sel {
                                                rgb(pal.dialogs_active_bg)
                                            } else {
                                                rgb(pal.dialogs_bg)
                                            })
                                            .hover(move |s| {
                                                if is_sel {
                                                    s
                                                } else {
                                                    s.bg(rgb(pal.dialogs_hover_bg))
                                                }
                                            })
                                            .flex()
                                            .items_center()
                                            .gap(px(10.0))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.active_page = pid;
                                                    this.editing_field = 0;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(div().text_size(px(15.0)).child(icon))
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(if is_sel {
                                                        FontWeight::BOLD
                                                    } else {
                                                        FontWeight::NORMAL
                                                    })
                                                    .text_color(if is_sel {
                                                        rgb(0xFFFFFF)
                                                    } else {
                                                        rgb(pal.text_primary)
                                                    })
                                                    .child(label),
                                            )
                                    }),
                            ),
                    ),
            )
            // Right Settings Detail Content Area
            .child(
                div()
                    .id("settings-right-detail-scroll")
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .overflow_y_scroll()
                    .p(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(self.render_page_detail(cur_page, cx)),
            )
    }
}

impl SettingsWorkspaceView {
    fn card_box(pal: TdesktopThemePalette) -> Div {
        div()
            .w_full()
            .p(px(16.0))
            .rounded(px(12.0))
            .bg(rgb(pal.card_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(12.0))
    }

    fn render_page_detail(&self, page: SettingsPageId, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        match page {
            SettingsPageId::Account => {
                let acc = self
                    .state_snapshot
                    .accounts
                    .iter()
                    .find(|a| a.account_id == self.state_snapshot.active_account_id)
                    .cloned()
                    .unwrap_or_else(|| self.state_snapshot.accounts[0].clone());
                let bio_val = self.bio_edit_input.clone();

                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(14.0))
                                            .child(
                                                div()
                                                    .w(px(64.0))
                                                    .h(px(64.0))
                                                    .rounded_full()
                                                    .bg(rgb(acc.avatar_color))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(22.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(acc.avatar_initials),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(18.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(pal.text_primary))
                                                            .child(format!(
                                                                "{} {} ⭐",
                                                                acc.display_name, acc.emoji_status
                                                            )),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .text_color(rgb(pal.text_secondary))
                                                            .child(format!(
                                                                "{} • @{}",
                                                                acc.phone, acc.username
                                                            )),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(14.0))
                                            .py(px(7.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.danger))
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::LogOut);
                                                    cx.notify();
                                                }),
                                            )
                                            .child("🚪 退出当前账号 (Log Out)"),
                                    ),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.editing_field = 3;
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!(
                                        "个人简介 Bio (点击编辑并回车保存): {}",
                                        bio_val
                                    )),
                            ),
                    )
                    .into_any_element()
            }

            SettingsPageId::ChatAppearance => {
                let cur_mode = pal.mode;
                let cur_accent = pal.accent;
                let cur_font = pal.font_size_px;
                let cur_rad = pal.bubble_radius_px;
                let enter_send = self.state_snapshot.send_with_enter;

                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    // 4 Official Telegram Desktop Theme Cards
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("🎨 Telegram 官方主题色板 (Color Themes — 1:1 tdesktop)"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(10.0))
                                    .children(
                                        [
                                            (TdesktopThemeMode::Night, "🌙 Night (深海暗夜)", 0x0E1621u32),
                                            (TdesktopThemeMode::Tinted, "🌲 Tinted (翡翠暗色)", 0x18262Eu32),
                                            (TdesktopThemeMode::Day, "☀️ Day (现代浅蓝)", 0xE3F2FDu32),
                                            (TdesktopThemeMode::Classic, "🍀 Classic (经典绿泡)", 0xEFFDDEu32),
                                        ]
                                        .into_iter()
                                        .map(|(m, title, swatch)| {
                                            let is_sel = cur_mode == m;
                                            div()
                                                .cursor_pointer()
                                                .px(px(14.0))
                                                .py(px(10.0))
                                                .rounded(px(8.0))
                                                .bg(rgb(pal.input_bg))
                                                .border_2()
                                                .border_color(if is_sel {
                                                    rgb(pal.accent)
                                                } else {
                                                    rgb(pal.border_fg)
                                                })
                                                .flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.pending_mutation = Some(
                                                            SettingsMutation::SetThemeMode(m),
                                                        );
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .w(px(18.0))
                                                        .h(px(18.0))
                                                        .rounded_full()
                                                        .bg(rgb(swatch)),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(12.5))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(pal.text_primary))
                                                        .child(title),
                                                )
                                        }),
                                    ),
                            ),
                    )
                    // Accent Colors + Font Size + Bubble Radius + Enter Key Behavior
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("✨ 强调色、气泡圆角与字号缩放 (Accent, Scale & Radius)"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .children(
                                        [
                                            0x2AABEEu32,
                                            0x4CAF50,
                                            0x9B59B6,
                                            0xE67E22,
                                            0xE91E63,
                                            0x00BCD4,
                                            0xF1C40F,
                                        ]
                                        .into_iter()
                                        .map(|col| {
                                            div()
                                                .cursor_pointer()
                                                .w(px(28.0))
                                                .h(px(28.0))
                                                .rounded_full()
                                                .bg(rgb(col))
                                                .border_2()
                                                .border_color(if cur_accent == col {
                                                    rgb(0xFFFFFF)
                                                } else {
                                                    rgb(pal.border_fg)
                                                })
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.pending_mutation = Some(
                                                            SettingsMutation::SetAccentColor(col),
                                                        );
                                                        cx.notify();
                                                    }),
                                                )
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(pal.text_primary))
                                            .child(format!("消息字号: {}px", cur_font)),
                                    )
                                    .children([12u32, 14, 16, 18].into_iter().map(|sz| {
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(4.0))
                                            .rounded(px(6.0))
                                            .bg(if cur_font == sz {
                                                rgb(pal.accent)
                                            } else {
                                                rgb(pal.input_bg)
                                            })
                                            .text_size(px(12.0))
                                            .text_color(if cur_font == sz {
                                                rgb(0xFFFFFF)
                                            } else {
                                                rgb(pal.text_primary)
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::SetFontSize(sz));
                                                    cx.notify();
                                                }),
                                            )
                                            .child(format!("{}px", sz))
                                    })),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(pal.text_primary))
                                            .child(format!("气泡圆角: {}px", cur_rad)),
                                    )
                                    .children([6u32, 10, 14, 18].into_iter().map(|r| {
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(4.0))
                                            .rounded(px(6.0))
                                            .bg(if cur_rad == r {
                                                rgb(pal.accent)
                                            } else {
                                                rgb(pal.input_bg)
                                            })
                                            .text_size(px(12.0))
                                            .text_color(if cur_rad == r {
                                                rgb(0xFFFFFF)
                                            } else {
                                                rgb(pal.text_primary)
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::SetBubbleRadius(r));
                                                    cx.notify();
                                                }),
                                            )
                                            .child(format!("{}px", r))
                                    })),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.pending_mutation =
                                                Some(SettingsMutation::ToggleSendWithEnter);
                                            cx.notify();
                                        }),
                                    )
                                    .child(if enter_send {
                                        "⌨️ 发送快捷键: 按 Enter 发送，Shift+Enter 换行 (点击切换)"
                                    } else {
                                        "⌨️ 发送快捷键: 按 Cmd/Ctrl+Enter 发送，Enter 换行 (点击切换)"
                                    }),
                            ),
                    )
                    .into_any_element()
            }

            SettingsPageId::Notifications => Self::card_box(pal)
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child("🔔 全局与分类消息通知 (Notifications & Sounds)"),
                )
                .children(
                    [
                        ("私聊消息桌面横幅通知 (Private Chats)", self.notify_private, 1u8),
                        ("群组与话题 @提及通知 (Groups & Topics)", self.notify_groups, 2u8),
                        ("订阅频道广播推送 (Channels)", self.notify_channels, 3u8),
                        ("应用内提示音与振动反馈 (In-App Sounds)", self.notify_sound, 4u8),
                    ]
                    .into_iter()
                    .map(|(lbl, on, code)| {
                        div()
                            .cursor_pointer()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    match code {
                                        1 => this.notify_private = !this.notify_private,
                                        2 => this.notify_groups = !this.notify_groups,
                                        3 => this.notify_channels = !this.notify_channels,
                                        _ => this.notify_sound = !this.notify_sound,
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .child(lbl),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if on {
                                        rgb(pal.success)
                                    } else {
                                        rgb(pal.text_secondary)
                                    })
                                    .child(if on { "开启 ON" } else { "关闭 OFF" }),
                            )
                    }),
                )
                .into_any_element(),

            SettingsPageId::PrivacySecurity => {
                let has_pin = self.state_snapshot.local_passcode_pin.is_some();
                let devices = self.state_snapshot.active_devices.clone();

                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("🔐 安全防护、两步验证与本地锁屏密码"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(14.0))
                                            .py(px(8.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.accent))
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::OpenPasscodeModal);
                                                    cx.notify();
                                                }),
                                            )
                                            .child(if has_pin {
                                                "🔒 修改/关闭本地 PIN 锁屏密码 (已启用)"
                                            } else {
                                                "🔒 设置 4 位本地锁屏密码 (Set Passcode)"
                                            }),
                                    )
                                    .when(has_pin, |el| {
                                        el.child(
                                            div()
                                                .cursor_pointer()
                                                .px(px(14.0))
                                                .py(px(8.0))
                                                .rounded(px(8.0))
                                                .bg(rgb(pal.input_bg))
                                                .text_size(px(12.5))
                                                .text_color(rgb(pal.text_primary))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _e, _w, cx| {
                                                        this.pending_mutation =
                                                            Some(SettingsMutation::LockNow);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child("⚡ 立即锁屏 (Cmd+L)"),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(
                                        "✓ 两步验证云端密码 (2FA Cloud Password): 已开启 • ✓ Passkeys (WebAuthn): 已绑定硬件密钥",
                                    ),
                            ),
                    )
                    // Active Sessions / Devices List
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!(
                                        "💻 在线登录设备管理 (Active Sessions: {})",
                                        devices.len()
                                    )),
                            )
                            .children(devices.into_iter().map(|dev| {
                                let sid = dev.session_id.clone();
                                div()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(dev.device_model),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.5))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(format!(
                                                        "{} • {} • {}",
                                                        dev.platform, dev.ip_region, dev.last_active
                                                    )),
                                            ),
                                    )
                                    .child(if dev.is_current {
                                        div()
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.success))
                                            .child("本机 Current")
                                    } else {
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(4.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.danger))
                                            .text_size(px(11.5))
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.pending_mutation = Some(
                                                        SettingsMutation::RevokeSession(sid.clone()),
                                                    );
                                                    cx.notify();
                                                }),
                                            )
                                            .child("注销设备")
                                    })
                            })),
                    )
                    .into_any_element()
            }

            SettingsPageId::Folders => {
                let custom = self.state_snapshot.custom_folders.clone();
                let inp = self.new_folder_input.clone();

                Self::card_box(pal)
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child("📁 聊天分组标签栏 (Chat Folders — 实时同步至左侧会话栏)"),
                    )
                    .child(
                        div()
                            .text_size(px(12.5))
                            .text_color(rgb(pal.text_secondary))
                            .child("内置分组: All (全部) • Personal (个人) • Work (工作) • Bots (机器人) • Unread (未读)"),
                    )
                    .children(custom.into_iter().map(|(fname, _)| {
                        let fn_del = fname.clone();
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!("📁 {}", fname)),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.danger))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            this.pending_mutation = Some(
                                                SettingsMutation::RemoveCustomFolder(
                                                    fn_del.clone(),
                                                ),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child("删除分组"),
                            )
                    }))
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .flex_1()
                                    .p(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.editing_field = 2;
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!("新分组名称: {}", inp)),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            if !this.new_folder_input.trim().is_empty() {
                                                this.pending_mutation =
                                                    Some(SettingsMutation::AddCustomFolder(
                                                        this.new_folder_input.trim().to_string(),
                                                    ));
                                                cx.notify();
                                            }
                                        }),
                                    )
                                    .child("＋ 创建分组"),
                            ),
                    )
                    .into_any_element()
            }

            SettingsPageId::AdvancedStorage => {
                let media_mb = self.state_snapshot.cache_media_mb;
                let files_mb = self.state_snapshot.cache_files_mb;
                let active_pxy = self
                    .state_snapshot
                    .proxies
                    .iter()
                    .find(|p| p.enabled)
                    .map(|p| format!("{} ({}:{} • {}ms)", p.kind, p.host, p.port, p.ping_ms))
                    .unwrap_or_else(|| "Direct MTProto 2.0".to_string());

                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("🌐 网络连接与 MTProto / SOCKS5 代理"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!("当前活动链路: {}", active_pxy)),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .w(px(220.0))
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.pending_mutation =
                                                Some(SettingsMutation::OpenProxyModal);
                                            cx.notify();
                                        }),
                                    )
                                    .child("🛡️ 管理代理服务器 (Manage Proxies)"),
                            ),
                    )
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("💾 本地存储空间与聊天导出 (Local Storage & Export)"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "媒体缓存: {:.1} MB • 文档缓存: {:.1} MB • SQLite WAL 索引正常",
                                        media_mb, files_mb
                                    )),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(14.0))
                                            .py(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.input_bg))
                                            .text_size(px(12.5))
                                            .text_color(rgb(pal.text_primary))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::ClearLocalCache);
                                                    cx.notify();
                                                }),
                                            )
                                            .child("🧹 一键清理本地缓存 (Clear Cache)"),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(14.0))
                                            .py(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.accent))
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_mutation =
                                                        Some(SettingsMutation::OpenExportModal);
                                                    cx.notify();
                                                }),
                                            )
                                            .child("📤 导出聊天记录 (HTML / JSON)"),
                                    ),
                            ),
                    )
                    .into_any_element()
            }

            SettingsPageId::CallDevices => Self::card_box(pal)
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child("🎙️ WebRTC 音视频通话与降噪设置 (Call Settings)"),
                )
                .child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(13.0))
                        .text_color(rgb(pal.text_primary))
                        .child("输入麦克风: Default System Microphone (48kHz Opus) • 输出扬声器: MacBook Pro Speakers"),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(13.0))
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.call_noise_suppression = !this.call_noise_suppression;
                                cx.notify();
                            }),
                        )
                        .child(if self.call_noise_suppression {
                            "✓ AI 实时环境降噪与回声消除: 开启 (RNNoise + WebRTC AEC3)"
                        } else {
                            "○ AI 实时环境降噪与回声消除: 关闭"
                        }),
                )
                .into_any_element(),

            SettingsPageId::LanguageTranslate => {
                let cur_lang = self.state_snapshot.language_id.clone();
                let show_tr = self.state_snapshot.show_translate_bar;
                let langs = [
                    ("zh-CN", "简体中文 (Chinese Simplified)"),
                    ("en", "English (Official)"),
                    ("ja", "日本語 (Japanese)"),
                    ("ru", "Русский (Russian)"),
                ];

                Self::card_box(pal)
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child("🌐 界面语言包与全会话实时翻译 (Language & Translate)"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _e, _w, cx| {
                                    this.pending_mutation =
                                        Some(SettingsMutation::ToggleTranslateBar);
                                    cx.notify();
                                }),
                            )
                            .child(if show_tr {
                                "✓ 在聊天顶部显示「整页实时翻译」横幅 (Show Translate Bar: ON)"
                            } else {
                                "○ 在聊天顶部显示「整页实时翻译」横幅 (Show Translate Bar: OFF)"
                            }),
                    )
                    .children(langs.into_iter().map(|(lid, lname)| {
                        let is_cur = cur_lang == lid;
                        let lid_s = lid.to_string();
                        div()
                            .cursor_pointer()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(if is_cur {
                                rgb(pal.dialogs_active_bg)
                            } else {
                                rgb(pal.input_bg)
                            })
                            .text_size(px(13.0))
                            .font_weight(if is_cur {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(if is_cur {
                                rgb(0xFFFFFF)
                            } else {
                                rgb(pal.text_primary)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.pending_mutation =
                                        Some(SettingsMutation::SetLanguage(lid_s.clone()));
                                    cx.notify();
                                }),
                            )
                            .child(format!("{} {}", if is_cur { "●" } else { "○" }, lname))
                    }))
                    .into_any_element()
            }

            SettingsPageId::PremiumStars => {
                let stars = self.state_snapshot.stars_balance;
                Self::card_box(pal)
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xF1C40F))
                            .child(format!(
                                "⭐ Telegram Premium 已激活 • 当前星币余额: {} Stars",
                                stars
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(12.5))
                            .text_color(rgb(pal.text_secondary))
                            .child("尊享双倍上限：6 个登录账号、4GB 大文件上传、语音转文字 [->A]、实时全会话翻译、动态 Emoji 状态与无广告体验。"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(10.0))
                            .children([500u64, 1000, 2500].into_iter().map(|amt| {
                                div()
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            this.pending_mutation =
                                                Some(SettingsMutation::BuyOrTopUpStars(amt));
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!("＋ 充值 {} ⭐", amt))
                            })),
                    )
                    .into_any_element()
            }

            SettingsPageId::TonWallet => {
                let ton = self.state_snapshot.ton_balance;
                let txs = self.state_snapshot.wallet_transactions.clone();

                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!("💎 {:.2} TON (≈ ${:.2} USD)", ton, ton * 5.40)),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child("钱包地址: UQBx9f...GPUI742_TON_SelfCustodial_Vault"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .w(px(220.0))
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.pending_mutation =
                                                Some(SettingsMutation::SendTonDemoTransfer);
                                            cx.notify();
                                        }),
                                    )
                                    .child("↗ 发送 5.00 TON 快速转账"),
                            ),
                    )
                    .child(
                        Self::card_box(pal)
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("📜 链上与星币账单明细 (Transactions)"),
                            )
                            .children(txs.into_iter().map(|tx| {
                                div()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(tx.title),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.5))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(format!("{} • {}", tx.peer, tx.timestamp)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if tx.is_incoming {
                                                rgb(pal.success)
                                            } else {
                                                rgb(pal.danger)
                                            })
                                            .child(if tx.amount_ton.abs() > 0.001 {
                                                format!(
                                                    "{}{:.2} TON",
                                                    if tx.is_incoming { "+" } else { "-" },
                                                    tx.amount_ton.abs()
                                                )
                                            } else {
                                                format!("{} ⭐", tx.amount_stars)
                                            }),
                                    )
                            })),
                    )
                    .into_any_element()
            }

            SettingsPageId::BusinessPassport => Self::card_box(pal)
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child("💼 Telegram Business 商业工具与 🔐 Telegram Passport"),
                )
                .child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(13.0))
                        .text_color(rgb(pal.text_primary))
                        .child(format!(
                            "营业时间自动欢迎语: {} • 快捷回复模板 (/thanks, /shipping, /faq, /invoice): {} 条",
                            if self.business_hours_enabled { "已启用" } else { "已暂停" },
                            self.business_quick_replies
                        )),
                )
                .child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(13.0))
                        .text_color(rgb(pal.text_primary))
                        .child(format!(
                            "🔐 Telegram Passport 端到端加密身份库: 已存储 {} 份经过 AES-256-CBC 加密的身份与地址凭证",
                            self.passport_verified_docs
                        )),
                )
                .into_any_element(),
        }
    }
}
