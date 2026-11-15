//! Standalone `AuthWizardView`: handles unauthenticated onboarding, password login/register,
//! server URL editing, Alipay QR login, email OTP registration, focus, and IME integration.

use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::TelegramClient;

use crate::components::ui::TextInputController;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

use super::auth_wizard_state::{
    has_network_error, AuthWizardEvent, AuthWizardTab, LoginInputField, LoginWizardStep,
};

pub(super) struct WizardInputFieldSpec<'a> {
    pub field: LoginInputField,
    pub value: &'a str,
    pub is_active: bool,
    pub is_password: bool,
    pub active_placeholder: &'a str,
    pub idle_placeholder: &'a str,
    pub trailing_icon: Option<IconName>,
    pub allow_clear: bool,
    pub flex_grow: bool,
}

/// Self-contained unauthenticated login/registration wizard view.
pub struct AuthWizardView {
    pub client: Arc<TelegramClient>,
    pub focus_handle: FocusHandle,
    pub palette: TdesktopThemePalette,
    pub is_chinese_locale: bool,
    pub login_step: LoginWizardStep,
    pub auth_wizard_tab: AuthWizardTab,
    pub active_login_field: LoginInputField,
    pub is_register_mode: bool,
    pub is_authenticating: bool,
    pub editing_server_url: bool,
    pub server_url_draft: String,
    pub username_input: String,
    pub password_input: String,
    pub first_name_input: String,
    pub email_input: String,
    pub email_code_input: String,
    pub email_code_cooldown: u32,
    pub email_cooldown_generation: u64,
    pub is_sending_email_code: bool,
    pub alipay_attempt_id: Option<String>,
    pub alipay_auth_url: Option<String>,
    pub alipay_qr_payload: Option<String>,
    pub alipay_status_text: String,
    pub alipay_is_loading: bool,
    pub alipay_poll_generation: u64,
    pub login_error: Option<String>,
    pub ime: TextInputController,
    pub country_code: String,
    pub phone_number: String,
    pub verification_code: String,
    pub is_authenticated: bool,
}

impl EventEmitter<AuthWizardEvent> for AuthWizardView {}

impl AuthWizardView {
    pub fn new(
        client: Arc<TelegramClient>,
        palette: TdesktopThemePalette,
        is_chinese_locale: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let server_url_draft = client.config.server_url.clone();
        Self {
            client,
            focus_handle: cx.focus_handle(),
            palette,
            is_chinese_locale,
            login_step: LoginWizardStep::Start,
            auth_wizard_tab: AuthWizardTab::PasswordLogin,
            active_login_field: LoginInputField::Username,
            is_register_mode: false,
            is_authenticating: false,
            editing_server_url: false,
            server_url_draft,
            username_input: String::new(),
            password_input: String::new(),
            first_name_input: String::new(),
            email_input: String::new(),
            email_code_input: String::new(),
            email_code_cooldown: 0,
            email_cooldown_generation: 0,
            is_sending_email_code: false,
            alipay_attempt_id: None,
            alipay_auth_url: None,
            alipay_qr_payload: None,
            alipay_status_text: String::new(),
            alipay_is_loading: false,
            alipay_poll_generation: 0,
            login_error: None,
            ime: TextInputController::new(),
            country_code: "+86".to_string(),
            phone_number: String::new(),
            verification_code: String::new(),
            is_authenticated: false,
        }
    }

