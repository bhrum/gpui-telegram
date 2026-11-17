//! Upstream Source: Telegram/SourceFiles/intro/intro_widget.cpp, intro_widget.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::IntroAction;
use super::intro_code::IntroCode;
use super::intro_email::IntroEmail;
use super::intro_password_check::IntroPasswordCheck;
use super::intro_phone::IntroPhone;
use super::intro_qr::IntroQr;
use super::intro_signup::IntroSignup;
use super::intro_start::IntroStart;
use super::intro_step::{IntroStepData, IntroStepType};
use super::intro_topbar::{apply_widget_key_input, render_intro_top_bar};
use crate::i18n::tr_str;
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroEnterPoint {
    #[default]
    Start,
    Qr,
    Phone,
    AddAccount,
}

#[derive(Debug, Clone)]
pub struct IntroWidget {
    pub current_step: IntroStepType,
    pub history: Vec<IntroStepType>,
    pub data: IntroStepData,
    pub language_code: String,
    pub is_settings_open: bool,
    pub palette: TdesktopThemePalette,
    pub server_url: String,
    pub server_url_editing: bool,
    pub server_connected: bool,
    pub server_status_message: String,
    pub pending_action: Option<IntroAction>,

    pub start_view: IntroStart,
    pub qr_view: IntroQr,
    pub phone_view: IntroPhone,
    pub code_view: IntroCode,
    pub signup_view: IntroSignup,
    pub password_check_view: IntroPasswordCheck,
    pub email_view: IntroEmail,
}

impl Default for IntroWidget {
    fn default() -> Self {
        Self::new(IntroEnterPoint::Start)
    }
}

impl IntroWidget {
    pub fn new(point: IntroEnterPoint) -> Self {
        let initial_step = match point {
            IntroEnterPoint::Start => IntroStepType::Start,
            IntroEnterPoint::Qr => IntroStepType::Qr,
            IntroEnterPoint::Phone | IntroEnterPoint::AddAccount => IntroStepType::Phone,
        };
        let phone_view = IntroPhone::new();

        Self {
            current_step: initial_step,
            history: Vec::new(),
            data: IntroStepData::default(),
            language_code: "en".to_string(),
            is_settings_open: false,
            palette: TdesktopThemePalette::default(),
            server_url: "http://127.0.0.1:8080".to_string(),
            server_url_editing: false,
            server_connected: false,
            server_status_message: String::new(),
            pending_action: None,
            start_view: IntroStart::new(),
            qr_view: IntroQr::new(),
            phone_view,
            code_view: IntroCode::default(),
            signup_view: IntroSignup::default(),
            password_check_view: IntroPasswordCheck::default(),
            email_view: IntroEmail::default(),
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn show_step(&mut self, next: IntroStepType) {
        if next != self.current_step {
            self.history.push(self.current_step);
            self.current_step = next;
        }
    }

    pub fn step_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_step = prev;
            true
        } else {
            false
        }
    }

    pub fn switch_language(&mut self, lang: impl Into<String>) {
        self.language_code = lang.into();
    }

    pub fn toggle_settings(&mut self) {
        self.is_settings_open = !self.is_settings_open;
    }

    pub fn toggle_theme_mode(&mut self) {
        let next_mode = if self.palette.mode.is_dark() {
            TdesktopThemeMode::Day
        } else {
            TdesktopThemeMode::Night
        };
        self.palette = TdesktopThemePalette::build(
            next_mode,
            next_mode.default_accent(),
            self.palette.font_size_px,
            self.palette.bubble_radius_px,
        );
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_server_status(&mut self, url: impl Into<String>, connected: bool, message: impl Into<String>) {
        self.server_url = url.into();
        self.server_connected = connected;
        self.server_status_message = message.into();
    }

    pub fn take_action(&mut self) -> Option<IntroAction> {
        self.pending_action.take()
    }

    pub fn handle_key_input(&mut self, key: &str, key_char: Option<&str>, backspace: bool) {
        apply_widget_key_input(self, key, key_char, backspace);
    }
}

impl Render for IntroWidget {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = self.palette.clone();
        let lang = self.language_code.clone();

        let step_view: AnyElement = match self.current_step {
            IntroStepType::Start => self.start_view.render_interactive(&palette, &lang, cx),
            IntroStepType::Qr | IntroStepType::AlipayQr => {
                self.qr_view.render_interactive(&palette, &lang, cx)
            }
            IntroStepType::Phone => self.phone_view.render_interactive(&palette, &lang, cx),
            IntroStepType::Code => self.code_view.render_interactive(&palette, &lang, cx),
            IntroStepType::Signup => self.signup_view.render_interactive(&palette, &lang, cx),
            IntroStepType::PasswordCheck => {
                self.password_check_view.render_interactive(&palette, &lang, cx)
            }
            IntroStepType::Email | IntroStepType::EmailRegister => {
                self.email_view.render_interactive(&palette, &lang, cx)
            }
        };

        div()
            .w_full()
            .h_full()
            .bg(rgb(palette.window_bg))
            .flex()
            .flex_col()
            .justify_between()
            .child(render_intro_top_bar(self, cx))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(24.0))
                    .child(
                        div()
                            .w(px(380.0))
                            .p(px(28.0))
                            .rounded(px(14.0))
                            .bg(rgb(palette.card_bg))
                            .border_1()
                            .border_color(rgb(palette.border_fg))
                            .shadow_md()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(step_view),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .pb(px(16.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(11.5))
                    .text_color(rgb(palette.text_secondary))
                    .child(if self.server_status_message.is_empty() {
                        tr_str(&lang, "Fabushi • 端到端云原生桌面客户端", "Fabushi • Native Desktop Client")
                            .to_string()
                    } else {
                        self.server_status_message.clone()
                    }),
            )
    }
}
