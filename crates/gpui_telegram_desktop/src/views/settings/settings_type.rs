//! Upstream Source: Telegram/SourceFiles/settings/settings_type.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use std::fmt;

/// Section type enumeration identifying every distinct Settings destination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsSectionType {
    Main,
    Information,
    Notifications,
    PrivacySecurity,
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
    PowerSaving,
    LocalStorage,
    Business,
    Search,
}

impl SettingsSectionType {
    pub fn id_str(&self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Information => "information",
            Self::Notifications => "notifications",
            Self::PrivacySecurity => "privacy",
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
            Self::PowerSaving => "power_saving",
            Self::LocalStorage => "local_storage",
            Self::Business => "business",
            Self::Search => "search",
        }
    }

    pub fn display_title(&self) -> &'static str {
        match self {
            Self::Main => "Settings",
            Self::Information => "Edit Profile",
            Self::Notifications => "Notifications and Sounds",
            Self::PrivacySecurity => "Privacy and Security",
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
            Self::PowerSaving => "Power Saving",
            Self::LocalStorage => "Data and Storage",
            Self::Business => "Telegram Business",
            Self::Search => "Search Settings",
        }
    }

    pub fn is_root(&self) -> bool {
        matches!(self, Self::Main)
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
