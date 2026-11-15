//! Rendering helpers for the Alipay QR login tab and Email OTP registration tab inside `AuthWizardView`.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName};

use super::auth_wizard_state::{AuthWizardTab, LoginInputField, LoginWizardStep};
use super::auth_wizard_view::{AuthWizardView, WizardInputFieldSpec};

impl AuthWizardView {
    /// Renders an authentic scannable Alipay QR matrix using `qrcode` crate (`EcLevel::Q`),
    /// or a loading/refresh placeholder card while awaiting backend QR payload generation.
    pub fn render_real_alipay_qr_matrix(
        payload: Option<&str>,
        is_loading: bool,
        zh: bool,
    ) -> AnyElement {
        if let Some(url) = payload.filter(|s| !s.trim().is_empty()) {
            if let Ok(code) =
                qrcode::QrCode::with_error_correction_level(url.as_bytes(), qrcode::EcLevel::Q)
            {
                let n = code.width();
                let colors = code.into_colors();
                let module_px: f32 = if n <= 45 {
                    4.0
                } else if n <= 65 {
                    3.0
                } else {
                    2.5
                };
                let grid_px = (n as f32) * module_px;
                let quiet_px = 12.0_f32;
                let card_px = grid_px + quiet_px * 2.0;

                let mut matrix_layer = div()
                    .relative()
                    .w(px(grid_px))
                    .h(px(grid_px))
                    .bg(rgb(0xFFFFFF));

                for y in 0..n {
                    let mut x = 0;
                    while x < n {
                        if matches!(colors[y * n + x], qrcode::Color::Dark) {
                            let start_x = x;
                            while x < n && matches!(colors[y * n + x], qrcode::Color::Dark) {
                                x += 1;
                            }
                            let span_w = ((x - start_x) as f32) * module_px;
                            matrix_layer = matrix_layer.child(
                                div()
                                    .absolute()
                                    .left(px((start_x as f32) * module_px))
                                    .top(px((y as f32) * module_px))
                                    .w(px(span_w))
                                    .h(px(module_px))
                                    .bg(rgb(0x000000)),
                            );
                        } else {
                            x += 1;
                        }
                    }
                }

                // Compact center Alipay badge (~2.5% area, safely covered by EcLevel::Q 25% ECC)
                let badge_outer = 28.0_f32;
                let badge_offset = (grid_px - badge_outer) * 0.5;
                matrix_layer = matrix_layer.child(
                    div()
                        .absolute()
                        .left(px(badge_offset))
                        .top(px(badge_offset))
                        .w(px(badge_outer))
                        .h(px(badge_outer))
                        .rounded(px(6.0))
                        .bg(rgb(0xFFFFFF))
                        .p(px(2.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w_full()
                                .h_full()
                                .rounded(px(4.0))
                                .bg(rgb(0x1677FF))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child("支"),
                        ),
                );

                return div()
                    .w(px(card_px))
                    .h(px(card_px))
                    .rounded(px(12.0))
                    .bg(rgb(0xFFFFFF))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(matrix_layer)
                    .into_any_element();
            }
        }

        div()
            .w(px(200.0))
            .h(px(200.0))
            .rounded(px(12.0))
            .bg(rgb(0xFFFFFF))
            .p(px(16.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .child(
                div()
                    .size(px(48.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1677FF))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(24.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("支"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x475569))
                    .child(if is_loading {
                        if zh {
                            "正在生成支付宝二维码..."
                        } else {
                            "Generating Alipay QR..."
                        }
                    } else if zh {
                        "请点击下方刷新获取二维码"
                    } else {
                        "Click Refresh below to load QR"
                    }),
            )
            .into_any_element()
    }

    pub(super) fn render_alipay_qr_tab(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let zh = self.is_chinese_locale;
        let status_text = if self.alipay_status_text.is_empty() {
            if zh {
                "请使用手机支付宝 App 扫码登录".to_string()
            } else {
                "Please scan with Alipay mobile app".to_string()
            }
        } else {
            self.alipay_status_text.clone()
        };
        let is_loading = self.alipay_is_loading;
        let qr_card = Self::render_real_alipay_qr_matrix(
            self.alipay_qr_payload.as_deref(),
            is_loading,
            zh,
        );

        div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(tr_str(lang, "使用支付宝扫码登录", "Log in with Alipay QR")),
            )
            .child(qr_card)
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(if is_loading {
                        rgb(pal.accent)
                    } else {
                        rgb(pal.text_secondary)
                    })
                    .child(status_text),
            )
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(40.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x1677FF))
                    .hover(|s| s.bg(rgb(0x0E5CD6)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.open_alipay_web_auth(cx);
                        }),
                    )
                    .child(tg_icon(IconName::SendPlane, 13.0, 0xFFFFFF))
                    .child(tr_str(
                        lang,
                        "在浏览器打开支付宝授权",
                        "Open Alipay Authorization in Browser",
                    )),
            )
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(36.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_primary))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.start_alipay_flow(cx);
                        }),
                    )
                    .child(tg_icon(IconName::ClockSending, 13.0, pal.icon_fg))
                    .child(tr_str(lang, "刷新二维码", "Refresh QR Code")),
            )
            .into_any_element()
    }

    pub(super) fn render_email_register_tab(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let zh = self.is_chinese_locale;
        let is_loading = self.is_authenticating;
        let cooldown = self.email_code_cooldown;
        let is_sending = self.is_sending_email_code;

        div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(tr_str(lang, "邮箱注册账号", "Register with Email")),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(self.render_wizard_input_field(
                        WizardInputFieldSpec {
                            field: LoginInputField::Email,
                            value: &self.email_input,
                            is_active: self.active_login_field == LoginInputField::Email,
                            is_password: false,
                            active_placeholder: tr_str(lang, "| 请输入邮箱地址", "| Email"),
                            idle_placeholder: tr_str(lang, "邮箱地址", "Email"),
                            trailing_icon: None,
                            allow_clear: false,
                            flex_grow: true,
                        },
                        cx,
                    ))
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .h(px(40.0))
                            .rounded(px(8.0))
                            .bg(if cooldown > 0 || is_sending {
                                rgb(pal.input_bg)
                            } else {
                                rgb(pal.accent)
                            })
                            .border_1()
                            .border_color(if cooldown > 0 || is_sending {
                                rgb(pal.border_fg)
                            } else {
                                rgb(pal.accent)
                            })
                            .hover(move |s| {
                                if cooldown == 0 && !is_sending {
                                    s.bg(rgb(pal.accent_hover))
                                } else {
                                    s
                                }
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if cooldown > 0 || is_sending {
                                rgb(pal.text_secondary)
                            } else {
                                rgb(pal.active_button_fg)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.send_email_code(cx);
                                }),
                            )
                            .child(if is_sending {
                                tr_str(lang, "发送中...", "Sending...").to_string()
                            } else if cooldown > 0 {
                                if zh {
                                    format!("重新获取 ({}s)", cooldown)
                                } else {
                                    format!("Resend ({}s)", cooldown)
                                }
                            } else {
                                tr_str(lang, "获取验证码", "Send Code").to_string()
                            }),
                    ),
            )
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::EmailCode,
                    value: &self.email_code_input,
                    is_active: self.active_login_field == LoginInputField::EmailCode,
                    is_password: false,
                    active_placeholder: tr_str(lang, "| 请输入6位验证码", "| 6-digit Code"),
                    idle_placeholder: tr_str(lang, "验证码", "Verification Code"),
                    trailing_icon: None,
                    allow_clear: false,
                    flex_grow: false,
                },
                cx,
            ))
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::Username,
                    value: &self.username_input,
                    is_active: self.active_login_field == LoginInputField::Username,
                    is_password: false,
                    active_placeholder: tr_str(lang, "| 请输入用户名", "| Username"),
                    idle_placeholder: tr_str(lang, "用户名", "Username"),
                    trailing_icon: None,
                    allow_clear: false,
                    flex_grow: false,
                },
                cx,
            ))
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::Password,
                    value: &self.password_input,
                    is_active: self.active_login_field == LoginInputField::Password,
                    is_password: true,
                    active_placeholder: tr_str(lang, "| 请输入密码", "| Password"),
                    idle_placeholder: tr_str(lang, "密码", "Password"),
                    trailing_icon: None,
                    allow_clear: false,
                    flex_grow: false,
                },
                cx,
            ))
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::FirstName,
                    value: &self.first_name_input,
                    is_active: self.active_login_field == LoginInputField::FirstName,
                    is_password: false,
                    active_placeholder: tr_str(
                        lang,
                        "| 请输入显示昵称（可选）",
                        "| Display Name (optional)",
                    ),
                    idle_placeholder: tr_str(
                        lang,
                        "显示昵称（可选）",
                        "Display Name (optional)",
                    ),
                    trailing_icon: None,
                    allow_clear: false,
                    flex_grow: false,
                },
                cx,
            ))
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(42.0))
                    .rounded(px(8.0))
                    .bg(if is_loading {
                        rgb(pal.accent_hover)
                    } else {
                        rgb(pal.accent)
                    })
                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.active_button_fg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.submit_email_register(cx);
                        }),
                    )
                    .child(if is_loading {
                        tr_str(lang, "正在注册并登录...", "Authenticating...")
                    } else {
                        tr_str(lang, "立即注册并登录", "Register & Sign In")
                    }),
            )
            .into_any_element()
    }

    pub(super) fn render_phone_step(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let country = self.country_code.clone();
        let phone = self.phone_number.clone();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(tr_str(lang, "输入您的手机号码", "Your Phone Number")),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .children(["+86", "+1", "+44", "+81"].into_iter().map(|cc| {
                        let is_sel = country == cc;
                        let cc_str = cc.to_string();
                        div()
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(if is_sel { pal.accent } else { pal.input_bg }))
                            .border_1()
                            .border_color(rgb(if is_sel { pal.accent } else { pal.border_fg }))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(if is_sel {
                                pal.active_button_fg
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.country_code = cc_str.clone();
                                    cx.notify();
                                }),
                            )
                            .child(cc)
                    })),
            )
            .child(
                div()
                    .w_full()
                    .h(px(42.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(rgb(pal.accent))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(format!(
                                "{} {}",
                                country,
                                if phone.is_empty() { "___ ____ ____" } else { &phone }
                            )),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .p(px(2.0))
                            .rounded(px(4.0))
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.phone_number.clear();
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                    ),
            )
            .child(self.render_numeric_keypad(false, cx))
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(14.0))
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(5.0))
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.login_step = LoginWizardStep::Start;
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::BackArrow, 13.0, pal.icon_fg))
                            .child(tr_str(lang, "返回", "Back")),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .flex_1()
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.accent))
                            .hover(move |s| s.bg(rgb(pal.accent_hover)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.active_button_fg))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.login_step = LoginWizardStep::Code;
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "下一步 (获取验证码) →", "NEXT →")),
                    ),
            )
            .into_any_element()
    }

    pub(super) fn render_qr_step(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let zh = self.is_chinese_locale;

        div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(tr_str(lang, "使用移动端扫码登录", "Scan QR Code with Mobile")),
            )
            .child(Self::render_real_alipay_qr_matrix(
                self.alipay_qr_payload.as_deref(),
                self.alipay_is_loading,
                zh,
            ))
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(42.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x1677FF))
                    .hover(|s| s.bg(rgb(0x0E5CD6)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.login_step = LoginWizardStep::Start;
                            this.switch_auth_tab(AuthWizardTab::AlipayQr, cx);
                        }),
                    )
                    .child(tg_icon(IconName::WalletStars, 13.0, 0xFFFFFF))
                    .child(tr_str(lang, "切换至支付宝真实扫码登录", "Switch to Alipay QR Login")),
            )
            .child(
                div()
                    .cursor_pointer()
                    .text_size(px(13.0))
                    .text_color(rgb(pal.accent))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.login_step = LoginWizardStep::Phone;
                            cx.notify();
                        }),
                    )
                    .child(tr_str(lang, "改用手机号登录", "Log in by phone number")),
            )
            .into_any_element()
    }

    pub(super) fn render_code_step(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let zh = self.is_chinese_locale;
        let country = self.country_code.clone();
        let phone = self.phone_number.clone();
        let code = self.verification_code.clone();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(if zh {
                        format!("验证码已发送至 {} {}", country, phone)
                    } else {
                        format!("Code sent to {} {}", country, phone)
                    }),
            )
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(44.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(rgb(pal.accent))
                    .px(px(14.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if !this.verification_code.trim().is_empty() {
                                this.submit_login(cx);
                            } else {
                                cx.notify();
                            }
                        }),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(if code.is_empty() { "• • • • •".to_string() } else { code }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .p(px(2.0))
                            .rounded(px(4.0))
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.verification_code.clear();
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                    ),
            )
            .child(self.render_numeric_keypad(true, cx))
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(44.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.accent))
                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.active_button_fg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.verification_code.trim().is_empty() {
                                this.login_error = Some(
                                    tr_str(
                                        this.active_lang_id(),
                                        "请输入您收到的验证码",
                                        "Please enter the verification code you received",
                                    )
                                    .to_string(),
                                );
                                cx.notify();
                            } else {
                                this.submit_login(cx);
                            }
                        }),
                    )
                    .child(tr_str(lang, "验证并登录", "Verify & Sign In")),
            )
            .child(
                div()
                    .cursor_pointer()
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_secondary))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.login_step = LoginWizardStep::Phone;
                            cx.notify();
                        }),
                    )
                    .child(tr_str(lang, "← 修改手机号", "← Change Phone Number")),
            )
            .into_any_element()
    }

    fn render_numeric_keypad(&self, for_code: bool, cx: &mut Context<Self>) -> Div {
        let pal = self.palette;
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap(px(6.0))
            .children(["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "⌫"].into_iter().map(
                |digit| {
                    let d_str = digit.to_string();
                    div()
                        .cursor_pointer()
                        .w(px(58.0))
                        .h(px(32.0))
                        .rounded(px(6.0))
                        .bg(rgb(pal.input_bg))
                        .when(!for_code, |el| el.border_1().border_color(rgb(pal.border_fg)))
                        .hover(move |s| s.bg(rgb(pal.hover_bg)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                if for_code {
                                    if d_str == "⌫" {
                                        this.verification_code.pop();
                                    } else if this.verification_code.len() < 6 {
                                        this.verification_code.push_str(&d_str);
                                        if this.verification_code.len() >= 5 {
                                            this.submit_login(cx);
                                        }
                                    }
                                } else if d_str == "⌫" {
                                    this.phone_number.pop();
                                } else {
                                    this.phone_number.push_str(&d_str);
                                }
                                cx.notify();
                            }),
                        )
                        .child(digit)
                },
            ))
    }
}
