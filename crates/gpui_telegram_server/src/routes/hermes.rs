use crate::routes::{broadcast_chat_pts, check_active_membership, err_resp, map_telegram_err};
use crate::server::{ensure_default_ai_bot, AppState, AuthBearer};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
};
use chrono::Utc;
use futures::stream::{self, StreamExt};
use gpui_telegram_agent::hermes::commands::InlineActionButton;
use gpui_telegram_agent::hermes::memory::{MemoryTarget, ObservationCategory, SoulProfile};
use gpui_telegram_agent::hermes::providers::{ApiMode, CredentialEntry, ProviderConfig};
use gpui_telegram_agent::hermes::security::{
    ApprovalDecisionAction, ApprovalStatus as SecurityApprovalStatus,
};
use gpui_telegram_agent::hermes::skills::{CreateSkillRequest, SkillDocument, SkillOrigin};
use gpui_telegram_agent::hermes::state_db::{
    ApprovalRecord, ApprovalStatus, CreateCronJobRequest, CreateKanbanCardRequest,
    CreateSessionRequest, CronJobPatch, CronJobRecord, KanbanBoardRecord, KanbanCardPatch,
    KanbanCardRecord, SessionListFilter, SessionRecord,
};
use gpui_telegram_agent::hermes::toolsets::{all_builtin_tool_specs, ToolsetId, ToolsetPreset};
use gpui_telegram_agent::hermes::trajectory::TrajectoryFormat;
use gpui_telegram_agent::hermes::{
    HeartbeatConfig, HermesEvent, HermesTurnRequest, HermesTurnResult,
};
use gpui_telegram_agent::tasks::CronRoutine;
use gpui_telegram_core::types::{ChatId, MsgId, PeerId, UserId};
use gpui_telegram_proto::message::{InlineKeyboardButton, MessageContent, MessageItem};
use gpui_telegram_proto::sync::UpdateEvent;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Shared Mapping & Bot Persona Helpers
// ============================================================================

fn parse_optional_bot_id(v: Option<&Value>) -> Option<String> {
    let val = v?;
    if let Some(s) = val.as_str() {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    val.as_u64()
        .map(|n| n.to_string())
        .or_else(|| val.as_i64().map(|n| n.to_string()))
}

pub(crate) fn map_proto_buttons(
    rows: &[Vec<InlineActionButton>],
) -> Vec<Vec<InlineKeyboardButton>> {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|b| InlineKeyboardButton {
                    text: b.text.clone(),
                    callback_data: if b.callback_data.trim().is_empty() {
                        None
                    } else {
                        Some(b.callback_data.clone())
                    },
                    url: b.url.clone(),
                    web_app_url: None,
                })
                .collect()
        })
        .collect()
}

pub(crate) fn map_buttons_json(rows: &[Vec<InlineActionButton>]) -> Value {
    let converted = map_proto_buttons(rows);
    serde_json::to_value(converted).unwrap_or_else(|_| json!([]))
}

fn format_usage_dto(res: &HermesTurnResult) -> Value {
    json!({
        "total_turns": res.iterations as u64,
        "prompt_tokens": res.usage.input_tokens,
        "completion_tokens": res.usage.output_tokens,
        "total_tokens": res.usage.total_tokens,
        "estimated_cost_usd": res.usage.estimated_cost_usd,
        "tool_calls": res.tool_calls.len() as u64,
        "by_model": [],
    })
}

fn turn_result_to_json(res: &HermesTurnResult) -> Value {
    json!({
        "turn_id": res.turn_id,
        "session_id": res.session_id,
        "status": "completed",
        "response": res.response_text,
        "model": res.model,
        "provider": res.provider,
        "events": res.events,
        "inline_keyboard": map_buttons_json(&res.buttons),
        "tool_calls_count": res.tool_calls.len(),
        "usage": format_usage_dto(res),
    })
}

fn session_to_dto(s: &SessionRecord) -> Value {
    let meta: Value = serde_json::from_str(&s.metadata_json).unwrap_or_else(|_| json!({}));
    let provider = meta
        .get("provider")
        .and_then(|v| v.as_str())
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(&s.api_mode)
        .to_string();
    let branch_name = meta
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .or_else(|| {
            if s.parent_session_id.is_some() {
                s.title.clone()
            } else {
                None
            }
        });
    json!({
        "session_id": s.id,
        "chat_id": s.chat_id,
        "bot_id": s.bot_id.as_deref().and_then(|b| b.parse::<u64>().ok()),
        "title": s.title,
        "model": s.model,
        "provider": provider,
        "parent_session_id": s.parent_session_id,
        "branch_name": branch_name,
        "message_count": s.message_count.max(0) as usize,
        "token_count": s.input_tokens.saturating_add(s.output_tokens).max(0) as u64,
        "created_at_ms": s.started_at,
        "updated_at_ms": s.ended_at.unwrap_or(s.started_at),
    })
}

fn skill_to_dto(doc: &SkillDocument, updated_at_ms: i64) -> Value {
    let content = if doc.raw_markdown.trim().is_empty() {
        doc.body.clone()
    } else {
        doc.raw_markdown.clone()
    };
    json!({
        "name": doc.frontmatter.name,
        "description": doc.frontmatter.description,
        "category": doc.frontmatter.origin.as_str(),
        "content": content,
        "enabled": true,
        "approved": doc.trusted || doc.security_report.safe_to_auto_install,
        "security_status": format!("{:?}", doc.security_report.risk_level).to_lowercase(),
        "tags": doc.frontmatter.tags,
        "updated_at_ms": updated_at_ms,
    })
}

fn cron_to_dto(rec: &CronJobRecord) -> Value {
    json!({
        "job_id": rec.id,
        "name": rec.name,
        "schedule": rec.schedule,
        "prompt": rec.prompt,
        "chat_id": rec.target_chat_id.or(rec.origin_chat_id),
        "bot_id": rec.deliver_to.as_deref().and_then(|s| s.parse::<u64>().ok()),
        "enabled": rec.enabled,
        "last_run_ms": rec.last_run_at,
        "next_run_ms": rec.next_run_at,
        "last_status": Value::Null,
    })
}

fn card_to_dto(card: &KanbanCardRecord) -> Value {
    json!({
        "card_id": card.id,
        "board_id": card.board_id,
        "title": card.title,
        "description": card.description,
        "column": card.column_name,
        "priority": card.priority,
        "assigned_agent": card.assignee,
        "position": card.position as i32,
        "created_at_ms": card.created_at,
        "updated_at_ms": card.updated_at,
    })
}

fn board_to_dto(board: &KanbanBoardRecord, cards: &[KanbanCardRecord]) -> Value {
    json!({
        "board_id": board.id,
        "name": board.name,
        "columns": board.columns,
        "cards": cards.iter().map(card_to_dto).collect::<Vec<_>>(),
        "created_at_ms": board.created_at,
    })
}

fn approval_status_str(status: ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::ApprovedOnce => "approved_once",
        ApprovalStatus::ApprovedSession => "approved_session",
        ApprovalStatus::ApprovedPermanent => "approved_permanent",
        ApprovalStatus::Denied => "denied",
    }
}

fn sec_approval_status_str(status: SecurityApprovalStatus) -> &'static str {
    match status {
        SecurityApprovalStatus::Pending => "pending",
        SecurityApprovalStatus::ApprovedOnce => "approved_once",
        SecurityApprovalStatus::ApprovedSession => "approved_session",
        SecurityApprovalStatus::ApprovedPermanent => "approved_permanent",
        SecurityApprovalStatus::Denied => "denied",
    }
}

fn db_approval_to_dto(rec: &ApprovalRecord) -> Value {
    json!({
        "approval_id": rec.id,
        "session_id": rec.session_id,
        "tool_name": rec.pattern_name,
        "command_preview": rec.command,
        "risk_level": rec.risk_level,
        "reason": rec.pattern_name,
        "status": approval_status_str(rec.status),
        "created_at_ms": rec.created_at,
        "decided_at_ms": rec.decided_at,
    })
}

fn toolsets_list_json(state: &AppState) -> Vec<Value> {
    let guard = state
        .hermes
        .toolsets()
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let all_specs = all_builtin_tool_specs();
    ToolsetId::ALL
        .iter()
        .map(|id| {
            let matching: Vec<_> = all_specs.iter().filter(|s| s.toolset == *id).collect();
            let tools: Vec<&str> = matching.iter().map(|s| s.name.as_str()).collect();
            let risk_level = matching
                .iter()
                .map(|s| match s.risk_level {
                    gpui_telegram_agent::ToolRiskLevel::ReadOnly => "read_only",
                    gpui_telegram_agent::ToolRiskLevel::Low => "low",
                    gpui_telegram_agent::ToolRiskLevel::Medium => "medium",
                    gpui_telegram_agent::ToolRiskLevel::High => "high",
                    gpui_telegram_agent::ToolRiskLevel::CriticalDestructive => "critical",
                })
                .max_by_key(|r| match *r {
                    "critical" => 4,
                    "high" => 3,
                    "medium" => 2,
                    "low" => 1,
                    _ => 0,
                })
                .unwrap_or("low");
            json!({
                "name": id.as_str(),
                "description": id.description(),
                "enabled": guard.is_toolset_enabled(*id),
                "tools": tools,
                "risk_level": risk_level,
            })
        })
        .collect()
}

