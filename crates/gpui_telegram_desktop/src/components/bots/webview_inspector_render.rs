//! Disclaimer, headless widget fallback, and native Mini App modal renderers for [`BotAttachWebView`].

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;

use super::bot_attach_web_view::{
    BotAttachWebView, BotWebViewPendingAction, MiniAppPopupState,
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

    /// Renders Telegram Desktop's centered `允许发送消息` write-access permission dialog.
    pub(crate) fn render_write_access_modal(
        &self,
        app_title: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let bot_id = self.bot_id;
        let bot_username = self.bot_username.clone();
        let app_id = self.app_id.clone();
        let title_owned = app_title.to_string();
        let bot_sub = if bot_username.trim().is_empty() {
            "@bot".to_string()
        } else {
            format!("@{}", bot_username.trim().trim_start_matches('@'))
        };

        div()
            .id("miniapp-write-access-backdrop")
            .absolute()
            .inset_0()
            .bg(rgba(0x00000078))
            .flex()
            .items_center()
            .justify_center()
            .p(px(24.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.resolve_write_access(false);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .id("miniapp-write-access-modal")
                    .w(px(340.0))
                    .p(px(20.0))
                    .rounded(px(14.0))
                    .bg(rgb(pal.modal_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _, _, cx| cx.stop_propagation()),
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "允许发送消息", "Allow Messages")),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                lang,
                                "您想允许此机器人向您发送消息？",
                                "Would you like to allow this bot to send messages to you?",
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.accent))
                            .child(bot_sub),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .flex()
                            .justify_end()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("miniapp-write-access-cancel-btn")
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .py(px(7.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.card_bg))
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(pal.accent))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.resolve_write_access(false);
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "取消", "Cancel")),
                            )
                            .child(
                                div()
                                    .id("miniapp-write-access-allow-btn")
                                    .cursor_pointer()
                                    .px(px(16.0))
                                    .py(px(7.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.write_access_prompt_open = false;
                                            this.pending_actions.push_back(
                                                BotWebViewPendingAction::AllowBotSendMessage {
                                                    bot_id,
                                                    bot_username: bot_username.clone(),
                                                    app_id: app_id.clone(),
                                                    app_title: title_owned.clone(),
                                                },
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "允许", "Allow")),
                            ),
                    ),
            )
            .into_any_element()
    }

    /// Renders a native Telegram WebApp `showPopup` modal (`web_app_open_popup`).
    pub(crate) fn render_custom_popup_modal(
        &self,
        popup: &MiniAppPopupState,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let title = popup.title.clone();
        let message = popup.message.clone();
        let buttons = popup.buttons.clone();

        div()
            .id("miniapp-custom-popup-backdrop")
            .absolute()
            .inset_0()
            .bg(rgba(0x00000078))
            .flex()
            .items_center()
            .justify_center()
            .p(px(24.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.resolve_custom_popup(None);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .id("miniapp-custom-popup-modal")
                    .w(px(330.0))
                    .p(px(20.0))
                    .rounded(px(14.0))
                    .bg(rgb(pal.modal_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _, _, cx| cx.stop_propagation()),
                    )
                    .when(!title.is_empty(), |col| {
                        col.child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(title),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(message),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .flex()
                            .flex_wrap()
                            .justify_end()
                            .gap(px(8.0))
                            .children(buttons.into_iter().enumerate().map(|(idx, btn)| {
                                let bid = btn.id.clone();
                                let fg = if btn.btn_type == "destructive" {
                                    pal.danger
                                } else {
                                    pal.accent
                                };
                                div()
                                    .id(("miniapp-popup-btn", idx))
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(fg))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.resolve_custom_popup(Some(&bid));
                                            cx.notify();
                                        }),
                                    )
                                    .child(btn.text)
                            })),
                    ),
            )
            .into_any_element()
    }
}
