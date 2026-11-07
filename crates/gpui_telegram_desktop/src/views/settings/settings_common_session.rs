//! Upstream Source: Telegram/SourceFiles/settings/settings_common_session.cpp, Telegram/SourceFiles/settings/settings_common_session.h
//! Subsystem: settings | Feature ID: TCG-N02 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::settings_type::SettingsSectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsContainerKind {
    Section,
    Layer,
}

#[derive(Debug, Clone)]
pub struct SettingsCommonSession {
    pub account_id: u64,
    pub title: String,
    pub container_kind: SettingsContainerKind,
    pub current_section: SettingsSectionType,
    pub is_visible: bool,
}

impl SettingsCommonSession {
    pub fn new(account_id: u64) -> Self {
        Self::with_account(account_id)
    }

    pub fn with_account(account_id: u64) -> Self {
        Self {
            account_id,
            title: format!("Settings Session (Account {})", account_id),
            container_kind: SettingsContainerKind::Section,
            current_section: SettingsSectionType::Main,
            is_visible: true,
        }
    }

    pub fn has_menu(&self, section_name: &str) -> bool {
        matches!(
            section_name,
            "chat" | "information" | "local_storage" | "websites" | "passkeys"
        )
    }

    pub fn has_menu_for_section(section: SettingsSectionType) -> bool {
        matches!(
            section,
            SettingsSectionType::Information
                | SettingsSectionType::LocalStorage
                | SettingsSectionType::Websites
                | SettingsSectionType::Passkeys
        )
    }

    pub fn set_container_kind(&mut self, kind: SettingsContainerKind) {
        self.container_kind = kind;
    }

    pub fn set_section(&mut self, section: SettingsSectionType) {
        self.current_section = section;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsCommonSession {
    fn default() -> Self {
        Self::with_account(0)
    }
}

impl Render for SettingsCommonSession {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let has_menu = Self::has_menu_for_section(self.current_section);

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(16.0))
            .py(px(10.0))
            .bg(rgb(0x17212b))
            .border_b_1()
            .border_color(rgb(0x0e1621))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.current_section.display_title()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .px(px(6.0))
                            .py(px(1.0))
                            .rounded(px(3.0))
                            .bg(rgb(0x232e3c))
                            .text_color(rgb(0x6c7883))
                            .child(format!("{:?}", self.container_kind)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .when(has_menu, |this| {
                        this.child(
                            div()
                                .size(px(28.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(rgb(0x232e3c)))
                                .cursor_pointer()
                                .text_color(rgb(0xffffff))
                                .child("⋮"),
                        )
                    }),
            )
            .into_any_element()
    }
}
