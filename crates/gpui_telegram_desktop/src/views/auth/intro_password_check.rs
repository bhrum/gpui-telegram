//! Upstream Source: Telegram/SourceFiles/intro/intro_password_check.cpp, intro_password_check.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{
    render_error_banner, render_input_box, render_primary_cta, render_text_link, IntroAction,
};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use super::webauthn_common::WebauthnCommon;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::prelude::FluentBuilder as _;
use gpui::*;

#[derive(Debug, Clone)]
pub struct IntroPasswordCheck {
    pub step: IntroStep,
    pub password: String,
    pub hint: String,
    pub is_password_visible: bool,
    pub is_recovering: bool,
    pub recovery_email_pattern: String,
    pub recovery_code: String,
    pub is_submitting: bool,
}

impl Default for IntroPasswordCheck {
    fn default() -> Self {
        Self::new("")
    }
}

impl IntroPasswordCheck {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            step: IntroStep::new(IntroStepType::PasswordCheck),
            password: String::new(),
            hint: hint.into(),
            is_password_visible: false,
            is_recovering: false,
            recovery_email_pattern: String::new(),
            recovery_code: String::new(),
            is_submitting: false,
        }
    }

    pub fn set_password(&mut self, pwd: impl Into<String>) {
        self.password = pwd.into();
    }

    pub fn toggle_visibility(&mut self) {
        self.is_password_visible = !self.is_password_visible;
    }

    pub fn to_recover(&mut self) {
        self.is_recovering = true;
    }

    pub fn to_password(&mut self) {
        self.is_recovering = false;
    }

    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.password.is_empty() && !self.is_recovering {
            self.step.show_error("Password cannot be empty.");
            return Err("Empty password");
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
        let pwd_display = if self.is_recovering {
            self.recovery_code.clone()
        } else if self.is_password_visible {
            self.password.clone()
        } else {
            "•".repeat(self.password.chars().count())
        };
        let input = render_input_box(
            if self.is_recovering {
                tr_str(lang_id, "恢复邮箱验证码", "Recovery email code")
            } else {
                tr_str(lang_id, "两步验证云密码", "Cloud password")
            },
            pwd_display,
            if self.is_recovering { "Code" } else { "Password" },
            true,
            self.step.data.error_text.is_some(),
            palette,
        );
        let mut vis_btn = render_text_link(
            if self.is_password_visible {
                tr_str(lang_id, "隐藏密码", "Hide password")
            } else {
                tr_str(lang_id, "显示密码", "Show password")
            },
            palette,
        );
        let mut submit_btn = render_primary_cta(tr_str(lang_id, "验证并登录", "Submit"), true, palette);
        let mut recover_link = render_text_link(
            if self.is_recovering {
                tr_str(lang_id, "返回输入密码", "Try password instead")
            } else {
                tr_str(lang_id, "忘记密码？", "Forgot password?")
            },
            palette,
        );
        let mut passkey_link = render_text_link(
            tr_str(lang_id, "使用 Passkey 密钥验证", "Verify with Passkey"),
            palette,
        );
        if let Some(cx_ref) = cx.as_deref_mut() {
            vis_btn = vis_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.password_check_view.toggle_visibility();
                cx.notify();
            }));
            submit_btn = submit_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.password_check_view.is_recovering {
                    let code = this.password_check_view.recovery_code.trim().to_string();
                    if code.is_empty() {
                        this.password_check_view.step.show_error("Recovery code is required.");
                    } else {
                        this.password_check_view.step.hide_error();
                        this.pending_action = Some(IntroAction::SubmitRecoverCode { code });
                    }
                } else if this.password_check_view.submit().is_ok() {
                    this.pending_action = Some(IntroAction::SubmitPassword {
                        password: this.password_check_view.password.clone(),
                    });
                }
                cx.notify();
            }));
            recover_link = recover_link.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.password_check_view.is_recovering {
                    this.password_check_view.to_password();
                } else {
                    this.password_check_view.to_recover();
                }
                cx.notify();
            }));
            passkey_link = passkey_link.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.pending_action = Some(IntroAction::TriggerPasskey);
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
                    .child(tg_icon(IconName::Lock, 28.0, 0xFFFFFF)),
            )
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(tr_str(lang_id, "输入两步验证密码", "Your Password")),
            )
            .child(
                div()
                    .max_w(px(300.0))
                    .text_size(px(13.5))
                    .text_color(rgb(palette.text_secondary))
                    .text_center()
                    .child(if self.is_recovering && !self.recovery_email_pattern.is_empty() {
                        format!("Recovery code sent to {}", self.recovery_email_pattern)
                    } else {
                        tr_str(
                            lang_id,
                            "您的账号已开启两步验证，请输入云端密码以继续。",
                            "Your account is protected with an additional cloud password.",
                        )
                        .to_string()
                    }),
            )
            .child(input)
            .when(!self.hint.is_empty() && !self.is_recovering, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(palette.text_secondary))
                        .child(format!("{}: {}", tr_str(lang_id, "密码提示", "Hint"), self.hint)),
                )
            })
            .when(!self.is_recovering, |el| el.child(vis_btn))
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(submit_btn)
            .child(recover_link)
            .when(WebauthnCommon::new().security_key_present(), |el| el.child(passkey_link))
            .into_any_element()
    }
}

impl Render for IntroPasswordCheck {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
