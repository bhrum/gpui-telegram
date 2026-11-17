//! Synchronization and action dispatch between `AppShell`, `SettingsAiProvidersSections`,
//! `GlobalTasksWorkspace`, and the Magpie backend routes (`/api/v1/hermes/magpie/*`).

use std::future::Future;

use gpui::{AsyncApp, ClipboardItem, Context, WeakEntity};
use gpui_telegram_client::net::hermes_client::{
    MagpieGroupsRequest, MagpieProbeRequest, MagpieSaveProviderRequest, MagpieStateDto,
};

use crate::i18n::tr_str;
use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::settings::sections::settings_ai_providers::{
    SubscriptionAccountDto, SubscriptionActivateRequest, SubscriptionActivateResponse,
};
use crate::views::settings::sections::SettingsAiProvidersEvent;
use crate::views::settings::settings_type::SettingsSectionType;
use crate::views::shell::AppShell;

fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

async fn http_fetch_subscriptions(
    base_url: &str,
    token: &str,
) -> Result<Vec<SubscriptionAccountDto>, String> {
    let url = format!("{}/api/v1/hermes/magpie/subscriptions", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| format!("HTTP GET subscriptions failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Server returned status {}", resp.status()));
    }
    resp.json::<Vec<SubscriptionAccountDto>>()
        .await
        .map_err(|e| format!("Failed to parse subscriptions JSON: {e}"))
}

async fn http_activate_subscription(
    base_url: &str,
    token: &str,
    sub_id: &str,
    model: Option<&str>,
) -> Result<SubscriptionActivateResponse, String> {
    let url = format!(
        "{}/api/v1/hermes/magpie/subscriptions/{}/activate",
        base_url.trim_end_matches('/'),
        url_encode(sub_id)
    );
    let client = reqwest::Client::new();
    let payload = SubscriptionActivateRequest {
        model: model.map(str::to_string),
    };
    let resp = client
        .post(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("HTTP POST activate failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Server returned status {}", resp.status()));
    }
    resp.json::<SubscriptionActivateResponse>()
        .await
        .map_err(|e| format!("Failed to parse activate response JSON: {e}"))
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

fn summarize_magpie_for_main(state: &MagpieStateDto, lang: &str) -> (Option<String>, bool) {
    if state.needs_provider_selection {
        return (
            Some(tr_str(lang, "请选择供应商", "Choose Provider").to_string()),
            true,
        );
    }
    let active = if !state.active_provider.is_empty() && !state.active_model.is_empty() {
        format!("{} · {}", state.active_provider, state.active_model)
    } else if !state.active_provider.is_empty() {
        state.active_provider.clone()
    } else if let Some(sub) = state.subscriptions.iter().find(|s| s.active) {
        sub.label.clone()
    } else {
        tr_str(lang, "已配置", "Ready").to_string()
    };
    (Some(active), false)
}

impl AppShell {
    /// Opens Settings directly to `SettingsSectionType::AiProviders` and fetches fresh Magpie state.
    pub fn open_ai_providers_settings(&mut self, cx: &mut Context<Self>) {
        self.navigate_settings_to(SettingsSectionType::AiProviders, cx);
        self.refresh_magpie_state(cx);
    }

    /// Applies fetched `MagpieStateDto` and optional explicit `SubscriptionAccountDto` records to desktop views.
    pub(crate) fn apply_magpie_state_and_subscriptions_to_views(
        &mut self,
        state: MagpieStateDto,
        subs: Option<Vec<SubscriptionAccountDto>>,
        cx: &mut Context<Self>,
    ) {
        let lang = self.active_lang_id().to_string();
        let (summary, needs_sel) = summarize_magpie_for_main(&state, &lang);
        let state_for_ws = state.clone();
        self.settings_router.update(cx, |sr, scx| {
            if let Some(ref ap) = sr.settings_ai_providers {
                ap.update(scx, |view, vcx| {
                    if let Some(s) = subs {
                        view.sync_subscriptions(s);
                    }
                    view.sync_state(state.clone());
                    vcx.notify();
                });
            }
            if let Some(ref sm) = sr.settings_main {
                sm.update(scx, |main, mcx| {
                    main.sync_ai_providers_status(summary.clone(), needs_sel);
                    mcx.notify();
                });
            }
        });
        self.tasks_workspace.update(cx, |ws, wcx| {
            ws.sync_magpie_state(state_for_ws);
            wcx.notify();
        });
        cx.notify();
    }

