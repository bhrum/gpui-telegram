//! Upstream Source: Telegram/SourceFiles/intro/intro_start.cpp, intro_start.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::intro_chrome::{
    render_primary_cta, render_telegram_hero_logo, render_text_link, IntroAction,
};
use super::intro_step::{IntroStep, IntroStepType};
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::TdesktopThemePalette;
use gpui::*;

#[derive(Debug, Clone)]
pub struct IntroStart {
    pub step: IntroStep,
}

impl Default for IntroStart {
    fn default() -> Self {
        Self::new()
    }
}

impl IntroStart {
    pub fn new() -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Start),
        }
    }

    pub fn submit(&mut self) -> IntroStepType {
        IntroStepType::Qr
    }

    pub fn button_text(&self) -> &'static str {
        self.step.next_button_text()
    }

    pub fn render_view(&self) -> AnyElement {
        let palette = TdesktopThemePalette::default();
        let noop: Option<fn(&MouseDownEvent, &mut Window, &mut App)> = None;
        self.render_body(&palette, "en", noop, noop, noop)
    }

    pub fn render_interactive(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        cx: &mut Context<IntroWidget>,
    ) -> AnyElement {
        let on_start = cx.listener(|this, _, _w, cx| {
            this.pending_action = Some(IntroAction::StartMessaging);
            this.show_step(IntroStepType::Qr);
            cx.notify();
        });
        let on_qr = cx.listener(|this, _, _w, cx| {
            this.pending_action = Some(IntroAction::OpenQrLogin);
            this.show_step(IntroStepType::Qr);
            cx.notify();
        });
        let on_phone = cx.listener(|this, _, _w, cx| {
            this.pending_action = Some(IntroAction::OpenPhoneLogin);
            this.show_step(IntroStepType::Phone);
            cx.notify();
        });
        self.render_body(palette, lang_id, Some(on_start), Some(on_qr), Some(on_phone))
    }

    fn render_body(
        &self,
        palette: &TdesktopThemePalette,
        lang_id: &str,
        on_start: Option<impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>,
        on_qr: Option<impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>,
        on_phone: Option<impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>,
    ) -> AnyElement {
        let cta = render_primary_cta(
            tr_str(lang_id, "开始使用", "Start Messaging"),
            true,
            palette,
        );
        let cta = if let Some(cb) = on_start {
            cta.on_mouse_down(MouseButton::Left, cb)
        } else {
            cta
        };

        let qr_link = render_text_link(
            tr_str(lang_id, "使用二维码快速登录", "Log in by QR Code"),
            palette,
        );
        let qr_link = if let Some(cb) = on_qr {
            qr_link.on_mouse_down(MouseButton::Left, cb)
        } else {
            qr_link
        };

        let phone_link = render_text_link(
            tr_str(lang_id, "使用手机号或账号密码登录", "Log in by Phone or Password"),
            palette,
        );
        let phone_link = if let Some(cb) = on_phone {
            phone_link.on_mouse_down(MouseButton::Left, cb)
        } else {
            phone_link
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0))
            .child(render_telegram_hero_logo(104.0, palette))
            .child(
                div()
                    .mt(px(4.0))
                    .text_size(px(23.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.text_primary))
                    .child("Fabushi"),
            )
            .child(
                div()
                    .max_w(px(330.0))
                    .text_size(px(14.0))
                    .text_color(rgb(palette.text_secondary))
                    .text_center()
                    .child(tr_str(
                        lang_id,
                        "欢迎使用 Fabushi 桌面端。\n快速、安全、开源的云原生即时通讯应用。",
                        "Welcome to the Fabushi desktop app.\nIt's fast, secure, and synced in real time.",
                    )),
            )
            .child(div().mt(px(8.0)).child(cta))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .child(qr_link)
                    .child(phone_link),
            )
            .into_any_element()
    }
}

impl Render for IntroStart {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
