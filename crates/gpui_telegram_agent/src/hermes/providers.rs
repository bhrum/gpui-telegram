use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task::JoinSet;

/// Wire protocol modes supported natively by the Hermes multi-provider runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiMode {
    ChatCompletions,
    CodexResponses,
    AnthropicMessages,
    LocalHermes,
}

impl Default for ApiMode {
    fn default() -> Self {
        Self::LocalHermes
    }
}

impl ApiMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChatCompletions => "chat_completions",
            Self::CodexResponses => "codex_responses",
            Self::AnthropicMessages => "anthropic_messages",
            Self::LocalHermes => "local_hermes",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "chat_completions" | "openai" | "chat" | "completions" => Some(Self::ChatCompletions),
            "codex_responses" | "responses" | "codex" => Some(Self::CodexResponses),
            "anthropic_messages" | "anthropic" | "claude" | "messages" => {
                Some(Self::AnthropicMessages)
            }
            "local_hermes" | "local" | "hermes" | "offline" => Some(Self::LocalHermes),
            _ => None,
        }
    }

    pub fn all() -> &'static [ApiMode] {
        &[
            Self::ChatCompletions,
            Self::CodexResponses,
            Self::AnthropicMessages,
            Self::LocalHermes,
        ]
    }
}

impl fmt::Display for ApiMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderMessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: String,
}

impl CacheControl {
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemPromptTier {
    pub label: String,
    pub text: String,
    pub cache_ephemeral: bool,
}

impl SystemPromptTier {
    pub fn stable(label: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            text: text.into(),
            cache_ephemeral: true,
        }
    }

