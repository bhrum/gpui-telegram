//! Upstream Source: Telegram/SourceFiles/settings/business/settings_greeting.cpp, Telegram/SourceFiles/settings/business/settings_greeting.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_DEFAULT_NO_ACTIVITY_DAYS: u32 = 7;
pub const K_PERIOD_OPTIONS: [u32; 4] = [7, 14, 21, 28];

#[derive(Debug, Clone)]
pub struct SettingsGreetingBusiness {
    pub enabled: bool,
    pub no_activity_days: u32,
    pub all_but_excluded: bool,
    pub excluded_peers_count: usize,
    pub included_peers_count: usize,
    pub shortcut_name: String,
    pub shortcut_messages_count: usize,
    pub status_message: Option<String>,
}

impl SettingsGreetingBusiness {
    pub fn new() -> Self {
        Self {
            enabled: false,
            no_activity_days: K_DEFAULT_NO_ACTIVITY_DAYS,
            all_but_excluded: true,
            excluded_peers_count: 0,
            included_peers_count: 0,
            shortcut_name: "hello".into(),
            shortcut_messages_count: 1,
            status_message: None,
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_no_activity_days(&mut self, days: u32) {
        if K_PERIOD_OPTIONS.contains(&days) {
            self.no_activity_days = days;
        }
    }

    pub fn save(&mut self) -> Result<(), &'static str> {
        if self.enabled && !self.all_but_excluded && self.included_peers_count == 0 {
            self.status_message = Some("BUSINESS_RECIPIENTS_EMPTY".into());
            return Err("BUSINESS_RECIPIENTS_EMPTY");
        }
        self.status_message = Some("Greeting settings saved".into());
        Ok(())
    }

    pub fn quitting(&mut self) {
        let _ = self.save();
    }
}

impl Default for SettingsGreetingBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsGreetingBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enabled = self.enabled;
        let days = self.no_activity_days;
        let messages_count = self.shortcut_messages_count;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header Info
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Greeting Messages"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Automatically greet new customers and inactive chats.")),
            )
            // Enable toggle
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                        this.toggle_enabled();
                        cx.notify();
                    }))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Send Greeting Messages"))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(if enabled { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if enabled { "ON" } else { "OFF" })),
            )
            // Details when enabled
            .when(enabled, |container| {
                container
                    // Edit Shortcut button
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child(format!("Edit Messages (/{})", self.shortcut_name)))
                            .child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("{} messages", messages_count))),
                    )
                    // Inactivity threshold selector
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x8292a1)).child("SEND AGAIN AFTER INACTIVITY"))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(8.0))
                                    .children(K_PERIOD_OPTIONS.iter().map(|&opt| {
                                        let is_selected = days == opt;
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .bg(if is_selected { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .text_xs()
                                            .text_color(rgb(0xffffff))
                                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                                this.set_no_activity_days(opt);
                                                cx.notify();
                                            }))
                                            .child(format!("{} days", opt))
                                    })),
                            ),
                    )
                    // Recipients description
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child("Recipients"))
                            .child(div().text_xs().text_color(rgb(0x5288c1)).child("All chats except excluded")),
                    )
            })
            .into_any_element()
    }
}
