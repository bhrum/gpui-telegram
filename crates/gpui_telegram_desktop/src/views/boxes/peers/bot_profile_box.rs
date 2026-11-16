//! Centered Telegram Bot Profile Popup Box (`BotProfileBox`).
//!
//! Displays a bot's public profile (`BotPublicProfileDto`) without creating a DM chat
//! until the user explicitly clicks `消息` (Message) or `打开应用` (Open App).

use std::collections::VecDeque;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::{parse_miniapp_uri, BotPublicProfileDto};

use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::app_shell::peer_avatar_color;
use crate::views::app_shell::shell_truth::derive_initials;
use crate::views::drawer::bot_computer_inspector::format_bot_mau_subtitle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotProfileBoxAction {
    Close,
    OpenApp {
        bot_username: String,
        short_name: String,
    },
    OpenMessageChat {
        bot_id: u64,
        bot_username: String,
        display_name: String,
        description: String,
        existing_chat_id: Option<i64>,
        has_started_chat: bool,
    },
    ToggleMute,
    AddToGroup,
    SendBotHelp,
    OpenBotSettings,
    OpenPrivacyPolicy(String),
    OpenMiniAppTos,
    Report,
    ToggleStopAndBlock,
}

pub struct BotProfileBox {
    pub is_visible: bool,
    pub profile: BotPublicProfileDto,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_actions: VecDeque<BotProfileBoxAction>,
    pub copied_feedback: Option<String>,
}

impl BotProfileBox {
    pub fn new(
        profile: BotPublicProfileDto,
        palette: TdesktopThemePalette,
        lang_id: impl Into<String>,
    ) -> Self {
        Self {
            is_visible: true,
            profile,
            palette,
            language_id: lang_id.into(),
            pending_actions: VecDeque::new(),
            copied_feedback: None,
        }
    }

    pub fn set_profile(&mut self, profile: BotPublicProfileDto) {
        self.profile = profile;
        self.copied_feedback = None;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang_id: impl Into<String>) {
        self.language_id = lang_id.into();
    }

    pub fn open(&mut self, profile: BotPublicProfileDto) {
        self.profile = profile;
        self.is_visible = true;
        self.copied_feedback = None;
    }

    pub fn close(&mut self) {
        if self.is_visible {
            self.is_visible = false;
            self.pending_actions.push_back(BotProfileBoxAction::Close);
        }
    }

    pub fn emit(&mut self, action: BotProfileBoxAction) {
        self.pending_actions.push_back(action);
    }

    pub fn take_action(&mut self) -> Option<BotProfileBoxAction> {
        self.pending_actions.pop_front()
    }

    pub fn take_actions(&mut self) -> Vec<BotProfileBoxAction> {
        self.pending_actions.drain(..).collect()
    }
}

fn popup_action_pill(
    id: &'static str,
    icon: IconName,
    label: &str,
    pal: TdesktopThemePalette,
    on_tap: impl Fn(&mut BotProfileBox, &mut Context<BotProfileBox>) + 'static,
    cx: &mut Context<BotProfileBox>,
) -> Stateful<Div> {
    div()
        .id(id)
        .cursor_pointer()
        .flex_1()
        .py(px(8.0))
        .rounded(px(10.0))
        .bg(rgb(pal.card_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .hover(move |s| s.bg(rgb(pal.hover_bg)))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(4.0))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                on_tap(this, cx);
                cx.notify();
            }),
        )
        .child(tg_icon(icon, 16.0, pal.accent))
        .child(
            div()
                .text_size(px(11.5))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(pal.text_link))
                .child(label.to_string()),
        )
}

fn popup_menu_row(
    id: &'static str,
    icon: IconName,
    label: &str,
    fg_color: u32,
    pal: TdesktopThemePalette,
    on_tap: impl Fn(&mut BotProfileBox, &mut Context<BotProfileBox>) + 'static,
    cx: &mut Context<BotProfileBox>,
) -> Stateful<Div> {
    div()
        .id(id)
        .cursor_pointer()
        .w_full()
        .px(px(12.0))
        .py(px(9.0))
        .rounded(px(8.0))
        .hover(move |s| s.bg(rgb(pal.hover_bg)))
        .flex()
        .items_center()
        .gap(px(10.0))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                on_tap(this, cx);
                cx.notify();
            }),
        )
        .child(tg_icon(icon, 15.0, fg_color))
        .child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(fg_color))
                .child(label.to_string()),
        )
}

