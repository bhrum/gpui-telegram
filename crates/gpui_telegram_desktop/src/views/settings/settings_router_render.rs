//! Rendering implementation for `SettingsRouter` modal overlay, header navigation bar,
//! lazy active section mounting, and interface language selector.

use super::settings_router::SettingsRouter;
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName};
use crate::views::settings::settings_type::SettingsSectionType;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, rgb, rgba, AnyElement, Context, FontWeight, InteractiveElement, IntoElement,
    MouseButton, ParentElement, Render, StatefulInteractiveElement, Styled, Window,
};

impl SettingsRouter {
    /// Renders an individual language card that selects `lang_code` on click.
    fn render_language_option(
        &self,
        lang_code: &'static str,
        badge_code: &'static str,
        title: &'static str,
        subtitle: &'static str,
        is_selected: bool,
        status_label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let pal = self.palette;
        let badge_fg = if is_selected {
            pal.active_button_fg
        } else {
            pal.text_primary
        };
        let badge_icon_fg = if is_selected {
            pal.active_button_fg
        } else {
            pal.accent
        };
        div()
            .cursor_pointer()
            .p(px(pal.scaled_px(16.0)))
            .rounded(px(pal.scaled_px(10.0)))
            .bg(rgb(pal.card_bg))
            .border_1()
            .border_color(if is_selected {
                rgb(pal.accent)
            } else {
                rgb(pal.border_fg)
            })
            .hover(move |s| s.bg(rgb(pal.hover_bg)))
            .flex()
            .items_center()
            .justify_between()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.select_language_code(lang_code, cx);
                }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(pal.scaled_px(12.0)))
                    .child(
                        div()
                            .px(px(pal.scaled_px(10.0)))
                            .py(px(pal.scaled_px(7.0)))
                            .rounded(px(pal.scaled_px(8.0)))
                            .bg(rgb(if is_selected { pal.accent } else { pal.input_bg }))
                            .border_1()
                            .border_color(rgb(if is_selected {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .flex()
                            .items_center()
                            .gap(px(pal.scaled_px(5.0)))
                            .child(tg_icon(IconName::Translate, pal.scaled_px(13.0), badge_icon_fg))
                            .child(
                                div()
                                    .text_size(px(pal.scaled_px(12.0)))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(badge_fg))
                                    .child(badge_code),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(pal.scaled_px(15.0)))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(title),
                            )
                            .child(
                                div()
                                    .text_size(px(pal.scaled_px(12.0)))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(subtitle),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(pal.scaled_px(6.0)))
                    .when(is_selected, |el| {
                        el.child(tg_icon(IconName::SelectCheck, pal.scaled_px(14.0), pal.accent))
                    })
                    .child(
                        div()
                            .text_size(px(pal.scaled_px(14.0)))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_selected {
                                rgb(pal.accent)
                            } else {
                                rgb(pal.text_secondary)
                            })
                            .child(status_label),
                    ),
            )
    }

    /// Renders the built-in interface language selector section.
    pub(super) fn render_language_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_zh = self.is_chinese_locale;
        let pal = self.palette;
        let lang = self.lang_id;
        div()
            .flex()
            .flex_col()
            .gap(px(pal.scaled_px(16.0)))
            .p(px(pal.scaled_px(24.0)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(pal.scaled_px(8.0)))
                    .child(tg_icon(IconName::Translate, pal.scaled_px(20.0), pal.accent))
                    .child(
                        div()
                            .text_size(px(pal.scaled_px(20.0)))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "界面语言设置", "Interface Language Settings")),
                    ),
            )
            .child(
                div()
                    .text_size(px(pal.scaled_px(13.0)))
                    .text_color(rgb(pal.text_secondary))
                    .child(if is_zh {
                        "选择应用的默认显示语言。修改后将即时生效并持久化。"
                    } else {
                        "Choose the default display language for the app. Changes apply and persist immediately."
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(pal.scaled_px(12.0)))
                    .child(self.render_language_option(
                        "zh-CN",
                        "ZH",
                        "简体中文 (Simplified Chinese)",
                        "zh-CN • 完整翻译支持",
                        is_zh,
                        if is_zh { "已启用" } else { "选择" },
                        cx,
                    ))
                    .child(self.render_language_option(
                        "en-US",
                        "EN",
                        "English (US)",
                        "en-US • Default English",
                        !is_zh,
                        if !is_zh { "Active" } else { "Select" },
                        cx,
                    )),
            )
    }

    /// Resolves and lazily materializes the currently active settings section view.
    fn render_active_section(&mut self, cx: &mut Context<Self>) -> AnyElement {
        match self.settings_section {
            SettingsSectionType::Main => self.ensure_main(cx).into_any_element(),
            SettingsSectionType::Information => self.ensure_information(cx).into_any_element(),
            SettingsSectionType::Notifications => self.ensure_notifications(cx).into_any_element(),
            SettingsSectionType::NotificationsReactions => {
                self.ensure_notif_reactions(cx).into_any_element()
            }
            SettingsSectionType::NotificationsType => self.ensure_notif_type(cx).into_any_element(),
            SettingsSectionType::PrivacySecurity => self.ensure_privacy(cx).into_any_element(),
            SettingsSectionType::BlockedPeers => self.ensure_blocked_peers(cx).into_any_element(),
            SettingsSectionType::ActiveSessions => {
                self.ensure_active_sessions(cx).into_any_element()
            }
            SettingsSectionType::LocalPasscode => {
                self.ensure_local_passcode(cx).into_any_element()
            }
            SettingsSectionType::Passkeys => self.ensure_passkeys(cx).into_any_element(),
            SettingsSectionType::GlobalTtl => self.ensure_global_ttl(cx).into_any_element(),
            SettingsSectionType::Websites => self.ensure_websites(cx).into_any_element(),
            SettingsSectionType::Chat => self.ensure_chat(cx).into_any_element(),
            SettingsSectionType::Folders => self.ensure_folders(cx).into_any_element(),
            SettingsSectionType::Advanced => self.ensure_advanced(cx).into_any_element(),
            SettingsSectionType::LocalStorage => self.ensure_local_storage(cx).into_any_element(),
            SettingsSectionType::PowerSaving => self.ensure_power_saving(cx).into_any_element(),
            SettingsSectionType::Calls => self.ensure_calls(cx).into_any_element(),
            SettingsSectionType::Credits => self.ensure_credits(cx).into_any_element(),
            SettingsSectionType::Premium => self.ensure_premium(cx).into_any_element(),
            SettingsSectionType::Shortcuts => self.ensure_shortcuts(cx).into_any_element(),
            SettingsSectionType::Business => self.ensure_business(cx).into_any_element(),
            SettingsSectionType::Computers => self.ensure_computers(cx).into_any_element(),
            SettingsSectionType::AiProviders => self.ensure_ai_providers(cx).into_any_element(),
            SettingsSectionType::Search => self.ensure_search(cx).into_any_element(),
            SettingsSectionType::Language => self.render_language_settings(cx).into_any_element(),
        }
    }

    /// Renders the settings navigation header bar and scrollable active section viewport.
    fn render_settings_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_root = self.settings_section.is_root();
        let zh = self.is_chinese_locale;
        let pal = self.palette;
        let lang = self.lang_id;
        let title = self.settings_section.display_title_localized(zh);
        let section_icon = self.settings_section.icon_name();

        let back_label = if is_root {
            tr_str(lang, "返回对话", "Back to Chats").to_string()
        } else if let Some(&prev) = self.settings_stack.last() {
            if zh {
                format!("返回 {}", prev.display_title_localized(true))
            } else {
                prev.display_title_localized(false).to_string()
            }
        } else {
            tr_str(lang, "返回", "Back").to_string()
        };

        let crumbs = self.settings_section.breadcrumb_path();

        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.window_bg))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(pal.scaled_px(20.0)))
                    .py(px(pal.scaled_px(12.0)))
                    .bg(rgb(pal.topbar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(pal.scaled_px(12.0)))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .gap(px(pal.scaled_px(6.0)))
                                    .px(px(pal.scaled_px(12.0)))
                                    .py(px(pal.scaled_px(6.0)))
                                    .rounded(px(pal.scaled_px(6.0)))
                                    .bg(if is_root {
                                        rgb(pal.accent)
                                    } else {
                                        rgb(pal.input_bg)
                                    })
                                    .hover(move |s| {
                                        s.bg(if is_root {
                                            rgb(pal.accent_hover)
                                        } else {
                                            rgb(pal.hover_bg)
                                        })
                                    })
                                    .text_size(px(pal.scaled_px(13.0)))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(if is_root {
                                        rgb(pal.active_button_fg)
                                    } else {
                                        rgb(pal.text_primary)
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.navigate_back(cx);
                                        }),
                                    )
                                    .child(tg_icon(
                                        IconName::BackArrow,
                                        pal.scaled_px(14.0),
                                        if is_root {
                                            pal.active_button_fg
                                        } else {
                                            pal.icon_fg
                                        },
                                    ))
                                    .child(back_label),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(pal.scaled_px(2.0)))
                                    .when(crumbs.len() > 1, |el| {
                                        el.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(pal.scaled_px(4.0)))
                                                .children(crumbs.into_iter().enumerate().map(
                                                    |(idx, sec)| {
                                                        let is_current =
                                                            sec == self.settings_section;
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(pal.scaled_px(4.0)))
                                                            .when(idx > 0, |sep| {
                                                                sep.child(
                                                                    div()
                                                                        .text_size(px(pal.scaled_px(11.0)))
                                                                        .text_color(rgb(
                                                                            pal.text_secondary,
                                                                        ))
                                                                        .child("›"),
                                                                )
                                                            })
                                                            .child(
                                                                div()
                                                                    .text_size(px(pal.scaled_px(11.0)))
                                                                    .text_color(if is_current {
                                                                        rgb(pal.accent)
                                                                    } else {
                                                                        rgb(pal.text_secondary)
                                                                    })
                                                                    .cursor_pointer()
                                                                    .hover(move |s| {
                                                                        s.text_color(rgb(
                                                                            pal.text_primary,
                                                                        ))
                                                                    })
                                                                    .on_mouse_down(
                                                                        MouseButton::Left,
                                                                        cx.listener(
                                                                            move |this, _, _, cx| {
                                                                                this.navigate_to(
                                                                                    sec, false, cx,
                                                                                );
                                                                            },
                                                                        ),
                                                                    )
                                                                    .child(
                                                                        sec.display_title_localized(
                                                                            zh,
                                                                        ),
                                                                    ),
                                                            )
                                                    },
                                                )),
                                        )
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(pal.scaled_px(8.0)))
                                            .child(tg_icon(section_icon, pal.scaled_px(18.0), pal.accent))
                                            .child(
                                                div()
                                                    .text_size(px(pal.scaled_px(16.0)))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(title),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(pal.scaled_px(8.0)))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(pal.scaled_px(10.0)))
                                    .py(px(pal.scaled_px(6.0)))
                                    .rounded(px(pal.scaled_px(6.0)))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(pal.scaled_px(5.0)))
                                    .text_size(px(pal.scaled_px(13.0)))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.navigate_to(
                                                SettingsSectionType::Search,
                                                false,
                                                cx,
                                            );
                                        }),
                                    )
                                    .child(tg_icon(IconName::Search, pal.scaled_px(13.0), pal.icon_fg))
                                    .child(tr_str(lang, "搜索", "Search")),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(pal.scaled_px(10.0)))
                                    .py(px(pal.scaled_px(6.0)))
                                    .rounded(px(pal.scaled_px(6.0)))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(pal.scaled_px(5.0)))
                                    .text_size(px(pal.scaled_px(13.0)))
                                    .text_color(rgb(pal.accent))
                                    .font_weight(FontWeight::BOLD)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.toggle_language(cx);
                                        }),
                                    )
                                    .child(tg_icon(IconName::Translate, pal.scaled_px(13.0), pal.accent))
                                    .child(tr_str(lang, "中文", "English")),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(pal.scaled_px(10.0)))
                                    .py(px(pal.scaled_px(6.0)))
                                    .rounded(px(pal.scaled_px(6.0)))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.danger)))
                                    .flex()
                                    .items_center()
                                    .gap(px(pal.scaled_px(5.0)))
                                    .text_size(px(pal.scaled_px(13.0)))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.close(cx);
                                        }),
                                    )
                                    .child(tg_icon(IconName::Close, pal.scaled_px(13.0), pal.icon_fg))
                                    .child(tr_str(lang, "关闭", "Close")),
                            ),
                    ),
            )
            .child(
                div()
                    .id("settings-scroll-view")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(self.render_active_section(cx)),
            )
    }
}

impl Render for SettingsRouter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let box_w = pal.scaled_px(540.0).min(880.0);
        let box_h = pal.scaled_px(650.0).min(840.0);
        div()
            .id("telegram-settings-modal-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .bg(rgba(0x00000088))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.close(cx);
                }),
            )
            .child(
                div()
                    .id("telegram-settings-modal-box")
                    .w(px(box_w))
                    .h(px(box_h))
                    .rounded(px(pal.scaled_px(14.0)))
                    .bg(rgb(pal.modal_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(self.render_settings_panel(cx)),
            )
    }
}
