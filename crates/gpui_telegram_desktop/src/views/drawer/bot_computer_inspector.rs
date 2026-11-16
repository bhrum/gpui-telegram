use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::{
    CLASSIC_INITIALS_AVATAR_SHAPE, GROK_AVATAR_SHAPES, PatchFabushiBotRequest,
};
use gpui_telegram_proto::{
    BotComputerProfileDto, ComputerNodeDto, ComputerNodeKind, ComputerRunPolicy,
};

use crate::components::grok_bot_avatar::{
    render_grok_bot_avatar, render_grok_persona_avatar, trigger_grok_avatar_bounce,
    trigger_grok_avatar_spin, GrokBotAvatarProps, GrokPersonaState, GROK_COLOR_PALETTE,
};
use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::computer_workbench::{decode_data_url_image, render_ime_text_preview};
use crate::views::drawer::inspector_tabs::{
    BotConfigFocusField, InspectorAction, InspectorSubpage,
};
use crate::views::ContextInspector;

fn node_endpoint(node: &ComputerNodeDto) -> &str {
    node.host
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&node.ip_address)
}

pub fn resolve_bot_computer_binding<'a>(
    computers: &'a [ComputerNodeDto],
    profile: Option<&BotComputerProfileDto>,
) -> (Option<&'a ComputerNodeDto>, Option<&'a ComputerNodeDto>) {
    let explicit = profile
        .and_then(|p| p.bound_node_id.as_deref())
        .and_then(|id| computers.iter().find(|c| c.node_id == id));
    let effective = explicit
        .or_else(|| computers.iter().find(|c| c.is_current_device))
        .or_else(|| computers.first());
    (explicit, effective)
}

fn computer_tier_badge(
    explicit: Option<&ComputerNodeDto>,
    effective: Option<&ComputerNodeDto>,
    lang: &str,
) -> (String, bool) {
    match explicit.or(effective) {
        Some(node) if !node.tier_badge.is_empty() => (node.tier_badge.clone(), true),
        Some(node) if node.kind == ComputerNodeKind::CloudServer => {
            (tr_str(lang, "云电脑", "Cloud").to_string(), true)
        }
        Some(node) if node.kind == ComputerNodeKind::LocalDevice => {
            (tr_str(lang, "本地", "Local").to_string(), true)
        }
        _ => (tr_str(lang, "免费", "Free").to_string(), false),
    }
}

fn format_policy_label(policy: ComputerRunPolicy, lang: &str) -> &'static str {
    if lang.starts_with("zh") {
        policy.label_zh()
    } else {
        policy.label_en()
    }
}

/// Formats a monthly active users count with thousands separators (e.g. `12,345`).
pub fn format_mau_count(mau: i64) -> String {
    let n = mau.max(0) as u64;
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

/// Formats the Telegram bot subtitle (`"{mau} 名每月活跃用户"` or `"机器人"`).
pub fn format_bot_mau_subtitle(mau: i64, lang: &str) -> String {
    if mau > 0 {
        let formatted = format_mau_count(mau);
        if lang.starts_with("zh") {
            format!("{formatted} 名每月活跃用户")
        } else {
            format!("{formatted} monthly users")
        }
    } else {
        tr_str(lang, "机器人", "bot").to_string()
    }
}

fn bot_action_pill(
    id: &'static str,
    icon: IconName,
    label: &str,
    pal: TdesktopThemePalette,
    on_tap: impl Fn(&mut ContextInspector, &mut Context<ContextInspector>) + 'static,
    cx: &mut Context<ContextInspector>,
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
        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
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
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(pal.text_link))
                .child(label.to_string()),
        )
}

fn bot_menu_row(
    id: &'static str,
    icon: IconName,
    label: &str,
    fg_color: u32,
    pal: TdesktopThemePalette,
    on_tap: impl Fn(&mut ContextInspector, &mut Context<ContextInspector>) + 'static,
    cx: &mut Context<ContextInspector>,
) -> Stateful<Div> {
    div()
        .id(id)
        .cursor_pointer()
        .w_full()
        .px(px(10.0))
        .py(px(8.0))
        .rounded(px(8.0))
        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
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
                .text_size(px(12.5))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(fg_color))
                .child(label.to_string()),
        )
}

