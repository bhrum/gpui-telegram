pub mod auth_advanced;
pub mod auth_factors;
pub mod chat_extras;
pub mod chat_lifecycle;
pub mod media;
pub mod messages;
pub mod social;
pub mod sync_polls;
pub mod users;
pub mod analytics_wallet;
pub mod acp;
pub mod hermes;

use crate::db::schema::{ChatMemberRecord, ChatRecord, UserRecord};
use crate::server::AppState;
use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use gpui_telegram_core::error::TelegramError;
use gpui_telegram_core::types::{ChatId, UserId};
use gpui_telegram_proto::contracts::UserProfileDto;
use gpui_telegram_proto::sync::{PtsUpdate, UpdateEvent};
use gpui_telegram_proto::ws::WsServerFrame;
use serde_json::{json, Value};

pub fn err_resp(status: StatusCode, code: &str, msg: &str) -> Response {
    (status, Json(json!({ "error": code, "message": msg }))).into_response()
}

pub fn parse_json_object(bytes: &Bytes) -> Result<Value, Response> {
    if bytes.is_empty() {
        return Err(err_resp(StatusCode::BAD_REQUEST, "empty_body", "Request body cannot be empty"));
    }
    let val: Value = serde_json::from_slice(bytes)
        .map_err(|e| err_resp(StatusCode::BAD_REQUEST, "invalid_json", &format!("Malformed JSON: {e}")))?;
    if !val.is_object() {
        return Err(err_resp(StatusCode::BAD_REQUEST, "invalid_payload", "Expected JSON object"));
    }
    Ok(val)
}

pub fn map_telegram_err(err: TelegramError) -> Response {
    match err {
        TelegramError::NotFound(m) => err_resp(StatusCode::NOT_FOUND, "not_found", &m),
        TelegramError::PermissionDenied(m) => err_resp(StatusCode::FORBIDDEN, "forbidden", &m),
        TelegramError::Validation(m) => {
            if m.contains("nonce conflict") {
                err_resp(StatusCode::CONFLICT, "nonce_conflict", &m)
            } else {
                err_resp(StatusCode::BAD_REQUEST, "validation_error", &m)
            }
        }
        TelegramError::Database(_) => err_resp(StatusCode::INTERNAL_SERVER_ERROR, "database_error", "Internal storage error"),
        other => err_resp(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", &other.to_string()),
    }
}

pub fn check_active_membership(
    state: &AppState,
    chat_id: ChatId,
    user_id: UserId,
) -> Result<(ChatRecord, Option<ChatMemberRecord>, bool), Response> {
    let chat = match state.db.get_chat(chat_id) {
        Ok(Some(c)) => c,
        Ok(None) => return Err(err_resp(StatusCode::NOT_FOUND, "not_found", "Chat not found")),
        Err(_) => return Err(err_resp(StatusCode::INTERNAL_SERVER_ERROR, "database_error", "Failed to query chat")),
    };
    let member = match state.db.get_chat_member(chat_id, user_id) {
        Ok(m) => m,
        Err(_) => return Err(err_resp(StatusCode::INTERNAL_SERVER_ERROR, "database_error", "Failed to query membership")),
    };
    if chat.creator_id == user_id {
        return Ok((chat, member, true));
    }
    let Some(m) = member else {
        return Err(err_resp(StatusCode::FORBIDDEN, "forbidden", "Caller is not a member of this chat"));
    };
    if matches!(m.role.as_str(), "kicked" | "banned" | "left") {
        return Err(err_resp(StatusCode::FORBIDDEN, "forbidden", "Caller membership is not active"));
    }
    let is_admin = matches!(
        m.role.to_ascii_lowercase().as_str(),
        "owner" | "creator" | "admin" | "administrator"
    );
    Ok((chat, Some(m), is_admin))
}

pub fn broadcast_chat_pts(state: &AppState, chat_id: ChatId, pts_update: PtsUpdate) {
    state.gateway_hub.broadcast_to_chat_and_members(chat_id, None, WsServerFrame::PtsPush(pts_update));
}

pub fn broadcast_chat_event(state: &AppState, chat_id: ChatId, ev: UpdateEvent) {
    state.gateway_hub.broadcast_to_chat_and_members(chat_id, None, WsServerFrame::Update(ev));
}

pub fn user_record_to_profile(state: &AppState, u: &UserRecord) -> UserProfileDto {
    let emoji_status = state.db.get_user_emoji_status(u.user_id).ok().flatten();
    let online = state.gateway_hub.user_connection_count(u.user_id) > 0;
    UserProfileDto {
        user_id: u.user_id.raw(),
        username: u.username.clone(),
        first_name: u.first_name.clone(),
        last_name: u.last_name.clone(),
        bio: u.bio.clone(),
        phone: u.phone.clone(),
        avatar_media_id: u.profile_photo_file_id.clone(),
        photo_file_id: u.profile_photo_file_id.clone(),
        emoji_status,
        is_bot: u.is_bot,
        is_verified: u.is_verified,
        is_premium: false,
        status: if online { "online".into() } else { "offline".into() },
        last_seen_ms: Some(u.updated_at_ms),
    }
}

pub fn issue_persistent_session(
    state: &AppState,
    user: &UserRecord,
    device_name: &str,
) -> gpui_telegram_proto::contracts::AuthResponse {
    let created_at = chrono::DateTime::from_timestamp_millis(user.created_at_ms).unwrap_or_else(chrono::Utc::now);
    let mem_user = crate::auth::session_manager::UserRecord {
        user_id: user.user_id,
        username: user.username.clone().unwrap_or_else(|| format!("user_{}", user.user_id.0)),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        password_hash: user.password_hash.clone().unwrap_or_default(),
        created_at,
    };
    state.session_manager.insert_user(mem_user.clone());
    let (session, token) = state.session_manager.create_session(user.user_id, device_name, chrono::Duration::days(30));
    let db_sess = crate::db::schema::SessionRecord {
        session_id: session.session_id.clone(),
        user_id: user.user_id,
        device_id: device_name.to_string(),
        client_type: "gpui_desktop".into(),
        app_version: env!("CARGO_PKG_VERSION").into(),
        ip_address: None,
        location_desc: None,
        account_generation: state.session_manager.get_or_create_fence(user.user_id).current(),
        token_hash: session.token_hash.clone(),
        created_at_ms: session.created_at.timestamp_millis(),
        last_active_ms: session.created_at.timestamp_millis(),
        expires_at_ms: session.expires_at.timestamp_millis(),
        revoked_at_ms: None,
    };
    let _ = state.db.create_session(&db_sess);
    gpui_telegram_proto::contracts::AuthResponse {
        user_id: user.user_id.raw(),
        username: mem_user.username,
        session_id: session.session_id,
        token,
        first_name: Some(user.first_name.clone()),
        last_name: user.last_name.clone(),
        account_generation: Some(db_sess.account_generation),
        created_at: Some(created_at.to_rfc3339()),
        expires_at: Some(session.expires_at.to_rfc3339()),
    }
}
