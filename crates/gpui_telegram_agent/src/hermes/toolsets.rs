use crate::computer::ComputerLeaseBroker;
use crate::mcp::{McpRegistry, ToolRiskLevel};
use crate::talk::TalkCoordinator;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use super::security::{
    ApprovalDecisionAction, ApprovalMode, PendingApproval, SecurityEngine, TirVerdict,
};
use super::tools_agentic::{execute_agentic_tool, LocalAgenticStore};
use super::tools_file::{execute_file_tool, FileCheckpointManager, TodoStore};
use super::tools_terminal::{execute_terminal_tool, ProcessRegistry, TerminalBackendConfig};
use super::tools_web_media::execute_web_media_tool;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// All 28 Hermes toolsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolsetId {
    Core,
    Web,
    Terminal,
    File,
    Memory,
    Skills,
    Vision,
    ImageGen,
    VideoGen,
    Browser,
    AudioTts,
    AudioStt,
    Cronjob,
    Subagents,
    CodeExecution,
    Clarify,
    Todo,
    Planning,
    Reasoning,
    Mcp,
    Kanban,
    DesktopUi,
    ComputerUse,
    Connections,
    Honcho,
    SessionSearch,
    Workspace,
    Voice,
}

impl ToolsetId {
    pub const ALL: [ToolsetId; 28] = [
        Self::Core,
        Self::Web,
        Self::Terminal,
        Self::File,
        Self::Memory,
        Self::Skills,
        Self::Vision,
        Self::ImageGen,
        Self::VideoGen,
        Self::Browser,
        Self::AudioTts,
        Self::AudioStt,
        Self::Cronjob,
        Self::Subagents,
        Self::CodeExecution,
        Self::Clarify,
        Self::Todo,
        Self::Planning,
        Self::Reasoning,
        Self::Mcp,
        Self::Kanban,
        Self::DesktopUi,
        Self::ComputerUse,
        Self::Connections,
        Self::Honcho,
        Self::SessionSearch,
        Self::Workspace,
        Self::Voice,
    ];

    pub fn all() -> &'static [ToolsetId] {
        &Self::ALL
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Web => "web",
            Self::Terminal => "terminal",
            Self::File => "file",
            Self::Memory => "memory",
            Self::Skills => "skills",
            Self::Vision => "vision",
            Self::ImageGen => "image_gen",
            Self::VideoGen => "video_gen",
            Self::Browser => "browser",
            Self::AudioTts => "audio_tts",
            Self::AudioStt => "audio_stt",
            Self::Cronjob => "cronjob",
            Self::Subagents => "subagents",
            Self::CodeExecution => "code_execution",
            Self::Clarify => "clarify",
            Self::Todo => "todo",
            Self::Planning => "planning",
            Self::Reasoning => "reasoning",
            Self::Mcp => "mcp",
            Self::Kanban => "kanban",
            Self::DesktopUi => "desktop_ui",
            Self::ComputerUse => "computer_use",
            Self::Connections => "connections",
            Self::Honcho => "honcho",
            Self::SessionSearch => "session_search",
            Self::Workspace => "workspace",
            Self::Voice => "voice",
        }
    }

    pub fn from_str(raw: &str) -> Option<Self> {
        let norm = raw.trim().to_ascii_lowercase().replace('-', "_");
        match norm.as_str() {
            "core" => Some(Self::Core),
            "web" => Some(Self::Web),
            "terminal" | "shell" => Some(Self::Terminal),
            "file" | "files" => Some(Self::File),
            "memory" => Some(Self::Memory),
            "skills" => Some(Self::Skills),
            "vision" => Some(Self::Vision),
            "image_gen" | "imagegen" => Some(Self::ImageGen),
            "video_gen" | "videogen" => Some(Self::VideoGen),
            "browser" => Some(Self::Browser),
            "audio_tts" | "audiotts" | "tts" => Some(Self::AudioTts),
            "audio_stt" | "audiostt" | "stt" => Some(Self::AudioStt),
            "cronjob" | "cron" => Some(Self::Cronjob),
            "subagents" | "delegate" => Some(Self::Subagents),
            "code_execution" | "codeexecution" => Some(Self::CodeExecution),
            "clarify" => Some(Self::Clarify),
            "todo" => Some(Self::Todo),
            "planning" | "plan" => Some(Self::Planning),
            "reasoning" | "think" => Some(Self::Reasoning),
            "mcp" => Some(Self::Mcp),
            "kanban" => Some(Self::Kanban),
            "desktop_ui" | "desktopui" => Some(Self::DesktopUi),
            "computer_use" | "computeruse" | "cua" => Some(Self::ComputerUse),
            "connections" => Some(Self::Connections),
            "honcho" => Some(Self::Honcho),
            "session_search" | "sessionsearch" => Some(Self::SessionSearch),
            "workspace" => Some(Self::Workspace),
            "voice" | "talk" => Some(Self::Voice),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Core => "Toolset introspection and dynamic toolset enable/disable controls",
            Self::Web => "Web search across providers and HTML-to-Markdown page extraction",
            Self::Terminal => "Foreground & background shell execution across 6 terminal backends",
            Self::File => "Workspace file reading, writing, fuzzy patching, regex search, and listing",
            Self::Memory => "Bounded dual-store persistent MEMORY.md and USER.md management",
            Self::Skills => "agentskills.io SKILL.md creation, patching, listing, and security scanning",
            Self::Vision => "Multimodal image inspection and metadata probing",
            Self::ImageGen => "Image generation via provider APIs or deterministic local synthesis",
            Self::VideoGen => "Video generation adapter and storyboard synthesis",
            Self::Browser => "Chrome DevTools Protocol browser navigation, snapshot, click, type, screenshot",
            Self::AudioTts => "Text-to-speech synthesis with local PCM/RIFF WAVE fallback",
            Self::AudioStt => "Speech-to-text transcription and WAV audio inspection",
            Self::Cronjob => "Scheduled recurring automation jobs, history, and immediate triggers",
            Self::Subagents => "Depth-bounded concurrent subagent task delegation and synthesis",
            Self::CodeExecution => "Sandboxed expression evaluation and RPC subprocess code runner",
            Self::Clarify => "Interactive user clarification prompts with Telegram inline buttons",
            Self::Todo => "Structured per-session task checklist management",
            Self::Planning => "Structured multi-step execution planning",
            Self::Reasoning => "Explicit scratchpad thinking and structured reasoning steps",
            Self::Mcp => "Model Context Protocol (MCP) tool discovery and execution bridge",
            Self::Kanban => "Full 14-tool Kanban boards, cards, comments, checklists, and search",
            Self::DesktopUi => "GPUI Desktop panel inspection and workbench UI action emission",
            Self::ComputerUse => "Generation-fenced Remote Computer (CUA) lease and input control",
            Self::Connections => "External integration credential connection management",
            Self::Honcho => "Dialectic user modeling, observation recording, and profile queries",
            Self::SessionSearch => "Full-text and CJK fallback search across past conversation sessions",
            Self::Workspace => "Workspace file checkpoint inspection and atomic turn rollback",
            Self::Voice => "Full-duplex Voice / Talk Mode coordinator state control",
        }
    }
}