pub fn render_bot_profile_main(
    inspector: &ContextInspector,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let model = &inspector.profile;
    let bot_id = model.bot_id;
    let is_owner_fabushi = model.is_viewer_owner && inspector.active_fabushi_bot.is_some();
    let is_verified = model.is_verified
        || model
            .username
            .trim()
            .trim_start_matches('@')
            .eq_ignore_ascii_case("BotFather");
    let is_blocked = model.is_blocked || inspector.is_contact_blocked;

    let avatar_color = if model.avatar_color != 0 {
        model.avatar_color
    } else {
        pal.accent
    };
    let fallback_initials = {
        let trimmed = model.avatar_initials.trim();
        if !trimmed.is_empty() && trimmed != "?" {
            trimmed.to_string()
        } else {
            let derived = crate::views::app_shell::shell_truth::derive_initials(&model.name);
            if derived.is_empty() {
                "BOT".to_string()
            } else {
                derived
            }
        }
    };

    let avatar = if let Some(ref path) = model.avatar_path {
        div()
            .w(px(80.0))
            .h(px(80.0))
            .rounded_full()
            .overflow_hidden()
            .border_1()
            .border_color(rgb(pal.border_fg))
            .child(img(path.clone()).w(px(80.0)).h(px(80.0)).object_fit(ObjectFit::Cover))
            .into_any_element()
    } else if is_owner_fabushi {
        if let Some(ref fab) = inspector.active_fabushi_bot {
            let spin_id = fab.agent_id.clone();
            let state = GrokPersonaState::from_activity_str(&fab.execution_state, false);
            let avatar_el = if fab
                .avatar_shape
                .eq_ignore_ascii_case(CLASSIC_INITIALS_AVATAR_SHAPE)
            {
                render_grok_persona_avatar(
                    &fab.agent_id,
                    &fab.avatar_shape,
                    &fab.avatar_color,
                    &fallback_initials,
                    80.0,
                    state,
                    pal.window_bg,
                    false,
                )
            } else {
                render_grok_bot_avatar(
                    GrokBotAvatarProps::new(&fab.agent_id, &fab.avatar_shape, &fab.avatar_color, 80.0)
                        .with_state(state)
                        .with_follow_pointer(true)
                        .with_surface_bg(pal.window_bg),
                )
            };
            div()
                .id("bot-inspector-grok-avatar")
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |_this, _, _, cx| {
                        trigger_grok_avatar_spin(&spin_id);
                        cx.notify();
                    }),
                )
                .child(avatar_el)
                .into_any_element()
        } else {
            div().into_any_element()
        }
    } else {
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
            .child(fallback_initials)
            .into_any_element()
    };

    let clean_username = model.username.trim().trim_start_matches('@').to_string();
    let display_username = if clean_username.is_empty() {
        String::new()
    } else {
        format!("@{clean_username}")
    };
    let share_link = if clean_username.is_empty() {
        model.name.clone()
    } else {
        format!("https://t.me/{clean_username}")
    };
    let subtitle_text = format_bot_mau_subtitle(model.bot_mau_count, lang);

    let has_main_app = model.bot_has_main_app || model.miniapp_url.is_some();
    let launch_uri = model.miniapp_url.clone().or_else(|| {
        (model.bot_has_main_app && !clean_username.is_empty())
            .then(|| format!("gpui-miniapp://{clean_username}/app"))
    });
    let has_help_cmd = model
        .bot_commands
        .iter()
        .any(|(c, _)| c.trim_start_matches('/').eq_ignore_ascii_case("help"));
    let has_settings_row = model.bot_has_settings
        || model
            .bot_commands
            .iter()
            .any(|(c, _)| c.trim_start_matches('/').eq_ignore_ascii_case("settings"));
    let privacy_url = if !model.bot_privacy_policy_url.trim().is_empty() {
        model.bot_privacy_policy_url.trim().to_string()
    } else {
        "https://telegram.org/privacy-tpa".to_string()
    };

    let mute_label = if model.is_notifications_enabled {
        tr_str(lang, "静音", "Mute")
    } else {
        tr_str(lang, "取消静音", "Unmute")
    };
    let copied_prefix = tr_str(lang, "已复制", "Copied").to_string();
    let copied_link_prefix = tr_str(lang, "已复制", "Copied").to_string();

    let mut action_rows = div()
        .w_full()
        .p(px(6.0))
        .rounded(px(12.0))
        .bg(rgb(pal.card_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .flex()
        .flex_col()
        .gap(px(2.0));

    if model.bot_can_join_groups {
        action_rows = action_rows.child(bot_menu_row(
            "bot-profile-add-to-group",
            IconName::GroupUsers,
            tr_str(lang, "添加到群组", "Add to Group or Channel"),
            pal.text_primary,
            pal,
            move |this, _| {
                this.pending_action = Some(InspectorAction::OpenAddBotToGroup(bot_id));
            },
            cx,
        ));
    }
    if has_help_cmd {
        action_rows = action_rows.child(bot_menu_row(
            "bot-profile-help",
            IconName::FolderBots,
            tr_str(lang, "机器人帮助", "Bot Help"),
            pal.text_primary,
            pal,
            |this, _| {
                this.pending_action = Some(InspectorAction::RunBotCommand("/help".to_string()));
            },
            cx,
        ));
    }
    if has_settings_row {
        action_rows = action_rows.child(bot_menu_row(
            "bot-profile-settings",
            IconName::SettingsGear,
            tr_str(lang, "机器人设置", "Bot Settings"),
            pal.text_primary,
            pal,
            move |this, _| {
                this.pending_action = Some(InspectorAction::OpenBotSettings(bot_id));
            },
            cx,
        ));
    }
    let priv_for_row = privacy_url.clone();
    action_rows = action_rows
        .child(bot_menu_row(
            "bot-profile-privacy",
            IconName::Lock,
            tr_str(lang, "机器人隐私政策", "Privacy Policy"),
            pal.text_primary,
            pal,
            move |this, _| {
                this.pending_action =
                    Some(InspectorAction::OpenBotPrivacyPolicy(priv_for_row.clone()));
            },
            cx,
        ))
        .child(div().w_full().h(px(1.0)).my(px(2.0)).bg(rgb(pal.border_fg)))
        .child(bot_menu_row(
            "bot-profile-report",
            IconName::DeleteTrash,
            tr_str(lang, "举报", "Report"),
            pal.danger,
            pal,
            move |this, _| {
                this.pending_action = Some(InspectorAction::ReportBot(bot_id));
            },
            cx,
        ))
        .child(bot_menu_row(
            "bot-profile-block-toggle",
            IconName::Close,
            if is_blocked {
                tr_str(lang, "取消屏蔽并重启机器人", "Restart & Unblock Bot")
            } else {
                tr_str(lang, "停用并屏蔽机器人", "Stop & Block Bot")
            },
            pal.danger,
            pal,
            move |this, _| {
                let next = !(this.profile.is_blocked || this.is_contact_blocked);
                this.profile.is_blocked = next;
                this.is_contact_blocked = next;
                this.pending_action = Some(InspectorAction::ToggleBlockBot(bot_id));
            },
            cx,
        ));

    let owner_section = if is_owner_fabushi {
        Some(render_owner_fabushi_and_computer_cards(inspector, pal, lang, cx))
    } else {
        None
    };

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(6.0))
                .py(px(6.0))
                .child(avatar)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap(px(5.0))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(model.name.clone()),
                        )
                        .when(is_verified, |row| {
                            row.child(tg_icon(IconName::Verified, 16.0, pal.accent))
                        }),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_secondary))
                        .child(subtitle_text),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(bot_action_pill(
                    "bot-action-message",
                    IconName::FolderChat,
                    tr_str(lang, "消息", "Message"),
                    pal,
                    |this, _| {
                        this.pending_action = Some(InspectorAction::FocusBotComposer);
                    },
                    cx,
                ))
                .child(bot_action_pill(
                    "bot-action-mute",
                    IconName::Mute,
                    mute_label,
                    pal,
                    |this, _| {
                        this.profile.is_notifications_enabled =
                            !this.profile.is_notifications_enabled;
                        this.pending_action = Some(InspectorAction::ToggleNotifications(
                            this.profile.is_notifications_enabled,
                        ));
                    },
                    cx,
                ))
                .child(bot_action_pill(
                    "bot-action-more",
                    IconName::MoreVertical,
                    tr_str(lang, "更多", "More"),
                    pal,
                    move |this, _| {
                        if this.profile.is_viewer_owner && this.active_fabushi_bot.is_some() {
                            this.subpage = InspectorSubpage::ComputersList;
                            this.pending_action = Some(InspectorAction::OpenBotComputersSubpage);
                        } else {
                            this.copied_feedback =
                                Some(format!("{copied_link_prefix}: {share_link}"));
                            this.pending_action =
                                Some(InspectorAction::CopyText(share_link.clone()));
                        }
                    },
                    cx,
                )),
        )
        .when(!model.bio.trim().is_empty() || !display_username.is_empty(), |col| {
            let mut info_card = div()
                .w_full()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0));

            if !model.bio.trim().is_empty() {
                info_card = info_card.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(12.5))
                                .text_color(rgb(pal.text_primary))
                                .child(model.bio.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(lang, "简介", "Info")),
                        ),
                );
            }

            if !display_username.is_empty() {
                let cp = display_username.clone();
                let cp_feedback = display_username.clone();
                if !model.bio.trim().is_empty() {
                    info_card = info_card.child(div().w_full().h(px(1.0)).bg(rgb(pal.border_fg)));
                }
                info_card = info_card.child(
                    div()
                        .id("bot-copy-username")
                        .cursor_pointer()
                        .rounded(px(6.0))
                        .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .py(px(2.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.copied_feedback =
                                    Some(format!("{copied_prefix}: {cp_feedback}"));
                                this.pending_action = Some(InspectorAction::CopyText(cp.clone()));
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
                                        .text_size(px(12.5))
                                        .text_color(rgb(pal.text_link))
                                        .child(display_username),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(lang, "用户名", "Username")),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .h(px(14.0))
                                        .rounded(px(3.0))
                                        .border_1()
                                        .border_color(rgb(pal.icon_fg))
                                        .p(px(2.0))
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .justify_between()
                                                .child(div().w(px(3.5)).h(px(3.5)).bg(rgb(pal.icon_fg)))
                                                .child(div().w(px(3.5)).h(px(3.5)).bg(rgb(pal.icon_fg))),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .justify_between()
                                                .child(div().w(px(3.5)).h(px(3.5)).bg(rgb(pal.icon_fg)))
                                                .child(div().w(px(2.5)).h(px(2.5)).bg(rgb(pal.accent))),
                                        ),
                                )
                                .child(tg_icon(IconName::Copy, 14.0, pal.icon_fg)),
                        ),
                );
            }

            col.child(info_card)
        })
        .when_some(copied_feedback_banner(inspector, pal), |col, banner| {
            col.child(banner)
        })
        .when(has_main_app, |col| {
            let url = launch_uri.clone().unwrap_or_default();
            col.child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .id("inspector-launch-miniapp-btn")
                            .cursor_pointer()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(10.0))
                            .bg(rgb(pal.accent))
                            .hover(|s| s.bg(rgb(pal.accent_hover)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.pending_action =
                                        Some(InspectorAction::LaunchMiniApp(url.clone()));
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
                                    .id("inspector-miniapp-tos-link")
                                    .cursor_pointer()
                                    .text_color(rgb(pal.text_link))
                                    .hover(|s| s.underline())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.pending_action =
                                                Some(InspectorAction::OpenMiniAppTos);
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
        .child(action_rows)
        .when_some(owner_section, |col, owner| col.child(owner))
        .into_any_element()
}

fn copied_feedback_banner(
    inspector: &ContextInspector,
    pal: TdesktopThemePalette,
) -> Option<AnyElement> {
    inspector.copied_feedback.as_ref().map(|hint| {
        div()
            .px(px(6.0))
            .text_size(px(11.0))
            .text_color(rgb(pal.success))
            .child(hint.clone())
            .into_any_element()
    })
}

fn render_owner_fabushi_and_computer_cards(
    inspector: &ContextInspector,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let model = &inspector.profile;
    let bot_id = model.bot_id;
    let profile_dto = inspector.bot_computer_profile.as_ref();
    let (explicit_node, effective_node) =
        resolve_bot_computer_binding(&inspector.computers, profile_dto);
    let (tier_label, has_bound_style) =
        computer_tier_badge(explicit_node, effective_node, lang);

    let bound_name = explicit_node
        .map(|n| n.display_name.clone())
        .or_else(|| effective_node.map(|n| format!("{} ({})", n.display_name, tr_str(lang, "当前设备", "Current"))))
        .unwrap_or_else(|| tr_str(lang, "未绑定", "Unbound").to_string());
    let model_name = profile_dto
        .map(|p| p.preferred_model.clone())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| tr_str(lang, "默认模型", "Default Model").to_string());
    let approval_mode = profile_dto
        .map(|p| p.approval_mode)
        .unwrap_or(ComputerRunPolicy::AskEveryTime);
    let persona_snippet = profile_dto
        .map(|p| p.persona_prompt.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| model.bot_persona_prompt.trim().to_string());

    let effective_node_id = effective_node.map(|n| n.node_id.clone());
    let detail_node_id = effective_node.map(|n| n.node_id.clone());
    let preview_title = if lang.starts_with("zh") {
        format!("{}的电脑", if model.name.is_empty() { "Agent" } else { model.name.as_str() })
    } else {
        format!("{}'s Computer", if model.name.is_empty() { "Agent" } else { model.name.as_str() })
    };

    let mini_preview_card = render_miniature_desktop_card(
        effective_node,
        &preview_title,
        pal,
        lang,
        cx.listener(move |this, _, _, cx| {
            if let Some(ref id) = effective_node_id {
                this.pending_action = Some(InspectorAction::OpenRemoteWorkbench {
                    node_id: id.clone(),
                });
            } else {
                this.subpage = InspectorSubpage::ComputersList;
                this.pending_action = Some(InspectorAction::OpenBotComputersSubpage);
            }
            cx.notify();
        }),
    );

    let in_attach = model.in_attach_menu;

    div()
        .w_full()
        .pt(px(4.0))
        .flex()
        .flex_col()
        .gap(px(12.0))
        .when_some(inspector.active_fabushi_bot.clone(), |col, fab| {
            let target_bot_id = fab.bot_user_id.max(0) as u64;
            let fab_bg = crate::components::avatar::parse_fabushi_hex_color(&fab.avatar_color)
                .unwrap_or(pal.accent);
            let highlight_fg = if fab.avatar_color.eq_ignore_ascii_case("black") {
                pal.accent
            } else {
                fab_bg
            };
            let mut shape_row = div().flex().flex_wrap().items_center().gap(px(5.0));
            for (idx, &shape) in GROK_AVATAR_SHAPES.iter().enumerate() {
                let selected = fab.avatar_shape.eq_ignore_ascii_case(shape);
                let shape_owned = shape.to_string();
                let agent_for_bounce = fab.agent_id.clone();
                shape_row = shape_row.child(
                    div()
                        .id(SharedString::from(format!("fabushi-insp-shape-{idx}")))
                        .cursor_pointer()
                        .p(px(4.0))
                        .rounded(px(9.0))
                        .bg(if selected {
                            TdesktopThemePalette::rgba_with_alpha(highlight_fg, 0.18)
                        } else {
                            rgb(pal.input_bg).into()
                        })
                        .border_1()
                        .border_color(rgb(if selected { highlight_fg } else { pal.border_fg }))
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                trigger_grok_avatar_bounce(&agent_for_bounce);
                                this.pending_action = Some(InspectorAction::PatchFabushiBot {
                                    bot_id: target_bot_id,
                                    req: PatchFabushiBotRequest {
                                        avatar_shape: Some(shape_owned.clone()),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(
                            render_grok_bot_avatar(
                                GrokBotAvatarProps::new(
                                    format!("insp-shape-{shape}"),
                                    shape,
                                    &fab.avatar_color,
                                    22.0,
                                )
                                .with_state(GrokPersonaState::Idle)
                                .with_paused(true)
                                .with_surface_bg(pal.card_bg),
                            ),
                        ),
                );
            }
            let classic_selected = fab
                .avatar_shape
                .eq_ignore_ascii_case(CLASSIC_INITIALS_AVATAR_SHAPE);
            let agent_for_classic = fab.agent_id.clone();
            shape_row = shape_row.child(
                div()
                    .id("fabushi-insp-shape-classic")
                    .cursor_pointer()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(px(9.0))
                    .bg(if classic_selected {
                        TdesktopThemePalette::rgba_with_alpha(highlight_fg, 0.18)
                    } else {
                        rgb(pal.input_bg).into()
                    })
                    .border_1()
                    .border_color(rgb(if classic_selected {
                        highlight_fg
                    } else {
                        pal.border_fg
                    }))
                    .text_size(px(10.5))
                    .font_weight(if classic_selected {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(rgb(if classic_selected {
                        highlight_fg
                    } else {
                        pal.text_primary
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            trigger_grok_avatar_bounce(&agent_for_classic);
                            this.pending_action = Some(InspectorAction::PatchFabushiBot {
                                bot_id: target_bot_id,
                                req: PatchFabushiBotRequest {
                                    avatar_shape: Some(CLASSIC_INITIALS_AVATAR_SHAPE.to_string()),
                                    ..Default::default()
                                },
                            });
                            cx.notify();
                        }),
                    )
                    .child("classic"),
            );

            let mut color_row = div().flex().flex_wrap().items_center().gap(px(6.0));
            for (idx, spec) in GROK_COLOR_PALETTE.iter().enumerate() {
                let selected = fab.avatar_color.eq_ignore_ascii_case(spec.id)
                    || crate::components::avatar::parse_fabushi_hex_color(&fab.avatar_color)
                        == Some(spec.swatch_rgb);
                let color_id = spec.id.to_string();
                let agent_for_bounce = fab.agent_id.clone();
                color_row = color_row.child(
                    div()
                        .id(SharedString::from(format!("fabushi-insp-color-{idx}")))
                        .cursor_pointer()
                        .w(px(20.0))
                        .h(px(20.0))
                        .rounded_full()
                        .bg(rgb(spec.swatch_rgb))
                        .border_2()
                        .border_color(rgb(if selected {
                            pal.text_primary
                        } else {
                            pal.border_fg
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                trigger_grok_avatar_bounce(&agent_for_bounce);
                                this.pending_action = Some(InspectorAction::PatchFabushiBot {
                                    bot_id: target_bot_id,
                                    req: PatchFabushiBotRequest {
                                        avatar_color: Some(color_id.clone()),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        ),
                );
            }

            let mut thinking_row = div().flex().flex_wrap().gap(px(5.0));
            for (idx, &lvl) in ["off", "low", "medium", "high"].iter().enumerate() {
                let selected = fab.thinking_level.eq_ignore_ascii_case(lvl);
                let lvl_owned = lvl.to_string();
                thinking_row = thinking_row.child(
                    div()
                        .id(SharedString::from(format!("fabushi-insp-thinking-{idx}")))
                        .cursor_pointer()
                        .px(px(7.0))
                        .py(px(2.5))
                        .rounded(px(999.0))
                        .bg(if selected {
                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                        } else {
                            rgb(pal.input_bg).into()
                        })
                        .border_1()
                        .border_color(rgb(if selected { pal.accent } else { pal.border_fg }))
                        .text_size(px(10.5))
                        .font_weight(if selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .text_color(rgb(if selected { pal.accent } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.pending_action = Some(InspectorAction::PatchFabushiBot {
                                    bot_id: target_bot_id,
                                    req: PatchFabushiBotRequest {
                                        thinking_level: Some(lvl_owned.clone()),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(lvl),
                );
            }

            let mut approval_row = div().flex().flex_wrap().gap(px(5.0));
            for (idx, &mode) in ["prompt", "autonomous", "strict"].iter().enumerate() {
                let selected = fab.approval_mode.eq_ignore_ascii_case(mode);
                let mode_owned = mode.to_string();
                approval_row = approval_row.child(
                    div()
                        .id(SharedString::from(format!("fabushi-insp-approval-{idx}")))
                        .cursor_pointer()
                        .px(px(7.0))
                        .py(px(2.5))
                        .rounded(px(999.0))
                        .bg(if selected {
                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                        } else {
                            rgb(pal.input_bg).into()
                        })
                        .border_1()
                        .border_color(rgb(if selected { pal.accent } else { pal.border_fg }))
                        .text_size(px(10.5))
                        .font_weight(if selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .text_color(rgb(if selected { pal.accent } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.pending_action = Some(InspectorAction::PatchFabushiBot {
                                    bot_id: target_bot_id,
                                    req: PatchFabushiBotRequest {
                                        approval_mode: Some(mode_owned.clone()),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(mode),
                );
            }

            col.child(
                div()
                    .w_full()
                    .p(px(12.0))
                    .rounded(px(14.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
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
                                    .child(tr_str(lang, "Fabushi 智能体", "Fabushi Agent")),
                            )
                            .child(
                                div()
                                    .px(px(7.0))
                                    .py(px(1.5))
                                    .rounded(px(999.0))
                                    .bg(TdesktopThemePalette::rgba_with_alpha(highlight_fg, 0.14))
                                    .text_size(px(10.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(highlight_fg))
                                    .child(format!("isolated · {}/", fab.agent_id)),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!(
                                "Memory: {} · Skills: {} · State: {}",
                                fab.memory_entries_count, fab.skills_count, fab.execution_state
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(10.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(lang, "头像几何形", "Avatar Shape")),
                    )
                    .child(shape_row)
                    .child(
                        div()
                            .text_size(px(10.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(lang, "头像配色", "Avatar Color")),
                    )
                    .child(color_row)
                    .child(
                        div()
                            .text_size(px(10.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(lang, "思考深度", "Thinking Level")),
                    )
                    .child(thinking_row)
                    .child(
                        div()
                            .text_size(px(10.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(lang, "工具审批策略", "Approval Mode")),
                    )
                    .child(approval_row),
            )
        })
        .child(
            div()
                .w_full()
                .p(px(12.0))
                .rounded(px(14.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
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
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .child(preview_title),
                                )
                                .child(
                                    div()
                                        .px(px(7.0))
                                        .py(px(1.5))
                                        .rounded(px(999.0))
                                        .bg(if has_bound_style {
                                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14)
                                        } else {
                                            rgb(pal.input_bg).into()
                                        })
                                        .text_size(px(10.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(if has_bound_style {
                                            pal.accent
                                        } else {
                                            pal.text_secondary
                                        }))
                                        .child(tier_label),
                                ),
                        )
                        .child(
                            div()
                                .id("bot-computer-card-more")
                                .cursor_pointer()
                                .w(px(26.0))
                                .h(px(26.0))
                                .rounded_full()
                                .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                                .flex()
                                .items_center()
                                .justify_center()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        if let Some(ref id) = detail_node_id {
                                            this.pending_action =
                                                Some(InspectorAction::OpenComputerDetailInSettings {
                                                    node_id: id.clone(),
                                                });
                                        } else {
                                            this.subpage = InspectorSubpage::ComputersList;
                                            this.pending_action =
                                                Some(InspectorAction::OpenBotComputersSubpage);
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(tg_icon(IconName::MoreVertical, 14.0, pal.icon_fg)),
                        ),
                )
                .child(mini_preview_card)
                .child(
                    div()
                        .id("bot-open-computers-list-row")
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(8.0))
                        .rounded(px(10.0))
                        .bg(rgb(pal.input_bg))
                        .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.subpage = InspectorSubpage::ComputersList;
                                this.pending_action = Some(InspectorAction::OpenBotComputersSubpage);
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(tg_icon(IconName::ComputerDesktop, 15.0, pal.text_primary))
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(pal.text_primary))
                                        .child(tr_str(
                                            lang,
                                            "查看云电脑和本地电脑",
                                            "View Cloud & Local Computers",
                                        )),
                                ),
                        )
                        .child(tg_icon(IconName::ChevronRight, 13.0, pal.icon_fg)),
                ),
        )
        .child(
            div()
                .w_full()
                .p(px(12.0))
                .rounded(px(14.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(tg_icon(IconName::BotRobot, 15.0, pal.accent))
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .child(tr_str(lang, "AI Bot 配置", "AI Bot Configuration")),
                                ),
                        )
                        .child(
                            div()
                                .id("bot-open-config-editor")
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(7.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                                .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.24)))
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.open_bot_config_editor(cx);
                                    }),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(pal.accent))
                                        .child(tr_str(lang, "配置 AI Bot", "Configure")),
                                )
                                .child(tg_icon(IconName::ChevronRight, 11.0, pal.accent)),
                        ),
                )
                .child(config_summary_row(
                    tr_str(lang, "默认电脑", "Default Computer"),
                    &bound_name,
                    pal,
                ))
                .child(config_summary_row(
                    tr_str(lang, "首选模型", "Model"),
                    &model_name,
                    pal,
                ))
                .child(config_summary_row(
                    tr_str(lang, "执行策略", "Run Policy"),
                    format_policy_label(approval_mode, lang),
                    pal,
                ))
                .when(!persona_snippet.is_empty(), |col| {
                    col.child(
                        div()
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .text_size(px(11.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(persona_snippet),
                    )
                })
                .when(bot_id > 0, |col| {
                    col.child(
                        div()
                            .id("inspector-toggle-attach-btn")
                            .cursor_pointer()
                            .w_full()
                            .py(px(6.0))
                            .px(px(8.0))
                            .rounded(px(7.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.pending_action =
                                        Some(InspectorAction::ToggleAttachMenuBot(bot_id));
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "附件菜单快捷入口", "Attach Menu Shortcut"))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(if in_attach {
                                        pal.success
                                    } else {
                                        pal.text_secondary
                                    }))
                                    .child(if in_attach {
                                        tr_str(lang, "已添加", "Added")
                                    } else {
                                        tr_str(lang, "添加", "Add")
                                    }),
                            ),
                    )
                }),
        )
        .into_any_element()
}

fn render_miniature_desktop_card(
    node: Option<&ComputerNodeDto>,
    title: &str,
    pal: TdesktopThemePalette,
    lang: &str,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> AnyElement {
    let live_img = node
        .and_then(|n| n.latest_frame_data_url.as_deref())
        .and_then(decode_data_url_image);
    let node_online = node.is_some_and(|n| n.online);
    let label_text = node
        .map(|n| format!("{} · {}", n.display_name, node_endpoint(n)))
        .unwrap_or_else(|| tr_str(lang, "点击手动打开工作台", "Click to open computer").to_string());

    div()
        .id("bot-miniature-desktop-card")
        .cursor_pointer()
        .w_full()
        .h(px(170.0))
        .rounded(px(12.0))
        .overflow_hidden()
        .bg(rgb(0xEFF3F8))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .hover(|s| s.border_color(rgb(pal.accent)))
        .flex()
        .flex_col()
        .justify_between()
        .on_mouse_down(MouseButton::Left, on_click)
        .child(
            div()
                .px(px(10.0))
                .py(px(6.0))
                .bg(rgb(0xFFFFFF))
                .border_b_1()
                .border_color(rgb(0xE2E8F0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .child(div().w(px(7.0)).h(px(7.0)).rounded_full().bg(rgb(0xFF5F56)))
                        .child(div().w(px(7.0)).h(px(7.0)).rounded_full().bg(rgb(0xFFBD2E)))
                        .child(div().w(px(7.0)).h(px(7.0)).rounded_full().bg(rgb(0x27C93F))),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0x475569))
                        .truncate()
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .child(
                            div()
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded_full()
                                .bg(rgb(if node_online { 0x16A34A } else { 0x94A3B8 })),
                        )
                        .child(
                            div()
                                .text_size(px(9.5))
                                .text_color(rgb(0x64748B))
                                .child(if node_online {
                                    tr_str(lang, "在线", "Online")
                                } else {
                                    tr_str(lang, "待机", "Ready")
                                }),
                        ),
                ),
        )
        .child(
            div()
                .flex_1()
                .min_h(px(0.0))
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .p(px(8.0))
                .child(if let Some(arc_img) = live_img {
                    img(arc_img)
                        .size_full()
                        .object_fit(ObjectFit::Cover)
                        .rounded(px(6.0))
                        .into_any_element()
                } else {
                    div()
                        .w_full()
                        .h_full()
                        .rounded(px(8.0))
                        .bg(rgb(0xFFFFFF))
                        .border_1()
                        .border_color(rgb(0xE2E8F0))
                        .p(px(8.0))
                        .flex()
                        .flex_col()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(tg_icon(IconName::TerminalMcp, 13.0, pal.accent))
                                .child(
                                    div()
                                        .text_size(px(10.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0x1E293B))
                                        .truncate()
                                        .child(label_text),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(div().w(px(120.0)).h(px(5.0)).rounded(px(3.0)).bg(rgb(0xE2E8F0)))
                                .child(div().w(px(86.0)).h(px(5.0)).rounded(px(3.0)).bg(rgb(0xEEF2F7))),
                        )
                        .into_any_element()
                }),
        )
        .child(
            div()
                .w_full()
                .pb(px(6.0))
                .flex()
                .justify_center()
                .child(
                    div()
                        .px(px(12.0))
                        .py(px(4.0))
                        .rounded_full()
                        .bg(rgba(0xFFFFFFDD))
                        .border_1()
                        .border_color(rgb(0xE2E8F0))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(tg_icon(IconName::ComputerDesktop, 13.0, pal.accent))
                        .child(tg_icon(IconName::FolderAll, 13.0, 0x3B82F6))
                        .child(tg_icon(IconName::TerminalMcp, 13.0, 0x0F172A))
                        .child(tg_icon(IconName::FileDoc, 13.0, 0x64748B))
                        .child(tg_icon(IconName::SettingsGear, 13.0, 0x475569)),
                ),
        )
        .into_any_element()
}

pub fn render_bot_computers_subpage(
    inspector: &ContextInspector,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let bot_id = inspector.profile.bot_id;
    let bound_id = inspector
        .bot_computer_profile
        .as_ref()
        .and_then(|p| p.bound_node_id.as_deref());

    let locals: Vec<&ComputerNodeDto> = inspector
        .computers
        .iter()
        .filter(|c| c.kind == ComputerNodeKind::LocalDevice)
        .collect();
    let clouds: Vec<&ComputerNodeDto> = inspector
        .computers
        .iter()
        .filter(|c| c.kind == ComputerNodeKind::CloudServer)
        .collect();

    let mut local_section = div().flex().flex_col().gap(px(8.0)).child(
        div()
            .text_size(px(12.5))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(pal.text_primary))
            .child(tr_str(lang, "本地电脑", "Local Computers")),
    );
    if locals.is_empty() {
        local_section = local_section.child(
            div()
                .p(px(12.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .text_size(px(12.0))
                .text_color(rgb(pal.text_secondary))
                .child(tr_str(
                    lang,
                    "暂未检测到已登录账号的本地电脑",
                    "No local computers registered on this account yet.",
                )),
        );
    } else {
        for (idx, node) in locals.into_iter().enumerate() {
            let is_bound = bound_id == Some(node.node_id.as_str());
            local_section = local_section.child(render_computer_item_card(
                node,
                is_bound,
                bot_id,
                idx,
                true,
                pal,
                lang,
                cx,
            ));
        }
    }

    let mut cloud_section = div().flex().flex_col().gap(px(8.0)).child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_size(px(12.5))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.text_primary))
                    .child(tr_str(lang, "云电脑", "Cloud Computers")),
            )
            .child(
                div()
                    .id("bot-create-cloud-btn")
                    .cursor_pointer()
                    .px(px(9.0))
                    .py(px(4.0))
                    .rounded(px(999.0))
                    .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                    .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.25)))
                    .text_size(px(11.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(pal.accent))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.pending_action =
                                Some(InspectorAction::OpenCreateCloudComputerModal {
                                    bind_bot_id: (bot_id > 0).then_some(bot_id),
                                });
                            cx.notify();
                        }),
                    )
                    .child(tr_str(lang, "+ 创建", "+ Create")),
            ),
    );

    if clouds.is_empty() {
        cloud_section = cloud_section.child(
            div()
                .w_full()
                .p(px(18.0))
                .rounded(px(14.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded(px(12.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.12))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(tg_icon(IconName::ComputerDesktop, 20.0, pal.accent)),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .text_center()
                        .child(tr_str(
                            lang,
                            "每个 Agent 都拥有自己的电脑",
                            "Every Agent gets its own computer",
                        )),
                )
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(pal.text_secondary))
                        .text_center()
                        .child(tr_str(
                            lang,
                            "只有在你的 Agent 需要保持 24/7 运行时，才添加一台常驻云电脑。",
                            "Add an always-on cloud computer only when your Agent needs 24/7 execution.",
                        )),
                ),
        );
    } else {
        for (idx, node) in clouds.into_iter().enumerate() {
            let is_bound = bound_id == Some(node.node_id.as_str());
            cloud_section = cloud_section.child(render_computer_item_card(
                node,
                is_bound,
                bot_id,
                idx,
                false,
                pal,
                lang,
                cx,
            ));
        }
    }

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        .child(local_section)
        .child(cloud_section)
        .into_any_element()
}

#[allow(clippy::too_many_arguments)]
fn render_computer_item_card(
    node: &ComputerNodeDto,
    is_bound: bool,
    bot_id: u64,
    idx: usize,
    is_local: bool,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let node_id_bind = node.node_id.clone();
    let node_id_open = node.node_id.clone();
    let node_id_detail = node.node_id.clone();
    let badge_text = if is_local {
        if node.is_current_device {
            tr_str(lang, "当前设备", "Current Device").to_string()
        } else if node.online {
            tr_str(lang, "在线 · 同账号登录设备", "Online · Same Account").to_string()
        } else {
            tr_str(lang, "离线", "Offline").to_string()
        }
    } else if node.online {
        tr_str(lang, "在线", "Online").to_string()
    } else {
        tr_str(lang, "离线", "Offline").to_string()
    };

    let endpoint = node_endpoint(node);
    let subtitle = if is_local {
        endpoint.to_string()
    } else {
        format!("{}:{}", endpoint, node.ssh_port.unwrap_or(22))
    };

    div()
        .w_full()
        .p(px(10.0))
        .rounded(px(12.0))
        .bg(rgb(pal.card_bg))
        .border_1()
        .border_color(rgb(if is_bound { pal.accent } else { pal.border_fg }))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .min_w(px(0.0))
                        .flex_1()
                        .child(
                            div()
                                .w(px(32.0))
                                .h(px(32.0))
                                .rounded(px(9.0))
                                .bg(rgb(pal.input_bg))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(tg_icon(IconName::ComputerDesktop, 16.0, pal.accent)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w(px(0.0))
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .truncate()
                                        .child(node.display_name.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(5.0))
                                        .child(
                                            div()
                                                .w(px(6.0))
                                                .h(px(6.0))
                                                .rounded_full()
                                                .bg(rgb(if node.online {
                                                    pal.success
                                                } else {
                                                    pal.text_secondary
                                                })),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(pal.text_secondary))
                                                .truncate()
                                                .child(format!("{subtitle} · {badge_text}")),
                                        ),
                                ),
                        ),
                )
                .child(
                    div()
                        .id((if is_local { "local-detail" } else { "cloud-detail" }, idx))
                        .cursor_pointer()
                        .w(px(26.0))
                        .h(px(26.0))
                        .rounded_full()
                        .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.pending_action =
                                    Some(InspectorAction::OpenComputerDetailInSettings {
                                        node_id: node_id_detail.clone(),
                                    });
                                cx.notify();
                            }),
                        )
                        .child(tg_icon(IconName::ChevronRight, 13.0, pal.icon_fg)),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .id((if is_local { "local-bind" } else { "cloud-bind" }, idx))
                        .cursor_pointer()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(7.0))
                        .bg(if is_bound {
                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                        } else {
                            rgb(pal.input_bg).into()
                        })
                        .border_1()
                        .border_color(rgb(if is_bound { pal.accent } else { pal.border_fg }))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(if is_bound {
                            pal.accent
                        } else {
                            pal.text_primary
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                let target = if is_bound {
                                    None
                                } else {
                                    Some(node_id_bind.clone())
                                };
                                this.pending_action = Some(InspectorAction::BindBotToComputer {
                                    bot_id,
                                    node_id: target,
                                });
                                cx.notify();
                            }),
                        )
                        .child(if is_bound {
                            tr_str(lang, "已设为默认", "Default")
                        } else if is_local {
                            tr_str(lang, "设为默认", "Set Default")
                        } else {
                            tr_str(lang, "绑定此 Bot", "Bind Bot")
                        }),
                )
                .child(
                    div()
                        .id((if is_local { "local-control" } else { "cloud-control" }, idx))
                        .cursor_pointer()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(7.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(pal.border_fg))
                        .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.pending_action = Some(InspectorAction::OpenRemoteWorkbench {
                                    node_id: node_id_open.clone(),
                                });
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "远程操控", "Remote Control")),
                ),
        )
        .into_any_element()
}

