//! Disclaimer, Inspector, CloudStorage, Security, and AI Co-Pilot subpanels for [`BotAttachWebView`].

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::CLOUD_STORAGE_MAX_KEYS;

use crate::i18n::tr_str;

use super::bot_attach_web_view::{
    BotAttachWebView, BotWebViewInputTarget, BotWebViewPendingAction,
};

impl BotAttachWebView {
    pub(crate) fn render_disclaimer_panel(
        &self,
        app_title: &str,
        bot_sub: &str,
        short_name: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let b_user = self.bot_username.clone();
        let s_name = short_name.to_string();

        div()
            .id("miniapp-disclaimer-panel")
            .flex_1()
            .p(px(24.0))
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(14.0))
            .child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(format!(
                        "{} · {}",
                        tr_str(lang, "首次启动小程序确认", "Mini App Launch Notice"),
                        app_title
                    )),
            )
            .child(
                div()
                    .text_size(px(12.5))
                    .text_color(rgb(pal.text_secondary))
                    .child(format!(
                        "{} ({}) {}",
                        bot_sub,
                        self.url,
                        tr_str(
                            lang,
                            "将接收经 HMAC-SHA256 签名的 initData 用户会话标识与主题参数。",
                            "will receive HMAC-SHA256 signed initData & theme params.",
                        )
                    )),
            )
            .child(
                div()
                    .id("miniapp-write-access-checkbox")
                    .cursor_pointer()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.allow_write_access_checked = !this.allow_write_access_checked;
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .w(px(16.0))
                            .h(px(16.0))
                            .rounded(px(4.0))
                            .bg(rgb(if self.allow_write_access_checked {
                                pal.accent
                            } else {
                                pal.input_bg
                            }))
                            .border_1()
                            .border_color(rgb(pal.accent))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(11.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(if self.allow_write_access_checked {
                                "✓"
                            } else {
                                ""
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_primary))
                            .child(format!(
                                "{} {} {}",
                                tr_str(lang, "允许", "Allow"),
                                bot_sub,
                                tr_str(
                                    lang,
                                    "向我发送伴侣机器人消息 (write_access)",
                                    "to message me (write_access)"
                                )
                            )),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(10.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .flex_1()
                            .h(px(38.0))
                            .rounded(px(9.0))
                            .bg(rgb(pal.card_bg))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_secondary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.close();
                                    this.pending_actions
                                        .push_back(BotWebViewPendingAction::Close);
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "取消", "Cancel")),
                    )
                    .child(
                        div()
                            .id("miniapp-disclaimer-confirm-btn")
                            .cursor_pointer()
                            .flex_1()
                            .h(px(38.0))
                            .rounded(px(9.0))
                            .bg(rgb(pal.accent))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    let allow = this.allow_write_access_checked;
                                    this.accept_disclaimer();
                                    this.pending_actions.push_back(
                                        BotWebViewPendingAction::AcceptDisclaimer {
                                            bot_username: b_user.clone(),
                                            short_name: s_name.clone(),
                                            allow_write_access: allow,
                                        },
                                    );
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "确认并启动", "Launch App")),
                    ),
            )
            .into_any_element()
    }

    pub(crate) fn render_widgets_panel(
        &self,
        app_title: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let widgets: Vec<AnyElement> = self
            .ui_schema
            .widgets
            .iter()
            .map(|w| self.render_widget(w, cx))
            .collect();

        div()
            .id("miniapp-widgets-scroll")
            .flex_1()
            .overflow_y_scroll()
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .when(!self.ui_schema.subtitle.is_empty(), |col| {
                col.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_secondary))
                        .child(self.ui_schema.subtitle.clone()),
                )
            })
            .when(widgets.is_empty(), |col| {
                col.child(
                    div()
                        .p(px(16.0))
                        .rounded(px(10.0))
                        .bg(rgb(pal.card_bg))
                        .text_size(px(12.5))
                        .text_color(rgb(pal.text_secondary))
                        .child(format!("Loaded {} ({})", app_title, self.url)),
                )
            })
            .children(widgets)
            .into_any_element()
    }

    pub(crate) fn render_cloud_storage_panel(
        &self,
        short_name: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let cs_key_focused = matches!(self.focused_input, Some(BotWebViewInputTarget::CloudKey));
        let cs_val_focused = matches!(self.focused_input, Some(BotWebViewInputTarget::CloudValue));
        let b_user_ref = self.bot_username.clone();
        let s_name_ref = short_name.to_string();

        div()
            .id("miniapp-cloud-storage-panel")
            .flex_1()
            .overflow_y_scroll()
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(format!(
                                "CloudStorage ({} / {})",
                                self.cloud_storage.len(),
                                CLOUD_STORAGE_MAX_KEYS
                            )),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.card_bg))
                            .text_size(px(11.0))
                            .text_color(rgb(pal.accent))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.pending_actions.push_back(
                                        BotWebViewPendingAction::CloudStorageRefresh {
                                            bot_username: b_user_ref.clone(),
                                            short_name: s_name_ref.clone(),
                                        },
                                    );
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "刷新", "Refresh")),
                    ),
            )
            .child(
                div()
                    .p(px(10.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .id("miniapp-cs-key-input")
                            .cursor_text()
                            .px(px(8.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(if cs_key_focused {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .text_size(px(12.0))
                            .text_color(rgb(if self.cloud_key_draft.is_empty() {
                                pal.text_secondary
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.focused_input = Some(BotWebViewInputTarget::CloudKey);
                                    cx.notify();
                                }),
                            )
                            .child(if self.cloud_key_draft.is_empty() {
                                "Key (<=128B)".to_string()
                            } else {
                                self.cloud_key_draft.clone()
                            }),
                    )
                    .child(
                        div()
                            .id("miniapp-cs-val-input")
                            .cursor_text()
                            .px(px(8.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(if cs_val_focused {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .text_size(px(12.0))
                            .text_color(rgb(if self.cloud_value_draft.is_empty() {
                                pal.text_secondary
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.focused_input = Some(BotWebViewInputTarget::CloudValue);
                                    cx.notify();
                                }),
                            )
                            .child(if self.cloud_value_draft.is_empty() {
                                "Value (<=4096B)".to_string()
                            } else {
                                self.cloud_value_draft.clone()
                            }),
                    )
                    .child(
                        div()
                            .id("miniapp-cs-save-btn")
                            .cursor_pointer()
                            .h(px(32.0))
                            .rounded(px(7.0))
                            .bg(rgb(pal.accent))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.trigger_save_cloud_storage();
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "写入 CloudStorage", "Save Key/Value")),
                    ),
            )
            .children(self.cloud_storage.iter().map(|item| {
                let del_key = item.key.clone();
                let b_user_del = self.bot_username.clone();
                let s_name_del = short_name.to_string();
                div()
                    .p(px(9.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(item.key.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(11.5))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(item.value.clone()),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .text_size(px(11.0))
                            .text_color(rgb(pal.danger))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.pending_actions.push_back(
                                        BotWebViewPendingAction::CloudStorageDelete {
                                            bot_username: b_user_del.clone(),
                                            short_name: s_name_del.clone(),
                                            key: del_key.clone(),
                                        },
                                    );
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "删除", "Delete")),
                    )
            }))
            .into_any_element()
    }

    pub(crate) fn render_security_panel(
        &self,
        init_hmac_ok: bool,
        companion_cid: i64,
        bot_sub: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let raw_init = self
            .launch_context
            .as_ref()
            .map(|c| c.init_data.raw_query_string.clone())
            .unwrap_or_default();
        let b_user_chat = self.bot_username.clone();

        div()
            .id("miniapp-security-panel")
            .flex_1()
            .overflow_y_scroll()
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .p(px(10.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(12.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child("Telegram WebApp initData (HMAC-SHA256)"),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(if init_hmac_ok {
                                pal.success
                            } else {
                                pal.text_secondary
                            }))
                            .child(if init_hmac_ok {
                                "Signature Verified ✓"
                            } else {
                                "Session Signed"
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!("URL: {}", self.url)),
                    )
                    .when(!raw_init.is_empty(), |col| {
                        col.child(
                            div()
                                .text_size(px(10.5))
                                .text_color(rgb(pal.msg_in_date_fg))
                                .child(raw_init),
                        )
                    }),
            )
            .child(
                div()
                    .id("miniapp-open-companion-btn")
                    .cursor_pointer()
                    .w_full()
                    .h(px(36.0))
                    .rounded(px(9.0))
                    .bg(rgb(pal.accent))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.is_open = false;
                            this.pending_actions.push_back(
                                BotWebViewPendingAction::OpenCompanionChat {
                                    companion_chat_id: companion_cid,
                                    bot_username: b_user_chat.clone(),
                                },
                            );
                            cx.notify();
                        }),
                    )
                    .child(format!(
                        "💬 {} {}",
                        tr_str(lang, "切换到伴侣对话", "Open Companion Chat"),
                        bot_sub
                    )),
            )
            .into_any_element()
    }

    pub(crate) fn render_copilot_strip(
        &self,
        bot_sub: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let copilot_focused = matches!(
            self.focused_input,
            Some(BotWebViewInputTarget::CopilotPrompt)
        );
        let suggestions = self.ui_schema.copilot_suggestions.clone();
        let copilot_prompt = self.copilot_prompt.clone();
        let reply_opt = self.copilot_last_reply.clone();

        div()
            .id("miniapp-copilot-strip")
            .w_full()
            .px(px(12.0))
            .py(px(8.0))
            .bg(rgb(pal.card_bg))
            .border_t_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .id("miniapp-copilot-toggle")
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.copilot_expanded = !this.copilot_expanded;
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.accent))
                            .child(format!("✨ Ask {} AI Co-Pilot", bot_sub)),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(if self.copilot_expanded { "▾" } else { "▸" }),
                    ),
            )
            .when(self.copilot_expanded, |col| {
                col.when_some(reply_opt, |c, reply| {
                    c.child(
                        div()
                            .id("miniapp-copilot-reply")
                            .p(px(7.0))
                            .rounded(px(7.0))
                            .bg(rgb(pal.composer_bg))
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_primary))
                            .child(reply),
                    )
                })
                .when(!suggestions.is_empty(), |c| {
                    c.child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(5.0))
                            .children(suggestions.into_iter().map(|s| {
                                let s_owned = s.clone();
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(3.0))
                                    .rounded(px(999.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(|st| st.bg(rgb(pal.hover_bg)))
                                    .text_size(px(11.0))
                                    .text_color(rgb(pal.text_link))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.copilot_prompt = s_owned.clone();
                                            this.trigger_ai_copilot();
                                            cx.notify();
                                        }),
                                    )
                                    .child(s)
                            })),
                    )
                })
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .id("miniapp-copilot-input")
                                .cursor_text()
                                .flex_1()
                                .min_h(px(32.0))
                                .px(px(9.0))
                                .py(px(6.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(if copilot_focused {
                                    pal.accent
                                } else {
                                    pal.border_fg
                                }))
                                .text_size(px(12.0))
                                .text_color(rgb(if copilot_prompt.is_empty() {
                                    pal.text_secondary
                                } else {
                                    pal.text_primary
                                }))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.focused_input =
                                            Some(BotWebViewInputTarget::CopilotPrompt);
                                        cx.notify();
                                    }),
                                )
                                .child(if copilot_prompt.is_empty() {
                                    tr_str(
                                        lang,
                                        "输入自然语言指令让 AI 操作小程序…",
                                        "Ask AI Co-Pilot to act on this Mini App…",
                                    )
                                    .to_string()
                                } else {
                                    copilot_prompt
                                }),
                        )
                        .child(
                            div()
                                .id("miniapp-copilot-send-btn")
                                .cursor_pointer()
                                .px(px(10.0))
                                .h(px(32.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.accent))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.trigger_ai_copilot();
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "执行", "Ask")),
                        ),
                )
            })
            .into_any_element()
    }
}
