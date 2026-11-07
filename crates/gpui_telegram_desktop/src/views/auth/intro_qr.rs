//! Upstream Source: Telegram/SourceFiles/intro/intro_qr.cpp, intro_qr.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroQr {
    pub step: IntroStep,
    pub token: String,
    pub expires_in_sec: u32,
    pub qr_max_size_px: f32,
    pub is_expired: bool,
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
            token: "tg://login?token=AQAAAJ1234567890".to_string(),
            expires_in_sec: Self::DEFAULT_EXPIRES_SEC,
            qr_max_size_px: Self::QR_MAX_SIZE,
            is_expired: false,
        }
    }

    pub fn set_token(&mut self, token: impl Into<String>, expires_sec: u32) {
        self.token = token.into();
        self.expires_in_sec = expires_sec;
        self.is_expired = false;
    }

    pub fn tick_timer(&mut self) {
        if self.expires_in_sec > 0 {
            self.expires_in_sec -= 1;
        } else {
            self.is_expired = true;
        }
    }

    pub fn fallback_to_phone(&self) -> IntroStepType {
        IntroStepType::Phone
    }

    pub fn render_view(&self) -> AnyElement {
        let timer_color = if self.expires_in_sec <= 5 {
            rgb(0xee5655)
        } else {
            rgb(0x8292a1)
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                // QR code matrix placeholder with Telegram center logo
                div()
                    .size(px(self.qr_max_size_px + 24.0))
                    .p(px(12.0))
                    .bg(rgb(0xffffff))
                    .rounded(px(8.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .size(px(self.qr_max_size_px))
                            .bg(rgb(0x17212b))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .size(px(44.0))
                                    .rounded_full()
                                    .bg(rgb(0x2aabee))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xffffff))
                                    .child("✈"),
                            ),
                    ),
            )
            .child(
                div()
                    .mt(px(18.0))
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Log in to Telegram by QR Code"),
            )
            .child(
                // 3 instructions steps
                div()
                    .mt(px(14.0))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .max_w(px(292.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x8292a1))
                            .child("1. Open Telegram on your phone"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x8292a1))
                            .child("2. Go to Settings > Devices > Link Desktop"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x8292a1))
                            .child("3. Point your phone at this screen"),
                    ),
            )
            .child(
                div()
                    .mt(px(12.0))
                    .text_xs()
                    .text_color(timer_color)
                    .child(format!("Code expires in {}s", self.expires_in_sec)),
            )
            .child(
                // Fallback link: "LOG IN BY PHONE NUMBER"
                div()
                    .mt(px(24.0))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x2aabee))
                    .hover(|s| s.text_color(rgb(0x54c0f5)))
                    .cursor_pointer()
                    .child("LOG IN BY PHONE NUMBER"),
            )
            .into_any_element()
    }
}

impl Render for IntroQr {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
