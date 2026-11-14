use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::*;

/// User actions emitted by `IntroWidget` and consumed by `AppShell`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntroAction {
    StartMessaging,
    OpenQrLogin,
    OpenPhoneLogin,
    RefreshQr,
    TriggerPasskey,
    SubmitPhone {
        phone: String,
    },
    SubmitCredentials {
        username: String,
        password: String,
    },
    SubmitCode {
        code: String,
    },
    ResendCode,
    SendEmailCode {
        email: String,
    },
    SubmitEmail {
        email: String,
        code: String,
    },
    SubmitPassword {
        password: String,
    },
    SubmitRecoverCode {
        code: String,
    },
    SubmitSignup {
        first_name: String,
        last_name: String,
        username: String,
        password: String,
    },
    ToggleTheme,
    SwitchLanguage(String),
    SaveServerUrl(String),
    CheckServerConnectivity,
}

pub fn render_telegram_hero_logo(size: f32, palette: &TdesktopThemePalette) -> Div {
    div()
        .w(px(size))
        .h(px(size))
        .rounded_full()
        .bg(rgb(palette.accent))
        .shadow_md()
        .flex()
        .items_center()
        .justify_center()
        .child(tg_icon(
            IconName::TelegramPlaneLogo,
            size * 0.54,
            0xFFFFFF,
        ))
}

pub fn render_primary_cta(
    label: impl Into<String>,
    enabled: bool,
    palette: &TdesktopThemePalette,
) -> Div {
    let label_str = label.into();
    let bg = if enabled {
        rgb(palette.accent)
    } else {
        rgb(palette.muted_badge_bg)
    };
    let hover_bg = if enabled {
        rgb(palette.accent_hover)
    } else {
        rgb(palette.muted_badge_bg)
    };

    div()
        .w(px(300.0))
        .h(px(42.0))
        .rounded(px(6.0))
        .bg(bg)
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(14.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(palette.active_button_fg))
        .child(label_str)
}

pub fn render_text_link(label: impl Into<String>, palette: &TdesktopThemePalette) -> Div {
    div()
        .cursor_pointer()
        .px(px(6.0))
        .py(px(4.0))
        .rounded(px(4.0))
        .hover(|s| s.bg(rgb(palette.dialogs_hover_bg)))
        .text_size(px(13.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(palette.text_link))
        .child(label.into())
}

pub fn render_input_box(
    label: &str,
    value_display: String,
    placeholder: &str,
    focused: bool,
    has_error: bool,
    palette: &TdesktopThemePalette,
) -> Div {
    let border_col = if has_error {
        rgb(palette.danger)
    } else if focused {
        rgb(palette.accent)
    } else {
        rgb(palette.border_fg)
    };
    let label_col = if has_error {
        rgb(palette.danger)
    } else if focused {
        rgb(palette.accent)
    } else {
        rgb(palette.text_secondary)
    };
    let is_empty = value_display.is_empty();
    let text_col = if is_empty {
        rgb(palette.text_secondary)
    } else {
        rgb(palette.text_primary)
    };

    div()
        .w(px(300.0))
        .px(px(12.0))
        .py(px(8.0))
        .rounded(px(8.0))
        .bg(rgb(palette.input_bg))
        .border_1()
        .border_color(border_col)
        .cursor_pointer()
        .flex()
        .flex_col()
        .gap(px(3.0))
        .child(
            div()
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(label_col)
                .child(label.to_string()),
        )
        .child(
            div()
                .h(px(22.0))
                .flex()
                .items_center()
                .text_size(px(14.5))
                .text_color(text_col)
                .child(if is_empty {
                    if focused {
                        format!("| {}", placeholder)
                    } else {
                        placeholder.to_string()
                    }
                } else if focused {
                    format!("{}|", value_display)
                } else {
                    value_display
                }),
        )
}

pub fn render_error_banner(err: Option<&str>, palette: &TdesktopThemePalette) -> AnyElement {
    match err {
        Some(msg) if !msg.is_empty() => div()
            .w(px(300.0))
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .bg(rgba(0xE5393518))
            .border_1()
            .border_color(rgb(palette.danger))
            .text_size(px(12.5))
            .text_color(rgb(palette.danger))
            .child(msg.to_string())
            .into_any_element(),
        _ => div().h(px(0.0)).into_any_element(),
    }
}