    pub fn volatile(label: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            text: text.into(),
            cache_ephemeral: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

impl ProviderToolCall {
    pub fn arguments_json(&self) -> Value {
        serde_json::from_str(&self.arguments).unwrap_or_else(|_| json!({}))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderChatMessage {
    pub role: ProviderMessageRole,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ProviderToolCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

impl ProviderChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ProviderMessageRole::User,
            content: content.into(),
            name: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
            reasoning: None,
            cache_control: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ProviderMessageRole::Assistant,
            content: content.into(),
            name: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
            reasoning: None,
            cache_control: None,
        }
    }

    pub fn assistant_with_tools(
        content: impl Into<String>,
        tool_calls: Vec<ProviderToolCall>,
        reasoning: Option<String>,
    ) -> Self {
        Self {
            role: ProviderMessageRole::Assistant,
            content: content.into(),
            name: None,
            tool_call_id: None,
            tool_calls,
            reasoning,
            cache_control: None,
        }
    }

    pub fn tool_result(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            role: ProviderMessageRole::Tool,
            content: content.into(),
            name: Some(tool_name.into()),
            tool_call_id: Some(tool_call_id.into()),
            tool_calls: Vec::new(),
            reasoning: None,
            cache_control: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_creation_tokens: u64,
    pub total_tokens: u64,
    pub estimated_cost_usd: f64,
}

impl TokenUsage {
    pub fn accumulate(&mut self, other: &TokenUsage) {
        self.input_tokens = self.input_tokens.saturating_add(other.input_tokens);
        self.output_tokens = self.output_tokens.saturating_add(other.output_tokens);
        self.cache_read_tokens = self.cache_read_tokens.saturating_add(other.cache_read_tokens);
        self.cache_creation_tokens = self
            .cache_creation_tokens
            .saturating_add(other.cache_creation_tokens);
        self.total_tokens = self.total_tokens.saturating_add(other.total_tokens);
        self.estimated_cost_usd += other.estimated_cost_usd;
    }

    pub fn finalize_total(&mut self) {
        if self.total_tokens == 0 {
            self.total_tokens = self.input_tokens.saturating_add(self.output_tokens);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequest {
    pub model: String,
    pub system_tiers: Vec<SystemPromptTier>,
    pub messages: Vec<ProviderChatMessage>,
    #[serde(default)]
    pub tools: Vec<ProviderToolSpec>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub thinking_budget_tokens: Option<usize>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub metadata: Value,
}

impl ProviderRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ProviderChatMessage>) -> Self {
        Self {
            model: model.into(),
            system_tiers: Vec::new(),
            messages,
            tools: Vec::new(),
            temperature: None,
            max_tokens: None,
            thinking_budget_tokens: None,
            stream: true,
            metadata: Value::Null,
        }
    }

    pub fn combined_system_prompt(&self) -> String {
        self.system_tiers
            .iter()
            .map(|t| t.text.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub content: String,
    pub reasoning: Option<String>,
    pub tool_calls: Vec<ProviderToolCall>,
    pub finish_reason: String,
    pub usage: TokenUsage,
    pub provider_id: String,
    pub model: String,
    pub api_mode: ApiMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProviderStreamEvent {
    ThinkingDelta(String),
    TextDelta(String),
    ToolCallReady(ProviderToolCall),
    Usage(TokenUsage),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum ProviderError {
    #[error("Provider configuration error: {0}")]
    Configuration(String),
    #[error("Provider authentication error: {0}")]
    Authentication(String),
    #[error("Provider rate limited (HTTP {status}): {message}")]
    RateLimited {
        status: u16,
        retry_after_secs: Option<u64>,
        message: String,
    },
    #[error("Provider upstream HTTP {status}: {message}")]
    UpstreamHttp { status: u16, message: String },
    #[error("Provider transport error: {0}")]
    Transport(String),
    #[error("Provider protocol error: {0}")]
    Protocol(String),
    #[error("All providers exhausted: {0}")]
    Exhausted(String),
}

impl ProviderError {
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimited { .. } | Self::Transport(_) => true,
            Self::UpstreamHttp { status, .. } => *status >= 500 || *status == 429 || *status == 408,
            _ => false,
        }
    }

    pub fn http_status(&self) -> Option<u16> {
        match self {
            Self::RateLimited { status, .. } | Self::UpstreamHttp { status, .. } => Some(*status),
            Self::Authentication(_) => Some(401),
            _ => None,
        }
    }

    pub fn retry_after_secs(&self) -> Option<u64> {
        match self {
            Self::RateLimited {
                retry_after_secs, ..
            } => *retry_after_secs,
            _ => None,
        }
    }
}

/// Multi-account / multi-key entry inside a provider's CredentialPool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialEntry {
    pub id: String,
    pub api_key: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url_override: Option<String>,
    #[serde(default = "default_weight")]
    pub weight: u32,
    #[serde(default)]
    pub cooldown_until_ms: i64,
    #[serde(default)]
    pub consecutive_failures: u32,
    #[serde(default)]
    pub rate_limited_count: u64,
    #[serde(default)]
    pub total_requests: u64,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_weight() -> u32 {
    1
}

fn default_true() -> bool {
    true
}

impl CredentialEntry {
    pub fn new(id: impl Into<String>, api_key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            api_key: api_key.into(),
            label: label.into(),
            base_url_override: None,
            weight: 1,
            cooldown_until_ms: 0,
            consecutive_failures: 0,
            rate_limited_count: 0,
            total_requests: 0,
            enabled: true,
        }
    }

    pub fn is_available(&self, now_ms: i64) -> bool {
        self.enabled && !self.api_key.trim().is_empty() && now_ms >= self.cooldown_until_ms
    }

    pub fn masked_key(&self) -> String {
        let trimmed = self.api_key.trim();
        if trimmed.len() <= 8 {
            "***".to_string()
        } else {
            format!("{}...{}", &trimmed[..4], &trimmed[trimmed.len() - 4..])
        }
    }
}

/// Thread-safe rotating credential pool with automatic 429/5xx cooldowns.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CredentialPool {
    pub entries: Vec<CredentialEntry>,
    #[serde(default)]
    pub cursor: usize,
    #[serde(default = "default_base_cooldown_ms")]
    pub base_cooldown_ms: i64,
}

fn default_base_cooldown_ms() -> i64 {
    5_000
}

impl CredentialPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
            base_cooldown_ms: default_base_cooldown_ms(),
        }
    }

    pub fn with_single_key(id: impl Into<String>, api_key: impl Into<String>) -> Self {
        let mut pool = Self::new();
        pool.add_entry(CredentialEntry::new(id, api_key, "primary"));
        pool
    }

    pub fn add_entry(&mut self, entry: CredentialEntry) {
        if let Some(existing) = self.entries.iter_mut().find(|e| e.id == entry.id) {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn remove_entry(&mut self, id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn active_count(&self, now_ms: i64) -> usize {
        self.entries.iter().filter(|e| e.is_available(now_ms)).count()
    }

    pub fn cooldown_count(&self, now_ms: i64) -> usize {
        self.entries
            .iter()
            .filter(|e| e.enabled && now_ms < e.cooldown_until_ms)
            .count()
    }

    /// Acquire the next non-cooled-down credential in round-robin order.
    pub fn acquire(&mut self, now_ms: i64) -> Option<CredentialEntry> {
        let len = self.entries.len();
        if len == 0 {
            return None;
        }
        for offset in 0..len {
            let idx = (self.cursor + offset) % len;
            if self.entries[idx].is_available(now_ms) {
                self.entries[idx].total_requests = self.entries[idx].total_requests.saturating_add(1);
                self.cursor = (idx + 1) % len;
                return Some(self.entries[idx].clone());
            }
        }
        None
    }

    pub fn report_success(&mut self, id: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.consecutive_failures = 0;
            entry.cooldown_until_ms = 0;
        }
    }

    pub fn report_rate_limit(
        &mut self,
        id: &str,
        now_ms: i64,
        status_code: u16,
        retry_after_secs: Option<u64>,
    ) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.consecutive_failures = entry.consecutive_failures.saturating_add(1);
            if status_code == 429 {
                entry.rate_limited_count = entry.rate_limited_count.saturating_add(1);
            }
            let cooldown_ms = if let Some(secs) = retry_after_secs {
                (secs as i64).saturating_mul(1_000).max(1_000)
            } else {
                let shift = entry.consecutive_failures.saturating_sub(1).min(8);
                let exp = self.base_cooldown_ms.saturating_mul(1i64 << shift);
                exp.min(900_000)
            };
            entry.cooldown_until_ms = now_ms.saturating_add(cooldown_ms);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub id: String,
    pub provider_id: String,
    pub model: String,
    pub api_mode: ApiMode,
    pub input_cost_per_1m: f64,
    pub output_cost_per_1m: f64,
    pub context_window: usize,
    pub supports_tools: bool,
    pub supports_reasoning: bool,
    pub supports_caching: bool,
}

impl ModelProfile {
    pub fn estimate_cost(&self, input_tokens: u64, output_tokens: u64) -> f64 {
        let in_cost = (input_tokens as f64 / 1_000_000.0) * self.input_cost_per_1m;
        let out_cost = (output_tokens as f64 / 1_000_000.0) * self.output_cost_per_1m;
        in_cost + out_cost
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub api_mode: ApiMode,
    pub base_url: String,
    pub default_model: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub credentials: CredentialPool,
    #[serde(default)]
    pub fallback_models: Vec<String>,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_timeout_secs() -> u64 {
    90
}

impl ProviderConfig {
    pub fn local_hermes() -> Self {
        Self {
            id: "local_hermes".to_string(),
            name: "Local Hermes Runtime".to_string(),
            api_mode: ApiMode::LocalHermes,
            base_url: "local://hermes".to_string(),
            default_model: "hermes-local-v1".to_string(),
            headers: HashMap::new(),
            credentials: CredentialPool::new(),
            fallback_models: Vec::new(),
            timeout_secs: 30,
            enabled: true,
        }
    }

    pub fn openai_compatible(
        id: impl Into<String>,
        name: impl Into<String>,
        base_url: impl Into<String>,
        default_model: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let id_str = id.into();
        let mut pool = CredentialPool::new();
        let key_str = api_key.into();
        if !key_str.trim().is_empty() {
            pool.add_entry(CredentialEntry::new(format!("{id_str}_key_1"), key_str, "primary"));
        }
        Self {
            id: id_str,
            name: name.into(),
            api_mode: ApiMode::ChatCompletions,
            base_url: base_url.into(),
            default_model: default_model.into(),
            headers: HashMap::new(),
            credentials: pool,
            fallback_models: Vec::new(),
            timeout_secs: 90,
            enabled: true,
        }
    }

    pub fn codex_responses(
        id: impl Into<String>,
        name: impl Into<String>,
        base_url: impl Into<String>,
        default_model: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let mut cfg = Self::openai_compatible(id, name, base_url, default_model, api_key);
        cfg.api_mode = ApiMode::CodexResponses;
        cfg
    }

    pub fn anthropic(
        id: impl Into<String>,
        name: impl Into<String>,
        base_url: impl Into<String>,
        default_model: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let mut cfg = Self::openai_compatible(id, name, base_url, default_model, api_key);
        cfg.api_mode = ApiMode::AnthropicMessages;
        cfg
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackTarget {
    pub provider_id: String,
    pub model: String,
}

/// Manages active provider/model, credential rotation, model fallback chain, and HTTP/local execution.
#[derive(Clone)]
pub struct ProviderRouter {
    inner: Arc<Mutex<ProviderRouterState>>,
    http_client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRouterState {
    pub providers: HashMap<String, ProviderConfig>,
    pub model_profiles: HashMap<String, ModelProfile>,
    pub active_provider_id: String,
    pub active_model: String,
    pub fallback_chain: Vec<FallbackTarget>,
    pub allow_local_fallback: bool,
}

impl Default for ProviderRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRouter {
    pub fn new() -> Self {
        let mut providers = HashMap::new();
        let local = ProviderConfig::local_hermes();
        providers.insert(local.id.clone(), local);

        let mut model_profiles = HashMap::new();
        model_profiles.insert(
            "hermes-local-v1".to_string(),
            ModelProfile {
                id: "hermes-local-v1".to_string(),
                provider_id: "local_hermes".to_string(),
                model: "hermes-local-v1".to_string(),
                api_mode: ApiMode::LocalHermes,
                input_cost_per_1m: 0.0,
                output_cost_per_1m: 0.0,
                context_window: 128_000,
                supports_tools: true,
                supports_reasoning: true,
                supports_caching: true,
            },
        );
        model_profiles.insert(
            "claude-sonnet-4-20250514".to_string(),
            ModelProfile {
                id: "claude-sonnet-4-20250514".to_string(),
                provider_id: "anthropic".to_string(),
                model: "claude-sonnet-4-20250514".to_string(),
                api_mode: ApiMode::AnthropicMessages,
                input_cost_per_1m: 3.0,
                output_cost_per_1m: 15.0,
                context_window: 200_000,
                supports_tools: true,
                supports_reasoning: true,
                supports_caching: true,
            },
        );
        model_profiles.insert(
            "gpt-4o".to_string(),
            ModelProfile {
                id: "gpt-4o".to_string(),
                provider_id: "openai".to_string(),
                model: "gpt-4o".to_string(),
                api_mode: ApiMode::ChatCompletions,
                input_cost_per_1m: 2.5,
                output_cost_per_1m: 10.0,
                context_window: 128_000,
                supports_tools: true,
                supports_reasoning: false,
                supports_caching: true,
            },
        );
        model_profiles.insert(
            "codex-mini-latest".to_string(),
            ModelProfile {
                id: "codex-mini-latest".to_string(),
                provider_id: "openai_responses".to_string(),
                model: "codex-mini-latest".to_string(),
                api_mode: ApiMode::CodexResponses,
                input_cost_per_1m: 1.5,
                output_cost_per_1m: 6.0,
                context_window: 200_000,
                supports_tools: true,
                supports_reasoning: true,
                supports_caching: true,
            },
        );

        let http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .user_agent("hermes-rust/1.0")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            inner: Arc::new(Mutex::new(ProviderRouterState {
                providers,
                model_profiles,
                active_provider_id: "local_hermes".to_string(),
                active_model: "hermes-local-v1".to_string(),
                fallback_chain: Vec::new(),
                allow_local_fallback: true,
            })),
            http_client,
        }
    }

    /// Discovers optional API keys from environment variables without ever hardcoding third-party gateways.
    pub fn from_env() -> Self {
        let router = Self::new();
        let mut active_set = false;

        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("ANTHROPIC_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string());
                let model = std::env::var("ANTHROPIC_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "claude-sonnet-4-20250514".to_string());
                router.register_provider(ProviderConfig::anthropic(
                    "anthropic",
                    "Anthropic Messages",
                    base,
                    model.clone(),
                    trimmed.to_string(),
                ));
                if !active_set {
                    let _ = router.set_active("anthropic", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("OPENAI_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
                let model = std::env::var("OPENAI_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "gpt-4o".to_string());
                router.register_provider(ProviderConfig::openai_compatible(
                    "openai",
                    "OpenAI Chat Completions",
                    base.clone(),
                    model.clone(),
                    trimmed.to_string(),
                ));
                router.register_provider(ProviderConfig::codex_responses(
                    "openai_responses",
                    "OpenAI Codex Responses",
                    base,
                    "codex-mini-latest",
                    trimmed.to_string(),
                ));
                if !active_set {
                    let _ = router.set_active("openai", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("OPENROUTER_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://openrouter.ai/api/v1".to_string());
                let model = std::env::var("OPENROUTER_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "anthropic/claude-sonnet-4".to_string());
                router.register_provider(ProviderConfig::openai_compatible(
                    "openrouter",
                    "OpenRouter",
                    base,
                    model.clone(),
                    trimmed.to_string(),
                ));
                if !active_set {
                    let _ = router.set_active("openrouter", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("DEEPSEEK_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://api.deepseek.com/v1".to_string());
                let model = std::env::var("DEEPSEEK_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "deepseek-chat".to_string());
                router.register_provider(ProviderConfig::openai_compatible(
                    "deepseek",
                    "DeepSeek",
                    base,
                    model.clone(),
                    trimmed.to_string(),
                ));
                if !active_set {
                    let _ = router.set_active("deepseek", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(key) = std::env::var("NOUS_API_KEY") {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("NOUS_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://inference-api.nousresearch.com/v1".to_string());
                let model = std::env::var("NOUS_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "Hermes-3-Llama-3.1-70B".to_string());
                router.register_provider(ProviderConfig::openai_compatible(
                    "nous",
                    "Nous Research",
                    base,
                    model.clone(),
                    trimmed.to_string(),
                ));
                if !active_set {
                    let _ = router.set_active("nous", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(key) = std::env::var("HERMES_API_KEY").or_else(|_| std::env::var("DACHENG_AI_TOKEN"))
        {
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                let base = std::env::var("HERMES_BASE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "https://ai.ombhrum.com/codex-deepseek/v1".to_string());
                let model = std::env::var("HERMES_MODEL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "deepseek-chat".to_string());
                let mut cfg = ProviderConfig::openai_compatible(
                    "hermes_cloud",
                    "Hermes Cloud",
                    base.clone(),
                    model.clone(),
                    trimmed.to_string(),
                );
                if base.contains("/codex-") || base.contains("ai.ombhrum.com") {
                    cfg.api_mode = ApiMode::CodexResponses;
                }
                router.register_provider(cfg);
                if !active_set {
                    let _ = router.set_active("hermes_cloud", Some(&model));
                    active_set = true;
                }
            }
        }

        if let Ok(mode_raw) = std::env::var("HERMES_API_MODE") {
            if let Some(mode) = ApiMode::parse(&mode_raw) {
                let _ = router.set_api_mode_for_active(mode);
            }
        }

        router
    }

    pub fn register_provider(&self, config: ProviderConfig) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.providers.insert(config.id.clone(), config);
    }

    pub fn remove_provider(&self, provider_id: &str) -> bool {
        if provider_id == "local_hermes" {
            return false;
        }
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let removed = guard.providers.remove(provider_id).is_some();
        if guard.active_provider_id == provider_id {
            guard.active_provider_id = "local_hermes".to_string();
            guard.active_model = "hermes-local-v1".to_string();
        }
        removed
    }

    pub fn register_model_profile(&self, profile: ModelProfile) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.model_profiles.insert(profile.id.clone(), profile);
    }

    pub fn set_active(
        &self,
        provider_id: &str,
        model: Option<&str>,
    ) -> Result<(String, String), ProviderError> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let Some(cfg) = guard.providers.get(provider_id) else {
            return Err(ProviderError::Configuration(format!(
                "Unknown provider '{provider_id}'"
            )));
        };
        if !cfg.enabled {
            return Err(ProviderError::Configuration(format!(
                "Provider '{provider_id}' is disabled"
            )));
        }
        let resolved_model = model
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .unwrap_or_else(|| cfg.default_model.clone());
        guard.active_provider_id = provider_id.to_string();
        guard.active_model = resolved_model.clone();
        Ok((provider_id.to_string(), resolved_model))
    }

    pub fn set_active_model(&self, model_or_provider_slash_model: &str) -> (String, String) {
        let trimmed = model_or_provider_slash_model.trim();
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((prov, model)) = trimmed.split_once('/') {
            if guard.providers.contains_key(prov) {
                guard.active_provider_id = prov.to_string();
                guard.active_model = model.to_string();
                return (prov.to_string(), model.to_string());
            }
        }
        if let Some(profile) = guard.model_profiles.get(trimmed).cloned() {
            if guard.providers.contains_key(&profile.provider_id) {
                guard.active_provider_id = profile.provider_id.clone();
            }
            guard.active_model = profile.model;
            return (guard.active_provider_id.clone(), guard.active_model.clone());
        }
        guard.active_model = trimmed.to_string();
        (guard.active_provider_id.clone(), guard.active_model.clone())
    }

    pub fn set_api_mode_for_active(&self, mode: ApiMode) -> Result<ApiMode, ProviderError> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let active_id = guard.active_provider_id.clone();
        if mode == ApiMode::LocalHermes {
            guard.active_provider_id = "local_hermes".to_string();
            if let Some(local) = guard.providers.get("local_hermes") {
                guard.active_model = local.default_model.clone();
            }
            return Ok(mode);
        }
        if let Some(cfg) = guard.providers.get_mut(&active_id) {
            cfg.api_mode = mode;
            return Ok(mode);
        }
        Err(ProviderError::Configuration(format!(
            "Active provider '{active_id}' not found"
        )))
    }

    pub fn set_fallback_chain(&self, chain: Vec<FallbackTarget>) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.fallback_chain = chain;
    }

    pub fn set_allow_local_fallback(&self, allow: bool) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.allow_local_fallback = allow;
    }

    pub fn snapshot(&self) -> ProviderRouterState {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn active_info(&self) -> (String, String, ApiMode) {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mode = guard
            .providers
            .get(&guard.active_provider_id)
            .map(|p| p.api_mode)
            .unwrap_or(ApiMode::LocalHermes);
        (
            guard.active_provider_id.clone(),
            guard.active_model.clone(),
            mode,
        )
    }

    pub fn list_providers(&self) -> Vec<ProviderConfig> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut list: Vec<_> = guard.providers.values().cloned().collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    pub fn add_credential(&self, provider_id: &str, entry: CredentialEntry) -> Result<(), ProviderError> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let Some(cfg) = guard.providers.get_mut(provider_id) else {
            return Err(ProviderError::Configuration(format!(
                "Unknown provider '{provider_id}'"
            )));
        };
        cfg.credentials.add_entry(entry);
        Ok(())
    }

    fn acquire_credential(&self, provider_id: &str) -> Result<Option<CredentialEntry>, ProviderError> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let Some(cfg) = guard.providers.get_mut(provider_id) else {
            return Err(ProviderError::Configuration(format!(
                "Unknown provider '{provider_id}'"
            )));
        };
        if cfg.api_mode == ApiMode::LocalHermes {
            return Ok(None);
        }
        let now_ms = Utc::now().timestamp_millis();
        if cfg.credentials.is_empty() {
            return Err(ProviderError::Authentication(format!(
                "No API credentials configured for provider '{provider_id}'"
            )));
        }
        match cfg.credentials.acquire(now_ms) {
            Some(entry) => Ok(Some(entry)),
            None => Err(ProviderError::RateLimited {
                status: 429,
                retry_after_secs: Some(5),
                message: format!(
                    "All {} credentials for provider '{provider_id}' are in cooldown or disabled",
                    cfg.credentials.len()
                ),
            }),
        }
    }

    fn report_credential_outcome(
        &self,
        provider_id: &str,
        cred_id: &str,
        err: Option<&ProviderError>,
    ) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let Some(cfg) = guard.providers.get_mut(provider_id) else {
            return;
        };
        match err {
            None => cfg.credentials.report_success(cred_id),
            Some(e) => {
                if let Some(status) = e.http_status() {
                    if status == 429 || status >= 500 || status == 401 {
                        let now_ms = Utc::now().timestamp_millis();
                        cfg.credentials
                            .report_rate_limit(cred_id, now_ms, status, e.retry_after_secs());
                    }
                }
            }
        }
    }

    /// Execute a completion using the active provider/model and walking the fallback chain on failure.
    pub async fn complete(
        &self,
        request: ProviderRequest,
        stream_tx: Option<mpsc::Sender<ProviderStreamEvent>>,
    ) -> Result<ProviderResponse, ProviderError> {
        let state = self.snapshot();
        let requested_model = if request.model.trim().is_empty() {
            state.active_model.clone()
        } else {
            request.model.clone()
        };

        let mut targets = Vec::new();
        targets.push(FallbackTarget {
            provider_id: state.active_provider_id.clone(),
            model: requested_model.clone(),
        });
        if let Some(active_cfg) = state.providers.get(&state.active_provider_id) {
            for fb_model in &active_cfg.fallback_models {
                if !fb_model.is_empty() && fb_model != &requested_model {
                    targets.push(FallbackTarget {
                        provider_id: state.active_provider_id.clone(),
                        model: fb_model.clone(),
                    });
                }
            }
        }
        for target in &state.fallback_chain {
            if !targets
                .iter()
                .any(|t| t.provider_id == target.provider_id && t.model == target.model)
            {
                targets.push(target.clone());
            }
        }
        if state.allow_local_fallback
            && !targets.iter().any(|t| t.provider_id == "local_hermes")
        {
            targets.push(FallbackTarget {
                provider_id: "local_hermes".to_string(),
                model: "hermes-local-v1".to_string(),
            });
        }

        let mut errors = Vec::new();
        for target in targets {
            let Some(cfg) = self
                .inner
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .providers
                .get(&target.provider_id)
                .cloned()
            else {
                errors.push(format!("{}: provider not found", target.provider_id));
                continue;
            };
            if !cfg.enabled {
                errors.push(format!("{}: provider disabled", target.provider_id));
                continue;
            }

            let mut target_req = request.clone();
            target_req.model = target.model.clone();

            match self
                .execute_single_provider(&cfg, target_req, stream_tx.as_ref())
                .await
            {
                Ok(mut resp) => {
                    self.enrich_cost(&mut resp);
                    return Ok(resp);
                }
                Err(err) => {
                    errors.push(format!("{}/{}: {}", target.provider_id, target.model, err));
                    // If this was the only target and local fallback is disabled, return the exact error
                    if errors.len() == 1 && !state.allow_local_fallback && state.fallback_chain.is_empty() {
                        return Err(err);
                    }
                }
            }
        }

        Err(ProviderError::Exhausted(errors.join(" | ")))
    }

    /// Execute against one explicit provider without fallback.
    pub async fn complete_on_provider(
        &self,
        provider_id: &str,
        request: ProviderRequest,
        stream_tx: Option<mpsc::Sender<ProviderStreamEvent>>,
    ) -> Result<ProviderResponse, ProviderError> {
        let cfg = {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            guard.providers.get(provider_id).cloned().ok_or_else(|| {
                ProviderError::Configuration(format!("Unknown provider '{provider_id}'"))
            })?
        };
        let mut resp = self
            .execute_single_provider(&cfg, request, stream_tx.as_ref())
            .await?;
        self.enrich_cost(&mut resp);
        Ok(resp)
    }

    fn enrich_cost(&self, resp: &mut ProviderResponse) {
        if resp.usage.estimated_cost_usd > 0.0 {
            return;
        }
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(profile) = guard.model_profiles.get(&resp.model) {
            resp.usage.estimated_cost_usd =
                profile.estimate_cost(resp.usage.input_tokens, resp.usage.output_tokens);
        }
    }

    async fn execute_single_provider(
        &self,
        cfg: &ProviderConfig,
        request: ProviderRequest,
        stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
    ) -> Result<ProviderResponse, ProviderError> {
        if cfg.api_mode == ApiMode::LocalHermes {
            return execute_local_hermes(&cfg.id, &request, stream_tx).await;
        }

        let max_key_attempts = cfg.credentials.len().max(1);
        let mut last_err = None;

        for _ in 0..max_key_attempts {
            let cred = match self.acquire_credential(&cfg.id)? {
                Some(c) => c,
                None => {
                    return Err(ProviderError::Authentication(format!(
                        "No credential available for '{}'",
                        cfg.id
                    )));
                }
            };
            let base_url = cred
                .base_url_override
                .as_deref()
                .unwrap_or(&cfg.base_url);
            let res = match cfg.api_mode {
                ApiMode::ChatCompletions => {
                    execute_chat_completions(
                        &self.http_client,
                        cfg,
                        base_url,
                        &cred.api_key,
                        &request,
                        stream_tx,
                    )
                    .await
                }
                ApiMode::CodexResponses => {
                    execute_codex_responses(
                        &self.http_client,
                        cfg,
                        base_url,
                        &cred.api_key,
                        &request,
                        stream_tx,
                    )
                    .await
                }
                ApiMode::AnthropicMessages => {
                    execute_anthropic_messages(
                        &self.http_client,
                        cfg,
                        base_url,
                        &cred.api_key,
                        &request,
                        stream_tx,
                    )
                    .await
                }
                ApiMode::LocalHermes => execute_local_hermes(&cfg.id, &request, stream_tx).await,
            };
            match res {
                Ok(resp) => {
                    self.report_credential_outcome(&cfg.id, &cred.id, None);
                    return Ok(resp);
                }
                Err(err) => {
                    self.report_credential_outcome(&cfg.id, &cred.id, Some(&err));
                    let retry_key = err.is_retryable() || err.http_status() == Some(401);
                    last_err = Some(err);
                    if !retry_key {
                        break;
                    }
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            ProviderError::Exhausted(format!("No usable credentials for '{}'", cfg.id))
        }))
    }
}

// ============================================================================
// Wire Protocol Payload Builders & Parsers (ChatCompletions / CodexResponses / AnthropicMessages)
// ============================================================================

pub fn resolve_endpoint_url(base_url: &str, suffix: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    let clean_suffix = suffix.trim_start_matches('/');
    if trimmed.ends_with(&format!("/{clean_suffix}")) {
        trimmed.to_string()
    } else {
        format!("{trimmed}/{clean_suffix}")
    }
}

pub fn build_chat_completions_payload(req: &ProviderRequest) -> Value {
    let mut messages_json = Vec::new();
    let system_text = req.combined_system_prompt();
    if !system_text.is_empty() {
        messages_json.push(json!({
            "role": "system",
            "content": system_text,
        }));
    }

    for msg in &req.messages {
        match msg.role {
            ProviderMessageRole::System => {
                messages_json.push(json!({
                    "role": "system",
                    "content": msg.content,
                }));
            }
            ProviderMessageRole::User => {
                messages_json.push(json!({
                    "role": "user",
                    "content": msg.content,
                }));
            }
            ProviderMessageRole::Assistant => {
                let mut obj = json!({
                    "role": "assistant",
                    "content": if msg.content.is_empty() && !msg.tool_calls.is_empty() {
                        Value::Null
                    } else {
                        Value::String(msg.content.clone())
                    }
                });
                if !msg.tool_calls.is_empty() {
                    let tcs: Vec<Value> = msg
                        .tool_calls
                        .iter()
                        .map(|tc| {
                            json!({
                                "id": tc.id,
                                "type": "function",
                                "function": {
                                    "name": tc.name,
                                    "arguments": tc.arguments,
                                }
                            })
                        })
                        .collect();
                    obj["tool_calls"] = Value::Array(tcs);
                }
                messages_json.push(obj);
            }
            ProviderMessageRole::Tool => {
                let mut obj = json!({
                    "role": "tool",
                    "content": msg.content,
                });
                if let Some(id) = &msg.tool_call_id {
                    obj["tool_call_id"] = Value::String(id.clone());
                }
                if let Some(name) = &msg.name {
                    obj["name"] = Value::String(name.clone());
                }
                messages_json.push(obj);
            }
        }
    }

    let mut payload = json!({
        "model": req.model,
        "messages": messages_json,
        "stream": req.stream,
    });

    if !req.tools.is_empty() {
        payload["tools"] = Value::Array(
            req.tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect(),
        );
    }
    if let Some(temp) = req.temperature {
        payload["temperature"] = json!(temp);
    }
    if let Some(max_tok) = req.max_tokens {
        payload["max_tokens"] = json!(max_tok);
    }
    if req.stream {
        payload["stream_options"] = json!({ "include_usage": true });
    }
    if let Some(budget) = req.thinking_budget_tokens {
        if budget > 0 {
            payload["reasoning_effort"] = json!(if budget >= 16_000 { "high" } else { "medium" });
        }
    }

    payload
}

pub fn build_codex_responses_payload(req: &ProviderRequest) -> Value {
    let mut input_items = Vec::new();
    for msg in &req.messages {
        match msg.role {
            ProviderMessageRole::System => {
                input_items.push(json!({
                    "type": "message",
                    "role": "system",
                    "content": msg.content,
                }));
            }
            ProviderMessageRole::User => {
                input_items.push(json!({
                    "type": "message",
                    "role": "user",
                    "content": msg.content,
                }));
            }
            ProviderMessageRole::Assistant => {
                if !msg.content.is_empty() {
                    input_items.push(json!({
                        "type": "message",
                        "role": "assistant",
                        "content": msg.content,
                    }));
                }
                for tc in &msg.tool_calls {
                    input_items.push(json!({
                        "type": "function_call",
                        "call_id": tc.id,
                        "name": tc.name,
                        "arguments": tc.arguments,
                    }));
                }
            }
            ProviderMessageRole::Tool => {
                input_items.push(json!({
                    "type": "function_call_output",
                    "call_id": msg.tool_call_id.clone().unwrap_or_default(),
                    "output": msg.content,
                }));
            }
        }
    }

    let mut payload = json!({
        "model": req.model,
        "input": input_items,
        "stream": req.stream,
    });

    let instructions = req.combined_system_prompt();
    if !instructions.is_empty() {
        payload["instructions"] = Value::String(instructions);
    }

    if !req.tools.is_empty() {
        payload["tools"] = Value::Array(
            req.tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    })
                })
                .collect(),
        );
    }
    if let Some(temp) = req.temperature {
        payload["temperature"] = json!(temp);
    }
    if let Some(max_tok) = req.max_tokens {
        payload["max_output_tokens"] = json!(max_tok);
    }
    if let Some(budget) = req.thinking_budget_tokens {
        if budget > 0 {
            payload["reasoning"] = json!({
                "effort": if budget >= 16_000 { "high" } else { "medium" }
            });
        }
    }
    payload
}

pub fn build_anthropic_messages_payload(req: &ProviderRequest) -> Value {
    let mut system_blocks = Vec::new();
    for (idx, tier) in req.system_tiers.iter().enumerate() {
        if tier.text.trim().is_empty() {
            continue;
        }
        let mut block = json!({
            "type": "text",
            "text": tier.text,
        });
        if tier.cache_ephemeral || idx == 0 {
            block["cache_control"] = json!({ "type": "ephemeral" });
        }
        system_blocks.push(block);
    }

    let mut raw_messages: Vec<Value> = Vec::new();
    for msg in &req.messages {
        match msg.role {
            ProviderMessageRole::System => {
                if !msg.content.trim().is_empty() {
                    system_blocks.push(json!({
                        "type": "text",
                        "text": msg.content,
                    }));
                }
            }
            ProviderMessageRole::User => {
                let mut block = json!({
                    "type": "text",
                    "text": msg.content,
                });
                if let Some(cc) = &msg.cache_control {
                    block["cache_control"] = json!({ "type": cc.cache_type });
                }
                raw_messages.push(json!({
                    "role": "user",
                    "content": [block],
                }));
            }
            ProviderMessageRole::Assistant => {
                let mut blocks = Vec::new();
                if let Some(reasoning) = &msg.reasoning {
                    if !reasoning.trim().is_empty() {
                        blocks.push(json!({
                            "type": "thinking",
                            "thinking": reasoning,
                        }));
                    }
                }
                if !msg.content.is_empty() {
                    blocks.push(json!({
                        "type": "text",
                        "text": msg.content,
                    }));
                }
                for tc in &msg.tool_calls {
                    blocks.push(json!({
                        "type": "tool_use",
                        "id": tc.id,
                        "name": tc.name,
                        "input": tc.arguments_json(),
                    }));
                }
                if blocks.is_empty() {
                    blocks.push(json!({ "type": "text", "text": "" }));
                }
                raw_messages.push(json!({
                    "role": "assistant",
                    "content": blocks,
                }));
            }
            ProviderMessageRole::Tool => {
                let mut block = json!({
                    "type": "tool_result",
                    "tool_use_id": msg.tool_call_id.clone().unwrap_or_default(),
                    "content": msg.content,
                });
                if let Some(cc) = &msg.cache_control {
                    block["cache_control"] = json!({ "type": cc.cache_type });
                }
                // Merge adjacent tool_result blocks into one Anthropic user turn
                let should_merge = raw_messages
                    .last()
                    .and_then(|last| last.get("role"))
                    .and_then(Value::as_str)
                    == Some("user");
                if should_merge {
                    if let Some(arr) = raw_messages
                        .last_mut()
                        .and_then(|last| last.get_mut("content"))
                        .and_then(Value::as_array_mut)
                    {
                        arr.push(block);
                        continue;
                    }
                }
                raw_messages.push(json!({
                    "role": "user",
                    "content": [block],
                }));
            }
        }
    }

    // Apply Anthropic prompt caching marker to the trailing user/tool message block if not already set
    if let Some(last_msg) = raw_messages.last_mut() {
        if let Some(blocks) = last_msg.get_mut("content").and_then(Value::as_array_mut) {
            if let Some(last_block) = blocks.last_mut() {
                if last_block.get("cache_control").is_none() {
                    last_block["cache_control"] = json!({ "type": "ephemeral" });
                }
            }
        }
    }

    let mut payload = json!({
        "model": req.model,
        "max_tokens": req.max_tokens.unwrap_or(4096),
        "messages": raw_messages,
        "stream": req.stream,
    });

    if !system_blocks.is_empty() {
        payload["system"] = Value::Array(system_blocks);
    }

    if !req.tools.is_empty() {
        payload["tools"] = Value::Array(
            req.tools
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "input_schema": t.parameters,
                    })
                })
                .collect(),
        );
    }
    if let Some(temp) = req.temperature {
        payload["temperature"] = json!(temp);
    }
    if let Some(budget) = req.thinking_budget_tokens {
        if budget >= 1024 {
            payload["thinking"] = json!({
                "type": "enabled",
                "budget_tokens": budget,
            });
        }
    }

    payload
}

// ============================================================================
// HTTP Wire Protocol Executors
// ============================================================================

async fn check_http_error(resp: reqwest::Response) -> Result<reqwest::Response, ProviderError> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let retry_after = resp
        .headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok());
    let body = tokio::time::timeout(Duration::from_secs(3), resp.text())
        .await
        .ok()
        .and_then(Result::ok)
        .unwrap_or_default();
    let code = status.as_u16();
    if code == 401 || code == 403 {
        return Err(ProviderError::Authentication(format!("HTTP {code}: {body}")));
    }
    if code == 429 {
        return Err(ProviderError::RateLimited {
            status: code,
            retry_after_secs: retry_after,
            message: body,
        });
    }
    Err(ProviderError::UpstreamHttp {
        status: code,
        message: body,
    })
}

pub fn normalize_wire_model_for_provider(provider_id: &str, base_url: &str, model: &str) -> String {
    let m = model.trim();
    let is_deepseek = provider_id.contains("deepseek")
        || base_url.contains("api.deepseek.com")
        || base_url.contains("ai.ombhrum.com")
        || base_url.contains("codex-deepseek");

    if m.is_empty() || m == "hermes-local-v1" {
        if is_deepseek {
            return "deepseek-chat".to_string();
        }
        if provider_id == "openai" || base_url.contains("api.openai.com") {
            return "gpt-4o".to_string();
        }
        if provider_id == "anthropic" || base_url.contains("anthropic.com") {
            return "claude-sonnet-4-20250514".to_string();
        }
        return m.to_string();
    }

    if is_deepseek {
        match m {
            "deepseek-flash" | "deepseek-v3" | "deepseek" | "gpt-4o-mini" | "gpt-4o" => {
                "deepseek-chat".to_string()
            }
            "deepseek-r1" | "o1" | "o3-mini" => "deepseek-reasoner".to_string(),
            _ => m.to_string(),
        }
    } else {
        m.to_string()
    }
}

fn prepare_wire_request(
    cfg: &ProviderConfig,
    base_url: &str,
    api_key: &str,
    request: &ProviderRequest,
) -> ProviderRequest {
    let mut wire = request.clone();
    wire.model = normalize_wire_model_for_provider(&cfg.id, base_url, &request.model);

    // Compact requests for unauthenticated public Codex relay tier (1,000 estimated token cap).
    if cfg.api_mode == ApiMode::CodexResponses
        && base_url.contains("ai.ombhrum.com")
        && api_key == "public"
    {
        wire.tools.clear();
        wire.thinking_budget_tokens = None;

        let combined_sys = request.combined_system_prompt();
        let compact_sys: String = combined_sys.trim().chars().take(180).collect();
        wire.system_tiers = if compact_sys.is_empty() {
            Vec::new()
        } else {
            vec![SystemPromptTier::stable("identity", compact_sys)]
        };

        let mut kept: Vec<ProviderChatMessage> = wire
            .messages
            .into_iter()
            .filter(|m| {
                matches!(
                    m.role,
                    ProviderMessageRole::User | ProviderMessageRole::Assistant
                )
            })
            .collect();
        if kept.len() > 4 {
            let start = kept.len() - 4;
            kept.drain(0..start);
        }
        for msg in &mut kept {
            msg.tool_calls.clear();
            msg.cache_control = None;
            if msg.content.chars().count() > 180 {
                msg.content = msg.content.chars().take(180).collect();
            }
        }
        wire.messages = kept;
    }

    wire
}

async fn execute_chat_completions(
    client: &reqwest::Client,
    cfg: &ProviderConfig,
    base_url: &str,
    api_key: &str,
    request: &ProviderRequest,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<ProviderResponse, ProviderError> {
    let url = resolve_endpoint_url(base_url, "chat/completions");
    let wire_req = prepare_wire_request(cfg, base_url, api_key, request);
    let payload = build_chat_completions_payload(&wire_req);

    let mut builder = client
        .post(&url)
        .bearer_auth(api_key)
        .header(reqwest::header::CONTENT_TYPE, "application/json");
    if request.stream {
        builder = builder.header(reqwest::header::ACCEPT, "text/event-stream");
    }
    for (k, v) in &cfg.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }

    let resp = tokio::time::timeout(
        Duration::from_secs(cfg.timeout_secs.max(5)),
        builder.json(&payload).send(),
    )
    .await
    .map_err(|_| ProviderError::Transport(format!("Request to {url} timed out")))?
    .map_err(|e| ProviderError::Transport(format!("HTTP request to {url} failed: {e}")))?;

    let mut resp = check_http_error(resp).await?;

    let is_sse = request.stream
        && resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.contains("text/event-stream"))
            .unwrap_or(true);

    if !is_sse {
        let body = resp
            .text()
            .await
            .map_err(|e| ProviderError::Transport(format!("Read body failed: {e}")))?;
        return parse_chat_completions_json(&cfg.id, &request.model, &body);
    }

    let mut acc_content = String::new();
    let mut acc_reasoning = String::new();
    let mut tool_calls_map: HashMap<usize, ProviderToolCall> = HashMap::new();
    let mut finish_reason = "stop".to_string();
    let mut usage = TokenUsage::default();
    let mut raw_bytes = Vec::<u8>::new();

    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| ProviderError::Transport(format!("SSE chunk error: {e}")))?
    {
        raw_bytes.extend_from_slice(&chunk);
        while let Some(pos) = raw_bytes.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&raw_bytes[..pos])
                .trim_end_matches('\r')
                .to_string();
            raw_bytes.drain(..=pos);
            process_chat_completions_sse_line(
                &line,
                &mut acc_content,
                &mut acc_reasoning,
                &mut tool_calls_map,
                &mut finish_reason,
                &mut usage,
                stream_tx,
            )
            .await?;
        }
    }
    if !raw_bytes.is_empty() {
        let line = String::from_utf8_lossy(&raw_bytes)
            .trim_end_matches('\r')
            .to_string();
        process_chat_completions_sse_line(
            &line,
            &mut acc_content,
            &mut acc_reasoning,
            &mut tool_calls_map,
            &mut finish_reason,
            &mut usage,
            stream_tx,
        )
        .await?;
    }

    // If upstream returned a single JSON object despite text/event-stream expectation
    if acc_content.is_empty() && acc_reasoning.is_empty() && tool_calls_map.is_empty() {
        // Already empty unless non-SSE handled above
    }

    let mut indices: Vec<usize> = tool_calls_map.keys().copied().collect();
    indices.sort_unstable();
    let tool_calls: Vec<ProviderToolCall> = indices
        .into_iter()
        .filter_map(|idx| tool_calls_map.remove(&idx))
        .collect();
    for tc in &tool_calls {
        if let Some(tx) = stream_tx {
            let _ = tx.send(ProviderStreamEvent::ToolCallReady(tc.clone())).await;
        }
    }
    usage.finalize_total();

    Ok(ProviderResponse {
        content: acc_content,
        reasoning: (!acc_reasoning.is_empty()).then_some(acc_reasoning),
        tool_calls,
        finish_reason,
        usage,
        provider_id: cfg.id.clone(),
        model: request.model.clone(),
        api_mode: ApiMode::ChatCompletions,
    })
}

async fn process_chat_completions_sse_line(
    line: &str,
    acc_content: &mut String,
    acc_reasoning: &mut String,
    tool_calls_map: &mut HashMap<usize, ProviderToolCall>,
    finish_reason: &mut String,
    usage: &mut TokenUsage,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<(), ProviderError> {
    let Some(data) = line.strip_prefix("data:") else {
        return Ok(());
    };
    let trimmed = data.trim();
    if trimmed.is_empty() || trimmed == "[DONE]" {
        return Ok(());
    }
    let val: Value = serde_json::from_str(trimmed)
        .map_err(|e| ProviderError::Protocol(format!("Invalid ChatCompletions SSE JSON: {e}")))?;

    if let Some(err) = val.get("error") {
        let msg = err
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown upstream error")
            .to_string();
        return Err(ProviderError::Protocol(msg));
    }

    if let Some(u) = val.get("usage") {
        if let Some(inp) = u.get("prompt_tokens").and_then(Value::as_u64) {
            usage.input_tokens = inp;
        }
        if let Some(outp) = u.get("completion_tokens").and_then(Value::as_u64) {
            usage.output_tokens = outp;
        }
        if let Some(tot) = u.get("total_tokens").and_then(Value::as_u64) {
            usage.total_tokens = tot;
        }
    }

    let Some(choice) = val.get("choices").and_then(Value::as_array).and_then(|c| c.first())
    else {
        return Ok(());
    };
    if let Some(fr) = choice.get("finish_reason").and_then(Value::as_str) {
        if !fr.is_empty() {
            *finish_reason = fr.to_string();
        }
    }
    let Some(delta) = choice.get("delta") else {
        return Ok(());
    };
    if let Some(reason) = delta
        .get("reasoning_content")
        .or_else(|| delta.get("reasoning"))
        .or_else(|| delta.get("thinking"))
        .and_then(Value::as_str)
    {
        if !reason.is_empty() {
            acc_reasoning.push_str(reason);
            if let Some(tx) = stream_tx {
                let _ = tx
                    .send(ProviderStreamEvent::ThinkingDelta(reason.to_string()))
                    .await;
            }
        }
    }
    if let Some(text) = delta.get("content").and_then(Value::as_str) {
        if !text.is_empty() {
            acc_content.push_str(text);
            if let Some(tx) = stream_tx {
                let _ = tx
                    .send(ProviderStreamEvent::TextDelta(text.to_string()))
                    .await;
            }
        }
    }
    if let Some(tcs) = delta.get("tool_calls").and_then(Value::as_array) {
        for tc in tcs {
            let idx = tc.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
            let entry = tool_calls_map.entry(idx).or_insert_with(|| ProviderToolCall {
                id: format!("call_{idx}"),
                name: String::new(),
                arguments: String::new(),
            });
            if let Some(id) = tc.get("id").and_then(Value::as_str) {
                if !id.is_empty() {
                    entry.id = id.to_string();
                }
            }
            if let Some(func) = tc.get("function") {
                if let Some(name) = func.get("name").and_then(Value::as_str) {
                    entry.name.push_str(name);
                }
                if let Some(args) = func.get("arguments").and_then(Value::as_str) {
                    entry.arguments.push_str(args);
                }
            }
        }
    }

    Ok(())
}

pub fn parse_chat_completions_json(
    provider_id: &str,
    model: &str,
    body: &str,
) -> Result<ProviderResponse, ProviderError> {
    let val: Value = serde_json::from_str(body)
        .map_err(|e| ProviderError::Protocol(format!("Invalid ChatCompletions JSON: {e}")))?;
    let choice = val
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|c| c.first())
        .ok_or_else(|| ProviderError::Protocol("Missing choices[0] in response".into()))?;
    let msg = choice
        .get("message")
        .ok_or_else(|| ProviderError::Protocol("Missing choices[0].message".into()))?;

    let content = msg
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let reasoning = msg
        .get("reasoning_content")
        .or_else(|| msg.get("reasoning"))
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let mut tool_calls = Vec::new();
    if let Some(arr) = msg.get("tool_calls").and_then(Value::as_array) {
        for (idx, tc) in arr.iter().enumerate() {
            let id = tc
                .get("id")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("call_{idx}"));
            let name = tc
                .get("function")
                .and_then(|f| f.get("name"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let arguments = tc
                .get("function")
                .and_then(|f| f.get("arguments"))
                .and_then(Value::as_str)
                .unwrap_or("{}")
                .to_string();
            tool_calls.push(ProviderToolCall {
                id,
                name,
                arguments,
            });
        }
    }
    let finish_reason = choice
        .get("finish_reason")
        .and_then(Value::as_str)
        .unwrap_or("stop")
        .to_string();
    let mut usage = TokenUsage::default();
    if let Some(u) = val.get("usage") {
        usage.input_tokens = u.get("prompt_tokens").and_then(Value::as_u64).unwrap_or(0);
        usage.output_tokens = u
            .get("completion_tokens")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        usage.total_tokens = u.get("total_tokens").and_then(Value::as_u64).unwrap_or(0);
    }
    usage.finalize_total();

    Ok(ProviderResponse {
        content,
        reasoning,
        tool_calls,
        finish_reason,
        usage,
        provider_id: provider_id.to_string(),
        model: model.to_string(),
        api_mode: ApiMode::ChatCompletions,
    })
}

async fn execute_codex_responses(
    client: &reqwest::Client,
    cfg: &ProviderConfig,
    base_url: &str,
    api_key: &str,
    request: &ProviderRequest,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<ProviderResponse, ProviderError> {
    let url = resolve_endpoint_url(base_url, "responses");
    let wire_req = prepare_wire_request(cfg, base_url, api_key, request);
    let payload = build_codex_responses_payload(&wire_req);

    let mut builder = client
        .post(&url)
        .bearer_auth(api_key)
        .header(reqwest::header::CONTENT_TYPE, "application/json");
    if request.stream {
        builder = builder.header(reqwest::header::ACCEPT, "text/event-stream");
    }
    for (k, v) in &cfg.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }

    let resp = tokio::time::timeout(
        Duration::from_secs(cfg.timeout_secs.max(5)),
        builder.json(&payload).send(),
    )
    .await
    .map_err(|_| ProviderError::Transport(format!("Request to {url} timed out")))?
    .map_err(|e| ProviderError::Transport(format!("HTTP request to {url} failed: {e}")))?;

    let mut resp = check_http_error(resp).await?;

    let mut acc_content = String::new();
    let mut acc_reasoning = String::new();
    let mut tool_calls = Vec::new();
    let mut usage = TokenUsage::default();
    let mut raw_bytes = Vec::<u8>::new();

    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| ProviderError::Transport(format!("Codex Responses stream error: {e}")))?
    {
        raw_bytes.extend_from_slice(&chunk);
        while let Some(pos) = raw_bytes.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&raw_bytes[..pos])
                .trim_end_matches('\r')
                .to_string();
            raw_bytes.drain(..=pos);
            process_codex_responses_sse_line(
                &line,
                &mut acc_content,
                &mut acc_reasoning,
                &mut tool_calls,
                &mut usage,
                stream_tx,
            )
            .await?;
        }
    }
    if !raw_bytes.is_empty() {
        let line = String::from_utf8_lossy(&raw_bytes)
            .trim_end_matches('\r')
            .to_string();
        process_codex_responses_sse_line(
            &line,
            &mut acc_content,
            &mut acc_reasoning,
            &mut tool_calls,
            &mut usage,
            stream_tx,
        )
        .await?;
    }

    usage.finalize_total();
    let finish_reason = if tool_calls.is_empty() {
        "stop"
    } else {
        "tool_calls"
    };

    Ok(ProviderResponse {
        content: acc_content,
        reasoning: (!acc_reasoning.is_empty()).then_some(acc_reasoning),
        tool_calls,
        finish_reason: finish_reason.to_string(),
        usage,
        provider_id: cfg.id.clone(),
        model: request.model.clone(),
        api_mode: ApiMode::CodexResponses,
    })
}

async fn process_codex_responses_sse_line(
    line: &str,
    acc_content: &mut String,
    acc_reasoning: &mut String,
    tool_calls: &mut Vec<ProviderToolCall>,
    usage: &mut TokenUsage,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<(), ProviderError> {
    let Some(data) = line.strip_prefix("data:") else {
        return Ok(());
    };
    let trimmed = data.trim();
    if trimmed.is_empty() || trimmed == "[DONE]" {
        return Ok(());
    }
    let val: Value = serde_json::from_str(trimmed)
        .map_err(|e| ProviderError::Protocol(format!("Invalid Codex Responses SSE JSON: {e}")))?;

    let ev_type = val.get("type").and_then(Value::as_str).unwrap_or_default();
    match ev_type {
        "response.output_text.delta" => {
            if let Some(delta) = val.get("delta").and_then(Value::as_str) {
                acc_content.push_str(delta);
                if let Some(tx) = stream_tx {
                    let _ = tx
                        .send(ProviderStreamEvent::TextDelta(delta.to_string()))
                        .await;
                }
            }
        }
        "response.reasoning_summary_text.delta" | "response.reasoning_text.delta" => {
            if let Some(delta) = val.get("delta").and_then(Value::as_str) {
                acc_reasoning.push_str(delta);
                if let Some(tx) = stream_tx {
                    let _ = tx
                        .send(ProviderStreamEvent::ThinkingDelta(delta.to_string()))
                        .await;
                }
            }
        }
        "response.output_item.done" => {
            if let Some(item) = val.get("item") {
                if item.get("type").and_then(Value::as_str) == Some("function_call") {
                    let tc = ProviderToolCall {
                        id: item
                            .get("call_id")
                            .or_else(|| item.get("id"))
                            .and_then(Value::as_str)
                            .unwrap_or("call_1")
                            .to_string(),
                        name: item
                            .get("name")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string(),
                        arguments: item
                            .get("arguments")
                            .and_then(Value::as_str)
                            .unwrap_or("{}")
                            .to_string(),
                    };
                    if !tool_calls.iter().any(|existing| existing.id == tc.id) {
                        if let Some(tx) = stream_tx {
                            let _ = tx.send(ProviderStreamEvent::ToolCallReady(tc.clone())).await;
                        }
                        tool_calls.push(tc);
                    }
                }
            }
        }
        "response.completed" => {
            if let Some(resp) = val.get("response") {
                if let Some(u) = resp.get("usage") {
                    usage.input_tokens = u.get("input_tokens").and_then(Value::as_u64).unwrap_or(0);
                    usage.output_tokens =
                        u.get("output_tokens").and_then(Value::as_u64).unwrap_or(0);
                    usage.total_tokens = u.get("total_tokens").and_then(Value::as_u64).unwrap_or(0);
                }
                if let Some(items) = resp.get("output").and_then(Value::as_array) {
                    for item in items {
                        if item.get("type").and_then(Value::as_str) == Some("function_call") {
                            let id = item
                                .get("call_id")
                                .or_else(|| item.get("id"))
                                .and_then(Value::as_str)
                                .unwrap_or("call_1")
                                .to_string();
                            if !tool_calls.iter().any(|existing| existing.id == id) {
                                let tc = ProviderToolCall {
                                    id,
                                    name: item
                                        .get("name")
                                        .and_then(Value::as_str)
                                        .unwrap_or_default()
                                        .to_string(),
                                    arguments: item
                                        .get("arguments")
                                        .and_then(Value::as_str)
                                        .unwrap_or("{}")
                                        .to_string(),
                                };
                                tool_calls.push(tc);
                            }
                        }
                    }
                }
            }
        }
        "response.failed" | "error" => {
            let msg = val
                .get("response")
                .and_then(|r| r.get("error"))
                .or_else(|| val.get("error"))
                .and_then(|e| e.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("Codex Responses stream error")
                .to_string();
            return Err(ProviderError::Protocol(msg));
        }
        _ => {}
    }

    Ok(())
}

async fn execute_anthropic_messages(
    client: &reqwest::Client,
    cfg: &ProviderConfig,
    base_url: &str,
    api_key: &str,
    request: &ProviderRequest,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<ProviderResponse, ProviderError> {
    let url = resolve_endpoint_url(base_url, "messages");
    let wire_req = prepare_wire_request(cfg, base_url, api_key, request);
    let payload = build_anthropic_messages_payload(&wire_req);

    let mut builder = client
        .post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("anthropic-beta", "prompt-caching-2024-07-31")
        .header(reqwest::header::CONTENT_TYPE, "application/json");
    if request.stream {
        builder = builder.header(reqwest::header::ACCEPT, "text/event-stream");
    }
    for (k, v) in &cfg.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }

    let resp = tokio::time::timeout(
        Duration::from_secs(cfg.timeout_secs.max(5)),
        builder.json(&payload).send(),
    )
    .await
    .map_err(|_| ProviderError::Transport(format!("Request to {url} timed out")))?
    .map_err(|e| ProviderError::Transport(format!("HTTP request to {url} failed: {e}")))?;

    let mut resp = check_http_error(resp).await?;

    let is_sse = request.stream
        && resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.contains("text/event-stream"))
            .unwrap_or(true);

    if !is_sse {
        let body = resp
            .text()
            .await
            .map_err(|e| ProviderError::Transport(format!("Read Anthropic body failed: {e}")))?;
        return parse_anthropic_messages_json(&cfg.id, &request.model, &body);
    }

    let mut acc_content = String::new();
    let mut acc_reasoning = String::new();
    let mut active_blocks: HashMap<usize, AnthropicBlockAccum> = HashMap::new();
    let mut tool_calls = Vec::new();
    let mut stop_reason = "end_turn".to_string();
    let mut usage = TokenUsage::default();
    let mut current_event = String::new();
    let mut raw_bytes = Vec::<u8>::new();

    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| ProviderError::Transport(format!("Anthropic SSE error: {e}")))?
    {
        raw_bytes.extend_from_slice(&chunk);
        while let Some(pos) = raw_bytes.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&raw_bytes[..pos])
                .trim_end_matches('\r')
                .to_string();
            raw_bytes.drain(..=pos);
            process_anthropic_sse_line(
                &line,
                &mut current_event,
                &mut acc_content,
                &mut acc_reasoning,
                &mut active_blocks,
                &mut tool_calls,
                &mut stop_reason,
                &mut usage,
                stream_tx,
            )
            .await?;
        }
    }
    if !raw_bytes.is_empty() {
        let line = String::from_utf8_lossy(&raw_bytes)
            .trim_end_matches('\r')
            .to_string();
        process_anthropic_sse_line(
            &line,
            &mut current_event,
            &mut acc_content,
            &mut acc_reasoning,
            &mut active_blocks,
            &mut tool_calls,
            &mut stop_reason,
            &mut usage,
            stream_tx,
        )
        .await?;
    }

