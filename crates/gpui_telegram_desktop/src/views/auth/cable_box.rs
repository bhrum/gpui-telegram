//! Upstream Source: Telegram/SourceFiles/webauthn/cable_box.cpp, cable_box.h
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CableSheet {
    #[default]
    Qr,
    Connecting,
    Continue,
    Error,
}

#[derive(Debug, Clone, Default)]
pub struct CableBoxState {
    pub sheet: CableSheet,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CableBox {
    pub state: CableBoxState,
    pub qr_text: String,
    pub is_register: bool,
    pub bluetooth_available: bool,
    pub is_closed: bool,
    pub palette: TdesktopThemePalette,
    pub lang_id: String,
}

impl Default for CableBox {
    fn default() -> Self {
        Self::new("", true, true)
    }
}

impl CableBox {
    pub fn new(qr_text: impl Into<String>, is_register: bool, bluetooth_available: bool) -> Self {
        Self {
            state: CableBoxState::default(),
            qr_text: qr_text.into(),
            is_register,
            bluetooth_available,
            is_closed: false,
            palette: TdesktopThemePalette::default(),
            lang_id: "zh-CN".to_string(),
        }
    }

    pub fn set_sheet(&mut self, sheet: CableSheet) {
        self.state.sheet = sheet;
    }

    pub fn set_error(&mut self, err: impl Into<String>) {
        self.state.error_message = Some(err.into());
        self.state.sheet = CableSheet::Error;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for CableBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let palette = self.palette;
        let lang = &self.lang_id;
        let title = if self.is_register {
            tr_str(lang, "创建通行密钥", "Create a Passkey")
        } else {
            tr_str(lang, "使用通行密钥登录", "Sign In with a Passkey")
        };

        let content = match self.state.sheet {
            CableSheet::Qr => div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .size(px(160.0))
                        .p(px(8.0))
                        .bg(rgb(palette.composer_bg))
                        .border_1()
                        .border_color(rgb(palette.border_fg))
                        .rounded(px(8.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .size(px(144.0))
                                .bg(rgb(palette.dialogs_bg))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(tg_icon(IconName::Lock, 40.0, palette.accent)),
                        ),
                )
                .child(
                    div()
                        .mt(px(12.0))
                        .text_xs()
                        .text_color(rgb(palette.text_secondary))
                        .child(tr_str(
                            lang,
                            "请使用手机相机扫码以验证通行密钥",
                            "Scan with your phone's camera to authenticate",
                        )),
                ),
            CableSheet::Connecting => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(palette.accent))
                .child(tr_str(lang, "正在连接手机…", "Connecting to phone...")),
            CableSheet::Continue => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(palette.success))
                .child(tr_str(
                    lang,
                    "请在手机上确认生物识别验证",
                    "Please confirm biometric on your phone",
                )),
            CableSheet::Error => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(palette.danger))
                .child(self.state.error_message.clone().unwrap_or_else(|| {
                    tr_str(lang, "通行密钥验证失败", "Authentication failed").to_string()
                })),
        };

        let btn_hover = rgb(palette.dialogs_hover_bg);
        div()
            .flex()
            .flex_col()
            .items_center()
            .w(px(340.0))
            .p(px(20.0))
            .rounded(px(10.0))
            .bg(rgb(palette.modal_bg))
            .border_1()
            .border_color(rgb(palette.border_fg))
            .shadow_xl()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(title),
            )
            .child(
                div()
                    .my(px(16.0))
                    .w_full()
                    .flex()
                    .justify_center()
                    .child(content),
            )
            .child(
                div()
                    .w_full()
                    .h(px(36.0))
                    .rounded(px(6.0))
                    .bg(rgb(palette.dialogs_bg))
                    .hover(move |s| s.bg(btn_hover))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(palette.text_primary))
                            .child(tr_str(
                                lang,
                                "改用硬件安全密钥",
                                "USE SECURITY KEY INSTEAD",
                            )),
                    ),
            )
    }
}
