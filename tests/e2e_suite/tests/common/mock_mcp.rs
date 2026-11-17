#![allow(dead_code)]

use gpui_telegram_core::action_bus::*;
use gpui_telegram_test_harness::tier4_e2e::E2eSystemDriver;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

// --- Mock MCP JSON-RPC 2.0 Client Simulator (delegates to real ActionRegistry::execute) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

pub struct MockMcpClient {
    registry: Arc<ActionRegistry>,
    req_counter: std::sync::atomic::AtomicU64,
}

impl MockMcpClient {
    pub fn new(registry: Arc<ActionRegistry>) -> Self {
        Self {
            registry,
            req_counter: std::sync::atomic::AtomicU64::new(1),
        }
    }

    pub fn initialize(&self, client_name: &str) -> JsonRpcResponse {
        let id = self
            .req_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": { "listChanged": true }
                },
                "serverInfo": {
                    "name": "GPUI Telegram Platform Action Bus MCP Server",
                    "version": "1.2.0"
                },
                "client": client_name
            })),
            error: None,
        }
    }

    pub fn tools_list(&self) -> JsonRpcResponse {
        let id = self
            .req_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let resp = McpExporter::export_tools_list(&self.registry);
        JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id,
            result: Some(json!({ "tools": resp.tools })),
            error: None,
        }
    }

    pub async fn tools_call(&self, name: &str, arguments: Value) -> JsonRpcResponse {
        self.tools_call_with_approval(name, arguments, false).await
    }

    pub async fn tools_call_with_approval(
        &self,
        name: &str,
        arguments: Value,
        approved: bool,
    ) -> JsonRpcResponse {
        let id = self
            .req_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let ctx = ActionContext::new().with_user(42).with_approval(approved);
        match self.registry.execute(name, ctx, arguments).await {
            Ok(res) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(json!({ "content": [{ "type": "text", "text": res.to_string() }] })),
                error: None,
            },
            Err(ActionError::NotFound(msg)) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: msg,
                    data: None,
                }),
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: format!("Action execution error: {e}"),
                    data: None,
                }),
            },
        }
    }
}

// --- Real HTTP Platform IM Driver (executes IM tool calls against live EphemeralTestServer) ---

fn encode_query_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char);
            }
            b' ' => out.push_str("%20"),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}

pub struct HttpPlatformImDriver {
    driver: Arc<E2eSystemDriver>,
    token: String,
}

