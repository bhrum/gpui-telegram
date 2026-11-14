//! Upstream Source: Telegram/SourceFiles/intro/intro_code.cpp, intro_code.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{render_error_banner, render_primary_cta, render_text_link, IntroAction};
use super::intro_code_input::IntroCodeInput;
use super::intro_step::{IntroCallStatus, IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::TdesktopThemePalette;
use gpui::*;

#[derive(Debug, Clone)]
pub struct IntroCode {
    pub step: IntroStep,
    pub input: IntroCodeInput,
    pub call_timeout_sec: i32,
    pub is_by_telegram: bool,
    pub is_submitting: bool,
    pub fragment_url: String,
}

impl Default for IntroCode {
    fn default() -> Self {
        Self::new(5, true, 60)
    }
}

impl IntroCode {
    pub fn new(length: usize, by_telegram: bool, timeout_sec: i32) -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Code),
            input: IntroCodeInput::new(length),
            call_timeout_sec: timeout_sec,
            is_by_telegram: by_telegram,
            is_submitting: false,
            fragment_url: String::new(),
        }
    }

    pub fn tick_timer(&mut self) {
        if self.call_timeout_sec > 0 {
            self.call_timeout_sec -= 1;
        }
    }

    pub fn formatted_timeout(&self) -> String {
        if self.call_timeout_sec <= 0 {
            return String::new();
        }
        let hours = self.call_timeout_sec / 3600;
        let minutes = (self.call_timeout_sec / 60) % 60;
        let seconds = self.call_timeout_sec % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{}:{:02}", minutes, seconds)
        }
    }

    pub fn no_telegram_code(&mut self) {
        self.is_by_telegram = false;
        self.step.data.call_status = IntroCallStatus::Waiting;
    }

    pub fn submit(&mut self, is_2fa_enabled: bool) -> Result<IntroStepType, &'static str> {
        if !self.input.is_complete() {
            self.input.set_error(true);
            self.step.show_error("Please enter the complete verification code.");
            return Err("Incomplete code");
        }
        self.step.hide_error();
        self.is_submitting = true;
        if is_2fa_enabled {
            Ok(IntroStepType::PasswordCheck)
        } else {
            Ok(IntroStepType::Signup)
        }
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
        let timer_text = self.formatted_timeout();
        let phone_display = if self.step.data.phone.is_empty() {
            tr_str(lang_id, "输入验证码", "Enter verification code").to_string()
        } else {
            self.step.data.phone.clone()
        };

        let mut submit_btn = render_primary_cta(
            tr_str(lang_id, "继续", "Continue"),
            self.input.is_complete(),
            palette,
        );
        let mut resend_link = render_text_link(
            if timer_text.is_empty() {
                tr_str(lang_id, "重新发送短信验证码", "Resend verification code").to_string()
            } else {
                format!(
                    "{} ({})",
                    tr_str(lang_id, "语音/短信重发倒计时", "Telegram call in"),
                    timer_text
                )
            },
            palette,
        );
        let mut edit_phone_link = render_text_link(
            tr_str(lang_id, "修改手机号", "Edit phone number"),
            palette,
        );
        if let Some(cx_ref) = cx.as_deref_mut() {
            submit_btn = submit_btn.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    let code = this.code_view.input.code();
                    if this.code_view.input.is_complete() {
                        this.code_view.step.hide_error();
                        this.pending_action = Some(IntroAction::SubmitCode { code });
                    } else {
                        this.code_view.input.set_error(true);
                        this.code_view
                            .step
                            .show_error("Please enter the complete verification code.");
                    }
                    cx.notify();
                }),
            );
            resend_link = resend_link.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    this.code_view.no_telegram_code();
                    this.pending_action = Some(IntroAction::ResendCode);
                    cx.notify();
                }),
            );
            edit_phone_link = edit_phone_link.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    this.show_step(IntroStepType::Phone);
                    cx.notify();
                }),
            );
        }

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(21.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(phone_display),
            )
            .child(
                div()
                    .max_w(px(300.0))
                    .text_size(px(13.5))
                    .text_color(rgb(palette.text_secondary))
                    .text_center()
                    .child(if self.is_by_telegram {
                        tr_str(
                            lang_id,
                            "我们已向您其他设备上的 Telegram 应用发送验证码，请直接输入。",
                            "We have sent the code to the Telegram app on your other device.",
                        )
                    } else {
                        tr_str(
                            lang_id,
                            "我们已发送短信验证码，请在下方输入您收到的数字验证码。",
                            "We have sent a verification code via SMS. Enter it below.",
                        )
                    }),
            )
            .child(self.input.render_themed(palette))
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(submit_btn)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .child(resend_link)
                    .child(edit_phone_link),
            )
            .into_any_element()
    }
}

impl Render for IntroCode {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
