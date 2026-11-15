//! Upstream Source: Telegram/SourceFiles/intro/intro_qr.cpp, intro_qr.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{render_error_banner, render_text_link, IntroAction};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use super::webauthn_common::WebauthnCommon;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use qrcode::{Color as QrColor, EcLevel, QrCode};

#[derive(Debug, Clone)]
pub struct IntroQr {
    pub step: IntroStep,
    pub token: String,
    pub expires_in_sec: u32,
    pub qr_max_size_px: f32,
    pub is_expired: bool,
    pub is_alipay: bool,
    pub attempt_id: Option<String>,
    pub auth_url: Option<String>,
}

impl Default for IntroQr {
    fn default() -> Self {
        Self::new()
    }
}

impl IntroQr {
    pub const QR_MAX_SIZE: f32 = 180.0;
    pub const DEFAULT_EXPIRES_SEC: u32 = 30;

    pub fn new() -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Qr),
            token: String::new(),
            expires_in_sec: Self::DEFAULT_EXPIRES_SEC,
            qr_max_size_px: Self::QR_MAX_SIZE,
            is_expired: false,
            is_alipay: false,
            attempt_id: None,
            auth_url: None,
        }
    }

    pub fn set_alipay_attempt(&mut self, attempt_id: impl Into<String>, auth_url: impl Into<String>) {
        self.attempt_id = Some(attempt_id.into());
        self.auth_url = Some(auth_url.into());
        self.is_alipay = true;
        self.is_expired = false;
        self.expires_in_sec = 180;
    }

    pub fn set_token(&mut self, token: impl Into<String>, expires_sec: u32) {
        self.token = token.into();
        self.expires_in_sec = expires_sec;
        self.is_expired = false;
    }

    pub fn tick_timer(&mut self) {
        if self.expires_in_sec > 0 {
            self.expires_in_sec -= 1;
            if self.expires_in_sec == 0 {
                self.is_expired = true;
            }
        }
    }

    pub fn fallback_to_phone(&self) -> IntroStepType {
        IntroStepType::Phone
    }

    pub fn qr_payload(&self) -> Option<String> {
        if self.is_alipay {
            return self.auth_url.as_ref().filter(|s| !s.trim().is_empty()).cloned();
        }
        let trimmed = self.token.trim();
        if trimmed.is_empty() {
            None
        } else if trimmed.starts_with("tg://")
            || trimmed.starts_with("http://")
            || trimmed.starts_with("https://")
        {
            Some(trimmed.to_string())
        } else {
            Some(format!("tg://login?token={trimmed}"))
        }
    }

    pub fn render_view(&self) -> AnyElement {
        let palette = TdesktopThemePalette::default();
        self.render_qr_card(&palette, "en", None)
    }

    pub fn render_interactive(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        cx: &mut Context<IntroWidget>,
    ) -> AnyElement {
        self.render_qr_card(palette, lang_id, Some(cx))
    }

    fn render_qr_card(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        mut cx: Option<&mut Context<IntroWidget>>,
    ) -> AnyElement {
        let payload = self.qr_payload();
        let qr_box = if let Some(ref text) = payload {
            if let Ok(code) = QrCode::with_error_correction_level(text.as_bytes(), EcLevel::M) {
                render_real_qr_grid(&code, palette)
            } else {
                render_waiting_qr_box(palette, lang_id)
            }
        } else {
            render_waiting_qr_box(palette, lang_id)
        };

        let mut refresh_btn = render_text_link(
            if self.is_expired || payload.is_none() {
                tr_str(lang_id, "刷新二维码", "Refresh QR Code")
            } else {
                tr_str(lang_id, "重新获取二维码", "Reload QR Code")
            },
            palette,
        );
        let mut phone_link = render_text_link(
            tr_str(lang_id, "改用手机号或账号密码登录", "Or log in using your phone number"),
            palette,
        );
        let mut passkey_link = render_text_link(
            tr_str(lang_id, "使用 Passkey 密钥登录", "Sign in with Passkey"),
            palette,
        );
        if let Some(cx_ref) = cx.as_deref_mut() {
            refresh_btn = refresh_btn.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    this.pending_action = Some(IntroAction::RefreshQr);
                    cx.notify();
                }),
            );
            phone_link = phone_link.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    this.pending_action = Some(IntroAction::OpenPhoneLogin);
                    this.show_step(IntroStepType::Phone);
                    cx.notify();
                }),
            );
            passkey_link = passkey_link.on_mouse_down(
                MouseButton::Left,
                cx_ref.listener(|this, _, _w, cx| {
                    this.pending_action = Some(IntroAction::TriggerPasskey);
                    cx.notify();
                }),
            );
        }

        let has_passkey = WebauthnCommon::new().security_key_present();
        let steps = [
            tr_str(lang_id, "1. 在手机上打开客户端", "1. Open the app on your phone"),
            tr_str(
                lang_id,
                "2. 前往 设置 > 设备 > 连接桌面设备",
                "2. Go to Settings > Devices > Link Desktop Device",
            ),
            tr_str(
                lang_id,
                "3. 扫描上方二维码确认登录",
                "3. Point your phone at this screen to confirm login",
            ),
        ];

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(14.0))
            .child(qr_box)
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(tr_str(lang_id, "扫码快速登录", "Log in by QR Code")),
            )
            .child(
                div()
                    .w(px(300.0))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(steps.into_iter().map(|line| {
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(palette.text_secondary))
                            .child(line)
                    })),
            )
            .child(render_error_banner(self.step.data.error_text.as_deref(), palette))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .child(refresh_btn)
                    .child(phone_link)
                    .when(has_passkey, |el| el.child(passkey_link)),
            )
            .into_any_element()
    }
}

fn render_real_qr_grid(code: &QrCode, palette: &TdesktopThemePalette) -> Div {
    let dim = code.width().max(1);
    let colors = code.to_colors();
    let cell = (164.0 / dim as f32).max(3.0);
    let side = cell * dim as f32;

    div()
        .relative()
        .p(px(12.0))
        .rounded(px(10.0))
        .bg(rgb(0xFFFFFF))
        .shadow_md()
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(side))
                .h(px(side))
                .flex()
                .flex_col()
                .children((0..dim).map(|y| {
                    div()
                        .flex()
                        .flex_row()
                        .children((0..dim).map(|x| {
                            let is_dark = colors[y * dim + x] == QrColor::Dark;
                            div().w(px(cell)).h(px(cell)).bg(if is_dark {
                                rgb(0x111827)
                            } else {
                                rgb(0xFFFFFF)
                            })
                        }))
                })),
        )
        .child(
            div()
                .absolute()
                .w(px(44.0))
                .h(px(44.0))
                .rounded_full()
                .bg(rgb(palette.accent))
                .border_2()
                .border_color(rgb(0xFFFFFF))
                .flex()
                .items_center()
                .justify_center()
                .child(tg_icon(IconName::BotRobot, 22.0, 0xFFFFFF)),
        )
}

fn render_waiting_qr_box(palette: &TdesktopThemePalette, lang_id: &str) -> Div {
    div()
        .w(px(188.0))
        .h(px(188.0))
        .rounded(px(10.0))
        .bg(rgb(palette.card_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(10.0))
        .p(px(16.0))
        .child(tg_icon(IconName::BotRobot, 34.0, palette.accent))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(palette.text_secondary))
                .text_center()
                .child(tr_str(lang_id, "等待服务器生成二维码...", "Waiting for QR token...")),
        )
}

impl Render for IntroQr {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
