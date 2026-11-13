//! Upstream Source: Telegram/SourceFiles/intro/intro_phone.cpp, intro_phone.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::countries_instance::CountriesInstance;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroPhone {
    pub step: IntroStep,
    pub country_code: String,
    pub phone_number: String,
    pub country_name: String,
    pub flag_emoji: String,
    pub countries: CountriesInstance,
    pub is_country_dropdown_open: bool,
}

impl Default for IntroPhone {
    fn default() -> Self {
        Self::new()
    }
}

impl IntroPhone {
    pub fn new() -> Self {
        let countries = CountriesInstance::new();
        let default_iso = "US";
        let country_name = countries.country_name_by_iso2(default_iso);
        let flag_emoji = CountriesInstance::flag_emoji_by_iso2(default_iso);

        Self {
            step: IntroStep::new(IntroStepType::Phone),
            country_code: "1".to_string(),
            phone_number: String::new(),
            country_name,
            flag_emoji,
            countries,
            is_country_dropdown_open: false,
        }
    }

    pub fn set_country(&mut self, iso2: &str) {
        if let Some(info) = self.countries.by_iso2(iso2) {
            self.country_name = info.name.clone();
            self.flag_emoji = CountriesInstance::flag_emoji_by_iso2(&info.iso2);
            if let Some(code) = info.codes.first() {
                self.country_code = code.calling_code.clone();
            }
        }
    }

    pub fn on_phone_input(&mut self, text: impl Into<String>) {
        self.phone_number = text.into().chars().filter(|c| c.is_ascii_digit()).collect();
        let full = format!("{}{}", self.country_code, self.phone_number);
        let detected_iso = self.countries.country_iso2_by_phone(&full);
        if !detected_iso.is_empty() {
            self.country_name = self.countries.country_name_by_iso2(&detected_iso);
            self.flag_emoji = CountriesInstance::flag_emoji_by_iso2(&detected_iso);
        }
    }

    pub fn full_number(&self) -> String {
        format!("+{}{}", self.country_code, self.phone_number)
    }

    pub fn submit(&mut self) -> Result<IntroStepType, &'static str> {
        if self.phone_number.trim().is_empty() {
            self.step.show_error("Please enter your phone number");
            return Err("Phone number is empty");
        }
        self.step.hide_error();
        self.step.data.phone = self.full_number();
        self.step.data.country = self.country_name.clone();
        Ok(IntroStepType::Code)
    }

    pub fn render_view(&self) -> AnyElement {
        let phone_text = if self.phone_number.is_empty() {
            "-- --- ----".to_string()
        } else {
            self.phone_number.clone()
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Your Phone Number"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(320.0))
                    .child("Please confirm your country code and enter your phone number."),
            )
            .child(
                // Country selector row
                div()
                    .mt(px(28.0))
                    .w(px(300.0))
                    .p(px(10.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212b))
                    .border_1()
                    .border_color(rgb(0x242f3d))
                    .flex()
                    .items_center()
                    .justify_between()
                    .cursor_pointer()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(div().child(self.flag_emoji.clone()))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xffffff))
                                    .child(self.country_name.clone()),
                            ),
                    )
                    .child(div().text_color(rgb(0x8292a1)).child("▼")),
            )
            .child(
                // Dual inputs: Country Code (+1) and Phone Number (225px)
                div()
                    .mt(px(12.0))
                    .w(px(300.0))
                    .flex()
                    .gap(px(10.0))
                    .child(
                        div()
                            .w(px(64.0))
                            .h(px(42.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .border_1()
                            .border_color(rgb(0x242f3d))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(format!("+{}", self.country_code)),
                    )
                    .child(
                        div()
                            .w(px(226.0))
                            .h(px(42.0))
                            .px(px(12.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .border_1()
                            .border_color(rgb(0x242f3d))
                            .flex()
                            .items_center()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(phone_text),
                    ),
            )
            .child(
                // Action Next Button (width 300px, height 42px)
                div()
                    .mt(px(32.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2aabee))
                    .hover(|s| s.bg(rgb(0x229ed9)))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child("NEXT"),
                    ),
            )
            .into_any_element()
    }
}

impl Render for IntroPhone {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