    /// Applies fetched `MagpieStateDto` to `SettingsAiProvidersSections`, `SettingsMainSections`, and `GlobalTasksWorkspace`.
    pub(crate) fn apply_magpie_state_to_views(
        &mut self,
        state: MagpieStateDto,
        cx: &mut Context<Self>,
    ) {
        self.apply_magpie_state_and_subscriptions_to_views(state, None, cx);
    }

    /// Fetches `/api/v1/hermes/magpie/state` and `/api/v1/hermes/magpie/subscriptions`, updating materialized desktop views.
    pub fn refresh_magpie_state(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let base_url = self.client.http_client.base_url.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.hermes.magpie_state(&token).await;
            let subs_res = http_fetch_subscriptions(&base_url, &token).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                match res {
                    Ok(state) => {
                        let subs = subs_res.ok();
                        shell.apply_magpie_state_and_subscriptions_to_views(state, subs, cx);
                    }
                    Err(e) => {
                        let msg = format!("Magpie sync error: {e}");
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.set_status(None, Some(msg.clone()));
                                    vcx.notify();
                                });
                            }
                        });
                    }
                }
            });
        })
        .detach();
    }

    fn spawn_magpie_state_mutation<F>(
        &mut self,
        zh_ok: &'static str,
        en_ok: &'static str,
        close_editor_on_ok: bool,
        fut_fn: impl FnOnce(std::sync::Arc<gpui_telegram_client::TelegramClient>, String) -> F + 'static,
        cx: &mut Context<Self>,
    ) where
        F: Future<Output = Result<MagpieStateDto, String>> + 'static,
    {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = fut_fn(client, token).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                let lang = shell.active_lang_id().to_string();
                match res {
                    Ok(state) => {
                        let status_msg = tr_str(&lang, zh_ok, en_ok).to_string();
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    if close_editor_on_ok {
                                        view.editor_open = false;
                                        view.active_field = crate::views::settings::sections::AiProviderField::None;
                                    }
                                    view.set_status(Some(status_msg.clone()), None);
                                    vcx.notify();
                                });
                            }
                        });
                        shell.apply_magpie_state_to_views(state, cx);
                        shell.window_runtime.enqueue_toast(status_msg, 2400);
                        shell.refresh_tasks_and_mcp(cx);
                    }
                    Err(err) => {
                        let msg = format!("{}: {err}", tr_str(&lang, "操作失败", "Action failed"));
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.set_status(None, Some(msg.clone()));
                                    vcx.notify();
                                });
                            }
                        });
                        shell.window_runtime.enqueue_toast(msg, 2800);
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    pub(crate) fn run_magpie_subscription_scan(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let base_url = self.client.http_client.base_url.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let scan_res = client.hermes.magpie_scan_subscriptions(&token).await;
            let subs_res = http_fetch_subscriptions(&base_url, &token).await;
            let state_res = client.hermes.magpie_state(&token).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                let lang = shell.active_lang_id().to_string();
                let toast = match &scan_res {
                    Ok(resp) => {
                        let active = resp.subscriptions.iter().filter(|s| s.active).count();
                        if active > 0 {
                            if lang.starts_with("zh") {
                                format!("扫描完成：检测到 {active} 个可用本地 AI 订阅凭据")
                            } else {
                                format!("Scan complete: detected {active} active AI credential(s)")
                            }
                        } else {
                            tr_str(
                                &lang,
                                "扫描完成：未发现已登录的本地凭据，请在下方选择供应商预设",
                                "Scan complete: no active local credentials found — select a provider preset below",
                            )
                            .to_string()
                        }
                    }
                    Err(e) => format!("Subscription scan failed: {e}"),
                };
                if let Ok(state) = state_res {
                    let subs = subs_res.ok();
                    shell.apply_magpie_state_and_subscriptions_to_views(state, subs, cx);
                }
                shell.settings_router.update(cx, |sr, scx| {
                    if let Some(ref ap) = sr.settings_ai_providers {
                        ap.update(scx, |view, vcx| {
                            view.set_status(Some(toast.clone()), None);
                            vcx.notify();
                        });
                    }
                });
                shell.window_runtime.enqueue_toast(toast, 2600);
                cx.notify();
            });
        })
        .detach();
    }

    /// Activates a discovered local AI subscription as the primary provider.
    pub(crate) fn activate_subscription(
        &mut self,
        subscription_id: String,
        model: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let base_url = self.client.http_client.base_url.clone();
        let client = self.client.clone();
        let sub_id = subscription_id.clone();
        let req_model = model.clone();

        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let act_res =
                http_activate_subscription(&base_url, &token, &sub_id, req_model.as_deref()).await;
            let subs_res = http_fetch_subscriptions(&base_url, &token).await;
            let state_res = client.hermes.magpie_state(&token).await;

            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                let lang = shell.active_lang_id().to_string();
                match act_res {
                    Ok(resp) => {
                        let toast = if lang.starts_with("zh") {
                            format!(
                                "已成功激活「{}」为主要模型供应商 (模型: {})",
                                resp.active_provider_id, resp.active_model
                            )
                        } else {
                            format!(
                                "Activated \"{}\" as primary provider ({})",
                                resp.active_provider_id, resp.active_model
                            )
                        };
                        if let Ok(state) = state_res {
                            let subs = subs_res.ok();
                            shell.apply_magpie_state_and_subscriptions_to_views(state, subs, cx);
                        }
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.set_status(Some(toast.clone()), None);
                                    vcx.notify();
                                });
                            }
                        });
                        shell.window_runtime.enqueue_toast(toast, 2800);
                        shell.refresh_tasks_and_mcp(cx);
                    }
                    Err(err) => {
                        let msg = format!(
                            "{}: {err}",
                            tr_str(&lang, "激活智能体失败", "Failed to activate AI agent")
                        );
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.set_status(None, Some(msg.clone()));
                                    vcx.notify();
                                });
                            }
                        });
                        shell.window_runtime.enqueue_toast(msg, 3000);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub(crate) fn run_magpie_probe(
        &mut self,
        provider_id: String,
        model: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let req = MagpieProbeRequest {
                model,
                prompt: Some("Reply with OK".to_string()),
            };
            let res = client
                .hermes
                .magpie_probe_provider(&token, &provider_id, Some(&req))
                .await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                let lang = shell.active_lang_id().to_string();
                match res {
                    Ok(probe) => {
                        let toast = if probe.ok {
                            format!(
                                "Probe OK ({} · {}ms)",
                                probe.provider_id, probe.latency_ms
                            )
                        } else {
                            format!(
                                "{}: {}",
                                tr_str(&lang, "连通性测试失败", "Probe failed"),
                                probe.error.as_deref().unwrap_or("error")
                            )
                        };
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.loading = false;
                                    view.probe_result = Some(probe.clone());
                                    vcx.notify();
                                });
                            }
                        });
                        shell.window_runtime.enqueue_toast(toast, 2800);
                    }
                    Err(e) => {
                        let msg = format!("Probe error: {e}");
                        shell.settings_router.update(cx, |sr, scx| {
                            if let Some(ref ap) = sr.settings_ai_providers {
                                ap.update(scx, |view, vcx| {
                                    view.set_status(None, Some(msg.clone()));
                                    vcx.notify();
                                });
                            }
                        });
                        shell.window_runtime.enqueue_toast(msg, 2800);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Handles events emitted by `SettingsAiProvidersSections` via `SettingsRouter`.
    pub fn handle_ai_providers_settings_event(
        &mut self,
        ev: &SettingsAiProvidersEvent,
        cx: &mut Context<Self>,
    ) {
        match ev {
            SettingsAiProvidersEvent::RefreshState => self.refresh_magpie_state(cx),
            SettingsAiProvidersEvent::ScanSubscriptions => self.run_magpie_subscription_scan(cx),
            SettingsAiProvidersEvent::ActivateSubscription {
                subscription_id,
                model,
            } => {
                self.activate_subscription(subscription_id.clone(), model.clone(), cx);
            }
            SettingsAiProvidersEvent::CopyText(txt) => {
                cx.write_to_clipboard(ClipboardItem::new_string(txt.clone()));
                self.window_runtime.enqueue_toast(
                    format!("{}: {txt}", tr_str(self.active_lang_id(), "已复制", "Copied")),
                    2200,
                );
            }
            SettingsAiProvidersEvent::SaveProvider(req) => {
                let req = req.clone();
                self.spawn_magpie_state_mutation(
                    "AI 供应商已保存并激活",
                    "AI provider saved & activated",
                    true,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_save_provider(&tok, &req)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::DeleteProvider(pid) => {
                let pid = pid.clone();
                self.spawn_magpie_state_mutation(
                    "已移除供应商",
                    "Provider removed",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_delete_provider(&tok, &pid)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::ProbeProvider { provider_id, model } => {
                self.run_magpie_probe(provider_id.clone(), model.clone(), cx);
            }
            SettingsAiProvidersEvent::ToggleAllowLocalFallback(allow) => {
                let target_pid = self
                    .settings_router
                    .read(cx)
                    .settings_ai_providers
                    .as_ref()
                    .and_then(|ap| ap.read(cx).state.as_ref().map(|s| s.active_provider.clone()))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "local_hermes".to_string());
                let req = MagpieSaveProviderRequest {
                    provider_id: Some(target_pid),
                    activate: false,
                    allow_local_fallback: Some(*allow),
                    ..Default::default()
                };
                self.spawn_magpie_state_mutation(
                    "本地回退策略已更新",
                    "Local fallback setting updated",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_save_provider(&tok, &req)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::SetRoutingStrategy(strategy) => {
                let req = MagpieGroupsRequest {
                    group: None,
                    routing_strategy: Some(strategy.clone()),
                };
                self.spawn_magpie_state_mutation(
                    "全局路由策略已更新",
                    "Routing strategy updated",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_upsert_group(&tok, &req)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::UpsertRoutingGroup(group) => {
                let req = MagpieGroupsRequest {
                    group: Some(group.clone()),
                    routing_strategy: None,
                };
                self.spawn_magpie_state_mutation(
                    "模型路由组已保存",
                    "Routing group saved",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_upsert_group(&tok, &req)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::DeleteRoutingGroup(gid) => {
                let gid = gid.clone();
                self.spawn_magpie_state_mutation(
                    "已删除模型路由组",
                    "Routing group deleted",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_delete_group(&tok, &gid)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::UpdateMasking(cfg) => {
                let cfg = cfg.clone();
                self.spawn_magpie_state_mutation(
                    "隐私脱敏规则已保存",
                    "Privacy masking updated",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_update_masking(&tok, &cfg)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
            SettingsAiProvidersEvent::PreviewMasking(req) => {
                let Some(fence) = session_fence(self) else {
                    return;
                };
                let token = fence.token.clone();
                let client = self.client.clone();
                let req = req.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client.hermes.magpie_preview_masking(&token, &req).await;
                    let _ = weak.update(app, |shell, cx| {
                        if !fence_matches(&fence, shell) {
                            return;
                        }
                        if let Ok(preview) = res {
                            shell.settings_router.update(cx, |sr, scx| {
                                if let Some(ref ap) = sr.settings_ai_providers {
                                    ap.update(scx, |view, vcx| {
                                        view.masking_preview_result = Some(preview);
                                        vcx.notify();
                                    });
                                }
                            });
                        }
                    });
                })
                .detach();
            }
            SettingsAiProvidersEvent::ImportConfig(req) => {
                let req = req.clone();
                self.spawn_magpie_state_mutation(
                    "配置导入成功",
                    "Magpie config imported",
                    false,
                    move |c, tok| async move {
                        c.hermes
                            .magpie_import(&tok, &req)
                            .await
                            .map(|r| r.state)
                            .map_err(|e| e.to_string())
                    },
                    cx,
                );
            }
        }
    }
}