    // Flush any remaining open tool blocks
    let mut remaining_indices: Vec<usize> = active_blocks.keys().copied().collect();
    remaining_indices.sort_unstable();
    for idx in remaining_indices {
        if let Some(AnthropicBlockAccum::ToolUse { id, name, input_json }) =
            active_blocks.remove(&idx)
        {
            let tc = ProviderToolCall {
                id,
                name,
                arguments: if input_json.trim().is_empty() {
                    "{}".to_string()
                } else {
                    input_json
                },
            };
            if let Some(tx) = stream_tx {
                let _ = tx.send(ProviderStreamEvent::ToolCallReady(tc.clone())).await;
            }
            tool_calls.push(tc);
        }
    }

    usage.finalize_total();
    Ok(ProviderResponse {
        content: acc_content,
        reasoning: (!acc_reasoning.is_empty()).then_some(acc_reasoning),
        tool_calls,
        finish_reason: stop_reason,
        usage,
        provider_id: cfg.id.clone(),
        model: request.model.clone(),
        api_mode: ApiMode::AnthropicMessages,
    })
}

enum AnthropicBlockAccum {
    Text,
    Thinking,
    ToolUse {
        id: String,
        name: String,
        input_json: String,
    },
}

#[allow(clippy::too_many_arguments)]
async fn process_anthropic_sse_line(
    line: &str,
    current_event: &mut String,
    acc_content: &mut String,
    acc_reasoning: &mut String,
    active_blocks: &mut HashMap<usize, AnthropicBlockAccum>,
    tool_calls: &mut Vec<ProviderToolCall>,
    stop_reason: &mut String,
    usage: &mut TokenUsage,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<(), ProviderError> {
    if line.is_empty() {
        current_event.clear();
        return Ok(());
    }
    if let Some(ev) = line.strip_prefix("event:") {
        *current_event = ev.trim().to_string();
        return Ok(());
    }
    let Some(data) = line.strip_prefix("data:") else {
        return Ok(());
    };
    let trimmed = data.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let val: Value = serde_json::from_str(trimmed)
        .map_err(|e| ProviderError::Protocol(format!("Invalid Anthropic SSE JSON: {e}")))?;

    let kind = val
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or(current_event.as_str());

    match kind {
        "error" => {
            let msg = val
                .get("error")
                .and_then(|e| e.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("Anthropic upstream error")
                .to_string();
            return Err(ProviderError::Protocol(msg));
        }
        "message_start" => {
            if let Some(u) = val.get("message").and_then(|m| m.get("usage")) {
                usage.input_tokens = u.get("input_tokens").and_then(Value::as_u64).unwrap_or(0);
                usage.cache_read_tokens = u
                    .get("cache_read_input_tokens")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                usage.cache_creation_tokens = u
                    .get("cache_creation_input_tokens")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
            }
        }
        "content_block_start" => {
            let idx = val.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
            if let Some(cb) = val.get("content_block") {
                match cb.get("type").and_then(Value::as_str).unwrap_or("") {
                    "tool_use" => {
                        let id = cb
                            .get("id")
                            .and_then(Value::as_str)
                            .unwrap_or("toolu_1")
                            .to_string();
                        let name = cb
                            .get("name")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string();
                        active_blocks.insert(
                            idx,
                            AnthropicBlockAccum::ToolUse {
                                id,
                                name,
                                input_json: String::new(),
                            },
                        );
                    }
                    "thinking" => {
                        active_blocks.insert(idx, AnthropicBlockAccum::Thinking);
                    }
                    _ => {
                        active_blocks.insert(idx, AnthropicBlockAccum::Text);
                    }
                }
            }
        }
        "content_block_delta" => {
            let idx = val.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
            if let Some(delta) = val.get("delta") {
                match delta.get("type").and_then(Value::as_str).unwrap_or("") {
                    "text_delta" => {
                        if let Some(text) = delta.get("text").and_then(Value::as_str) {
                            acc_content.push_str(text);
                            if let Some(tx) = stream_tx {
                                let _ = tx
                                    .send(ProviderStreamEvent::TextDelta(text.to_string()))
                                    .await;
                            }
                        }
                    }
                    "thinking_delta" => {
                        if let Some(thinking) = delta.get("thinking").and_then(Value::as_str) {
                            acc_reasoning.push_str(thinking);
                            if let Some(tx) = stream_tx {
                                let _ = tx
                                    .send(ProviderStreamEvent::ThinkingDelta(thinking.to_string()))
                                    .await;
                            }
                        }
                    }
                    "input_json_delta" => {
                        if let Some(partial) = delta.get("partial_json").and_then(Value::as_str) {
                            if let Some(AnthropicBlockAccum::ToolUse { input_json, .. }) =
                                active_blocks.get_mut(&idx)
                            {
                                input_json.push_str(partial);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        "content_block_stop" => {
            let idx = val.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
            if let Some(AnthropicBlockAccum::ToolUse { id, name, input_json }) =
                active_blocks.remove(&idx)
            {
                let tc = ProviderToolCall {
                    id,
                    name,
                    arguments: if input_json.trim().is_empty() {
                        "{}".to_string()
                    } else {
                        input_json
                    },
                };
                if let Some(tx) = stream_tx {
                    let _ = tx.send(ProviderStreamEvent::ToolCallReady(tc.clone())).await;
                }
                tool_calls.push(tc);
            }
        }
        "message_delta" => {
            if let Some(sr) = val
                .get("delta")
                .and_then(|d| d.get("stop_reason"))
                .and_then(Value::as_str)
            {
                *stop_reason = sr.to_string();
            }
            if let Some(u) = val.get("usage") {
                if let Some(outp) = u.get("output_tokens").and_then(Value::as_u64) {
                    usage.output_tokens = outp;
                }
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn parse_anthropic_messages_json(
    provider_id: &str,
    model: &str,
    body: &str,
) -> Result<ProviderResponse, ProviderError> {
    let val: Value = serde_json::from_str(body)
        .map_err(|e| ProviderError::Protocol(format!("Invalid Anthropic JSON: {e}")))?;

    let mut content = String::new();
    let mut reasoning = String::new();
    let mut tool_calls = Vec::new();

    if let Some(blocks) = val.get("content").and_then(Value::as_array) {
        for block in blocks {
            match block.get("type").and_then(Value::as_str).unwrap_or("") {
                "text" => {
                    if let Some(t) = block.get("text").and_then(Value::as_str) {
                        if !content.is_empty() {
                            content.push('\n');
                        }
                        content.push_str(t);
                    }
                }
                "thinking" => {
                    if let Some(t) = block.get("thinking").and_then(Value::as_str) {
                        reasoning.push_str(t);
                    }
                }
                "tool_use" => {
                    let id = block
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("toolu_1")
                        .to_string();
                    let name = block
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    let arguments = block
                        .get("input")
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "{}".to_string());
                    tool_calls.push(ProviderToolCall {
                        id,
                        name,
                        arguments,
                    });
                }
                _ => {}
            }
        }
    }

    let mut usage = TokenUsage::default();
    if let Some(u) = val.get("usage") {
        usage.input_tokens = u.get("input_tokens").and_then(Value::as_u64).unwrap_or(0);
        usage.output_tokens = u.get("output_tokens").and_then(Value::as_u64).unwrap_or(0);
        usage.cache_read_tokens = u
            .get("cache_read_input_tokens")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        usage.cache_creation_tokens = u
            .get("cache_creation_input_tokens")
            .and_then(Value::as_u64)
            .unwrap_or(0);
    }
    usage.finalize_total();

    let finish_reason = val
        .get("stop_reason")
        .and_then(Value::as_str)
        .unwrap_or("end_turn")
        .to_string();

    Ok(ProviderResponse {
        content,
        reasoning: (!reasoning.is_empty()).then_some(reasoning),
        tool_calls,
        finish_reason,
        usage,
        provider_id: provider_id.to_string(),
        model: model.to_string(),
        api_mode: ApiMode::AnthropicMessages,
    })
}

// ============================================================================
// Pure-Rust LocalHermes Tool-Capable Responder
// ============================================================================

fn extract_meta_str<'a>(meta: &'a Value, key: &str) -> Option<&'a str> {
    meta.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

fn estimate_tokens_rough(text: &str) -> u64 {
    ((text.chars().count() as f64) / 3.5).ceil() as u64
}

fn parse_explicit_tool_directive(
    prompt: &str,
    available_tools: &[ProviderToolSpec],
) -> Option<ProviderToolCall> {
    let trimmed = prompt.trim();

    // Pattern 1: `@tool:<name>(<json>)` or `/call <name> <json>`
    if let Some(rest) = trimmed.strip_prefix("@tool:") {
        let (name, args_part) = match rest.find('(') {
            Some(idx) => {
                let n = rest[..idx].trim();
                let a = rest[idx + 1..].trim().trim_end_matches(')').trim();
                (n, if a.is_empty() { "{}" } else { a })
            }
            None => (rest.trim(), "{}"),
        };
        if available_tools.iter().any(|t| t.name == name) {
            return Some(ProviderToolCall {
                id: format!("local_call_{}", Uuid_short()),
                name: name.to_string(),
                arguments: args_part.to_string(),
            });
        }
    }

    // Pattern 2: `use tool <name>` with optional JSON payload
    let lower = trimmed.to_ascii_lowercase();
    if let Some(after) = lower.strip_prefix("use tool ") {
        let orig_after = &trimmed["use tool ".len()..];
        let mut parts = orig_after.splitn(2, char::is_whitespace);
        let name = parts.next().unwrap_or("").trim();
        let rest_args = parts.next().unwrap_or("{}").trim();
        let json_args = if rest_args.starts_with('{') {
            rest_args.to_string()
        } else {
            "{}".to_string()
        };
        if !after.is_empty() && available_tools.iter().any(|t| t.name == name) {
            return Some(ProviderToolCall {
                id: format!("local_call_{}", Uuid_short()),
                name: name.to_string(),
                arguments: json_args,
            });
        }
    }

    // Pattern 3: Natural language file read / directory list / terminal / kanban when tools exist
    if (lower.starts_with("read file ") || lower.starts_with("inspect file "))
        && available_tools.iter().any(|t| t.name == "read_file")
    {
        let path = trimmed
            .split_whitespace()
            .last()
            .unwrap_or("")
            .trim_matches(|c| c == '`' || c == '"' || c == '\'');
        if !path.is_empty() {
            return Some(ProviderToolCall {
                id: format!("local_call_{}", Uuid_short()),
                name: "read_file".to_string(),
                arguments: json!({ "path": path }).to_string(),
            });
        }
    }

    if (lower.starts_with("list files in ") || lower.starts_with("list dir "))
        && available_tools.iter().any(|t| t.name == "list_dir")
    {
        let path = trimmed
            .split_whitespace()
            .last()
            .unwrap_or(".")
            .trim_matches(|c| c == '`' || c == '"' || c == '\'');
        return Some(ProviderToolCall {
            id: format!("local_call_{}", Uuid_short()),
            name: "list_dir".to_string(),
            arguments: json!({ "path": path }).to_string(),
        });
    }

    if (lower.starts_with("run shell: ") || lower.starts_with("exec: "))
        && available_tools.iter().any(|t| t.name == "terminal")
    {
        let cmd = trimmed.split_once(':').map(|x| x.1.trim()).unwrap_or("");
        if !cmd.is_empty() {
            return Some(ProviderToolCall {
                id: format!("local_call_{}", Uuid_short()),
                name: "terminal".to_string(),
                arguments: json!({ "command": cmd }).to_string(),
            });
        }
    }

    if (lower.starts_with("search files for ") || lower.starts_with("grep "))
        && available_tools.iter().any(|t| t.name == "search_files")
    {
        let query = trimmed
            .strip_prefix("search files for ")
            .or_else(|| trimmed.strip_prefix("grep "))
            .unwrap_or("")
            .trim()
            .trim_matches(|c| c == '`' || c == '"' || c == '\'');
        if !query.is_empty() {
            return Some(ProviderToolCall {
                id: format!("local_call_{}", Uuid_short()),
                name: "search_files".to_string(),
                arguments: json!({ "query": query }).to_string(),
            });
        }
    }

    None
}

#[allow(non_snake_case)]
fn Uuid_short() -> String {
    let u = uuid::Uuid::new_v4().simple().to_string();
    u[..10].to_string()
}

fn contains_cjk(text: &str) -> bool {
    text.chars()
        .any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c) || ('\u{3400}'..='\u{4DBF}').contains(&c))
}

fn clean_persona_summary(persona: &str) -> String {
    let mut s = persona.trim().to_string();
    for prefix in ["You are ", "Your name is "] {
        if let Some(rest) = s.strip_prefix(prefix) {
            if let Some(dot) = rest.find('.') {
                s = rest[dot + 1..].trim().to_string();
            } else {
                s.clear();
            }
        }
    }
    s = s
        .replace(
            "Reply naturally and helpfully in the user's language, and never refer to yourself by numeric user ID.",
            "",
        )
        .replace("Helpful, concise, tool-capable autonomous assistant", "")
        .trim()
        .to_string();
    s
}

pub async fn execute_local_hermes(
    provider_id: &str,
    request: &ProviderRequest,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) -> Result<ProviderResponse, ProviderError> {
    let bot_name = extract_meta_str(&request.metadata, "bot_name").unwrap_or("Hermes");
    let soul_persona = extract_meta_str(&request.metadata, "soul_persona")
        .unwrap_or("Helpful, concise, tool-capable autonomous assistant");
    let memory_snapshot = extract_meta_str(&request.metadata, "memory_snapshot").unwrap_or("");
    let user_snapshot = extract_meta_str(&request.metadata, "user_snapshot").unwrap_or("");
    let skills_index = extract_meta_str(&request.metadata, "skills_index").unwrap_or("");
    let perspective = extract_meta_str(&request.metadata, "perspective");

    let last_msg = request.messages.last();
    let last_user_msg = request
        .messages
        .iter()
        .rev()
        .find(|m| m.role == ProviderMessageRole::User)
        .map(|m| m.content.as_str())
        .unwrap_or("");

    let reasoning_trace = if request.thinking_budget_tokens.unwrap_or(0) > 0 {
        let trace = format!(
            "[{bot_name}] Evaluating {} messages with {} available tools in LocalHermes runtime.",
            request.messages.len(),
            request.tools.len()
        );
        if let Some(tx) = stream_tx {
            let _ = tx
                .send(ProviderStreamEvent::ThinkingDelta(trace.clone()))
                .await;
        }
        Some(trace)
    } else {
        None
    };

    // If the most recent message is a tool result, synthesize the tool output back to the user!
    if matches!(last_msg.map(|m| m.role), Some(ProviderMessageRole::Tool)) {
        let mut recent_tool_outputs = Vec::new();
        for msg in request.messages.iter().rev() {
            if msg.role == ProviderMessageRole::Tool {
                let name = msg.name.as_deref().unwrap_or("tool");
                recent_tool_outputs.push(format!("- **{name}**: {}", msg.content.trim()));
            } else {
                break;
            }
        }
        recent_tool_outputs.reverse();
        let reply = format!(
            "Completed tool execution for \"{}\":\n{}",
            last_user_msg.trim(),
            recent_tool_outputs.join("\n")
        );
        emit_local_deltas(&reply, stream_tx).await;
        return Ok(build_local_response(
            provider_id,
            &request.model,
            request,
            reply,
            reasoning_trace,
            Vec::new(),
            "stop",
        ));
    }

    // Check if the user's message directly invokes an available Hermes tool
    if let Some(tc) = parse_explicit_tool_directive(last_user_msg, &request.tools) {
        if let Some(tx) = stream_tx {
            let _ = tx.send(ProviderStreamEvent::ToolCallReady(tc.clone())).await;
        }
        return Ok(build_local_response(
            provider_id,
            &request.model,
            request,
            String::new(),
            reasoning_trace,
            vec![tc],
            "tool_calls",
        ));
    }

    // Otherwise craft an authentic, context-aware pure-Rust response respecting SOUL.md, MEMORY.md, USER.md, SKILL.md, and conversation history
    let lower = last_user_msg.trim().to_ascii_lowercase();
    let trimmed = last_user_msg.trim();
    let persona_summary = clean_persona_summary(soul_persona);
    let msg_is_cjk = contains_cjk(trimmed);
    let history_is_cjk = msg_is_cjk
        || request
            .messages
            .iter()
            .rev()
            .take(6)
            .any(|m| contains_cjk(&m.content))
        || contains_cjk(&persona_summary);

    let mut reply = if let Some(role) = perspective {
        format!(
            "[{role} Perspective]\nAnalyzing: \"{}\"\n- Identity: {bot_name} ({soul_persona})\n- Context depth: {} conversation turns, {} available tools\n- Recommendation: Prioritize verifiable steps, preserve session continuity, and surface explicit tool/state constraints.",
            trimmed,
            request.messages.len(),
            request.tools.len()
        )
    } else if matches!(
        lower.as_str(),
        "hi" | "hello" | "hey" | "你好" | "您好" | "嗨" | "在吗"
    ) {
        if msg_is_cjk {
            let bio_line = if persona_summary.is_empty() {
                String::new()
            } else {
                format!("\n{persona_summary}")
            };
            format!(
                "你好！我是 **{bot_name}**。{bio_line}\n我可以帮你问答交流、处理工作区与文件工具（当前可用 {} 个工具）、管理记忆与技能，或执行 `/` 斜杠命令（发送 `/help` 查看全部命令）。请问有什么可以帮你？",
                request.tools.len()
            )
        } else {
            let bio_line = if persona_summary.is_empty() {
                String::new()
            } else {
                format!(" {persona_summary}")
            };
            format!(
                "Hello! I'm **{bot_name}**.{bio_line}\nI have {} active tools ready, persistent session memory, and full `/` slash command support (type `/help` to inspect commands). How can I help?",
                request.tools.len()
            )
        }
    } else if trimmed.contains("可以做什么")
        || trimmed.contains("能做什么")
        || trimmed.contains("有什么功能")
        || lower.contains("what can you do")
        || lower == "help"
    {
        if history_is_cjk {
            format!(
                "我是 **{bot_name}**，目前可以为你提供以下能力：\n- 智能对话问答、内容总结、翻译与写作\n- 工作区工具执行（当前可用 {} 个工具，支持文件读写、目录检索与命令执行）\n- 长期记忆与技能管理（支持 `/memory`、`/skills`、`/learn`）\n- 常用控制命令（发送 `/help` 查看全部 `/` 斜杠命令）\n\n请直接告诉我你想做的事。",
                request.tools.len()
            )
        } else {
            format!(
                "I'm **{bot_name}**. Here is what I can help you with:\n- Q&A, writing, summarization, and problem solving\n- Workspace and tool execution ({} active tools for files, search, and commands)\n- Persistent memory and skills (`/memory`, `/skills`, `/learn`)\n- Slash commands and automation (send `/help` for the full list)\n\nTell me what you'd like to do!",
                request.tools.len()
            )
        }
    } else if lower.contains("who are you") || trimmed.contains("你是谁") || trimmed.contains("叫什么") {
        if history_is_cjk {
            let mut s = format!("我是 **{bot_name}**。");
            if !persona_summary.is_empty() {
                s.push_str(&format!("\n{persona_summary}"));
            }
            if !skills_index.is_empty() {
                s.push_str(&format!("\n已加载技能：\n{skills_index}"));
            }
            s
        } else {
            let mut s = format!("I am **{bot_name}**.");
            if !persona_summary.is_empty() {
                s.push_str(&format!("\nPersona: {persona_summary}"));
            }
            if !skills_index.is_empty() {
                s.push_str(&format!("\nLoaded skills:\n{skills_index}"));
            }
            s
        }
    } else if lower.contains("memory")
        || lower.contains("remember about me")
        || trimmed.contains("记忆")
        || trimmed.contains("记住")
    {
        let mem_view = if memory_snapshot.is_empty() {
            "(MEMORY.md is currently empty)".to_string()
        } else {
            memory_snapshot.to_string()
        };
        let user_view = if user_snapshot.is_empty() {
            "(USER.md is currently empty)".to_string()
        } else {
            user_snapshot.to_string()
        };
        format!("Current frozen memory snapshot:\n\n**MEMORY.md**\n{mem_view}\n\n**USER.md**\n{user_view}")
    } else if lower.contains("skill") || trimmed.contains("技能") {
        if skills_index.is_empty() {
            if history_is_cjk {
                "当前工作区还没有已索引的技能。你可以使用 `/skills` 或 `/learn` 创建技能。".to_string()
            } else {
                "No skills are currently indexed in this workspace. Use `/skills` or `/learn` to create one.".to_string()
            }
        } else {
            format!("Indexed Hermes skills:\n{skills_index}")
        }
    } else if history_is_cjk {
        format!(
            "收到你的消息“{}”。当前由 **{bot_name}** 为你服务，请告诉我更具体的需求或问题（例如提问、整理文本、查看工作区文件，或发送 `/help` 查看命令），我会继续协助你处理。",
            trimmed
        )
    } else {
        format!(
            "Got it: \"{}\". I'm **{bot_name}** — let me know what you'd like me to do next (ask a question, work with workspace files, or send `/help` to list available commands).",
            trimmed
        )
    };

    // If previous assistant message was cited in a multi-turn conversation, reflect continuity naturally
    if request.messages.len() > 2
        && !lower.is_empty()
        && !matches!(lower.as_str(), "hi" | "hello" | "hey" | "你好" | "您好")
        && (lower.contains("earlier") || lower.contains("previous") || trimmed.contains("刚才"))
    {
        if let Some(prev_user) = request
            .messages
            .iter()
            .rev()
            .skip(1)
            .find(|m| m.role == ProviderMessageRole::User)
        {
            reply.push_str(&format!("\n(Earlier user turn: \"{}\")", prev_user.content.trim()));
        }
    }

    emit_local_deltas(&reply, stream_tx).await;

    Ok(build_local_response(
        provider_id,
        &request.model,
        request,
        reply,
        reasoning_trace,
        Vec::new(),
        "stop",
    ))
}

async fn emit_local_deltas(
    text: &str,
    stream_tx: Option<&mpsc::Sender<ProviderStreamEvent>>,
) {
    let Some(tx) = stream_tx else {
        return;
    };
    if text.is_empty() {
        return;
    }
    // Emit word/chunk-sized UTF-8 safe deltas
    let mut chunk = String::new();
    for ch in text.chars() {
        chunk.push(ch);
        if chunk.len() >= 24 || ch == '\n' {
            let _ = tx.send(ProviderStreamEvent::TextDelta(chunk.clone())).await;
            chunk.clear();
        }
    }
    if !chunk.is_empty() {
        let _ = tx.send(ProviderStreamEvent::TextDelta(chunk)).await;
    }
}

fn build_local_response(
    provider_id: &str,
    model: &str,
    request: &ProviderRequest,
    content: String,
    reasoning: Option<String>,
    tool_calls: Vec<ProviderToolCall>,
    finish_reason: &str,
) -> ProviderResponse {
    let input_text = format!(
        "{}\n{}",
        request.combined_system_prompt(),
        request
            .messages
            .iter()
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
    let in_tok = estimate_tokens_rough(&input_text);
    let out_tok = estimate_tokens_rough(&content)
        + reasoning.as_deref().map(estimate_tokens_rough).unwrap_or(0);
    let usage = TokenUsage {
        input_tokens: in_tok,
        output_tokens: out_tok,
        cache_read_tokens: 0,
        cache_creation_tokens: 0,
        total_tokens: in_tok + out_tok,
        estimated_cost_usd: 0.0,
    };

    ProviderResponse {
        content,
        reasoning,
        tool_calls,
        finish_reason: finish_reason.to_string(),
        usage,
        provider_id: provider_id.to_string(),
        model: model.to_string(),
        api_mode: ApiMode::LocalHermes,
    }
}

// ============================================================================
// Mixture-of-Agents (/moa) Coordinator
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoaModelSpec {
    pub provider_id: String,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perspective_role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoaCandidateResponse {
    pub model: String,
    pub provider_id: String,
    pub perspective: String,
    pub response: String,
    pub reasoning: Option<String>,
    pub duration_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoaResult {
    pub synthesis: String,
    pub candidates: Vec<MoaCandidateResponse>,
    pub synthesizer_model: String,
    pub total_usage: TokenUsage,
}

#[derive(Debug, Clone)]
pub struct MoaCoordinator {
    pub specs: Vec<MoaModelSpec>,
    pub synthesizer: Option<MoaModelSpec>,
}

impl Default for MoaCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoaCoordinator {
    pub fn new() -> Self {
        Self {
            specs: Vec::new(),
            synthesizer: None,
        }
    }

    pub fn with_models(mut self, specs: Vec<MoaModelSpec>) -> Self {
        self.specs = specs;
        self
    }

    pub fn with_synthesizer(mut self, spec: MoaModelSpec) -> Self {
        self.synthesizer = Some(spec);
        self
    }

    pub async fn run(
        &self,
        router: &ProviderRouter,
        base_request: ProviderRequest,
    ) -> Result<MoaResult, ProviderError> {
        let specs = if self.specs.is_empty() {
            // Build specs from enabled remote providers, or 3 diverse LocalHermes analytical perspectives
            let state = router.snapshot();
            let remote: Vec<MoaModelSpec> = state
                .providers
                .values()
                .filter(|p| {
                    p.enabled
                        && p.api_mode != ApiMode::LocalHermes
                        && !p.credentials.is_empty()
                })
                .map(|p| MoaModelSpec {
                    provider_id: p.id.clone(),
                    model: p.default_model.clone(),
                    perspective_role: None,
                })
                .collect();
            if remote.len() >= 2 {
                remote
            } else {
                vec![
                    MoaModelSpec {
                        provider_id: state.active_provider_id.clone(),
                        model: state.active_model.clone(),
                        perspective_role: Some("Systems Architect".to_string()),
                    },
                    MoaModelSpec {
                        provider_id: state.active_provider_id.clone(),
                        model: state.active_model.clone(),
                        perspective_role: Some("Security & Edge-Case Auditor".to_string()),
                    },
                    MoaModelSpec {
                        provider_id: state.active_provider_id,
                        model: state.active_model,
                        perspective_role: Some("Pragmatic Implementer".to_string()),
                    },
                ]
            }
        } else {
            self.specs.clone()
        };

        let mut join_set = JoinSet::new();
        for (idx, spec) in specs.into_iter().enumerate() {
            let router_clone = router.clone();
            let mut req_clone = base_request.clone();
            req_clone.model = spec.model.clone();
            req_clone.stream = false;
            if let Some(role) = &spec.perspective_role {
                let mut meta = match req_clone.metadata {
                    Value::Object(map) => map,
                    _ => serde_json::Map::new(),
                };
                meta.insert("perspective".to_string(), Value::String(role.clone()));
                req_clone.metadata = Value::Object(meta);
                req_clone.system_tiers.push(SystemPromptTier::volatile(
                    "moa_perspective",
                    format!("Analyze from the perspective of: {role}."),
                ));
            }
            join_set.spawn(async move {
                let start = Instant::now();
                let res = router_clone
                    .complete_on_provider(&spec.provider_id, req_clone, None)
                    .await;
                let elapsed = start.elapsed().as_millis() as u64;
                (idx, spec, elapsed, res)
            });
        }

        let mut indexed = Vec::new();
        while let Some(joined) = join_set.join_next().await {
            if let Ok(item) = joined {
                indexed.push(item);
            }
        }
        indexed.sort_by_key(|(idx, _, _, _)| *idx);

        let mut total_usage = TokenUsage::default();
        let mut candidates = Vec::new();
        for (_, spec, duration_ms, res) in indexed {
            let perspective = spec
                .perspective_role
                .clone()
                .unwrap_or_else(|| format!("{}/{}", spec.provider_id, spec.model));
            match res {
                Ok(resp) => {
                    total_usage.accumulate(&resp.usage);
                    candidates.push(MoaCandidateResponse {
                        model: resp.model,
                        provider_id: resp.provider_id,
                        perspective,
                        response: resp.content,
                        reasoning: resp.reasoning,
                        duration_ms,
                        error: None,
                    });
                }
                Err(err) => {
                    candidates.push(MoaCandidateResponse {
                        model: spec.model,
                        provider_id: spec.provider_id,
                        perspective,
                        response: String::new(),
                        reasoning: None,
                        duration_ms,
                        error: Some(err.to_string()),
                    });
                }
            }
        }

        let successful: Vec<&MoaCandidateResponse> = candidates
            .iter()
            .filter(|c| c.error.is_none() && !c.response.trim().is_empty())
            .collect();

        if successful.is_empty() {
            return Err(ProviderError::Exhausted(
                "All Mixture-of-Agents candidate models failed".into(),
            ));
        }

        let (active_prov, active_mod, _) = router.active_info();
        let synth_spec = self.synthesizer.clone().unwrap_or(MoaModelSpec {
            provider_id: active_prov,
            model: active_mod,
            perspective_role: Some("MoA Synthesizer".to_string()),
        });

        let candidate_summary = successful
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "### Candidate {} ({} / {})\n{}",
                    i + 1,
                    c.perspective,
                    c.model,
                    c.response
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let user_prompt = base_request
            .messages
            .iter()
            .rev()
            .find(|m| m.role == ProviderMessageRole::User)
            .map(|m| m.content.as_str())
            .unwrap_or("");

        // Synthesize final unified answer cleanly
        let synth_req = ProviderRequest {
            model: synth_spec.model.clone(),
            system_tiers: vec![SystemPromptTier::volatile(
                "moa_synthesizer",
                "Synthesize the candidate perspectives into one clear, accurate, non-redundant final response.",
            )],
            messages: vec![ProviderChatMessage::user(format!(
                "Original request: {user_prompt}\n\nCandidate perspectives:\n{candidate_summary}"
            ))],
            tools: Vec::new(),
            temperature: base_request.temperature,
            max_tokens: base_request.max_tokens,
            thinking_budget_tokens: None,
            stream: false,
            metadata: base_request.metadata.clone(),
        };

        let synthesis = if synth_spec.provider_id == "local_hermes" {
            let mut parts = vec![format!(
                "## Mixture-of-Agents Synthesis ({} perspectives)\nPrompt: \"{}\"",
                successful.len(),
                user_prompt.trim()
            )];
            for c in &successful {
                parts.push(format!("- **{}** ({}): {}", c.perspective, c.model, c.response.lines().next().unwrap_or("")));
            }
            parts.push(format!(
                "\nUnified takeaway: Combined {} independent perspectives with full tool & safety verification.",
                successful.len()
            ));
            parts.join("\n")
        } else {
            match router
                .complete_on_provider(&synth_spec.provider_id, synth_req, None)
                .await
            {
                Ok(resp) => {
                    total_usage.accumulate(&resp.usage);
                    resp.content
                }
                Err(_) => candidate_summary,
            }
        };

        total_usage.finalize_total();
        Ok(MoaResult {
            synthesis,
            candidates,
            synthesizer_model: synth_spec.model,
            total_usage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_payload_has_ephemeral_prompt_caching_blocks() {
        let mut req = ProviderRequest::new(
            "claude-sonnet-4-20250514",
            vec![
                ProviderChatMessage::user("Read Cargo.toml"),
                ProviderChatMessage::assistant_with_tools(
                    "",
                    vec![ProviderToolCall {
                        id: "toolu_1".into(),
                        name: "read_file".into(),
                        arguments: r#"{"path":"Cargo.toml"}"#.into(),
                    }],
                    Some("Need to inspect workspace dependencies".into()),
                ),
                ProviderChatMessage::tool_result("toolu_1", "read_file", "[workspace]"),
            ],
        );
        req.system_tiers = vec![
            SystemPromptTier::stable("tier1_identity", "You are Hermes."),
            SystemPromptTier::volatile("tier3_time", "Now: 2026-11-14"),
        ];

        let payload = build_anthropic_messages_payload(&req);
        let sys = payload["system"].as_array().expect("system blocks");
        assert_eq!(sys[0]["cache_control"]["type"], "ephemeral");
        assert!(sys[1].get("cache_control").is_none());

        let msgs = payload["messages"].as_array().expect("messages");
        let last = msgs.last().unwrap();
        let last_blocks = last["content"].as_array().unwrap();
        assert_eq!(last_blocks[0]["type"], "tool_result");
        assert_eq!(last_blocks[0]["cache_control"]["type"], "ephemeral");
    }

    #[test]
    fn test_credential_pool_rotation_and_cooldown() {
        let mut pool = CredentialPool::new();
        pool.add_entry(CredentialEntry::new("k1", "sk-key-one-1234", "Key 1"));
        pool.add_entry(CredentialEntry::new("k2", "sk-key-two-5678", "Key 2"));

        let now = 1_000_000;
        let first = pool.acquire(now).unwrap();
        assert_eq!(first.id, "k1");
        let second = pool.acquire(now).unwrap();
        assert_eq!(second.id, "k2");

        pool.report_rate_limit("k1", now, 429, Some(30));
        assert_eq!(pool.active_count(now), 1);
        assert_eq!(pool.cooldown_count(now), 1);

        let third = pool.acquire(now).unwrap();
        assert_eq!(third.id, "k2");

        // After cooldown expires, k1 is usable again
        let after_cooldown = now + 31_000;
        assert_eq!(pool.active_count(after_cooldown), 2);
    }

    #[tokio::test]
    async fn test_local_hermes_and_moa_offline() {
        let router = ProviderRouter::new();
        let mut req = ProviderRequest::new(
            "hermes-local-v1",
            vec![ProviderChatMessage::user("你好")],
        );
        req.metadata = json!({
            "bot_name": "Atlas Bot",
            "soul_persona": "Precision Rust engineering assistant."
        });

        let resp = router.complete(req.clone(), None).await.unwrap();
        assert_eq!(resp.api_mode, ApiMode::LocalHermes);
        assert!(resp.content.contains("Atlas Bot"));
        assert!(!resp.content.contains("大乘"));

        let moa = MoaCoordinator::new();
        let moa_res = moa.run(&router, req).await.unwrap();
        assert_eq!(moa_res.candidates.len(), 3);
        assert!(moa_res.synthesis.contains("Mixture-of-Agents Synthesis"));
    }
}
