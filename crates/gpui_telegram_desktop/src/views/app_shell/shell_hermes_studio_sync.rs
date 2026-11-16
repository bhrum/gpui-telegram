//! Hermes Studio, ACP Tasks, MCP tools, Cron, Memory, and Provider state synchronization.

use std::sync::Arc;

use gpui::{AppContext, AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDecisionRequest, HermesCommandRequest, HermesCreateKanbanBoardRequest,
    HermesCreateKanbanCardRequest, HermesHonchoRequest, HermesMemoryMutationRequest,
    HermesMoveKanbanCardRequest, HermesRollbackRequest, HermesSaveProviderRequest,
    HermesSaveSkillRequest, HermesSessionBranchRequest, HermesSessionSearchRequest,
    HermesUpdateToolsetsRequest,
};
use gpui_telegram_client::TelegramClient;
use serde_json::json;

use crate::components::tool_permission_dock::ToolApprovalDecision;
use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::{
    resolve_active_target_chat_id, shell_mcp_to_entries, shell_tasks_to_acp_entries,
};
use crate::views::app_shell::shell_panels::{parse_acp_tasks, parse_mcp_servers, parse_mcp_tools};
use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::global_tasks_workspace::{CronHistoryEntry, TasksWorkspaceAction};
use crate::views::shell::AppShell;

fn preset_toolsets_for_name(preset: &str) -> Vec<String> {
    let items: &[&str] = match preset {
        "safe" => &[
            "core", "web", "file", "memory", "vision", "audio_tts", "audio_stt", "kanban", "todo",
        ],
        "full" => &[
            "core", "web", "terminal", "file", "memory", "skills", "vision", "image_gen",
            "video_gen", "browser", "audio_tts", "audio_stt", "cronjob", "subagents", "mcp",
            "kanban", "todo", "clarify", "approval", "session_search", "delegate", "honcho", "rl",
            "code_exec", "computer_use", "talk_mode", "telegram_bridge", "voice_call",
        ],
        _ => &[
            "core", "web", "terminal", "file", "memory", "skills", "vision", "audio_tts",
            "audio_stt", "cronjob", "subagents", "mcp", "kanban", "todo", "clarify", "approval",
            "session_search", "delegate", "honcho", "code_exec", "computer_use", "talk_mode",
            "telegram_bridge", "voice_call",
        ],
    };
    items.iter().map(|s| (*s).to_string()).collect()
}

fn session_fence(shell: &AppShell) -> Option<SessionRequestFence> {
    let (Some(token), Some(uid)) =
        (shell.session_sync.session_token.clone(), shell.session_sync.user_id)
    else {
        return None;
    };
    Some(SessionRequestFence {
        auth_generation: shell.session_sync.auth_generation,
        request_generation: 0,
        token,
        user_id: uid,
    })
}

fn fence_matches(fence: &SessionRequestFence, shell: &AppShell) -> bool {
    fence.matches(
        shell.is_authenticated,
        shell.session_sync.auth_generation,
        0,
        shell.session_sync.session_token.as_deref(),
        shell.session_sync.user_id,
    )
}

fn spawn_honcho_request(
    client: Arc<TelegramClient>,
    bot_id: Option<u64>,
    req: HermesHonchoRequest,
    cx: &mut Context<AppShell>,
) {
    cx.spawn(async move |weak: WeakEntity<AppShell>, app: &mut AsyncApp| {
        let res = client.hermes_post_honcho_for_bot(bot_id, &req).await;
        let _ = weak.update(app, |s, cx| {
            if let Ok(h) = res {
                s.tasks_workspace.update(cx, |w, w_cx| {
                    w.sync_honcho_state(h);
                    w_cx.notify();
                });
            }
            cx.notify();
        });
    })
    .detach();
}

macro_rules! spawn_and_refresh {
    ($cx:expr, $fut:expr) => {
        $cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let _ = $fut.await;
            let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
        })
        .detach()
    };
}

