//! HTTP dialog/history fetch workflows, contact/group creation, categorized search,
//! and attachment dispatch helpers for `AppShell`.

use std::collections::HashSet;

use gpui::{AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::http_client::AddContactHttpRequest;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::map_stage_media_to_gallery;
use crate::views::app_shell::shell_truth::{
    apply_fenced_chat_list_response, apply_fenced_history_response_preserving, derive_initials,
    overlay_rich_message_metadata, SessionRequestFence,
};
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::chat_sidebar::{
    CategorizedSearchResult, DialogItemModel, PendingAddContactRequest, PendingCreateGroupRequest,
};
use crate::views::shell::AppShell;

impl AppShell {
    /// Dispatches `POST /api/v1/contacts/add` for adding a friend or creating an AI bot dialog.
    pub fn dispatch_add_contact(&mut self, req: PendingAddContactRequest, cx: &mut Context<Self>) {
        let token = match self.session_sync.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let auth_gen = self.session_sync.auth_generation;
        let client = self.client.clone();
        let http_req = AddContactHttpRequest {
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            username: req.username.clone(),
            phone: req.phone.clone(),
            is_bot: req.is_bot,
            bot_system_prompt: req.system_prompt.clone(),
        };
        let local_dialog_id = req.local_dialog_id;

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.add_contact(&token, http_req).await;
            let _ = weak_shell.update(app, |shell, cx| {
                if !shell.is_authenticated
                    || shell.session_sync.auth_generation != auth_gen
                    || shell.session_sync.session_token.as_deref() != Some(token.as_str())
                {
                    return;
                }
                if let Ok(resp) = res {
                    let server_chat_id = resp.chat_id;
                    let mut selected_peer: Option<DialogItemModel> = None;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == local_dialog_id) {
                            d.id = server_chat_id;
                            d.peer_name = resp.title.clone();
                            d.avatar_color = resp.avatar_color.unwrap_or(0x2AABEE);
                            selected_peer = Some(d.clone());
                        }
                        if sb.active_chat_id == Some(local_dialog_id) {
                            sb.active_chat_id = Some(server_chat_id);
                        }
                        sb_cx.notify();
                    });
                    if let Some(d) = selected_peer {
                        shell.stage.update(cx, |st, st_cx| {
                            st.set_chat(
                                server_chat_id,
                                &d.peer_name,
                                "online",
                                &d.avatar_initials,
                                d.avatar_color,
                            );
                            st_cx.notify();
                        });
                    }
                    shell.load_historical_messages(server_chat_id, cx);
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Dispatches `POST /api/v1/chats/create` to materialize a newly created group chat.
    pub fn dispatch_create_group(
        &mut self,
        req: PendingCreateGroupRequest,
        cx: &mut Context<Self>,
    ) {
        let token = match self.session_sync.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let auth_gen = self.session_sync.auth_generation;
        let client = self.client.clone();
        let title = req.title.clone();
        let member_ids = req.member_ids.clone();
        let member_usernames = req.member_usernames.clone();
        let about = req.about.clone();
        let local_dialog_id = req.local_dialog_id;

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .http_client
                .create_group_with_token(&token, &title, member_ids, member_usernames, about)
                .await;
            let _ = weak_shell.update(app, |shell, cx| {
                if !shell.is_authenticated
                    || shell.session_sync.auth_generation != auth_gen
                    || shell.session_sync.session_token.as_deref() != Some(token.as_str())
                {
                    return;
                }
                if let Ok(resp) = res {
                    let server_chat_id = resp.chat_id.max(0) as u64;
                    let mut selected_peer: Option<DialogItemModel> = None;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == local_dialog_id) {
                            d.id = server_chat_id;
                            selected_peer = Some(d.clone());
                        }
                        if sb.active_chat_id == Some(local_dialog_id) {
                            sb.active_chat_id = Some(server_chat_id);
                        }
                        sb_cx.notify();
                    });
                    if let Some(d) = selected_peer {
                        shell.stage.update(cx, |st, st_cx| {
                            st.set_chat(
                                server_chat_id,
                                &d.peer_name,
                                "group",
                                &d.avatar_initials,
                                d.avatar_color,
                            );
                            st_cx.notify();
                        });
                    }
                    shell.load_historical_messages(server_chat_id, cx);
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Dispatches `GET /api/v1/search/messages?q=...&category=...` for categorized sidebar search.
    pub fn dispatch_categorized_search(
        &mut self,
        query: String,
        category: String,
        search_seq: u64,
        cx: &mut Context<Self>,
    ) {
        self.session_sync.last_dispatched_search_seq = search_seq;
        if category.eq_ignore_ascii_case("Apps") {
            let q_opt = (!query.trim().is_empty()).then(|| query.clone());
            self.refresh_miniapps_catalog(q_opt, cx);
        }
        let token = match self.session_sync.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let auth_gen = self.session_sync.auth_generation;
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .http_client
                .search_global(&token, &query, &category)
                .await;
            let _ = weak_shell.update(app, |shell, cx| {
                if !shell.is_authenticated
                    || shell.session_sync.auth_generation != auth_gen
                    || shell.sidebar.read(cx).search_request_seq != search_seq
                {
                    return;
                }
                if let Ok(hits) = res {
                    let mapped: Vec<CategorizedSearchResult> = hits
                        .into_iter()
                        .map(|hit| {
                            let title = if hit.title.is_empty() {
                                format!("Chat #{}", hit.chat_id)
                            } else {
                                hit.title
                            };
                            let initials =
                                title.chars().take(2).collect::<String>().to_uppercase();
                            let cat = if hit.category.is_empty() {
                                category.clone()
                            } else {
                                hit.category
                            };
                            CategorizedSearchResult {
                                chat_id: hit.chat_id,
                                message_id: hit.message_id,
                                category: cat,
                                title,
                                subtitle: hit.snippet,
                                timestamp: "刚刚".to_string(),
                                avatar_initials: if initials.is_empty() {
                                    "TG".to_string()
                                } else {
                                    initials
                                },
                                avatar_color: hit.avatar_color.filter(|&c| c != 0).unwrap_or(0x2AABEE),
                                is_bot: hit.is_bot,
                            }
                        })
                        .collect();
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.remote_search_results = mapped;
                        sb_cx.notify();
                    });
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Refreshes the authoritative chat list from `GET /api/v1/chats` and enriches bot/peer metadata.
    pub fn refresh_chats(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (
            self.session_sync.session_token.clone(),
            self.session_sync.user_id,
        ) else {
            return;
        };
        if !self.is_authenticated {
            return;
        }
        self.session_sync.chat_list_request_generation =
            self.session_sync.chat_list_request_generation.wrapping_add(1);
        let fence = SessionRequestFence {
            auth_generation: self.session_sync.auth_generation,
            request_generation: self.session_sync.chat_list_request_generation,
            token: token.clone(),
            user_id: uid,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.get_chats(&token).await;
            let rich_chats = client.hermes_list_chats_rich().await.ok();
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(chats) => {
                        if let Some(ref rich_list) = rich_chats {
                            for rc in rich_list {
                                if rc.chat_id > 0 {
                                    let cid = rc.chat_id as u64;
                                    shell.session_sync.rich_chats_by_id.insert(cid, rc.clone());
                                    if let Some(pid) = rc.peer_user_id {
                                        if pid > 0 {
                                            shell.session_sync.chat_peer_user_ids.insert(cid, pid);
                                        }
                                    }
                                }
                            }
                        }
                        let applied = shell.sidebar.update(cx, |sb, sb_cx| {
                            let ok = shell.stage.update(sb_cx, |st, st_cx| {
                                let applied = shell.inspector.update(st_cx, |insp, insp_cx| {
                                    let applied = apply_fenced_chat_list_response(
                                        &fence,
                                        shell.is_authenticated,
                                        shell.session_sync.auth_generation,
                                        shell.session_sync.chat_list_request_generation,
                                        shell.session_sync.session_token.as_deref(),
                                        shell.session_sync.user_id,
                                        sb,
                                        st,
                                        insp,
                                        &chats,
                                    );
                                    if applied {
                                        insp_cx.notify();
                                    }
                                    applied
                                });
                                if applied {
                                    st_cx.notify();
                                }
                                applied
                            });
                            if ok {
                                for c in &chats {
                                    if c.is_bot.unwrap_or(false) {
                                        sb.bot_ids.insert(c.chat_id);
                                    }
                                }
                                if let Some(ref rich_list) = rich_chats {
                                    for rc in rich_list {
                                        if rc.is_bot && rc.chat_id > 0 {
                                            sb.bot_ids.insert(rc.chat_id as u64);
                                        }
                                    }
                                }
                                for d in &mut sb.dialogs {
                                    if d.avatar_initials.is_empty() || d.avatar_initials == "?" {
                                        let fixed = derive_initials(&d.peer_name);
                                        if !fixed.is_empty() {
                                            d.avatar_initials = fixed;
                                        }
                                    }
                                }
                                sb_cx.notify();
                            }
                            ok
                        });
                        if applied {
                            if let Some(selected_id) = shell.sidebar.read(cx).active_chat_id {
                                shell.sync_selected_chat_inspector_and_stage(selected_id, cx);
                            }
                            shell.refresh_miniapps_catalog(None, cx);
                        }
                    }
                    Err(err) => {
                        if fence.matches(
                            shell.is_authenticated,
                            shell.session_sync.auth_generation,
                            shell.session_sync.chat_list_request_generation,
                            shell.session_sync.session_token.as_deref(),
                            shell.session_sync.user_id,
                        ) {
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{}: {}",
                                    tr_str(
                                        shell.active_lang_id(),
                                        "刷新会话列表失败",
                                        "Failed to refresh chats"
                                    ),
                                    err
                                ),
                                3200,
                            );
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Fetches historical messages from the backend for `chat_id` under a session/generation fence.
    pub fn load_historical_messages(&mut self, chat_id: u64, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (
            self.session_sync.session_token.clone(),
            self.session_sync.user_id,
        ) else {
            return;
        };
        if !self.is_authenticated {
            return;
        }
        self.session_sync.history_request_generation =
            self.session_sync.history_request_generation.wrapping_add(1);
        let fence = SessionRequestFence {
            auth_generation: self.session_sync.auth_generation,
            request_generation: self.session_sync.history_request_generation,
            token: token.clone(),
            user_id: uid,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .get_chat_messages_with_token(&token, chat_id, Some(50))
                .await;
            let rich_msgs = client
                .hermes_messages_history_rich(chat_id as i64, Some(50))
                .await
                .ok();
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(msgs) => {
                        let protected_ids: HashSet<u64> = shell
                            .session_sync
                            .in_flight_reactions
                            .iter()
                            .filter_map(|(&(cid, mid), &cnt)| {
                                (cid == chat_id && cnt > 0).then_some(mid)
                            })
                            .collect();
                        let ok = shell.sidebar.update(cx, |sb, sb_cx| {
                            shell.stage.update(sb_cx, |st, st_cx| {
                                let res = apply_fenced_history_response_preserving(
                                    &fence,
                                    shell.is_authenticated,
                                    shell.session_sync.auth_generation,
                                    shell.session_sync.history_request_generation,
                                    shell.session_sync.session_token.as_deref(),
                                    shell.session_sync.user_id,
                                    sb,
                                    st,
                                    chat_id,
                                    msgs,
                                    &protected_ids,
                                );
                                if res {
                                    if let Some(ref rich) = rich_msgs {
                                        overlay_rich_message_metadata(&mut st.messages, rich);
                                        st.row_height_cache.mark_dirty();
                                        if let Some(hist) = st.chat_histories.get_mut(&chat_id) {
                                            overlay_rich_message_metadata(hist, rich);
                                        }
                                    }
                                    st_cx.notify();
                                }
                                res
                            })
                        });
                        if ok && shell.stage.read(cx).active_chat_id == Some(chat_id) {
                            let (_, media, files, links, voices) =
                                map_stage_media_to_gallery(&shell.stage.read(cx).messages);
                            shell.inspector.update(cx, |insp, insp_cx| {
                                insp.sync_shared_content(media, files, links, voices, Vec::new());
                                insp_cx.notify();
                            });
                        }
                    }
                    Err(err) => {
                        if fence.matches(
                            shell.is_authenticated,
                            shell.session_sync.auth_generation,
                            shell.session_sync.history_request_generation,
                            shell.session_sync.session_token.as_deref(),
                            shell.session_sync.user_id,
                        ) {
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{}: {}",
                                    tr_str(
                                        shell.active_lang_id(),
                                        "加载聊天记录失败",
                                        "Failed to load history"
                                    ),
                                    err
                                ),
                                3200,
                            );
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(crate) fn append_outgoing_attachment(&mut self, kind: &str, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        match kind {
            "poll" => {
                self.open_modal_dialog(
                    tr_str(lang, "创建投票", "Create Poll"),
                    ModalDialogKind::CreatePoll,
                    420.0,
                    480.0,
                    cx,
                );
            }
            "voice" => {
                let start = !self.rtc_talk.voice_recorder.is_recording();
                self.toggle_voice_recording(start, cx);
            }
            "photo" | "video" | "file" => {
                self.window_runtime.enqueue_toast(
                    tr_str(
                        lang,
                        "请将文件拖拽至聊天窗口上传发送",
                        "Drop a local file into the chat window to upload and send",
                    ),
                    2600,
                );
                cx.notify();
            }
            "location" => {
                self.window_runtime.enqueue_toast(
                    tr_str(
                        lang,
                        "位置共享需要选择真实位置后发送",
                        "Location sharing requires selecting a location",
                    ),
                    2600,
                );
                cx.notify();
            }
            _ => {}
        }
    }
}
