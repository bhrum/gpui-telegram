//! Upstream Source: Telegram/SourceFiles/intro/intro_start.cpp, intro_start.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroStart {
    pub step: IntroStep,
    pub is_hovered: bool,
    pub cta_clicked: bool,
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
            is_hovered: false,
            cta_clicked: false,
        }
    }

    pub fn submit(&mut self) -> IntroStepType {
        self.cta_clicked = true;
        IntroStepType::Qr
    }

    pub fn button_text(&self) -> &'static str {
        "Start Messaging"
    }

    pub fn render_view(&self) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                // Telegram animated paper plane badge
                div()
                    .size(px(120.0))
                    .rounded_full()
                    .bg(rgb(0x2aabee))
                    .flex()
                    .items_center()
                    .justify_center()
                    .shadow_lg()
                    .child(
                        div()
                            .text_size(px(52.0))
                            .child("✈"),
                    ),
            )
            .child(
                div()
                    .mt(px(28.0))
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Telegram Desktop"),
            )
            .child(
                div()
                    .mt(px(10.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(320.0))
                    .child("The world's fastest messaging app. Free and secure."),
            )
            .child(
                // "Start Messaging" CTA button (introNextButton: 300x42px)
                div()
                    .mt(px(36.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2aabee))
                    .hover(|s| s.bg(rgb(0x229ed9)))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.button_text()),
                    ),
            )
            .into_any_element()
    }
}

impl Render for IntroStart {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