impl HttpPlatformImDriver {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            driver: Arc::new(E2eSystemDriver::new(base_url)),
            token: token.into(),
        }
    }

    pub fn with_token(&self, token: impl Into<String>) -> Self {
        Self {
            driver: self.driver.clone(),
            token: token.into(),
        }
    }

    async fn call(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value, String> {
        let (status, val) = self
            .driver
            .json_req(method, path, Some(&self.token), body)
            .await
            .map_err(|e| format!("HTTP request failed: {e}"))?;
        if !(200..300).contains(&status) {
            let msg = val["message"]
                .as_str()
                .or_else(|| val["error"].as_str())
                .unwrap_or("request failed");
            return Err(format!("HTTP {status}: {msg}"));
        }
        Ok(val)
    }

    pub async fn execute(&self, tool_name: &str, params: Value) -> Result<Value, String> {
        match tool_name {
            "im_send_message" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
                let mut payload = json!({ "text": text });
                if let Some(reply_to) = params.get("reply_to").and_then(|v| v.as_i64()) {
                    payload["reply_to"] = json!(reply_to);
                }
                if let Some(topic_id) = params.get("topic_id").and_then(|v| v.as_i64()) {
                    payload["topic_id"] = json!(topic_id);
                }
                if let Some(nonce) = params.get("client_nonce").and_then(|v| v.as_u64()) {
                    payload["client_nonce"] = json!(nonce);
                }
                let val = self
                    .call(
                        Method::POST,
                        &format!("/api/v1/chats/{chat_id}/messages"),
                        Some(payload),
                    )
                    .await?;
                let msg_id = val["message"]["message_id"]
                    .as_i64()
                    .or_else(|| val["message_id"].as_i64());
                let msg_obj = val.get("message").cloned().unwrap_or_else(|| val.clone());
                Ok(json!({
                    "success": true,
                    "message_id": msg_id,
                    "pts": val["pts"],
                    "message": msg_obj
                }))
            }
            "im_search_messages" => {
                let query = params
                    .get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mut path = format!(
                    "/api/v1/search?category=messages&q={}",
                    encode_query_component(query)
                );
                if let Some(cid) = params.get("chat_id").and_then(|v| v.as_i64()) {
                    path.push_str(&format!("&chat_id={cid}"));
                }
                self.call(Method::GET, &path, None).await
            }
            "im_edit_message" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let msg_id = params
                    .get("message_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing message_id")?;
                let new_text = params
                    .get("text")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing text")?;
                self.call(
                    Method::PUT,
                    &format!("/api/v1/chats/{chat_id}/messages/{msg_id}"),
                    Some(json!({ "text": new_text })),
                )
                .await
            }
            "im_delete_message" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let msg_id = params
                    .get("message_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing message_id")?;
                self.call(
                    Method::DELETE,
                    &format!("/api/v1/chats/{chat_id}/messages/{msg_id}"),
                    None,
                )
                .await
            }
            "im_set_reaction" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let msg_id = params
                    .get("message_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing message_id")?;
                let emoji = params
                    .get("reaction")
                    .or_else(|| params.get("emoji"))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing reaction")?;
                let action = params
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("toggle");
                self.call(
                    Method::POST,
                    &format!("/api/v1/chats/{chat_id}/messages/{msg_id}/reactions"),
                    Some(json!({ "emoji": emoji, "action": action })),
                )
                .await
            }
            "im_pin_message" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let msg_id = params
                    .get("message_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing message_id")?;
                self.call(
                    Method::POST,
                    &format!("/api/v1/chats/{chat_id}/pins"),
                    Some(json!({ "message_id": msg_id })),
                )
                .await
            }
            "im_create_poll" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let question = params
                    .get("question")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing question")?;
                let options = params
                    .get("options")
                    .and_then(|v| v.as_array())
                    .ok_or("Missing options")?;
                let mut val = self
                    .call(
                        Method::POST,
                        &format!("/api/v1/chats/{chat_id}/polls"),
                        Some(json!({
                            "question": question,
                            "options": options
                        })),
                    )
                    .await?;
                let poll_id = val["poll"]["poll_id"]
                    .as_i64()
                    .or_else(|| val["poll"]["id"].as_str().and_then(|s| s.parse().ok()))
                    .unwrap_or(0);
                val["poll_id"] = json!(poll_id);
                Ok(val)
            }
            "im_vote_poll" => {
                let poll_id = params
                    .get("poll_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing poll_id")?;
                let option_ids: Vec<u8> = if let Some(arr) = params.get("option_ids").and_then(|v| v.as_array()) {
                    arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect()
                } else if let Some(idx) = params.get("option_index").and_then(|v| v.as_u64()) {
                    vec![idx as u8]
                } else {
                    return Err("Missing option_index or option_ids".into());
                };
                self.call(
                    Method::POST,
                    &format!("/api/v1/polls/{poll_id}/vote"),
                    Some(json!({ "option_ids": option_ids })),
                )
                .await
            }
            "im_manage_chat" => {
                let title = params
                    .get("title")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing title")?;
                let chat_type = params
                    .get("chat_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("group");
                let mut body = json!({ "title": title, "chat_type": chat_type });
                if let Some(is_forum) = params.get("is_forum").and_then(|v| v.as_bool()) {
                    body["is_forum"] = json!(is_forum);
                }
                if let Some(members) = params.get("member_ids") {
                    body["member_ids"] = members.clone();
                }
                self.call(Method::POST, "/api/v1/chats", Some(body)).await
            }
            "im_manage_topic" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let title = params
                    .get("title")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing title")?;
                self.call(
                    Method::POST,
                    &format!("/api/v1/chats/{chat_id}/topics"),
                    Some(json!({ "title": title })),
                )
                .await
            }
            "im_manage_members" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let target_user_id = params
                    .get("user_id")
                    .and_then(|v| v.as_u64())
                    .ok_or("Missing user_id")?;
                let action = params
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("add");
                if action == "add" {
                    self.call(
                        Method::POST,
                        &format!("/api/v1/chats/{chat_id}/members"),
                        Some(json!({ "user_id": target_user_id })),
                    )
                    .await
                } else if action == "kick" {
                    self.call(
                        Method::DELETE,
                        &format!("/api/v1/chats/{chat_id}/members/{target_user_id}"),
                        None,
                    )
                    .await
                } else {
                    Err(format!("Unsupported member action: {action}"))
                }
            }
            "im_get_chat_history" => {
                let chat_id = params
                    .get("chat_id")
                    .and_then(|v| v.as_i64())
                    .ok_or("Missing chat_id")?;
                let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50);
                self.call(
                    Method::GET,
                    &format!("/api/v1/chats/{chat_id}/messages?limit={limit}"),
                    None,
                )
                .await
            }
            "im_miniapp_list" => self.call(Method::GET, "/api/v1/miniapps/installed", None).await,
            "im_miniapp_invoke" => {
                let bot = params
                    .get("bot_username")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing bot_username")?;
                let short = params
                    .get("short_name")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing short_name")?;
                let action_type = params
                    .get("action_type")
                    .or_else(|| params.get("action"))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing action_type")?;
                let mut body = json!({ "action_type": action_type });
                if let Some(wid) = params.get("widget_id") {
                    body["widget_id"] = wid.clone();
                }
                if let Some(val) = params.get("value") {
                    body["value"] = val.clone();
                }
                if let Some(data) = params.get("data") {
                    body["data"] = data.clone();
                }
                if let Some(cid) = params.get("chat_id") {
                    body["chat_id"] = cid.clone();
                }
                if let Some(qid) = params.get("query_id") {
                    body["query_id"] = qid.clone();
                }
                self.call(
                    Method::POST,
                    &format!("/api/v1/miniapps/{bot}/{short}/action"),
                    Some(body),
                )
                .await
            }
            unknown => Err(format!("Unknown native IM tool: {unknown}")),
        }
    }
}
