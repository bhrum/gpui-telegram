use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::net::hermes_client::HermesSaveProviderRequest;

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::views::global_tasks_workspace::{
    GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

use super::{action_btn, studio_input_box, studio_section_card};

// ============================================================================
// Tab 6: Sessions & Providers (FTS5 Search, Branch/Compress/Export, API Modes, Usage)
// ============================================================================

pub fn render_sessions_providers_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();
    let api_modes = [
        "chat_completions",
        "codex_responses",
        "anthropic_messages",
        "gemini_generate_content",
        "local_hermes",
    ];
    let needs_provider_sel = ws
        .magpie_state
        .as_ref()
        .map_or(ws.status_summary.needs_provider_selection, |m| {
            m.needs_provider_selection
        });
    let active_subs = ws
        .magpie_state
        .as_ref()
        .map_or(0, |m| m.subscriptions.iter().filter(|s| s.active).count());
    let routing_strat = ws
        .magpie_state
        .as_ref()
        .map(|m| m.routing_strategy.clone())
        .unwrap_or_else(|| "smart".to_string());
    let masking_on = ws.magpie_state.as_ref().is_some_and(|m| m.masking.enabled);
    let active_pid_for_probe = ws
        .magpie_state
        .as_ref()
        .map(|m| m.active_provider.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| ws.status_summary.active_provider.clone());

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // 1. Token / Cost Usage & Insights Summary
        .child(
            studio_section_card(
                tr_str(&lang, "Token 与成本用量洞察", "Token, Cost & Session Insights"),
                IconName::WalletStars,
                pal,
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(10.0))
                    .children([
                        ("Turns", format!("{}", ws.usage_summary.total_turns)),
                        ("Prompt Tokens", format!("{}", ws.usage_summary.prompt_tokens)),
                        (
                            "Completion Tokens",
                            format!("{}", ws.usage_summary.completion_tokens),
                        ),
                        ("Total Tokens", format!("{}", ws.usage_summary.total_tokens)),
                        ("Tool Calls", format!("{}", ws.usage_summary.tool_calls)),
                        (
                            "Est. Cost",
                            format!("${:.4}", ws.usage_summary.estimated_cost_usd),
                        ),
                    ]
                    .into_iter()
                    .map(|(k, v)| {
                        div()
                            .w(px(145.0))
                            .p(px(9.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.window_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(k),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.accent))
                                    .child(v),
                            )
                    })),
            ),
        )
        // 2. API Mode, Fallback Chain & Magpie Providers
        .child(
            studio_section_card(
                tr_str(&lang, "API 协议模式与模型供应商 (Magpie)", "API Modes, Magpie Providers & Fallback Chain"),
                IconName::SettingsGear,
                pal,
            )
            .child(
                div()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(if needs_provider_sel { pal.danger } else { pal.accent }))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(if needs_provider_sel {
                                        pal.danger
                                    } else {
                                        pal.text_primary
                                    }))
                                    .child(if needs_provider_sel {
                                        tr_str(
                                            &lang,
                                            "未检测到激活的 AI 订阅或供应商 — 请选择供应商",
                                            "No AI subscription or provider configured — Choose a Provider",
                                        )
                                        .to_string()
                                    } else {
                                        format!(
                                            "{}: {} / {} · {} subs · route={} · mask={}",
                                            tr_str(&lang, "Magpie 已就绪", "Magpie Ready"),
                                            ws.status_summary.active_provider,
                                            ws.status_summary.active_model,
                                            active_subs,
                                            routing_strat,
                                            if masking_on { "on" } else { "off" }
                                        )
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .child(action_btn(
                                tr_str(&lang, "选择 AI 供应商 (Magpie)", "Configure AI Providers (Magpie)"),
                                pal.accent,
                                pal.active_button_fg,
                                |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::OpenAiProvidersSettings);
                                },
                                cx,
                            ))
                            .child(action_btn(
                                tr_str(&lang, "扫描本地订阅", "Scan Subscriptions"),
                                pal.input_bg,
                                pal.text_primary,
                                |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ScanMagpieSubscriptions);
                                },
                                cx,
                            ))
                            .when(!active_pid_for_probe.is_empty(), |row| {
                                let pid = active_pid_for_probe.clone();
                                row.child(action_btn(
                                    tr_str(&lang, "探测当前供应商", "Probe Active"),
                                    pal.input_bg,
                                    pal.accent,
                                    move |this, _| {
                                        this.pending_action = Some(
                                            TasksWorkspaceAction::ProbeMagpieProvider(pid.clone()),
                                        );
                                    },
                                    cx,
                                ))
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.text_secondary))
                            .child("API Mode:"),
                    )
                    .children({
                        let mut mode_btns: Vec<AnyElement> = Vec::new();
                        for mode in api_modes {
                            let sel = ws.active_api_mode == mode;
                            let m_str = mode.to_string();
                            mode_btns.push(
                                action_btn(
                                    mode,
                                    if sel { pal.accent } else { pal.input_bg },
                                    if sel {
                                        pal.active_button_fg
                                    } else {
                                        pal.text_primary
                                    },
                                    move |this, _| {
                                        this.active_api_mode = m_str.clone();
                                        this.pending_action =
                                            Some(TasksWorkspaceAction::SetApiMode(m_str.clone()));
                                    },
                                    cx,
                                )
                                .into_any_element(),
                            );
                        }
                        mode_btns
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div().flex_1().child(studio_input_box(
                            &ws.fallback_chain_draft,
                            "Fallback chain: local_hermes -> openai -> anthropic",
                            StudioInputTarget::FallbackChain,
                            ws.active_input,
                            pal,
                            false,
                            cx,
                        )),
                    )
                    .child(action_btn(
                        "Save Chain",
                        pal.input_bg,
                        pal.accent,
                        |this, _| {
                            let pid = this
                                .providers
                                .iter()
                                .find(|p| p.is_active)
                                .map(|p| p.provider_id.clone())
                                .unwrap_or_else(|| "local_hermes".to_string());
                            this.pending_action = Some(TasksWorkspaceAction::SaveProviderConfig(
                                HermesSaveProviderRequest {
                                    provider_id: pid,
                                    name: None,
                                    api_mode: Some(this.active_api_mode.clone()),
                                    base_url: None,
                                    api_key: None,
                                    default_model: None,
                                    enabled: Some(true),
                                    set_active: Some(true),
                                },
                            ));
                        },
                        cx,
                    )),
            )
            .children(ws.providers.iter().map(|p| {
                let pid = p.provider_id.clone();
                let mode = ws.active_api_mode.clone();
                div()
                    .p(px(9.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(if p.is_active {
                        pal.accent
                    } else {
                        pal.border_fg
                    }))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!(
                                        "{} ({}) · {}",
                                        p.name, p.provider_id, p.default_model
                                    )),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "Mode: {} · API key: {}",
                                        p.api_mode,
                                        if p.has_api_key { "configured" } else { "local" }
                                    )),
                            ),
                    )
                    .child(action_btn(
                        if p.is_active { "Active" } else { "Activate" },
                        if p.is_active { pal.success } else { pal.input_bg },
                        if p.is_active {
                            pal.active_button_fg
                        } else {
                            pal.text_primary
                        },
                        move |this, _| {
                            this.pending_action = Some(TasksWorkspaceAction::SaveProviderConfig(
                                HermesSaveProviderRequest {
                                    provider_id: pid.clone(),
                                    name: None,
                                    api_mode: Some(mode.clone()),
                                    base_url: None,
                                    api_key: None,
                                    default_model: None,
                                    enabled: Some(true),
                                    set_active: Some(true),
                                },
                            ));
                        },
                        cx,
                    ))
            })),
        )
        // 3. FTS5 Session Search + Branch / Compress / Rollback / Export (ShareGPT / OpenAI / Anthropic)
        .child(
            studio_section_card(
                tr_str(&lang, "FTS5 会话搜索与分支导出", "FTS5 Session Search, Branch & Export"),
                IconName::Search,
                pal,
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div().flex_1().child(studio_input_box(
                            &ws.session_search_draft,
                            tr_str(&lang, "全文搜索历史会话...", "FTS5 search past sessions..."),
                            StudioInputTarget::SessionSearch,
                            ws.active_input,
                            pal,
                            false,
                            cx,
                        )),
                    )
                    .child(action_btn(
                        tr_str(&lang, "搜索", "Search"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            this.pending_action = Some(TasksWorkspaceAction::SearchSessions(
                                this.session_search_draft.trim().to_string(),
                            ));
                        },
                        cx,
                    )),
            )
            .when(!ws.session_search_hits.is_empty(), |el| {
                el.child(
                    div()
                        .p(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.window_bg))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child(format!("Search Hits ({})", ws.session_search_hits.len())),
                        )
                        .children(ws.session_search_hits.iter().take(6).map(|hit| {
                            let txt = hit
                                .get("snippet")
                                .or_else(|| hit.get("content"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("Match");
                            div()
                                .text_xs()
                                .text_color(rgb(pal.text_secondary))
                                .child(txt.to_string())
                        })),
                )
            })
            .children(ws.sessions.iter().map(|s| {
                let sid_b = s.session_id.clone();
                let sid_c = s.session_id.clone();
                let sid_r = s.session_id.clone();
                let sid_sg = s.session_id.clone();
                let sid_oa = s.session_id.clone();
                let sid_an = s.session_id.clone();
                div()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(
                                        s.title
                                            .clone()
                                            .unwrap_or_else(|| s.session_id.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "{} msgs · {} tok · {}",
                                        s.message_count, s.token_count, s.model
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .child(action_btn(
                                "Branch",
                                pal.input_bg,
                                pal.accent,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::BranchSession(sid_b.clone()));
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Compress",
                                pal.input_bg,
                                pal.text_primary,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::CompressSession(sid_c.clone()));
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Rollback",
                                pal.input_bg,
                                pal.danger,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::RollbackSession(sid_r.clone()));
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "ShareGPT",
                                pal.input_bg,
                                pal.success,
                                move |this, _| {
                                    this.pending_action = Some(TasksWorkspaceAction::ExportSession {
                                        session_id: sid_sg.clone(),
                                        format: "sharegpt".to_string(),
                                    });
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "OpenAI",
                                pal.input_bg,
                                pal.success,
                                move |this, _| {
                                    this.pending_action = Some(TasksWorkspaceAction::ExportSession {
                                        session_id: sid_oa.clone(),
                                        format: "openai".to_string(),
                                    });
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Anthropic",
                                pal.input_bg,
                                pal.success,
                                move |this, _| {
                                    this.pending_action = Some(TasksWorkspaceAction::ExportSession {
                                        session_id: sid_an.clone(),
                                        format: "anthropic".to_string(),
                                    });
                                },
                                cx,
                            )),
                    )
            }))
            .when_some(ws.session_export_preview.clone(), |el, exp| {
                el.child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.window_bg))
                        .border_1()
                        .border_color(rgb(pal.accent))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child(format!(
                                    "Export Preview ({} · {})",
                                    exp.format, exp.session_id
                                )),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(pal.text_secondary))
                                .child(exp.content.chars().take(600).collect::<String>()),
                        ),
                )
            }),
        )
        .into_any_element()
}