fn build_memory_state_json(state: &AppState) -> Result<Value, Response> {
    let mem = state
        .hermes
        .memory()
        .read(MemoryTarget::Memory)
        .map_err(map_telegram_err)?;
    let usr = state
        .hermes
        .memory()
        .read(MemoryTarget::User)
        .map_err(map_telegram_err)?;
    let now_ms = Utc::now().timestamp_millis();

    let mut entries = Vec::with_capacity(mem.entries.len() + usr.entries.len());
    for (idx, item) in mem.entries.iter().enumerate() {
        entries.push(json!({
            "id": format!("mem_{}", idx + 1),
            "key": format!("memory_{}", idx + 1),
            "content": item,
            "category": "memory",
            "scope": "memory",
            "char_count": item.chars().count(),
            "updated_at_ms": now_ms,
        }));
    }
    for (idx, item) in usr.entries.iter().enumerate() {
        entries.push(json!({
            "id": format!("usr_{}", idx + 1),
            "key": format!("user_{}", idx + 1),
            "content": item,
            "category": "user",
            "scope": "user",
            "char_count": item.chars().count(),
            "updated_at_ms": now_ms,
        }));
    }

    Ok(json!({
        "entries": entries,
        "memory_markdown": mem.raw,
        "user_markdown": usr.raw,
        "total_chars": mem.char_count + usr.char_count,
        "char_limit": mem.char_limit + usr.char_limit,
    }))
}

fn honcho_profile_to_dto(profile: gpui_telegram_agent::hermes::memory::DialecticProfile) -> Value {
    let notes: Vec<Value> = profile
        .observations
        .into_iter()
        .map(|obs| serde_json::to_value(obs).unwrap_or(Value::Null))
        .collect();
    json!({
        "enabled": true,
        "peer_name": profile.user_id,
        "session_notes": notes,
        "profile_summary": profile.summary,
        "updated_at_ms": profile.updated_at_ms,
    })
}

fn provider_to_dto(
    cfg: &ProviderConfig,
    active_provider: &str,
    model_profiles: &HashMap<String, gpui_telegram_agent::hermes::providers::ModelProfile>,
) -> Value {
    let mut models = Vec::new();
    if !cfg.default_model.trim().is_empty() {
        models.push(cfg.default_model.clone());
    }
    for m in &cfg.fallback_models {
        if !models.contains(m) {
            models.push(m.clone());
        }
    }
    for prof in model_profiles.values() {
        if prof.provider_id == cfg.id && !models.contains(&prof.model) {
            models.push(prof.model.clone());
        }
    }
    json!({
        "provider_id": cfg.id,
        "name": cfg.name,
        "api_mode": cfg.api_mode.as_str(),
        "base_url": if cfg.base_url.trim().is_empty() { None } else { Some(cfg.base_url.clone()) },
        "default_model": cfg.default_model,
        "models": models,
        "enabled": cfg.enabled,
        "has_api_key": !cfg.credentials.entries.is_empty(),
        "is_active": cfg.id == active_provider,
    })
}

pub(crate) fn load_bot_persona_config(
    state: &AppState,
    bot_user_id: UserId,
) -> (Option<String>, Option<String>, Option<String>) {
    let key = bot_user_id.0.to_string();
    let kv_soul = state
        .hermes
        .store()
        .kv_get("bot_soul", &key)
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok());
    let kv_persona = state
        .hermes
        .store()
        .kv_get("bot_persona", &key)
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok());

    let user = state.db.get_user(bot_user_id).ok().flatten();
    let mut system_prompt = kv_soul
        .as_ref()
        .and_then(|v| {
            v.get("system_prompt")
                .or_else(|| v.get("content"))
                .and_then(|s| s.as_str())
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .or_else(|| {
            kv_persona
                .as_ref()
                .and_then(|v| {
                    v.get("system_prompt")
                        .or_else(|| v.get("persona"))
                        .and_then(|s| s.as_str())
                })
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToString::to_string)
        });

    if system_prompt.is_none() {
        if let Some(u) = &user {
            let mut identity = format!("You are {} (@{}).", u.first_name, u.username.as_deref().unwrap_or("bot"));
            if let Some(bio) = u.bio.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
                identity.push(' ');
                identity.push_str(bio);
            }
            system_prompt = Some(identity);
        }
    }

    let model = kv_soul
        .as_ref()
        .and_then(|v| v.get("model").and_then(|s| s.as_str()))
        .or_else(|| {
            kv_persona
                .as_ref()
                .and_then(|v| v.get("model").and_then(|s| s.as_str()))
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);

    let welcome = kv_soul
        .as_ref()
        .and_then(|v| v.get("welcome_message").and_then(|s| s.as_str()))
        .or_else(|| {
            kv_persona
                .as_ref()
                .and_then(|v| v.get("welcome_message").and_then(|s| s.as_str()))
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);

    (system_prompt, model, welcome)
}

pub(crate) fn save_bot_persona_kv(
    state: &AppState,
    bot_user_id: UserId,
    chat_id: Option<ChatId>,
    persona_name: &str,
    persona: Option<&str>,
    system_prompt: Option<&str>,
    model: Option<&str>,
    welcome_message: Option<&str>,
) {
    let key = bot_user_id.0.to_string();
    let payload = json!({
        "bot_id": bot_user_id.0,
        "chat_id": chat_id.map(|c| c.0),
        "persona_name": persona_name,
        "persona": persona,
        "system_prompt": system_prompt.or(persona),
        "model": model,
        "welcome_message": welcome_message,
        "updated_at_ms": Utc::now().timestamp_millis(),
    });
    if let Ok(raw) = serde_json::to_string(&payload) {
        let _ = state.hermes.store().kv_set("bot_persona", &key, &raw);
    }
}

pub(crate) fn build_default_bot_quick_actions(
    res: &HermesTurnResult,
    prompt_hint: &str,
) -> Vec<Vec<InlineKeyboardButton>> {
    if !res.buttons.is_empty() {
        return map_proto_buttons(&res.buttons);
    }
    let has_pending_approval = res
        .events
        .iter()
        .any(|ev| matches!(ev, HermesEvent::ApprovalRequired { .. }));
    if has_pending_approval {
        for ev in &res.events {
            if let HermesEvent::ApprovalRequired {
                approval_id,
                tool_name: _,
                ..
            } = ev
            {
                return vec![vec![
                    InlineKeyboardButton {
                        text: "✅ Approve Once".to_string(),
                        callback_data: Some(format!("hermes:approve:{approval_id}:approve_once")),
                        url: None,
                        web_app_url: None,
                    },
                    InlineKeyboardButton {
                        text: "🛡️ Session".to_string(),
                        callback_data: Some(format!("hermes:approve:{approval_id}:approve_session")),
                        url: None,
                        web_app_url: None,
                    },
                    InlineKeyboardButton {
                        text: "❌ Deny".to_string(),
                        callback_data: Some(format!("hermes:approve:{approval_id}:deny")),
                        url: None,
                        web_app_url: None,
                    },
                ]];
            }
        }
    }

    let lower = prompt_hint.to_ascii_lowercase();
    let mut row = Vec::new();
    if lower.contains("voice")
        || lower.contains("talk")
        || lower.contains("call")
        || lower.contains("speak")
    {
        row.push(InlineKeyboardButton {
            text: "🎙️ Talk Mode".to_string(),
            callback_data: Some("talk:start".to_string()),
            url: None,
            web_app_url: None,
        });
    }
    if lower.contains("desktop")
        || lower.contains("computer")
        || lower.contains("screen")
        || lower.contains("browser")
    {
        row.push(InlineKeyboardButton {
            text: "🖥️ Remote Computer".to_string(),
            callback_data: Some("computer:start".to_string()),
            url: None,
            web_app_url: None,
        });
    }
    if row.is_empty() {
        Vec::new()
    } else {
        vec![row]
    }
}

pub(crate) async fn deliver_hermes_proactive_message(
    state: &AppState,
    chat_id: ChatId,
    bot_id_hint: Option<&str>,
    text: &str,
    buttons: Option<Vec<Vec<InlineKeyboardButton>>>,
) -> Option<MessageItem> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if state.db.get_chat(chat_id).ok().flatten().is_none() {
        return None;
    }

    let bot_user_id = bot_id_hint
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .map(UserId)
        .filter(|uid| {
            state
                .db
                .get_user(*uid)
                .ok()
                .flatten()
                .is_some_and(|u| u.is_bot)
        })
        .or_else(|| {
            state
                .db
                .get_chat_members(chat_id)
                .ok()
                .and_then(|members| {
                    members.into_iter().find(|uid| {
                        state
                            .db
                            .get_user(*uid)
                            .ok()
                            .flatten()
                            .is_some_and(|u| u.is_bot)
                    })
                })
        })
        .unwrap_or_else(|| ensure_default_ai_bot(&state.db));

    let kb_json_opt = buttons
        .as_ref()
        .filter(|kb| !kb.is_empty())
        .and_then(|kb| serde_json::to_string(kb).ok());

    let mut bot_msg = MessageItem::new(
        MsgId(0),
        chat_id,
        bot_user_id,
        PeerId::Chat(chat_id),
        MessageContent::plain_text(trimmed),
    );
    bot_msg.flags.is_out = false;

    let pts_update = state
        .message_pipeline
        .process_incoming_message(bot_msg)
        .ok()?;
    let saved_msg = match &pts_update.event {
        UpdateEvent::NewMessage(m) => {
            if let Some(ref kb_json) = kb_json_opt {
                let _ = state
                    .db
                    .save_message_extras(chat_id, m.id, Some(kb_json), None);
            }
            Some(m.clone())
        }
        _ => None,
    };
    broadcast_chat_pts(state, chat_id, pts_update);
    saved_msg
}

fn split_spoken_segments(text: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?' | '。' | '！' | '？' | '\n') {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                segments.push(trimmed);
            }
            current.clear();
        }
    }
    let tail = current.trim().to_string();
    if !tail.is_empty() {
        segments.push(tail);
    }
    if segments.is_empty() && !text.trim().is_empty() {
        segments.push(text.trim().to_string());
    }
    segments
}

