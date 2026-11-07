//! Rich Location, Contact & Ephemeral Plate Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_location.cpp:L48-L79`
//! - `Telegram/SourceFiles/history/view/media/history_view_contact.cpp:L39-L100`
//! - `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.cpp:L22-L58`
//! - `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.cpp:L39`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_UNTIL_OFF_PERIOD: i64 = i64::MAX;
pub const K_LIVE_ELAPSED_PART_OPACITY: f32 = 0.2;
pub const K_MAX_LINES: usize = 3;
pub const K_FIT_SLACK: usize = 6;
pub const K_TOAST_DURATION_MS: u64 = 5_000;

/// Upstream RemainingTimeText from `history_view_location.cpp:L48-L66`
pub fn remaining_time_text(elapsed: i64, period: i64) -> String {
    if period == K_UNTIL_OFF_PERIOD {
        return "∞".to_string();
    }
    let remaining = (period - elapsed).clamp(0, period);
    if remaining < 10 {
        format!("{}s", remaining)
    } else if remaining < 600 {
        format!("{}m", remaining / 60)
    } else if remaining < 3600 {
        (remaining / 60).to_string()
    } else if remaining < 86400 {
        format!("{}h", remaining / 3600)
    } else {
        format!("{}d", remaining / 86400)
    }
}

/// Upstream RemainingTimeProgress from `history_view_location.cpp:L68-L79`
pub fn remaining_time_progress(elapsed: i64, period: i64) -> f64 {
    if period == K_UNTIL_OFF_PERIOD {
        1.0
    } else if period < 1 {
        0.0
    } else {
        let remaining = (period - elapsed).clamp(0, period);
        (remaining as f64) / (period as f64)
    }
}

#[derive(Debug, Clone)]
pub struct RichLocationCard {
    pub latitude: f64,
    pub longitude: f64,
    pub title: Option<String>,
    pub address: Option<String>,
    pub is_live: bool,
    pub live_period: i64,
    pub elapsed_seconds: i64,
}

impl RichLocationCard {
    pub fn new_static(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            title: None,
            address: None,
            is_live: false,
            live_period: 0,
            elapsed_seconds: 0,
        }
    }

    pub fn new_live(latitude: f64, longitude: f64, period_seconds: i64) -> Self {
        Self {
            latitude,
            longitude,
            title: Some("Live Location".to_string()),
            address: None,
            is_live: true,
            live_period: period_seconds,
            elapsed_seconds: 0,
        }
    }

    pub fn render(self) -> impl IntoElement {
        let countdown = remaining_time_text(self.elapsed_seconds, self.live_period);
        let latitude = self.latitude;
        let longitude = self.longitude;
        let title = self.title;
        let address = self.address;
        let is_live = self.is_live;

        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .overflow_hidden()
            // Map placeholder area
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(140.0))
                    .bg(rgb(0x243342))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().text_size(px(24.0)).child("📍"))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(format!("{:.4}, {:.4}", latitude, longitude)),
                            ),
                    ),
            )
            // Description & Live Info
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p(px(10.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(title.unwrap_or_else(|| "Location".to_string())),
                            )
                            .when_some(address, |el, addr| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child(addr),
                                )
                            }),
                    )
                    .when(is_live, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded_full()
                                .bg(rgb(0x2AABEE))
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child(format!("Live ({})", countdown)),
                        )
                    }),
            )
    }
}

#[derive(Debug, Clone)]
pub struct RichContactCard {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_id: Option<u64>,
}

impl RichContactCard {
    pub fn new(first_name: impl Into<String>, phone: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: String::new(),
            phone_number: phone.into(),
            user_id: None,
        }
    }

    pub fn full_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    pub fn render(self) -> impl IntoElement {
        let name = self.full_name();
        let phone_number = self.phone_number;

        div()
            .flex()
            .items_center()
            .justify_between()
            .w(px(300.0))
            .p(px(12.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    // Avatar circle
                    .child(
                        div()
                            .w(px(40.0))
                            .h(px(40.0))
                            .rounded_full()
                            .bg(rgb(0x2AABEE))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(name.chars().next().unwrap_or('?').to_uppercase().to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(name),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(phone_number),
                            ),
                    ),
            )
            .child(
                div()
                    .px(px(10.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x233140))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x2AABEE))
                    .child("Add"),
            )
    }
}
