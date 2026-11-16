use gpui::{AsyncApp, ClipboardItem, Context, WeakEntity};
use gpui_telegram_client::net::http_client::AddContactHttpRequest;
use gpui_telegram_client::net::RichSendMessageRequest;
use gpui_telegram_proto::contracts::auth::UpdateProfileRequest as ProtoUpdateProfileRequest;
use gpui_telegram_proto::contracts::chats::CreateChatRequest;
use gpui_telegram_proto::contracts::features::StarGiftRequest;
use gpui_telegram_proto::contracts::messages::CreatePollHttpRequest;

use crate::components::ui::layers::standalone_layer_stack::LayerStackOption;
use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::{
    chat_dialogs_to_forward_targets, contacts_to_modal_rows, resolve_active_target_chat_id,
};
use crate::views::app_shell::shell_theme::toggle_desktop_theme_mode;
use crate::views::boxes::modal_types::{ModalActionOutcome, ModalDialogKind};
use crate::views::main_menu_drawer::MainMenuCommand;
use crate::views::nav_rail::NavDestination;
use crate::views::settings::settings_type::SettingsSectionType;
use crate::views::shell::{ActiveWorkspacePanel, AppShell};
use crate::views::global_tasks_workspace::TasksWorkspaceTab;

impl AppShell {
    pub fn open_modal_dialog(&mut self, title: impl Into<String>, kind: ModalDialogKind, w: f32, h: f32, cx: &mut Context<Self>) {
        self.sidebar.update(cx, |sb, _| sb.is_plus_menu_open = false);
        self.stage.update(cx, |st, _| {
            st.more_menu_open = false;
            st.ttl_submenu_open = false;
        });
        let is_connection_modal = matches!(kind, ModalDialogKind::ConnectionProxy);
        let targets = chat_dialogs_to_forward_targets(&self.sidebar.read(cx).dialogs);
        let rows = contacts_to_modal_rows(&self.session_sync.contacts_list);
        self.modal_container.update(cx, |m, mcx| {
            m.dialog.sync_contacts(rows);
            m.show_dialog(title, kind, targets, w, h, LayerStackOption::CloseOther);
            mcx.notify();
        });
        if is_connection_modal {
            let client = self.client.clone();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                if let Ok(proxies) = client.list_proxies().await {
                    let _ = weak.update(app, |s, cx| {
                        s.modal_container.update(cx, |m, mcx| {
                            m.dialog.proxies = proxies;
                            mcx.notify();
                        });
                        cx.notify();
                    });
                }
            })
            .detach();
        }
        cx.notify();
    }

    pub fn handle_main_menu_command(&mut self, cmd: MainMenuCommand, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        self.main_menu_drawer.update(cx, |d, _| d.close());
        self.sidebar.update(cx, |sb, _| sb.is_plus_menu_open = false);
        self.stage.update(cx, |st, _| {
            st.more_menu_open = false;
            st.ttl_submenu_open = false;
        });
        match cmd {
            MainMenuCommand::SwitchAccount(acc_id) => {
                if Some(acc_id as u64) != self.session_sync.user_id {
                    self.window_runtime.enqueue_toast(tr_str(lang, "正在切换账号…", "Switching account…"), 2000);
                }
            }
            MainMenuCommand::AddAccount => self.logout(cx),
            MainMenuCommand::SetEmojiStatus(emoji) => {
                let client = self.client.clone();
                let req = ProtoUpdateProfileRequest {
                    first_name: None,
                    last_name: None,
                    username: None,
                    phone: None,
                    bio: None,
                    avatar_media_id: None,
                    emoji_status: Some(emoji),
                };
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| { let _ = client.update_profile(&req).await; }).detach();
            }
            MainMenuCommand::MyProfile => {
                self.navigate_settings_to(SettingsSectionType::Information, cx);
            }
            MainMenuCommand::NewFabushiBot => self.open_fabushi_creator_modal(cx),
            MainMenuCommand::NewGroup => self.open_modal_dialog(tr_str(lang, "新建群组", "New Group"), ModalDialogKind::NewGroup, 420.0, 380.0, cx),
            MainMenuCommand::NewChannel => self.open_modal_dialog(tr_str(lang, "新建频道", "New Channel"), ModalDialogKind::NewChannel, 420.0, 380.0, cx),
            MainMenuCommand::Contacts => {
                self.refresh_contacts(cx);
                self.open_modal_dialog(tr_str(lang, "联系人", "Contacts"), ModalDialogKind::Contacts, 420.0, 500.0, cx);
            }
            MainMenuCommand::Calls => {
                self.active_panel = ActiveWorkspacePanel::Calls;
                self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::Calls));
                self.sync_calls_history(cx);
            }
            MainMenuCommand::SavedMessages => {
                self.active_panel = ActiveWorkspacePanel::Chats;
                self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::SavedMessages));
                let target_id = self.sidebar.read(cx).saved_messages_id.or_else(|| {
                    let uid = self.session_sync.user_id?;
                    self.sidebar.read(cx).dialogs.iter().find(|d| d.id == uid).map(|d| d.id)
                });
                if let Some(id) = target_id {
                    self.sidebar.update(cx, |sb, _| sb.select_dialog(id));
                    self.sync_selected_chat_inspector_and_stage(id, cx);
                }
            }
            MainMenuCommand::TaskCenter => {
                self.active_panel = ActiveWorkspacePanel::TaskCenter;
                self.tasks_workspace.update(cx, |w, _| w.set_tab(TasksWorkspaceTab::Tasks));
                self.refresh_tasks_and_mcp(cx);
            }
            MainMenuCommand::McpTools => {
                self.active_panel = ActiveWorkspacePanel::McpTools;
                self.tasks_workspace.update(cx, |w, _| w.set_tab(TasksWorkspaceTab::McpTools));
                self.refresh_tasks_and_mcp(cx);
            }
            MainMenuCommand::RemoteComputer => self.open_computer_workbench_live(cx),
            MainMenuCommand::Settings => {
                self.navigate_settings_to(SettingsSectionType::Main, cx);
            }
            MainMenuCommand::ToggleNightMode => {
                let next = toggle_desktop_theme_mode(self.theme_state.theme_mode);
                self.apply_theme_selection(next, None, None, cx);
            }
            MainMenuCommand::WalletAndStars => {
                self.navigate_settings_to(SettingsSectionType::Credits, cx);
            }
            MainMenuCommand::ExportData => {
                let title = self.stage.read(cx).chat_title.clone();
                self.open_modal_dialog(tr_str(lang, "导出聊天数据", "Export Chat Data"), ModalDialogKind::ExportData { chat_title: title }, 420.0, 340.0, cx);
            }
            MainMenuCommand::ShortcutsLegend => self.open_modal_dialog(tr_str(lang, "键盘快捷键", "Keyboard Shortcuts"), ModalDialogKind::ShortcutsLegend, 460.0, 440.0, cx),
            MainMenuCommand::AboutTelegram => self.open_modal_dialog(
                tr_str(lang, "关于应用", "About App"),
                ModalDialogKind::AboutTelegram,
                420.0,
                320.0,
                cx,
            ),
            MainMenuCommand::Logout => self.logout(cx),
        }
        cx.notify();
    }

    pub fn handle_modal_outcome(&mut self, outcome: ModalActionOutcome, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let active_chat = resolve_active_target_chat_id(self.stage.read(cx).active_chat_id, self.sidebar.read(cx).active_chat_id);
        match outcome {
            ModalActionOutcome::CreatePollSubmitted { question, poll_type, options } => {
                if let Some(chat_id) = active_chat {
                    let is_anon = matches!(poll_type, crate::views::stage::cards::PollType::Anonymous);
                    let is_quiz = matches!(poll_type, crate::views::stage::cards::PollType::Quiz);
                    let req = CreatePollHttpRequest {
                        question,
                        options: options.into_iter().map(|o| o.text).collect(),
                        is_anonymous: is_anon,
                        multiple_choice: false,
                        is_quiz,
                        correct_option_id: if is_quiz { Some(0) } else { None },
                        solution: None,
                        topic_id: None,
                        client_nonce: Some(client.next_client_nonce()),
                    };
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let _ = client.create_poll(chat_id as i64, &req).await;
                        let _ = weak.update(app, |s, cx| s.load_historical_messages(chat_id, cx));
                    }).detach();
                }
            }
            ModalActionOutcome::ForwardToChat { target_chat_id, message_ids, extra_caption, .. } => {
                let from_chat = active_chat.unwrap_or(target_chat_id) as i64;
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    for mid in message_ids {
                        let text_opt = if extra_caption.trim().is_empty() { None } else { Some(extra_caption.clone()) };
                        let req = RichSendMessageRequest {
                            chat_id: target_chat_id as i64, text: text_opt, content: None, entities: Vec::new(),
                            reply_to: None, quote_text: None, topic_id: None, client_nonce: Some(client.next_client_nonce()),
                            forward_from_chat_id: Some(from_chat), forward_from_msg_id: Some(mid as i64),
                            silent: false, scheduled_at_ms: None, inline_keyboard: None,
                        };
                        let _ = client.send_message_durable(req).await;
                    }
                    let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                }).detach();
            }
            ModalActionOutcome::CreateGroupSubmitted { title, enable_forum_topics, .. } => {
                let req = CreateChatRequest {
                    title, chat_type: Some(serde_json::json!(1)),
                    is_forum: enable_forum_topics, ..Default::default()
                };
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(created) = client.create_chat(&req).await {
                        let _ = weak.update(app, |s, cx| { s.refresh_chats(cx); s.sync_selected_chat_inspector_and_stage(created.chat_id as u64, cx); });
                    }
                }).detach();
            }
            ModalActionOutcome::CreateChannelSubmitted { title, username_slug, .. } => {
                let req = CreateChatRequest {
                    title, chat_type: Some(serde_json::json!(2)),
                    username: Some(username_slug).filter(|s| !s.is_empty()), ..Default::default()
                };
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(created) = client.create_chat(&req).await {
                        let _ = weak.update(app, |s, cx| { s.refresh_chats(cx); s.sync_selected_chat_inspector_and_stage(created.chat_id as u64, cx); });
                    }
                }).detach();
            }
            ModalActionOutcome::AddContactSubmitted { first_name, last_name, phone } => {
                let req = AddContactHttpRequest {
                    first_name, last_name: Some(last_name).filter(|s| !s.is_empty()),
                    username: None, phone: Some(phone).filter(|s| !s.is_empty()), is_bot: false, bot_system_prompt: None,
                };
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.add_contact(req).await;
                    let _ = weak.update(app, |s, cx| { s.refresh_contacts(cx); s.refresh_chats(cx); });
                }).detach();
            }
            ModalActionOutcome::OpenContactChat { name, .. } => {
                if let Some(id) = self.sidebar.read(cx).dialogs.iter().find(|d| d.peer_name == name).map(|d| d.id) {
                    self.sidebar.update(cx, |sb, _| sb.select_dialog_from_user(id));
                    self.sync_selected_chat_inspector_and_stage(id, cx);
                }
            }
            ModalActionOutcome::StartCallWithPeer { peer_name, is_video } => {
                let pid = self.current_peer_user_id(cx).unwrap_or(0);
                if pid > 0 { self.start_webrtc_call(pid, peer_name, is_video, cx); }
            }
            ModalActionOutcome::CopyToClipboard(text) => cx.write_to_clipboard(ClipboardItem::new_string(text)),
            ModalActionOutcome::SelectBackground { preset_id } => self.apply_theme_selection(self.theme_state.theme_mode, None, Some(preset_id), cx),
            ModalActionOutcome::SelectLanguage { language_code } => {
                self.update_preferences(cx, |p| p.localization.language_code = language_code);
            }
            ModalActionOutcome::SavePeerEdit { title, bio, .. } => {
                if let Some(cid) = active_chat {
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let _ = client.update_chat_info(cid as i64, Some(&title), Some(&bio)).await;
                        let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                    }).detach();
                }
            }
            ModalActionOutcome::ConfirmDeleteMessages { message_ids, revoke_for_everyone } => {
                if let Some(cid) = active_chat {
                    self.stage.update(cx, |st, _| st.delete_messages_by_ids(&message_ids));
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        for mid in message_ids { let _ = client.delete_message(cid as i64, mid as i64, revoke_for_everyone).await; }
                    }).detach();
                }
            }
            ModalActionOutcome::ConfirmPinMessage { message_id, .. } => {
                if let Some(cid) = active_chat {
                    self.stage.update(cx, |st, _| st.toggle_pin_message(message_id));
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| { let _ = client.pin_message(cid as i64, message_id as i64).await; }).detach();
                }
            }
            ModalActionOutcome::RunChatExport { format_json } => {
                if let Some(cid) = active_chat {
                    let fmt = if format_json { Some("json") } else { Some("html") }.map(str::to_string);
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        if let Ok(path) = client.export_chat_history(cid as i64, fmt.as_deref()).await {
                            let _ = weak.update(app, |s, cx| {
                                let pfx = tr_str(s.active_lang_id(), "已导出聊天记录", "Exported");
                                s.window_runtime.enqueue_toast(format!("{pfx}: {path}"), 3000);
                                cx.notify();
                            });
                        }
                    }).detach();
                }
            }
            ModalActionOutcome::SendStarGift { title, stars, .. } => {
                if let Some(pid) = self.current_peer_user_id(cx) {
                    let req = StarGiftRequest { to_user_id: pid, chat_id: active_chat.map(|v| v as i64), amount_stars: stars as i64, gift_id: None, message: Some(title) };
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let _ = client.wallet_gift_stars(&req).await;
                        let _ = weak.update(app, |s, cx| s.refresh_wallet(cx));
                    }).detach();
                }
            }
            ModalActionOutcome::ConfirmClearChatHistory { .. } => {
                if let Some(cid) = active_chat {
                    let lang = self.active_lang_id();
                    self.sidebar.update(cx, |sb, sb_cx| {
                        sb.clear_history(cid);
                        sb_cx.notify();
                    });
                    self.stage.update(cx, |st, st_cx| {
                        st.messages.clear();
                        st.chat_histories.insert(cid, Vec::new());
                        st.pinned_message = None;
                        st.row_height_cache.rebuild_prefix_sums(&st.messages);
                        st_cx.notify();
                    });
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "聊天记录已清空", "Chat history cleared"),
                        2200,
                    );
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let _ = client.clear_chat_history(cid as i64).await;
                        let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                    })
                    .detach();
                }
            }
            ModalActionOutcome::ConfirmLeaveOrDeleteChat => {
                if let Some(cid) = active_chat {
                    let lang = self.active_lang_id();
                    self.session_sync.rich_chats_by_id.remove(&cid);
                    let next_chat_id = self.sidebar.update(cx, |sb, sb_cx| {
                        sb.delete_dialog(cid);
                        sb_cx.notify();
                        sb.dialogs.first().map(|d| d.id)
                    });
                    if let Some(next_id) = next_chat_id {
                        self.sidebar.update(cx, |sb, _| sb.select_dialog(next_id));
                        self.sync_selected_chat_inspector_and_stage(next_id, cx);
                    } else {
                        self.stage.update(cx, |st, st_cx| {
                            st.clear_selection();
                            st_cx.notify();
                        });
                        self.set_inspector_open(false, cx);
                    }
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "对话已移除", "Chat removed"),
                        2200,
                    );
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let _ = client.leave_or_delete_chat(cid as i64).await;
                        let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                    })
                    .detach();
                }
            }
            ModalActionOutcome::SetLocalPasscode(pin_opt) => {
                let enabled = pin_opt.as_ref().is_some_and(|s| !s.trim().is_empty());
                if !enabled {
                    self.overlays.is_passcode_locked = false;
                    self.overlays.passcode_unlock_input.clear();
                    self.overlays.passcode_unlock_error = None;
                }
                let mut next_hash: Option<String> = None;
                let mut next_auto_lock: u64 = 3600;
                self.settings_router.update(cx, |sr, scx| {
                    let lp = sr.ensure_local_passcode(scx);
                    lp.update(scx, |lp, lp_cx| {
                        if let Some(pin) = pin_opt {
                            let _ = lp.set_passcode(&pin);
                        } else {
                            lp.has_passcode = false;
                            lp.passcode_hash = None;
                            lp.is_locked = false;
                        }
                        lp.pending_prefs_sync = false;
                        next_hash = lp.passcode_hash.clone();
                        next_auto_lock = lp.auto_lock.as_seconds();
                        lp_cx.notify();
                    });
                });
                self.update_preferences(cx, |p| {
                    p.local_passcode_hash = next_hash;
                    p.auto_lock_seconds = next_auto_lock;
                });
                self.settings_router.update(cx, |sr, scx| {
                    let sp = sr.ensure_privacy(scx);
                    sp.update(scx, |sp, sp_cx| {
                        sp.set_passcode_enabled(enabled);
                        sp_cx.notify();
                    });
                });
                self.push_user_settings_to_backend(cx);
                let lang = self.active_lang_id();
                self.window_runtime.enqueue_toast(
                    if enabled {
                        tr_str(lang, "本地锁定密码已启用", "Local passcode enabled")
                    } else {
                        tr_str(lang, "本地锁定密码已关闭", "Local passcode disabled")
                    },
                    2200,
                );
            }
            ModalActionOutcome::AddProxy(proxy_rec) => {
                let (kind, host, port, enabled) = (
                    proxy_rec.kind.clone(),
                    proxy_rec.host.clone(),
                    proxy_rec.port,
                    proxy_rec.enabled,
                );
                self.settings_router.update(cx, |sr, scx| {
                    let sa = sr.ensure_advanced(scx);
                    sa.update(scx, |sa, sa_cx| {
                        sa.sync_proxy(&kind, &host, port, enabled);
                        sa_cx.notify();
                    });
                });
                let c = client.clone();
                let lang_str = self.active_lang_id().to_string();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = c.upsert_proxy(proxy_rec).await;
                    let _ = weak.update(app, |s, cx| {
                        s.window_runtime.enqueue_toast(
                            tr_str(&lang_str, "代理节点已保存", "Proxy saved"),
                            2200,
                        );
                        cx.notify();
                    });
                })
                .detach();
            }
            ModalActionOutcome::SelectProxy { proxy_id } => {
                let selected_rec = if proxy_id == 0 {
                    None
                } else {
                    self.modal_container
                        .read(cx)
                        .dialog
                        .proxies
                        .iter()
                        .find(|p| p.id == proxy_id)
                        .cloned()
                };
                self.settings_router.update(cx, |sr, scx| {
                    let sa = sr.ensure_advanced(scx);
                    sa.update(scx, |sa, sa_cx| {
                        if let Some(ref p) = selected_rec {
                            sa.sync_proxy(&p.kind, &p.host, p.port, true);
                        } else {
                            sa.sync_proxy("Direct", "", 1080, false);
                        }
                        sa_cx.notify();
                    });
                });
                let c = client.clone();
                let lang_str = self.active_lang_id().to_string();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if proxy_id == 0 {
                        if let Ok(existing) = c.list_proxies().await {
                            for mut item in existing {
                                if item.enabled {
                                    item.enabled = false;
                                    let _ = c.upsert_proxy(item).await;
                                }
                            }
                        }
                    } else if let Some(mut rec) = selected_rec {
                        rec.enabled = true;
                        let _ = c.upsert_proxy(rec).await;
                    } else if let Ok(existing) = c.list_proxies().await {
                        if let Some(mut rec) = existing.into_iter().find(|p| p.id == proxy_id) {
                            rec.enabled = true;
                            let _ = c.upsert_proxy(rec.clone()).await;
                            let _ = weak.update(app, |s, cx| {
                                s.settings_router.update(cx, |sr, scx| {
                                    let sa = sr.ensure_advanced(scx);
                                    sa.update(scx, |sa, sa_cx| {
                                        sa.sync_proxy(&rec.kind, &rec.host, rec.port, true);
                                        sa_cx.notify();
                                    });
                                });
                            });
                        }
                    }
                    let _ = weak.update(app, |s, cx| {
                        let msg = if proxy_id == 0 {
                            tr_str(&lang_str, "已切换为直连模式", "Switched to Direct connection")
                        } else {
                            tr_str(&lang_str, "已启用所选代理服务器", "Proxy server enabled")
                        };
                        s.window_runtime.enqueue_toast(msg, 2200);
                        cx.notify();
                    });
                })
                .detach();
            }
            ModalActionOutcome::SavePrivacyRule { category, rule } => {
                self.apply_privacy_rule_update(&category, rule, cx);
            }
            _ => {}
        }
        cx.notify();
    }
}
