//! Upstream Source: Telegram/SourceFiles/settings/detailed_settings_button.cpp, Telegram/SourceFiles/settings/detailed_settings_button.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct DetailedSettingsButton {
    pub title: String,
    pub description: String,
    pub icon_name: Option<String>,
    pub is_toggled: bool,
    pub is_locked: bool,
    pub badge: Option<String>,
    pub is_visible: bool,
}

impl DetailedSettingsButton {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            icon_name: None,
            is_toggled: false,
            is_locked: false,
            badge: None,
            is_visible: true,
        }
    }

    pub fn with_icon(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn with_toggle(mut self, toggled: bool) -> Self {
        self.is_toggled = toggled;
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn toggle(&mut self) -> bool {
        if !self.is_locked {
            self.is_toggled = !self.is_toggled;
        }
        self.is_toggled
    }

    pub fn set_toggle_locked(&mut self, locked: bool) {
        self.is_locked = locked;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.is_locked = locked;
    }

    pub fn set_badge(&mut self, badge: impl Into<String>) {
        self.badge = Some(badge.into());
    }

    pub fn toggled(&self) -> bool {
        self.is_toggled
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn accessibility_name(&self) -> String {
        format!("{}: {}", self.title, if self.is_toggled { "On" } else { "Off" })
    }

    pub fn accessibility_state(&self) -> &'static str {
        if self.is_locked {
            "disabled"
        } else if self.is_toggled {
            "checked"
        } else {
            "unchecked"
        }
    }
}

impl Default for DetailedSettingsButton {
    fn default() -> Self {
        Self::new("Detailed Setting", "Setting description details")
    }
}

impl Render for DetailedSettingsButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let is_toggled = self.is_toggled;
        let is_locked = self.is_locked;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(16.0))
            .py(px(12.0))
            .rounded(px(8.0))
            .bg(rgb(0x17212b))
            .hover(|s| s.bg(rgb(0x232e3c)))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                this.toggle();
                cx.notify();
            }))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .when_some(self.icon_name.clone(), |this, icon| {
                        this.child(
                            div()
                                .size(px(24.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(6.0))
                                .bg(rgb(0x2b3846))
                                .text_color(rgb(0x5288c1))
                                .text_sm()
                                .child(icon),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xffffff))
                                    .child(self.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8292a1))
                                    .child(self.description.clone()),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .when_some(self.badge.clone(), |this, b| {
                        this.child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(10.0))
                                .bg(rgb(0x5288c1))
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .child(b),
                        )
                    })
                    .child(
                        // Animated toggle switch track and thumb
                        div()
                            .w(px(38.0))
                            .h(px(22.0))
                            .rounded(px(11.0))
                            .bg(if is_toggled { rgb(0x5288c1) } else { rgb(0x2b3846) })
                            .opacity(if is_locked { 0.5 } else { 1.0 })
                            .p(px(2.0))
                            .child(
                                div()
                                    .size(px(18.0))
                                    .rounded(px(9.0))
                                    .bg(rgb(0xffffff))
                                    .ml(if is_toggled { px(16.0) } else { px(0.0) }),
                            ),
                    ),
            )
            .into_any_element()
    }
}
