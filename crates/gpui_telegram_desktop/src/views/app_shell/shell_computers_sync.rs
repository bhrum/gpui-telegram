//! Multi-device & cloud computer synchronization, local relay execution loop,
//! Bot-to-computer profile bindings, and remote workbench command dispatch.

use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use gpui::{App, AppContext, AsyncApp, Context, Entity, WeakEntity};
use gpui_telegram_client::TelegramClient;
use gpui_telegram_proto::computer_contracts::{
    ComputerListResponse, ComputerNodeDto, ComputerNodeKind, PendingComputerCommandDto,
    RemoteComputerCommandRequest, RemoteComputerOp, UpdateBotComputerRequest,
    UpdateComputerPolicyRequest, UpsertCloudComputerRequest,
};

use crate::i18n::tr_str;
use crate::views::computer_workbench::{ComputerWorkbench, WorkbenchCommand};
use crate::views::drawer::inspector_tabs::{InspectorAction, InspectorSubpage};
use crate::views::settings::sections::SettingsComputersEvent;
use crate::views::settings::settings_type::SettingsSectionType;
use crate::views::shell::{ActiveWorkspacePanel, AppShell};

async fn run_on_tokio<F, T>(client: &Arc<TelegramClient>, fut: F) -> Result<T, String>
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
{
    client
        .runtime()
        .spawn(fut)
        .await
        .map_err(|e| format!("tokio task join error: {e}"))
}

#[derive(Debug, Clone, Copy)]
struct ComputerAuthFence {
    auth_generation: u64,
    relay_generation: u64,
}

impl AppShell {
    fn enqueue_computer_toast(&mut self, zh: &str, en: &str, duration_ms: u64) {
        let lang = self.active_lang_id().to_string();
        self.window_runtime.enqueue_toast(tr_str(&lang, zh, en), duration_ms);
    }

    fn computer_fence(&self) -> Option<ComputerAuthFence> {
        if !self.is_authenticated || self.session_sync.session_token.is_none() {
            return None;
        }
        Some(ComputerAuthFence {
            auth_generation: self.session_sync.auth_generation,
            relay_generation: self.computers.relay_generation,
        })
    }

    fn computer_fence_matches(&self, fence: ComputerAuthFence) -> bool {
        self.is_authenticated
            && self.session_sync.session_token.is_some()
            && self.session_sync.auth_generation == fence.auth_generation
            && self.computers.relay_generation == fence.relay_generation
    }

    /// Collects unique AI Bot `(bot_id, display_name)` pairs known to this workspace.
    pub fn known_bots_for_computers(&self, cx: &App) -> Vec<(u64, String)> {
        let mut by_id: BTreeMap<u64, String> = BTreeMap::new();
        for rc in self.session_sync.rich_chats_by_id.values() {
            if rc.is_bot {
                let id = rc
                    .peer_user_id
                    .unwrap_or_else(|| rc.chat_id.max(0) as u64);
                if id > 0 {
                    let name = rc
                        .peer_display_name
                        .as_deref()
                        .filter(|s| !s.trim().is_empty())
                        .unwrap_or(&rc.title)
                        .to_string();
                    by_id.entry(id).or_insert(name);
                }
            }
        }
        for app in self
            .miniapps
            .installed_miniapps
            .iter()
            .chain(self.miniapps.catalog_miniapps.iter())
        {
            if app.bot_user_id > 0 {
                let id = app.bot_user_id as u64;
                by_id
                    .entry(id)
                    .or_insert_with(|| format!("@{} ({})", app.bot_username, app.title));
            }
        }
        let sb = self.sidebar.read(cx);
        for cid in &sb.bot_ids {
            if let Some(chat) = sb.dialogs.iter().find(|c| c.id == *cid) {
                let resolved_id = self
                    .session_sync
                    .rich_chats_by_id
                    .get(cid)
                    .and_then(|rc| rc.peer_user_id)
                    .unwrap_or(*cid);
                if resolved_id > 0 {
                    by_id
                        .entry(resolved_id)
                        .or_insert_with(|| chat.peer_name.clone());
                }
            }
        }
        let insp = self.inspector.read(cx);
        if insp.profile.is_bot && insp.profile.bot_id > 0 && !insp.profile.name.is_empty() {
            by_id.insert(insp.profile.bot_id, insp.profile.name.clone());
        }
        by_id.into_iter().collect()
    }

    fn effective_inspector_bot_id(&self, cx: &App) -> Option<u64> {
        let insp = self.inspector.read(cx);
        if !insp.profile.is_bot {
            return None;
        }
        if insp.profile.bot_id > 0 {
            return Some(insp.profile.bot_id);
        }
        let selected_chat = self.sidebar.read(cx).active_chat_id?;
        Some(
            self.session_sync
                .rich_chats_by_id
                .get(&selected_chat)
                .and_then(|rc| rc.peer_user_id)
                .unwrap_or(selected_chat),
        )
    }