impl Render for BotProfileBox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let bot = &self.profile.bot;
        let bot_id = bot.bot_user_id.max(0) as u64;
        let clean_username = bot.username.trim().trim_start_matches('@').to_string();
        let display_name = if !bot.display_name.trim().is_empty() {
            bot.display_name.trim().to_string()
        } else if !clean_username.is_empty() {
            clean_username.clone()
        } else {
            "Bot".to_string()
        };
        let is_verified = bot.is_official || clean_username.eq_ignore_ascii_case("BotFather");
        let mau_subtitle = format_bot_mau_subtitle(bot.mau_count, lang);
        let avatar_color = peer_avatar_color(bot_id.max(1));
        let initials = {
            let d = derive_initials(&display_name);
            if d.is_empty() {
                "BOT".to_string()
            } else {
                d
            }
        };

        let profile_bio = if !bot.about.trim().is_empty() {
            bot.about.trim().to_string()
        } else if !bot.description.trim().is_empty() {
            bot.description.trim().to_string()
        } else {
            self.profile
                .main_app
                .as_ref()
                .map(|a| a.description.trim().to_string())
                .unwrap_or_default()
        };
        let stage_description = if !bot.description.trim().is_empty() {
            bot.description.trim().to_string()
        } else {
            profile_bio.clone()
        };

        let main_short = self
            .profile
            .main_app
            .as_ref()
            .map(|a| a.short_name.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                (!bot.main_app_short_name.trim().is_empty())
                    .then(|| bot.main_app_short_name.clone())
            })
            .or_else(|| self.profile.apps.first().map(|a| a.short_name.clone()))
            .or_else(|| parse_miniapp_uri(&bot.menu_button_url).map(|(_, s, _)| s));
        let has_main_app = bot.has_main_app
            || self.profile.main_app.is_some()
            || !self.profile.apps.is_empty()
            || (bot.menu_button_type == "web_app" && main_short.is_some());
        let app_short_name = main_short.unwrap_or_else(|| "app".to_string());

        let has_help_cmd = bot
            .commands
            .iter()
            .any(|c| c.command.trim_start_matches('/').eq_ignore_ascii_case("help"));
        let has_settings_row = bot.has_settings
            || bot
                .commands
                .iter()
                .any(|c| c.command.trim_start_matches('/').eq_ignore_ascii_case("settings"));
        let privacy_url = if !bot.privacy_policy_url.trim().is_empty() {
            bot.privacy_policy_url.trim().to_string()
        } else if let Some(ref ma) = self.profile.main_app {
            if !ma.privacy_policy_url.trim().is_empty() {
                ma.privacy_policy_url.trim().to_string()
            } else {
                "https://telegram.org/privacy-tpa".to_string()
            }
        } else {
            "https://telegram.org/privacy-tpa".to_string()
        };

        let at_username = if clean_username.is_empty() {
            String::new()
        } else {
            format!("@{clean_username}")
        };
        let share_link = if clean_username.is_empty() {
            display_name.clone()
        } else {
            format!("https://t.me/{clean_username}")
        };
        let existing_chat_id = self.profile.existing_chat_id;
        let has_started_chat = self.profile.has_started_chat;
        let is_muted = self.profile.is_muted;
        let is_blocked = self.profile.is_blocked;
        let can_join_groups = bot.can_join_groups;

        let msg_username = clean_username.clone();
        let msg_display = display_name.clone();
        let msg_desc = stage_description;
        let open_username = clean_username.clone();
        let open_short = app_short_name;
        let copied_prefix = tr_str(lang, "已复制", "Copied").to_string();
        let copied_link_prefix = copied_prefix.clone();

        let mut list_card = div()
            .w_full()
            .p(px(4.0))
            .rounded(px(12.0))
            .bg(rgb(pal.card_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(2.0));

        if can_join_groups {
            list_card = list_card.child(popup_menu_row(
                "bot-profile-box-add-group-row",
                IconName::GroupUsers,
                tr_str(lang, "添加到群组", "Add to Group or Channel"),
                pal.text_primary,
                pal,
                |this, _| {
                    this.emit(BotProfileBoxAction::AddToGroup);
                },
                cx,
            ));
        }
        if has_help_cmd {
            list_card = list_card.child(popup_menu_row(
                "bot-profile-box-help-row",
                IconName::FolderBots,
                tr_str(lang, "机器人帮助", "Bot Help"),
                pal.text_primary,
                pal,
                |this, _| {
                    this.emit(BotProfileBoxAction::SendBotHelp);
                },
                cx,
            ));
        }
        if has_settings_row {
            list_card = list_card.child(popup_menu_row(
                "bot-profile-box-settings-row",
                IconName::SettingsGear,
                tr_str(lang, "机器人设置", "Bot Settings"),
                pal.text_primary,
                pal,
                |this, _| {
                    this.emit(BotProfileBoxAction::OpenBotSettings);
                },
                cx,
            ));
        }
        list_card = list_card
            .child(popup_menu_row(
                "bot-profile-box-privacy-row",
                IconName::Lock,
                tr_str(lang, "机器人隐私政策", "Privacy Policy"),
                pal.text_primary,
                pal,
                move |this, _| {
                    this.emit(BotProfileBoxAction::OpenPrivacyPolicy(privacy_url.clone()));
                },
                cx,
            ))
            .child(div().w_full().h(px(1.0)).my(px(2.0)).bg(rgb(pal.border_fg)))
            .child(popup_menu_row(
                "bot-profile-box-report-row",
                IconName::DeleteTrash,
                tr_str(lang, "举报", "Report"),
                pal.danger,
                pal,
                |this, _| {
                    this.emit(BotProfileBoxAction::Report);
                },
                cx,
            ))
            .child(popup_menu_row(
                "bot-profile-box-block-row",
                IconName::Close,
                if is_blocked {
                    tr_str(lang, "取消屏蔽并重启机器人", "Restart & Unblock Bot")
                } else {
                    tr_str(lang, "停用并屏蔽机器人", "Stop & Block Bot")
                },
                pal.danger,
                pal,
                |this, _| {
                    this.profile.is_blocked = !this.profile.is_blocked;
                    this.emit(BotProfileBoxAction::ToggleStopAndBlock);
                },
                cx,
            ));

        div()
            .id("bot-profile-box-modal")
            .w(px(pal.scaled_px(380.0).min(520.0)))
            .max_h(px(pal.scaled_px(640.0)))
            .rounded(px(pal.scaled_px(14.0)))
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .shadow_lg()
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .h(px(48.0))
                    .flex_shrink_0()
                    .px(px(16.0))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "机器人信息", "Bot Info")),
                    )
                    .child(
                        div()
                            .id("bot-profile-box-close-btn")
                            .cursor_pointer()
                            .w(px(28.0))
                            .h(px(28.0))
                            .rounded_full()
                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.close();
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::Close, 15.0, pal.icon_fg)),
                    ),
            )
            .child(
                div()
                    .id("bot-profile-box-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_y_scroll()
                    .p(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .w(px(80.0))
                                    .h(px(80.0))
                                    .rounded_full()
                                    .bg(rgb(avatar_color))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(26.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(initials),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(5.0))
                                    .child(
                                        div()
                                            .text_size(px(17.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(display_name),
                                    )
                                    .when(is_verified, |row| {
                                        row.child(tg_icon(IconName::Verified, 16.0, pal.accent))
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(mau_subtitle),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(popup_action_pill(
                                "bot-profile-box-message-btn",
                                IconName::FolderChat,
                                tr_str(lang, "消息", "Message"),
                                pal,
                                move |this, _| {
                                    this.is_visible = false;
                                    this.emit(BotProfileBoxAction::OpenMessageChat {
                                        bot_id,
                                        bot_username: msg_username.clone(),
                                        display_name: msg_display.clone(),
                                        description: msg_desc.clone(),
                                        existing_chat_id,
                                        has_started_chat,
                                    });
                                },
                                cx,
                            ))
                            .child(popup_action_pill(
                                "bot-profile-box-mute-btn",
                                IconName::Mute,
                                if is_muted {
                                    tr_str(lang, "取消静音", "Unmute")
                                } else {
                                    tr_str(lang, "静音", "Mute")
                                },
                                pal,
                                |this, _| {
                                    this.profile.is_muted = !this.profile.is_muted;
                                    this.emit(BotProfileBoxAction::ToggleMute);
                                },
                                cx,
                            ))
                            .child(popup_action_pill(
                                "bot-profile-box-more-btn",
                                IconName::MoreVertical,
                                tr_str(lang, "更多", "More"),
                                pal,
                                move |this, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        share_link.clone(),
                                    ));
                                    this.copied_feedback = Some(format!(
                                        "{copied_link_prefix}: {share_link}"
                                    ));
                                },
                                cx,
                            )),
                    )
                    .when(!profile_bio.is_empty() || !at_username.is_empty(), |col| {
                        let mut info_card = div()
                            .w_full()
                            .p(px(12.0))
                            .rounded(px(12.0))
                            .bg(rgb(pal.card_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_col()
                            .gap(px(8.0));

                        if !profile_bio.is_empty() {
                            info_card = info_card.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(pal.text_primary))
                                            .child(profile_bio.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(tr_str(lang, "简介", "Description")),
                                    ),
                            );
                        }

                        if !at_username.is_empty() {
                            let cp = at_username.clone();
                            let cp_fb = at_username.clone();
                            if !profile_bio.is_empty() {
                                info_card = info_card
                                    .child(div().w_full().h(px(1.0)).bg(rgb(pal.border_fg)));
                            }
                            info_card = info_card.child(
                                div()
                                    .id("bot-profile-box-username-row")
                                    .cursor_pointer()
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .py(px(2.0))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                cp.clone(),
                                            ));
                                            this.copied_feedback =
                                                Some(format!("{copied_prefix}: {cp_fb}"));
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(2.0))
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .text_color(rgb(pal.text_link))
                                                    .child(at_username),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(tr_str(lang, "用户名", "Username")),
                                            ),
                                    )
                                    .child(tg_icon(IconName::Copy, 14.0, pal.icon_fg)),
                            );
                        }

                        col.child(info_card)
                    })
                    .when_some(self.copied_feedback.clone(), |col, hint| {
                        col.child(
                            div()
                                .px(px(4.0))
                                .text_size(px(11.0))
                                .text_color(rgb(pal.success))
                                .child(hint),
                        )
                    })
                    .when(has_main_app, |col| {
                        col.child(
                            div()
                                .w_full()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .id("bot-profile-box-open-app-btn")
                                        .cursor_pointer()
                                        .w_full()
                                        .h(px(38.0))
                                        .rounded(px(10.0))
                                        .bg(rgb(pal.accent))
                                        .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(13.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                this.is_visible = false;
                                                this.emit(BotProfileBoxAction::OpenApp {
                                                    bot_username: open_username.clone(),
                                                    short_name: open_short.clone(),
                                                });
                                                cx.notify();
                                            }),
                                        )
                                        .child(tr_str(lang, "打开应用", "Open App")),
                                )
                                .child(
                                    div()
                                        .px(px(4.0))
                                        .flex()
                                        .flex_wrap()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(
                                            lang,
                                            "启动此小程序即表示您同意 ",
                                            "By launching this Mini App, you agree to the ",
                                        ))
                                        .child(
                                            div()
                                                .id("bot-profile-box-tos-link")
                                                .cursor_pointer()
                                                .text_color(rgb(pal.text_link))
                                                .hover(|s| s.underline())
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        this.emit(BotProfileBoxAction::OpenMiniAppTos);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tr_str(
                                                    lang,
                                                    "小程序服务条款",
                                                    "Mini App Terms of Service",
                                                )),
                                        )
                                        .child(tr_str(lang, "。", ".")),
                                ),
                        )
                    })
                    .child(list_card),
            )
            .into_any_element()
    }
}