pub fn render_bot_config_editor(
    inspector: &ContextInspector,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let bot_id = inspector.profile.bot_id;
    let bound_id = inspector
        .bot_computer_profile
        .as_ref()
        .and_then(|p| p.bound_node_id.as_deref());

    let (persona_text, persona_preedit) = inspector
        .persona_input
        .as_ref()
        .map(|c| c.read(cx).text_and_preedit(inspector.bot_draft_persona.clone()))
        .unwrap_or_else(|| (inspector.bot_draft_persona.clone(), None));
    let (model_text, model_preedit) = inspector
        .model_input
        .as_ref()
        .map(|c| c.read(cx).text_and_preedit(inspector.bot_draft_model.clone()))
        .unwrap_or_else(|| (inspector.bot_draft_model.clone(), None));

    let model_presets = ["gpt-5.4", "claude-sonnet-4.6", "gemini-2.5-pro", "deepseek-reasoner"];
    let policy_choices = [
        ComputerRunPolicy::AskEveryTime,
        ComputerRunPolicy::AllowReadOnly,
        ComputerRunPolicy::AlwaysAllowAuthorized,
        ComputerRunPolicy::Disabled,
    ];

    let mut target_chips = div().flex().flex_wrap().gap(px(6.0)).child(
        div()
            .id("bot-cfg-bind-none")
            .cursor_pointer()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(999.0))
            .bg(if bound_id.is_none() {
                TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
            } else {
                rgb(pal.input_bg).into()
            })
            .border_1()
            .border_color(rgb(if bound_id.is_none() {
                pal.accent
            } else {
                pal.border_fg
            }))
            .text_size(px(11.0))
            .text_color(rgb(if bound_id.is_none() {
                pal.accent
            } else {
                pal.text_primary
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.pending_action = Some(InspectorAction::BindBotToComputer {
                        bot_id,
                        node_id: None,
                    });
                    cx.notify();
                }),
            )
            .child(tr_str(lang, "跟随当前设备", "Follow Current")),
    );
    for (idx, node) in inspector.computers.iter().enumerate() {
        let selected = bound_id == Some(node.node_id.as_str());
        let nid = node.node_id.clone();
        target_chips = target_chips.child(
            div()
                .id(("bot-cfg-bind-node", idx))
                .cursor_pointer()
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(999.0))
                .bg(if selected {
                    TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                } else {
                    rgb(pal.input_bg).into()
                })
                .border_1()
                .border_color(rgb(if selected { pal.accent } else { pal.border_fg }))
                .text_size(px(11.0))
                .text_color(rgb(if selected {
                    pal.accent
                } else {
                    pal.text_primary
                }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.pending_action = Some(InspectorAction::BindBotToComputer {
                            bot_id,
                            node_id: Some(nid.clone()),
                        });
                        cx.notify();
                    }),
                )
                .child(node.display_name.clone()),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(
            div()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(tr_str(lang, "绑定默认运行电脑", "Bound Computer")),
                )
                .child(target_chips),
        )
        .child(
            div()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(tr_str(lang, "系统角色提示词", "System Persona Prompt")),
                )
                .child(
                    div()
                        .id("bot-persona-input-box")
                        .cursor_text()
                        .min_h(px(74.0))
                        .p(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(
                            if inspector.bot_config_focus == BotConfigFocusField::Persona {
                                pal.accent
                            } else {
                                pal.border_fg
                            },
                        ))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.ensure_bot_inputs(cx);
                                this.bot_config_focus = BotConfigFocusField::Persona;
                                cx.notify();
                            }),
                        )
                        .child(render_ime_text_preview(
                            &persona_text,
                            persona_preedit.as_deref(),
                            tr_str(
                                lang,
                                "描述此 Bot 的专长、语气与安全边界...",
                                "Describe persona, skills, and safety rules...",
                            ),
                            inspector.bot_config_focus == BotConfigFocusField::Persona,
                            false,
                            pal.text_primary,
                            pal,
                        )),
                ),
        )
        .child(
            div()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(tr_str(lang, "首选模型", "Preferred Model")),
                )
                .child(
                    div()
                        .id("bot-model-input-box")
                        .cursor_text()
                        .min_h(px(34.0))
                        .px(px(8.0))
                        .py(px(6.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(
                            if inspector.bot_config_focus == BotConfigFocusField::Model {
                                pal.accent
                            } else {
                                pal.border_fg
                            },
                        ))
                        .flex()
                        .items_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.ensure_bot_inputs(cx);
                                this.bot_config_focus = BotConfigFocusField::Model;
                                cx.notify();
                            }),
                        )
                        .child(render_ime_text_preview(
                            &model_text,
                            model_preedit.as_deref(),
                            "gpt-5.4 / claude-sonnet-4.6",
                            inspector.bot_config_focus == BotConfigFocusField::Model,
                            false,
                            pal.text_primary,
                            pal,
                        )),
                )
                .child(
                    div().flex().flex_wrap().gap(px(5.0)).children(
                        model_presets.into_iter().enumerate().map(|(i, preset)| {
                            div()
                                .id(("bot-model-preset", i))
                                .cursor_pointer()
                                .px(px(7.0))
                                .py(px(3.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                                .text_size(px(10.5))
                                .text_color(rgb(pal.text_secondary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.bot_draft_model = preset.to_string();
                                        cx.notify();
                                    }),
                                )
                                .child(preset)
                        }),
                    ),
                ),
        )
        .child(
            div()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(tr_str(lang, "默认执行审批策略", "Execution Approval Policy")),
                )
                .child(
                    div().flex().flex_wrap().gap(px(6.0)).children(
                        policy_choices.into_iter().enumerate().map(|(idx, pol)| {
                            let active = inspector.bot_draft_approval_mode == pol;
                            div()
                                .id(("bot-cfg-policy", idx))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(5.0))
                                .rounded(px(8.0))
                                .bg(if active {
                                    TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                                } else {
                                    rgb(pal.input_bg).into()
                                })
                                .border_1()
                                .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                                .text_size(px(11.0))
                                .font_weight(if active {
                                    FontWeight::SEMIBOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .text_color(rgb(if active {
                                    pal.accent
                                } else {
                                    pal.text_primary
                                }))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.bot_draft_approval_mode = pol;
                                        cx.notify();
                                    }),
                                )
                                .child(format_policy_label(pol, lang))
                        }),
                    ),
                ),
        )
        .child(
            div()
                .p(px(10.0))
                .rounded(px(12.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(bot_toggle_row(
                    "bot-toggle-cross-device",
                    tr_str(lang, "允许跨设备协同控制", "Allow Cross-Device Control"),
                    inspector.bot_draft_allow_cross_device,
                    pal,
                    cx.listener(|this, _, _, cx| {
                        this.bot_draft_allow_cross_device = !this.bot_draft_allow_cross_device;
                        cx.notify();
                    }),
                ))
                .child(bot_toggle_row(
                    "bot-toggle-cloud-self-cfg",
                    tr_str(lang, "允许 Bot 自主配置云电脑", "Allow Cloud Self-Configuration"),
                    inspector.bot_draft_allow_cloud_self_config,
                    pal,
                    cx.listener(|this, _, _, cx| {
                        this.bot_draft_allow_cloud_self_config =
                            !this.bot_draft_allow_cloud_self_config;
                        cx.notify();
                    }),
                )),
        )
        .child(
            div()
                .id("bot-save-config-btn")
                .cursor_pointer()
                .w_full()
                .h(px(36.0))
                .rounded(px(9.0))
                .bg(rgb(pal.accent))
                .hover(|s| s.bg(rgb(pal.accent_hover)))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.5))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(pal.active_button_fg))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.bot_config_focus = BotConfigFocusField::None;
                        this.pending_action = Some(InspectorAction::SaveBotComputerConfig {
                            bot_id,
                            persona_prompt: this.bot_draft_persona.clone(),
                            preferred_model: this.bot_draft_model.clone(),
                            approval_mode: this.bot_draft_approval_mode,
                            allow_cross_device_control: this.bot_draft_allow_cross_device,
                            allow_cloud_self_config: this.bot_draft_allow_cloud_self_config,
                        });
                        cx.notify();
                    }),
                )
                .child(tr_str(lang, "保存 AI Bot 配置", "Save AI Bot Config")),
        )
        .into_any_element()
}

fn config_summary_row(label: &str, value: &str, pal: TdesktopThemePalette) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(11.5))
                .text_color(rgb(pal.text_secondary))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_size(px(11.5))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(pal.text_primary))
                .truncate()
                .child(value.to_string()),
        )
}

fn bot_toggle_row(
    id: &'static str,
    label: &str,
    enabled: bool,
    pal: TdesktopThemePalette,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    div()
        .id(id)
        .cursor_pointer()
        .py(px(4.0))
        .flex()
        .items_center()
        .justify_between()
        .on_mouse_down(MouseButton::Left, on_click)
        .child(
            div()
                .text_size(px(11.5))
                .text_color(rgb(pal.text_primary))
                .child(label.to_string()),
        )
        .child(
            div()
                .w(px(34.0))
                .h(px(18.0))
                .rounded_full()
                .bg(rgb(if enabled { pal.accent } else { pal.input_bg }))
                .border_1()
                .border_color(rgb(if enabled { pal.accent } else { pal.border_fg }))
                .px(px(2.0))
                .flex()
                .items_center()
                .justify_end()
                .when(!enabled, |el| el.justify_start())
                .child(div().w(px(12.0)).h(px(12.0)).rounded_full().bg(rgb(0xFFFFFF))),
        )
}
