use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDto, HermesCronJobDto, HermesHeartbeatConfigDto, HermesHonchoStateDto,
    HermesKanbanBoardDto, HermesMemoryStateDto, HermesProviderConfigDto, HermesSaveCronRequest,
    HermesSaveProviderRequest, HermesSessionDto, HermesSessionExportResponse, HermesSkillDto,
    HermesSoulDto, HermesStatusDto, HermesTerminalProcessDto, HermesToolsetDto,
    HermesUsageSummaryDto,
};

use crate::components::tool_permission_dock::{PendingToolApproval, ToolApprovalDecision};
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::hermes_studio_panels;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TasksWorkspaceTab {
    #[default]
    Tasks,
    Skills,
    MemorySoul,
    McpTools,
    Cron,
    Sessions,
    Approvals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioInputTarget {
    #[default]
    None,
    NewTask,
    KanbanBoardName,
    KanbanCardTitle,
    SkillSearch,
    SkillEditorName,
    SkillEditorContent,
    SkillLearnName,
    MemoryMarkdown,
    UserMarkdown,
    SoulPersonaName,
    SoulContent,
    HonchoQuery,
    CronName,
    CronSchedule,
    CronPrompt,
    CronTargetChat,
    SessionSearch,
    FallbackChain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpTaskEntry {
    pub id: String,
    pub title: String,
    pub status: String,
    pub schedule: Option<String>,
    pub last_output: Option<String>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpServerEntry {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub is_connected: bool,
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubagentLineageEntry {
    pub session_id: String,
    pub parent_session_id: Option<String>,
    pub title: String,
    pub model: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronHistoryEntry {
    pub job_id: String,
    pub job_name: String,
    pub status: String,
    pub summary: String,
    pub ran_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TasksWorkspaceAction {
    RefreshAll,
    CreateTask(String),
    ApproveTask(String),
    CallMcpTool(String, String),
    ApprovePermission(String, ToolApprovalDecision),
    CreateKanbanBoard(String),
    CreateKanbanCard {
        board_id: String,
        title: String,
        column: String,
        priority: String,
    },
    MoveKanbanCard {
        card_id: String,
        column: String,
    },
    DeleteKanbanCard(String),
    DecideHermesApproval {
        approval_id: String,
        decision: String,
    },
    SaveSkill {
        name: String,
        content: String,
        enabled: bool,
    },
    ToggleSkillEnabled {
        name: String,
        enabled: bool,
    },
    ApproveSkill(String),
    DeleteSkill(String),
    LearnSkillFromSession(String),
    ToggleNoBundledSkills(bool),
    SaveMemoryRaw {
        scope: String,
        content: String,
    },
    ReloadFrozenSnapshot,
    SaveSoul(HermesSoulDto),
    QueryHoncho(String),
    ToggleHoncho(bool),
    ToggleToolset {
        name: String,
        enabled: bool,
    },
    ApplyToolsetPreset(String),
    SetTerminalBackend(String),
    PollTerminalProcesses,
    KillTerminalProcess(String),
    SaveCronJob(HermesSaveCronRequest),
    ToggleCronJob {
        job_id: String,
        enabled: bool,
    },
    RunCronNow(String),
    DeleteCronJob(String),
    SaveHeartbeat(HermesHeartbeatConfigDto),
    SearchSessions(String),
    BranchSession(String),
    CompressSession(String),
    ExportSession {
        session_id: String,
        format: String,
    },
    RollbackSession(String),
    SetApiMode(String),
    SaveProviderConfig(HermesSaveProviderRequest),
}

pub fn default_28_hermes_toolsets() -> Vec<HermesToolsetDto> {
    let defs: &[(&str, &str, &str, bool)] = &[
        ("core", "Core chat & status tools", "safe", true),
        ("web", "Web search & page extraction", "safe", true),
        ("terminal", "Shell & background process execution", "high", true),
        ("file", "Workspace file read/write/patch/search", "caution", true),
        ("memory", "Bounded MEMORY.md / USER.md store", "safe", true),
        ("skills", "SKILL.md registry & /learn extraction", "caution", true),
        ("vision", "Image understanding & OCR analysis", "safe", true),
        ("image_gen", "Text-to-image synthesis", "safe", false),
        ("video_gen", "Video synthesis & clipping", "safe", false),
        ("browser", "CDP browser navigation & snapshots", "caution", false),
        ("audio_tts", "Text-to-speech WAV synthesis", "safe", true),
        ("audio_stt", "Speech-to-text transcription", "safe", true),
        ("cronjob", "Cron routines & scheduled prompts", "caution", true),
        ("subagents", "Delegated child subagents & lineage", "caution", true),
        ("code_execution", "Sandboxed expression & script runner", "high", true),
        ("clarify", "Interactive inline clarify prompts", "safe", true),
        ("todo", "Session checklist tracking", "safe", true),
        ("planning", "Structured plan specification builder", "safe", true),
        ("reasoning", "Extended reasoning scratchpad", "safe", true),
        ("mcp", "External MCP server bridge", "caution", true),
        ("kanban", "14-tool Kanban boards, cards & checklists", "safe", true),
        ("desktop_ui", "Desktop stage inspection & actions", "caution", true),
        ("computer_use", "Remote Computer CUA lease actions", "high", false),
        ("connections", "Provider & external connection manager", "caution", true),
        ("honcho", "Dialectic user modeling & memory", "safe", true),
        ("session_search", "FTS5 + CJK session search", "safe", true),
        ("workspace", "Workspace context & rollback checkpoints", "caution", true),
        ("voice", "Full-duplex Talk Mode & barge-in", "safe", true),
    ];
    defs.iter()
        .map(|(name, desc, risk, enabled)| HermesToolsetDto {
            name: (*name).to_string(),
            description: (*desc).to_string(),
            enabled: *enabled,
            tools: Vec::new(),
            risk_level: (*risk).to_string(),
        })
        .collect()
}

pub struct GlobalTasksWorkspace {
    pub active_tab: usize,
    pub tab: TasksWorkspaceTab,
    pub tasks: Vec<AcpTaskEntry>,
    pub mcp_servers: Vec<McpServerEntry>,
    pub pending_approvals: Vec<PendingToolApproval>,
    pub task_draft: String,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_action: Option<TasksWorkspaceAction>,
    pub focus_handle: Option<FocusHandle>,
    pub active_input: StudioInputTarget,
    pub status_summary: HermesStatusDto,
    pub kanban_boards: Vec<HermesKanbanBoardDto>,
    pub active_board_id: String,
    pub kanban_board_draft: String,
    pub kanban_card_draft: String,
    pub kanban_card_column: String,
    pub hermes_approvals: Vec<HermesApprovalDto>,
    pub subagent_lineage: Vec<SubagentLineageEntry>,
    pub skills: Vec<HermesSkillDto>,
    pub skill_search_query: String,
    pub selected_skill_name: Option<String>,
    pub skill_editor_name: String,
    pub skill_editor_content: String,
    pub skill_learn_draft: String,
    pub no_bundled_skills: bool,
    pub memory_state: HermesMemoryStateDto,
    pub memory_edit_draft: String,
    pub user_edit_draft: String,
    pub snapshot_frozen_label: String,
    pub soul_state: HermesSoulDto,
    pub soul_persona_draft: String,
    pub soul_content_draft: String,
    pub honcho_state: HermesHonchoStateDto,
    pub honcho_query_draft: String,
    pub toolsets: Vec<HermesToolsetDto>,
    pub active_toolset_preset: String,
    pub terminal_backend: String,
    pub terminal_processes: Vec<HermesTerminalProcessDto>,
    pub cron_jobs: Vec<HermesCronJobDto>,
    pub cron_history: Vec<CronHistoryEntry>,
    pub cron_name_draft: String,
    pub cron_schedule_draft: String,
    pub cron_prompt_draft: String,
    pub cron_chat_draft: String,
    pub heartbeat: HermesHeartbeatConfigDto,
    pub sessions: Vec<HermesSessionDto>,
    pub session_search_draft: String,
    pub session_search_hits: Vec<serde_json::Value>,
    pub selected_session_id: Option<String>,
    pub session_export_preview: Option<HermesSessionExportResponse>,
    pub providers: Vec<HermesProviderConfigDto>,
    pub active_api_mode: String,
    pub fallback_chain_draft: String,
    pub usage_summary: HermesUsageSummaryDto,
}

impl GlobalTasksWorkspace {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            tab: TasksWorkspaceTab::Tasks,
            tasks: Vec::new(),
            mcp_servers: Vec::new(),
            pending_approvals: Vec::new(),
            task_draft: String::new(),
            is_loading: false,
            error_message: None,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".to_string(),
            pending_action: None,
            focus_handle: None,
            active_input: StudioInputTarget::None,
            status_summary: HermesStatusDto {
                healthy: true,
                engine: "hermes-rust".to_string(),
                active_model: "hermes-local-v1".to_string(),
                active_provider: "local_hermes".to_string(),
                api_mode: "local_hermes".to_string(),
                toolsets_count: 28,
                tools_count: 58,
                ..Default::default()
            },
            kanban_boards: Vec::new(),
            active_board_id: String::new(),
            kanban_board_draft: String::new(),
            kanban_card_draft: String::new(),
            kanban_card_column: "Backlog".to_string(),
            hermes_approvals: Vec::new(),
            subagent_lineage: Vec::new(),
            skills: Vec::new(),
            skill_search_query: String::new(),
            selected_skill_name: None,
            skill_editor_name: String::new(),
            skill_editor_content: String::new(),
            skill_learn_draft: String::new(),
            no_bundled_skills: false,
            memory_state: HermesMemoryStateDto {
                char_limit: 2200,
                ..Default::default()
            },
            memory_edit_draft: String::new(),
            user_edit_draft: String::new(),
            snapshot_frozen_label: "Session Snapshot Frozen (Synced)".to_string(),
            soul_state: HermesSoulDto::default(),
            soul_persona_draft: "Hermes".to_string(),
            soul_content_draft: String::new(),
            honcho_state: HermesHonchoStateDto::default(),
            honcho_query_draft: String::new(),
            toolsets: default_28_hermes_toolsets(),
            active_toolset_preset: "hermes-telegram".to_string(),
            terminal_backend: "local".to_string(),
            terminal_processes: Vec::new(),
            cron_jobs: Vec::new(),
            cron_history: Vec::new(),
            cron_name_draft: String::new(),
            cron_schedule_draft: "*/15 * * * *".to_string(),
            cron_prompt_draft: String::new(),
            cron_chat_draft: String::new(),
            heartbeat: HermesHeartbeatConfigDto {
                enabled: false,
                interval_secs: 300,
                prompt: "Check workspace & pending tasks".to_string(),
                ..Default::default()
            },
            sessions: Vec::new(),
            session_search_draft: String::new(),
            session_search_hits: Vec::new(),
            selected_session_id: None,
            session_export_preview: None,
            providers: Vec::new(),
            active_api_mode: "local_hermes".to_string(),
            fallback_chain_draft: "local_hermes -> openai -> anthropic".to_string(),
            usage_summary: HermesUsageSummaryDto::default(),
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.palette = palette;
        s.language_id = language_id.to_string();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, language_id: impl Into<String>) {
        self.language_id = language_id.into();
    }

    pub fn sync_tasks(&mut self, tasks: Vec<AcpTaskEntry>) {
        self.tasks = tasks;
        self.is_loading = false;
    }

    pub fn sync_mcp_servers(&mut self, servers: Vec<McpServerEntry>) {
        self.mcp_servers = servers;
        self.is_loading = false;
    }

    pub fn sync_approvals(&mut self, requests: Vec<PendingToolApproval>) {
        self.pending_approvals = requests;
    }

    pub fn set_loading(&mut self, loading: bool, err: Option<String>) {
        self.is_loading = loading;
        self.error_message = err;
    }

    pub fn take_action(&mut self) -> Option<TasksWorkspaceAction> {
        self.pending_action.take()
    }

    pub fn set_tab(&mut self, tab: TasksWorkspaceTab) {
        self.tab = tab;
        self.active_tab = match tab {
            TasksWorkspaceTab::Tasks | TasksWorkspaceTab::Approvals => 0,
            TasksWorkspaceTab::Skills => 1,
            TasksWorkspaceTab::MemorySoul => 2,
            TasksWorkspaceTab::McpTools => 3,
            TasksWorkspaceTab::Cron => 4,
            TasksWorkspaceTab::Sessions => 5,
        };
    }

    pub fn sync_studio_status(&mut self, status: HermesStatusDto) {
        if !status.api_mode.trim().is_empty() {
            self.active_api_mode = status.api_mode.clone();
        }
        self.status_summary = status;
        self.is_loading = false;
    }

    pub fn sync_kanban_boards(&mut self, boards: Vec<HermesKanbanBoardDto>) {
        if self.active_board_id.is_empty()
            || !boards.iter().any(|b| b.board_id == self.active_board_id)
        {
            self.active_board_id = boards
                .first()
                .map(|b| b.board_id.clone())
                .unwrap_or_default();
        }
        self.kanban_boards = boards;
    }

    pub fn sync_hermes_approvals(&mut self, approvals: Vec<HermesApprovalDto>) {
        self.hermes_approvals = approvals;
    }

    pub fn sync_skills(&mut self, skills: Vec<HermesSkillDto>) {
        if self.selected_skill_name.is_none() {
            if let Some(first) = skills.first() {
                self.selected_skill_name = Some(first.name.clone());
                self.skill_editor_name = first.name.clone();
                self.skill_editor_content = first.content.clone();
            }
        } else if let Some(ref sel) = self.selected_skill_name {
            if let Some(found) = skills.iter().find(|s| &s.name == sel) {
                self.skill_editor_name = found.name.clone();
                if self.skill_editor_content.is_empty() {
                    self.skill_editor_content = found.content.clone();
                }
            }
        }
        self.skills = skills;
    }

    pub fn sync_memory_state(&mut self, mem: HermesMemoryStateDto) {
        self.memory_edit_draft = mem.memory_markdown.clone();
        self.user_edit_draft = mem.user_markdown.clone();
        self.memory_state = mem;
        self.snapshot_frozen_label = "Snapshot Synced".to_string();
    }

    pub fn sync_soul_state(&mut self, soul: HermesSoulDto) {
        if !soul.persona_name.trim().is_empty() {
            self.soul_persona_draft = soul.persona_name.clone();
        }
        self.soul_content_draft = soul.content.clone();
        self.soul_state = soul;
    }

    pub fn sync_honcho_state(&mut self, honcho: HermesHonchoStateDto) {
        self.honcho_state = honcho;
    }

    pub fn sync_toolsets(&mut self, toolsets: Vec<HermesToolsetDto>) {
        if !toolsets.is_empty() {
            self.toolsets = toolsets;
        }
    }

    pub fn sync_terminal_processes(&mut self, procs: Vec<HermesTerminalProcessDto>) {
        self.terminal_processes = procs;
    }

    pub fn sync_cron_jobs(&mut self, jobs: Vec<HermesCronJobDto>) {
        self.cron_jobs = jobs;
    }

    pub fn sync_heartbeat(&mut self, hb: HermesHeartbeatConfigDto) {
        self.heartbeat = hb;
    }

    pub fn sync_sessions(&mut self, sessions: Vec<HermesSessionDto>) {
        self.subagent_lineage = sessions
            .iter()
            .map(|s| SubagentLineageEntry {
                session_id: s.session_id.clone(),
                parent_session_id: s.parent_session_id.clone(),
                title: s
                    .title
                    .clone()
                    .unwrap_or_else(|| s.session_id.clone()),
                model: s.model.clone(),
                status: if s.parent_session_id.is_some() {
                    "subagent/branch".to_string()
                } else {
                    "root".to_string()
                },
            })
            .collect();
        if self.selected_session_id.is_none() {
            self.selected_session_id = sessions.first().map(|s| s.session_id.clone());
        }
        self.sessions = sessions;
    }

    pub fn sync_session_search_hits(&mut self, hits: Vec<serde_json::Value>) {
        self.session_search_hits = hits;
    }

    pub fn sync_session_export(&mut self, preview: HermesSessionExportResponse) {
        self.session_export_preview = Some(preview);
    }

    pub fn sync_providers(&mut self, providers: Vec<HermesProviderConfigDto>) {
        if let Some(active) = providers.iter().find(|p| p.is_active) {
            if !active.api_mode.trim().is_empty() {
                self.active_api_mode = active.api_mode.clone();
            }
        }
        self.providers = providers;
    }

    pub fn sync_usage_summary(&mut self, usage: HermesUsageSummaryDto) {
        self.usage_summary = usage;
    }

    pub fn record_cron_history(&mut self, entry: CronHistoryEntry) {
        self.cron_history.insert(0, entry);
        if self.cron_history.len() > 25 {
            self.cron_history.truncate(25);
        }
    }

    pub fn select_skill(&mut self, name: &str) {
        if let Some(sk) = self.skills.iter().find(|s| s.name == name) {
            self.selected_skill_name = Some(sk.name.clone());
            self.skill_editor_name = sk.name.clone();
            self.skill_editor_content = sk.content.clone();
        }
    }

    pub fn apply_text_key_to_active_input(&mut self, key: &str, key_char: Option<&str>) -> bool {
        let is_multiline = matches!(
            self.active_input,
            StudioInputTarget::SkillEditorContent
                | StudioInputTarget::MemoryMarkdown
                | StudioInputTarget::UserMarkdown
                | StudioInputTarget::SoulContent
                | StudioInputTarget::CronPrompt
        );
        if key == "enter" && !is_multiline {
            match self.active_input {
                StudioInputTarget::NewTask => {
                    if !self.task_draft.trim().is_empty() {
                        self.pending_action =
                            Some(TasksWorkspaceAction::CreateTask(self.task_draft.trim().to_string()));
                        self.task_draft.clear();
                        return true;
                    }
                }
                StudioInputTarget::KanbanBoardName => {
                    if !self.kanban_board_draft.trim().is_empty() {
                        self.pending_action = Some(TasksWorkspaceAction::CreateKanbanBoard(
                            self.kanban_board_draft.trim().to_string(),
                        ));
                        self.kanban_board_draft.clear();
                        return true;
                    }
                }
                StudioInputTarget::KanbanCardTitle => {
                    if !self.kanban_card_draft.trim().is_empty() {
                        self.pending_action = Some(TasksWorkspaceAction::CreateKanbanCard {
                            board_id: self.active_board_id.clone(),
                            title: self.kanban_card_draft.trim().to_string(),
                            column: self.kanban_card_column.clone(),
                            priority: "medium".to_string(),
                        });
                        self.kanban_card_draft.clear();
                        return true;
                    }
                }
                StudioInputTarget::SkillLearnName => {
                    if !self.skill_learn_draft.trim().is_empty() {
                        self.pending_action = Some(TasksWorkspaceAction::LearnSkillFromSession(
                            self.skill_learn_draft.trim().to_string(),
                        ));
                        self.skill_learn_draft.clear();
                        return true;
                    }
                }
                StudioInputTarget::HonchoQuery => {
                    if !self.honcho_query_draft.trim().is_empty() {
                        self.pending_action = Some(TasksWorkspaceAction::QueryHoncho(
                            self.honcho_query_draft.trim().to_string(),
                        ));
                        return true;
                    }
                }
                StudioInputTarget::SessionSearch => {
                    self.pending_action = Some(TasksWorkspaceAction::SearchSessions(
                        self.session_search_draft.trim().to_string(),
                    ));
                    return true;
                }
                _ => return false,
            }
            return false;
        }

        let buf = match self.active_input {
            StudioInputTarget::None => return false,
            StudioInputTarget::NewTask => &mut self.task_draft,
            StudioInputTarget::KanbanBoardName => &mut self.kanban_board_draft,
            StudioInputTarget::KanbanCardTitle => &mut self.kanban_card_draft,
            StudioInputTarget::SkillSearch => &mut self.skill_search_query,
            StudioInputTarget::SkillEditorName => &mut self.skill_editor_name,
            StudioInputTarget::SkillEditorContent => &mut self.skill_editor_content,
            StudioInputTarget::SkillLearnName => &mut self.skill_learn_draft,
            StudioInputTarget::MemoryMarkdown => &mut self.memory_edit_draft,
            StudioInputTarget::UserMarkdown => &mut self.user_edit_draft,
            StudioInputTarget::SoulPersonaName => &mut self.soul_persona_draft,
            StudioInputTarget::SoulContent => &mut self.soul_content_draft,
            StudioInputTarget::HonchoQuery => &mut self.honcho_query_draft,
            StudioInputTarget::CronName => &mut self.cron_name_draft,
            StudioInputTarget::CronSchedule => &mut self.cron_schedule_draft,
            StudioInputTarget::CronPrompt => &mut self.cron_prompt_draft,
            StudioInputTarget::CronTargetChat => &mut self.cron_chat_draft,
            StudioInputTarget::SessionSearch => &mut self.session_search_draft,
            StudioInputTarget::FallbackChain => &mut self.fallback_chain_draft,
        };

        if key == "backspace" {
            buf.pop();
            return true;
        }
        if key == "enter" && is_multiline {
            buf.push('\n');
            return true;
        }
        if key == "space" {
            buf.push(' ');
            return true;
        }
        if let Some(ch) = key_char {
            if !ch.is_empty() && !ch.chars().any(|c| c.is_control()) {
                buf.push_str(ch);
                return true;
            }
        }
        if key.chars().count() == 1 {
            buf.push_str(key);
            return true;
        }
        false
    }

    pub fn handle_key_input(&mut self, key: &str) {
        self.apply_text_key_to_active_input(key, None);
    }
}

impl Default for GlobalTasksWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

pub fn localize_task_status<'a>(lang_id: &str, status: &'a str) -> &'a str {
    match status.trim().to_ascii_lowercase().as_str() {
        "pending" => tr_str(lang_id, "等待中", "pending"),
        "running" => tr_str(lang_id, "运行中", "running"),
        "completed" => tr_str(lang_id, "已完成", "completed"),
        "failed" => tr_str(lang_id, "失败", "failed"),
        _ => status,
    }
}

impl Render for GlobalTasksWorkspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_handle.is_none() {
            self.focus_handle = Some(cx.focus_handle());
        }
        let focus = self.focus_handle.clone().unwrap();
        let pal = self.palette;
        let lang = self.language_id.clone();

        let approvals_badge = self.pending_approvals.len()
            + self
                .hermes_approvals
                .iter()
                .filter(|a| a.status == "pending")
                .count();

        let tabs: &[(TasksWorkspaceTab, &str)] = &[
            (
                TasksWorkspaceTab::Tasks,
                tr_str(&lang, "任务与看板", "Tasks & Kanban"),
            ),
            (
                TasksWorkspaceTab::Skills,
                tr_str(&lang, "技能中心", "Skills Hub"),
            ),
            (
                TasksWorkspaceTab::MemorySoul,
                tr_str(&lang, "记忆与人设", "Memory & SOUL"),
            ),
            (
                TasksWorkspaceTab::McpTools,
                tr_str(&lang, "工具集与终端", "Toolsets & Terminal"),
            ),
            (
                TasksWorkspaceTab::Cron,
                tr_str(&lang, "定时与心跳", "Cron & Heartbeat"),
            ),
            (
                TasksWorkspaceTab::Sessions,
                tr_str(&lang, "会话与模型", "Sessions & Providers"),
            ),
        ];

        let content: AnyElement = match self.tab {
            TasksWorkspaceTab::Tasks | TasksWorkspaceTab::Approvals => {
                hermes_studio_panels::render_tasks_kanban_approvals_tab(self, cx)
            }
            TasksWorkspaceTab::Skills => hermes_studio_panels::render_skills_hub_tab(self, cx),
            TasksWorkspaceTab::MemorySoul => hermes_studio_panels::render_memory_soul_tab(self, cx),
            TasksWorkspaceTab::McpTools => {
                hermes_studio_panels::render_toolsets_terminal_tab(self, cx)
            }
            TasksWorkspaceTab::Cron => hermes_studio_panels::render_cron_heartbeat_tab(self, cx),
            TasksWorkspaceTab::Sessions => {
                hermes_studio_panels::render_sessions_providers_tab(self, cx)
            }
        };

        div()
            .id("hermes-agent-studio-root")
            .track_focus(&focus)
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
                if this.apply_text_key_to_active_input(
                    ev.keystroke.key.as_str(),
                    ev.keystroke.key_char.as_deref(),
                ) {
                    cx.notify();
                    cx.stop_propagation();
                }
            }))
            .flex()
            .flex_col()
            .size_full()
            .min_h(px(0.0))
            .gap(px(12.0))
            .child(
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(tabs.iter().map(|(tab_val, label)| {
                                let is_sel = self.tab == *tab_val
                                    || (*tab_val == TasksWorkspaceTab::Tasks
                                        && self.tab == TasksWorkspaceTab::Approvals);
                                let target_tab = *tab_val;
                                let display = if *tab_val == TasksWorkspaceTab::Tasks
                                    && approvals_badge > 0
                                {
                                    format!("{} ({})", label, approvals_badge)
                                } else {
                                    (*label).to_string()
                                };
                                div()
                                    .px(px(11.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(if is_sel { pal.accent } else { pal.input_bg }))
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(if is_sel {
                                        pal.active_button_fg
                                    } else {
                                        pal.text_secondary
                                    }))
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.set_tab(target_tab);
                                            cx.notify();
                                        }),
                                    )
                                    .child(display)
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(3.0))
                                    .rounded(px(6.0))
                                    .bg(TdesktopThemePalette::rgba_with_alpha(
                                        if self.status_summary.healthy {
                                            pal.success
                                        } else {
                                            pal.danger
                                        },
                                        0.16,
                                    ))
                                    .text_xs()
                                    .text_color(rgb(if self.status_summary.healthy {
                                        pal.success
                                    } else {
                                        pal.danger
                                    }))
                                    .child(format!(
                                        "{} · {} ({})",
                                        self.status_summary.engine,
                                        self.status_summary.active_model,
                                        self.active_api_mode
                                    )),
                            )
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(px(7.0))
                                    .bg(rgb(pal.input_bg))
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.pending_action =
                                                Some(TasksWorkspaceAction::RefreshAll);
                                            cx.notify();
                                        }),
                                    )
                                    .child(if self.is_loading {
                                        tr_str(&lang, "同步中...", "Syncing...")
                                    } else {
                                        tr_str(&lang, "刷新", "Refresh")
                                    }),
                            ),
                    ),
            )
            .when_some(self.error_message.clone(), |el, err| {
                el.child(
                    div()
                        .px(px(12.0))
                        .py(px(6.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.14))
                        .border_1()
                        .border_color(rgb(pal.danger))
                        .text_xs()
                        .text_color(rgb(pal.danger))
                        .child(err),
                )
            })
            .child(
                div()
                    .id("hermes-studio-body-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_y_scroll()
                    .child(content),
            )
    }
}

#[allow(dead_code)]
fn _keep_icon_helper_linked(pal: &TdesktopThemePalette) -> AnyElement {
    tg_icon(IconName::BotRobot, 14.0, pal.accent).into_any_element()
}
