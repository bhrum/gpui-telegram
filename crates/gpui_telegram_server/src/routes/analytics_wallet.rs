use crate::routes::{check_active_membership, map_telegram_err, parse_json_object};
use crate::search::schema::SearchQueryParams;
use crate::server::{extract_message_content_text, is_welcome_bot_chat, AppState, AuthBearer};
use axum::{body::Bytes, extract::{Path, Query, State}, http::StatusCode, response::{IntoResponse, Json, Response}};
use gpui_telegram_core::types::{ChatId, MsgId, UserId};
use gpui_telegram_proto::contracts::{SearchHitDto, StarGiftRequest};
use gpui_telegram_proto::message::MessageContent;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub async fn search_unified(
    auth: AuthBearer, State(state): State<Arc<AppState>>, Query(params): Query<HashMap<String, String>>,
) -> Response {
    let q = params.get("q").map(|s| s.trim().to_string()).unwrap_or_default();
    let category = params.get("category").or_else(|| params.get("filter")).map(|s| s.trim().to_lowercase()).unwrap_or_else(|| "chats".into());
    let limit = params.get("limit").and_then(|s| s.parse::<usize>().ok()).unwrap_or(50).clamp(1, 100);
    let scoped_chat = match params.get("chat_id").and_then(|s| s.parse::<i64>().ok()).filter(|&c| c > 0) {
        Some(cid) => {
            if let Err(r) = check_active_membership(&state, ChatId(cid), auth.user_id) { return r; }
            Some(ChatId(cid))
        }
        None => None,
    };

    let member_chats: Vec<ChatId> = match scoped_chat {
        Some(cid) => vec![cid],
        None => state.db.list_user_member_chat_ids(auth.user_id).unwrap_or_default(),
    };
    let allowed_set: HashSet<i64> = member_chats.iter().map(|c| c.0).collect();
    let mut hits: Vec<SearchHitDto> = Vec::new();
    let mut results: Vec<Value> = Vec::new();
    let q_lower = q.to_lowercase();

    // 1. Scoped Tantivy full-text search over message bodies
    if !q.is_empty() && matches!(category.as_str(), "all" | "chats" | "messages" | "") {
        let mut sq = SearchQueryParams::new(&q).with_limit(limit * 3);
        if let Some(cid) = scoped_chat { sq = sq.with_chat_id(cid.0.to_string()); }
        if let Ok(t_hits) = state.search_indexer.search_hits(&sq) {
            for h in t_hits {
                if let Ok(cid_num) = h.chat_id.parse::<i64>() {
                    if !allowed_set.contains(&cid_num) { continue; }
                    let mid = h.message_id.parse::<i64>().unwrap_or(0);
                    let sid = h.sender_id.parse::<u64>().unwrap_or(0);
                    let rec = state.db.get_message(ChatId(cid_num), MsgId(mid)).ok().flatten();
                    if rec.as_ref().and_then(|r| r.deleted_at_ms).is_some() { continue; }
                    let text = rec.as_ref().and_then(|r| crate::messaging::pipeline::MessagePipeline::record_to_item(r).ok())
                        .map(|i| extract_message_content_text(&i.content))
                        .unwrap_or_else(|| h.snippet_html.clone());
                    let media_type = rec.and_then(|r| r.media_type);
                    hits.push(SearchHitDto {
                        chat_id: cid_num, message_id: mid, sender_id: sid, date_ms: h.date,
                        text, snippet_html: h.snippet_html, media_type,
                    });
                    if hits.len() >= limit { break; }
                }
            }
        }
    }

    // 2. Categorized chat & media/link/file filter scan over authorized chats
    if let Ok(chats) = state.db.list_user_chats(auth.user_id) {
        for c in chats {
            if !allowed_set.contains(&c.chat_id.0) { continue; }
            let members = state.db.get_chat_members(c.chat_id).unwrap_or_default();
            let is_bot = is_welcome_bot_chat(c.chat_id) || c.chat_type == 4
                || c.username.as_deref().map(|u| u.to_lowercase().ends_with("bot")).unwrap_or(false)
                || (c.chat_type == 0 && members.iter().any(|&m| m != auth.user_id && state.db.get_user(m).ok().flatten().map(|u| u.is_bot).unwrap_or(false)));
            let ctype = if is_bot { "bot" } else if c.chat_type == 0 { "private" } else if c.chat_type == 3 { "channel" } else { "group" };
            let msgs = state.db.list_messages_with_topic(c.chat_id, None, None, 40).unwrap_or_default();
            let preview = msgs.last().and_then(|r| crate::messaging::pipeline::MessagePipeline::record_to_item(r).ok())
                .map(|i| extract_message_content_text(&i.content))
                .unwrap_or_else(|| c.about.clone().unwrap_or_default());

            if matches!(category.as_str(), "chats" | "channels" | "apps" | "bots" | "all" | "") {
                let cat_ok = if category == "apps" || category == "bots" { is_bot } else if category == "channels" { matches!(ctype, "channel" | "group") } else { true };
                let q_ok = q_lower.is_empty() || c.title.to_lowercase().contains(&q_lower) || preview.to_lowercase().contains(&q_lower);
                if cat_ok && q_ok {
                    results.push(json!({
                        "chat_id": c.chat_id.0, "title": c.title, "subtitle": preview,
                        "chat_type": ctype, "is_bot": is_bot, "kind": "chat",
                    }));
                }
            } else {
                for r in &msgs {
                    if let Ok(item) = crate::messaging::pipeline::MessagePipeline::record_to_item(r) {
                        let txt = extract_message_content_text(&item.content);
                        let matched = match category.as_str() {
                            "media" | "photos" | "videos" => matches!(item.content, MessageContent::Media(_) | MessageContent::Photo { .. } | MessageContent::Video { .. }),
                            "files" | "documents" => matches!(item.content, MessageContent::Document { .. } | MessageContent::Media(_)),
                            "music" | "audio" => matches!(item.content, MessageContent::Audio { .. }),
                            "voice" => matches!(item.content, MessageContent::Voice(_) | MessageContent::VideoNote(_)),
                            "links" => txt.contains("http://") || txt.contains("https://"),
                            _ => false,
                        };
                        if matched && (q_lower.is_empty() || txt.to_lowercase().contains(&q_lower)) {
                            hits.push(SearchHitDto { chat_id: c.chat_id.0, message_id: r.message_id.0, sender_id: r.sender_id.0, date_ms: r.date_ms, text: txt.clone(), snippet_html: txt.clone(), media_type: r.media_type.clone() });
                            results.push(json!({ "chat_id": c.chat_id.0, "message_id": r.message_id.0, "title": c.title, "subtitle": txt, "kind": category }));
                        }
                    }
                }
            }
        }
    }

    if results.is_empty() && !hits.is_empty() {
        for h in &hits {
            results.push(json!({ "chat_id": h.chat_id, "message_id": h.message_id, "sender_id": h.sender_id, "text": h.text, "snippet_html": h.snippet_html, "date_ms": h.date_ms, "kind": "message" }));
        }
    }
    let total = hits.len().max(results.len());
    (StatusCode::OK, Json(json!({ "query": q, "category": category, "total": total, "hits": hits, "results": results }))).into_response()
}