impl AppShell {
    /// Refreshes available Hermes slash commands for composer autocomplete.
    pub fn refresh_hermes_commands(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(cmds) = client.hermes_list_commands(None).await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches(&fence, shell) {
                        return;
                    }
                    shell.composer.update(cx, |c, c_cx| {
                        c.set_hermes_commands(cmds);
                        c_cx.notify();
                    });
                    cx.notify();
                });
            }
        })
        .detach();
    }

    /// Fetches ACP tasks, MCP servers/tools, and full Hermes Studio state from the backend.
    pub fn refresh_tasks_and_mcp(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        self.panel_state.tasks_loading = true;
        self.panel_state.mcp_loading = true;
        self.tasks_workspace.update(cx, |w, _| w.set_loading(true, None));
        let active_bot_id = self.active_chat_bot_id(cx);
        let tok_magpie = fence.token.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let tasks_res = client.list_tasks().await;
            let servers_res = client.list_mcp_servers().await;
            let tools_res = client.list_mcp_tools().await;
            let status_res = client.hermes_status_for_bot(active_bot_id).await.ok();
            let kanban_res = client.hermes_kanban_boards().await.ok();
            let approvals_res = client.hermes_list_approvals_for_bot(active_bot_id).await.ok();
            let skills_res = client.hermes_list_skills_for_bot(active_bot_id).await.ok();
            let memory_res = client.hermes_get_memory_for_bot(active_bot_id).await.ok();
            let soul_res = client.hermes_get_soul(active_bot_id).await.ok();
            let honcho_res = client.hermes_get_honcho_for_bot(active_bot_id).await.ok();
            let toolsets_res = client.hermes_list_toolsets_for_bot(active_bot_id).await.ok();
            let procs_res = client.hermes_terminal_processes().await.ok();
            let cron_res = client.hermes_list_cron().await.ok();
            let hb_res = client.hermes_get_heartbeat().await.ok();
            let sessions_res = client.hermes_list_sessions_for_bot(active_bot_id).await.ok();
            let providers_res = client.hermes_list_providers().await.ok();
            let usage_res = client.hermes_usage_stats().await.ok();
            let cmds_res = client.hermes_list_commands(None).await.ok();
            let magpie_res = client.hermes.magpie_state(&tok_magpie).await.ok();
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                shell.panel_state.tasks_loading = false;
                shell.panel_state.mcp_loading = false;
                if let Ok(v) = tasks_res {
                    shell.panel_state.tasks = parse_acp_tasks(&v);
                    shell.panel_state.tasks_loaded = true;
                }
                if let Ok(sv) = servers_res {
                    shell.panel_state.mcp_servers = parse_mcp_servers(&sv);
                }
                if let Ok(tv) = tools_res {
                    shell.panel_state.mcp_tools = parse_mcp_tools(&tv);
                }
                shell.panel_state.mcp_loaded = true;
                let acp = shell_tasks_to_acp_entries(&shell.panel_state.tasks);
                let mcp = shell_mcp_to_entries(
                    &shell.panel_state.mcp_servers,
                    &shell.panel_state.mcp_tools,
                );
                shell.tasks_workspace.update(cx, |w, w_cx| {
                    w.sync_tasks(acp);
                    w.sync_mcp_servers(mcp);
                    if let Some(st) = status_res { w.sync_studio_status(st); }
                    if let Some(kb) = kanban_res { w.sync_kanban_boards(kb); }
                    if let Some(ap) = approvals_res { w.sync_hermes_approvals(ap); }
                    if let Some(sk) = skills_res { w.sync_skills(sk); }
                    if let Some(mem) = memory_res { w.sync_memory_state(mem); }
                    if let Some(soul) = soul_res { w.sync_soul_state(soul); }
                    if let Some(honcho) = honcho_res { w.sync_honcho_state(honcho); }
                    if let Some(ts) = toolsets_res { w.sync_toolsets(ts); }
                    if let Some(procs) = procs_res { w.sync_terminal_processes(procs); }
                    if let Some(cron) = cron_res { w.sync_cron_jobs(cron); }
                    if let Some(hb) = hb_res { w.sync_heartbeat(hb); }
                    if let Some(sess) = sessions_res { w.sync_sessions(sess); }
                    if let Some(prov) = providers_res { w.sync_providers(prov); }
                    if let Some(usage) = usage_res { w.sync_usage_summary(usage); }
                    w.set_loading(false, None);
                    w_cx.notify();
                });
                if let Some(cmds) = cmds_res {
                    shell.composer.update(cx, |c, c_cx| {
                        c.set_hermes_commands(cmds);
                        c_cx.notify();
                    });
                }
                if let Some(ms) = magpie_res {
                    shell.apply_magpie_state_to_views(ms, cx);
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Dispatches actions emitted by the Global Tasks / Hermes Studio workspace panel.
    pub fn handle_tasks_workspace_action(
        &mut self,
        action: TasksWorkspaceAction,
        cx: &mut Context<Self>,
    ) {
        let client = self.client.clone();
        let token = self.session_sync.session_token.clone().unwrap_or_default();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let active_bot_id = self.active_chat_bot_id(cx);
        let active_session = self.rtc_talk.active_talk_session_id.clone();
        match action {
            TasksWorkspaceAction::RefreshAll => self.refresh_tasks_and_mcp(cx),
            TasksWorkspaceAction::CreateTask(goal) => {
                spawn_and_refresh!(cx, client.create_task(&json!({ "title": goal, "goal": goal })));
            }
            TasksWorkspaceAction::ApproveTask(task_id) => {
                spawn_and_refresh!(cx, client.approve_task(&task_id, true));
            }
            TasksWorkspaceAction::CallMcpTool(server_id, tool_name) => {
                spawn_and_refresh!(
                    cx,
                    client.call_mcp_tool(&json!({ "server_id": server_id, "tool_name": tool_name }))
                );
            }
            TasksWorkspaceAction::ApprovePermission(req_id, decision) => {
                let allow = !matches!(decision, ToolApprovalDecision::Deny);
                spawn_and_refresh!(
                    cx,
                    client.approve_mcp_permission(&json!({ "request_id": req_id, "approved": allow }))
                );
            }
            TasksWorkspaceAction::CreateKanbanBoard(name) => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_kanban_create_board(&HermesCreateKanbanBoardRequest {
                        name,
                        columns: Some(vec![
                            "Backlog".into(),
                            "In Progress".into(),
                            "Review".into(),
                            "Done".into(),
                        ]),
                    })
                );
            }
            TasksWorkspaceAction::CreateKanbanCard {
                board_id,
                title,
                column,
                priority,
            } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_kanban_create_card(&HermesCreateKanbanCardRequest {
                        board_id,
                        title,
                        description: None,
                        column: Some(column),
                        priority: Some(priority),
                        assigned_agent: None,
                    })
                );
            }
            TasksWorkspaceAction::MoveKanbanCard { card_id, column } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_kanban_move_card(
                        &card_id,
                        &HermesMoveKanbanCardRequest {
                            column: Some(column),
                            ..Default::default()
                        },
                    )
                );
            }
            TasksWorkspaceAction::DeleteKanbanCard(card_id) => {
                spawn_and_refresh!(cx, client.hermes.kanban_delete_card(&token, &card_id));
            }
            TasksWorkspaceAction::DecideHermesApproval {
                approval_id,
                decision,
            } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_decide_approval(
                        &approval_id,
                        &HermesApprovalDecisionRequest { decision, reason: None },
                    )
                );
            }
            TasksWorkspaceAction::SaveSkill {
                name,
                content,
                enabled,
            } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_save_skill_for_bot(
                        active_bot_id,
                        &HermesSaveSkillRequest {
                            name,
                            description: None,
                            category: Some("workspace".into()),
                            content,
                            enabled: Some(enabled),
                            tags: None,
                        }
                    )
                );
            }
            TasksWorkspaceAction::ToggleSkillEnabled { name, enabled } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes.patch_skill(&token, &name, &json!({ "enabled": enabled }))
                );
            }
            TasksWorkspaceAction::ApproveSkill(name) => {
                spawn_and_refresh!(cx, client.hermes.approve_skill(&token, &name));
            }
            TasksWorkspaceAction::DeleteSkill(name) => {
                spawn_and_refresh!(cx, client.hermes_delete_skill_for_bot(active_bot_id, &name));
            }
            TasksWorkspaceAction::LearnSkillFromSession(name) => {
                let cmd = if name.trim().is_empty() {
                    "/learn".to_string()
                } else {
                    format!("/learn {}", name.trim())
                };
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_run_command(&HermesCommandRequest {
                            command: cmd,
                            session_id: active_session,
                            chat_id: active_chat.map(|id| id as i64),
                            bot_id: active_bot_id,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(r) = res {
                            s.window_runtime.enqueue_toast(r.output, 2800);
                        }
                        s.refresh_tasks_and_mcp(cx);
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::ToggleNoBundledSkills(enabled) => {
                self.tasks_workspace.update(cx, |w, w_cx| {
                    w.no_bundled_skills = enabled;
                    w_cx.notify();
                });
                let cmd = if enabled {
                    "/skills no-bundled on".to_string()
                } else {
                    "/skills no-bundled off".to_string()
                };
                spawn_and_refresh!(
                    cx,
                    client.hermes_run_command(&HermesCommandRequest {
                        command: cmd,
                        session_id: active_session,
                        chat_id: active_chat.map(|id| id as i64),
                        bot_id: active_bot_id,
                    })
                );
            }
            TasksWorkspaceAction::SaveMemoryRaw { scope, content } => {
                let (mem_md, user_md) = {
                    let w = self.tasks_workspace.read(cx);
                    if scope == "user" {
                        (w.memory_edit_draft.clone(), content.clone())
                    } else {
                        (content.clone(), w.user_edit_draft.clone())
                    }
                };
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let saved = client
                        .hermes_save_memory_for_bot(active_bot_id, &mem_md, Some(&user_md))
                        .await;
                    let updated = match saved {
                        Ok(m) => Ok(m),
                        Err(_) => {
                            client
                                .hermes_mutate_memory_for_bot(
                                    active_bot_id,
                                    &HermesMemoryMutationRequest {
                                        action: "set_raw".into(),
                                        scope: Some(scope),
                                        content: Some(content),
                                        ..Default::default()
                                    },
                                )
                                .await
                        }
                    };
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(m) = updated {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_memory_state(m);
                                w_cx.notify();
                            });
                            s.window_runtime.enqueue_toast(
                                tr_str(
                                    s.active_lang_id(),
                                    "已保存记忆快照",
                                    "Saved MEMORY.md / USER.md",
                                ),
                                2200,
                            );
                        }
                        cx.notify();
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::ReloadFrozenSnapshot => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(m) = client.hermes_get_memory_for_bot(active_bot_id).await {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_memory_state(m);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::SaveSoul(mut soul) => {
                if soul.bot_id.is_none() {
                    soul.bot_id = active_bot_id;
                }
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client.hermes_save_soul(&soul).await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(saved) = res {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_soul_state(saved);
                                w_cx.notify();
                            });
                            s.window_runtime.enqueue_toast(
                                tr_str(
                                    s.active_lang_id(),
                                    "已保存 SOUL.md 人格",
                                    "Saved SOUL.md persona",
                                ),
                                2200,
                            );
                        }
                        cx.notify();
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::QueryHoncho(query) => spawn_honcho_request(
                client,
                active_bot_id,
                HermesHonchoRequest {
                    action: Some("query".into()),
                    enabled: None,
                    peer_name: None,
                    note: None,
                    query: Some(query),
                },
                cx,
            ),
            TasksWorkspaceAction::ToggleHoncho(enabled) => spawn_honcho_request(
                client,
                active_bot_id,
                HermesHonchoRequest {
                    action: Some("toggle".into()),
                    enabled: Some(enabled),
                    peer_name: None,
                    note: None,
                    query: None,
                },
                cx,
            ),
            TasksWorkspaceAction::ToggleToolset { name, enabled } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_update_toolsets_for_bot(
                            active_bot_id,
                            &HermesUpdateToolsetsRequest {
                                enabled_toolsets: None,
                                toolset: Some(name),
                                enabled: Some(enabled),
                            },
                        )
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(ts) = res {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_toolsets(ts);
                                w_cx.notify();
                            });
                        }
                        cx.notify();
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::ApplyToolsetPreset(preset) => {
                let enabled_toolsets = preset_toolsets_for_name(&preset);
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_update_toolsets_for_bot(
                            active_bot_id,
                            &HermesUpdateToolsetsRequest {
                                enabled_toolsets: Some(enabled_toolsets),
                                toolset: None,
                                enabled: None,
                            },
                        )
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(ts) = res {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_toolsets(ts);
                                w_cx.notify();
                            });
                        }
                        s.window_runtime
                            .enqueue_toast(format!("Applied preset: {}", preset), 2200);
                        cx.notify();
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::SetTerminalBackend(backend) => {
                self.tasks_workspace.update(cx, |w, w_cx| {
                    w.terminal_backend = backend.clone();
                    w_cx.notify();
                });
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_run_command(&HermesCommandRequest {
                            command: format!("/terminal backend {}", backend),
                            session_id: active_session,
                            chat_id: active_chat.map(|id| id as i64),
                            bot_id: active_bot_id,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        s.window_runtime
                            .enqueue_toast(format!("Terminal backend: {}", backend), 2000);
                        cx.notify();
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::PollTerminalProcesses => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(procs) = client.hermes_terminal_processes().await {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_terminal_processes(procs);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::KillTerminalProcess(process_id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_run_command(&HermesCommandRequest {
                            command: format!("/process kill {}", process_id),
                            session_id: active_session,
                            chat_id: active_chat.map(|id| id as i64),
                            bot_id: active_bot_id,
                        })
                        .await;
                    if let Ok(procs) = client.hermes_terminal_processes().await {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_terminal_processes(procs);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::SaveCronJob(req) => {
                spawn_and_refresh!(cx, client.hermes_save_cron(&req));
            }
            TasksWorkspaceAction::ToggleCronJob { job_id, enabled } => {
                spawn_and_refresh!(
                    cx,
                    client.hermes.update_cron(&token, &job_id, &json!({ "enabled": enabled }))
                );
            }
            TasksWorkspaceAction::RunCronNow(job_id) => {
                let job_name = self
                    .tasks_workspace
                    .read(cx)
                    .cron_jobs
                    .iter()
                    .find(|j| j.job_id == job_id)
                    .map(|j| j.name.clone())
                    .unwrap_or_else(|| job_id.clone());
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client.hermes_run_cron(&job_id).await;
                    let _ = weak.update(app, |s, cx| {
                        let (status, summary) = match res {
                            Ok(v) => (
                                "ok".to_string(),
                                v.get("summary")
                                    .or_else(|| v.get("output"))
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("Executed successfully")
                                    .to_string(),
                            ),
                            Err(e) => ("error".to_string(), e.to_string()),
                        };
                        s.tasks_workspace.update(cx, |w, w_cx| {
                            w.record_cron_history(CronHistoryEntry {
                                job_id,
                                job_name: job_name.clone(),
                                status,
                                summary: summary.clone(),
                                ran_at_ms: chrono::Utc::now().timestamp_millis(),
                            });
                            w_cx.notify();
                        });
                        s.window_runtime
                            .enqueue_toast(format!("{}: {}", job_name, summary), 2600);
                        s.refresh_tasks_and_mcp(cx);
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::DeleteCronJob(job_id) => {
                spawn_and_refresh!(cx, client.hermes_delete_cron(&job_id));
            }
            TasksWorkspaceAction::SaveHeartbeat(cfg) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(saved) = client.hermes_save_heartbeat(&cfg).await {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_heartbeat(saved);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::SearchSessions(query) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(hits) = client
                        .hermes_search_sessions(&HermesSessionSearchRequest {
                            query,
                            limit: Some(30),
                        })
                        .await
                    {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_session_search_hits(hits);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::BranchSession(session_id) => {
                spawn_and_refresh!(
                    cx,
                    client.hermes_branch_session(
                        &session_id,
                        &HermesSessionBranchRequest {
                            branch_name: Some("studio-branch".into()),
                            from_turn_id: None,
                        },
                    )
                );
            }
            TasksWorkspaceAction::CompressSession(session_id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes.compress_session(&token, &session_id).await;
                    let _ = weak.update(app, |s, cx| {
                        s.window_runtime.enqueue_toast("Session compressed", 2200);
                        s.refresh_tasks_and_mcp(cx);
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::ExportSession { session_id, format } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(exp) = client.hermes_export_session(&session_id, Some(&format)).await {
                        let _ = weak.update(app, |s, cx| {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_session_export(exp);
                                w_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
            TasksWorkspaceAction::RollbackSession(session_id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_rollback_turn(&HermesRollbackRequest {
                            session_id,
                            turn_id: None,
                            steps: Some(1),
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(rb) = res {
                            s.window_runtime.enqueue_toast(rb.message, 2600);
                        }
                        s.refresh_tasks_and_mcp(cx);
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::SetApiMode(mode) => {
                let provider_id = self
                    .tasks_workspace
                    .read(cx)
                    .providers
                    .iter()
                    .find(|p| p.is_active)
                    .or_else(|| self.tasks_workspace.read(cx).providers.first())
                    .map(|p| p.provider_id.clone())
                    .unwrap_or_else(|| "default".to_string());
                self.tasks_workspace.update(cx, |w, w_cx| {
                    w.active_api_mode = mode.clone();
                    w_cx.notify();
                });
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_save_provider(&HermesSaveProviderRequest {
                            provider_id,
                            api_mode: Some(mode.clone()),
                            set_active: Some(true),
                            ..Default::default()
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        s.window_runtime
                            .enqueue_toast(format!("API Mode: {}", mode), 2200);
                        s.refresh_tasks_and_mcp(cx);
                    });
                })
                .detach();
            }
            TasksWorkspaceAction::SaveProviderConfig(req) => {
                spawn_and_refresh!(cx, client.hermes_save_provider(&req));
            }
            TasksWorkspaceAction::OpenAiProvidersSettings => {
                self.open_ai_providers_settings(cx);
            }
            TasksWorkspaceAction::ScanMagpieSubscriptions => {
                self.run_magpie_subscription_scan(cx);
            }
            TasksWorkspaceAction::ProbeMagpieProvider(pid) => {
                self.run_magpie_probe(pid, None, cx);
            }
        }
    }
}
