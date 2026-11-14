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
        let targets = chat_dialogs_to_forward_targets(&self.sidebar.read(cx).dialogs);
        let rows = contacts_to_modal_rows(&self.contacts_list);
        self.modal_container.update(cx, |m, _| {
            m.dialog.sync_contacts(rows);
            m.show_dialog(title, kind, targets, w, h, LayerStackOption::CloseOther);
        });
        cx.notify();
    }

    pub fn handle_main_menu_command(&mut self, cmd: MainMenuCommand, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        self.main_menu_drawer.update(cx, |d, _| d.close());
        match cmd {
            MainMenuCommand::SwitchAccount(acc_id) => {
                if Some(acc_id as u64) != self.user_id {
                    self.window_runtime.enqueue_toast(tr_str(lang, "正在切换账号…", "Switching account…"), 2000);
                }
            }
            MainMenuCommand::AddAccount => self.logout(cx),
            MainMenuCommand::SetEmojiStatus(emoji) => {
                let client = self.client.clone();
                let req = ProtoUpdateProfileRequest {
                    first_name: None, last_name: None, username: None, bio: None,
                    avatar_media_id: None, emoji_status: Some(emoji),
                };
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| { let _ = client.update_profile(&req).await; }).detach();
            }
            MainMenuCommand::MyProfile => {
                self.active_panel = ActiveWorkspacePanel::Settings;
                self.settings_section = SettingsSectionType::Information;
            }
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
                    let uid = self.user_id?;
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
                self.active_panel = ActiveWorkspacePanel::Settings;
                self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::Settings));
                self.refresh_wallet(cx);
            }
            MainMenuCommand::ToggleNightMode => {
                let next = toggle_desktop_theme_mode(self.theme_mode);
                self.apply_theme_selection(next, None, None, cx);
            }
            MainMenuCommand::WalletAndStars => {
                self.active_panel = ActiveWorkspacePanel::Settings;
                self.settings_section = SettingsSectionType::Credits;
                self.refresh_wallet(cx);
            }
            MainMenuCommand::ExportData => {
                let title = self.stage.read(cx).chat_title.clone();
                self.open_modal_dialog(tr_str(lang, "导出聊天数据", "Export Chat Data"), ModalDialogKind::ExportData { chat_title: title }, 420.0, 340.0, cx);
            }
            MainMenuCommand::ShortcutsLegend => self.open_modal_dialog(tr_str(lang, "键盘快捷键", "Keyboard Shortcuts"), ModalDialogKind::ShortcutsLegend, 460.0, 440.0, cx),
            MainMenuCommand::AboutTelegram => self.open_modal_dialog("Telegram Desktop", ModalDialogKind::AboutTelegram, 420.0, 320.0, cx),
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
            ModalActionOutcome::SelectBackground { preset_id } => self.apply_theme_selection(self.theme_mode, None, Some(preset_id), cx),
            ModalActionOutcome::SelectLanguage { language_code } => {
                let is_zh = language_code.starts_with("zh");
                self.update_preferences(cx, |p| p.localization.language_code = language_code);
                self.is_chinese_locale = is_zh;
                self.sync_theme_and_locale_to_views(cx);
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
                            let _ = weak.update(app, |s, _| s.window_runtime.enqueue_toast(format!("Exported: {path}"), 3000));
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
            _ => {}
        }
        cx.notify();
    }
}