    pub fn active_lang_id(&self) -> &'static str {
        if self.is_chinese_locale {
            "zh-CN"
        } else {
            "en"
        }
    }

    pub fn apply_theme_and_locale(
        &mut self,
        palette: TdesktopThemePalette,
        is_chinese_locale: bool,
    ) {
        self.palette = palette;
        self.is_chinese_locale = is_chinese_locale;
    }

    pub fn reset_after_logout(&mut self, server_url: String) {
        if self.client.config.server_url != server_url {
            let mut new_cfg = self.client.config.clone();
            new_cfg.server_url = server_url.clone();
            self.client = Arc::new(TelegramClient::new(new_cfg));
        }
        self.is_authenticated = false;
        self.is_authenticating = false;
        self.login_step = LoginWizardStep::Start;
        self.verification_code.clear();
        self.login_error = None;
        self.ime.clear_preedit();
        self.ime.just_committed_ime = false;
        self.editing_server_url = false;
        self.server_url_draft = server_url;
        self.alipay_poll_generation = self.alipay_poll_generation.wrapping_add(1);
        self.email_cooldown_generation = self.email_cooldown_generation.wrapping_add(1);
        self.alipay_is_loading = false;
        self.is_sending_email_code = false;
    }

    pub fn set_login_error(&mut self, err: Option<String>) {
        self.login_error = err;
    }

    pub(super) fn render_wizard_input_field(
        &self,
        spec: WizardInputFieldSpec<'_>,
        cx: &mut Context<Self>,
    ) -> Div {
        let pal = self.palette;
        let field = spec.field;
        let preedit = if spec.is_active {
            self.ime.ime_preedit.clone()
        } else {
            None
        };
        let is_empty = spec.value.is_empty() && preedit.is_none();
        let display_val = if spec.is_password {
            "•".repeat(spec.value.chars().count())
        } else {
            spec.value.to_string()
        };
        let active_ph = spec.active_placeholder.to_string();
        let idle_ph = spec.idle_placeholder.to_string();

        let base = if spec.flex_grow {
            div().flex_1()
        } else {
            div().w_full()
        };

        base.cursor_pointer()
            .h(px(40.0))
            .rounded(px(8.0))
            .bg(rgb(pal.input_bg))
            .border_1()
            .border_color(if spec.is_active {
                rgb(pal.accent)
            } else {
                rgb(pal.border_fg)
            })
            .px(px(12.0))
            .flex()
            .items_center()
            .justify_between()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    this.ime.clear_preedit();
                    this.active_login_field = field;
                    this.focus_handle.focus(window);
                    cx.notify();
                }),
            )
            .child(if is_empty {
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(pal.text_secondary))
                    .child(if spec.is_active { active_ph } else { idle_ph })
            } else {
                div()
                    .flex()
                    .items_center()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_primary))
                            .child(display_val),
                    )
                    .when_some(preedit, |p_el, p| {
                        p_el.child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(pal.accent))
                                .border_b_1()
                                .border_color(rgb(pal.accent))
                                .child(p),
                        )
                    })
                    .when(spec.is_active, |c_el| {
                        c_el.child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(pal.accent))
                                .child("|"),
                        )
                    })
            })
            .when(spec.allow_clear || spec.trailing_icon.is_some(), |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .when(spec.allow_clear && !spec.value.is_empty(), |c_el| {
                            c_el.child(
                                div()
                                    .cursor_pointer()
                                    .p(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, window, cx| {
                                            this.clear_specific_login_field(field);
                                            this.ime.clear_preedit();
                                            this.active_login_field = field;
                                            this.focus_handle.focus(window);
                                            cx.stop_propagation();
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                            )
                        })
                        .when_some(spec.trailing_icon, |t_el, icon| {
                            t_el.child(tg_icon(icon, 13.0, pal.text_secondary))
                        }),
                )
            })
    }

    fn render_top_controls(&self, cx: &mut Context<Self>) -> Div {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let is_dark = pal.mode.is_dark();

        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_secondary))
                    .child("AI Chat • MTProto 2.0"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|_this, _event, _window, cx| {
                                    cx.emit(AuthWizardEvent::ToggleTheme);
                                }),
                            )
                            .child(tg_icon(
                                if is_dark {
                                    IconName::SunDay
                                } else {
                                    IconName::NightMoon
                                },
                                13.0,
                                pal.icon_fg,
                            ))
                            .child(if is_dark {
                                tr_str(lang, "日间", "Day")
                            } else {
                                tr_str(lang, "夜间", "Night")
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|_this, _event, _window, cx| {
                                    cx.emit(AuthWizardEvent::ToggleLanguage);
                                }),
                            )
                            .child(tg_icon(IconName::Translate, 13.0, pal.icon_fg))
                            .child(tr_str(lang, "中文 / EN", "EN / 中文")),
                    ),
            )
    }

    fn render_server_url_bar(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let server_url = self.client.config.server_url.clone();
        let has_net_err = has_network_error(self.login_error.as_deref());
        let dot_color = if has_net_err {
            rgb(pal.danger)
        } else {
            rgb(pal.success)
        };

        if self.editing_server_url {
            let draft_display = if self.server_url_draft.is_empty() {
                "http://<Google-Cloud-IP>:8080".to_string()
            } else {
                format!("{}|", self.server_url_draft)
            };
            div()
                .w_full()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .flex_1()
                        .h(px(30.0))
                        .px(px(10.0))
                        .rounded(px(6.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(pal.accent))
                        .flex()
                        .items_center()
                        .text_size(px(11.5))
                        .text_color(if self.server_url_draft.is_empty() {
                            rgb(pal.text_secondary)
                        } else {
                            rgb(pal.text_primary)
                        })
                        .child(draft_display),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .h(px(30.0))
                        .px(px(10.0))
                        .rounded(px(6.0))
                        .bg(rgb(pal.accent))
                        .hover(move |s| s.bg(rgb(pal.accent_hover)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.apply_custom_server_url(cx);
                            }),
                        )
                        .child(tr_str(lang, "保存", "Save")),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .h(px(30.0))
                        .px(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(pal.border_fg))
                        .hover(move |s| s.bg(rgb(pal.hover_bg)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(rgb(pal.text_secondary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.editing_server_url = false;
                                this.server_url_draft = this.client.config.server_url.clone();
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "取消", "Cancel")),
                )
                .into_any_element()
        } else {
            div()
                .cursor_pointer()
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(4.0))
                .rounded(px(6.0))
                .bg(rgb(pal.input_bg))
                .border_1()
                .border_color(if has_net_err {
                    rgb(pal.danger)
                } else {
                    rgb(pal.border_fg)
                })
                .hover(move |s| s.border_color(rgb(pal.accent)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.server_url_draft = this.client.config.server_url.clone();
                        this.editing_server_url = true;
                        cx.notify();
                    }),
                )
                .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(dot_color))
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(pal.text_secondary))
                        .child(format!("Cloud VM: {}", server_url)),
                )
                .child(
                    div()
                        .text_size(px(10.5))
                        .text_color(rgb(pal.accent))
                        .child(tr_str(lang, "修改", "Edit")),
                )
                .into_any_element()
        }
    }

    fn render_tab_switcher(&self, cx: &mut Context<Self>) -> Div {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let tab_btn = |tab: AuthWizardTab,
                       active_bg: u32,
                       icon: IconName,
                       label: &'static str,
                       cx: &mut Context<Self>| {
            let is_active = self.auth_wizard_tab == tab;
            div()
                .cursor_pointer()
                .flex_1()
                .h(px(32.0))
                .rounded(px(6.0))
                .bg(if is_active {
                    rgb(active_bg)
                } else {
                    rgb(pal.input_bg)
                })
                .hover(move |s| {
                    if !is_active {
                        s.bg(rgb(pal.hover_bg))
                    } else {
                        s
                    }
                })
                .flex()
                .items_center()
                .justify_center()
                .gap(px(5.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(if is_active {
                    rgb(pal.active_button_fg)
                } else {
                    rgb(pal.text_primary)
                })
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.switch_auth_tab(tab, cx);
                    }),
                )
                .child(tg_icon(
                    icon,
                    12.0,
                    if is_active {
                        pal.active_button_fg
                    } else {
                        pal.icon_fg
                    },
                ))
                .child(label)
        };

        div()
            .w_full()
            .flex()
            .items_center()
            .bg(rgb(pal.input_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .rounded(px(8.0))
            .p(px(3.0))
            .gap(px(4.0))
            .child(tab_btn(
                AuthWizardTab::PasswordLogin,
                pal.accent,
                IconName::Lock,
                tr_str(lang, "密码登录", "Password"),
                cx,
            ))
            .child(tab_btn(
                AuthWizardTab::AlipayQr,
                0x1677FF,
                IconName::WalletStars,
                tr_str(lang, "支付宝扫码", "Alipay QR"),
                cx,
            ))
            .child(tab_btn(
                AuthWizardTab::EmailRegister,
                pal.accent,
                IconName::SendPlane,
                tr_str(lang, "邮箱注册", "Email Register"),
                cx,
            ))
    }

    fn render_password_login_tab(&self, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let is_reg = self.is_register_mode;
        let is_loading = self.is_authenticating;

        div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(if is_reg {
                        tr_str(lang, "注册云端账号", "Create Cloud Account")
                    } else {
                        tr_str(lang, "登录云端服务", "Sign In to Cloud Server")
                    }),
            )
            .when(is_reg, |el| {
                el.child(self.render_wizard_input_field(
                    WizardInputFieldSpec {
                        field: LoginInputField::FirstName,
                        value: &self.first_name_input,
                        is_active: self.active_login_field == LoginInputField::FirstName,
                        is_password: false,
                        active_placeholder: tr_str(lang, "| 请输入显示昵称", "| Your Display Name"),
                        idle_placeholder: tr_str(lang, "显示昵称", "Your Display Name"),
                        trailing_icon: Some(IconName::Verified),
                        allow_clear: true,
                        flex_grow: false,
                    },
                    cx,
                ))
            })
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::Username,
                    value: &self.username_input,
                    is_active: self.active_login_field == LoginInputField::Username,
                    is_password: false,
                    active_placeholder: tr_str(lang, "| 请输入用户名", "| Username"),
                    idle_placeholder: tr_str(lang, "用户名", "Username"),
                    trailing_icon: Some(IconName::ContactsUsers),
                    allow_clear: true,
                    flex_grow: false,
                },
                cx,
            ))
            .child(self.render_wizard_input_field(
                WizardInputFieldSpec {
                    field: LoginInputField::Password,
                    value: &self.password_input,
                    is_active: self.active_login_field == LoginInputField::Password,
                    is_password: true,
                    active_placeholder: tr_str(lang, "| 请输入密码", "| Password"),
                    idle_placeholder: tr_str(lang, "密码", "Password"),
                    trailing_icon: Some(IconName::Lock),
                    allow_clear: true,
                    flex_grow: false,
                },
                cx,
            ))
            .child(
                div()
                    .cursor_pointer()
                    .w_full()
                    .h(px(42.0))
                    .rounded(px(8.0))
                    .bg(if is_loading {
                        rgb(pal.accent_hover)
                    } else {
                        rgb(pal.accent)
                    })
                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.active_button_fg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.is_register_mode {
                                this.submit_register(cx);
                            } else {
                                this.submit_login(cx);
                            }
                        }),
                    )
                    .child(if is_loading {
                        tr_str(
                            lang,
                            "正在连接云服务器登录...",
                            "Authenticating with Cloud Server...",
                        )
                    } else if is_reg {
                        tr_str(lang, "注册并登录", "Create Account & Sign In")
                    } else {
                        tr_str(lang, "登录到云服务器", "Sign In")
                    }),
            )
            .child(
                div()
                    .cursor_pointer()
                    .text_size(px(12.0))
                    .text_color(rgb(pal.accent))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.is_register_mode = !this.is_register_mode;
                            this.login_error = None;
                            cx.notify();
                        }),
                    )
                    .child(if is_reg {
                        tr_str(lang, "← 已有账号？直接登录", "← Already have an account? Sign In")
                    } else {
                        tr_str(lang, "没有账号？点击注册", "Don't have an account? Register")
                    }),
            )
            .into_any_element()
    }

    pub fn render_login_wizard(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;

        div()
            .w(px(440.0))
            .rounded(px(16.0))
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .p(px(32.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(18.0))
            .child(self.render_top_controls(cx))
            .child(
                div()
                    .w(px(84.0))
                    .h(px(84.0))
                    .rounded_full()
                    .bg(rgb(pal.accent))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(tg_icon(IconName::BotRobot, 44.0, pal.active_button_fg)),
            )
            .child(match self.login_step {
                LoginWizardStep::Start => div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .child(self.render_server_url_bar(cx))
                    .child(self.render_tab_switcher(cx))
                    .when_some(self.login_error.clone(), |el, err| {
                        el.child(
                            div()
                                .w_full()
                                .p(px(8.0))
                                .rounded(px(6.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.14))
                                .border_1()
                                .border_color(rgb(pal.danger))
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .text_size(px(12.0))
                                .text_color(rgb(pal.danger))
                                .child(tg_icon(IconName::Close, 12.0, pal.danger))
                                .child(err),
                        )
                    })
                    .child(match self.auth_wizard_tab {
                        AuthWizardTab::PasswordLogin => self.render_password_login_tab(cx),
                        AuthWizardTab::AlipayQr => self.render_alipay_qr_tab(cx),
                        AuthWizardTab::EmailRegister => self.render_email_register_tab(cx),
                    })
                    .into_any_element(),
                LoginWizardStep::Phone => self.render_phone_step(cx),
                LoginWizardStep::QrCode => self.render_qr_step(cx),
                LoginWizardStep::Code => self.render_code_step(cx),
            })
    }
}

impl Render for AuthWizardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.focus_handle.is_focused(window) {
            self.focus_handle.focus(window);
        }
        let pal = self.palette;
        let focus_handle = self.focus_handle.clone();
        let view = cx.entity().clone();
        let ime_canvas = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, window, cx| {
                window.handle_input(
                    &focus_handle,
                    ElementInputHandler::new(bounds, view.clone()),
                    cx,
                );
            },
        )
        .size_0();

        div()
            .id("auth-wizard-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_window_key_down(event, window, cx);
            }))
            .size_full()
            .bg(rgb(pal.window_bg))
            .flex()
            .items_center()
            .justify_center()
            .child(ime_canvas)
            .child(self.render_login_wizard(cx))
    }
}
