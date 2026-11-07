//! Upstream Source: Telegram/SourceFiles/settings/business/settings_location.cpp, Telegram/SourceFiles/settings/business/settings_location.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_LOCATION_PREVIEW_WIDTH: f32 = 300.0;
pub const K_LOCATION_PREVIEW_HEIGHT: f32 = 160.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct SettingsLocationBusiness {
    pub address: String,
    pub point: Option<GeoPoint>,
    pub map_supported: bool,
    pub is_picking: bool,
    pub status_message: Option<String>,
}

impl SettingsLocationBusiness {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            point: None,
            map_supported: true,
            is_picking: false,
            status_message: None,
        }
    }

    pub fn set_address(&mut self, text: String) {
        self.address = text;
    }

    pub fn set_point(&mut self, lat: f64, lon: f64) {
        self.point = Some(GeoPoint { lat, lon });
        self.is_picking = false;
    }

    pub fn clear_point(&mut self) {
        self.point = None;
        self.is_picking = false;
    }

    pub fn toggle_picker(&mut self) {
        self.is_picking = !self.is_picking;
    }

    pub fn save(&mut self) {
        let trimmed = self.address.trim().to_string();
        self.address = trimmed;
        self.status_message = Some("Location saved".into());
    }
}

impl Default for SettingsLocationBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsLocationBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_point = self.point.is_some();
        let pt = self.point;
        let is_picking = self.is_picking;

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
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Business Location"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Display your physical address and pin location on your profile.")),
            )
            // Address input card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(4.0))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Street Address"))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child(if self.address.is_empty() { "Enter business address...".into() } else { self.address.clone() })),
            )
            // Map location toggle / preview
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
                        this.toggle_picker();
                        cx.notify();
                    }))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Pin on Map"))
                    .child(div().text_xs().text_color(if has_point { rgb(0x5288c1) } else { rgb(0x8292a1) }).child(if has_point { "SET" } else { "NOT SET" })),
            )
            // Interactive Map Picker Mock / Card
            .when(is_picking, |container| {
                container.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .p(px(16.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x232e3c))
                        .gap(px(8.0))
                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Select Location Coordinates"))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.set_point(40.7128, -74.0060); // New York
                                            this.set_address("New York, NY, USA".into());
                                            cx.notify();
                                        }))
                                        .child("New York"),
                                )
                                .child(
                                    div()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.set_point(51.5074, -0.1278); // London
                                            this.set_address("London, UK".into());
                                            cx.notify();
                                        }))
                                        .child("London"),
                                ),
                        ),
                )
            })
            // Map Preview Card
            .when_some(pt, |container, p| {
                container.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(K_LOCATION_PREVIEW_HEIGHT))
                        .rounded(px(8.0))
                        .bg(rgb(0x1a2634))
                        .gap(px(4.0))
                        .child(div().text_2xl().child("📍"))
                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("Lat: {:.4}, Lon: {:.4}", p.lat, p.lon)))
                        .child(
                            div()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0xe05555))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                    this.clear_point();
                                    cx.notify();
                                }))
                                .child("Remove Pin"),
                        ),
                )
            })
            .into_any_element()
    }
}
