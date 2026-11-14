use gpui::{AppContext, AsyncApp, ClipboardItem, Context, Entity, WeakEntity};
use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDecisionRequest, HermesCommandRequest, HermesCreateKanbanBoardRequest,
    HermesCreateKanbanCardRequest, HermesHonchoRequest, HermesMemoryMutationRequest,
    HermesMoveKanbanCardRequest, HermesRollbackRequest, HermesSaveProviderRequest,
    HermesSaveSkillRequest, HermesSessionBranchRequest, HermesSessionSearchRequest,
    HermesUpdateToolsetsRequest,
};
use serde_json::json;

use crate::components::tool_permission_dock::ToolApprovalDecision;
use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::{
    contacts_to_modal_rows, resolve_active_target_chat_id, shell_mcp_to_entries,
    shell_tasks_to_acp_entries,
};
use crate::views::app_shell::shell_panels::{parse_acp_tasks, parse_mcp_servers, parse_mcp_tools};
use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::drawer::inspector_tabs::InspectorAction;
use crate::views::global_tasks_workspace::{CronHistoryEntry, TasksWorkspaceAction};
use crate::views::nav_rail::NavFolderItem;
use crate::views::settings::sections::settings_credits::StarsTransaction;
use crate::views::settings::sections::settings_folders::ChatFolderItem;
use crate::views::shell::AppShell;
use crate::views::sidebar::dialogs_row_view::SidebarContextAction;
use crate::views::MediaViewer;