fn sse_response_from_chunks(chunks: Vec<String>) -> Response {
    let byte_stream = stream::iter(
        chunks
            .into_iter()
            .map(|c| Ok::<axum::body::Bytes, Infallible>(axum::body::Bytes::from(c))),
    );
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-cache")
        .body(Body::from_stream(byte_stream))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

// ============================================================================
// Health, Turns, Events & Slash Commands
// ============================================================================

pub async fn hermes_status(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let (active_provider, active_model, api_mode) = state.hermes.providers().active_info();
    let (toolsets_count, tools_count) = {
        let reg = state
            .hermes
            .toolsets()
            .read()
            .unwrap_or_else(|e| e.into_inner());
        (reg.enabled_toolsets.len(), reg.active_specs().len())
    };
    let skills_count = state.hermes.skills().skills_list(None).len();
    let cron_count = state
        .hermes
        .store()
        .list_cron_jobs(false)
        .map(|v| v.len())
        .unwrap_or(0);
    let sessions_count = state
        .hermes
        .store()
        .list_sessions(SessionListFilter::default())
        .map(|v| v.len())
        .unwrap_or(0);
    let pending_db = state
        .hermes
        .store()
        .list_approvals(None, true)
        .map(|v| v.len())
        .unwrap_or(0);
    let pending_mem = state
        .hermes
        .security()
        .read()
        .map(|s| s.approvals.pending.len())
        .unwrap_or(0);
    let heartbeat_enabled = state.hermes.heartbeat().status().enabled;

    (
        StatusCode::OK,
        Json(json!({
            "healthy": true,
            "engine": "hermes-rust",
            "active_model": active_model,
            "active_provider": active_provider,
            "api_mode": api_mode.as_str(),
            "toolsets_count": toolsets_count,
            "tools_count": tools_count,
            "skills_count": skills_count,
            "cron_count": cron_count,
            "sessions_count": sessions_count,
            "pending_approvals_count": pending_db.max(pending_mem),
            "heartbeat_enabled": heartbeat_enabled,
        })),
    )
        .into_response()
}

pub async fn hermes_turns_create(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let prompt = payload
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if prompt.trim().is_empty() {
        return err_resp(StatusCode::BAD_REQUEST, "invalid_prompt", "prompt is required");
    }

    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    if let Some(cid) = chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }

    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let bot_id = parse_optional_bot_id(payload.get("bot_id"));
    let mut model = payload
        .get("model")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let provider = payload
        .get("provider")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let (Some(prov), Some(m)) = (provider, &model) {
        if !m.contains('/') {
            model = Some(format!("{prov}/{m}"));
        }
    }
    let mut system_prompt = payload
        .get("system_prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);

    if let Some(bid_num) = bot_id.as_deref().and_then(|b| b.parse::<u64>().ok()) {
        let (bot_sp, bot_model, _) = load_bot_persona_config(&state, UserId(bid_num));
        if system_prompt.is_none() {
            system_prompt = bot_sp;
        }
        if model.is_none() {
            model = bot_model;
        }
    }

    let yolo_mode = payload.get("yolo_mode").and_then(|v| v.as_bool()).unwrap_or(false);
    if yolo_mode {
        if let Some(ref sid) = session_id {
            if let Ok(mut sec) = state.hermes.security().write() {
                sec.approvals
                    .set_session_mode(sid.clone(), gpui_telegram_agent::hermes::security::ApprovalMode::Yolo);
            }
        }
    }

    let req = HermesTurnRequest {
        session_id,
        chat_id,
        bot_id,
        user_id: Some(auth.user_id.0.to_string()),
        source: "api".to_string(),
        prompt,
        model,
        system_prompt,
        thinking_budget_tokens: payload
            .get("thinking_budget_tokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
        preset: payload
            .get("preset")
            .and_then(|v| v.as_str())
            .and_then(ToolsetPreset::from_str),
        max_iterations: payload
            .get("max_iterations")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize),
        max_tokens: payload
            .get("max_tokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32),
        temperature: payload
            .get("temperature")
            .and_then(|v| v.as_f64())
            .map(|n| n as f32),
        skip_slash_commands: payload
            .get("skip_slash_commands")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        caller_depth: 0,
    };

    match state.hermes.run_turn(req).await {
        Ok(res) => {
            if payload
                .get("deliver_to_chat")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                if let Some(cid) = chat_id {
                    let buttons = build_default_bot_quick_actions(&res, &res.response_text);
                    let kb_opt = if buttons.is_empty() { None } else { Some(buttons) };
                    let _ = deliver_hermes_proactive_message(
                        &state,
                        ChatId(cid),
                        None,
                        &res.response_text,
                        kb_opt,
                    )
                    .await;
                }
            }
            (StatusCode::OK, Json(turn_result_to_json(&res))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct EventsQuery {
    pub stream: Option<bool>,
    pub sse: Option<bool>,
}

pub async fn hermes_turn_events(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(turn_id): Path<String>,
    Query(query): Query<EventsQuery>,
    headers: HeaderMap,
) -> Response {
    let events = state.hermes.get_turn_events(&turn_id);
    let wants_sse = query.stream == Some(true)
        || query.sse == Some(true)
        || headers
            .get(header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|a| a.contains("text/event-stream"));

    if wants_sse {
        let mut chunks: Vec<String> = events
            .iter()
            .filter_map(|ev| serde_json::to_string(ev).ok())
            .map(|line| format!("event: hermes\ndata: {line}\n\n"))
            .collect();
        chunks.push("data: [DONE]\n\n".to_string());
        return sse_response_from_chunks(chunks);
    }

    (
        StatusCode::OK,
        Json(json!({
            "turn_id": turn_id,
            "events": events,
            "items": events,
        })),
    )
        .into_response()
}

pub async fn hermes_command_run(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let command = payload
        .get("command")
        .or_else(|| payload.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if command.is_empty() {
        return err_resp(StatusCode::BAD_REQUEST, "invalid_command", "command is required");
    }

    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    if let Some(cid) = chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }

    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let bot_id = parse_optional_bot_id(payload.get("bot_id"));

    match state
        .hermes
        .execute_command(session_id, chat_id, bot_id.as_deref(), &command)
    {
        Ok((resolved_sid, outcome)) => (
            StatusCode::OK,
            Json(json!({
                "handled": outcome.handled,
                "command": outcome.command,
                "session_id": resolved_sid,
                "output": outcome.response_text,
                "reply_text": outcome.response_text,
                "inline_keyboard": map_buttons_json(&outcome.buttons),
                "effects": [outcome.ui_effect],
                "data": {
                    "is_error": outcome.is_error,
                    "mutated_fields": outcome.mutated_fields,
                    "should_continue_to_llm": outcome.should_continue_to_llm,
                    "rewritten_prompt": outcome.rewritten_prompt,
                },
            })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandsQuery {
    pub prefix: Option<String>,
}

pub async fn hermes_commands_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CommandsQuery>,
) -> Response {
    let list = match query.prefix.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(prefix) => state.hermes.autocomplete_commands(prefix),
        None => state.hermes.list_commands(),
    };
    let commands: Vec<Value> = list
        .into_iter()
        .map(|d| {
            json!({
                "name": d.name,
                "slash_name": d.slash_name,
                "aliases": d.aliases,
                "syntax": d.syntax,
                "category": d.category.label(),
                "description": d.description,
                "accepts_args": d.accepts_args,
            })
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "commands": commands,
            "total": commands.len(),
        })),
    )
        .into_response()
}

// ============================================================================
// Sessions
// ============================================================================

#[derive(Debug, Deserialize, Default)]
pub struct SessionsListQuery {
    pub chat_id: Option<i64>,
    pub bot_id: Option<String>,
    pub source: Option<String>,
    pub active_only: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn hermes_sessions_list(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(q): Query<SessionsListQuery>,
) -> Response {
    if let Some(cid) = q.chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }
    let filter = SessionListFilter {
        chat_id: q.chat_id,
        bot_id: q.bot_id.filter(|s| !s.trim().is_empty()),
        source: q.source.filter(|s| !s.trim().is_empty()),
        active_only: q.active_only.unwrap_or(false),
        limit: Some(q.limit.unwrap_or(100)),
        offset: q.offset,
    };
    match state.hermes.store().list_sessions(filter) {
        Ok(list) => {
            let sessions: Vec<Value> = list.iter().map(session_to_dto).collect();
            (
                StatusCode::OK,
                Json(json!({ "sessions": sessions, "total": sessions.len() })),
            )
                .into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_session_get(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> Response {
    let sess = match state.hermes.store().get_session(&session_id) {
        Ok(Some(s)) => s,
        Ok(None) => {
            return err_resp(
                StatusCode::NOT_FOUND,
                "session_not_found",
                "Hermes session not found",
            )
        }
        Err(e) => return map_telegram_err(e),
    };
    if let Some(cid) = sess.chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }
    let messages = state
        .hermes
        .store()
        .get_messages(&session_id, Some(200))
        .unwrap_or_default();
    let branches = state
        .hermes
        .store()
        .list_branches(&session_id)
        .unwrap_or_default()
        .iter()
        .map(session_to_dto)
        .collect::<Vec<_>>();

    (
        StatusCode::OK,
        Json(json!({
            "session": session_to_dto(&sess),
            "messages": messages,
            "branches": branches,
        })),
    )
        .into_response()
}

pub async fn hermes_sessions_search(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let query = payload
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let limit = payload
        .get("limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(25) as usize;
    match state
        .hermes
        .store()
        .session_search(&query, session_id, limit)
    {
        Ok(hits) => (
            StatusCode::OK,
            Json(json!({ "results": hits, "hits": hits })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_session_branch(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let branch_name = payload
        .get("branch_name")
        .or_else(|| payload.get("title"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let up_to_seq = payload.get("up_to_seq").and_then(|v| v.as_i64());
    match state
        .hermes
        .store()
        .branch_session(&session_id, up_to_seq, branch_name)
    {
        Ok(branched) => (StatusCode::CREATED, Json(session_to_dto(&branched))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_session_compress(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
    payload: Option<Json<Value>>,
) -> Response {
    let focus = payload
        .as_ref()
        .and_then(|Json(v)| v.get("focus").and_then(|s| s.as_str()))
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let cmd = match focus {
        Some(f) => format!("/compact {f}"),
        None => "/compact".to_string(),
    };
    match state
        .hermes
        .execute_command(Some(&session_id), None, None, &cmd)
    {
        Ok((sid, outcome)) => (
            StatusCode::OK,
            Json(json!({
                "session_id": sid,
                "compressed": !outcome.is_error,
                "summary": outcome.response_text,
                "mutated_fields": outcome.mutated_fields,
            })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_session_undo(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> Response {
    match state.hermes.store().undo_last_turn(&session_id) {
        Ok(res) => (StatusCode::OK, Json(json!(res))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_session_export(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
    payload: Option<Json<Value>>,
) -> Response {
    let val = payload.map(|Json(v)| v).unwrap_or_else(|| json!({}));
    let raw_fmt = val
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("markdown")
        .trim()
        .to_ascii_lowercase();
    let redact = val.get("redact").and_then(|v| v.as_bool()).unwrap_or(true);

    if raw_fmt == "markdown" || raw_fmt == "md" {
        let sess = match state.hermes.store().get_session(&session_id) {
            Ok(Some(s)) => s,
            Ok(None) => {
                return err_resp(
                    StatusCode::NOT_FOUND,
                    "session_not_found",
                    "Hermes session not found",
                )
            }
            Err(e) => return map_telegram_err(e),
        };
        let msgs = state
            .hermes
            .store()
            .get_messages(&session_id, None)
            .unwrap_or_default();
        let mut md = format!(
            "# Hermes Session {}\n\n- Model: `{}`\n- Messages: {}\n\n",
            sess.title.as_deref().unwrap_or(&sess.id),
            sess.model,
            msgs.len()
        );
        for m in msgs {
            md.push_str(&format!("### {} (#{}) \n\n{}\n\n", m.role, m.seq, m.content));
        }
        return (
            StatusCode::OK,
            Json(json!({
                "session_id": sess.id,
                "format": "markdown",
                "content": md,
                "exported_at_ms": Utc::now().timestamp_millis(),
            })),
        )
            .into_response();
    }

    let fmt = TrajectoryFormat::parse(&raw_fmt).unwrap_or(TrajectoryFormat::HermesNative);
    match state.hermes.export_session(&session_id, fmt, redact, None) {
        Ok(exported) => (
            StatusCode::OK,
            Json(json!({
                "session_id": exported.session_id,
                "format": raw_fmt,
                "content": exported.payload,
                "message_count": exported.message_count,
                "redaction_report": exported.redaction_report,
                "exported_at_ms": Utc::now().timestamp_millis(),
            })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

// ============================================================================
// Memory, SOUL & Honcho
// ============================================================================

pub async fn hermes_memory_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    match build_memory_state_json(&state) {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(resp) => resp,
    }
}

pub async fn hermes_memory_put(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    if let Some(mem_md) = payload.get("memory_markdown").and_then(|v| v.as_str()) {
        if let Err(e) = state.hermes.memory().set_raw(MemoryTarget::Memory, mem_md) {
            return map_telegram_err(e);
        }
    }
    if let Some(user_md) = payload.get("user_markdown").and_then(|v| v.as_str()) {
        if let Err(e) = state.hermes.memory().set_raw(MemoryTarget::User, user_md) {
            return map_telegram_err(e);
        }
    }
    match build_memory_state_json(&state) {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(resp) => resp,
    }
}

pub async fn hermes_memory_action(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let action = payload
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("add")
        .trim()
        .to_ascii_lowercase();
    let scope_raw = payload
        .get("scope")
        .or_else(|| payload.get("target"))
        .and_then(|v| v.as_str())
        .unwrap_or("memory");
    let target = MemoryTarget::parse(scope_raw).unwrap_or(MemoryTarget::Memory);
    let content = payload
        .get("content")
        .or_else(|| payload.get("entry"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let old_text = payload
        .get("old_text")
        .or_else(|| payload.get("key"))
        .and_then(|v| v.as_str())
        .unwrap_or(content);

    let res = match action.as_str() {
        "add" => state.hermes.memory().add(target, content),
        "replace" | "update" => state.hermes.memory().replace(target, old_text, content),
        "remove" | "delete" => state.hermes.memory().remove(target, old_text),
        "clear" => state.hermes.memory().clear(target),
        "set_raw" | "set" => state.hermes.memory().set_raw(target, content),
        other => {
            return err_resp(
                StatusCode::BAD_REQUEST,
                "invalid_memory_action",
                &format!("Unsupported memory action '{other}'"),
            );
        }
    };
    if let Err(e) = res {
        return map_telegram_err(e);
    }
    match build_memory_state_json(&state) {
        Ok(val) => (StatusCode::OK, Json(val)).into_response(),
        Err(resp) => resp,
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct SoulQuery {
    pub bot_id: Option<u64>,
    pub chat_id: Option<i64>,
}

pub async fn hermes_soul_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(query): Query<SoulQuery>,
) -> Response {
    if let Some(bot_id) = query.bot_id {
        if let Ok(Some(raw)) = state
            .hermes
            .store()
            .kv_get("bot_soul", &bot_id.to_string())
        {
            if let Ok(val) = serde_json::from_str::<Value>(&raw) {
                return (StatusCode::OK, Json(val)).into_response();
            }
        }
        if let Ok(Some(raw)) = state
            .hermes
            .store()
            .kv_get("bot_persona", &bot_id.to_string())
        {
            if let Ok(val) = serde_json::from_str::<Value>(&raw) {
                let persona_name = val
                    .get("persona_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Hermes Bot")
                    .to_string();
                let sp = val
                    .get("system_prompt")
                    .or_else(|| val.get("persona"))
                    .and_then(|v| v.as_str())
                    .map(ToString::to_string);
                return (
                    StatusCode::OK,
                    Json(json!({
                        "bot_id": bot_id,
                        "chat_id": query.chat_id.or_else(|| val.get("chat_id").and_then(|v| v.as_i64())),
                        "persona_name": persona_name,
                        "content": sp.clone().unwrap_or_default(),
                        "system_prompt": sp,
                        "welcome_message": val.get("welcome_message").cloned().unwrap_or(Value::Null),
                        "model": val.get("model").cloned().unwrap_or(Value::Null),
                        "updated_at_ms": val.get("updated_at_ms").and_then(|v| v.as_i64()).unwrap_or_else(|| Utc::now().timestamp_millis()),
                    })),
                )
                    .into_response();
            }
        }
        if let Ok(Some(u)) = state.db.get_user(UserId(bot_id)) {
            let content = u.bio.clone().unwrap_or_default();
            return (
                StatusCode::OK,
                Json(json!({
                    "bot_id": bot_id,
                    "chat_id": query.chat_id,
                    "persona_name": u.first_name,
                    "content": content,
                    "system_prompt": u.bio,
                    "welcome_message": Value::Null,
                    "model": Value::Null,
                    "updated_at_ms": u.updated_at_ms,
                })),
            )
                .into_response();
        }
    }

    match state.hermes.memory().read_soul() {
        Ok(profile) => (
            StatusCode::OK,
            Json(json!({
                "bot_id": query.bot_id,
                "chat_id": query.chat_id,
                "persona_name": profile.name,
                "content": profile.raw_markdown,
                "system_prompt": profile.persona,
                "welcome_message": Value::Null,
                "model": Value::Null,
                "updated_at_ms": Utc::now().timestamp_millis(),
            })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_soul_put(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let bot_id = payload.get("bot_id").and_then(|v| v.as_u64());
    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    let persona_name = payload
        .get("persona_name")
        .or_else(|| payload.get("name"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Hermes")
        .to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let system_prompt = payload
        .get("system_prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .or_else(|| {
            if !content.trim().is_empty() {
                Some(content.clone())
            } else {
                None
            }
        });
    let welcome_message = payload
        .get("welcome_message")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let model = payload
        .get("model")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let now_ms = Utc::now().timestamp_millis();

    if let Some(bid) = bot_id {
        let dto = json!({
            "bot_id": bid,
            "chat_id": chat_id,
            "persona_name": persona_name,
            "content": if content.trim().is_empty() { system_prompt.clone().unwrap_or_default() } else { content },
            "system_prompt": system_prompt,
            "welcome_message": welcome_message,
            "model": model,
            "updated_at_ms": now_ms,
        });
        if let Ok(raw) = serde_json::to_string(&dto) {
            if let Err(e) = state.hermes.store().kv_set("bot_soul", &bid.to_string(), &raw) {
                return map_telegram_err(e);
            }
            let _ = state
                .hermes
                .store()
                .kv_set("bot_persona", &bid.to_string(), &raw);
        }
        return (StatusCode::OK, Json(dto)).into_response();
    }

    let profile = if !content.trim().is_empty() {
        match state.hermes.memory().write_soul_raw(&content) {
            Ok(p) => p,
            Err(e) => return map_telegram_err(e),
        }
    } else {
        let mut p = SoulProfile::new(
            &persona_name,
            "Personal AI Assistant",
            system_prompt.as_deref().unwrap_or(""),
        );
        p.raw_markdown = p.to_markdown();
        if let Err(e) = state.hermes.memory().write_soul(&p) {
            return map_telegram_err(e);
        }
        p
    };

    (
        StatusCode::OK,
        Json(json!({
            "bot_id": Value::Null,
            "chat_id": chat_id,
            "persona_name": profile.name,
            "content": profile.raw_markdown,
            "system_prompt": system_prompt.unwrap_or(profile.persona),
            "welcome_message": welcome_message,
            "model": model,
            "updated_at_ms": now_ms,
        })),
    )
        .into_response()
}

#[derive(Debug, Deserialize, Default)]
pub struct HonchoQuery {
    pub user_id: Option<String>,
    pub peer_name: Option<String>,
}

pub async fn hermes_honcho_get(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(q): Query<HonchoQuery>,
) -> Response {
    let uid = q
        .user_id
        .or(q.peer_name)
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| format!("user_{}", auth.user_id.0));
    match state.hermes.honcho().profile(&uid) {
        Ok(prof) => (StatusCode::OK, Json(honcho_profile_to_dto(prof))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_honcho_post(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let uid = payload
        .get("peer_name")
        .or_else(|| payload.get("user_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("user_{}", auth.user_id.0));
    let category = payload
        .get("category")
        .and_then(|v| v.as_str())
        .map(ObservationCategory::parse)
        .unwrap_or(ObservationCategory::Fact);

    if let Some(note) = payload
        .get("note")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if let Err(e) = state
            .hermes
            .honcho()
            .record_observation(&uid, category, note, 1.0, None)
        {
            return map_telegram_err(e);
        }
    }

    if let Some(q) = payload
        .get("query")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        match state.hermes.honcho().query(&uid, q) {
            Ok(qres) => {
                let prof = state.hermes.honcho().profile(&uid).ok();
                let mut dto = prof
                    .map(honcho_profile_to_dto)
                    .unwrap_or_else(|| json!({ "enabled": true, "peer_name": uid, "session_notes": [] }));
                if let Some(obj) = dto.as_object_mut() {
                    obj.insert("profile_summary".to_string(), json!(qres.answer));
                }
                return (StatusCode::OK, Json(dto)).into_response();
            }
            Err(e) => return map_telegram_err(e),
        }
    }

    match state.hermes.honcho().profile(&uid) {
        Ok(prof) => (StatusCode::OK, Json(honcho_profile_to_dto(prof))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

// ============================================================================
// Skills Hub & Security Quarantine
// ============================================================================

#[derive(Debug, Deserialize, Default)]
pub struct SkillsQuery {
    pub filter: Option<String>,
}

pub async fn hermes_skills_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(q): Query<SkillsQuery>,
) -> Response {
    let now_ms = Utc::now().timestamp_millis();
    let skills: Vec<Value> = state
        .hermes
        .skills()
        .skills_list(q.filter.as_deref())
        .iter()
        .map(|doc| skill_to_dto(doc, now_ms))
        .collect();
    (
        StatusCode::OK,
        Json(json!({ "skills": skills, "total": skills.len() })),
    )
        .into_response()
}

pub async fn hermes_skills_create(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let content = payload
        .get("content")
        .or_else(|| payload.get("body"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let category = payload
        .get("category")
        .or_else(|| payload.get("origin"))
        .and_then(|v| v.as_str())
        .map(SkillOrigin::parse);
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("Custom skill '{}'", name));
    let tags: Option<Vec<String>> = payload
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(ToString::to_string))
                .collect()
        });
    let trusted = payload.get("trusted").and_then(|v| v.as_bool()).unwrap_or(false);

    let res = if content.trim_start().starts_with("---") {
        state
            .hermes
            .skills()
            .skill_upsert_markdown(&content, category, trusted)
    } else {
        state.hermes.skills().skill_create(CreateSkillRequest {
            name,
            description,
            body: content,
            platforms: None,
            toolsets: None,
            tags,
            version: None,
            origin: category,
            trusted: Some(trusted),
        })
    };

    match res {
        Ok(doc) => (
            StatusCode::CREATED,
            Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_skill_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Response {
    match state.hermes.skills().skill_view(&name) {
        Ok(doc) => (
            StatusCode::OK,
            Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_skill_patch(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    if let (Some(old_text), Some(new_text)) = (
        payload.get("old_text").and_then(|v| v.as_str()),
        payload.get("new_text").and_then(|v| v.as_str()),
    ) {
        return match state
            .hermes
            .skills()
            .skill_patch(&name, old_text, new_text)
        {
            Ok(doc) => (
                StatusCode::OK,
                Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
            )
                .into_response(),
            Err(e) => map_telegram_err(e),
        };
    }

    let existing = match state.hermes.skills().skill_view(&name) {
        Ok(doc) => doc,
        Err(e) => return map_telegram_err(e),
    };
    let content = payload
        .get("content")
        .or_else(|| payload.get("body"))
        .and_then(|v| v.as_str())
        .unwrap_or(&existing.raw_markdown)
        .to_string();
    let res = if content.trim_start().starts_with("---") {
        state
            .hermes
            .skills()
            .skill_upsert_markdown(&content, Some(existing.frontmatter.origin), existing.trusted)
    } else {
        let description = payload
            .get("description")
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap_or(existing.frontmatter.description);
        let tags = payload
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or(existing.frontmatter.tags);
        state.hermes.skills().skill_create(CreateSkillRequest {
            name: existing.frontmatter.name,
            description,
            body: content,
            platforms: Some(existing.frontmatter.platforms),
            toolsets: Some(existing.frontmatter.toolsets),
            tags: Some(tags),
            version: Some(existing.frontmatter.version),
            origin: Some(existing.frontmatter.origin),
            trusted: Some(existing.trusted),
        })
    };

    match res {
        Ok(doc) => (
            StatusCode::OK,
            Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_skill_delete(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Response {
    match state.hermes.skills().skill_delete(&name) {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({ "deleted": true, "name": name })),
        )
            .into_response(),
        Ok(false) => err_resp(StatusCode::NOT_FOUND, "skill_not_found", "Skill not found"),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_skill_approve(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    payload: Option<Json<Value>>,
) -> Response {
    let allow_high_risk = payload
        .as_ref()
        .and_then(|Json(v)| v.get("allow_high_risk").and_then(|b| b.as_bool()))
        .unwrap_or(true);

    let pending = state.hermes.skills().list_pending_skill_writes();
    if let Some(proposal) = pending
        .into_iter()
        .find(|p| p.id == name || p.skill_name.eq_ignore_ascii_case(&name))
    {
        return match state
            .hermes
            .skills()
            .approve_skill_write(&proposal.id, allow_high_risk)
        {
            Ok(doc) => (
                StatusCode::OK,
                Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
            )
                .into_response(),
            Err(e) => map_telegram_err(e),
        };
    }

    match state.hermes.skills().skill_view(&name) {
        Ok(existing) => {
            match state.hermes.skills().skill_create(CreateSkillRequest {
                name: existing.frontmatter.name,
                description: existing.frontmatter.description,
                body: existing.body,
                platforms: Some(existing.frontmatter.platforms),
                toolsets: Some(existing.frontmatter.toolsets),
                tags: Some(existing.frontmatter.tags),
                version: Some(existing.frontmatter.version),
                origin: Some(existing.frontmatter.origin),
                trusted: Some(true),
            }) {
                Ok(doc) => (
                    StatusCode::OK,
                    Json(skill_to_dto(&doc, Utc::now().timestamp_millis())),
                )
                    .into_response(),
                Err(e) => map_telegram_err(e),
            }
        }
        Err(e) => map_telegram_err(e),
    }
}

// ============================================================================
// Toolsets & Security Approvals
// ============================================================================

pub async fn hermes_toolsets_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let items = toolsets_list_json(&state);
    (
        StatusCode::OK,
        Json(json!({ "toolsets": items, "total": items.len() })),
    )
        .into_response()
}

pub async fn hermes_toolsets_put(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    {
        let mut reg = state
            .hermes
            .toolsets()
            .write()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(preset_name) = payload.get("preset").and_then(|v| v.as_str()) {
            if let Some(preset) = ToolsetPreset::from_str(preset_name) {
                reg.apply_preset(preset);
            } else {
                return err_resp(
                    StatusCode::BAD_REQUEST,
                    "invalid_preset",
                    &format!("Unknown preset '{preset_name}'"),
                );
            }
        } else if let Some(arr) = payload.get("enabled_toolsets").and_then(|v| v.as_array()) {
            for id in ToolsetId::ALL {
                if id != ToolsetId::Core {
                    reg.disable(id);
                }
            }
            for item in arr {
                if let Some(name) = item.as_str() {
                    if let Err(reason) = reg.enable_by_name(name) {
                        return err_resp(StatusCode::BAD_REQUEST, "invalid_toolset", &reason);
                    }
                }
            }
        } else if let Some(name) = payload.get("toolset").and_then(|v| v.as_str()) {
            let enable = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
            let res = if enable {
                reg.enable_by_name(name).map(|_| ())
            } else {
                reg.disable_by_name(name).map(|_| ())
            };
            if let Err(reason) = res {
                return err_resp(StatusCode::BAD_REQUEST, "invalid_toolset", &reason);
            }
        }
    }

    let items = toolsets_list_json(&state);
    (
        StatusCode::OK,
        Json(json!({ "toolsets": items, "total": items.len() })),
    )
        .into_response()
}

pub async fn hermes_approvals_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let mut seen = HashSet::new();
    let mut items = Vec::new();

    if let Ok(sec) = state.hermes.security().read() {
        for p in sec
            .approvals
            .pending
            .iter()
            .chain(sec.approvals.resolved.iter())
        {
            if seen.insert(p.id.clone()) {
                items.push(json!({
                    "approval_id": p.id,
                    "session_id": p.session_id,
                    "tool_name": p.tool_name,
                    "command_preview": p.command,
                    "risk_level": p.risk_level,
                    "reason": p.reason,
                    "status": sec_approval_status_str(p.status),
                    "created_at_ms": p.created_at.timestamp_millis(),
                    "decided_at_ms": p.decided_at.map(|d| d.timestamp_millis()),
                }));
            }
        }
    }

    if let Ok(rows) = state.hermes.store().list_approvals(None, false) {
        for row in rows {
            if seen.insert(row.id.clone()) {
                items.push(db_approval_to_dto(&row));
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({ "approvals": items, "total": items.len() })),
    )
        .into_response()
}

pub async fn hermes_approval_decide(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(approval_id): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let raw_dec = payload
        .get("decision")
        .or_else(|| payload.get("action"))
        .and_then(|v| v.as_str())
        .unwrap_or("approve_once")
        .trim()
        .to_ascii_lowercase();
    let actor = payload
        .get("reason")
        .and_then(|v| v.as_str())
        .map(|r| format!("user_{}:{r}", auth.user_id.0))
        .unwrap_or_else(|| format!("user_{}", auth.user_id.0));

    let (mem_action, db_status) = match raw_dec.as_str() {
        "approve" | "approve_once" | "once" => {
            (ApprovalDecisionAction::ApproveOnce, ApprovalStatus::ApprovedOnce)
        }
        "approve_session" | "session" => (
            ApprovalDecisionAction::ApproveSession,
            ApprovalStatus::ApprovedSession,
        ),
        "approve_permanent" | "permanent" => (
            ApprovalDecisionAction::ApprovePermanent,
            ApprovalStatus::ApprovedPermanent,
        ),
        "yolo" | "enable_yolo" => {
            (ApprovalDecisionAction::EnableYolo, ApprovalStatus::ApprovedSession)
        }
        "deny" | "reject" => (ApprovalDecisionAction::Deny, ApprovalStatus::Denied),
        other => {
            return err_resp(
                StatusCode::BAD_REQUEST,
                "invalid_decision",
                &format!("Unsupported approval decision '{other}'"),
            );
        }
    };

    let mem_res = state
        .hermes
        .security()
        .write()
        .ok()
        .and_then(|mut sec| sec.approvals.decide(&approval_id, mem_action, actor.clone()).ok());
    let db_res = state
        .hermes
        .store()
        .decide_approval(&approval_id, db_status, Some(actor.clone()))
        .ok();

    if let Some(p) = mem_res {
        return (
            StatusCode::OK,
            Json(json!({
                "approval_id": p.id,
                "session_id": p.session_id,
                "tool_name": p.tool_name,
                "command_preview": p.command,
                "risk_level": p.risk_level,
                "reason": p.reason,
                "status": sec_approval_status_str(p.status),
                "created_at_ms": p.created_at.timestamp_millis(),
                "decided_at_ms": p.decided_at.map(|d| d.timestamp_millis()),
            })),
        )
            .into_response();
    }

    if let Some(rec) = db_res {
        return (StatusCode::OK, Json(db_approval_to_dto(&rec))).into_response();
    }

    err_resp(
        StatusCode::NOT_FOUND,
        "approval_not_found",
        "Pending approval request not found",
    )
}

// ============================================================================
// Cron & Heartbeat
// ============================================================================

pub async fn hermes_cron_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    match state.hermes.store().list_cron_jobs(false) {
        Ok(list) => {
            let jobs: Vec<Value> = list.iter().map(cron_to_dto).collect();
            (
                StatusCode::OK,
                Json(json!({ "jobs": jobs, "total": jobs.len() })),
            )
                .into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_cron_create(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let job_id = payload
        .get("job_id")
        .or_else(|| payload.get("id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let schedule = payload
        .get("schedule")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let prompt = payload
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    if let Some(cid) = chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }

    let bot_id = parse_optional_bot_id(payload.get("bot_id"));
    let enabled = payload.get("enabled").and_then(|v| v.as_bool());
    let timezone = payload
        .get("timezone")
        .and_then(|v| v.as_str())
        .map(ToString::to_string);

    let mut routine = CronRoutine::new("cron_check".to_string(), schedule.clone())
        .with_time_zone(timezone.clone());
    if !schedule.is_empty() && !routine.validate_expression() {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "invalid_schedule",
            &format!("Invalid cron or @every expression '{schedule}'"),
        );
    }
    let next_run_at = payload
        .get("next_run_at")
        .and_then(|v| v.as_i64())
        .or_else(|| routine.compute_next_run(Utc::now()).map(|dt| dt.timestamp_millis()));

    if let Some(ref existing_id) = job_id {
        if let Ok(Some(_)) = state.hermes.store().get_cron_job(existing_id) {
            let patch = CronJobPatch {
                name: (!name.is_empty()).then_some(name),
                schedule: (!schedule.is_empty()).then_some(schedule),
                prompt: (!prompt.is_empty()).then_some(prompt),
                target_chat_id: chat_id.map(Some),
                deliver_to: bot_id.map(Some),
                enabled,
                next_run_at: next_run_at.map(Some),
                ..Default::default()
            };
            return match state.hermes.store().update_cron_job(existing_id, patch) {
                Ok(rec) => (StatusCode::OK, Json(cron_to_dto(&rec))).into_response(),
                Err(e) => map_telegram_err(e),
            };
        }
    }

    match state.hermes.store().create_cron_job(CreateCronJobRequest {
        id: job_id,
        name,
        schedule,
        timezone,
        target_chat_id: chat_id,
        prompt,
        model: payload
            .get("model")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        skills: None,
        enabled,
        deliver_to: bot_id,
        origin_chat_id: chat_id,
        next_run_at,
    }) {
        Ok(rec) => (StatusCode::CREATED, Json(cron_to_dto(&rec))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_cron_patch(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let schedule = payload
        .get("schedule")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let next_run_at = if let Some(ref sched) = schedule {
        let mut routine = CronRoutine::new(job_id.clone(), sched.clone());
        if !routine.validate_expression() {
            return err_resp(
                StatusCode::BAD_REQUEST,
                "invalid_schedule",
                &format!("Invalid cron or @every expression '{sched}'"),
            );
        }
        Some(routine.compute_next_run(Utc::now()).map(|dt| dt.timestamp_millis()))
    } else {
        None
    };

    let patch = CronJobPatch {
        name: payload
            .get("name")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        schedule,
        prompt: payload
            .get("prompt")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        target_chat_id: payload.get("chat_id").map(|v| v.as_i64()),
        deliver_to: payload.get("bot_id").map(|v| parse_optional_bot_id(Some(v))),
        enabled: payload.get("enabled").and_then(|v| v.as_bool()),
        next_run_at,
        ..Default::default()
    };

    match state.hermes.store().update_cron_job(&job_id, patch) {
        Ok(rec) => (StatusCode::OK, Json(cron_to_dto(&rec))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_cron_delete(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Response {
    match state.hermes.store().delete_cron_job(&job_id) {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({ "deleted": true, "job_id": job_id })),
        )
            .into_response(),
        Ok(false) => err_resp(StatusCode::NOT_FOUND, "cron_not_found", "Cron job not found"),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_cron_run(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Response {
    let deliver_to = state
        .hermes
        .store()
        .get_cron_job(&job_id)
        .ok()
        .flatten()
        .and_then(|j| j.deliver_to);

    match state.hermes.run_cron_job_now(&job_id).await {
        Ok(outcome) => {
            if outcome.status == "completed" {
                if let Some(cid) = outcome.target_chat_id {
                    let _ = deliver_hermes_proactive_message(
                        &state,
                        ChatId(cid),
                        deliver_to.as_deref(),
                        &outcome.response_text,
                        None,
                    )
                    .await;
                }
            }
            (StatusCode::OK, Json(json!(outcome))).into_response()
        }
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_heartbeat_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let cfg = state.hermes.heartbeat().status();
    let bot_id = state
        .hermes
        .store()
        .kv_get("heartbeat", "bot_id")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u64>().ok());
    (
        StatusCode::OK,
        Json(json!({
            "enabled": cfg.enabled,
            "interval_secs": cfg.interval_secs,
            "chat_id": cfg.target_chat_id,
            "bot_id": bot_id,
            "prompt": cfg.prompt,
            "last_beat_ms": cfg.last_tick_ms,
        })),
    )
        .into_response()
}

pub async fn hermes_heartbeat_post(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let mut cfg: HeartbeatConfig = state.hermes.heartbeat().status();
    if let Some(enabled) = payload.get("enabled").and_then(|v| v.as_bool()) {
        cfg.enabled = enabled;
    }
    if let Some(secs) = payload.get("interval_secs").and_then(|v| v.as_u64()) {
        cfg.interval_secs = secs.max(5);
    }
    if let Some(chat_val) = payload.get("chat_id").or_else(|| payload.get("target_chat_id")) {
        if chat_val.is_null() {
            cfg.target_chat_id = None;
        } else if let Some(cid) = chat_val.as_i64() {
            if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
                return resp;
            }
            cfg.target_chat_id = Some(cid);
        }
    }
    if let Some(prompt) = payload.get("prompt").and_then(|v| v.as_str()) {
        if !prompt.trim().is_empty() {
            cfg.prompt = prompt.to_string();
        }
    }
    if let Some(bot_id_str) = parse_optional_bot_id(payload.get("bot_id")) {
        let _ = state.hermes.store().kv_set("heartbeat", "bot_id", &bot_id_str);
    }

    state.hermes.heartbeat().set_config(cfg.clone());
    let bot_id = state
        .hermes
        .store()
        .kv_get("heartbeat", "bot_id")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u64>().ok());

    (
        StatusCode::OK,
        Json(json!({
            "enabled": cfg.enabled,
            "interval_secs": cfg.interval_secs,
            "chat_id": cfg.target_chat_id,
            "bot_id": bot_id,
            "prompt": cfg.prompt,
            "last_beat_ms": cfg.last_tick_ms,
        })),
    )
        .into_response()
}

// ============================================================================
// Kanban Boards & Cards
// ============================================================================

pub async fn hermes_kanban_boards_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let boards = match state.hermes.store().list_kanban_boards() {
        Ok(b) => b,
        Err(e) => return map_telegram_err(e),
    };
    let dtos: Vec<Value> = boards
        .iter()
        .map(|board| {
            let cards = state
                .hermes
                .store()
                .list_kanban_cards(&board.id, None)
                .unwrap_or_default();
            board_to_dto(board, &cards)
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({ "boards": dtos, "total": dtos.len() })),
    )
        .into_response()
}

pub async fn hermes_kanban_boards_create(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let columns: Option<Vec<String>> = payload
        .get("columns")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(ToString::to_string))
                .collect()
        });
    match state
        .hermes
        .store()
        .create_kanban_board(name, description, columns)
    {
        Ok(board) => (StatusCode::CREATED, Json(board_to_dto(&board, &[]))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_kanban_board_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(board_id): Path<String>,
) -> Response {
    match state.hermes.store().get_kanban_board(&board_id) {
        Ok(Some(board)) => {
            let cards = state
                .hermes
                .store()
                .list_kanban_cards(&board.id, None)
                .unwrap_or_default();
            (StatusCode::OK, Json(board_to_dto(&board, &cards))).into_response()
        }
        Ok(None) => err_resp(StatusCode::NOT_FOUND, "board_not_found", "Kanban board not found"),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_kanban_cards_create(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let board_id = payload
        .get("board_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let req = CreateKanbanCardRequest {
        id: None,
        board_id,
        column_name: payload
            .get("column")
            .or_else(|| payload.get("column_name"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        title,
        description: payload
            .get("description")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        priority: payload
            .get("priority")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        assignee: payload
            .get("assigned_agent")
            .or_else(|| payload.get("assignee"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        labels: None,
    };
    match state.hermes.store().create_kanban_card(req) {
        Ok(card) => (StatusCode::CREATED, Json(card_to_dto(&card))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_kanban_cards_patch(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(card_id): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let patch = KanbanCardPatch {
        title: payload
            .get("title")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        description: payload
            .get("description")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        priority: payload
            .get("priority")
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        assignee: payload
            .get("assigned_agent")
            .or_else(|| payload.get("assignee"))
            .map(|v| v.as_str().map(ToString::to_string)),
        column_name: payload
            .get("column")
            .or_else(|| payload.get("column_name"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string),
        position: payload.get("position").and_then(|v| v.as_i64()),
        ..Default::default()
    };
    match state.hermes.store().update_kanban_card(&card_id, patch) {
        Ok(card) => (StatusCode::OK, Json(card_to_dto(&card))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn hermes_kanban_cards_delete(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(card_id): Path<String>,
) -> Response {
    match state.hermes.store().delete_kanban_card(&card_id) {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({ "deleted": true, "card_id": card_id })),
        )
            .into_response(),
        Ok(false) => err_resp(StatusCode::NOT_FOUND, "card_not_found", "Kanban card not found"),
        Err(e) => map_telegram_err(e),
    }
}

// ============================================================================
// Providers, Usage, File Rollback, Terminal Processes, Talk Turn
// ============================================================================

pub async fn hermes_providers_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let snap = state.hermes.providers().snapshot();
    let mut list: Vec<&ProviderConfig> = snap.providers.values().collect();
    list.sort_by(|a, b| a.id.cmp(&b.id));
    let providers: Vec<Value> = list
        .into_iter()
        .map(|cfg| provider_to_dto(cfg, &snap.active_provider_id, &snap.model_profiles))
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "providers": providers,
            "active_provider": snap.active_provider_id,
            "active_model": snap.active_model,
        })),
    )
        .into_response()
}

pub async fn hermes_providers_put(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let provider_id = payload
        .get("provider_id")
        .or_else(|| payload.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if provider_id.is_empty() {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "invalid_provider_id",
            "provider_id is required",
        );
    }

    let snap_before = state.hermes.providers().snapshot();
    let mut cfg = snap_before
        .providers
        .get(&provider_id)
        .cloned()
        .unwrap_or_else(|| {
            ProviderConfig::openai_compatible(
                provider_id.clone(),
                payload
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&provider_id),
                payload
                    .get("base_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("https://api.openai.com/v1"),
                payload
                    .get("default_model")
                    .and_then(|v| v.as_str())
                    .unwrap_or("gpt-4o-mini"),
                "",
            )
        });

    if let Some(name) = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        cfg.name = name.to_string();
    }
    if let Some(raw_mode) = payload.get("api_mode").and_then(|v| v.as_str()) {
        if let Some(mode) = ApiMode::parse(raw_mode) {
            cfg.api_mode = mode;
        }
    }
    if let Some(base_url) = payload
        .get("base_url")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        cfg.base_url = base_url.to_string();
    }
    if let Some(default_model) = payload
        .get("default_model")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        cfg.default_model = default_model.to_string();
    }
    if let Some(enabled) = payload.get("enabled").and_then(|v| v.as_bool()) {
        cfg.enabled = enabled;
    }

    state.hermes.providers().register_provider(cfg);
    if let Some(api_key) = payload
        .get("api_key")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let _ = state.hermes.providers().add_credential(
            &provider_id,
            CredentialEntry::new(
                format!("{provider_id}_api_key"),
                api_key.to_string(),
                "gateway",
            ),
        );
    }

    if payload
        .get("set_active")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        let model_override = payload
            .get("default_model")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty());
        if let Err(err) = state.hermes.providers().set_active(&provider_id, model_override) {
            return err_resp(StatusCode::BAD_REQUEST, "provider_activation_error", &err.to_string());
        }
    }

    let snap_after = state.hermes.providers().snapshot();
    match snap_after.providers.get(&provider_id) {
        Some(updated) => (
            StatusCode::OK,
            Json(provider_to_dto(
                updated,
                &snap_after.active_provider_id,
                &snap_after.model_profiles,
            )),
        )
            .into_response(),
        None => err_resp(StatusCode::NOT_FOUND, "provider_not_found", "Provider not found"),
    }
}

pub async fn hermes_usage_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let summary = match state.hermes.store().usage_summary(None, None) {
        Ok(s) => s,
        Err(e) => return map_telegram_err(e),
    };
    let tool_calls: u64 = state
        .hermes
        .store()
        .insights(None, None)
        .map(|i| i.top_tools.iter().map(|t| t.call_count.max(0) as u64).sum())
        .unwrap_or(0);

    (
        StatusCode::OK,
        Json(json!({
            "total_turns": summary.total_messages.max(0) as u64,
            "prompt_tokens": summary.total_input_tokens.max(0) as u64,
            "completion_tokens": summary.total_output_tokens.max(0) as u64,
            "total_tokens": summary.total_tokens.max(0) as u64,
            "estimated_cost_usd": summary.total_cost_usd,
            "tool_calls": tool_calls,
            "by_model": summary.by_model,
        })),
    )
        .into_response()
}

pub async fn hermes_rollback_post(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if session_id.is_empty() {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "invalid_session_id",
            "session_id is required",
        );
    }
    let turn_index = payload
        .get("turn_index")
        .and_then(|v| v.as_u64())
        .or_else(|| {
            payload
                .get("turn_id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<u64>().ok())
        });

    match state.hermes.rollback_turn(&session_id, turn_index) {
        Ok(report) => {
            let mut restored_files = report.restored_files;
            restored_files.extend(report.deleted_files.iter().map(|f| format!("deleted:{f}")));
            (
                StatusCode::OK,
                Json(json!({
                    "rolled_back": true,
                    "session_id": report.session_id,
                    "restored_files": restored_files,
                    "message": format!("Rolled back turn {}", report.turn_index),
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::OK,
            Json(json!({
                "rolled_back": false,
                "session_id": session_id,
                "restored_files": [],
                "message": e.to_string(),
            })),
        )
            .into_response(),
    }
}

pub async fn hermes_terminal_processes_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let list = state.hermes.processes().list().await;
    let processes: Vec<Value> = list
        .into_iter()
        .map(|p| {
            json!({
                "process_id": p.process_id,
                "session_id": Value::Null,
                "command": p.command,
                "status": if p.running { "running" } else { "exited" },
                "pid": p.pid,
                "exit_code": p.exit_code,
                "started_at_ms": p.started_at.timestamp_millis(),
                "output_tail": format!("stdout: {} bytes, stderr: {} bytes", p.stdout_bytes, p.stderr_bytes),
            })
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({ "processes": processes, "total": processes.len() })),
    )
        .into_response()
}

pub async fn hermes_talk_turn(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let transcript = payload
        .get("transcript")
        .or_else(|| payload.get("prompt"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if transcript.is_empty() {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "invalid_transcript",
            "transcript is required",
        );
    }

    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    if let Some(cid) = chat_id {
        if let Err(resp) = check_active_membership(&state, ChatId(cid), auth.user_id) {
            return resp;
        }
    }

    let bot_id_u64 = payload.get("bot_id").and_then(|v| v.as_u64());
    let barge_in = payload
        .get("barge_in")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let voice_preset = payload
        .get("voice_preset")
        .and_then(|v| v.as_str())
        .map(ToString::to_string);

    let session_id = {
        let mut coord = state.talk_coordinator.write();
        let sid = payload
            .get("session_id")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| {
                if coord.session_id.is_empty() {
                    format!("talk_{}", Uuid::new_v4().simple())
                } else {
                    coord.session_id.clone()
                }
            });
        coord.session_id = sid.clone();
        if barge_in {
            let _ = coord.handle_barge_in();
        }
        coord.state = gpui_telegram_agent::TalkSessionState::Thinking;
        coord.signal_seq = coord.signal_seq.saturating_add(1);
        sid
    };

    let (bot_sp, bot_model, _) = bot_id_u64
        .map(|bid| load_bot_persona_config(&state, UserId(bid)))
        .unwrap_or((None, None, None));

    let voice_sys = format!(
        "{} Keep responses concise and natural for real-time voice playback{}.",
        bot_sp.unwrap_or_else(|| "You are Hermes Voice Assistant.".to_string()),
        voice_preset
            .as_deref()
            .map(|v| format!(" (voice preset: {v})"))
            .unwrap_or_default()
    );

    let req = HermesTurnRequest {
        session_id: Some(format!("hermes_{session_id}")),
        chat_id,
        bot_id: bot_id_u64.map(|id| id.to_string()),
        user_id: Some(auth.user_id.0.to_string()),
        source: "talk_mode".to_string(),
        prompt: transcript.clone(),
        model: bot_model,
        system_prompt: Some(voice_sys),
        ..Default::default()
    };

    let res = match state.hermes.run_turn(req).await {
        Ok(r) => r,
        Err(e) => return map_telegram_err(e),
    };

    let tts_segments = split_spoken_segments(&res.response_text);
    let now_ms = Utc::now().timestamp_millis();
    {
        let mut coord = state.talk_coordinator.write();
        coord.state = gpui_telegram_agent::TalkSessionState::Speaking;
        coord.signal_seq = coord.signal_seq.saturating_add(1);
    }
    let _ = state.db.upsert_talk_session(
        &session_id,
        auth.user_id.0,
        "speaking",
        &json!([
            { "role": "user", "text": transcript, "ts": now_ms },
            { "role": "assistant", "text": res.response_text, "ts": Utc::now().timestamp_millis() }
        ]),
        now_ms,
    );

    (
        StatusCode::OK,
        Json(json!({
            "session_id": session_id,
            "call_id": Value::Null,
            "state": "speaking",
            "transcript": transcript,
            "reply_text": res.response_text,
            "tts_segments": tts_segments,
            "bot_id": bot_id_u64,
            "chat_id": chat_id,
        })),
    )
        .into_response()
}

// ============================================================================
// OpenAI-Compatible API Gateway & ACP JSON-RPC
// ============================================================================

fn resolve_optional_bearer_user(headers: &HeaderMap, state: &AppState) -> Option<UserId> {
    let auth_hdr = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = auth_hdr
        .strip_prefix("Bearer ")
        .or_else(|| auth_hdr.strip_prefix("bearer "))?
        .trim();
    if token.is_empty() {
        return None;
    }
    if let Some(session) = state.session_manager.verify_token(token) {
        return Some(session.user_id);
    }
    let session_id = token.split('.').next().unwrap_or(token);
    let db_sess = state.db.get_session(session_id).ok().flatten()?;
    if db_sess.revoked_at_ms.is_some()
        || db_sess.expires_at_ms <= Utc::now().timestamp_millis()
    {
        return None;
    }
    let created_at =
        chrono::DateTime::from_timestamp_millis(db_sess.created_at_ms).unwrap_or_else(Utc::now);
    let expires_at =
        chrono::DateTime::from_timestamp_millis(db_sess.expires_at_ms).unwrap_or_else(Utc::now);
    let sess_tok = gpui_telegram_core::auth::SessionToken::new_with_hash(
        db_sess.session_id.clone(),
        db_sess.user_id,
        created_at,
        expires_at,
        &db_sess.device_id,
        db_sess.token_hash,
    );
    state.session_manager.insert(sess_tok);
    state
        .session_manager
        .verify_token(token)
        .map(|s| s.user_id)
}

pub async fn openai_chat_completions(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let caller_uid = resolve_optional_bearer_user(&headers, &state);
    let chat_id = payload.get("chat_id").and_then(|v| v.as_i64());
    if let Some(cid) = chat_id {
        let Some(uid) = caller_uid else {
            return err_resp(
                StatusCode::UNAUTHORIZED,
                "missing_token",
                "Authenticated Bearer token is required when referencing chat_id",
            );
        };
        if let Err(resp) = check_active_membership(&state, ChatId(cid), uid) {
            return resp;
        }
    }

    let messages = payload
        .get("messages")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if messages.is_empty() {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "invalid_messages",
            "messages array must be non-empty",
        );
    }

    let mut sys_parts = Vec::new();
    let mut convo_parts = Vec::new();
    for msg in &messages {
        let role = msg.get("role").and_then(|v| v.as_str()).unwrap_or("user");
        let content = msg.get("content").and_then(|v| v.as_str()).unwrap_or("");
        if role == "system" {
            if !content.trim().is_empty() {
                sys_parts.push(content.trim().to_string());
            }
        } else {
            convo_parts.push((role.to_string(), content.to_string()));
        }
    }
    let prompt = if convo_parts.len() == 1 && convo_parts[0].0 == "user" {
        convo_parts[0].1.clone()
    } else {
        convo_parts
            .iter()
            .map(|(r, c)| format!("{r}: {c}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("openai_{}", Uuid::new_v4().simple()));
    let model = payload
        .get("model")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let stream_flag = payload.get("stream").and_then(|v| v.as_bool()).unwrap_or(false);

    let req = HermesTurnRequest {
        session_id: Some(session_id),
        chat_id,
        bot_id: None,
        user_id: caller_uid.map(|u| u.0.to_string()),
        source: "openai_gateway".to_string(),
        prompt,
        model: model.clone(),
        system_prompt: (!sys_parts.is_empty()).then(|| sys_parts.join("\n\n")),
        thinking_budget_tokens: None,
        preset: None,
        max_iterations: None,
        max_tokens: payload
            .get("max_tokens")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32),
        temperature: payload
            .get("temperature")
            .and_then(|v| v.as_f64())
            .map(|n| n as f32),
        skip_slash_commands: true,
        caller_depth: 0,
    };

    if stream_flag {
        let rx = match state.hermes.run_turn_stream(req).await {
            Ok(rx) => rx,
            Err(e) => return map_telegram_err(e),
        };
        let cmpl_id = format!("chatcmpl-{}", Uuid::new_v4().simple());
        let created = Utc::now().timestamp();
        let default_model = model.unwrap_or_else(|| state.hermes.providers().active_info().1);
        let s = stream::unfold((rx, false, false, cmpl_id, default_model), move |(mut rx, mut finished, mut emitted_text, id, mdl)| async move {
            if finished {
                return None;
            }
            while let Some(ev) = rx.recv().await {
                match ev {
                    HermesEvent::TextDelta { text, .. } => {
                        if !text.is_empty() {
                            emitted_text = true;
                        }
                        let chunk = json!({
                            "id": id,
                            "object": "chat.completion.chunk",
                            "created": created,
                            "model": mdl,
                            "choices": [{
                                "index": 0,
                                "delta": { "content": text },
                                "finish_reason": Value::Null,
                            }]
                        });
                        let payload_str = format!("data: {chunk}\n\n");
                        return Some((Ok::<axum::body::Bytes, Infallible>(axum::body::Bytes::from(payload_str)), (rx, false, emitted_text, id, mdl)));
                    }
                    HermesEvent::Done { response_text, .. } => {
                        let fallback_delta = if !emitted_text && !response_text.is_empty() {
                            json!({ "role": "assistant", "content": response_text })
                        } else {
                            json!({})
                        };
                        let final_chunk = json!({
                            "id": id,
                            "object": "chat.completion.chunk",
                            "created": created,
                            "model": mdl,
                            "choices": [{
                                "index": 0,
                                "delta": fallback_delta,
                                "finish_reason": "stop",
                            }]
                        });
                        let payload_str = format!("data: {final_chunk}\n\ndata: [DONE]\n\n");
                        finished = true;
                        return Some((Ok(axum::body::Bytes::from(payload_str)), (rx, finished, emitted_text, id, mdl)));
                    }
                    HermesEvent::Error { message, .. } => {
                        let err_chunk = json!({ "error": { "message": message } });
                        let payload_str = format!("data: {err_chunk}\n\ndata: [DONE]\n\n");
                        finished = true;
                        return Some((Ok(axum::body::Bytes::from(payload_str)), (rx, finished, emitted_text, id, mdl)));
                    }
                    _ => continue,
                }
            }
            Some((Ok(axum::body::Bytes::from("data: [DONE]\n\n")), (rx, true, emitted_text, id, mdl)))
        })
        .boxed();

        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/event-stream; charset=utf-8")
            .header(header::CACHE_CONTROL, "no-cache")
            .body(Body::from_stream(s))
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    match state.hermes.run_turn(req).await {
        Ok(res) => (
            StatusCode::OK,
            Json(json!({
                "id": format!("chatcmpl-{}", res.turn_id),
                "object": "chat.completion",
                "created": Utc::now().timestamp(),
                "model": res.model,
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": res.response_text,
                    },
                    "finish_reason": "stop",
                }],
                "usage": {
                    "prompt_tokens": res.usage.input_tokens,
                    "completion_tokens": res.usage.output_tokens,
                    "total_tokens": res.usage.total_tokens,
                },
                "hermes_session_id": res.session_id,
            })),
        )
            .into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn openai_models_list(State(state): State<Arc<AppState>>) -> Response {
    let snap = state.hermes.providers().snapshot();
    let mut seen = HashSet::new();
    let mut data = Vec::new();
    let now = Utc::now().timestamp();

    for prof in snap.model_profiles.values() {
        if seen.insert(prof.model.clone()) {
            data.push(json!({
                "id": prof.model,
                "object": "model",
                "created": now,
                "owned_by": prof.provider_id,
            }));
        }
    }
    for p in snap.providers.values() {
        if !p.default_model.trim().is_empty() && seen.insert(p.default_model.clone()) {
            data.push(json!({
                "id": p.default_model,
                "object": "model",
                "created": now,
                "owned_by": p.id,
            }));
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "object": "list",
            "data": data,
        })),
    )
        .into_response()
}

pub async fn hermes_acp_rpc(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(rpc_req): Json<Value>,
) -> Response {
    let resp = state.hermes.acp_json_rpc(rpc_req).await;
    (StatusCode::OK, Json(resp)).into_response()
}