/// Built-in Hermes toolset preset bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolsetPreset {
    HermesCli,
    HermesTelegram,
    HermesAcp,
    Minimal,
    Safe,
    Full,
}

impl ToolsetPreset {
    pub fn from_str(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "hermes-cli" | "cli" => Some(Self::HermesCli),
            "hermes-telegram" | "telegram" => Some(Self::HermesTelegram),
            "hermes-acp" | "acp" => Some(Self::HermesAcp),
            "minimal" => Some(Self::Minimal),
            "safe" => Some(Self::Safe),
            "full" | "all" => Some(Self::Full),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HermesCli => "hermes-cli",
            Self::HermesTelegram => "hermes-telegram",
            Self::HermesAcp => "hermes-acp",
            Self::Minimal => "minimal",
            Self::Safe => "safe",
            Self::Full => "full",
        }
    }

    pub fn toolsets(&self) -> Vec<ToolsetId> {
        match self {
            Self::HermesCli => vec![
                ToolsetId::Core,
                ToolsetId::File,
                ToolsetId::Workspace,
                ToolsetId::Terminal,
                ToolsetId::CodeExecution,
                ToolsetId::Web,
                ToolsetId::Browser,
                ToolsetId::Memory,
                ToolsetId::Skills,
                ToolsetId::Todo,
                ToolsetId::Planning,
                ToolsetId::Reasoning,
                ToolsetId::Subagents,
                ToolsetId::SessionSearch,
                ToolsetId::Cronjob,
                ToolsetId::Mcp,
                ToolsetId::Clarify,
                ToolsetId::Vision,
                ToolsetId::Kanban,
            ],
            Self::HermesTelegram => vec![
                ToolsetId::Core,
                ToolsetId::File,
                ToolsetId::Workspace,
                ToolsetId::Web,
                ToolsetId::Memory,
                ToolsetId::Skills,
                ToolsetId::AudioTts,
                ToolsetId::AudioStt,
                ToolsetId::Voice,
                ToolsetId::Vision,
                ToolsetId::ImageGen,
                ToolsetId::VideoGen,
                ToolsetId::Cronjob,
                ToolsetId::Kanban,
                ToolsetId::Clarify,
                ToolsetId::Todo,
                ToolsetId::Planning,
                ToolsetId::Reasoning,
                ToolsetId::Subagents,
                ToolsetId::SessionSearch,
                ToolsetId::Honcho,
                ToolsetId::Connections,
                ToolsetId::ComputerUse,
                ToolsetId::DesktopUi,
                ToolsetId::Mcp,
                ToolsetId::Terminal,
                ToolsetId::CodeExecution,
            ],
            Self::HermesAcp => vec![
                ToolsetId::Core,
                ToolsetId::File,
                ToolsetId::Workspace,
                ToolsetId::Terminal,
                ToolsetId::CodeExecution,
                ToolsetId::Memory,
                ToolsetId::Skills,
                ToolsetId::Todo,
                ToolsetId::Planning,
                ToolsetId::Reasoning,
                ToolsetId::Mcp,
                ToolsetId::Subagents,
                ToolsetId::SessionSearch,
                ToolsetId::DesktopUi,
            ],
            Self::Minimal => vec![ToolsetId::Core, ToolsetId::File, ToolsetId::Reasoning],
            Self::Safe => vec![
                ToolsetId::Core,
                ToolsetId::Web,
                ToolsetId::Memory,
                ToolsetId::Skills,
                ToolsetId::Todo,
                ToolsetId::Planning,
                ToolsetId::Reasoning,
                ToolsetId::Clarify,
                ToolsetId::SessionSearch,
                ToolsetId::Kanban,
                ToolsetId::Honcho,
            ],
            Self::Full => ToolsetId::ALL.to_vec(),
        }
    }
}

/// User-defined composite toolset definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomToolset {
    pub name: String,
    pub description: String,
    pub toolsets: Vec<ToolsetId>,
    #[serde(default)]
    pub included_tools: Vec<String>,
    #[serde(default)]
    pub excluded_tools: Vec<String>,
}

/// Manages active toolsets, presets, custom toolset compositions, and per-tool enablement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsetRegistry {
    pub active_preset: Option<ToolsetPreset>,
    pub enabled_toolsets: BTreeSet<ToolsetId>,
    pub custom_toolsets: HashMap<String, CustomToolset>,
    pub active_custom_toolset: Option<String>,
    pub disabled_tools: HashSet<String>,
    pub extra_enabled_tools: HashSet<String>,
}

impl Default for ToolsetRegistry {
    fn default() -> Self {
        Self::from_preset(ToolsetPreset::Full)
    }
}

impl ToolsetRegistry {
    pub fn from_preset(preset: ToolsetPreset) -> Self {
        let enabled_toolsets: BTreeSet<ToolsetId> = preset.toolsets().into_iter().collect();
        Self {
            active_preset: Some(preset),
            enabled_toolsets,
            custom_toolsets: HashMap::new(),
            active_custom_toolset: None,
            disabled_tools: HashSet::new(),
            extra_enabled_tools: HashSet::new(),
        }
    }

    pub fn from_toolsets(toolsets: impl IntoIterator<Item = ToolsetId>) -> Self {
        let mut enabled: BTreeSet<ToolsetId> = toolsets.into_iter().collect();
        enabled.insert(ToolsetId::Core);
        Self {
            active_preset: None,
            enabled_toolsets: enabled,
            custom_toolsets: HashMap::new(),
            active_custom_toolset: None,
            disabled_tools: HashSet::new(),
            extra_enabled_tools: HashSet::new(),
        }
    }

    pub fn apply_preset(&mut self, preset: ToolsetPreset) {
        self.active_preset = Some(preset);
        self.active_custom_toolset = None;
        self.enabled_toolsets = preset.toolsets().into_iter().collect();
        self.disabled_tools.clear();
        self.extra_enabled_tools.clear();
    }

    pub fn register_custom_toolset(&mut self, custom: CustomToolset) {
        self.custom_toolsets.insert(custom.name.clone(), custom);
    }

    pub fn apply_custom_toolset(&mut self, name: &str) -> Result<(), String> {
        let custom = self
            .custom_toolsets
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Custom toolset '{name}' not found"))?;
        self.active_preset = None;
        self.active_custom_toolset = Some(custom.name);
        self.enabled_toolsets = custom.toolsets.into_iter().collect();
        self.enabled_toolsets.insert(ToolsetId::Core);
        self.extra_enabled_tools = custom.included_tools.into_iter().collect();
        self.disabled_tools = custom.excluded_tools.into_iter().collect();
        Ok(())
    }

    pub fn enable(&mut self, toolset: ToolsetId) -> bool {
        self.enabled_toolsets.insert(toolset)
    }

    pub fn disable(&mut self, toolset: ToolsetId) -> bool {
        self.enabled_toolsets.remove(&toolset)
    }

