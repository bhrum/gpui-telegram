//! Chrome, header, navigation tabs, overflow menu, and footer rendering for [`BotAttachWebView`].

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::MiniAppActionRequest;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

use super::bot_attach_web_view::{BotAttachWebView, BotWebViewPendingAction};

fn overflow_menu_btn(
    id: &'static str,
    label: impl Into<String>,
    fg: u32,
    pal: TdesktopThemePalette,
    on_click: impl Fn(&mut BotAttachWebView, &mut Context<BotAttachWebView>) + 'static,
    cx: &mut Context<BotAttachWebView>,
) -> impl IntoElement {
    div()
        .id(id)
        .cursor_pointer()
        .px(px(10.0))
        .py(px(5.0))
        .rounded(px(7.0))
        .bg(rgb(pal.card_bg))
        .hover(|s| s.bg(rgb(pal.menu_hover_bg)))
        .text_size(px(11.5))
        .text_color(rgb(fg))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                on_click(this, cx);
                cx.notify();
            }),
        )
        .child(label.into())
}

impl Render for BotAttachWebView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().into_any_element();
        }
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let app_title = if !self.ui_schema.title.trim().is_empty() {
            self.ui_schema.title.clone()
        } else {
            self.active_bot
                .as_ref()
                .map(|b| b.name.clone())
                .unwrap_or_else(|| "Mini App".to_string())
        };
        let bot_sub = if self.bot_username.is_empty() {
            "@bot".to_string()
        } else {
            format!("@{}", self.bot_username.trim_start_matches('@'))
        };
        let short_name = self.current_short_name();
        let init_hmac_ok = self
            .launch_context
            .as_ref()
            .is_some_and(|c| c.init_data.hmac_verified);
        let chat_id_opt = self.target_chat_i64();
        let companion_cid = self.companion_chat_id.unwrap_or(0);
        let browser_path = format!(
            "/miniapps/{}/{}",
            self.bot_username.trim_start_matches('@'),
            short_name
        );

        let show_back = self.ui_schema.back_button_visible;
        let show_settings = self.ui_schema.settings_button_visible || self.active_tab == 2;
        let is_full = self.is_fullscreen;
        let tab = self.active_tab;

        let body: AnyElement = if !self.disclaimer_accepted {
            self.render_disclaimer_panel(&app_title, &bot_sub, &short_name, cx)
        } else if tab == 0 {
            self.render_widgets_panel(&app_title, cx)
        } else if tab == 1 {
            self.render_cloud_storage_panel(&short_name, cx)
        } else {
            self.render_security_panel(init_hmac_ok, companion_cid, &bot_sub, cx)
        };
        let copilot_strip =
            (self.disclaimer_accepted && tab == 0).then(|| self.render_copilot_strip(&bot_sub, cx));

        let sheet = div()
            .id("bot-attach-webview-sheet")
            .when(is_full, |el| el.w_full().h_full())
            .when(!is_full, |el| {
                el.w(px(500.0)).h(px(680.0)).rounded(px(16.0)).shadow_lg()
            })
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .overflow_hidden()
            .flex()
            .flex_col()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    if this.menu_open {
                        this.menu_open = false;
                        cx.notify();
                    }
                    cx.stop_propagation();
                }),
            )
            // 1. Telegram Desktop Mini App Header
            .child(
                div()
                    .w_full()
                    .h(px(54.0))
                    .px(px(12.0))
                    .bg(rgb(pal.top_bar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .when(show_back, |row| {
                                let b_user = self.bot_username.clone();
                                let s_name = short_name.clone();
                                row.child(
                                    div()
                                        .id("miniapp-back-btn")
                                        .cursor_pointer()
                                        .w(px(30.0))
                                        .h(px(30.0))
                                        .rounded_full()
                                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                this.pending_actions.push_back(
                                                    BotWebViewPendingAction::SendAppAction {
                                                        bot_username: b_user.clone(),
                                                        short_name: s_name.clone(),
                                                        req: MiniAppActionRequest {
                                                            action_type: "back_button_click"
                                                                .to_string(),
                                                            chat_id: chat_id_opt,
                                                            ..Default::default()
                                                        },
                                                    },
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child(tg_icon(IconName::BackArrow, 17.0, pal.icon_fg)),
                                )
                            })
                            .child(
                                div()
                                    .id("miniapp-close-btn")
                                    .cursor_pointer()
                                    .w(px(30.0))
                                    .h(px(30.0))
                                    .rounded_full()
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            if this.is_dirty && !this.confirm_close_open {
                                                this.confirm_close_open = true;
                                            } else {
                                                this.close();
                                                this.pending_actions
                                                    .push_back(BotWebViewPendingAction::Close);
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::Close, 16.0, pal.icon_fg)),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(app_title),
                                    )
                                    .child(tg_icon(IconName::Verified, 13.0, pal.accent))
                                    .when(init_hmac_ok, |row| {
                                        row.child(
                                            div()
                                                .id("miniapp-initdata-verified-badge")
                                                .px(px(5.0))
                                                .py(px(1.0))
                                                .rounded(px(4.0))
                                                .bg(rgb(pal.composer_bg))
                                                .text_size(px(9.5))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(pal.success))
                                                .child("initData HMAC ✓"),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "{} · {}",
                                        bot_sub,
                                        tr_str(lang, "伴侣机器人", "Companion Bot")
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .id("miniapp-minimize-btn")
                                    .cursor_pointer()
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded_full()
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.icon_fg))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.is_open = false;
                                            this.pending_actions
                                                .push_back(BotWebViewPendingAction::Minimize);
                                            cx.notify();
                                        }),
                                    )
                                    .child("—"),
                            )
                            .when(show_settings, |row| {
                                row.child(
                                    div()
                                        .id("miniapp-settings-btn")
                                        .cursor_pointer()
                                        .w(px(28.0))
                                        .h(px(28.0))
                                        .rounded_full()
                                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.active_tab =
                                                    if this.active_tab == 2 { 0 } else { 2 };
                                                cx.notify();
                                            }),
                                        )
                                        .child(tg_icon(IconName::SettingsGear, 15.0, pal.icon_fg)),
                                )
                            })
                            .child(
                                div()
                                    .id("miniapp-more-btn")
                                    .cursor_pointer()
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded_full()
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.menu_open = !this.menu_open;
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::MoreVertical, 15.0, pal.icon_fg)),
                            ),
                    ),
            )
            // 2. Navigation Sub-Tabs (App / CloudStorage / Info)
            .child(
                div()
                    .w_full()
                    .px(px(12.0))
                    .py(px(6.0))
                    .bg(rgb(pal.card_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .id("miniapp-tab-view")
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(999.0))
                            .bg(rgb(if tab == 0 { pal.accent } else { pal.input_bg }))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if tab == 0 {
                                0xFFFFFF
                            } else {
                                pal.text_secondary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = 0;
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "小程序界面", "Mini App")),
                    )
                    .child({
                        let b_user = self.bot_username.clone();
                        let s_name = short_name.clone();
                        div()
                            .id("miniapp-tab-storage")
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(999.0))
                            .bg(rgb(if tab == 1 { pal.accent } else { pal.input_bg }))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if tab == 1 {
                                0xFFFFFF
                            } else {
                                pal.text_secondary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.active_tab = 1;
                                    this.pending_actions.push_back(
                                        BotWebViewPendingAction::CloudStorageRefresh {
                                            bot_username: b_user.clone(),
                                            short_name: s_name.clone(),
                                        },
                                    );
                                    cx.notify();
                                }),
                            )
                            .child(format!(
                                "{} ({})",
                                tr_str(lang, "云存储", "CloudStorage"),
                                self.cloud_storage.len()
                            ))
                    })
                    .child(
                        div()
                            .id("miniapp-tab-info")
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(999.0))
                            .bg(rgb(if tab == 2 { pal.accent } else { pal.input_bg }))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if tab == 2 {
                                0xFFFFFF
                            } else {
                                pal.text_secondary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.active_tab = 2;
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "安全与伴侣", "Security")),
                    ),
            )
            // Overflow Menu (⋯)
            .when(self.menu_open, |el| {
                let b_user_reload = self.bot_username.clone();
                let s_name_reload = short_name.clone();
                let b_user_bot = self.bot_username.clone();
                let b_user_rm = self.bot_username.clone();
                let s_name_rm = short_name.clone();
                let web_path = browser_path.clone();
                el.child(
                    div()
                        .id("miniapp-overflow-menu")
                        .w_full()
                        .px(px(12.0))
                        .py(px(8.0))
                        .bg(rgb(pal.menu_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .flex_wrap()
                        .gap(px(6.0))
                        .child(overflow_menu_btn(
                            "miniapp-menu-reload",
                            tr_str(lang, "🔄 刷新", "🔄 Reload"),
                            pal.text_primary,
                            pal,
                            move |this, _| {
                                this.menu_open = false;
                                this.is_loading = true;
                                this.pending_actions.push_back(BotWebViewPendingAction::Reload {
                                    bot_username: b_user_reload.clone(),
                                    short_name: s_name_reload.clone(),
                                    chat_id: chat_id_opt,
                                });
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-fullscreen",
                            if is_full {
                                tr_str(lang, "窗口模式", "Compact Sheet")
                            } else {
                                tr_str(lang, "全屏模式", "Fullscreen")
                            },
                            pal.text_primary,
                            pal,
                            |this, _| {
                                this.menu_open = false;
                                this.is_fullscreen = !this.is_fullscreen;
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-open-bot",
                            format!("💬 {}", bot_sub),
                            pal.accent,
                            pal,
                            move |this, _| {
                                this.menu_open = false;
                                this.is_open = false;
                                this.pending_actions.push_back(
                                    BotWebViewPendingAction::OpenCompanionChat {
                                        companion_chat_id: companion_cid,
                                        bot_username: b_user_bot.clone(),
                                    },
                                );
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-browser",
                            tr_str(lang, "🌐 浏览器打开", "🌐 Open in Browser"),
                            pal.text_primary,
                            pal,
                            move |this, _| {
                                this.menu_open = false;
                                this.pending_actions.push_back(
                                    BotWebViewPendingAction::OpenInBrowser {
                                        path: web_path.clone(),
                                    },
                                );
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-remove",
                            tr_str(lang, "🗑 移除小程序", "🗑 Remove App"),
                            pal.danger,
                            pal,
                            move |this, _| {
                                this.menu_open = false;
                                this.close();
                                this.pending_actions.push_back(
                                    BotWebViewPendingAction::UninstallApp {
                                        bot_username: b_user_rm.clone(),
                                        short_name: s_name_rm.clone(),
                                    },
                                );
                            },
                            cx,
                        )),
                )
            })
            // Dirty close confirmation banner
            .when(self.confirm_close_open, |el| {
                el.child(
                    div()
                        .id("miniapp-confirm-close-bar")
                        .w_full()
                        .px(px(14.0))
                        .py(px(8.0))
                        .bg(rgb(pal.card_bg))
                        .border_b_1()
                        .border_color(rgb(pal.danger))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_primary))
                                .child(tr_str(
                                    lang,
                                    "您有未保存操作，确定关闭或改为最小化？",
                                    "Unsaved changes. Close anyway or minimize?",
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(6.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.5))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.confirm_close_open = false;
                                                cx.notify();
                                            }),
                                        )
                                        .child(tr_str(lang, "取消", "Stay")),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(6.0))
                                        .bg(rgb(pal.danger))
                                        .text_size(px(11.5))
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.close();
                                                this.pending_actions
                                                    .push_back(BotWebViewPendingAction::Close);
                                                cx.notify();
                                            }),
                                        )
                                        .child(tr_str(lang, "关闭", "Close")),
                                ),
                        ),
                )
            })
            // 3. First-Launch Disclaimer OR Tab Body
            .child(body)
            // 4. Built-in AI Co-Pilot Strip (Surpassing Telegram)
            .when_some(copilot_strip, |el, strip| el.child(strip))
            // 5. Pinned Telegram MainButton & SecondaryButton Bar
            .when(
                self.disclaimer_accepted
                    && tab == 0
                    && (self
                        .ui_schema
                        .main_button
                        .as_ref()
                        .is_some_and(|b| b.is_visible)
                        || self
                            .ui_schema
                            .secondary_button
                            .as_ref()
                            .is_some_and(|b| b.is_visible)),
                |el| {
                    let sec = self
                        .ui_schema
                        .secondary_button
                        .clone()
                        .filter(|b| b.is_visible);
                    let main_b = self.ui_schema.main_button.clone().filter(|b| b.is_visible);
                    let b_user_sec = self.bot_username.clone();
                    let s_name_sec = short_name.clone();
                    let b_user_main = self.bot_username.clone();
                    let s_name_main = short_name.clone();
                    el.child(
                        div()
                            .id("miniapp-bottom-bar")
                            .w_full()
                            .px(px(12.0))
                            .py(px(10.0))
                            .bg(rgb(pal.top_bar_bg))
                            .border_t_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .when_some(sec, |row, sb| {
                                let label = sb.text.clone();
                                row.child(
                                    div()
                                        .id("miniapp-secondary-btn")
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(38.0))
                                        .rounded(px(9.0))
                                        .bg(rgb(pal.card_bg))
                                        .border_1()
                                        .border_color(rgb(pal.border_fg))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                this.is_loading = true;
                                                this.pending_actions.push_back(
                                                    BotWebViewPendingAction::SendAppAction {
                                                        bot_username: b_user_sec.clone(),
                                                        short_name: s_name_sec.clone(),
                                                        req: MiniAppActionRequest::secondary_button_click(
                                                            chat_id_opt,
                                                        ),
                                                    },
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child(label),
                                )
                            })
                            .when_some(main_b, |row, mb| {
                                let bg_col = Self::parse_hex_color(mb.color.as_deref(), pal.accent);
                                let fg_col =
                                    Self::parse_hex_color(mb.text_color.as_deref(), 0xFFFFFF);
                                let sends_data = mb.sends_data;
                                let label = if self.is_loading {
                                    format!("⏳ {}", mb.text)
                                } else {
                                    mb.text.clone()
                                };
                                row.child(
                                    div()
                                        .id("miniapp-main-btn")
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(38.0))
                                        .rounded(px(9.0))
                                        .bg(rgb(bg_col))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(13.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(fg_col))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                this.is_loading = true;
                                                let req = if sends_data {
                                                    let payload = serde_json::to_string(
                                                        &this.ui_schema.widgets,
                                                    )
                                                    .unwrap_or_else(|_| "{}".to_string());
                                                    MiniAppActionRequest::send_data(
                                                        payload,
                                                        chat_id_opt,
                                                    )
                                                } else {
                                                    MiniAppActionRequest::main_button_click(
                                                        chat_id_opt,
                                                    )
                                                };
                                                this.pending_actions.push_back(
                                                    BotWebViewPendingAction::SendAppAction {
                                                        bot_username: b_user_main.clone(),
                                                        short_name: s_name_main.clone(),
                                                        req,
                                                    },
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child(label),
                                )
                            }),
                    )
                },
            )
            .when_some(self.status_toast.clone(), |el, msg| {
                el.child(
                    div()
                        .w_full()
                        .px(px(12.0))
                        .py(px(6.0))
                        .bg(rgb(pal.tooltip_bg))
                        .text_size(px(11.5))
                        .text_color(rgb(pal.tooltip_fg))
                        .child(msg),
                )
            });

        div()
            .id("bot-attach-webview-backdrop")
            .w_full()
            .h_full()
            .bg(rgba(0x00000088))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    if !this.is_dirty {
                        this.is_open = false;
                        this.pending_actions
                            .push_back(BotWebViewPendingAction::Minimize);
                        cx.notify();
                    }
                }),
            )
            .child(sheet)
            .into_any_element()
    }
}
