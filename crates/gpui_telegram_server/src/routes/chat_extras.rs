use crate::routes::{broadcast_chat_event, check_active_membership, err_resp, map_telegram_err, parse_json_object};
use crate::server::{ensure_user_welcome_bot, AppState, AuthBearer};
use axum::{body::Bytes, extract::{Path, Query, State}, http::StatusCode, response::{IntoResponse, Json, Response}};
use gpui_telegram_core::types::{ChatId, MsgId};
use gpui_telegram_proto::contracts::UpsertFolderRequest;
use gpui_telegram_proto::sync::UpdateEvent;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

fn chat_detail_with_flag(
    state: &AppState,
    user_id: gpui_telegram_core::types::UserId,
    chat_id: ChatId,
    flag_key: &str,
) -> serde_json::Value {
    if let Ok(Some(chat)) = state.db.get_chat(chat_id) {
        let mut val = crate::routes::chat_lifecycle::chat_detail_json(state, user_id, &chat);
        if let Some(obj) = val.as_object_mut() {
            obj.insert(flag_key.to_string(), json!(true));
        }
        val
    } else {
        json!({ "chat_id": chat_id.0, flag_key: true })
    }
}

pub async fn chats_welcome(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    let chat_id = ensure_user_welcome_bot(&state, auth.user_id, None);
    (StatusCode::CREATED, Json(chat_detail_with_flag(&state, auth.user_id, chat_id, "welcome_bot"))).into_response()
}