    /// Synchronizes cached computers and bot computer profiles into Inspector, Settings, and Workbench.
    pub fn sync_computers_to_views(&mut self, cx: &mut Context<Self>) {
        let nodes = self.computers.computers.clone();
        let tailnet_status = self.computers.tailnet_status.clone();
        let bots = self.known_bots_for_computers(cx);
        let active_bot_id = self.effective_inspector_bot_id(cx);
        let active_bot_profile = active_bot_id
            .and_then(|id| self.computers.bot_profiles.get(&id).cloned());

        self.inspector.update(cx, |insp, icx| {
            insp.sync_computers(nodes.clone());
            if let Some(id) = active_bot_id {
                if insp.profile.is_bot && insp.profile.bot_id == 0 {
                    insp.profile.bot_id = id;
                }
            }
            if insp.profile.is_bot {
                if let Some(prof) = active_bot_profile {
                    insp.sync_bot_computer_profile(Some(prof));
                }
            }
            icx.notify();
        });

        let sr_computers_opt = self.settings_router.read(cx).settings_computers.clone();
        if let Some(sc_ent) = sr_computers_opt {
            sc_ent.update(cx, |sc, sc_cx| {
                sc.sync_computers(nodes.clone());
                sc.sync_known_bots(bots, active_bot_id);
                sc.sync_tailnet_status(tailnet_status.clone());
                sc_cx.notify();
            });
        }

        if let Some(ref wb_ent) = self.overlays.computer_overlay.clone() {
            let target_id = wb_ent.read(cx).node_id.clone();
            if let Some(tid) = target_id {
                let node_opt = self.computers.computers.iter().find(|n| n.node_id == tid).cloned();
                let peer_opt = tailnet_status.as_ref().and_then(|ts| {
                    ts.peers.iter().find(|p| {
                        p.node_id == tid
                            || node_opt
                                .as_ref()
                                .and_then(|n| n.device_id.as_deref())
                                .is_some_and(|did| !did.is_empty() && p.device_id == did)
                    })
                });
                if node_opt.is_some() || peer_opt.is_some() {
                    wb_ent.update(cx, |wb, wcx| {
                        if let Some(ref node_clone) = node_opt {
                            wb.refresh_target_node_status(node_clone);
                        }
                        wb.sync_tailnet_peer_state(peer_opt);
                        wcx.notify();
                    });
                }
            }
        }
    }

    fn apply_computer_list_response(&mut self, resp: ComputerListResponse) {
        self.computers.computers = resp.computers;
        if let Some(host_node) = self
            .computers
            .computers
            .iter()
            .find(|n| n.is_current_device)
        {
            self.computers.current_host_node_id = Some(host_node.node_id.clone());
        }
        self.computers.pending_approvals = resp.pending_approvals;
    }

    fn merge_pending_approvals(&mut self, incoming: Vec<PendingComputerCommandDto>) {
        for item in incoming {
            if item.requires_user_approval && !item.approved {
                if !self
                    .computers
                    .pending_approvals
                    .iter()
                    .any(|p| p.command_id == item.command_id)
                {
                    self.computers.pending_approvals.push(item);
                }
            }
        }
    }

