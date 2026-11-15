use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::net::hermes_client::HermesSaveCronRequest;

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::views::global_tasks_workspace::{
    GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

use super::{action_btn, studio_input_box, studio_section_card};

// ============================================================================
// Tab 5: Cron & Heartbeat
// ============================================================================

fn describe_cron_preview(expr: &str) -> &'static str {
    let trimmed = expr.trim();
    match trimmed {
        "*/5 * * * *" => "Every 5 minutes (24/7)",
        "*/15 * * * *" => "Every 15 minutes",
        "0 * * * *" => "Hourly at :00",
        "0 9 * * *" => "Daily at 09:00 UTC",
        "0 9 * * 1-5" => "Weekdays Mon-Fri at 09:00",
        _ if trimmed.starts_with("every ") => "Fixed interval schedule",
        _ if trimmed.split_whitespace().count() == 5 => "Valid 5-field cron expression",
        _ => "Enter 5-field cron (min hr dom mon dow) or 'every Nm'",
    }
}

pub fn render_cron_heartbeat_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();
    let cron_presets = [
        ("*/5 * * * *", "5m"),
        ("*/15 * * * *", "15m"),
        ("0 * * * *", "Hourly"),
        ("0 9 * * 1-5", "Weekdays 9am"),
    ];

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // 1. Heartbeat Status & Interval
        .child(
            studio_section_card(
                tr_str(&lang, "自主心跳守护 (Heartbeat)", "Autonomous Heartbeat Monitor"),
                IconName::ClockSending,
                pal,
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(action_btn(
                                if ws.heartbeat.enabled {
                                    "Heartbeat: ACTIVE"
                                } else {
                                    "Heartbeat: PAUSED"
                                },
                                if ws.heartbeat.enabled {
                                    pal.success
                                } else {
                                    pal.input_bg
                                },
                                if ws.heartbeat.enabled {
                                    pal.active_button_fg
                                } else {
                                    pal.text_secondary
                                },
                                |this, _| {
                                    let mut cfg = this.heartbeat.clone();
                                    cfg.enabled = !cfg.enabled;
                                    this.heartbeat = cfg.clone();
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::SaveHeartbeat(cfg));
                                },
                                cx,
                            ))
                            .children({
                                let mut interval_btns: Vec<AnyElement> = Vec::new();
                                for secs in [60_u64, 300, 900, 1800] {
                                    let sel = ws.heartbeat.interval_secs == secs;
                                    interval_btns.push(
                                        action_btn(
                                            format!("{}s", secs),
                                            if sel { pal.accent } else { pal.input_bg },
                                            if sel {
                                                pal.active_button_fg
                                            } else {
                                                pal.text_primary
                                            },
                                            move |this, _| {
                                                let mut cfg = this.heartbeat.clone();
                                                cfg.interval_secs = secs;
                                                this.heartbeat = cfg.clone();
                                                this.pending_action =
                                                    Some(TasksWorkspaceAction::SaveHeartbeat(cfg));
                                            },
                                            cx,
                                        )
                                        .into_any_element(),
                                    );
                                }
                                interval_btns
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(pal.text_secondary))
                            .child(format!("Prompt: {}", ws.heartbeat.prompt)),
                    ),
            ),
        )
        // 2. Cron Expression Builder & Scheduled Jobs
        .child(
            studio_section_card(
                tr_str(&lang, "定时任务 (Cron Jobs)", "Scheduled Cron Routines"),
                IconName::Poll,
                pal,
            )
            .child(
                div()
                    .p(px(10.0))
                    .rounded(px(9.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .child(
                                div().w(px(170.0)).child(studio_input_box(
                                    &ws.cron_name_draft,
                                    "Job name (e.g. Daily Digest)",
                                    StudioInputTarget::CronName,
                                    ws.active_input,
                                    pal,
                                    false,
                                    cx,
                                )),
                            )
                            .child(
                                div().w(px(160.0)).child(studio_input_box(
                                    &ws.cron_schedule_draft,
                                    "*/15 * * * *",
                                    StudioInputTarget::CronSchedule,
                                    ws.active_input,
                                    pal,
                                    false,
                                    cx,
                                )),
                            )
                            .child(
                                div().w(px(120.0)).child(studio_input_box(
                                    &ws.cron_chat_draft,
                                    "Target chat ID",
                                    StudioInputTarget::CronTargetChat,
                                    ws.active_input,
                                    pal,
                                    false,
                                    cx,
                                )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .children({
                                let mut preset_btns: Vec<AnyElement> = Vec::new();
                                for (expr, label) in cron_presets {
                                    let e_str = expr.to_string();
                                    preset_btns.push(
                                        action_btn(
                                            label,
                                            pal.input_bg,
                                            pal.accent,
                                            move |this, _| {
                                                this.cron_schedule_draft = e_str.clone();
                                            },
                                            cx,
                                        )
                                        .into_any_element(),
                                    );
                                }
                                preset_btns
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.accent))
                                    .child(describe_cron_preview(&ws.cron_schedule_draft)),
                            ),
                    )
                    .child(studio_input_box(
                        &ws.cron_prompt_draft,
                        "Agent prompt executed on each trigger...",
                        StudioInputTarget::CronPrompt,
                        ws.active_input,
                        pal,
                        true,
                        cx,
                    ))
                    .child(
                        div().flex().justify_end().child(action_btn(
                            tr_str(&lang, "保存定时任务", "Create Cron Job"),
                            pal.accent,
                            pal.active_button_fg,
                            |this, _| {
                                let name = this.cron_name_draft.trim().to_string();
                                let sched = this.cron_schedule_draft.trim().to_string();
                                let prompt = this.cron_prompt_draft.trim().to_string();
                                if !name.is_empty() && !sched.is_empty() && !prompt.is_empty() {
                                    let chat_id = this.cron_chat_draft.trim().parse::<i64>().ok();
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::SaveCronJob(
                                            HermesSaveCronRequest {
                                                job_id: None,
                                                name,
                                                schedule: sched,
                                                prompt,
                                                chat_id,
                                                bot_id: None,
                                                enabled: Some(true),
                                            },
                                        ));
                                    this.cron_name_draft.clear();
                                    this.cron_prompt_draft.clear();
                                }
                            },
                            cx,
                        )),
                    ),
            )
            .when(ws.cron_jobs.is_empty(), |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(pal.text_secondary))
                        .child("No cron jobs scheduled yet."),
                )
            })
            .children(ws.cron_jobs.iter().map(|job| {
                let jid_run = job.job_id.clone();
                let jid_tog = job.job_id.clone();
                let jid_del = job.job_id.clone();
                let next_en = !job.enabled;
                div()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(3.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!("{} · `{}`", job.name, job.schedule)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "Target chat: {:?} · Last: {}",
                                        job.chat_id,
                                        job.last_status.as_deref().unwrap_or("never")
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .child(action_btn(
                                "Run Now",
                                pal.accent,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::RunCronNow(jid_run.clone()));
                                },
                                cx,
                            ))
                            .child(action_btn(
                                if job.enabled { "Enabled" } else { "Paused" },
                                if job.enabled { pal.success } else { pal.input_bg },
                                if job.enabled {
                                    pal.active_button_fg
                                } else {
                                    pal.text_secondary
                                },
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ToggleCronJob {
                                            job_id: jid_tog.clone(),
                                            enabled: next_en,
                                        });
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Delete",
                                pal.input_bg,
                                pal.danger,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::DeleteCronJob(jid_del.clone()));
                                },
                                cx,
                            )),
                    )
            }))
            .when(!ws.cron_history.is_empty(), |el| {
                el.child(
                    div()
                        .pt(px(6.0))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_secondary))
                                .child("Recent Execution History"),
                        )
                        .children(ws.cron_history.iter().take(6).map(|h| {
                            div()
                                .text_xs()
                                .text_color(rgb(pal.text_secondary))
                                .child(format!("[{}] {} — {}", h.status, h.job_name, h.summary))
                        })),
                )
            }),
        )
        .into_any_element()
}
