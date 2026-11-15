//! Inspector Drawer Tabs models and Profile/Members GPUI renderers (`info/`).

use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::ContextInspector;
use gpui::prelude::FluentBuilder;
use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectorTab {
    Profile,
    Members,
    SharedMedia,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedMediaSubTab {
    Media,
    Files,
    Voice,
    Links,
    Music,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberItem {
    pub id: u64,
    pub name: String,
    pub initials: String,
    pub color: u32,
    pub role: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedMediaItem {
    pub id: u64,
    pub url: String,
    pub local_path: Option<String>,
    pub caption: String,
    pub is_video: bool,
    pub duration_sec: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFileItem {
    pub name: String,
    pub size_text: String,
    pub extension: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedLinkItem {
    pub title: String,
    pub url: String,
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedVoiceItem {
    pub id: u64,
    pub title: String,
    pub duration_sec: u32,
    pub date: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InspectorSubpage {
    #[default]
    Main,
    ComputersList,
    BotConfigEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BotConfigFocusField {
    #[default]
    None,
    Persona,
    Model,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectorAction {
    StartVoiceCall(String),
    StartVideoCall(String),
    CopyText(String),
    OpenAddMember,
    CycleMemberRole(u64),
    RevokeInviteLink,
    ToggleNotifications(bool),
    ChangeAutoDelete(String),
    OpenMedia { url: String, is_video: bool },
    OpenEditPeer,
    LaunchMiniApp(String),
    ToggleAttachMenuBot(u64),
    RunBotCommand(String),
    OpenBotComputersSubpage,
    CloseBotComputersSubpage,
    OpenRemoteWorkbench { node_id: String },
    OpenComputerDetailInSettings { node_id: String },
    OpenCreateCloudComputerModal { bind_bot_id: Option<u64> },
    BindBotToComputer { bot_id: u64, node_id: Option<String> },
    SaveBotComputerConfig {
        bot_id: u64,
        persona_prompt: String,
        preferred_model: String,
        approval_mode: gpui_telegram_proto::ComputerRunPolicy,
        allow_cross_device_control: bool,
        allow_cloud_self_config: bool,
    },
}

pub struct ProfileTabModel {
    pub name: String,
    pub username: String,
    pub phone: String,
    pub bio: String,
    pub status: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub avatar_path: Option<String>,
    pub facts: Vec<(String, String)>,
    pub is_notifications_enabled: bool,
    pub is_bot: bool,
    pub bot_id: u64,
    pub bot_persona_prompt: String,
    pub bot_commands: Vec<(String, String)>,
    pub miniapp_title: Option<String>,
    pub miniapp_url: Option<String>,
    pub in_attach_menu: bool,
}

pub struct MembersTabModel {
    pub members: Vec<MemberItem>,
    pub search_query: String,
    pub invite_link: String,
    pub is_group_or_channel: bool,
}

pub struct SharedMediaTabModel {
    pub active_subtab: SharedMediaSubTab,
    pub media_count: u32,
    pub media_items: Vec<SharedMediaItem>,
    pub files: Vec<SharedFileItem>,
    pub links: Vec<SharedLinkItem>,
    pub voice_notes: Vec<SharedVoiceItem>,
    pub music_items: Vec<SharedFileItem>,
}

pub struct SettingsTabModel {
    pub e2ee_fingerprint: String,
    pub auto_delete_period: String,
    pub notification_sound: String,
}

pub struct InspectorTabsView;

impl InspectorTabsView {
    pub fn render_profile(
        model: &ProfileTabModel,
        is_secret_chat_active: bool,
        is_contact_blocked: bool,
        copied_feedback: Option<&String>,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<ContextInspector>,
    ) -> AnyElement {
        let peer_name = model.name.clone();
        let peer_for_vid = model.name.clone();
        let avatar_color = if model.avatar_color != 0 { model.avatar_color } else { pal.accent };

        let fallback_initials = {
            let trimmed_init = model.avatar_initials.trim();
            if !trimmed_init.is_empty() && trimmed_init != "?" {
                trimmed_init.to_string()
            } else {
                let derived = crate::views::app_shell::shell_truth::derive_initials(&model.name);
                if derived.is_empty() {
                    "?".to_string()
                } else {
                    derived
                }
            }
        };

        let avatar = if let Some(ref path) = model.avatar_path {
            div().w(px(80.0)).h(px(80.0)).rounded_full().overflow_hidden()
                .child(img(path.clone()).w(px(80.0)).h(px(80.0)).object_fit(ObjectFit::Cover))
                .into_any_element()
        } else {
            div().w(px(80.0)).h(px(80.0)).rounded_full().bg(rgb(avatar_color)).flex().items_center().justify_center()
                .text_size(px(26.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg))
                .child(fallback_initials)
                .into_any_element()
        };

        let mut rows: Vec<(String, String)> = Vec::new();
        if !model.phone.is_empty() {
            rows.push((tr_str(lang, "手机号", "Mobile").into(), model.phone.clone()));
        }
        if !model.username.is_empty() {
            rows.push((tr_str(lang, "用户名", "Username").into(), model.username.clone()));
        }
        if !model.bio.is_empty() {
            rows.push((tr_str(lang, "简介", "Bio").into(), model.bio.clone()));
        }
        rows.extend(model.facts.iter().cloned());

        div().flex().flex_col().gap(px(12.0))
            .child(div().flex().flex_col().items_center().gap(px(6.0)).py(px(8.0)).child(avatar)
                .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary))
                    .child(if model.name.is_empty() { tr_str(lang, "未选择对话", "No chat selected").to_string() } else { model.name.clone() }))
                .child(div().text_size(px(12.0)).text_color(rgb(pal.text_secondary)).child(model.status.clone())))
            .child(div().flex().items_center().justify_around().gap(px(6.0))
                .child(Self::action_pill(IconName::PhoneCall, tr_str(lang, "语音", "Call"), pal, move |this| {
                    this.pending_action = Some(InspectorAction::StartVoiceCall(peer_name.clone()));
                }, cx))
                .child(Self::action_pill(IconName::VideoCall, tr_str(lang, "视频", "Video"), pal, move |this| {
                    this.pending_action = Some(InspectorAction::StartVideoCall(peer_for_vid.clone()));
                }, cx))
                .child(Self::action_pill(IconName::Mute, tr_str(lang, "静音", "Mute"), pal, move |this| {
                    this.profile.is_notifications_enabled = !this.profile.is_notifications_enabled;
                    this.pending_action = Some(InspectorAction::ToggleNotifications(this.profile.is_notifications_enabled));
                }, cx))
                .child(Self::action_pill(IconName::Edit, tr_str(lang, "编辑", "More"), pal, |this| {
                    this.pending_action = Some(InspectorAction::OpenEditPeer);
                }, cx)))
            .children(rows.into_iter().map(|(label, val)| {
                let cp = val.clone();
                let copied_prefix = tr_str(lang, "已复制", "Copied").to_string();
                div().cursor_pointer().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                    .flex().items_center().justify_between()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                        this.copied_feedback = Some(format!("{}: {}", copied_prefix, cp));
                        this.pending_action = Some(InspectorAction::CopyText(cp.clone()));
                        cx.notify();
                    }))
                    .child(div().flex().flex_col().gap(px(2.0))
                        .child(div().text_size(px(12.5)).text_color(rgb(pal.text_primary)).child(val))
                        .child(div().text_size(px(11.0)).text_color(rgb(pal.text_secondary)).child(label)))
                    .child(tg_icon(IconName::Copy, 14.0, pal.icon_fg))
            }))
            .when_some(copied_feedback.cloned(), |el, hint| {
                el.child(div().text_size(px(11.0)).text_color(rgb(pal.success)).child(hint))
            })
            .when(model.is_bot, |el| {
                let mini_title = model
                    .miniapp_title
                    .clone()
                    .unwrap_or_else(|| tr_str(lang, "打开小程序", "Open Mini App").to_string());
                let mini_url = model.miniapp_url.clone();
                let bot_id = model.bot_id;
                let in_attach = model.in_attach_menu;
                let persona = model.bot_persona_prompt.clone();
                let cmds = model.bot_commands.clone();
                el.child(
                    div()
                        .w_full()
                        .p(px(10.0))
                        .rounded(px(10.0))
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
                                                .child(tr_str(lang, "机器人与小程序", "Bot & Mini App")),
                                        ),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded(px(999.0))
                                        .bg(rgb(pal.composer_bg))
                                        .text_size(px(10.5))
                                        .text_color(rgb(pal.accent))
                                        .child("AI Companion"),
                                ),
                        )
                        .when_some(mini_url, |col, url| {
                            col.child(
                                div()
                                    .id("inspector-launch-miniapp-btn")
                                    .cursor_pointer()
                                    .w_full()
                                    .h(px(34.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .hover(|s| s.bg(rgb(pal.accent_hover)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(6.0))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.pending_action = Some(InspectorAction::LaunchMiniApp(url.clone()));
                                        cx.notify();
                                    }))
                                    .child("🚀")
                                    .child(mini_title),
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
                                    .justify_center()
                                    .text_size(px(11.5))
                                    .text_color(rgb(pal.text_link))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.pending_action = Some(InspectorAction::ToggleAttachMenuBot(bot_id));
                                        cx.notify();
                                    }))
                                    .child(if in_attach {
                                        tr_str(lang, "从附件菜单移除", "Remove from Attachment Menu")
                                    } else {
                                        tr_str(lang, "+ 添加到附件菜单", "+ Add to Attachment Menu")
                                    }),
                            )
                        })
                        .when(!persona.trim().is_empty(), |col| {
                            col.child(
                                div()
                                    .p(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_size(px(10.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.text_secondary))
                                            .child(tr_str(lang, "AI 人设 / System Persona", "AI Persona")),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.5))
                                            .text_color(rgb(pal.text_primary))
                                            .child(persona),
                                    ),
                            )
                        })
                        .when(!cmds.is_empty(), |col| {
                            col.child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(5.0))
                                    .children(cmds.into_iter().map(|(cmd, desc)| {
                                        let run_cmd = if cmd.starts_with('/') {
                                            cmd.clone()
                                        } else {
                                            format!("/{}", cmd)
                                        };
                                        let label = if desc.is_empty() {
                                            run_cmd.clone()
                                        } else {
                                            format!("{} · {}", run_cmd, desc)
                                        };
                                        div()
                                            .cursor_pointer()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.input_bg))
                                            .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_link))
                                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                this.pending_action = Some(InspectorAction::RunBotCommand(run_cmd.clone()));
                                                cx.notify();
                                            }))
                                            .child(label)
                                    })),
                            )
                        }),
                )
            })
            .child(div().flex().gap(px(6.0))
                .child(Self::action_pill(
                    IconName::Lock,
                    if is_secret_chat_active {
                        tr_str(lang, "已加密", "E2EE")
                    } else {
                        tr_str(lang, "加密聊天", "Secret")
                    },
                    pal,
                    |this| {
                        this.is_secret_chat_active = !this.is_secret_chat_active;
                    },
                    cx,
                ))
                .child(Self::action_pill(
                    IconName::Close,
                    if is_contact_blocked {
                        tr_str(lang, "取消屏蔽", "Unblock")
                    } else {
                        tr_str(lang, "屏蔽", "Block")
                    },
                    pal,
                    |this| {
                        this.is_contact_blocked = !this.is_contact_blocked;
                    },
                    cx,
                )))
            .into_any_element()
    }

    fn action_pill(icon: IconName, label: &str, pal: TdesktopThemePalette, on_tap: impl Fn(&mut ContextInspector) + 'static, cx: &mut Context<ContextInspector>) -> impl IntoElement {
        div().cursor_pointer().flex_1().py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
            .flex().flex_col().items_center().gap(px(4.0))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { on_tap(this); cx.notify(); }))
            .child(tg_icon(icon, 16.0, pal.accent))
            .child(div().text_size(px(11.0)).text_color(rgb(pal.text_link)).child(label.to_string()))
    }
}