    pub fn enable_by_name(&mut self, name: &str) -> Result<Vec<ToolsetId>, String> {
        if let Some(preset) = ToolsetPreset::from_str(name) {
            self.apply_preset(preset);
            return Ok(self.enabled_toolsets.iter().copied().collect());
        }
        if self.custom_toolsets.contains_key(name) {
            self.apply_custom_toolset(name)?;
            return Ok(self.enabled_toolsets.iter().copied().collect());
        }
        let id = ToolsetId::from_str(name)
            .ok_or_else(|| format!("Unknown toolset or preset '{name}'"))?;
        self.enable(id);
        Ok(vec![id])
    }

    pub fn disable_by_name(&mut self, name: &str) -> Result<ToolsetId, String> {
        let id = ToolsetId::from_str(name)
            .ok_or_else(|| format!("Unknown toolset '{name}'"))?;
        if id == ToolsetId::Core {
            return Err("Cannot disable 'core' toolset".into());
        }
        self.disable(id);
        Ok(id)
    }

    pub fn is_toolset_enabled(&self, id: ToolsetId) -> bool {
        self.enabled_toolsets.contains(&id)
    }

    pub fn is_tool_enabled(&self, spec: &HermesToolSpec) -> bool {
        if self.disabled_tools.contains(&spec.name) {
            return false;
        }
        if self.extra_enabled_tools.contains(&spec.name) {
            return true;
        }
        self.enabled_toolsets.contains(&spec.toolset)
    }

    pub fn active_specs(&self) -> Vec<HermesToolSpec> {
        all_builtin_tool_specs()
            .into_iter()
            .filter(|spec| self.is_tool_enabled(spec))
            .collect()
    }

    pub fn summary_json(&self) -> Value {
        let all_specs = all_builtin_tool_specs();
        let toolsets: Vec<Value> = ToolsetId::ALL
            .iter()
            .map(|id| {
                let tools: Vec<&str> = all_specs
                    .iter()
                    .filter(|s| s.toolset == *id)
                    .map(|s| s.name.as_str())
                    .collect();
                json!({
                    "id": id.as_str(),
                    "enabled": self.enabled_toolsets.contains(id),
                    "description": id.description(),
                    "tools": tools,
                })
            })
            .collect();
        json!({
            "active_preset": self.active_preset.map(|p| p.as_str()),
            "active_custom_toolset": self.active_custom_toolset,
            "enabled_count": self.enabled_toolsets.len(),
            "total_toolsets": ToolsetId::ALL.len(),
            "active_tool_count": self.active_specs().len(),
            "toolsets": toolsets,
        })
    }
}

/// Complete specification of a Hermes tool, capable of emitting OpenAI & Anthropic JSON schemas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HermesToolSpec {
    pub name: String,
    pub toolset: ToolsetId,
    pub description: String,
    pub parameters_schema: Value,
    pub risk_level: ToolRiskLevel,
    pub requires_approval: bool,
    pub read_only: bool,
}

impl HermesToolSpec {
    pub fn new(
        name: impl Into<String>,
        toolset: ToolsetId,
        description: impl Into<String>,
        parameters_schema: Value,
        risk_level: ToolRiskLevel,
        read_only: bool,
    ) -> Self {
        let requires_approval = risk_level.requires_explicit_confirmation();
        Self {
            name: name.into(),
            toolset,
            description: description.into(),
            parameters_schema,
            risk_level,
            requires_approval,
            read_only,
        }
    }

    /// Generates an OpenAI `/v1/chat/completions` function calling schema object.
    pub fn to_openai_tool(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": self.parameters_schema,
            }
        })
    }

    /// Generates an Anthropic `/v1/messages` tool schema object.
    pub fn to_anthropic_tool(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "input_schema": self.parameters_schema,
        })
    }
}

pub fn export_openai_tools(specs: &[HermesToolSpec]) -> Vec<Value> {
    specs.iter().map(HermesToolSpec::to_openai_tool).collect()
}

