//! Authentic GPUI implementation of Telegram Desktop's Auto-Lock Configuration Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/auto_lock_box.h:L16-L29` and `auto_lock_box.cpp:L21-L151`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Anonymous-namespace constants from `auto_lock_box.cpp:L21-L24`.
pub const K_CUSTOM: i32 = i32::MAX;
pub const K_OPTIONS: [i32; 5] = [60, 300, 3600, 18000, K_CUSTOM];
pub const K_DEFAULT_CUSTOM: &str = "10:00";

/// Formats seconds into "HH:MM" or "MM:SS" per `auto_lock_box.cpp:L25-L29`.
pub fn time_string(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// Parses a "HH:MM" or "MM:SS" string into total seconds per `auto_lock_box.cpp:L97-L101`.
pub fn parse_custom_time(input: &str) -> Option<i32> {
    let parts: Vec<&str> = input.trim().split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let p0 = parts[0].parse::<i32>().ok()?;
    let p1 = parts[1].parse::<i32>().ok()?;
    if p0 < 0 || p1 < 0 || p1 >= 60 {
        return None;
    }
    Some(p0 * 3600 + p1 * 60)
}

/// AutoLockBox component managing application lock timeout intervals.
/// Cites upstream `AutoLockBox::prepare` (`auto_lock_box.cpp:L36-L139`).
#[derive(Clone)]
pub struct AutoLockBox {
    pub current_seconds: i32,
    pub custom_input: String,
    pub is_closed: bool,
    pub on_save: Option<Arc<dyn Fn(i32) + Send + Sync>>,
}

impl AutoLockBox {
    pub fn new(current_seconds: i32) -> Self {
        let is_standard = K_OPTIONS[..4].contains(&current_seconds);
        let custom_input = if is_standard {
            K_DEFAULT_CUSTOM.to_string()
        } else {
            time_string(current_seconds)
        };

        Self {
            current_seconds,
            custom_input,
            is_closed: false,
            on_save: None,
        }
    }

    /// Selects an auto-lock interval.
    pub fn select_option(&mut self, seconds: i32) {
        self.current_seconds = seconds;
    }

    /// Updates custom time string and validates format.
    pub fn update_custom_time(&mut self, text: impl Into<String>) -> bool {
        let text = text.into();
        if let Some(secs) = parse_custom_time(&text) {
            self.custom_input = text;
            self.current_seconds = secs;
            true
        } else {
            self.custom_input = text;
            false
        }
    }

    /// Mirrors `AutoLockBox::durationChanged` (`auto_lock_box.cpp:L141-L151`).
    pub fn duration_changed(&mut self, seconds: i32) {
        self.current_seconds = seconds;
        self.is_closed = true;
        if let Some(ref cb) = self.on_save {
            cb(seconds);
        }
    }

    pub fn save(&mut self) {
        let target = self.current_seconds;
        self.duration_changed(target);
    }
}

impl Default for AutoLockBox {
    fn default() -> Self {
        Self::new(300) // Default 5 minutes
    }
}

impl Render for AutoLockBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("auto_lock_box_card")
            .w(px(340.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Title: tr::lng_passcode_autolock() (`auto_lock_box.cpp:L37`)
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x222222))
                .child("Auto-lock if away for"),
        );

        // Options list: 1m, 5m, 1h, 5h, custom (`auto_lock_box.cpp:L50-L71`)
        let options = [
            (60, "in 1 minute"),
            (300, "in 5 minutes"),
            (3600, "in 1 hour"),
            (18000, "in 5 hours"),
            (K_CUSTOM, "custom"),
        ];

        let mut list = div().flex().flex_col().gap(px(8.0));
        for (sec, label) in options {
            let is_selected = if sec == K_CUSTOM {
                !K_OPTIONS[..4].contains(&self.current_seconds)
            } else {
                self.current_seconds == sec
            };

            let radio_color = if is_selected {
                rgb(0x5288C1)
            } else {
                rgb(0xCCCCCC)
            };

            let mut row = div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .py(px(4.0))
                .cursor_pointer()
                .child(
                    div()
                        .size(px(18.0))
                        .rounded_full()
                        .border_2()
                        .border_color(radio_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(if is_selected {
                            div().size(px(8.0)).rounded_full().bg(rgb(0x5288C1))
                        } else {
                            div()
                        }),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(0x333333))
                        .child(label),
                );

            if sec == K_CUSTOM && !K_OPTIONS[..4].contains(&self.current_seconds) {
                row = row.child(
                    div()
                        .px(px(8.0))
                        .py(px(2.0))
                        .bg(rgb(0xF4F4F5))
                        .rounded(px(4.0))
                        .text_size(px(13.0))
                        .text_color(rgb(0x333333))
                        .child(self.custom_input.clone()),
                );
            }

            list = list.child(row);
        }
        card = card.child(list);

        // Footer buttons: OK and Cancel
        let btn_row = div()
            .flex()
            .justify_end()
            .gap(px(8.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x5288C1))
                    .child("Save"),
            );

        card.child(btn_row).into_any_element()
    }
}
