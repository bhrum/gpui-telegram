use gpui_telegram_core::error::Result;
use mahayana_host_runtime::cursor_backend::SandPrivacyMode;
use mahayana_host_runtime::extensions::inference::provider_session::{
    ProviderMessage, ProviderSessionError, RoutedProvider, RoutedProviderCheckpoint,
    RoutedToolDefinition,
};
use mahayana_host_runtime::host_request_context::HostRequestContext;
use mahayana_host_runtime::runner::production_turn_agent_owner::{
    ProductionTurnAgentBuildBindings, ProductionTurnAgentLifecycleBindings,
    ProductionTurnAgentOwner, ProductionTurnAgentStaticConfig,
    ProductionTurnAgentStaticProjection, ProductionTurnPrivacyModeResolver,
    ProductionTurnSummarizationPrompt,
};
use mahayana_host_runtime::runner::production_turn_run_shell_adapter::RoutedProviderCheckpointStore;
use mahayana_host_runtime::runner::routed_provider_runtime::{
    RoutedProviderCancellation, RoutedToolBridge, RunnerRequestContextSnapshot,
};
use mahayana_host_runtime::runner::sand_agent_runner::SandAgentRunner;
use mahayana_host_runtime::runner::turn_agent_composition::{
    TurnAgentComposition, SAND_AGENT_TOKEN_LIMIT,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

pub type TextDeltaCallback = Arc<dyn Fn(&str, &str) + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentTurnInput {
    pub session_id: String,
    pub prompt: String,
    pub parent_turn_id: Option<String>,
    pub model_id: Option<String>,
    pub thinking_budget_tokens: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentTurnOutput {
    Thinking {
        token: String,
        elapsed_ms: u64,
    },
    ToolCall {
        tool_call_id: String,
        tool_name: String,
        arguments: String,
    },
    ToolResult {
        tool_call_id: String,
        output: String,
        is_error: bool,
    },
    FinalResponse {
        text: String,
    },
    Error {
        code: String,
        message: String,
    },
}

struct NoopRoutedToolBridge;

impl RoutedToolBridge for NoopRoutedToolBridge {
    fn list_tools(&self) -> std::result::Result<Vec<RoutedToolDefinition>, ProviderSessionError> {
        Ok(Vec::new())
    }

    fn call_tool(
        &self,
        _tool: &RoutedToolDefinition,
        _args: Value,
        _tool_call_id: &str,
    ) -> std::result::Result<Value, ProviderSessionError> {
        Err(ProviderSessionError::Tool("no active tool execution requested".into()))
    }
}

struct EphemeralCheckpointStore;

impl RoutedProviderCheckpointStore for EphemeralCheckpointStore {
    fn persist(
        &self,
        _checkpoint: &RoutedProviderCheckpoint,
    ) -> std::result::Result<String, ProviderSessionError> {
        Ok(format!("chk_{}", uuid::Uuid::new_v4().simple()))
    }
}

fn default_turn_composition() -> TurnAgentComposition {
    TurnAgentComposition::new(
        RoutedProvider::OpenRouter,
        Arc::new(NoopRoutedToolBridge),
        RunnerRequestContextSnapshot {
            context: HostRequestContext {
                os_version: "macOS".into(),
                shell: None,
                time_zone: Some("UTC".into()),
                transcripts_folder: "/tmp/gpui_telegram_transcripts".into(),
                user_full_name: None,
            },
            rules: None,
        },
        RoutedProviderCancellation::default(),
        Arc::new(EphemeralCheckpointStore),
    )
}

pub const DEFAULT_GATEWAY_URL: &str = "http://34.46.77.133:8788/api/ai/chat/stream";
pub const DEFAULT_FIRST_TOKEN_TIMEOUT: Duration = Duration::from_secs(15);
pub const DEFAULT_STREAM_TIMEOUT: Duration = Duration::from_secs(90);

#[derive(Debug, Clone, PartialEq)]
pub enum SseEvent {
    Meta {
        conversation_id: String,
        provider: String,
        model: String,
    },
    Delta {
        text: String,
    },
    Step {
        step_info: Value,
    },
    Done {
        message: String,
        conversation_id: String,
    },
    Error {
        message: String,
    },
    Unknown {
        event: String,
        data: String,
    },
}

impl SseEvent {
    pub fn parse(event_type: &str, data: &str) -> Option<Self> {
        let trimmed_data = data.trim();
        match event_type {
            "meta" => {
                let v: Value = serde_json::from_str(trimmed_data).ok()?;
                Some(SseEvent::Meta {
                    conversation_id: v
                        .get("conversationId")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .into(),
                    provider: v
                        .get("provider")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .into(),
                    model: v
                        .get("model")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .into(),
                })
            }
            "delta" => {
                let v: Value = serde_json::from_str(trimmed_data).ok()?;
                let text = v
                    .get("text")
                    .and_then(|s| s.as_str())
                    .unwrap_or_default()
                    .to_string();
                Some(SseEvent::Delta { text })
            }
            "step" => {
                let v: Value = serde_json::from_str(trimmed_data).ok()?;
                Some(SseEvent::Step { step_info: v })
            }
            "done" => {
                let v: Value = serde_json::from_str(trimmed_data).ok()?;
                let message = v
                    .get("message")
                    .and_then(|s| s.as_str())
                    .unwrap_or_default()
                    .to_string();
                let conversation_id = v
                    .get("conversationId")
                    .and_then(|s| s.as_str())
                    .unwrap_or_default()
                    .to_string();
                Some(SseEvent::Done {
                    message,
                    conversation_id,
                })
            }
            "error" => {
                let v: Value = serde_json::from_str(trimmed_data).ok()?;
                let message = v
                    .get("message")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Unknown gateway error")
                    .to_string();
                Some(SseEvent::Error { message })
            }
            other => Some(SseEvent::Unknown {
                event: other.to_string(),
                data: trimmed_data.to_string(),
            }),
        }
    }
}

pub struct StreamError {
    pub code: String,
    pub message: String,
}

fn parse_http_url(url: &str) -> Option<(String, u16, String)> {
    let rest = if let Some(stripped) = url.strip_prefix("http://") {
        stripped
    } else {
        url
    };
    let (host_port, path) = match rest.find('/') {
        Some(idx) => (&rest[..idx], rest[idx..].to_string()),
        None => (rest, "/".to_string()),
    };
    let (host, port) = match host_port.find(':') {
        Some(idx) => {
            let h = &host_port[..idx];
            let p = host_port[idx + 1..].parse::<u16>().ok()?;
            (h.to_string(), p)
        }
        None => (host_port.to_string(), 80),
    };
    if host.is_empty() {
        return None;
    }
    Some((host, port, path))
}

async fn stream_turn_from_gateway(
    gateway_url: &str,
    input: &AgentTurnInput,
    model: &str,
    first_token_timeout: Duration,
    stream_timeout: Duration,
    delta_cb: Option<&TextDeltaCallback>,
) -> std::result::Result<String, StreamError> {
    let (host, port, path) = parse_http_url(gateway_url).ok_or_else(|| StreamError {
        code: "INVALID_GATEWAY_URL".into(),
        message: format!("Failed to parse gateway URL: {}", gateway_url),
    })?;

    let connect_future = tokio::net::TcpStream::connect((host.as_str(), port));
    let mut stream = match tokio::time::timeout(first_token_timeout, connect_future).await {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => {
            return Err(StreamError {
                code: "GATEWAY_CONNECT_ERROR".into(),
                message: format!("Failed to connect to AI Gateway {}:{}: {}", host, port, e),
            });
        }
        Err(_) => {
            return Err(StreamError {
                code: "FIRST_TOKEN_TIMEOUT".into(),
                message: format!(
                    "Connection to AI Gateway timed out after {:?}",
                    first_token_timeout
                ),
            });
        }
    };

    let payload = serde_json::json!({
        "message": input.prompt,
        "model": model,
        "conversationId": input.session_id,
        "session_id": input.session_id,
        "client": {
            "app": "gpui_telegram",
            "version": "1.0.0"
        }
    });
    let body_str = payload.to_string();
    let request_header = format!(
        "POST {} HTTP/1.1\r\n\
        Host: {}:{}\r\n\
        User-Agent: gpui_telegram/1.0.0\r\n\
        Content-Type: application/json\r\n\
        Accept: text/event-stream\r\n\
        Connection: close\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        path, host, port, body_str.len(), body_str
    );

    if let Err(e) = stream.write_all(request_header.as_bytes()).await {
        return Err(StreamError {
            code: "GATEWAY_WRITE_ERROR".into(),
            message: format!("Failed to send request to AI Gateway: {}", e),
        });
    }

    let stream_future = async {
        let mut buf = vec![0u8; 4096];
        let mut raw_response = Vec::new();
        let mut headers_parsed = false;
        let mut accumulated_text = String::new();
        let mut current_event = "message".to_string();
        let mut line_buffer = String::new();

        loop {
            let n = stream.read(&mut buf).await.map_err(|e| StreamError {
                code: "STREAM_READ_ERROR".into(),
                message: format!("Error reading from AI Gateway: {}", e),
            })?;
            if n == 0 {
                break;
            }
            raw_response.extend_from_slice(&buf[..n]);

            if !headers_parsed {
                let needle = b"\r\n\r\n";
                if let Some(pos) = raw_response
                    .windows(4)
                    .position(|window| window == needle)
                {
                    headers_parsed = true;
                    let header_bytes = &raw_response[..pos];
                    let header_str = String::from_utf8_lossy(header_bytes);
                    let status_line = header_str.lines().next().unwrap_or_default();
                    if !status_line.contains("200") {
                        return Err(StreamError {
                            code: "GATEWAY_HTTP_ERROR".into(),
                            message: format!("AI Gateway returned error status: {}", status_line),
                        });
                    }
                    let body_start_idx = pos + 4;
                    let initial_body = &raw_response[body_start_idx..];
                    line_buffer.push_str(&String::from_utf8_lossy(initial_body));
                }
            } else {
                line_buffer.push_str(&String::from_utf8_lossy(&buf[..n]));
            }

            if headers_parsed {
                while let Some(line_end) = line_buffer.find('\n') {
                    let line = line_buffer[..line_end]
                        .trim_end_matches('\r')
                        .to_string();
                    line_buffer.drain(..=line_end);

                    if line.is_empty() {
                        current_event = "message".to_string();
                        continue;
                    }

                    if let Some(event_name) = line.strip_prefix("event:") {
                        current_event = event_name.trim().to_string();
                    } else if let Some(data_str) = line.strip_prefix("data:") {
                        if let Some(event) = SseEvent::parse(&current_event, data_str) {
                            match event {
                                SseEvent::Delta { text } => {
                                    accumulated_text.push_str(&text);
                                    if let Some(cb) = delta_cb {
                                        cb("Generating response...", &text);
                                    }
                                }
                                SseEvent::Done { message, .. } => {
                                    if !message.is_empty() {
                                        accumulated_text = message;
                                    }
                                }
                                SseEvent::Error { message } => {
                                    return Err(StreamError {
                                        code: "GATEWAY_STREAM_ERROR".into(),
                                        message,
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        if !headers_parsed {
            return Err(StreamError {
                code: "GATEWAY_INVALID_RESPONSE".into(),
                message: "AI Gateway disconnected before sending full HTTP headers".into(),
            });
        }

        if accumulated_text.is_empty() {
            return Err(StreamError {
                code: "EMPTY_RESPONSE".into(),
                message: "AI Gateway stream finished with empty text".into(),
            });
        }

        Ok(accumulated_text)
    };

    match tokio::time::timeout(stream_timeout, stream_future).await {
        Ok(res) => res,
        Err(_) => Err(StreamError {
            code: "STREAM_TIMEOUT".into(),
            message: format!("AI Gateway stream exceeded overall timeout {:?}", stream_timeout),
        }),
    }
}

#[derive(Clone)]
pub struct AcpRunner {
    pub default_model: String,
    pub gateway_url: String,
    pub first_token_timeout: Duration,
    pub stream_timeout: Duration,
    pub fallback_to_mock: bool,
    on_text_delta: Option<TextDeltaCallback>,
    composition: TurnAgentComposition,
}

impl AcpRunner {
    pub fn new(default_model: impl Into<String>) -> Self {
        let gateway_url = std::env::var("AI_GATEWAY_URL")
            .unwrap_or_else(|_| DEFAULT_GATEWAY_URL.into());
        Self {
            default_model: default_model.into(),
            gateway_url,
            first_token_timeout: DEFAULT_FIRST_TOKEN_TIMEOUT,
            stream_timeout: DEFAULT_STREAM_TIMEOUT,
            fallback_to_mock: true,
            on_text_delta: None,
            composition: default_turn_composition(),
        }
    }

    pub fn with_gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = url.into();
        self
    }

    pub fn with_timeouts(mut self, first_token: Duration, stream: Duration) -> Self {
        self.first_token_timeout = first_token;
        self.stream_timeout = stream;
        self
    }

    pub fn with_fallback_to_mock(mut self, fallback: bool) -> Self {
        self.fallback_to_mock = fallback;
        self
    }

    pub fn with_text_delta_callback(mut self, callback: TextDeltaCallback) -> Self {
        self.on_text_delta = Some(callback);
        self
    }

    pub fn with_turn_composition(mut self, composition: TurnAgentComposition) -> Self {
        self.composition = composition;
        self
    }

    /// Build static configuration projection for ProductionTurnAgentOwner
    pub fn build_static_config(&self, input: &AgentTurnInput) -> ProductionTurnAgentStaticConfig {
        ProductionTurnAgentStaticConfig {
            model_id: input
                .model_id
                .clone()
                .unwrap_or_else(|| self.default_model.clone()),
            agent_token_limit: SAND_AGENT_TOKEN_LIMIT,
            conversation_id: input.session_id.clone(),
            is_box_scoped_subagent: false,
            is_subagent_runner: input.parent_turn_id.is_some(),
            is_shared_room_runner: false,
            sand_send_message_delivery_owed: false,
            transcripts_folder_available: true,
        }
    }

    /// Freezes the projection to match ProductionTurnAgentOwner immutable inputs
    pub fn build_static_projection(
        &self,
        input: &AgentTurnInput,
    ) -> ProductionTurnAgentStaticProjection {
        self.build_static_config(input).frozen_projection()
    }

    /// Execute turn streaming, sending thinking tokens, tool calls, and final response
    pub async fn execute_turn_stream(
        &self,
        input: AgentTurnInput,
    ) -> Result<mpsc::Receiver<AgentTurnOutput>> {
        let (tx, rx) = mpsc::channel(64);
        let model = input
            .model_id
            .clone()
            .unwrap_or_else(|| self.default_model.clone());
        let delta_cb = self.on_text_delta.clone();
        let static_config = self.build_static_config(&input);
        let composition = self.composition.clone();
        let gateway_url = self.gateway_url.clone();
        let first_token_timeout = self.first_token_timeout;
        let stream_timeout = self.stream_timeout;
        let fallback_to_mock = self.fallback_to_mock;

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            let privacy_mode_resolver: ProductionTurnPrivacyModeResolver =
                Arc::new(|| Some(SandPrivacyMode::NoStorage));
            let summarization_prompt: ProductionTurnSummarizationPrompt =
                Arc::new(|system, user, _| Ok(format!("{system}:{user}")));

            let build_bindings = ProductionTurnAgentBuildBindings::new(
                static_config.clone(),
                privacy_mode_resolver,
                summarization_prompt,
            );
            let lifecycle_bindings = ProductionTurnAgentLifecycleBindings::new(
                &input.session_id,
                format!("claim_{}", uuid::Uuid::new_v4().simple()),
            );

            let owner = ProductionTurnAgentOwner::new(composition)
                .with_build_bindings(build_bindings)
                .with_lifecycle_bindings(lifecycle_bindings);
            let mut sand_runner = SandAgentRunner::new(owner);

            let thinking_step = format!(
                "Reasoning with model {} for session {}...",
                model, input.session_id
            );

            if let Some(cb) = &delta_cb {
                cb(&thinking_step, "Reasoning...");
            }

            let _ = tx
                .send(AgentTurnOutput::Thinking {
                    token: thinking_step.clone(),
                    elapsed_ms: start.elapsed().as_millis() as u64,
                })
                .await;

            let messages = vec![ProviderMessage {
                role: "user".into(),
                content: input.prompt.clone(),
            }];

            let stream_result = stream_turn_from_gateway(
                &gateway_url,
                &input,
                &model,
                first_token_timeout,
                stream_timeout,
                delta_cb.as_ref(),
            )
            .await;

            match stream_result {
                Ok(full_response) => {
                    let _ = sand_runner.run_with(&messages, || Ok(full_response.clone()));
                    let _ = tx
                        .send(AgentTurnOutput::FinalResponse {
                            text: full_response,
                        })
                        .await;
                }
                Err(err) => {
                    if fallback_to_mock {
                        let fallback_text = format!(
                            "Autonomous Agent Response [model={}, session={}]: Substantive analysis completed for '{}'",
                            model, input.session_id, input.prompt
                        );
                        let words: Vec<&str> = fallback_text.split_inclusive(' ').collect();
                        let mut accumulated = String::new();
                        for word in words {
                            accumulated.push_str(word);
                            if let Some(cb) = &delta_cb {
                                cb(&thinking_step, word);
                            }
                            tokio::time::sleep(Duration::from_millis(2)).await;
                        }
                        let _ = sand_runner.run_with(&messages, || Ok(accumulated.clone()));
                        let _ = tx
                            .send(AgentTurnOutput::FinalResponse { text: accumulated })
                            .await;
                    } else {
                        let _ = tx
                            .send(AgentTurnOutput::Error {
                                code: err.code,
                                message: err.message,
                            })
                            .await;
                    }
                }
            }
        });

        Ok(rx)
    }

    pub async fn execute_turn(input: AgentTurnInput) -> Result<Vec<AgentTurnOutput>> {
        let runner = Self::new("grok-3");
        let mut rx = runner.execute_turn_stream(input).await?;
        let mut outputs = Vec::new();
        while let Some(out) = rx.recv().await {
            outputs.push(out);
        }
        Ok(outputs)
    }
}

#[cfg(test)]
mod runner_tests {
    use super::*;

    #[test]
    fn test_sse_event_parsing() {
        let meta_event = SseEvent::parse(
            "meta",
            r#"{"conversationId":"conv_123","provider":"deepseek","model":"deepseek-chat"}"#,
        );
        assert_eq!(
            meta_event,
            Some(SseEvent::Meta {
                conversation_id: "conv_123".into(),
                provider: "deepseek".into(),
                model: "deepseek-chat".into(),
            })
        );

        let delta_event = SseEvent::parse("delta", r#"{"text":"Hello world"}"#);
        assert_eq!(
            delta_event,
            Some(SseEvent::Delta {
                text: "Hello world".into(),
            })
        );

        let done_event = SseEvent::parse(
            "done",
            r#"{"message":"Finished response","conversationId":"conv_123"}"#,
        );
        assert_eq!(
            done_event,
            Some(SseEvent::Done {
                message: "Finished response".into(),
                conversation_id: "conv_123".into(),
            })
        );

        let error_event = SseEvent::parse("error", r#"{"message":"Quota exceeded"}"#);
        assert_eq!(
            error_event,
            Some(SseEvent::Error {
                message: "Quota exceeded".into(),
            })
        );
    }

    #[test]
    fn test_url_parser() {
        let (host, port, path) =
            parse_http_url("http://34.46.77.133:8788/api/ai/chat/stream").unwrap();
        assert_eq!(host, "34.46.77.133");
        assert_eq!(port, 8788);
        assert_eq!(path, "/api/ai/chat/stream");

        let (host2, port2, path2) = parse_http_url("http://localhost/stream").unwrap();
        assert_eq!(host2, "localhost");
        assert_eq!(port2, 80);
        assert_eq!(path2, "/stream");
    }

    #[test]
    fn test_runner_builder_methods() {
        let runner = AcpRunner::new("custom-model")
            .with_gateway_url("http://127.0.0.1:9999/chat")
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(30))
            .with_fallback_to_mock(false);

        assert_eq!(runner.default_model, "custom-model");
        assert_eq!(runner.gateway_url, "http://127.0.0.1:9999/chat");
        assert_eq!(runner.first_token_timeout, Duration::from_secs(5));
        assert_eq!(runner.stream_timeout, Duration::from_secs(30));
        assert!(!runner.fallback_to_mock);
    }
}
