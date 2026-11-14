//! Upstream Source: Telegram/SourceFiles/intro/intro_email.cpp, intro_email.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{
    render_error_banner, render_input_box, render_primary_cta, render_text_link, IntroAction,
};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroEmailField {
    #[default]
    Email,
    Code,
}

#[derive(Debug, Clone)]
pub struct IntroEmail {
    pub step: IntroStep,
    pub email: String,
    pub verification_code: String,
    pub is_code_sent: bool,
    pub is_submitting: bool,
    pub cooldown_seconds: u32,
    pub active_field: IntroEmailField,
}

impl Default for IntroEmail {
    fn default() -> Self {
        Self::new()
    }
}

impl IntroEmail {
    pub fn new() -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Email),
            email: String::new(),
            verification_code: String::new(),
            is_code_sent: false,
            is_submitting: false,
            cooldown_seconds: 0,
            active_field: IntroEmailField::Email,
        }
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
    }

    pub fn set_code(&mut self, code: impl Into<String>) {
        self.verification_code = code.into();
    }

    pub fn tick_cooldown(&mut self) {
        if self.cooldown_seconds > 0 {
            self.cooldown_seconds -= 1;
        }
    }

    pub fn send_code(&mut self) -> Result<(), &'static str> {
        let trimmed = self.email.trim();
        if !trimmed.contains('@') || !trimmed.contains('.') {
            self.step.show_error("Please enter a valid email address.");
            return Err("Invalid email address");
        }
        self.step.hide_error();
        self.is_code_sent = true;
        self.cooldown_seconds = 60;
        self.active_field = IntroEmailField::Code;
        Ok(())
    }

    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.verification_code.trim().is_empty() {
            self.step.show_error("Verification code is required.");
            return Err("Missing verification code");
        }
        self.step.hide_error();
        self.is_submitting = true;
        Ok(())
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
        let mut email_box = render_input_box(
            tr_str(lang_id, "邮箱地址", "Email address"),
            self.email.clone(),
            "name@example.com",
            self.active_field == IntroEmailField::Email,
            self.step.data.error_text.is_some() && !self.is_code_sent,
            palette,
        );
        let mut code_box = render_input_box(
            tr_str(lang_id, "邮箱验证码", "Email verification code"),
            self.verification_code.clone(),
            tr_str(lang_id, "输入收到的验证码", "Enter code from email"),
            self.active_field == IntroEmailField::Code,
            self.step.data.error_text.is_some() && self.is_code_sent,
            palette,
        );
        let mut send_link = render_text_link(
            if self.cooldown_seconds > 0 {
                format!("{} ({}s)", tr_str(lang_id, "已发送，等待重发", "Resend code in"), self.cooldown_seconds)
            } else {
                tr_str(lang_id, "发送邮箱验证码", "Send Verification Code").to_string()
            },
            palette,
        );
        let mut submit_btn = render_primary_cta(
            tr_str(lang_id, "验证并继续", "Verify Email"),
            !self.verification_code.trim().is_empty(),
            palette,
        );
        let mut skip_to_signup = render_text_link(
            tr_str(lang_id, "直接填写注册资料", "Continue to Sign Up details"),
            palette,
        );

        if let Some(cx_ref) = cx.as_deref_mut() {
            email_box = email_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.email_view.active_field = IntroEmailField::Email;
                cx.notify();
            }));
            code_box = code_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.email_view.active_field = IntroEmailField::Code;
                cx.notify();
            }));
            send_link = send_link.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.email_view.cooldown_seconds == 0 && this.email_view.send_code().is_ok() {
                    this.pending_action = Some(IntroAction::SendEmailCode {
                        email: this.email_view.email.trim().to_string(),
                    });
                }
                cx.notify();
            }));
            submit_btn = submit_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.email_view.submit().is_ok() {
                    this.pending_action = Some(IntroAction::SubmitEmail {
                        email: this.email_view.email.trim().to_string(),
                        code: this.email_view.verification_code.trim().to_string(),
                    });
                }
                cx.notify();
            }));
            skip_to_signup = skip_to_signup.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.show_step(IntroStepType::Signup);
                cx.notify();
            }));
        }

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(14.0))
            .child(
                div()
                    .w(px(64.0))
                    .h(px(64.0))
                    .rounded_full()
                    .bg(rgb(palette.dialogs_active_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(tg_icon(IconName::SendPlane, 28.0, 0xFFFFFF)),
            )
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(tr_str(lang_id, "绑定并验证邮箱", "Verify Your Email")),
            )
            .child(email_box)
            .child(send_link)
            .child(code_box)
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(submit_btn)
            .child(skip_to_signup)
            .into_any_element()
    }
}

impl Render for IntroEmail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