pub fn export_anthropic_tools(specs: &[HermesToolSpec]) -> Vec<Value> {
    specs.iter().map(HermesToolSpec::to_anthropic_tool).collect()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HermesToolCall {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HermesToolArtifact {
    pub kind: String,
    pub name: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_base64: Option<String>,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HermesUiAction {
    pub action_type: String,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HermesInlineButton {
    pub label: String,
    pub callback_data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HermesToolResult {
    pub tool_call_id: String,
    pub tool_name: String,
    pub success: bool,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<HermesToolArtifact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ui_actions: Vec<HermesUiAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inline_buttons: Vec<HermesInlineButton>,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl HermesToolResult {
    pub fn ok(
        tool_name: impl Into<String>,
        output: impl Into<String>,
        structured: Option<Value>,
    ) -> Self {
        Self {
            tool_call_id: String::new(),
            tool_name: tool_name.into(),
            success: true,
            output: output.into(),
            structured,
            artifacts: Vec::new(),
            ui_actions: Vec::new(),
            inline_buttons: Vec::new(),
            duration_ms: 0,
            error: None,
        }
    }

    pub fn err(tool_name: impl Into<String>, message: impl Into<String>) -> Self {
        let msg = message.into();
        Self {
            tool_call_id: String::new(),
            tool_name: tool_name.into(),
            success: false,
            output: format!("Error: {msg}"),
            structured: Some(json!({ "error": msg })),
            artifacts: Vec::new(),
            ui_actions: Vec::new(),
            inline_buttons: Vec::new(),
            duration_ms: 0,
            error: Some(msg),
        }
    }

    pub fn with_call_id(mut self, id: impl Into<String>) -> Self {
        self.tool_call_id = id.into();
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_artifact(mut self, artifact: HermesToolArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn with_ui_action(mut self, action: HermesUiAction) -> Self {
        self.ui_actions.push(action);
        self
    }

    pub fn with_inline_buttons(mut self, buttons: Vec<HermesInlineButton>) -> Self {
        self.inline_buttons.extend(buttons);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubagentDelegationRequest {
    pub task_id: String,
    pub parent_session_id: String,
    pub role: String,
    pub prompt: String,
    pub toolsets: Vec<ToolsetId>,
    pub model: Option<String>,
    pub depth: usize,
    pub max_depth: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubagentDelegationOutcome {
    pub task_id: String,
    pub role: String,
    pub success: bool,
    pub summary: String,
    pub tool_calls_used: usize,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClarifyPrompt {
    pub prompt_id: String,
    pub question: String,
    pub options: Vec<String>,
    pub allow_free_text: bool,
    pub multi_select: bool,
}

/// Decoupled async bridge allowing `hermes-runtime` to wire `HermesStateStore`,
/// `MemoryManager`, `SkillsManager`, real subagents, and UI event sinks into tools
/// without creating circular module dependencies.
pub trait HermesStateBridge: Send + Sync {
    fn memory_op<'a>(&'a self, args: &'a Value) -> BoxFuture<'a, Result<Value, String>>;
    fn skill_op<'a>(&'a self, args: &'a Value) -> BoxFuture<'a, Result<Value, String>>;
    fn session_search<'a>(
        &'a self,
        query: &'a str,
        limit: usize,
    ) -> BoxFuture<'a, Result<Value, String>>;
    fn honcho_op<'a>(&'a self, args: &'a Value) -> BoxFuture<'a, Result<Value, String>>;
    fn cron_op<'a>(&'a self, args: &'a Value) -> BoxFuture<'a, Result<Value, String>>;
    fn kanban_op<'a>(&'a self, action: &'a str, args: &'a Value) -> BoxFuture<'a, Result<Value, String>>;
    fn delegate_subagent<'a>(
        &'a self,
        req: SubagentDelegationRequest,
    ) -> BoxFuture<'a, Result<SubagentDelegationOutcome, String>>;
    fn emit_ui_action<'a>(&'a self, action: HermesUiAction) -> BoxFuture<'a, Result<Value, String>>;
    fn record_checkpoint<'a>(
        &'a self,
        _session_id: &'a str,
        _turn_index: u64,
        _file_path: &'a str,
        _original_content: Option<&'a [u8]>,
    ) -> BoxFuture<'a, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
    fn record_approval<'a>(&'a self, _approval: PendingApproval) -> BoxFuture<'a, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
}

pub type ApprovalCallback = Arc<
    dyn Fn(&PendingApproval) -> Option<ApprovalDecisionAction> + Send + Sync,
>;
pub type UiActionCallback = Arc<dyn Fn(&HermesUiAction) + Send + Sync>;

/// Execution context passed to every Hermes tool invocation.
#[derive(Clone)]
pub struct HermesToolContext {
    pub workspace_root: PathBuf,
    pub session_id: String,
    pub chat_id: Option<i64>,
    pub bot_id: Option<i64>,
    pub turn_index: u64,
    pub caller_depth: usize,
    pub max_depth: usize,
    pub max_concurrent_subagents: usize,
    pub security: Arc<RwLock<SecurityEngine>>,
    pub toolsets: Arc<RwLock<ToolsetRegistry>>,
    pub checkpoints: Arc<Mutex<FileCheckpointManager>>,
    pub processes: Arc<ProcessRegistry>,
    pub mcp_registry: Arc<RwLock<McpRegistry>>,
    pub computer_broker: Arc<Mutex<ComputerLeaseBroker>>,
    pub talk_coordinator: Arc<Mutex<TalkCoordinator>>,
    pub todos: Arc<Mutex<TodoStore>>,
    pub agentic_store: Arc<Mutex<LocalAgenticStore>>,
    pub cancellation: Arc<AtomicBool>,
    pub approval_callback: Option<ApprovalCallback>,
    pub ui_action_emitter: Option<UiActionCallback>,
    pub state_bridge: Option<Arc<dyn HermesStateBridge>>,
    pub http_client: reqwest::Client,
    pub browser_cdp_url: Option<String>,
    pub terminal_backend: TerminalBackendConfig,
}

impl HermesToolContext {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        let root = workspace_root.into();
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .user_agent("gpui_telegram_hermes/1.0")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            workspace_root: root.clone(),
            session_id: "default".into(),
            chat_id: None,
            bot_id: None,
            turn_index: 1,
            caller_depth: 0,
            max_depth: 3,
            max_concurrent_subagents: 4,
            security: Arc::new(RwLock::new(SecurityEngine::new(root))),
            toolsets: Arc::new(RwLock::new(ToolsetRegistry::from_preset(ToolsetPreset::Full))),
            checkpoints: Arc::new(Mutex::new(FileCheckpointManager::new())),
            processes: Arc::new(ProcessRegistry::new()),
            mcp_registry: Arc::new(RwLock::new(McpRegistry::new())),
            computer_broker: Arc::new(Mutex::new(ComputerLeaseBroker::new())),
            talk_coordinator: Arc::new(Mutex::new(TalkCoordinator::new())),
            todos: Arc::new(Mutex::new(TodoStore::new())),
            agentic_store: Arc::new(Mutex::new(LocalAgenticStore::new())),
            cancellation: Arc::new(AtomicBool::new(false)),
            approval_callback: None,
            ui_action_emitter: None,
            state_bridge: None,
            http_client,
            browser_cdp_url: std::env::var("HERMES_BROWSER_CDP_URL").ok(),
            terminal_backend: TerminalBackendConfig::default(),
        }
    }

    pub fn with_session(mut self, session_id: impl Into<String>, turn_index: u64) -> Self {
        self.session_id = session_id.into();
        self.turn_index = turn_index;
        self
    }

    pub fn with_chat_and_bot(mut self, chat_id: Option<i64>, bot_id: Option<i64>) -> Self {
        self.chat_id = chat_id;
        self.bot_id = bot_id;
        self
    }

    pub fn with_preset(mut self, preset: ToolsetPreset) -> Self {
        self.toolsets = Arc::new(RwLock::new(ToolsetRegistry::from_preset(preset)));
        self
    }

    pub fn with_approval_mode(self, mode: ApprovalMode) -> Self {
        if let Ok(mut sec) = self.security.write() {
            sec.approvals.default_mode = mode;
            sec.approvals.set_session_mode(self.session_id.clone(), mode);
        }
        self
    }

    pub fn with_state_bridge(mut self, bridge: Arc<dyn HermesStateBridge>) -> Self {
        self.state_bridge = Some(bridge);
        self
    }

    pub fn with_mcp_registry(mut self, registry: Arc<RwLock<McpRegistry>>) -> Self {
        self.mcp_registry = registry;
        self
    }

    pub fn with_ui_emitter(mut self, emitter: UiActionCallback) -> Self {
        self.ui_action_emitter = Some(emitter);
        self
    }

    pub fn with_approval_callback(mut self, cb: ApprovalCallback) -> Self {
        self.approval_callback = Some(cb);
        self
    }

    pub fn with_browser_cdp_url(mut self, url: impl Into<String>) -> Self {
        self.browser_cdp_url = Some(url.into());
        self
    }

    pub fn cancel(&self) {
        self.cancellation.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation.load(Ordering::SeqCst)
    }

    /// Creates an isolated child context for subagent delegation.
    pub fn child_for_subagent(
        &self,
        child_session_id: impl Into<String>,
        allowed_toolsets: Option<&[ToolsetId]>,
    ) -> Self {
        let mut child = self.clone();
        child.session_id = child_session_id.into();
        child.caller_depth = self.caller_depth.saturating_add(1);
        child.cancellation = Arc::new(AtomicBool::new(false));
        if let Some(ts) = allowed_toolsets {
            child.toolsets = Arc::new(RwLock::new(ToolsetRegistry::from_toolsets(ts.iter().copied())));
        }
        child
    }

    pub fn emit_ui(&self, action: &HermesUiAction) {
        if let Some(cb) = &self.ui_action_emitter {
            cb(action);
        }
        if let Ok(mut store) = self.agentic_store.lock() {
            store.ui_action_log.push(action.clone());
        }
    }
}

/// Complete catalog of all 58 Hermes tools across all 28 toolsets.
pub fn all_builtin_tool_specs() -> Vec<HermesToolSpec> {
    vec![
        // 1. Core (3)
        HermesToolSpec::new(
            "toolset_list",
            ToolsetId::Core,
            "List all 28 Hermes toolsets, their enabled status, and included tools.",
            json!({
                "type": "object",
                "properties": {
                    "only_enabled": { "type": "boolean", "description": "If true, list only enabled toolsets" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "toolset_enable",
            ToolsetId::Core,
            "Enable a Hermes toolset, preset bundle, or custom toolset by name.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Toolset, preset, or custom toolset name" }
                },
                "required": ["name"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "toolset_disable",
            ToolsetId::Core,
            "Disable a Hermes toolset by name.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Toolset name to disable" }
                },
                "required": ["name"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        // 2. File (5) & Workspace (1)
        HermesToolSpec::new(
            "read_file",
            ToolsetId::File,
            "Read a workspace file with line numbers, optional line range/offset/limit, and binary protection.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "start_line": { "type": "integer", "minimum": 1 },
                    "end_line": { "type": "integer", "minimum": 1 },
                    "offset": { "type": "integer", "minimum": 1 },
                    "limit": { "type": "integer", "minimum": 1 },
                    "with_line_numbers": { "type": "boolean" }
                },
                "required": ["path"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "write_file",
            ToolsetId::File,
            "Create or overwrite a workspace file with automatic pre-write checkpoint snapshot for rollback.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" },
                    "append": { "type": "boolean" }
                },
                "required": ["path", "content"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "patch",
            ToolsetId::File,
            "Apply exact/fuzzy whitespace search-and-replace or V4A/unified patch hunks with automatic rollback checkpoint.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "old_text": { "type": "string" },
                    "new_text": { "type": "string" },
                    "patch": { "type": "string", "description": "Optional V4A or unified diff patch text" },
                    "replace_all": { "type": "boolean" }
                },
                "required": ["path"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "search_files",
            ToolsetId::File,
            "Pure-Rust regex content search and glob file finder respecting workspace ignore rules.",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "path": { "type": "string" },
                    "glob": { "type": "string" },
                    "mode": { "type": "string", "enum": ["content", "files"] },
                    "case_insensitive": { "type": "boolean" },
                    "is_regex": { "type": "boolean" },
                    "max_results": { "type": "integer", "minimum": 1 }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "list_dir",
            ToolsetId::File,
            "List directory contents sorted by directories then files.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "recursive": { "type": "boolean" },
                    "max_depth": { "type": "integer", "minimum": 1 },
                    "include_hidden": { "type": "boolean" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "workspace_rollback",
            ToolsetId::Workspace,
            "Inspect file checkpoints or atomically roll back modified/created files for a turn.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["rollback", "list"] },
                    "turn_index": { "type": "integer" }
                }
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        // 3. Todo & Planning (3)
        HermesToolSpec::new(
            "todo_write",
            ToolsetId::Todo,
            "Update or replace the structured session task checklist.",
            json!({
                "type": "object",
                "properties": {
                    "todos": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "content": { "type": "string" },
                                "status": { "type": "string", "enum": ["pending", "in_progress", "completed", "cancelled"] },
                                "priority": { "type": "string" }
                            },
                            "required": ["content", "status"]
                        }
                    }
                },
                "required": ["todos"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "todo_read",
            ToolsetId::Todo,
            "Read the current structured session task checklist.",
            json!({ "type": "object", "properties": {} }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "plan_create",
            ToolsetId::Planning,
            "Create a structured implementation plan with ordered steps and optional todo sync.",
            json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "goal": { "type": "string" },
                    "steps": { "type": "array", "items": { "type": "string" } },
                    "sync_todos": { "type": "boolean" }
                },
                "required": ["title", "steps"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        // 4. Terminal & Code Execution (6)
        HermesToolSpec::new(
            "terminal",
            ToolsetId::Terminal,
            "Execute a shell command across Local/Docker/SSH/Singularity/Modal/Daytona backends with TIR & danger gates.",
            json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" },
                    "workdir": { "type": "string" },
                    "timeout_ms": { "type": "integer", "minimum": 100 },
                    "background": { "type": "boolean" },
                    "backend": { "type": "string", "enum": ["local", "docker", "ssh", "singularity", "modal", "daytona"] },
                    "max_output_bytes": { "type": "integer" }
                },
                "required": ["command"]
            }),
            ToolRiskLevel::High,
            false,
        ),
        HermesToolSpec::new(
            "process_list",
            ToolsetId::Terminal,
            "List active and recently exited background processes from the ProcessRegistry.",
            json!({ "type": "object", "properties": {} }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "process_poll",
            ToolsetId::Terminal,
            "Poll stdout/stderr output and exit status for a background process.",
            json!({
                "type": "object",
                "properties": {
                    "process_id": { "type": "string" },
                    "wait_ms": { "type": "integer" }
                },
                "required": ["process_id"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "process_write",
            ToolsetId::Terminal,
            "Write stdin input to a running background process.",
            json!({
                "type": "object",
                "properties": {
                    "process_id": { "type": "string" },
                    "input": { "type": "string" },
                    "eof": { "type": "boolean" }
                },
                "required": ["process_id", "input"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "process_kill",
            ToolsetId::Terminal,
            "Terminate a running background process.",
            json!({
                "type": "object",
                "properties": {
                    "process_id": { "type": "string" }
                },
                "required": ["process_id"]
            }),
            ToolRiskLevel::High,
            false,
        ),
        HermesToolSpec::new(
            "execute_code",
            ToolsetId::CodeExecution,
            "Execute safe pure-Rust expressions or bounded subprocess code (python3/node/sh) with zero env secret leakage.",
            json!({
                "type": "object",
                "properties": {
                    "language": { "type": "string", "enum": ["expr", "math", "rust_expr", "json", "python", "python3", "node", "javascript", "sh"] },
                    "code": { "type": "string" },
                    "timeout_ms": { "type": "integer" },
                    "allowed_tools": { "type": "array", "items": { "type": "string" } },
                    "input": {}
                },
                "required": ["language", "code"]
            }),
            ToolRiskLevel::High,
            false,
        ),
        // 5. Web & Browser (7)
        HermesToolSpec::new(
            "web_search",
            ToolsetId::Web,
            "Search the web via DuckDuckGo, SearXNG, Tavily, Perplexity, or Brave Search.",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "provider": { "type": "string", "enum": ["duckduckgo", "searxng", "tavily", "perplexity", "brave"] },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 20 },
                    "endpoint_url": { "type": "string" }
                },
                "required": ["query"]
            }),
            ToolRiskLevel::Low,
            true,
        ),
        HermesToolSpec::new(
            "web_extract",
            ToolsetId::Web,
            "Fetch a web page and extract clean Markdown content with prompt-injection scanning.",
            json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "max_chars": { "type": "integer" },
                    "include_links": { "type": "boolean" }
                },
                "required": ["url"]
            }),
            ToolRiskLevel::Low,
            true,
        ),
        HermesToolSpec::new(
            "browser_navigate",
            ToolsetId::Browser,
            "Navigate a Chrome DevTools Protocol (CDP) browser tab to a URL.",
            json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "cdp_url": { "type": "string" }
                },
                "required": ["url"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "browser_snapshot",
            ToolsetId::Browser,
            "Capture DOM accessibility/text snapshot of the active CDP browser tab.",
            json!({
                "type": "object",
                "properties": {
                    "cdp_url": { "type": "string" },
                    "selector": { "type": "string" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "browser_click",
            ToolsetId::Browser,
            "Click an element by CSS selector or coordinates in the active CDP browser tab.",
            json!({
                "type": "object",
                "properties": {
                    "selector": { "type": "string" },
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "cdp_url": { "type": "string" }
                }
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "browser_type",
            ToolsetId::Browser,
            "Type text into a focused or selected element in the active CDP browser tab.",
            json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string" },
                    "selector": { "type": "string" },
                    "submit": { "type": "boolean" },
                    "cdp_url": { "type": "string" }
                },
                "required": ["text"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "browser_screenshot",
            ToolsetId::Browser,
            "Capture a PNG screenshot of the active CDP browser tab.",
            json!({
                "type": "object",
                "properties": {
                    "full_page": { "type": "boolean" },
                    "output_path": { "type": "string" },
                    "cdp_url": { "type": "string" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        // 6. Audio, Voice, Vision, ImageGen, VideoGen (6)
        HermesToolSpec::new(
            "text_to_speech",
            ToolsetId::AudioTts,
            "Synthesize spoken audio via OpenAI/Edge/ElevenLabs or deterministic local PCM/RIFF WAVE synthesizer.",
            json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string" },
                    "provider": { "type": "string", "enum": ["local", "openai", "edge", "elevenlabs", "auto"] },
                    "voice": { "type": "string" },
                    "sample_rate": { "type": "integer" },
                    "output_path": { "type": "string" }
                },
                "required": ["text"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "speech_to_text",
            ToolsetId::AudioStt,
            "Transcribe WAV/audio files via Whisper/OpenAI/local adapter returning transcript, confidence, and duration.",
            json!({
                "type": "object",
                "properties": {
                    "audio_path": { "type": "string" },
                    "audio_base64": { "type": "string" },
                    "provider": { "type": "string", "enum": ["local", "openai", "whisper"] },
                    "language": { "type": "string" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "voice_session",
            ToolsetId::Voice,
            "Inspect or transition full-duplex Voice / Talk Mode session state and VAD audio buffer.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["status", "start", "barge_in", "resume", "close"] }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "vision_analyze",
            ToolsetId::Vision,
            "Analyze local or remote images by probing image metadata and building multimodal inspection payloads.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "image_path": { "type": "string" },
                    "image_base64": { "type": "string" },
                    "image_url": { "type": "string" },
                    "detail": { "type": "string", "enum": ["low", "high", "auto"] }
                },
                "required": ["prompt"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "image_generate",
            ToolsetId::ImageGen,
            "Generate an image asset via provider API or deterministic local SVG/PNG generator.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "size": { "type": "string" },
                    "provider": { "type": "string", "enum": ["local", "openai", "auto"] },
                    "format": { "type": "string", "enum": ["svg", "png"] },
                    "output_path": { "type": "string" }
                },
                "required": ["prompt"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "video_generate",
            ToolsetId::VideoGen,
            "Generate a video asset via provider API or offline storyboard manifest fallback.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "duration_secs": { "type": "integer" },
                    "resolution": { "type": "string" },
                    "provider": { "type": "string" },
                    "output_path": { "type": "string" }
                },
                "required": ["prompt"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        // 7. Subagents & Cron (2)
        HermesToolSpec::new(
            "delegate_task",
            ToolsetId::Subagents,
            "Spawn depth-bounded concurrent subagents with isolated toolsets, lineage tracking, and synthesized results.",
            json!({
                "type": "object",
                "properties": {
                    "task": { "type": "string" },
                    "role": { "type": "string" },
                    "toolsets": { "type": "array", "items": { "type": "string" } },
                    "model": { "type": "string" },
                    "tasks": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "role": { "type": "string" },
                                "prompt": { "type": "string" },
                                "toolsets": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["prompt"]
                        }
                    },
                    "max_concurrent": { "type": "integer", "minimum": 1, "maximum": 16 },
                    "max_depth": { "type": "integer", "minimum": 1, "maximum": 8 }
                }
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "cronjob_manage",
            ToolsetId::Cronjob,
            "Create, list, pause, resume, delete, preview, inspect history, or run cronjobs now.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["create", "list", "get", "pause", "resume", "delete", "run_now", "history", "preview"] },
                    "id": { "type": "string" },
                    "name": { "type": "string" },
                    "schedule": { "type": "string" },
                    "timezone": { "type": "string" },
                    "prompt": { "type": "string" },
                    "target_chat_id": { "type": "integer" },
                    "model": { "type": "string" }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        // 8. All 14 Kanban Tools (14)
        HermesToolSpec::new(
            "kanban_list_boards",
            ToolsetId::Kanban,
            "List all Kanban boards.",
            json!({ "type": "object", "properties": {} }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "kanban_create_board",
            ToolsetId::Kanban,
            "Create a new Kanban board with customizable columns.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "columns": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["name"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_get_board",
            ToolsetId::Kanban,
            "Get a Kanban board with its columns and cards.",
            json!({
                "type": "object",
                "properties": { "board_id": { "type": "string" } },
                "required": ["board_id"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "kanban_create_card",
            ToolsetId::Kanban,
            "Create a new card on a Kanban board.",
            json!({
                "type": "object",
                "properties": {
                    "board_id": { "type": "string" },
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "column": { "type": "string" },
                    "priority": { "type": "string" },
                    "labels": { "type": "array", "items": { "type": "string" } },
                    "assignee": { "type": "string" }
                },
                "required": ["board_id", "title"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_get_card",
            ToolsetId::Kanban,
            "Get full details of a Kanban card including comments and checklists.",
            json!({
                "type": "object",
                "properties": { "card_id": { "type": "string" } },
                "required": ["card_id"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "kanban_update_card",
            ToolsetId::Kanban,
            "Update title, description, priority, or labels on a Kanban card.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "priority": { "type": "string" },
                    "labels": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["card_id"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_move_card",
            ToolsetId::Kanban,
            "Move a Kanban card to another column or position.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "column": { "type": "string" },
                    "position": { "type": "integer" }
                },
                "required": ["card_id", "column"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_delete_card",
            ToolsetId::Kanban,
            "Delete a Kanban card by ID.",
            json!({
                "type": "object",
                "properties": { "card_id": { "type": "string" } },
                "required": ["card_id"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "kanban_assign_card",
            ToolsetId::Kanban,
            "Assign a Kanban card to a user or agent.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "assignee": { "type": "string" }
                },
                "required": ["card_id", "assignee"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_add_comment",
            ToolsetId::Kanban,
            "Add a comment to a Kanban card.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "body": { "type": "string" },
                    "author": { "type": "string" }
                },
                "required": ["card_id", "body"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_list_comments",
            ToolsetId::Kanban,
            "List comments on a Kanban card.",
            json!({
                "type": "object",
                "properties": { "card_id": { "type": "string" } },
                "required": ["card_id"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "kanban_add_checklist",
            ToolsetId::Kanban,
            "Add a checklist (with optional initial items) to a Kanban card.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "title": { "type": "string" },
                    "items": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["card_id", "title"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_toggle_checklist",
            ToolsetId::Kanban,
            "Toggle or set completion status of a checklist item on a Kanban card.",
            json!({
                "type": "object",
                "properties": {
                    "card_id": { "type": "string" },
                    "checklist_id": { "type": "string" },
                    "item_index": { "type": "integer", "minimum": 0 },
                    "checked": { "type": "boolean" }
                },
                "required": ["card_id"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "kanban_search_cards",
            ToolsetId::Kanban,
            "Search Kanban cards by keyword, board, column, assignee, or priority.",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "board_id": { "type": "string" },
                    "column": { "type": "string" },
                    "assignee": { "type": "string" },
                    "priority": { "type": "string" }
                }
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        // 9. Clarify, DesktopUI, CUA, Connections, Reasoning, MCP, Search, Memory, Skills, Honcho (13)
        HermesToolSpec::new(
            "clarify",
            ToolsetId::Clarify,
            "Ask the user a structured multi-choice or free-text clarification question with inline Telegram buttons.",
            json!({
                "type": "object",
                "properties": {
                    "question": { "type": "string" },
                    "options": { "type": "array", "items": { "type": "string" } },
                    "allow_free_text": { "type": "boolean" },
                    "multi_select": { "type": "boolean" }
                },
                "required": ["question"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "desktop_ui_inspect",
            ToolsetId::DesktopUi,
            "Inspect gpui-telegram desktop workbench panels, active destination, and recent UI events.",
            json!({ "type": "object", "properties": {} }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "desktop_ui_act",
            ToolsetId::DesktopUi,
            "Emit a structured desktop/workbench UI action (e.g. open_panel, launch_talk_mode, open_kanban).",
            json!({
                "type": "object",
                "properties": {
                    "action_type": { "type": "string" },
                    "payload": { "type": "object" }
                },
                "required": ["action_type"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "computer_use",
            ToolsetId::ComputerUse,
            "Acquire/release ComputerControlLease and execute generation-fenced Remote Computer CUA actions.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["acquire_lease", "release_lease", "emergency_takeover", "lease_status", "click", "move_cursor", "key_press", "type_string", "screenshot", "wait"] },
                    "device_id": { "type": "string" },
                    "lease_id": { "type": "string" },
                    "x": { "type": "integer" },
                    "y": { "type": "integer" },
                    "button": { "type": "string" },
                    "key": { "type": "string" },
                    "text": { "type": "string" },
                    "wait_ms": { "type": "integer" }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::High,
            false,
        ),
        HermesToolSpec::new(
            "manage_connections",
            ToolsetId::Connections,
            "Connect, list, test, or disconnect external integrations with credential redaction.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["list", "connect", "disconnect", "test", "get"] },
                    "name": { "type": "string" },
                    "provider": { "type": "string" },
                    "endpoint": { "type": "string" },
                    "api_key": { "type": "string" }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "think",
            ToolsetId::Reasoning,
            "Record structured scratchpad thoughts and hypotheses without side effects.",
            json!({
                "type": "object",
                "properties": {
                    "thought": { "type": "string" },
                    "hypothesis": { "type": "string" },
                    "steps": { "type": "array", "items": { "type": "string" } },
                    "conclusion": { "type": "string" }
                },
                "required": ["thought"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "reason",
            ToolsetId::Reasoning,
            "Perform explicit multi-step analytical reasoning and confidence scoring.",
            json!({
                "type": "object",
                "properties": {
                    "problem": { "type": "string" },
                    "steps": { "type": "array", "items": { "type": "string" } },
                    "conclusion": { "type": "string" },
                    "confidence": { "type": "number" }
                },
                "required": ["problem"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "mcp_list_tools",
            ToolsetId::Mcp,
            "List all registered Model Context Protocol (MCP) tools and their risk levels.",
            json!({ "type": "object", "properties": {} }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "mcp_call",
            ToolsetId::Mcp,
            "Invoke an external MCP tool registered in McpRegistry with risk enforcement.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "arguments": { "type": "object" }
                },
                "required": ["name"]
            }),
            ToolRiskLevel::Medium,
            false,
        ),
        HermesToolSpec::new(
            "session_search",
            ToolsetId::SessionSearch,
            "Search past conversation sessions and messages via FTS5 or CJK substring fallback.",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "limit": { "type": "integer", "minimum": 1, "maximum": 50 }
                },
                "required": ["query"]
            }),
            ToolRiskLevel::ReadOnly,
            true,
        ),
        HermesToolSpec::new(
            "memory",
            ToolsetId::Memory,
            "Read, add, replace, or remove entries in bounded MEMORY.md (2200 chars) or USER.md (1375 chars).",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["read", "add", "replace", "remove"] },
                    "target": { "type": "string", "enum": ["memory", "user"] },
                    "content": { "type": "string" },
                    "old_text": { "type": "string" }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "skill_manage",
            ToolsetId::Skills,
            "List, view, create, patch, delete, or security-scan agentskills.io SKILL.md skills.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["list", "view", "create", "patch", "delete", "scan"] },
                    "name": { "type": "string" },
                    "description": { "type": "string" },
                    "content": { "type": "string" },
                    "old_text": { "type": "string" },
                    "new_text": { "type": "string" },
                    "tags": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
        HermesToolSpec::new(
            "honcho_profile",
            ToolsetId::Honcho,
            "Inspect, query, or record observations in Honcho dialectic user modeling.",
            json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "enum": ["profile", "query", "record_observation"] },
                    "user_id": { "type": "string" },
                    "query": { "type": "string" },
                    "observation": { "type": "string" },
                    "category": { "type": "string" }
                },
                "required": ["action"]
            }),
            ToolRiskLevel::Low,
            false,
        ),
    ]
}

pub fn find_tool_spec(name: &str) -> Option<HermesToolSpec> {
    all_builtin_tool_specs().into_iter().find(|s| s.name == name)
}

/// Central tool dispatch engine enforcing enablement, cancellation, TIR checks, and routing.
pub struct HermesToolExecutor;

impl HermesToolExecutor {
    pub fn list_specs(ctx: &HermesToolContext) -> Vec<HermesToolSpec> {
        ctx.toolsets
            .read()
            .map(|r| r.active_specs())
            .unwrap_or_else(|_| all_builtin_tool_specs())
    }

    pub async fn execute_call(call: &HermesToolCall, ctx: &HermesToolContext) -> HermesToolResult {
        Self::execute(&call.name, call.arguments.clone(), ctx)
            .await
            .with_call_id(call.id.clone())
    }

    pub async fn execute(
        spec_name: &str,
        args: Value,
        ctx: &HermesToolContext,
    ) -> HermesToolResult {
        let start = Instant::now();

        if ctx.is_cancelled() {
            return HermesToolResult::err(spec_name, "Execution cancelled")
                .with_duration(start.elapsed().as_millis() as u64);
        }

        let Some(spec) = find_tool_spec(spec_name) else {
            // Fall back to MCP registry if registered directly by tool name
            let mcp_exists = ctx
                .mcp_registry
                .read()
                .ok()
                .is_some_and(|reg| reg.get(spec_name).is_some());
            if mcp_exists {
                let mut res = execute_agentic_tool(
                    "mcp_call",
                    json!({ "name": spec_name, "arguments": args }),
                    ctx,
                )
                .await;
                res.tool_name = spec_name.to_string();
                return res.with_duration(start.elapsed().as_millis() as u64);
            }
            return HermesToolResult::err(spec_name, format!("Unknown Hermes tool '{spec_name}'"))
                .with_duration(start.elapsed().as_millis() as u64);
        };

        let enabled = ctx
            .toolsets
            .read()
            .map(|r| r.is_tool_enabled(&spec))
            .unwrap_or(true);
        if !enabled {
            return HermesToolResult::err(
                spec_name,
                format!(
                    "Tool '{spec_name}' belongs to disabled toolset '{}'",
                    spec.toolset.as_str()
                ),
            )
            .with_duration(start.elapsed().as_millis() as u64);
        }

        // TIR pre-execution check (note: terminal & execute_code also run full ApprovalQueue inside tools_terminal)
        if let Ok(sec) = ctx.security.read() {
            match sec.pre_execute_check(spec_name, &args) {
                TirVerdict::Block { reason } => {
                    return HermesToolResult::err(spec_name, reason)
                        .with_duration(start.elapsed().as_millis() as u64);
                }
                TirVerdict::Allow | TirVerdict::RequireApproval { .. } => {}
            }
        }

        let res = match spec.toolset {
            ToolsetId::Core => execute_core_toolset_tool(spec_name, args, ctx),
            ToolsetId::File | ToolsetId::Workspace | ToolsetId::Todo | ToolsetId::Planning => {
                execute_file_tool(spec_name, args, ctx).await
            }
            ToolsetId::Terminal | ToolsetId::CodeExecution => {
                execute_terminal_tool(spec_name, args, ctx).await
            }
            ToolsetId::Web
            | ToolsetId::Browser
            | ToolsetId::AudioTts
            | ToolsetId::AudioStt
            | ToolsetId::Vision
            | ToolsetId::ImageGen
            | ToolsetId::VideoGen => execute_web_media_tool(spec_name, args, ctx).await,
            ToolsetId::Subagents
            | ToolsetId::Cronjob
            | ToolsetId::Kanban
            | ToolsetId::Clarify
            | ToolsetId::DesktopUi
            | ToolsetId::ComputerUse
            | ToolsetId::Connections
            | ToolsetId::Reasoning
            | ToolsetId::Mcp
            | ToolsetId::SessionSearch
            | ToolsetId::Memory
            | ToolsetId::Skills
            | ToolsetId::Honcho
            | ToolsetId::Voice => execute_agentic_tool(spec_name, args, ctx).await,
        };

        res.with_duration(start.elapsed().as_millis() as u64)
    }
}

fn execute_core_toolset_tool(
    name: &str,
    args: Value,
    ctx: &HermesToolContext,
) -> HermesToolResult {
    match name {
        "toolset_list" => {
            let only_enabled = args
                .get("only_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let Ok(reg) = ctx.toolsets.read() else {
                return HermesToolResult::err(name, "ToolsetRegistry lock poisoned");
            };
            let mut summary = reg.summary_json();
            if only_enabled {
                if let Some(arr) = summary.get_mut("toolsets").and_then(|v| v.as_array_mut()) {
                    arr.retain(|item| item.get("enabled").and_then(|e| e.as_bool()) == Some(true));
                }
            }
            HermesToolResult::ok(
                name,
                serde_json::to_string_pretty(&summary).unwrap_or_default(),
                Some(summary),
            )
        }
        "toolset_enable" => {
            let Some(target) = args.get("name").and_then(|v| v.as_str()) else {
                return HermesToolResult::err(name, "Missing required 'name' parameter");
            };
            let Ok(mut reg) = ctx.toolsets.write() else {
                return HermesToolResult::err(name, "ToolsetRegistry lock poisoned");
            };
            match reg.enable_by_name(target) {
                Ok(enabled) => {
                    let names: Vec<&str> = enabled.iter().map(|t| t.as_str()).collect();
                    let payload = json!({ "enabled": names, "active_tool_count": reg.active_specs().len() });
                    HermesToolResult::ok(name, format!("Enabled toolset(s): {}", names.join(", ")), Some(payload))
                }
                Err(e) => HermesToolResult::err(name, e),
            }
        }
        "toolset_disable" => {
            let Some(target) = args.get("name").and_then(|v| v.as_str()) else {
                return HermesToolResult::err(name, "Missing required 'name' parameter");
            };
            let Ok(mut reg) = ctx.toolsets.write() else {
                return HermesToolResult::err(name, "ToolsetRegistry lock poisoned");
            };
            match reg.disable_by_name(target) {
                Ok(id) => {
                    let payload = json!({ "disabled": id.as_str(), "active_tool_count": reg.active_specs().len() });
                    HermesToolResult::ok(name, format!("Disabled toolset '{}'", id.as_str()), Some(payload))
                }
                Err(e) => HermesToolResult::err(name, e),
            }
        }
        _ => HermesToolResult::err(name, format!("Unsupported core tool '{name}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_28_toolsets_and_58_tools() {
        assert_eq!(ToolsetId::all().len(), 28);
        let specs = all_builtin_tool_specs();
        assert!(specs.len() >= 40, "Expected 40+ tools, got {}", specs.len());
        for ts in ToolsetId::all() {
            assert!(
                specs.iter().any(|s| s.toolset == *ts),
                "Toolset {:?} has no registered tools",
                ts
            );
        }
        let openai = export_openai_tools(&specs);
        let anthropic = export_anthropic_tools(&specs);
        assert_eq!(openai.len(), specs.len());
        assert_eq!(anthropic.len(), specs.len());
        assert_eq!(openai[0]["type"], "function");
        assert!(anthropic[0].get("input_schema").is_some());
    }

    #[test]
    fn test_presets_and_custom_toolset() {
        let mut reg = ToolsetRegistry::from_preset(ToolsetPreset::Minimal);
        assert!(reg.is_toolset_enabled(ToolsetId::File));
        assert!(!reg.is_toolset_enabled(ToolsetId::Terminal));

        reg.register_custom_toolset(CustomToolset {
            name: "my_custom".into(),
            description: "Custom bundle".into(),
            toolsets: vec![ToolsetId::Kanban, ToolsetId::Web],
            included_tools: vec!["think".into()],
            excluded_tools: vec!["kanban_delete_card".into()],
        });
        reg.apply_custom_toolset("my_custom").unwrap();
        assert!(reg.is_toolset_enabled(ToolsetId::Kanban));
        assert!(reg.is_tool_enabled(&find_tool_spec("think").unwrap()));
        assert!(!reg.is_tool_enabled(&find_tool_spec("kanban_delete_card").unwrap()));
    }
}
