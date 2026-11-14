//! Upstream Source: Telegram/SourceFiles/intro/intro_signup.cpp, intro_signup.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{render_error_banner, render_input_box, render_primary_cta, IntroAction};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroSignupField {
    #[default]
    FirstName,
    LastName,
    Username,
    Password,
}

#[derive(Debug, Clone)]
pub struct IntroSignup {
    pub step: IntroStep,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub invert_order: bool,
    pub has_avatar: bool,
    pub terms_accepted: bool,
    pub active_field: IntroSignupField,
}

impl Default for IntroSignup {
    fn default() -> Self {
        Self::new(false)
    }
}

impl IntroSignup {
    pub fn new(invert_order: bool) -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Signup),
            first_name: String::new(),
            last_name: String::new(),
            username: String::new(),
            password: String::new(),
            invert_order,
            has_avatar: false,
            terms_accepted: true,
            active_field: IntroSignupField::FirstName,
        }
    }

    pub fn set_first_name(&mut self, name: impl Into<String>) {
        self.first_name = name.into();
    }

    pub fn set_last_name(&mut self, name: impl Into<String>) {
        self.last_name = name.into();
    }

    pub fn toggle_avatar(&mut self) {
        self.has_avatar = !self.has_avatar;
    }

    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.first_name.trim().is_empty() {
            self.step.show_error("First name cannot be empty.");
            return Err("Missing first name");
        }
        self.step.hide_error();
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
        let mut first_box = render_input_box(
            tr_str(lang_id, "名字", "First name"),
            self.first_name.clone(),
            "Pavel",
            self.active_field == IntroSignupField::FirstName,
            self.step.data.error_text.is_some(),
            palette,
        );
        let mut last_box = render_input_box(
            tr_str(lang_id, "姓氏（可选）", "Last name (optional)"),
            self.last_name.clone(),
            "Durov",
            self.active_field == IntroSignupField::LastName,
            false,
            palette,
        );
        let mut user_box = render_input_box(
            tr_str(lang_id, "用户名（可选）", "Username (optional)"),
            self.username.clone(),
            "username",
            self.active_field == IntroSignupField::Username,
            false,
            palette,
        );
        let mut pass_box = render_input_box(
            tr_str(lang_id, "登录密码（可选）", "Account password (optional)"),
            "•".repeat(self.password.chars().count()),
            "••••••••",
            self.active_field == IntroSignupField::Password,
            false,
            palette,
        );
        let mut avatar_btn = div()
            .w(px(64.0))
            .h(px(64.0))
            .rounded_full()
            .bg(if self.has_avatar { rgb(palette.accent) } else { rgb(palette.input_bg) })
            .border_2()
            .border_color(rgb(palette.accent))
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .child(tg_icon(
                IconName::Camera,
                24.0,
                if self.has_avatar { 0xFFFFFF } else { palette.accent },
            ));
        let mut submit_btn = render_primary_cta(tr_str(lang_id, "完成注册", "Sign Up"), true, palette);

        if let Some(cx_ref) = cx.as_deref_mut() {
            avatar_btn = avatar_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.signup_view.toggle_avatar();
                cx.notify();
            }));
            first_box = first_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.signup_view.active_field = IntroSignupField::FirstName;
                cx.notify();
            }));
            last_box = last_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.signup_view.active_field = IntroSignupField::LastName;
                cx.notify();
            }));
            user_box = user_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.signup_view.active_field = IntroSignupField::Username;
                cx.notify();
            }));
            pass_box = pass_box.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                this.signup_view.active_field = IntroSignupField::Password;
                cx.notify();
            }));
            submit_btn = submit_btn.on_mouse_down(MouseButton::Left, cx_ref.listener(|this, _, _w, cx| {
                if this.signup_view.submit().is_ok() {
                    this.pending_action = Some(IntroAction::SubmitSignup {
                        first_name: this.signup_view.first_name.trim().to_string(),
                        last_name: this.signup_view.last_name.trim().to_string(),
                        username: this.signup_view.username.trim().to_string(),
                        password: this.signup_view.password.clone(),
                    });
                }
                cx.notify();
            }));
        }

        let name_fields = if self.invert_order {
            vec![last_box, first_box]
        } else {
            vec![first_box, last_box]
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(avatar_btn)
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(tr_str(lang_id, "完善个人资料", "Your Info")),
            )
            .children(name_fields)
            .child(user_box)
            .child(pass_box)
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(submit_btn)
            .child(
                div()
                    .max_w(px(300.0))
                    .text_size(px(11.5))
                    .text_color(rgb(palette.text_secondary))
                    .text_center()
                    .child(tr_str(
                        lang_id,
                        "点击注册即表示同意服务条款与隐私政策。",
                        "By signing up, you agree to the Terms of Service.",
                    )),
            )
            .into_any_element()
    }
}

impl Render for IntroSignup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
