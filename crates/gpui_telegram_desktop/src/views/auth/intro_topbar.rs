use super::intro_chrome::IntroAction;
use super::intro_email::IntroEmailField;
use super::intro_phone::IntroPhoneField;
use super::intro_signup::IntroSignupField;
use super::intro_step::IntroStepType;
use super::intro_widget::IntroWidget;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName};
use gpui::prelude::FluentBuilder as _;
use gpui::*;

pub fn render_intro_top_bar(widget: &IntroWidget, cx: &mut Context<IntroWidget>) -> AnyElement {
    let palette = widget.palette.clone();
    let lang = widget.language_code.as_str();
    let is_dark = palette.mode.is_dark();
    let can_back = widget.can_go_back();
    let editing = widget.server_url_editing;
    let connected = widget.server_connected;
    let server_display = if editing {
        format!("{}|", widget.server_url)
    } else if widget.server_url.is_empty() {
        "http://127.0.0.1:8080".to_string()
    } else {
        widget.server_url.clone()
    };

    div()
        .w_full()
        .h(px(56.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .justify_between()
        .child(if can_back {
            div()
                .w(px(36.0))
                .h(px(36.0))
                .rounded_full()
                .hover(|s| s.bg(rgb(palette.dialogs_hover_bg)))
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _w, cx| {
                        this.step_back();
                        cx.notify();
                    }),
                )
                .child(tg_icon(IconName::BackArrow, 18.0, palette.icon_fg))
        } else {
            div().w(px(36.0)).h(px(36.0))
        })
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                // Editable Server URL chip with live connectivity indicator
                .child(
                    div()
                        .h(px(32.0))
                        .px(px(10.0))
                        .rounded_full()
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(if editing {
                            rgb(palette.accent)
                        } else {
                            rgb(palette.border_fg)
                        })
                        .flex()
                        .items_center()
                        .gap(px(7.0))
                        .child(
                            div()
                                .cursor_pointer()
                                .w(px(8.0))
                                .h(px(8.0))
                                .rounded_full()
                                .bg(if connected {
                                    rgb(palette.online_dot_fg)
                                } else {
                                    rgb(0xF59E0B)
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _w, cx| {
                                        this.pending_action =
                                            Some(IntroAction::CheckServerConnectivity);
                                        cx.notify();
                                    }),
                                ),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .text_color(rgb(palette.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _w, cx| {
                                        this.server_url_editing = !this.server_url_editing;
                                        if !this.server_url_editing {
                                            this.pending_action = Some(IntroAction::SaveServerUrl(
                                                this.server_url.clone(),
                                            ));
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(server_display),
                        )
                        .when(editing, |el| {
                            el.child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .bg(rgb(palette.accent))
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _w, cx| {
                                            this.server_url_editing = false;
                                            this.pending_action = Some(IntroAction::SaveServerUrl(
                                                this.server_url.clone(),
                                            ));
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "保存", "Save")),
                            )
                        }),
                )
                // Language Switcher
                .child(
                    div()
                        .h(px(32.0))
                        .px(px(10.0))
                        .rounded_full()
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(rgb(palette.border_fg))
                        .hover(|s| s.bg(rgb(palette.dialogs_hover_bg)))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .gap(px(5.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(palette.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _w, cx| {
                                let next = if this.language_code.starts_with("zh") {
                                    "en"
                                } else {
                                    "zh-CN"
                                };
                                this.switch_language(next);
                                this.pending_action =
                                    Some(IntroAction::SwitchLanguage(next.to_string()));
                                cx.notify();
                            }),
                        )
                        .child(tg_icon(IconName::Translate, 14.0, palette.icon_fg))
                        .child(if lang.starts_with("zh") { "中文" } else { "EN" }),
                )
                // Day / Night Theme Toggle
                .child(
                    div()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(rgb(palette.border_fg))
                        .hover(|s| s.bg(rgb(palette.dialogs_hover_bg)))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _w, cx| {
                                this.toggle_theme_mode();
                                this.pending_action = Some(IntroAction::ToggleTheme);
                                cx.notify();
                            }),
                        )
                        .child(tg_icon(
                            if is_dark { IconName::SunDay } else { IconName::NightMoon },
                            15.0,
                            palette.icon_fg,
                        )),
                ),
        )
        .into_any_element()
}

fn mutate_text(buf: &mut String, ch: Option<&str>, backspace: bool) {
    if backspace {
        buf.pop();
    } else if let Some(s) = ch {
        buf.push_str(s);
    }
}

pub fn apply_widget_key_input(
    w: &mut IntroWidget,
    key: &str,
    key_char: Option<&str>,
    backspace: bool,
) {
    if w.server_url_editing {
        if key == "enter" || key == "Enter" {
            w.server_url_editing = false;
            w.pending_action = Some(IntroAction::SaveServerUrl(w.server_url.clone()));
        } else {
            mutate_text(&mut w.server_url, key_char, backspace);
        }
        return;
    }

    match w.current_step {
        IntroStepType::Phone => match w.phone_view.active_field {
            IntroPhoneField::Phone | IntroPhoneField::CountryCode => {
                if backspace {
                    let mut cur = w.phone_view.phone_number.clone();
                    cur.pop();
                    w.phone_view.phone_number = cur;
                } else if let Some(s) = key_char {
                    let mut next = w.phone_view.phone_number.clone();
                    next.extend(s.chars().filter(|c| c.is_ascii_digit()));
                    w.phone_view.on_phone_input(next);
                }
            }
            IntroPhoneField::Username => mutate_text(&mut w.phone_view.username, key_char, backspace),
            IntroPhoneField::Password => mutate_text(&mut w.phone_view.password, key_char, backspace),
        },
        IntroStepType::Code => {
            if backspace {
                w.code_view.input.pop_digit();
            } else if let Some(s) = key_char {
                for ch in s.chars().filter(|c| c.is_ascii_digit()) {
                    w.code_view.input.push_digit(ch);
                }
            }
        }
        IntroStepType::Email | IntroStepType::EmailRegister => match w.email_view.active_field {
            IntroEmailField::Email => mutate_text(&mut w.email_view.email, key_char, backspace),
            IntroEmailField::Code => {
                mutate_text(&mut w.email_view.verification_code, key_char, backspace)
            }
        },
        IntroStepType::PasswordCheck => {
            if w.password_check_view.is_recovering {
                mutate_text(&mut w.password_check_view.recovery_code, key_char, backspace);
            } else {
                mutate_text(&mut w.password_check_view.password, key_char, backspace);
            }
        }
        IntroStepType::Signup => match w.signup_view.active_field {
            IntroSignupField::FirstName => {
                mutate_text(&mut w.signup_view.first_name, key_char, backspace)
            }
            IntroSignupField::LastName => {
                mutate_text(&mut w.signup_view.last_name, key_char, backspace)
            }
            IntroSignupField::Username => {
                mutate_text(&mut w.signup_view.username, key_char, backspace)
            }
            IntroSignupField::Password => {
                mutate_text(&mut w.signup_view.password, key_char, backspace)
            }
        },
        IntroStepType::Start | IntroStepType::Qr | IntroStepType::AlipayQr => {}
    }
}