pub async fn chat_statistics(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    match state.db.compute_chat_statistics(ChatId(chat_id)) {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn chat_export(auth: AuthBearer, State(state): State<Arc<AppState>>, Path(chat_id): Path<i64>, Query(q): Query<HashMap<String, String>>) -> Response {
    if let Err(r) = check_active_membership(&state, ChatId(chat_id), auth.user_id) { return r; }
    let limit = q.get("limit").and_then(|s| s.parse::<u32>().ok()).unwrap_or(500).clamp(1, 5000);
    match state.db.export_chat_payload(ChatId(chat_id), limit) {
        Ok(payload) => (StatusCode::OK, Json(payload)).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn wallet_balance(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    match state.db.get_wallet_balance(auth.user_id) {
        Ok(b) => (StatusCode::OK, Json(b)).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn wallet_transactions(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    match state.db.list_wallet_transactions(auth.user_id) {
        Ok(txs) => (StatusCode::OK, Json(json!({ "transactions": txs }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn wallet_topup(auth: AuthBearer, State(state): State<Arc<AppState>>, body: Bytes) -> Response {
    let payload = match parse_json_object(&body) { Ok(v) => v, Err(r) => return r };
    let amount = payload.get("amount_stars").or_else(|| payload.get("amount")).and_then(|v| v.as_i64()).unwrap_or(0);
    let desc = payload.get("description").and_then(|v| v.as_str()).unwrap_or("Stars Top-up");
    match state.db.topup_wallet_stars(auth.user_id, amount, desc) {
        Ok((balance, tx)) => (StatusCode::OK, Json(json!({ "balance": balance, "transaction": tx }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}

pub async fn wallet_send_stars(auth: AuthBearer, State(state): State<Arc<AppState>>, Json(req): Json<StarGiftRequest>) -> Response {
    let desc = req.message.as_deref().unwrap_or(req.gift_id.as_deref().unwrap_or("Telegram Stars Gift"));
    match state.db.transfer_wallet_stars(auth.user_id, UserId(req.to_user_id), req.amount_stars, desc) {
        Ok((balance, tx)) => (StatusCode::OK, Json(json!({ "balance": balance, "transaction": tx }))).into_response(),
        Err(e) => map_telegram_err(e),
    }
}