fn preset_toolsets_for_name(preset: &str) -> Vec<String> {
    match preset {
        "safe" => vec![
            "core",
            "web",
            "file",
            "memory",
            "vision",
            "audio_tts",
            "audio_stt",
            "kanban",
            "todo",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
        "full" => vec![
            "core",
            "web",
            "terminal",
            "file",
            "memory",
            "skills",
            "vision",
            "image_gen",
            "video_gen",
            "browser",
            "audio_tts",
            "audio_stt",
            "cronjob",
            "subagents",
            "mcp",
            "kanban",
            "todo",
            "clarify",
            "approval",
            "session_search",
            "delegate",
            "honcho",
            "rl",
            "code_exec",
            "computer_use",
            "talk_mode",
            "telegram_bridge",
            "voice_call",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
        _ => vec![
            "core",
            "web",
            "terminal",
            "file",
            "memory",
            "skills",
            "vision",
            "audio_tts",
            "audio_stt",
            "cronjob",
            "subagents",
            "mcp",
            "kanban",
            "todo",
            "clarify",
            "approval",
            "session_search",
            "delegate",
            "honcho",
            "code_exec",
            "computer_use",
            "talk_mode",
            "telegram_bridge",
            "voice_call",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    }
}

impl AppShell {
    pub fn refresh_folders(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(folders) = client.list_folders().await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence.matches(
                        shell.is_authenticated,
                        shell.auth_generation,
                        0,
                        shell.session_token.as_deref(),
                        shell.user_id,
                    ) {
                        return;
                    }
                    let nav_folders: Vec<NavFolderItem> = folders
                        .iter()
                        .map(|f| NavFolderItem {
                            folder_id: f.id as i32,
                            title: f.title.clone(),
                            icon_hint: f.emoticon.clone().unwrap_or_default(),
                            unread_badge: 0,
                            is_muted: false,
                        })
                        .collect();
                    let cfg_folders: Vec<ChatFolderItem> = folders
                        .iter()
                        .map(|f| ChatFolderItem {
                            id: f.id as i32,
                            title: f.title.clone(),
                            icon_name: f.emoticon.clone().unwrap_or_else(|| "Folder".into()),
                            included_peers_count: f.include_chats.len(),
                            excluded_peers_count: f.exclude_chats.len(),
                            unread_count: 0,
                            is_removed: false,
                        })
                        .collect();
                    shell.nav_rail.update(cx, |r, _| r.set_folders(nav_folders));
                    shell
                        .settings_folders
                        .update(cx, |sf, _| sf.sync_folders(cfg_folders));
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub fn refresh_contacts(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(list) = client.list_contacts().await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence.matches(
                        shell.is_authenticated,
                        shell.auth_generation,
                        0,
                        shell.session_token.as_deref(),
                        shell.user_id,
                    ) {
                        return;
                    }
                    let rows = contacts_to_modal_rows(&list);
                    shell.contacts_list = list;
                    shell
                        .modal_container
                        .update(cx, |m, _| m.dialog.sync_contacts(rows));
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub fn refresh_wallet(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let uname = self.username.clone().unwrap_or_default();
        self.settings_main.update(cx, |sm, _| {
            sm.sync_profile(uname.clone(), String::new(), uname, String::new(), false, false)
        });
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let bal = client.wallet_balance().await.ok();
            let txs = client.wallet_transactions().await.unwrap_or_default();
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                if let Some(b) = bal {
                    let mapped: Vec<StarsTransaction> = txs
                        .into_iter()
                        .map(|t| StarsTransaction {
                            id: t.tx_id,
                            title: t.description,
                            date: t.created_at_ms.to_string(),
                            amount: t.amount_stars,
                            is_outgoing: t.amount_stars < 0 || t.kind == "gift_sent",
                        })
                        .collect();
                    let ton = b.ton_nano as f64 / 1_000_000_000.0;
                    shell.settings_credits.update(cx, |sc, _| {
                        sc.sync_wallet(b.stars_balance.max(0) as u64, ton, mapped)
                    });
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn refresh_hermes_commands(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(cmds) = client.hermes_list_commands(None).await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence.matches(
                        shell.is_authenticated,
                        shell.auth_generation,
                        0,
                        shell.session_token.as_deref(),
                        shell.user_id,
                    ) {
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

    pub fn refresh_tasks_and_mcp(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        self.panel_state.tasks_loading = true;
        self.panel_state.mcp_loading = true;
        self.tasks_workspace.update(cx, |w, _| w.set_loading(true, None));
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let active_bot_id = active_chat.and_then(|cid| {
            self.rich_chats_by_id
                .get(&cid)
                .and_then(|rc| if rc.is_bot { rc.peer_user_id.or(Some(cid)) } else { None })
                .or_else(|| self.chat_peer_user_ids.get(&cid).copied())
        });
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let tasks_res = client.list_tasks().await;
            let servers_res = client.list_mcp_servers().await;
            let tools_res = client.list_mcp_tools().await;
            let status_res = client.hermes_status().await.ok();
            let kanban_res = client.hermes_kanban_boards().await.ok();
            let approvals_res = client.hermes_list_approvals().await.ok();
            let skills_res = client.hermes_list_skills().await.ok();
            let memory_res = client.hermes_get_memory().await.ok();
            let soul_res = client.hermes_get_soul(active_bot_id).await.ok();
            let honcho_res = client.hermes_get_honcho().await.ok();
            let toolsets_res = client.hermes_list_toolsets().await.ok();
            let procs_res = client.hermes_terminal_processes().await.ok();
            let cron_res = client.hermes_list_cron().await.ok();
            let hb_res = client.hermes_get_heartbeat().await.ok();
            let sessions_res = client.hermes_list_sessions().await.ok();
            let providers_res = client.hermes_list_providers().await.ok();
            let usage_res = client.hermes_usage_stats().await.ok();
            let cmds_res = client.hermes_list_commands(None).await.ok();
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
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
                    if let Some(st) = status_res {
                        w.sync_studio_status(st);
                    }
                    if let Some(kb) = kanban_res {
                        w.sync_kanban_boards(kb);
                    }
                    if let Some(ap) = approvals_res {
                        w.sync_hermes_approvals(ap);
                    }
                    if let Some(sk) = skills_res {
                        w.sync_skills(sk);
                    }
                    if let Some(mem) = memory_res {
                        w.sync_memory_state(mem);
                    }
                    if let Some(soul) = soul_res {
                        w.sync_soul_state(soul);
                    }
                    if let Some(honcho) = honcho_res {
                        w.sync_honcho_state(honcho);
                    }
                    if let Some(ts) = toolsets_res {
                        w.sync_toolsets(ts);
                    }
                    if let Some(procs) = procs_res {
                        w.sync_terminal_processes(procs);
                    }
                    if let Some(cron) = cron_res {
                        w.sync_cron_jobs(cron);
                    }
                    if let Some(hb) = hb_res {
                        w.sync_heartbeat(hb);
                    }
                    if let Some(sess) = sessions_res {
                        w.sync_sessions(sess);
                    }
                    if let Some(prov) = providers_res {
                        w.sync_providers(prov);
                    }
                    if let Some(usage) = usage_res {
                        w.sync_usage_summary(usage);
                    }
                    w.set_loading(false, None);
                    w_cx.notify();
                });
                if let Some(cmds) = cmds_res {
                    shell.composer.update(cx, |c, c_cx| {
                        c.set_hermes_commands(cmds);
                        c_cx.notify();
                    });
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn handle_tasks_workspace_action(
        &mut self,
        action: TasksWorkspaceAction,
        cx: &mut Context<Self>,
    ) {
        let client = self.client.clone();
        let token = self.session_token.clone().unwrap_or_default();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let active_session = self.active_talk_session_id.clone();
        match action {
            TasksWorkspaceAction::RefreshAll => self.refresh_tasks_and_mcp(cx),
            TasksWorkspaceAction::CreateTask(goal) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .create_task(&json!({ "title": goal, "goal": goal }))
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::ApproveTask(task_id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.approve_task(&task_id, true).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::CallMcpTool(server_id, tool_name) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .call_mcp_tool(&json!({ "server_id": server_id, "tool_name": tool_name }))
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::ApprovePermission(req_id, decision) => {
                let allow = !matches!(decision, ToolApprovalDecision::Deny);
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .approve_mcp_permission(&json!({ "request_id": req_id, "approved": allow }))
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::CreateKanbanBoard(name) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_kanban_create_board(&HermesCreateKanbanBoardRequest {
                            name,
                            columns: Some(vec![
                                "Backlog".into(),
                                "In Progress".into(),
                                "Review".into(),
                                "Done".into(),
                            ]),
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::CreateKanbanCard {
                board_id,
                title,
                column,
                priority,
            } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_kanban_create_card(&HermesCreateKanbanCardRequest {
                            board_id,
                            title,
                            description: None,
                            column: Some(column),
                            priority: Some(priority),
                            assigned_agent: None,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::MoveKanbanCard { card_id, column } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_kanban_move_card(
                            &card_id,
                            &HermesMoveKanbanCardRequest {
                                column: Some(column),
                                ..Default::default()
                            },
                        )
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::DeleteKanbanCard(card_id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes.kanban_delete_card(&token, &card_id).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::DecideHermesApproval {
                approval_id,
                decision,
            } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_decide_approval(
                            &approval_id,
                            &HermesApprovalDecisionRequest {
                                decision,
                                reason: None,
                            },
                        )
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::SaveSkill {
                name,
                content,
                enabled,
            } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_save_skill(&HermesSaveSkillRequest {
                            name,
                            description: None,
                            category: Some("workspace".into()),
                            content,
                            enabled: Some(enabled),
                            tags: None,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::ToggleSkillEnabled { name, enabled } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes
                        .patch_skill(&token, &name, &json!({ "enabled": enabled }))
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::ApproveSkill(name) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes.approve_skill(&token, &name).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::DeleteSkill(name) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes_delete_skill(&name).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
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
                            bot_id: None,
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
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_run_command(&HermesCommandRequest {
                            command: cmd,
                            session_id: active_session,
                            chat_id: active_chat.map(|id| id as i64),
                            bot_id: None,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
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
                        .hermes
                        .save_memory(&token, &mem_md, Some(&user_md))
                        .await;
                    let updated = match saved {
                        Ok(m) => Ok(m),
                        Err(_) => {
                            client
                                .hermes_mutate_memory(&HermesMemoryMutationRequest {
                                    action: "set_raw".into(),
                                    scope: Some(scope),
                                    content: Some(content),
                                    ..Default::default()
                                })
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
                    if let Ok(m) = client.hermes_get_memory().await {
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
            TasksWorkspaceAction::SaveSoul(soul) => {
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
            TasksWorkspaceAction::QueryHoncho(query) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_post_honcho(&HermesHonchoRequest {
                            action: Some("query".into()),
                            enabled: None,
                            peer_name: None,
                            note: None,
                            query: Some(query),
                        })
                        .await;
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
            TasksWorkspaceAction::ToggleHoncho(enabled) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_post_honcho(&HermesHonchoRequest {
                            action: Some("toggle".into()),
                            enabled: Some(enabled),
                            peer_name: None,
                            note: None,
                            query: None,
                        })
                        .await;
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
            TasksWorkspaceAction::ToggleToolset { name, enabled } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .hermes_update_toolsets(&HermesUpdateToolsetsRequest {
                            enabled_toolsets: None,
                            toolset: Some(name),
                            enabled: Some(enabled),
                        })
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
                        .hermes_update_toolsets(&HermesUpdateToolsetsRequest {
                            enabled_toolsets: Some(enabled_toolsets),
                            toolset: None,
                            enabled: None,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        if let Ok(ts) = res {
                            s.tasks_workspace.update(cx, |w, w_cx| {
                                w.sync_toolsets(ts);
                                w_cx.notify();
                            });
                        }
                        s.window_runtime.enqueue_toast(
                            format!("Applied preset: {}", preset),
                            2200,
                        );
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
                            bot_id: None,
                        })
                        .await;
                    let _ = weak.update(app, |s, cx| {
                        s.window_runtime.enqueue_toast(
                            format!("Terminal backend: {}", backend),
                            2000,
                        );
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
                            bot_id: None,
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
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes_save_cron(&req).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
            TasksWorkspaceAction::ToggleCronJob { job_id, enabled } => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes
                        .update_cron(&token, &job_id, &json!({ "enabled": enabled }))
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
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
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes_delete_cron(&job_id).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
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
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client
                        .hermes_branch_session(
                            &session_id,
                            &HermesSessionBranchRequest {
                                branch_name: Some("studio-branch".into()),
                                from_turn_id: None,
                            },
                        )
                        .await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
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
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.hermes_save_provider(&req).await;
                    let _ = weak.update(app, |s, cx| s.refresh_tasks_and_mcp(cx));
                })
                .detach();
            }
        }
    }

    pub fn handle_inspector_action(&mut self, action: InspectorAction, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        match action {
            InspectorAction::StartVoiceCall(name) => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(false, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    self.start_webrtc_call(pid, name, false, cx);
                } else {
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "请选择私聊用户或机器人对话", "Select a direct peer or bot chat"),
                        2600,
                    );
                }
            }
            InspectorAction::StartVideoCall(name) => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(true, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    self.start_webrtc_call(pid, name, true, cx);
                } else {
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "请选择私聊用户或机器人对话", "Select a direct peer or bot chat"),
                        2600,
                    );
                }
            }
            InspectorAction::CopyText(txt) => {
                cx.write_to_clipboard(ClipboardItem::new_string(txt))
            }
            InspectorAction::ToggleNotifications(enabled) => {
                if let Some(cid) = active_chat {
                    let c = self.client.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        let _ = c.set_chat_muted(cid as i64, !enabled, None).await;
                    })
                    .detach();
                }
            }
            InspectorAction::OpenMedia { url, is_video } => {
                let pal = self.palette;
                self.media_viewer_overlay = Some(cx.new(|_| {
                    let mut m = MediaViewer::new(url, is_video);
                    m.apply_theme(pal, lang);
                    m
                }));
            }
            InspectorAction::OpenEditPeer => {
                let prof = &self.inspector.read(cx).profile;
                self.open_modal_dialog(
                    tr_str(lang, "编辑资料", "Edit Info"),
                    ModalDialogKind::EditPeer {
                        peer_title: prof.name.clone(),
                        peer_username: prof.username.clone(),
                        peer_bio: prof.bio.clone(),
                        is_channel: false,
                    },
                    420.0,
                    380.0,
                    cx,
                );
            }
            InspectorAction::RevokeInviteLink => {
                if let Some(cid) = active_chat {
                    let c = self.client.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        if let Ok(invs) = c.list_invites(cid as i64).await {
                            if let Some(first) = invs.first() {
                                let _ = c.revoke_invite(cid as i64, &first.invite_id).await;
                            }
                        }
                    })
                    .detach();
                }
            }
            InspectorAction::OpenAddMember => self.open_modal_dialog(
                tr_str(lang, "添加成员", "Add Member"),
                ModalDialogKind::Contacts,
                400.0,
                440.0,
                cx,
            ),
            InspectorAction::CycleMemberRole(_) | InspectorAction::ChangeAutoDelete(_) => {}
        }
        cx.notify();
    }

    pub fn handle_sidebar_context_action(
        &mut self,
        action: SidebarContextAction,
        cx: &mut Context<Self>,
    ) {
        self.sidebar
            .update(cx, |sb, _| sb.apply_context_action(action.clone()));
        let client = self.client.clone();
        match action {
            SidebarContextAction::TogglePin(id) => {
                let pinned = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .find(|d| d.id == id)
                    .map(|d| d.is_pinned)
                    .unwrap_or(false);
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_pinned(id as i64, pinned).await;
                })
                .detach();
            }
            SidebarContextAction::ToggleMute(id) => {
                let muted = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .find(|d| d.id == id)
                    .map(|d| d.is_muted)
                    .unwrap_or(false);
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_muted(id as i64, muted, None).await;
                })
                .detach();
            }
            SidebarContextAction::ToggleArchive(id) => {
                let arch = self.sidebar.read(cx).archived_ids.contains(&id);
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_archived(id as i64, arch).await;
                })
                .detach();
            }
            SidebarContextAction::DeleteChat(id) => {
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.delete_chat(id as i64).await;
                    let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                })
                .detach();
            }
            SidebarContextAction::ClearHistory(id) => {
                if self.stage.read(cx).active_chat_id == Some(id) {
                    self.stage.update(cx, |st, _| st.messages.clear());
                }
            }
            SidebarContextAction::ToggleUnread(_) => {}
        }
        cx.notify();
    }
}
