//! Upstream Source: Telegram/SourceFiles/settings/business/settings_away_message.cpp, Telegram/SourceFiles/settings/business/settings_away_message.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_START_TIME_MIN: i64 = 1375315200; // Telegram launch: 2013-08-01 00:00:00 UTC
pub const K_END_TIME_MIN: i64 = K_START_TIME_MIN + 3600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwayScheduleType {
    Always,
    OutsideWorkingHours,
    Custom,
}

#[derive(Debug, Clone)]
pub struct WorkingInterval {
    pub start: i64,
    pub end: i64,
}

impl WorkingInterval {
    pub fn is_valid(&self) -> bool {
        self.start >= K_START_TIME_MIN && self.end >= K_END_TIME_MIN && self.start < self.end
    }
}

#[derive(Debug, Clone)]
pub struct SettingsAwayMessageBusiness {
    pub enabled: bool,
    pub schedule_type: AwayScheduleType,
    pub custom_interval: WorkingInterval,
    pub offline_only: bool,
    pub all_but_excluded: bool,
    pub excluded_peers_count: usize,
    pub included_peers_count: usize,
    pub shortcut_name: String,
    pub shortcut_messages_count: usize,
    pub status_message: Option<String>,
}

impl SettingsAwayMessageBusiness {
    pub fn new() -> Self {
        let now = 1700000000;
        Self {
            enabled: false,
            schedule_type: AwayScheduleType::Always,
            custom_interval: WorkingInterval {
                start: now,
                end: now + 86400,
            },
            offline_only: false,
            all_but_excluded: true,
            excluded_peers_count: 0,
            included_peers_count: 0,
            shortcut_name: "away".into(),
            shortcut_messages_count: 1,
            status_message: None,
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_schedule_type(&mut self, schedule: AwayScheduleType) {
        self.schedule_type = schedule;
    }

    pub fn set_custom_interval(&mut self, start: i64, end: i64) -> Result<(), &'static str> {
        let interval = WorkingInterval { start, end };
        if !interval.is_valid() {
            return Err("BAD_CUSTOM_INTERVAL");
        }
        self.custom_interval = interval;
        Ok(())
    }

    pub fn toggle_offline_only(&mut self) {
        self.offline_only = !self.offline_only;
    }

    pub fn save(&mut self) -> Result<(), &'static str> {
        if self.enabled && !self.all_but_excluded && self.included_peers_count == 0 {
            self.status_message = Some("BUSINESS_RECIPIENTS_EMPTY".into());
            return Err("BUSINESS_RECIPIENTS_EMPTY");
        }
        self.status_message = Some("Saved".into());
        Ok(())
    }

    pub fn quitting(&mut self) {
        let _ = self.save();
    }
}

impl Default for SettingsAwayMessageBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsAwayMessageBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enabled = self.enabled;
        let schedule_type = self.schedule_type;
        let offline_only = self.offline_only;
        let messages_count = self.shortcut_messages_count;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header / Intro
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Away Messages"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Automatically send messages when you are offline or away.")),
            )
            // Enable toggle button
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
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Send Away Messages"))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(if enabled { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if enabled { "ON" } else { "OFF" })),
            )
            // Settings wrap when enabled
            .when(enabled, |container| {
                container
                    // Edit Shortcut messages button
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
                    // Schedule selector
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x8292a1)).child("SCHEDULE"))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .bg(if schedule_type == AwayScheduleType::Always { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .text_xs()
                                            .text_color(rgb(0xffffff))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_schedule_type(AwayScheduleType::Always);
                                                cx.notify();
                                            }))
                                            .child("Always"),
                                    )
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .bg(if schedule_type == AwayScheduleType::OutsideWorkingHours { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .text_xs()
                                            .text_color(rgb(0xffffff))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_schedule_type(AwayScheduleType::OutsideWorkingHours);
                                                cx.notify();
                                            }))
                                            .child("Outside Hours"),
                                    )
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .cursor_pointer()
                                            .bg(if schedule_type == AwayScheduleType::Custom { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .text_xs()
                                            .text_color(rgb(0xffffff))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_schedule_type(AwayScheduleType::Custom);
                                                cx.notify();
                                            }))
                                            .child("Custom"),
                                    ),
                            ),
                    )
                    // Offline Only toggle
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
                                this.toggle_offline_only();
                                cx.notify();
                            }))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child("Only When Offline"))
                            .child(div().text_xs().text_color(if offline_only { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if offline_only { "YES" } else { "NO" })),
                    )
            })
            .into_any_element()
    }
}