pub async fn invites_list(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    match state.db.list_chat_invites(ChatId(chat_id)) {
        Ok(invites) => (StatusCode::OK, Json(json!({ "invites": invites }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn invites_create(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    let (_chat, _m, is_admin) = match check_active_membership(&state, ChatId(chat_id), auth.user_id) { Ok(c) => c, Err(r) => return r };
    if !is_admin { return err_resp(StatusCode::FORBIDDEN, "forbidden", "Admin rights required to create invite links"); }
    let payload = if body.is_empty() { json!({}) } else { match parse_json_object(&body) { Ok(v) => v, Err(r) => return r } };
    let title = payload.get("title").and_then(|v| v.as_str());
    let usage_limit = payload.get("usage_limit").and_then(|v| v.as_u64()).map(|v| v as u32);
    let expire_date_ms = payload.get("expire_date_ms").and_then(|v| v.as_i64());
    let request_needed = payload.get("request_needed").and_then(|v| v.as_bool()).unwrap_or(false);
    match state.db.create_chat_invite(ChatId(chat_id), auth.user_id, title, usage_limit, expire_date_ms, request_needed) {
        Ok(inv) => (StatusCode::CREATED, Json(inv)).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn invites_revoke(auth: AuthBearer, State(state): State<Arc<AppState>>, Path((chat_id, invite_id)): Path<(i64, String)>) -> Response {
    let (_chat, _m, is_admin) = match check_active_membership(&state, ChatId(chat_id), auth.user_id) { Ok(c) => c, Err(r) => return r };
    if !is_admin { return err_resp(StatusCode::FORBIDDEN, "forbidden", "Admin rights required to revoke invite links"); }
    match state.db.revoke_chat_invite(ChatId(chat_id), &invite_id) {
        Ok(true) => (StatusCode::OK, Json(json!({ "revoked": true, "invite_id": invite_id }))).into_response(),
        Ok(false) => err_resp(StatusCode::NOT_FOUND, "not_found", "Invite link not found"),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn invites_join(auth: AuthBearer, State(state): State<Arc<AppState>>, body: Bytes) -> Response {
    let payload = match parse_json_object(&body) { Ok(v) => v, Err(r) => return r };
    let code = match payload.get("code").or_else(|| payload.get("link")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) {
        Some(c) => c,
        None => return err_resp(StatusCode::BAD_REQUEST, "invalid_invite", "code or link is required"),
    };
    match state.db.join_chat_by_invite_code(auth.user_id, code) {
        Ok(chat_id) => {
            state.gateway_hub.subscribe_chat(chat_id.0, auth.user_id);
            (StatusCode::OK, Json(chat_detail_with_flag(&state, auth.user_id, chat_id, "joined"))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn folders_list(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    match state.db.list_folders(auth.user_id) {
        Ok(folders) => (StatusCode::OK, Json(json!({ "folders": folders }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn folders_create(auth: AuthBearer, State(state): State<Arc<AppState>>, Json(req): Json<UpsertFolderRequest>) -> Response {
    if req.title.trim().is_empty() { return err_resp(StatusCode::BAD_REQUEST, "invalid_title", "Folder title cannot be empty"); }
    match state.db.upsert_folder(auth.user_id, if req.id > 0 { Some(req.id) } else { None }, &req) {
        Ok(folder) => {
            state.gateway_hub.broadcast_to_user(auth.user_id, gpui_telegram_proto::ws::WsServerFrame::Update(UpdateEvent::FolderChanged { folder: folder.clone() }));
            (StatusCode::CREATED, Json(folder)).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn folders_update(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(folder_id): Path<i64>, Json(req): Json<UpsertFolderRequest>) -> Response {
    if req.title.trim().is_empty() { return err_resp(StatusCode::BAD_REQUEST, "invalid_title", "Folder title cannot be empty"); }
    match state.db.upsert_folder(auth.user_id, Some(folder_id), &req) {
        Ok(folder) => {
            state.gateway_hub.broadcast_to_user(auth.user_id, gpui_telegram_proto::ws::WsServerFrame::Update(UpdateEvent::FolderChanged { folder: folder.clone() }));
            (StatusCode::OK, Json(folder)).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn folders_delete(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(folder_id): Path<i64>) -> Response {
    match state.db.delete_folder(auth.user_id, folder_id) {
        Ok(true) => {
            state.gateway_hub.broadcast_to_user(auth.user_id, gpui_telegram_proto::ws::WsServerFrame::Update(UpdateEvent::FolderChanged {
                folder: gpui_telegram_proto::contracts::FolderDto { id: folder_id, ..Default::default() },
            }));
            (StatusCode::OK, Json(json!({ "deleted": true, "folder_id": folder_id }))).into_response()
        }
        Ok(false) => err_resp(StatusCode::NOT_FOUND, "not_found", "Folder not found"),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn chat_archive(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let payload = if body.is_empty() { json!({ "archived": true }) } else { match parse_json_object(&body) { Ok(v) => v, Err(r) => return r } };
    let archived = payload.get("archived").and_then(|v| v.as_bool()).unwrap_or(true);
    match state.db.set_chat_setting(auth.user_id, ChatId(chat_id), Some(archived), None, None, None) {
        Ok((a, p, m, mu)) => (StatusCode::OK, Json(json!({ "chat_id": chat_id, "is_archived": a, "is_pinned": p, "is_muted": m, "mute_until_ms": mu }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn chat_pin_dialog(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let payload = if body.is_empty() { json!({ "pinned": true }) } else { match parse_json_object(&body) { Ok(v) => v, Err(r) => return r } };
    let pinned = payload.get("pinned").and_then(|v| v.as_bool()).unwrap_or(true);
    match state.db.set_chat_setting(auth.user_id, ChatId(chat_id), None, Some(pinned), None, None) {
        Ok((a, p, m, mu)) => (StatusCode::OK, Json(json!({ "chat_id": chat_id, "is_archived": a, "is_pinned": p, "is_muted": m, "mute_until_ms": mu }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn chat_mute(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let payload = if body.is_empty() { json!({ "muted": true }) } else { match parse_json_object(&body) { Ok(v) => v, Err(r) => return r } };
    let muted = payload.get("muted").and_then(|v| v.as_bool()).unwrap_or(true);
    let mute_until = payload.get("mute_until_ms").map(|v| v.as_i64());
    match state.db.set_chat_setting(auth.user_id, ChatId(chat_id), None, None, Some(muted), mute_until) {
        Ok((a, p, m, mu)) => (StatusCode::OK, Json(json!({ "chat_id": chat_id, "is_archived": a, "is_pinned": p, "is_muted": m, "mute_until_ms": mu }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn pins_list(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    match state.db.list_pinned_messages(ChatId(chat_id)) {
        Ok(ids) => (StatusCode::OK, Json(json!({ "chat_id": chat_id, "pinned_message_ids": ids }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn pins_pin(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    let (_chat, _m, is_admin) = match check_active_membership(&state, ChatId(chat_id), auth.user_id) { Ok(c) => c, Err(r) => return r };
    if !is_admin { return err_resp(StatusCode::FORBIDDEN, "forbidden", "Admin rights required to pin messages"); }
    let payload = match parse_json_object(&body) { Ok(v) => v, Err(r) => return r };
    let msg_id = match payload.get("message_id").and_then(|v| v.as_i64()).filter(|&id| id > 0) {
        Some(id) => id,
        None => return err_resp(StatusCode::BAD_REQUEST, "invalid_message_id", "Positive message_id is required"),
    };
    match state.db.pin_message(ChatId(chat_id), MsgId(msg_id), auth.user_id) {
        Ok(ids) => {
            broadcast_chat_event(&state, ChatId(chat_id), UpdateEvent::PinnedMessagesChanged {
                chat_id: ChatId(chat_id), pinned_ids: ids.iter().copied().map(MsgId).collect(),
            });
            (StatusCode::OK, Json(json!({ "chat_id": chat_id, "pinned_message_ids": ids }))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn pins_unpin(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, Query(q): Query<HashMap<String, String>>) -> Response {
    let (_chat, _m, is_admin) = match check_active_membership(&state, ChatId(chat_id), auth.user_id) { Ok(c) => c, Err(r) => return r };
    if !is_admin { return err_resp(StatusCode::FORBIDDEN, "forbidden", "Admin rights required to unpin messages"); }
    let msg_id = q.get("message_id").and_then(|s| s.parse::<i64>().ok()).map(MsgId);
    match state.db.unpin_message(ChatId(chat_id), msg_id) {
        Ok(ids) => {
            broadcast_chat_event(&state, ChatId(chat_id), UpdateEvent::PinnedMessagesChanged {
                chat_id: ChatId(chat_id), pinned_ids: ids.iter().copied().map(MsgId).collect(),
            });
            (StatusCode::OK, Json(json!({ "chat_id": chat_id, "pinned_message_ids": ids }))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn draft_get(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, Query(q): Query<HashMap<String, String>>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let topic_id = q.get("topic_id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    match state.db.get_draft(auth.user_id, ChatId(chat_id), topic_id) {
        Ok(draft) => (StatusCode::OK, Json(json!({ "chat_id": chat_id, "draft": draft }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn draft_put(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, body: Bytes) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let payload = match parse_json_object(&body) { Ok(v) => v, Err(r) => return r };
    let text = payload.get("text").and_then(|v| v.as_str()).unwrap_or("");
    let topic_id = payload.get("topic_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let reply_to = payload.get("reply_to_msg_id").or_else(|| payload.get("reply_to")).and_then(|v| v.as_i64());
    let no_webpage = payload.get("no_webpage").and_then(|v| v.as_bool()).unwrap_or(false);
    let entities = payload.get("entities").cloned().unwrap_or_else(|| json!([]));
    match state.db.upsert_draft(auth.user_id, ChatId(chat_id), topic_id, text, &entities, reply_to, no_webpage) {
        Ok(draft) => {
            state.gateway_hub.broadcast_to_user(auth.user_id, gpui_telegram_proto::ws::WsServerFrame::Update(UpdateEvent::DraftUpdated {
                chat_id: ChatId(chat_id), topic_id: if topic_id > 0 { Some(topic_id) } else { None }, draft: draft.clone(),
            }));
            (StatusCode::OK, Json(json!({ "chat_id": chat_id, "draft": draft }))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn draft_delete(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, Query(q): Query<HashMap<String, String>>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let topic_id = q.get("topic_id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    match state.db.upsert_draft(auth.user_id, ChatId(chat_id), topic_id, "", &json!([]), None, false) {
        Ok(draft) => {
            state.gateway_hub.broadcast_to_user(auth.user_id, gpui_telegram_proto::ws::WsServerFrame::Update(UpdateEvent::DraftUpdated {
                chat_id: ChatId(chat_id), topic_id: if topic_id > 0 { Some(topic_id) } else { None }, draft,
            }));
            (StatusCode::OK, Json(json!({ "chat_id": chat_id, "cleared": true }))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}
