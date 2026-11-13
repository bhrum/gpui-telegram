//! Upstream Source: Telegram/SourceFiles/settings/business/settings_working_hours.cpp, Telegram/SourceFiles/settings/business/settings_working_hours.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_DAY_SECONDS: i32 = 86400;

#[derive(Debug, Clone)]
pub struct DayInterval {
    pub start_seconds: i32,
    pub end_seconds: i32,
}

#[derive(Debug, Clone)]
pub struct DaySchedule {
    pub day_index: usize,
    pub name: &'static str,
    pub is_open: bool,
    pub intervals: Vec<DayInterval>,
}

#[derive(Debug, Clone)]
pub struct SettingsWorkingHoursBusiness {
    pub enabled: bool,
    pub days: Vec<DaySchedule>,
    pub timezone_id: String,
    pub timezone_name: String,
    pub utc_offset_hours: i32,
    pub status_message: Option<String>,
}

impl SettingsWorkingHoursBusiness {
    pub fn new() -> Self {
        let day_names = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        let mut days = Vec::with_capacity(7);
        for (i, &name) in day_names.iter().enumerate() {
            let is_open = i < 5;
            days.push(DaySchedule {
                day_index: i,
                name,
                is_open,
                intervals: if is_open {
                    vec![DayInterval {
                        start_seconds: 9 * 3600,
                        end_seconds: 18 * 3600,
                    }]
                } else {
                    Vec::new()
                },
            });
        }

        Self {
            enabled: false,
            days,
            timezone_id: "UTC".into(),
            timezone_name: "UTC (Coordinated Universal Time)".into(),
            utc_offset_hours: 0,
            status_message: None,
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn toggle_day(&mut self, index: usize) {
        if let Some(day) = self.days.get_mut(index) {
            day.is_open = !day.is_open;
            if day.is_open && day.intervals.is_empty() {
                day.intervals.push(DayInterval {
                    start_seconds: 9 * 3600,
                    end_seconds: 18 * 3600,
                });
            } else if !day.is_open {
                day.intervals.clear();
            }
        }
    }

    pub fn set_timezone(&mut self, id: String, name: String, offset: i32) {
        self.timezone_id = id;
        self.timezone_name = name;
        self.utc_offset_hours = offset;
    }

    pub fn save(&mut self) {
        self.status_message = Some("Working hours saved".into());
    }
}

impl Default for SettingsWorkingHoursBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsWorkingHoursBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enabled = self.enabled;
        let tz_name = self.timezone_name.clone();

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
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Working Hours"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Set your business operating schedule and timezone.")),
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
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Show Working Hours"))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(if enabled { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if enabled { "ON" } else { "OFF" })),
            )
            // Schedule table when enabled
            .when(enabled, |container| {
                container
                    // 7 days rows
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .children(self.days.iter().map(|day| {
                                let idx = day.day_index;
                                let is_open = day.is_open;
                                let intervals_text = if is_open {
                                    if day.intervals.is_empty() {
                                        "Open all day".to_string()
                                    } else {
                                        day.intervals
                                            .iter()
                                            .map(|iv| format!("{:02}:00 - {:02}:00", iv.start_seconds / 3600, iv.end_seconds / 3600))
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    }
                                } else {
                                    "Closed".to_string()
                                };

                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .py(px(6.0))
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                        this.toggle_day(idx);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(0xffffff)).child(day.name))
                                            .child(div().text_xs().text_color(if is_open { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(intervals_text)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if is_open { rgb(0x5288c1) } else { rgb(0x8292a1) })
                                            .child(if is_open { "OPEN" } else { "CLOSED" }),
                                    )
                            })),
                    )
                    // Timezone setting row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child("Timezone"))
                            .child(div().text_xs().text_color(rgb(0x5288c1)).child(tz_name)),
                    )
            })
            .into_any_element()
    }
}