    /// Registers the current local machine on login and starts the background local-command relay loop.
    pub fn register_current_host_and_start_relay(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = self.computer_fence() else {
            return;
        };
        if self.computers.relay_loop_started {
            return;
        }
        self.computers.relay_loop_started = true;
        self.client.bind_persistent_identity_dir(&self.preferences_dir);
        self.client.start_tailnet_mesh();
        let client = self.client.clone();

        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let reg_client = client.clone();
            let (reg_res, list_res, tailnet_st, tailnet_waiting) = run_on_tokio(&client, async move {
                let reg = reg_client.register_current_host_computer_async().await;
                let list = reg_client.list_computers_async().await;
                let ts = reg_client.tailnet_status();
                let waiting = reg_client.tailnet_pending_approvals();
                (reg, list, ts, waiting)
            })
            .await
            .map(|(reg, list, ts, waiting)| {
                (
                    reg.map_err(|e| e.to_string()),
                    list.map_err(|e| e.to_string()),
                    Some(ts),
                    waiting,
                )
            })
            .unwrap_or_else(|e| (Err(e.clone()), Err(e), None, Vec::new()));

            let keep = weak
                .update(app, |shell, cx| {
                    if !shell.computer_fence_matches(fence) {
                        return false;
                    }
                    if let Ok(node) = reg_res {
                        shell.computers.current_host_node_id = Some(node.node_id);
                    }
                    if let Ok(list) = list_res {
                        shell.apply_computer_list_response(list);
                    }
                    if let Some(ts) = tailnet_st {
                        shell.computers.tailnet_status = Some(ts);
                    }
                    shell.merge_pending_approvals(tailnet_waiting);
                    shell.sync_computers_to_views(cx);
                    cx.notify();
                    true
                })
                .unwrap_or(false);

            if !keep {
                return;
            }

            let mut tick_count: u64 = 0;
            loop {
                app.background_executor()
                    .timer(Duration::from_millis(2000))
                    .await;
                tick_count = tick_count.wrapping_add(1);

                let still_valid = weak
                    .update(app, |shell, _| shell.computer_fence_matches(fence))
                    .unwrap_or(false);
                if !still_valid {
                    break;
                }

                let do_heartbeat = tick_count % 10 == 0;
                let poll_client = client.clone();
                let (relay_res, list_res, tailnet_st, tailnet_waiting) =
                    run_on_tokio(&client, async move {
                        if do_heartbeat {
                            let _ = poll_client.register_current_host_computer_async().await;
                        }
                        let relay = poll_client.process_ready_local_computer_commands_async().await;
                        let list = poll_client.list_computers_async().await;
                        let ts = poll_client.tailnet_status();
                        let waiting = poll_client.tailnet_pending_approvals();
                        (relay, list, ts, waiting)
                    })
                    .await
                    .map(|(r, l, ts, waiting)| {
                        (
                            r.map_err(|e| e.to_string()),
                            l.map_err(|e| e.to_string()),
                            Some(ts),
                            waiting,
                        )
                    })
                    .unwrap_or_else(|e| (Err(e.clone()), Err(e), None, Vec::new()));

                let cont = weak
                    .update(app, |shell, cx| {
                        if !shell.computer_fence_matches(fence) {
                            return false;
                        }
                        if let Ok(list) = list_res {
                            shell.apply_computer_list_response(list);
                        }
                        if let Ok(waiting) = relay_res {
                            shell.merge_pending_approvals(waiting);
                        }
                        if let Some(ts) = tailnet_st {
                            shell.computers.tailnet_status = Some(ts);
                        }
                        shell.merge_pending_approvals(tailnet_waiting);
                        shell.sync_computers_to_views(cx);
                        cx.notify();
                        true
                    })
                    .unwrap_or(false);

                if !cont {
                    break;
                }
            }
        })
        .detach();
    }

    /// Refreshes the computer inventory and pending approvals off the UI thread.
    pub fn refresh_computers(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = self.computer_fence() else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let c = client.clone();
            let (res, ts, waiting) = run_on_tokio(&client, async move {
                let list = c.list_computers_async().await.map_err(|e| e.to_string());
                let ts = c.tailnet_status();
                let waiting = c.tailnet_pending_approvals();
                (list, Some(ts), waiting)
            })
            .await
            .unwrap_or_else(|e| (Err(e), None, Vec::new()));
            let _ = weak.update(app, |shell, cx| {
                if !shell.computer_fence_matches(fence) {
                    return;
                }
                if let Some(ts) = ts {
                    shell.computers.tailnet_status = Some(ts);
                }
                let ok = res.is_ok();
                if let Ok(list) = res {
                    shell.apply_computer_list_response(list);
                }
                shell.merge_pending_approvals(waiting);
                if ok || shell.computers.tailnet_status.is_some() {
                    shell.sync_computers_to_views(cx);
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Ensures the active Inspector AI Bot gets its computer inventory and `BotComputerProfileDto`.
    pub fn sync_active_bot_computer_to_inspector(&mut self, cx: &mut Context<Self>) {
        self.sync_computers_to_views(cx);
        let Some(bot_id) = self.effective_inspector_bot_id(cx) else {
            return;
        };
        let Some(fence) = self.computer_fence() else {
            return;
        };
        if self.computers.computers.is_empty() {
            self.refresh_computers(cx);
        }
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let c = client.clone();
            let res = run_on_tokio(&client, async move {
                c.get_bot_computer_profile_async(bot_id).await.map_err(|e| e.to_string())
            })
            .await
            .and_then(|inner| inner);
            let _ = weak.update(app, |shell, cx| {
                if !shell.computer_fence_matches(fence) {
                    return;
                }
                if let Ok(profile) = res {
                    shell.computers.bot_profiles.insert(bot_id, profile.clone());
                    if shell.effective_inspector_bot_id(cx) == Some(bot_id) {
                        shell.inspector.update(cx, |insp, icx| {
                            insp.sync_bot_computer_profile(Some(profile));
                            icx.notify();
                        });
                    }
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Opens the interactive `ComputerWorkbench` overlay bound to `node_id`.
    pub fn open_remote_workbench_for_node(&mut self, node_id: String, cx: &mut Context<Self>) {
        let pal = self.theme_state.palette;
        let lang = self.active_lang_id().to_string();
        let node_opt = self
            .computers
            .computers
            .iter()
            .find(|c| c.node_id == node_id)
            .cloned();
        let fallback_name = node_opt
            .as_ref()
            .map(|n| n.display_name.clone())
            .unwrap_or_else(|| node_id.clone());
        let peer_opt = self
            .computers
            .tailnet_status
            .as_ref()
            .and_then(|ts| {
                ts.peers.iter().find(|p| {
                    p.node_id == node_id
                        || node_opt
                            .as_ref()
                            .and_then(|n| n.device_id.as_deref())
                            .is_some_and(|did| !did.is_empty() && p.device_id == did)
                })
            })
            .cloned();

        let overlay = cx.new(|wcx| {
            let mut wb = ComputerWorkbench::new_live(fallback_name, pal, &lang);
            if let Some(ref node) = node_opt {
                wb.set_target_node(node);
            } else {
                wb.node_id = Some(node_id.clone());
            }
            if peer_opt.is_some() {
                wb.sync_tailnet_peer_state(peer_opt.as_ref());
            }
            wb.ensure_terminal_input(wcx);
            wb
        });

        let nid_for_obs = node_id.clone();
        cx.observe(&overlay, move |shell, ov, cx| {
            let (close, cmd) = ov.update(cx, |w, _| (w.should_close, w.take_command()));
            if close {
                shell.hide_remote_computer(cx);
                return;
            }
            if let Some(command) = cmd {
                shell.handle_workbench_command(nid_for_obs.clone(), command, ov.clone(), cx);
            }
        })
        .detach();

        self.overlays.computer_overlay = Some(overlay.clone());
        self.handle_workbench_command(node_id, WorkbenchCommand::RefreshScreen, overlay, cx);
        cx.notify();
    }

    /// Executes an interactive `WorkbenchCommand` on `node_id` and feeds results back into `overlay`.
    pub fn handle_workbench_command(
        &mut self,
        node_id: String,
        cmd: WorkbenchCommand,
        overlay: Entity<ComputerWorkbench>,
        cx: &mut Context<Self>,
    ) {
        let Some(fence) = self.computer_fence() else {
            return;
        };
        let operation = match &cmd {
            WorkbenchCommand::RefreshScreen => RemoteComputerOp::CaptureScreen,
            WorkbenchCommand::SendGuiClick { x, y } => RemoteComputerOp::GuiInput {
                action: "click".to_string(),
                x: Some(*x),
                y: Some(*y),
                text: None,
                key: None,
            },
            WorkbenchCommand::SendGuiText { text } => RemoteComputerOp::GuiInput {
                action: "type".to_string(),
                x: None,
                y: None,
                text: Some(text.clone()),
                key: None,
            },
            WorkbenchCommand::RunShell { command, cwd } => RemoteComputerOp::ShellExec {
                command: command.clone(),
                cwd: cwd.clone(),
                timeout_ms: Some(15_000),
            },
            WorkbenchCommand::ListFiles { path } => RemoteComputerOp::ListDir { path: path.clone() },
            WorkbenchCommand::ReadRemoteFile { path } => RemoteComputerOp::ReadFile {
                path: path.clone(),
                max_bytes: Some(64 * 1024),
            },
        };

        let caller_label = self
            .session_sync
            .username
            .clone()
            .unwrap_or_else(|| "Desktop".to_string());
        let client = self.client.clone();
        let req = RemoteComputerCommandRequest {
            node_id: node_id.clone(),
            caller_device_id: Some(client.device_id()),
            caller_bot_id: None,
            caller_label,
            operation,
        };

        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let c = client.clone();
            let res = run_on_tokio(&client, async move {
                c.execute_remote_computer_op_async(&req).await.map_err(|e| e.to_string())
            })
            .await
            .and_then(|inner| inner);

            let _ = weak.update(app, |shell, cx| {
                if !shell.computer_fence_matches(fence) {
                    return;
                }
                match res {
                    Ok(out) => {
                        let pending_hint = !out.ok
                            && (out.stderr.contains("approval")
                                || out.stderr.contains("pending")
                                || out.stderr.contains("审批"));
                        if let Some(ref frame) = out.frame_data_url {
                            if let Some(node) = shell
                                .computers
                                .computers
                                .iter_mut()
                                .find(|n| n.node_id == node_id)
                            {
                                node.latest_frame_data_url = Some(frame.clone());
                            }
                            shell.sync_computers_to_views(cx);
                        }
                        overlay.update(cx, |wb, wcx| {
                            match &cmd {
                                WorkbenchCommand::RefreshScreen => {
                                    if out.frame_data_url.is_some() {
                                        wb.update_frame_url(out.frame_data_url.clone());
                                        wb.status_banner = Some(format!(
                                            "{} ({}ms)",
                                            tr_str(&wb.language_id, "已更新实时画面", "Screen captured"),
                                            out.duration_ms
                                        ));
                                    } else if !out.stderr.is_empty() {
                                        wb.status_banner = Some(out.stderr.clone());
                                    }
                                }
                                WorkbenchCommand::SendGuiClick { x, y } => {
                                    wb.status_banner = Some(if out.ok {
                                        format!(
                                            "{} ({x}, {y})",
                                            tr_str(&wb.language_id, "远程点击已生效", "Click applied")
                                        )
                                    } else {
                                        out.stderr.clone()
                                    });
                                }
                                WorkbenchCommand::SendGuiText { .. } => {
                                    wb.status_banner = Some(if out.ok {
                                        tr_str(&wb.language_id, "文本已发送到远程桌面", "Text delivered")
                                            .to_string()
                                    } else {
                                        out.stderr.clone()
                                    });
                                }
                                WorkbenchCommand::RunShell { command, .. } => {
                                    wb.push_terminal_output(
                                        command.clone(),
                                        out.stdout.clone(),
                                        out.stderr.clone(),
                                        out.exit_code,
                                        None,
                                    );
                                    if pending_hint {
                                        wb.status_banner = Some(out.stderr.clone());
                                    }
                                }
                                WorkbenchCommand::ListFiles { path } => {
                                    wb.set_remote_files(path.clone(), out.entries.clone());
                                    if !out.ok && !out.stderr.is_empty() {
                                        wb.status_banner = Some(out.stderr.clone());
                                    }
                                }
                                WorkbenchCommand::ReadRemoteFile { path } => {
                                    if out.ok {
                                        wb.set_file_preview(path.clone(), out.stdout.clone());
                                    } else if !out.stderr.is_empty() {
                                        wb.status_banner = Some(out.stderr.clone());
                                    }
                                }
                            }
                            wcx.notify();
                        });
                        if pending_hint {
                            shell.refresh_computers(cx);
                        }
                    }
                    Err(e) => {
                        overlay.update(cx, |wb, wcx| {
                            wb.status_banner = Some(format!(
                                "{}: {e}",
                                tr_str(&wb.language_id, "远程操作失败", "Remote command failed")
                            ));
                            wcx.notify();
                        });
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Handles computer-related actions emitted from `ContextInspector`. Returns `true` when consumed.
    pub fn handle_computer_inspector_action(
        &mut self,
        action: InspectorAction,
        cx: &mut Context<Self>,
    ) -> bool {
        let fallback_bot_id = self.effective_inspector_bot_id(cx);
        match action {
            InspectorAction::OpenBotComputersSubpage => {
                self.refresh_computers(cx);
                true
            }
            InspectorAction::CloseBotComputersSubpage => true,
            InspectorAction::OpenRemoteWorkbench { node_id } => {
                self.open_remote_workbench_for_node(node_id, cx);
                true
            }
            InspectorAction::OpenComputerDetailInSettings { node_id } => {
                self.open_computers_settings_page(Some(node_id), None, cx);
                true
            }
            InspectorAction::OpenCreateCloudComputerModal { bind_bot_id } => {
                let target_bot = bind_bot_id.or(fallback_bot_id);
                self.open_computers_settings_page(None, Some(target_bot), cx);
                true
            }
            InspectorAction::BindBotToComputer { bot_id, node_id } => {
                let resolved_bot = if bot_id > 0 {
                    Some(bot_id)
                } else {
                    fallback_bot_id
                };
                if let Some(bid) = resolved_bot {
                    let req = UpdateBotComputerRequest {
                        bound_node_id: node_id.clone(),
                        clear_bound_node: node_id.is_none(),
                        ..Default::default()
                    };
                    self.persist_bot_computer_profile(bid, req, false, cx);
                }
                true
            }
            InspectorAction::SaveBotComputerConfig {
                bot_id,
                persona_prompt,
                preferred_model,
                approval_mode,
                allow_cross_device_control,
                allow_cloud_self_config,
            } => {
                let resolved_bot = if bot_id > 0 {
                    Some(bot_id)
                } else {
                    fallback_bot_id
                };
                if let Some(bid) = resolved_bot {
                    let req = UpdateBotComputerRequest {
                        persona_prompt: Some(persona_prompt),
                        preferred_model: Some(preferred_model),
                        approval_mode: Some(approval_mode),
                        allow_cross_device_control: Some(allow_cross_device_control),
                        allow_cloud_self_config: Some(allow_cloud_self_config),
                        ..Default::default()
                    };
                    self.persist_bot_computer_profile(bid, req, true, cx);
                }
                true
            }
            _ => false,
        }
    }

    fn persist_bot_computer_profile(
        &mut self,
        bot_id: u64,
        req: UpdateBotComputerRequest,
        return_to_main_subpage: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(fence) = self.computer_fence() else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let c = client.clone();
            let res = run_on_tokio(&client, async move {
                c.update_bot_computer_profile_async(bot_id, &req).await.map_err(|e| e.to_string())
            })
            .await
            .and_then(|inner| inner);
            let _ = weak.update(app, |shell, cx| {
                if !shell.computer_fence_matches(fence) {
                    return;
                }
                match res {
                    Ok(profile) => {
                        shell.computers.bot_profiles.insert(bot_id, profile.clone());
                        shell.inspector.update(cx, |insp, icx| {
                            if !profile.persona_prompt.is_empty() {
                                insp.profile.bot_persona_prompt = profile.persona_prompt.clone();
                            }
                            insp.sync_bot_computer_profile(Some(profile));
                            if return_to_main_subpage {
                                insp.subpage = InspectorSubpage::Main;
                            }
                            icx.notify();
                        });
                        shell.enqueue_computer_toast(
                            "Bot 电脑与运行配置已保存",
                            "Bot computer configuration saved",
                            2000,
                        );
                        shell.refresh_computers(cx);
                    }
                    Err(e) => {
                        let pfx = tr_str(shell.active_lang_id(), "保存 Bot 配置失败", "Failed to save bot config");
                        shell.window_runtime.enqueue_toast(format!("{pfx}: {e}"), 2600);
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    fn open_computers_settings_page(
        &mut self,
        detail_node_id: Option<String>,
        create_cloud_for_bot: Option<Option<u64>>,
        cx: &mut Context<Self>,
    ) {
        let nodes = self.computers.computers.clone();
        let tailnet_status = self.computers.tailnet_status.clone();
        let bots = self.known_bots_for_computers(cx);
        let active_bot_id = self.effective_inspector_bot_id(cx);
        self.settings_router.update(cx, |sr, scx| {
            sr.navigate_to(SettingsSectionType::Computers, true, scx);
            let sc = sr.ensure_computers(scx);
            sc.update(scx, |view, vcx| {
                view.sync_computers(nodes);
                view.sync_known_bots(bots, active_bot_id);
                view.sync_tailnet_status(tailnet_status);
                if let Some(nid) = detail_node_id {
                    view.open_detail(&nid);
                } else if let Some(bot_opt) = create_cloud_for_bot {
                    view.open_cloud_form(bot_opt, None);
                } else {
                    view.open_overview();
                }
                vcx.notify();
            });
            scx.notify();
        });
        self.active_panel = ActiveWorkspacePanel::Settings;
        self.refresh_computers(cx);
        cx.notify();
    }

    /// Handles events emitted by `SettingsComputersSections` via `SettingsRouter`.
    pub fn handle_computer_settings_event(
        &mut self,
        ev: &SettingsComputersEvent,
        cx: &mut Context<Self>,
    ) {
        match ev {
            SettingsComputersEvent::SelectNode(_) => {
                self.refresh_computers(cx);
            }
            SettingsComputersEvent::BackToOverview => {
                self.sync_computers_to_views(cx);
            }
            SettingsComputersEvent::OpenCreateCloudModal => {}
            SettingsComputersEvent::RefreshComputers => {
                self.refresh_computers(cx);
            }
            SettingsComputersEvent::OpenRemoteWorkbench(node_id) => {
                self.open_remote_workbench_for_node(node_id.clone(), cx);
            }
            SettingsComputersEvent::SaveCloudServer(req) => {
                let Some(fence) = self.computer_fence() else {
                    return;
                };
                let client = self.client.clone();
                let req = req.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let c = client.clone();
                    let res = run_on_tokio(&client, async move {
                        c.upsert_cloud_computer_async(&req).await.map_err(|e| e.to_string())
                    })
                    .await
                    .and_then(|inner| inner);
                    let _ = weak.update(app, |shell, cx| {
                        if !shell.computer_fence_matches(fence) {
                            return;
                        }
                        match res {
                            Ok(node) => {
                                let detail_id = node.node_id.clone();
                                if let Some(sc_ent) =
                                    shell.settings_router.read(cx).settings_computers.clone()
                                {
                                    sc_ent.update(cx, |sc, sc_cx| {
                                        sc.status_message = Some(
                                            tr_str(
                                                &sc.language_id,
                                                "云服务器连接验证通过并已保存",
                                                "Cloud server verified & saved",
                                            )
                                            .to_string(),
                                        );
                                        sc.open_detail(&detail_id);
                                        sc_cx.notify();
                                    });
                                }
                                shell.enqueue_computer_toast(
                                    "云服务器已保存到云端",
                                    "Cloud server saved",
                                    2200,
                                );
                                shell.refresh_computers(cx);
                            }
                            Err(e) => {
                                let err_msg = format!(
                                    "{}: {e}",
                                    tr_str(shell.active_lang_id(), "保存云服务器失败", "Failed to save cloud server")
                                );
                                if let Some(sc_ent) =
                                    shell.settings_router.read(cx).settings_computers.clone()
                                {
                                    sc_ent.update(cx, |sc, sc_cx| {
                                        sc.status_message = Some(err_msg.clone());
                                        sc_cx.notify();
                                    });
                                }
                                shell.window_runtime.enqueue_toast(err_msg, 2800);
                                cx.notify();
                            }
                        }
                    });
                })
                .detach();
            }
            SettingsComputersEvent::UpdatePolicy { node_id, req } => {
                let Some(fence) = self.computer_fence() else {
                    return;
                };
                let client = self.client.clone();
                let nid = node_id.clone();
                let req = req.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let c = client.clone();
                    let res = run_on_tokio(&client, async move {
                        c.update_computer_policy_async(&nid, &req).await.map_err(|e| e.to_string())
                    })
                    .await
                    .and_then(|inner| inner);
                    let _ = weak.update(app, |shell, cx| {
                        if !shell.computer_fence_matches(fence) {
                            return;
                        }
                        match res {
                            Ok(updated) => {
                                if let Some(slot) = shell
                                    .computers
                                    .computers
                                    .iter_mut()
                                    .find(|n| n.node_id == updated.node_id)
                                {
                                    *slot = updated;
                                } else {
                                    shell.computers.computers.push(updated);
                                }
                                shell.sync_computers_to_views(cx);
                                shell.enqueue_computer_toast(
                                    "电脑权限与策略已更新",
                                    "Computer authorization policy updated",
                                    1800,
                                );
                                cx.notify();
                            }
                            Err(e) => {
                                let pfx = tr_str(shell.active_lang_id(), "策略更新失败", "Policy update failed");
                                shell.window_runtime.enqueue_toast(format!("{pfx}: {e}"), 2400);
                                cx.notify();
                            }
                        }
                    });
                })
                .detach();
            }
            SettingsComputersEvent::DeleteNode(node_id) => {
                let Some(fence) = self.computer_fence() else {
                    return;
                };
                let client = self.client.clone();
                let nid = node_id.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let c = client.clone();
                    let nid_task = nid.clone();
                    let res = run_on_tokio(&client, async move {
                        c.delete_computer_async(&nid_task).await.map_err(|e| e.to_string())
                    })
                    .await
                    .and_then(|inner| inner);
                    let _ = weak.update(app, |shell, cx| {
                        if !shell.computer_fence_matches(fence) {
                            return;
                        }
                        let is_local = shell
                            .computers
                            .computers
                            .iter()
                            .find(|n| n.node_id == nid)
                            .is_some_and(|n| n.kind == ComputerNodeKind::LocalDevice);
                        match res {
                            Ok(()) => {
                                shell.computers.computers.retain(|n| n.node_id != nid);
                                if let Some(sc_ent) =
                                    shell.settings_router.read(cx).settings_computers.clone()
                                {
                                    sc_ent.update(cx, |sc, sc_cx| {
                                        sc.open_overview();
                                        sc_cx.notify();
                                    });
                                }
                                shell.sync_computers_to_views(cx);
                                if is_local {
                                    shell.enqueue_computer_toast(
                                        "离线设备已移除",
                                        "Device removed",
                                        2000,
                                    );
                                } else {
                                    shell.enqueue_computer_toast(
                                        "云服务器已删除",
                                        "Cloud server deleted",
                                        2000,
                                    );
                                }
                                cx.notify();
                            }
                            Err(e) => {
                                let pfx = if is_local {
                                    tr_str(
                                        shell.active_lang_id(),
                                        "移除设备失败",
                                        "Failed to remove device",
                                    )
                                } else {
                                    tr_str(
                                        shell.active_lang_id(),
                                        "删除云服务器失败",
                                        "Failed to delete cloud server",
                                    )
                                };
                                shell.window_runtime.enqueue_toast(format!("{pfx}: {e}"), 2600);
                                cx.notify();
                            }
                        }
                    });
                })
                .detach();
            }
            SettingsComputersEvent::TestNode(node_id) => {
                let Some(fence) = self.computer_fence() else {
                    return;
                };
                let client = self.client.clone();
                let nid = node_id.clone();
                let is_local = self
                    .computers
                    .computers
                    .iter()
                    .find(|n| n.node_id == nid)
                    .is_some_and(|n| n.kind == ComputerNodeKind::LocalDevice);
                let caller_label = self
                    .session_sync
                    .username
                    .clone()
                    .unwrap_or_else(|| "Desktop".to_string());
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let c = client.clone();
                    let res = run_on_tokio(&client, async move {
                        if is_local {
                            let req = RemoteComputerCommandRequest {
                                node_id: nid,
                                caller_device_id: Some(c.device_id()),
                                caller_bot_id: None,
                                caller_label,
                                operation: RemoteComputerOp::TestConnection,
                            };
                            c.execute_remote_computer_op_async(&req)
                                .await
                                .map_err(|e| e.to_string())
                        } else {
                            c.test_computer_connection_async(&nid)
                                .await
                                .map_err(|e| e.to_string())
                        }
                    })
                    .await
                    .and_then(|inner| inner);
                    let _ = weak.update(app, |shell, cx| {
                        if !shell.computer_fence_matches(fence) {
                            return;
                        }
                        let msg = match res {
                            Ok(r) if r.ok => format!(
                                "{} ({}ms) {}",
                                tr_str(shell.active_lang_id(), "连接正常", "Connection OK"),
                                r.duration_ms,
                                r.stdout.trim()
                            ),
                            Ok(r) => format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "连接测试失败", "Connection failed"),
                                if r.stderr.is_empty() { "error" } else { r.stderr.trim() }
                            ),
                            Err(e) => format!(
                                "{}: {e}",
                                tr_str(shell.active_lang_id(), "连接测试异常", "Connection error")
                            ),
                        };
                        if let Some(sc_ent) =
                            shell.settings_router.read(cx).settings_computers.clone()
                        {
                            sc_ent.update(cx, |sc, sc_cx| {
                                sc.status_message = Some(msg.clone());
                                sc_cx.notify();
                            });
                        }
                        shell.window_runtime.enqueue_toast(msg, 2400);
                        shell.refresh_computers(cx);
                    });
                })
                .detach();
            }
        }
    }

    /// Approves or denies a queued pending remote execution request for this device.
    pub fn decide_pending_computer_approval(
        &mut self,
        command_id: String,
        approved: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(fence) = self.computer_fence() else {
            return;
        };
        self.computers
            .pending_approvals
            .retain(|p| p.command_id != command_id);
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let c = client.clone();
            let res = run_on_tokio(&client, async move {
                let is_p2p = command_id.starts_with("p2p:");
                let decision = if is_p2p {
                    match c.decide_tailnet_approval_async(&command_id, approved).await {
                        Some(Ok(())) => Ok(()),
                        Some(Err(e)) => Err(e),
                        None => Err("Pending P2P approval expired or not found".to_string()),
                    }
                } else {
                    c.approve_computer_command_async(&command_id, approved)
                        .await
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                };
                if !is_p2p && decision.is_ok() && approved {
                    let _ = c.process_ready_local_computer_commands_async().await;
                }
                let list = c.list_computers_async().await.map_err(|e| e.to_string());
                let ts = Some(c.tailnet_status());
                let waiting = c.tailnet_pending_approvals();
                (decision, list, ts, waiting)
            })
            .await
            .unwrap_or_else(|e| (Err(e.clone()), Err(e), None, Vec::new()));

            let _ = weak.update(app, |shell, cx| {
                if !shell.computer_fence_matches(fence) {
                    return;
                }
                match res.0 {
                    Ok(_) => {
                        if let Some(ts) = res.2 {
                            shell.computers.tailnet_status = Some(ts);
                        }
                        if let Ok(list) = res.1 {
                            shell.apply_computer_list_response(list);
                        }
                        shell.merge_pending_approvals(res.3);
                        shell.sync_computers_to_views(cx);
                        if approved {
                            shell.enqueue_computer_toast(
                                "已允许远程指令并立即执行",
                                "Approved remote command",
                                2000,
                            );
                        } else {
                            shell.enqueue_computer_toast(
                                "已拒绝远程执行请求",
                                "Denied remote command",
                                1800,
                            );
                        }
                        cx.notify();
                    }
                    Err(e) => {
                        let pfx = tr_str(shell.active_lang_id(), "审批操作失败", "Approval failed");
                        shell.window_runtime.enqueue_toast(format!("{pfx}: {e}"), 2400);
                        shell.refresh_computers(cx);
                    }
                }
            });
        })
        .detach();
    }
}
