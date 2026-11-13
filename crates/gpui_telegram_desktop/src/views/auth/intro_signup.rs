//! Upstream Source: Telegram/SourceFiles/intro/intro_signup.cpp, intro_signup.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroSignup {
    pub step: IntroStep,
    pub first_name: String,
    pub last_name: String,
    pub invert_order: bool,
    pub has_avatar: bool,
    pub terms_accepted: bool,
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
            invert_order,
            has_avatar: false,
            terms_accepted: true,
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
            self.step.show_error("Please enter your first name");
            return Err("First name required");
        }
        self.step.hide_error();
        Ok(())
    }

    pub fn render_view(&self) -> AnyElement {
        let first_text = if self.first_name.is_empty() {
            "First name (required)".to_string()
        } else {
            self.first_name.clone()
        };

        let last_text = if self.last_name.is_empty() {
            "Last name (optional)".to_string()
        } else {
            self.last_name.clone()
        };

        let first_input = div()
            .w(px(300.0))
            .h(px(42.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(rgb(0x17212b))
            .border_1()
            .border_color(rgb(0x242f3d))
            .flex()
            .items_center()
            .text_sm()
            .text_color(rgb(0xffffff))
            .child(first_text);

        let last_input = div()
            .w(px(300.0))
            .h(px(42.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(rgb(0x17212b))
            .border_1()
            .border_color(rgb(0x242f3d))
            .flex()
            .items_center()
            .text_sm()
            .text_color(rgb(0xffffff))
            .child(last_text);

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                // Round Userpic Avatar Button
                div()
                    .size(px(80.0))
                    .rounded_full()
                    .bg(rgb(0x2aabee))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_2xl()
                    .text_color(rgb(0xffffff))
                    .child(if self.has_avatar { "👤" } else { "📷" }),
            )
            .child(
                div()
                    .mt(px(20.0))
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Your Name"),
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .child("Enter your name and add a profile photo."),
            )
            .child(
                div()
                    .mt(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children(if self.invert_order {
                        vec![last_input, first_input]
                    } else {
                        vec![first_input, last_input]
                    }),
            )
            .child(
                // Terms of service note
                div()
                    .mt(px(16.0))
                    .text_xs()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(300.0))
                    .child("By signing up, you agree to the Terms of Service."),
            )
            .child(
                // Next / Finish Button
                div()
                    .mt(px(24.0))
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
                            .child("SIGN UP"),
                    ),
            )
            .into_any_element()
    }
}

impl Render for IntroSignup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
