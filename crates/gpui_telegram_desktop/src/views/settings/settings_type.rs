//! Upstream Source: Telegram/SourceFiles/settings/settings_type.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Section type enumeration identifying every distinct Settings destination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettingsSectionType {
    Main,
    Information,
    Notifications,
    NotificationsType,
    NotificationsReactions,
    PrivacySecurity,
    BlockedPeers,
    ActiveSessions,
    Chat,
    Folders,
    Advanced,
    Calls,
    Credits,
    Premium,
    Shortcuts,
    Websites,
    Passkeys,
    LocalPasscode,
    GlobalTtl,
    PowerSaving,
    LocalStorage,
    Business,
    Language,
    Search,
}

impl SettingsSectionType {
    pub fn id_str(&self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Information => "information",
            Self::Notifications => "notifications",
            Self::NotificationsType => "notifications_type",
            Self::NotificationsReactions => "notifications_reactions",
            Self::PrivacySecurity => "privacy",
            Self::BlockedPeers => "blocked_peers",
            Self::ActiveSessions => "active_sessions",
            Self::Chat => "chat",
            Self::Folders => "folders",
            Self::Advanced => "advanced",
            Self::Calls => "calls",
            Self::Credits => "credits",
            Self::Premium => "premium",
            Self::Shortcuts => "shortcuts",
            Self::Websites => "websites",
            Self::Passkeys => "passkeys",
            Self::LocalPasscode => "local_passcode",
            Self::GlobalTtl => "global_ttl",
            Self::PowerSaving => "power_saving",
            Self::LocalStorage => "local_storage",
            Self::Business => "business",
            Self::Language => "language",
            Self::Search => "search",
        }
    }

    pub fn display_title(&self) -> &'static str {
        self.display_title_localized(false)
    }

    pub fn display_title_localized(&self, is_chinese: bool) -> &'static str {
        if is_chinese {
            match self {
                Self::Main => "设置",
                Self::Information => "编辑个人资料",
                Self::Notifications => "通知与声音",
                Self::NotificationsType => "通知分类设置",
                Self::NotificationsReactions => "互动反应通知",
                Self::PrivacySecurity => "隐私与安全",
                Self::BlockedPeers => "已屏蔽用户",
                Self::ActiveSessions => "活跃会话与设备",
                Self::Chat => "对话设置",
                Self::Folders => "对话分组",
                Self::Advanced => "高级设置",
                Self::Calls => "通话与音频",
                Self::Credits => "Telegram 星标",
                Self::Premium => "Telegram 会员",
                Self::Shortcuts => "键盘快捷键",
                Self::Websites => "已授权网站",
                Self::Passkeys => "密钥与两步验证",
                Self::LocalPasscode => "应用锁",
                Self::GlobalTtl => "账号注销与消息销毁",
                Self::PowerSaving => "省电模式",
                Self::LocalStorage => "数据与存储",
                Self::Business => "商业功能",
                Self::Language => "界面语言",
                Self::Search => "搜索设置",
            }
        } else {
            match self {
                Self::Main => "Settings",
                Self::Information => "Edit Profile",
                Self::Notifications => "Notifications and Sounds",
                Self::NotificationsType => "Notification Category",
                Self::NotificationsReactions => "Reactions Notifications",
                Self::PrivacySecurity => "Privacy and Security",
                Self::BlockedPeers => "Blocked Users",
                Self::ActiveSessions => "Devices & Active Sessions",
                Self::Chat => "Chat Settings",
                Self::Folders => "Chat Folders",
                Self::Advanced => "Advanced",
                Self::Calls => "Calls",
                Self::Credits => "Telegram Stars",
                Self::Premium => "Telegram Premium",
                Self::Shortcuts => "Keyboard Shortcuts",
                Self::Websites => "Logged in Websites",
                Self::Passkeys => "Passkeys",
                Self::LocalPasscode => "Passcode Lock",
                Self::GlobalTtl => "Auto-Delete Messages",
                Self::PowerSaving => "Power Saving",
                Self::LocalStorage => "Data and Storage",
                Self::Business => "Telegram Business",
                Self::Language => "Language",
                Self::Search => "Search Settings",
            }
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Main => "⚙️",
            Self::Information => "👤",
            Self::Notifications | Self::NotificationsType | Self::NotificationsReactions => "🔔",
            Self::PrivacySecurity | Self::LocalPasscode | Self::Passkeys => "🔒",
            Self::BlockedPeers => "🚫",
            Self::ActiveSessions => "💻",
            Self::GlobalTtl => "⏳",
            Self::Chat => "💬",
            Self::Folders => "📁",
            Self::Advanced => "🛠️",
            Self::LocalStorage => "💾",
            Self::PowerSaving => "🔋",
            Self::Calls => "📞",
            Self::Credits => "⭐",
            Self::Premium => "👑",
            Self::Shortcuts => "⌨️",
            Self::Websites => "🌐",
            Self::Business => "💼",
            Self::Language => "🗣️",
            Self::Search => "🔍",
        }
    }

    pub fn is_root(&self) -> bool {
        matches!(self, Self::Main)
    }

    pub fn parent_section(&self) -> Option<Self> {
        match self {
            Self::Main => None,
            Self::BlockedPeers
            | Self::ActiveSessions
            | Self::LocalPasscode
            | Self::Passkeys
            | Self::GlobalTtl
            | Self::Websites => Some(Self::PrivacySecurity),
            Self::NotificationsType | Self::NotificationsReactions => Some(Self::Notifications),
            Self::LocalStorage | Self::PowerSaving => Some(Self::Advanced),
            _ => Some(Self::Main),
        }
    }

    pub fn breadcrumb_path(&self) -> Vec<Self> {
        let mut path = Vec::new();
        let mut curr = Some(*self);
        while let Some(sec) = curr {
            path.push(sec);
            curr = sec.parent_section();
        }
        path.reverse();
        path
    }
}

impl fmt::Display for SettingsSectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_title())
    }
}

/// SettingsType view displaying section identity and navigation context
#[derive(Debug, Clone)]
pub struct SettingsType {
    pub current_section: SettingsSectionType,
    pub title: String,
    pub is_visible: bool,
}

impl SettingsType {
    pub fn new() -> Self {
        Self {
            current_section: SettingsSectionType::Main,
            title: SettingsSectionType::Main.display_title().into(),
            is_visible: true,
        }
    }

    pub fn for_section(section: SettingsSectionType) -> Self {
        Self {
            current_section: section,
            title: section.display_title().into(),
            is_visible: true,
        }
    }

    pub fn set_section(&mut self, section: SettingsSectionType) {
        self.current_section = section;
        self.title = section.display_title().into();
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsType {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsType {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(16.0))
            .py(px(8.0))
            .bg(rgb(0x18222d))
            .border_b_1()
            .border_color(rgb(0x0f161e))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x2b3846))
                            .text_color(rgb(0x8292a1))
                            .child(format!("#{}", self.current_section.id_str())),
                    ),
            )
            .into_any_element()
    }
}
