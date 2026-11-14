//! Upstream Source: Telegram/SourceFiles/intro/intro_phone.cpp, intro_phone.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::countries_instance::CountriesInstance;
use super::intro_chrome::{
    render_error_banner, render_input_box, render_primary_cta, render_text_link, IntroAction,
};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::prelude::FluentBuilder as _;
use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroPhoneField {
    CountryCode,
    #[default]
    Phone,
    Username,
    Password,
}

#[derive(Debug, Clone)]
pub struct IntroPhone {
    pub step: IntroStep,
    pub country_code: String,
    pub phone_number: String,
    pub country_name: String,
    pub flag_emoji: String,
    pub countries: CountriesInstance,
    pub is_country_dropdown_open: bool,
    pub is_password_login_mode: bool,
    pub username: String,
    pub password: String,
    pub is_password_visible: bool,
    pub active_field: IntroPhoneField,
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
        let code = countries
            .by_iso2(default_iso)
            .and_then(|c| c.codes.first().map(|cc| cc.calling_code.clone()))
            .unwrap_or_else(|| "1".to_string());

        Self {
            step: IntroStep::new(IntroStepType::Phone),
            country_code: code,
            phone_number: String::new(),
            country_name,
            flag_emoji: CountriesInstance::flag_emoji_by_iso2(default_iso),
            countries,
            is_country_dropdown_open: false,
            is_password_login_mode: false,
            username: String::new(),
            password: String::new(),
            is_password_visible: false,
            active_field: IntroPhoneField::Phone,
        }
    }

    pub fn set_country(&mut self, iso2: &str) {
        if let Some(c) = self.countries.by_iso2(iso2) {
            self.country_name = c.name.clone();
            if let Some(cc) = c.codes.first() {
                self.country_code = cc.calling_code.clone();
            }
            self.flag_emoji = CountriesInstance::flag_emoji_by_iso2(iso2);
        }
    }

    pub fn on_phone_input(&mut self, text: impl Into<String>) {
        let raw = text.into();
        let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        self.phone_number = digits;
        let resolved_iso = self.countries.country_iso2_by_phone(&self.full_number());
        if !resolved_iso.is_empty() {
            self.set_country(&resolved_iso);
        }
        self.step.hide_error();
    }

    pub fn full_number(&self) -> String {
        format!("+{}{}", self.country_code, self.phone_number)
    }

    pub fn submit(&mut self) -> Result<IntroStepType, &'static str> {
        if self.phone_number.is_empty() || self.phone_number.len() < 4 {
            self.step.show_error("Invalid phone number.");
            return Err("Invalid phone number");
        }
        self.step.hide_error();
        self.step.data.phone = self.full_number();
        self.step.data.country = self.country_name.clone();
        Ok(IntroStepType::Code)
    }

    pub fn render_view(&self) -> AnyElement {
        self.render_card(&TdesktopThemePalette::default(), "en", None)
    }

    pub fn render_interactive(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        cx: &mut Context<IntroWidget>,
    ) -> AnyElement {
        self.render_card(palette, lang_id, Some(cx))
    }

    fn render_card(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        mut cx: Option<&mut Context<IntroWidget>>,
    ) -> AnyElement {
        let is_pw_mode = self.is_password_login_mode;
        let tab_pill = |label: &str, active: bool| {
            div()
                .cursor_pointer()
                .px(px(12.0))
                .py(px(6.0))
                .rounded(px(6.0))
                .bg(if active { rgb(palette.accent) } else { rgba(0x00000000) })
                .text_size(px(12.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(if active { rgb(palette.active_button_fg) } else { rgb(palette.text_secondary) })
                .child(label.to_string())
        };
        let mut phone_tab = tab_pill(tr_str(lang_id, "手机号", "Phone Number"), !is_pw_mode);
        let mut pw_tab = tab_pill(tr_str(lang_id, "账号密码", "Username & Password"), is_pw_mode);
        let mut country_btn = div()
            .w(px(300.0))
            .px(px(12.0))
            .py(px(10.0))
            .rounded(px(8.0))
            .bg(rgb(palette.input_bg))
            .border_1()
            .border_color(rgb(palette.border_fg))
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_size(px(14.0)).text_color(rgb(palette.text_primary)).child(format!(
                "{} {} (+{})",
                self.flag_emoji, self.country_name, self.country_code
            )))
            .child(tg_icon(
                if self.is_country_dropdown_open { IconName::ChevronDown } else { IconName::ChevronRight },
                14.0,
                palette.icon_fg,
            ));
        let mut phone_input = render_input_box(
            tr_str(lang_id, "手机号码", "Phone number"),
            format!("+{} {}", self.country_code, self.phone_number),
            "",
            self.active_field == IntroPhoneField::Phone,
            self.step.data.error_text.is_some(),
            palette,
        );
        let mut user_input = render_input_box(
            tr_str(lang_id, "用户名", "Username"),
            self.username.clone(),
            "username",
            self.active_field == IntroPhoneField::Username,
            false,
            palette,
        );
        let pw_masked = if self.is_password_visible {
            self.password.clone()
        } else {
            "•".repeat(self.password.chars().count())
        };
        let mut pass_input = render_input_box(
            tr_str(lang_id, "密码", "Password"),
            pw_masked,
            tr_str(lang_id, "输入密码", "Enter password"),
            self.active_field == IntroPhoneField::Password,
            false,
            palette,
        );
        let mut submit_btn = render_primary_cta(
            if is_pw_mode { tr_str(lang_id, "登录", "Sign In") } else { tr_str(lang_id, "下一步", "Next") },
            true,
            palette,
        );
        let mut qr_link = render_text_link(tr_str(lang_id, "扫码快捷登录", "Quick log in using QR code"), palette);
        let mut reg_link = render_text_link(tr_str(lang_id, "注册新账号", "Create new account"), palette);
        let countries_list = self.countries.list().to_vec();
        let mut country_items: Vec<Div> = countries_list
            .iter()
            .map(|c| {
                let code_s = c.codes.first().map(|cc| cc.calling_code.as_str()).unwrap_or("");
                div()
                    .cursor_pointer()
                    .px(px(12.0))
                    .py(px(7.0))
                    .hover(|s| s.bg(rgb(palette.dialogs_hover_bg)))
                    .flex()
                    .justify_between()
                    .text_size(px(13.0))
                    .text_color(rgb(palette.text_primary))
                    .child(format!("{} {}", CountriesInstance::flag_emoji_by_iso2(&c.iso2), c.name))
                    .child(div().text_color(rgb(palette.text_secondary)).child(format!("+{}", code_s)))
            })
            .collect();

        if let Some(cx_ref) = cx.as_deref_mut() {
            phone_tab = phone_tab.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.is_password_login_mode = false;
                this.phone_view.active_field = IntroPhoneField::Phone;
                this.phone_view.step.hide_error();
                cx.notify();
            }));
            pw_tab = pw_tab.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.is_password_login_mode = true;
                this.phone_view.active_field = IntroPhoneField::Username;
                this.phone_view.step.hide_error();
                cx.notify();
            }));
            country_btn = country_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.is_country_dropdown_open = !this.phone_view.is_country_dropdown_open;
                cx.notify();
            }));
            phone_input = phone_input.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.active_field = IntroPhoneField::Phone;
                cx.notify();
            }));
            user_input = user_input.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.active_field = IntroPhoneField::Username;
                cx.notify();
            }));
            pass_input = pass_input.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.phone_view.active_field = IntroPhoneField::Password;
                cx.notify();
            }));
            let bound_items: Vec<Div> = country_items
                .into_iter()
                .enumerate()
                .map(|(idx, row)| {
                    let iso2 = countries_list[idx].iso2.clone();
                    row.on_mouse_down(MouseButton::Left, cx_ref.listener(move |this, _, _w, cx| {
                        this.phone_view.set_country(&iso2);
                        this.phone_view.is_country_dropdown_open = false;
                        cx.notify();
                    }))
                })
                .collect();
            country_items = bound_items;
            submit_btn = submit_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.phone_view.is_password_login_mode {
                    if this.phone_view.username.trim().is_empty() || this.phone_view.password.is_empty() {
                        this.phone_view.step.show_error("Please enter username and password.");
                    } else {
                        this.phone_view.step.hide_error();
                        this.pending_action = Some(IntroAction::SubmitCredentials {
                            username: this.phone_view.username.trim().to_string(),
                            password: this.phone_view.password.clone(),
                        });
                    }
                } else if this.phone_view.submit().is_ok() {
                    let phone = this.phone_view.full_number();
                    this.code_view.step.data.phone = phone.clone();
                    this.pending_action = Some(IntroAction::SubmitPhone { phone });
                }
                cx.notify();
            }));
            qr_link = qr_link.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.pending_action = Some(IntroAction::OpenQrLogin);
                this.show_step(IntroStepType::Qr);
                cx.notify();
            }));
            reg_link = reg_link.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.show_step(IntroStepType::Email);
                cx.notify();
            }));
        }

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(14.0))
            .child(
                div().text_size(px(20.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(palette.text_primary))
                    .child(if is_pw_mode {
                        tr_str(lang_id, "账号密码登录", "Sign In to Cloud Account")
                    } else {
                        tr_str(lang_id, "您的手机号码", "Your Phone Number")
                    }),
            )
            .child(
                div().p(px(3.0)).rounded(px(8.0)).bg(rgb(palette.input_bg)).flex().gap(px(4.0))
                    .child(phone_tab).child(pw_tab),
            )
            .when(!is_pw_mode, |el| {
                el.child(country_btn)
                    .when(self.is_country_dropdown_open, |d| {
                        d.child(
                            div()
                                .id("intro-phone-country-dropdown")
                                .w(px(300.0))
                                .max_h(px(150.0))
                                .overflow_y_scroll()
                                .rounded(px(8.0))
                                .bg(rgb(palette.card_bg))
                                .border_1()
                                .border_color(rgb(palette.border_fg))
                                .flex()
                                .flex_col()
                                .children(country_items),
                        )
                    })
                    .child(phone_input)
            })
            .when(is_pw_mode, |el| el.child(user_input).child(pass_input))
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(submit_btn)
            .child(div().flex().flex_col().items_center().gap(px(4.0)).child(qr_link).child(reg_link))
            .into_any_element()
    }
}

impl Render for IntroPhone {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
