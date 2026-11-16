//! Authentic Telegram Desktop Mini App sheet chrome, WKWebView bounds sync, overflow menu,
//! Main/Secondary buttons, and `@bot_username` footer rendering for [`BotAttachWebView`].

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::MiniAppActionRequest;

#[cfg(target_os = "macos")]
use super::macos_wkwebview::MacOsWkWebView;
use super::bot_attach_web_view::{BotAttachWebView, BotWebViewPendingAction};
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

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
                cx.stop_propagation();
                on_click(this, cx);
                cx.notify();
            }),
        )
        .child(label.into())
}

impl Render for BotAttachWebView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            self.hide_native_webview();
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
        let bot_sub = if self.bot_username.trim().is_empty() {
            "@bot".to_string()
        } else {
            format!("@{}", self.bot_username.trim().trim_start_matches('@'))
        };
        let short_name = self.current_short_name();
        let chat_id_opt = self.target_chat_i64();
        let resolved_url = self.resolved_webview_url();
        let browser_target = if resolved_url.starts_with("http://")
            || resolved_url.starts_with("https://")
        {
            resolved_url
        } else {
            format!(
                "/miniapps/{}/{}",
                self.bot_username.trim().trim_start_matches('@'),
                short_name
            )
        };

        let header_bg =
            Self::parse_hex_color(self.header_color_override.as_deref(), pal.top_bar_bg);
        let body_bg =
            Self::parse_hex_color(self.background_color_override.as_deref(), pal.modal_bg);
        let show_back = self.ui_schema.back_button_visible;
        let show_settings = self.ui_schema.settings_button_visible;
        let is_full = self.is_fullscreen;

        let overlay_covering_webview = !self.disclaimer_accepted
            || self.write_access_prompt_open
            || self.menu_open
            || self.confirm_close_open
            || self.custom_popup.is_some();
        let native_visible = self.is_open && !overlay_covering_webview;

        #[cfg(target_os = "macos")]
        let use_headless_fallback = MacOsWkWebView::is_headless_or_offline_test_env();
        #[cfg(not(target_os = "macos"))]
        let use_headless_fallback = true;

        let main_viewport: AnyElement = if !self.disclaimer_accepted {
            self.render_disclaimer_panel(&app_title, &bot_sub, &short_name, cx)
        } else {
            let entity = cx.entity().clone();
            let bounds_canvas = canvas(
                |bounds, _, _| bounds,
                move |bounds, _, window, cx| {
                    let _ = entity.update(cx, |this, entity_cx| {
                        let had_events = this.sync_native_webview(
                            window,
                            bounds,
                            native_visible,
                            entity_cx,
                        );
                        if had_events {
                            entity_cx
                                .spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                                    let _ = weak.update(app, |_, cx| cx.notify());
                                })
                                .detach();
                        }
                    });
                },
            )
            .absolute()
            .inset_0();

            let fallback_content = if use_headless_fallback {
                Some(self.render_widgets_panel(&app_title, cx))
            } else {
                None
            };

            div()
                .id("miniapp-webview-viewport")
                .relative()
                .flex_1()
                .w_full()
                .overflow_hidden()
                .bg(rgb(body_bg))
                .when_some(fallback_content, |el, panel| el.child(panel))
                .child(bounds_canvas)
                .when(self.write_access_prompt_open, |el| {
                    el.child(self.render_write_access_modal(&app_title, cx))
                })
                .when(!self.write_access_prompt_open, |el| {
                    el.when_some(self.custom_popup.clone(), |el, popup| {
                        el.child(self.render_custom_popup_modal(&popup, cx))
                    })
                })
                .into_any_element()
        };

        let footer_bot_username = self.bot_username.clone();
        let has_bottom_buttons = self.disclaimer_accepted
            && (self
                .ui_schema
                .main_button
                .as_ref()
                .is_some_and(|b| b.is_visible)
                || self
                    .ui_schema
                    .secondary_button
                    .as_ref()
                    .is_some_and(|b| b.is_visible));

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
            // 1. Clean Telegram Desktop Mini App Header
            .child(
                div()
                    .w_full()
                    .h(px(50.0))
                    .px(px(12.0))
                    .bg(rgb(header_bg))
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
                            .gap(px(4.0))
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
                                                if this.has_native_webview() {
                                                    this.dispatch_js_event(
                                                        "back_button_pressed",
                                                        "{}",
                                                    );
                                                } else {
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
                                                }
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
                                            if (this.is_dirty || this.closing_confirmation)
                                                && !this.confirm_close_open
                                            {
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
                            .items_center()
                            .gap(px(5.0))
                            .child(
                                div()
                                    .text_size(px(14.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(app_title.clone()),
                            )
                            .child(tg_icon(IconName::Verified, 13.0, pal.accent)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .when(show_settings, |row| {
                                let b_user = self.bot_username.clone();
                                let s_name = short_name.clone();
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
                                            cx.listener(move |this, _, _, cx| {
                                                if this.has_native_webview() {
                                                    this.dispatch_js_event(
                                                        "settings_button_pressed",
                                                        "{}",
                                                    );
                                                } else {
                                                    this.pending_actions.push_back(
                                                        BotWebViewPendingAction::SendAppAction {
                                                            bot_username: b_user.clone(),
                                                            short_name: s_name.clone(),
                                                            req: MiniAppActionRequest {
                                                                action_type:
                                                                    "settings_button_click"
                                                                        .to_string(),
                                                                chat_id: chat_id_opt,
                                                                ..Default::default()
                                                            },
                                                        },
                                                    );
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(tg_icon(IconName::SettingsGear, 15.0, pal.icon_fg)),
                                )
                            })
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
                                            this.minimize();
                                            this.pending_actions
                                                .push_back(BotWebViewPendingAction::Minimize);
                                            cx.notify();
                                        }),
                                    )
                                    .child("—"),
                            )
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
            // 2. Overflow Menu (⋮)
            .when(self.menu_open, |el| {
                let b_user_profile = self.bot_username.clone();
                let b_user_rm = self.bot_username.clone();
                let s_name_rm = short_name.clone();
                let web_path = browser_target.clone();
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
                            tr_str(lang, "刷新页面", "Reload Page"),
                            pal.text_primary,
                            pal,
                            |this, _| {
                                this.menu_open = false;
                                this.reload_webview();
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-open-bot",
                            tr_str(lang, "查看机器人", "View Bot"),
                            pal.accent,
                            pal,
                            move |this, _| {
                                this.menu_open = false;
                                this.minimize();
                                this.pending_actions.push_back(
                                    BotWebViewPendingAction::OpenBotProfile(
                                        b_user_profile.clone(),
                                    ),
                                );
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
                            "miniapp-menu-browser",
                            tr_str(lang, "浏览器打开", "Open in Browser"),
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
                            "miniapp-menu-tos",
                            tr_str(lang, "服务条款", "Terms of Service"),
                            pal.text_primary,
                            pal,
                            |this, _| {
                                this.menu_open = false;
                                this.minimize();
                                this.pending_actions
                                    .push_back(BotWebViewPendingAction::OpenMiniAppTos);
                            },
                            cx,
                        ))
                        .child(overflow_menu_btn(
                            "miniapp-menu-remove",
                            tr_str(lang, "移除小程序", "Remove App"),
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
            // 3. Closing Confirmation Banner
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
                                    "您确定要关闭此小程序吗？未保存的更改可能会丢失。",
                                    "Changes you made may not be saved. Close anyway?",
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
            // 4. WebView Viewport (live WKWebView on macOS, GPUI fallback in offline tests) + overlays
            .child(main_viewport)
            // 5. Native Telegram MainButton & SecondaryButton Bar
            .when(has_bottom_buttons, |el| {
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
                            let bg_col = Self::parse_hex_color(sb.color.as_deref(), pal.card_bg);
                            let fg_col =
                                Self::parse_hex_color(sb.text_color.as_deref(), pal.text_primary);
                            let active = sb.is_active;
                            let label = if sb.is_progress_visible {
                                format!("⏳ {}", sb.text)
                            } else {
                                sb.text.clone()
                            };
                            row.child(
                                div()
                                    .id("miniapp-secondary-btn")
                                    .when(active, |d| d.cursor_pointer())
                                    .flex_1()
                                    .h(px(38.0))
                                    .rounded(px(9.0))
                                    .bg(rgb(bg_col))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(fg_col))
                                    .when(active, |d| {
                                        d.on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                if this.has_native_webview() {
                                                    this.dispatch_js_event(
                                                        "secondary_button_pressed",
                                                        "{}",
                                                    );
                                                } else {
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
                                                }
                                                cx.notify();
                                            }),
                                        )
                                    })
                                    .child(label),
                            )
                        })
                        .when_some(main_b, |row, mb| {
                            let bg_col = Self::parse_hex_color(mb.color.as_deref(), pal.accent);
                            let fg_col =
                                Self::parse_hex_color(mb.text_color.as_deref(), 0xFFFFFF);
                            let sends_data = mb.sends_data;
                            let active = mb.is_active;
                            let label = if self.is_loading || mb.is_progress_visible {
                                format!("⏳ {}", mb.text)
                            } else {
                                mb.text.clone()
                            };
                            row.child(
                                div()
                                    .id("miniapp-main-btn")
                                    .when(active, |d| d.cursor_pointer())
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
                                    .when(active, |d| {
                                        d.on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                if this.has_native_webview() {
                                                    this.dispatch_js_event(
                                                        "main_button_pressed",
                                                        "{}",
                                                    );
                                                } else {
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
                                                }
                                                cx.notify();
                                            }),
                                        )
                                    })
                                    .child(label),
                            )
                        }),
                )
            })
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
            })
            // 6. Clean @bot_username Footer Bar
            .child(
                div()
                    .id("miniapp-bot-footer")
                    .cursor_pointer()
                    .w_full()
                    .h(px(30.0))
                    .px(px(12.0))
                    .bg(rgb(pal.top_bar_bg))
                    .border_t_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.minimize();
                            this.pending_actions.push_back(
                                BotWebViewPendingAction::OpenBotProfile(
                                    footer_bot_username.clone(),
                                ),
                            );
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(pal.text_secondary))
                            .child(bot_sub),
                    ),
            );

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
                    if !this.is_dirty && !this.closing_confirmation {
                        this.minimize();
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
