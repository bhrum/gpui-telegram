//! Create Managed Bot Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/create_managed_bot_box.h:L1-L35`
//! - `Telegram/SourceFiles/boxes/peers/create_managed_bot_box.cpp:L1-L522`
//! - `create_managed_bot_box.cpp:L40-L150` (Bot creation flow)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

/// Dialog box for initializing and attaching a managed business or agent bot,
/// or generating a full Bot + Mini App in one click via natural language.
/// Target symbol in ledger: `CreateManagedBotBoxPeers`.
#[derive(Debug, Clone)]
pub struct CreateManagedBotBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub bot_name: String,
    pub bot_username: String,
    pub bot_token: String,
    pub is_business_bot: bool,
    pub is_submitting: bool,
    pub is_ai_mode: bool,
    pub nl_prompt: String,
    pub active_field: u8,
    pub pending_submit_nl: Option<String>,
    pub pending_submit_credentials: Option<(String, String, String, bool)>,
    pub pending_close: bool,
    pub status_message: Option<String>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl Default for CreateManagedBotBoxPeers {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateManagedBotBoxPeers {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            width: px(420.0),
            height: px(440.0),
            bot_name: String::new(),
            bot_username: String::new(),
            bot_token: String::new(),
            is_business_bot: true,
            is_submitting: false,
            is_ai_mode: true,
            nl_prompt: String::new(),
            active_field: 0,
            pending_submit_nl: None,
            pending_submit_credentials: None,
            pending_close: false,
            status_message: None,
            palette: TdesktopThemePalette::default(),
            language_id: "en".to_string(),
        }
    }

    pub fn set_credentials(&mut self, name: &str, username: &str, token: &str) {
        self.bot_name = name.trim().to_string();
        self.bot_username = username.trim().to_string();
        self.bot_token = token.trim().to_string();
    }

    pub fn is_valid(&self) -> bool {
        !self.bot_name.trim().is_empty()
            && gpui_telegram_proto::miniapp_contracts::validate_bot_username(&self.bot_username)
                .is_ok()
    }

    pub fn open_ai_creator(&mut self) {
        self.is_visible = true;
        self.is_ai_mode = true;
        self.active_field = 0;
        self.is_submitting = false;
        self.status_message = None;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: impl Into<String>) {
        self.language_id = lang.into();
    }

    pub fn push_text(&mut self, s: &str) {
        if self.is_ai_mode || self.active_field == 0 {
            self.nl_prompt.push_str(s);
        } else {
            match self.active_field {
                1 => self.bot_name.push_str(s),
                2 => self.bot_username.push_str(s),
                _ => self.bot_token.push_str(s),
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.is_ai_mode || self.active_field == 0 {
            self.nl_prompt.pop();
        } else {
            match self.active_field {
                1 => {
                    self.bot_name.pop();
                }
                2 => {
                    self.bot_username.pop();
                }
                _ => {
                    self.bot_token.pop();
                }
            }
        }
    }

    pub fn submit(&mut self) {
        if self.is_ai_mode {
            let trimmed = self.nl_prompt.trim().to_string();
            if !trimmed.is_empty() {
                self.is_submitting = true;
                self.pending_submit_nl = Some(trimmed);
            }
        } else if self.is_valid() {
            self.is_submitting = true;
            self.pending_submit_credentials = Some((
                self.bot_name.clone(),
                self.bot_username.clone(),
                self.bot_token.clone(),
                self.is_business_bot,
            ));
        }
    }

    pub fn take_submit_nl(&mut self) -> Option<String> {
        self.pending_submit_nl.take()
    }

    pub fn take_submit_credentials(&mut self) -> Option<(String, String, String, bool)> {
        self.pending_submit_credentials.take()
    }

    pub fn take_close(&mut self) -> bool {
        let was = self.pending_close;
        self.pending_close = false;
        was
    }
}

impl Render for CreateManagedBotBoxPeers {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let is_ai = self.is_ai_mode;
        let prompt = self.nl_prompt.clone();
        let can_submit = if is_ai {
            !prompt.trim().is_empty() && !self.is_submitting
        } else {
            self.is_valid() && !self.is_submitting
        };

        let presets: [(&str, &str); 3] = [
            ("☕ 咖啡点单", "创建一个精品咖啡点单和会员积分小程序"),
            ("✅ 团队待办", "创建一个支持分配负责人、优先级和 AI 总结的待办清单小程序"),
            ("📊 投票决策", "创建一个实时团队投票和数据分析小程序"),
        ];

        div()
            .id("create-managed-bot-modal")
            .w(self.width)
            .min_h(self.height)
            .p(px(18.0))
            .rounded(px(14.0))
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .on_mouse_down(MouseButton::Left, cx.listener(|_, _, _, cx| cx.stop_propagation()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(tg_icon(IconName::BotRobot, 20.0, pal.accent))
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(tr_str(lang, "BotFather · AI 快速创建", "BotFather · AI Bot & Mini App")),
                            ),
                    )
                    .child(
                        div()
                            .id("create-managed-bot-close-btn")
                            .cursor_pointer()
                            .p(px(4.0))
                            .rounded_full()
                            .hover(|s| s.bg(rgb(pal.hover_bg)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.is_visible = false;
                                this.pending_close = true;
                                cx.notify();
                            }))
                            .child(tg_icon(IconName::Close, 16.0, pal.icon_fg)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(6.0))
                    .child(
                        div()
                            .id("create-bot-tab-ai")
                            .cursor_pointer()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(8.0))
                            .bg(rgb(if is_ai { pal.accent } else { pal.card_bg }))
                            .flex()
                            .justify_center()
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if is_ai { 0xFFFFFF } else { pal.text_primary }))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.is_ai_mode = true;
                                this.active_field = 0;
                                cx.notify();
                            }))
                            .child(tr_str(lang, "✨ 一句话 AI 生成", "✨ One-Click AI")),
                    )
                    .child(
                        div()
                            .id("create-bot-tab-manual")
                            .cursor_pointer()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(8.0))
                            .bg(rgb(if !is_ai { pal.accent } else { pal.card_bg }))
                            .flex()
                            .justify_center()
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if !is_ai { 0xFFFFFF } else { pal.text_primary }))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.is_ai_mode = false;
                                this.active_field = 1;
                                cx.notify();
                            }))
                            .child(tr_str(lang, "Token 接入", "Bot Token")),
                    ),
            )
            .when(is_ai, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(
                                    lang,
                                    "描述你想要的机器人和小程序，系统将自动创建可交互 Mini App 并绑定专属 AI 伴侣机器人：",
                                    "Describe the Bot + Mini App you want. Companion AI Bot + interactive Mini App will be provisioned automatically:",
                                )),
                        )
                        .child(
                            div()
                                .id("create-managed-bot-nl-input")
                                .w_full()
                                .min_h(px(76.0))
                                .p(px(10.0))
                                .rounded(px(10.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(pal.accent))
                                .text_size(px(13.0))
                                .text_color(rgb(if prompt.is_empty() { pal.text_secondary } else { pal.text_primary }))
                                .child(if prompt.is_empty() {
                                    tr_str(lang, "例如：帮我做一个极简咖啡点单小程序和AI店长…", "e.g., Build a specialty coffee ordering Mini App with AI barista…").to_string()
                                } else {
                                    prompt
                                }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(px(6.0))
                                .children(presets.into_iter().map(|(pill, full)| {
                                    let full_owned = full.to_string();
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(999.0))
                                        .bg(rgb(pal.card_bg))
                                        .border_1()
                                        .border_color(rgb(pal.border_fg))
                                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                                        .text_size(px(11.5))
                                        .text_color(rgb(pal.text_link))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                            this.nl_prompt = full_owned.clone();
                                            cx.notify();
                                        }))
                                        .child(pill)
                                })),
                        ),
                )
            })
            .when(!is_ai, |el| {
                let fields: [(u8, &str, String); 3] = [
                    (1, "Bot Name", self.bot_name.clone()),
                    (2, "@username (must end with bot)", self.bot_username.clone()),
                    (3, "HTTP API Token", self.bot_token.clone()),
                ];
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .children(fields.into_iter().map(|(fid, label, val)| {
                            let active = self.active_field == fid;
                            div()
                                .cursor_pointer()
                                .p(px(9.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.active_field = fid;
                                    cx.notify();
                                }))
                                .child(div().text_size(px(11.0)).text_color(rgb(pal.text_secondary)).child(label))
                                .child(div().text_size(px(13.0)).text_color(rgb(pal.text_primary)).child(if val.is_empty() { "—".to_string() } else { val }))
                        })),
                )
            })
            .when_some(self.status_message.clone(), |el, msg| {
                el.child(div().text_size(px(12.0)).text_color(rgb(pal.success)).child(msg))
            })
            .child(
                div()
                    .id("create-managed-bot-submit-btn")
                    .cursor_pointer()
                    .w_full()
                    .h(px(38.0))
                    .rounded(px(10.0))
                    .bg(rgb(if can_submit { pal.accent } else { pal.muted_badge_bg }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(13.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                        this.submit();
                        cx.notify();
                    }))
                    .child(if self.is_submitting {
                        tr_str(lang, "正在创建并部署…", "Provisioning…")
                    } else if is_ai {
                        tr_str(lang, "✨ 立即创建 Bot & 小程序", "✨ Create Bot & Mini App")
                    } else {
                        tr_str(lang, "保存并连接 Bot", "Connect Managed Bot")
                    }),
            )
            .into_any_element()
    }
}
