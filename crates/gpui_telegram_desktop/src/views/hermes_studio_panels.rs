use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::net::hermes_client::{
    HermesHeartbeatConfigDto, HermesSaveCronRequest, HermesSaveProviderRequest,
};

use crate::components::tool_permission_dock::ToolApprovalDecision;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::global_tasks_workspace::{
    localize_task_status, GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

fn studio_input_box(
    value: &str,
    placeholder: &str,
    target: StudioInputTarget,
    active: StudioInputTarget,
    pal: TdesktopThemePalette,
    multiline: bool,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> Div {
    let is_active = target == active;
    div()
        .px(px(10.0))
        .py(px(7.0))
        .rounded(px(8.0))
        .bg(rgb(pal.input_bg))
        .border_1()
        .border_color(rgb(if is_active { pal.accent } else { pal.border_fg }))
        .cursor_text()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, window, cx| {
                this.active_input = target;
                if let Some(ref f) = this.focus_handle {
                    f.focus(window);
                }
                cx.notify();
            }),
        )
        .when(multiline, |el| el.min_h(px(120.0)).items_start())
        .text_xs()
        .text_color(rgb(if value.is_empty() {
            pal.text_secondary
        } else {
            pal.text_primary
        }))
        .child(if value.is_empty() {
            placeholder.to_string()
        } else if is_active {
            format!("{}|", value)
        } else {
            value.to_string()
        })
}

fn studio_section_card(
    title: impl Into<String>,
    icon: IconName,
    pal: TdesktopThemePalette,
) -> Div {
    div()
        .p(px(14.0))
        .rounded(px(12.0))
        .bg(rgb(pal.card_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(7.0))
                .child(tg_icon(icon, 15.0, pal.accent))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(title.into()),
                ),
        )
}

fn action_btn(
    label: impl Into<String>,
    bg_color: u32,
    fg_color: u32,
    on_click: impl Fn(&mut GlobalTasksWorkspace, &mut Context<GlobalTasksWorkspace>) + 'static,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> impl IntoElement {
    div()
        .px(px(10.0))
        .py(px(5.0))
        .rounded(px(7.0))
        .bg(rgb(bg_color))
        .cursor_pointer()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(fg_color))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                on_click(this, cx);
                cx.notify();
            }),
        )
        .child(label.into())
}

// ============================================================================
// Tab 1: Tasks / Kanban & Approvals + Subagent Lineage Tree
// ============================================================================

pub fn render_tasks_kanban_approvals_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();

    let active_board = ws
        .kanban_boards
        .iter()
        .find(|b| b.board_id == ws.active_board_id)
        .or_else(|| ws.kanban_boards.first())
        .cloned();
    let columns: Vec<String> = active_board
        .as_ref()
        .filter(|b| !b.columns.is_empty())
        .map(|b| b.columns.clone())
        .unwrap_or_else(|| {
            vec![
                "Backlog".to_string(),
                "In Progress".to_string(),
                "Review".to_string(),
                "Done".to_string(),
            ]
        });

    let pending_hermes: Vec<_> = ws
        .hermes_approvals
        .iter()
        .filter(|a| a.status.eq_ignore_ascii_case("pending") || a.status.is_empty())
        .cloned()
        .collect();

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // 1. Kanban Board (14-tool Kanban system)
        .child(
            studio_section_card(
                tr_str(&lang, "Hermes 任务看板 (Kanban)", "Hermes Kanban Board"),
                IconName::Poll,
                pal,
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .children(ws.kanban_boards.iter().map(|b| {
                        let is_sel = active_board
                            .as_ref()
                            .map(|ab| ab.board_id == b.board_id)
                            .unwrap_or(false);
                        let bid = b.board_id.clone();
                        div()
                            .px(px(9.0))
                            .py(px(4.0))
                            .rounded(px(7.0))
                            .bg(rgb(if is_sel { pal.accent } else { pal.input_bg }))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(if is_sel {
                                pal.active_button_fg
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.active_board_id = bid.clone();
                                    cx.notify();
                                }),
                            )
                            .child(format!("{} ({})", b.name, b.cards.len()))
                    }))
                    .child(
                        div()
                            .w(px(170.0))
                            .child(studio_input_box(
                                &ws.kanban_board_draft,
                                tr_str(&lang, "新建看板名称...", "New board name..."),
                                StudioInputTarget::KanbanBoardName,
                                ws.active_input,
                                pal,
                                false,
                                cx,
                            )),
                    )
                    .child(action_btn(
                        tr_str(&lang, "+ 看板", "+ Board"),
                        pal.input_bg,
                        pal.accent,
                        |this, _| {
                            let name = this.kanban_board_draft.trim().to_string();
                            if !name.is_empty() {
                                this.pending_action =
                                    Some(TasksWorkspaceAction::CreateKanbanBoard(name));
                                this.kanban_board_draft.clear();
                            }
                        },
                        cx,
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(200.0))
                            .child(studio_input_box(
                                &ws.kanban_card_draft,
                                tr_str(
                                    &lang,
                                    "输入任务卡片标题并回车创建...",
                                    "New Kanban card title...",
                                ),
                                StudioInputTarget::KanbanCardTitle,
                                ws.active_input,
                                pal,
                                false,
                                cx,
                            )),
                    )
                    .children(columns.iter().map(|col| {
                        let c_name = col.clone();
                        let sel = ws.kanban_card_column == *col;
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(if sel { pal.accent } else { pal.input_bg }))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(if sel {
                                pal.active_button_fg
                            } else {
                                pal.text_secondary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.kanban_card_column = c_name.clone();
                                    cx.notify();
                                }),
                            )
                            .child(col.clone())
                    }))
                    .child(action_btn(
                        tr_str(&lang, "+ 添加卡片", "+ Add Card"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            let title = this.kanban_card_draft.trim().to_string();
                            if !title.is_empty() {
                                this.pending_action =
                                    Some(TasksWorkspaceAction::CreateKanbanCard {
                                        board_id: this.active_board_id.clone(),
                                        title,
                                        column: this.kanban_card_column.clone(),
                                        priority: "medium".to_string(),
                                    });
                                this.kanban_card_draft.clear();
                            }
                        },
                        cx,
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(10.0))
                    .children(columns.iter().enumerate().map(|(col_idx, col_name)| {
                        let cards: Vec<_> = active_board
                            .as_ref()
                            .map(|b| {
                                b.cards
                                    .iter()
                                    .filter(|c| c.column.eq_ignore_ascii_case(col_name))
                                    .cloned()
                                    .collect()
                            })
                            .unwrap_or_default();
                        let prev_col = if col_idx > 0 {
                            Some(columns[col_idx - 1].clone())
                        } else {
                            None
                        };
                        let next_col = columns.get(col_idx + 1).cloned();

                        div()
                            .flex_1()
                            .min_w(px(190.0))
                            .p(px(10.0))
                            .rounded(px(10.0))
                            .bg(rgb(pal.window_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
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
                                            .child(col_name.clone()),
                                    )
                                    .child(
                                        div()
                                            .px(px(6.0))
                                            .py(px(1.0))
                                            .rounded(px(999.0))
                                            .bg(rgb(pal.input_bg))
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .child(format!("{}", cards.len())),
                                    ),
                            )
                            .when(cards.is_empty(), |el| {
                                el.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(&lang, "暂无卡片", "Empty column")),
                                )
                            })
                            .children(cards.into_iter().map(|card| {
                                let cid_prev = card.card_id.clone();
                                let cid_next = card.card_id.clone();
                                let cid_del = card.card_id.clone();
                                let prev_target = prev_col.clone();
                                let next_target = next_col.clone();
                                let prio_col = match card.priority.as_str() {
                                    "high" | "urgent" | "critical" => pal.danger,
                                    "low" => pal.text_secondary,
                                    _ => pal.accent,
                                };
                                div()
                                    .p(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.card_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .flex()
                                    .flex_col()
                                    .gap(px(5.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(card.title.clone()),
                                    )
                                    .when(!card.description.trim().is_empty(), |el| {
                                        el.child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(pal.text_secondary))
                                                .child(card.description.clone()),
                                        )
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .px(px(5.0))
                                                    .py(px(1.0))
                                                    .rounded(px(4.0))
                                                    .bg(TdesktopThemePalette::rgba_with_alpha(
                                                        prio_col, 0.16,
                                                    ))
                                                    .text_xs()
                                                    .text_color(rgb(prio_col))
                                                    .child(card.priority.clone()),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(4.0))
                                                    .when_some(prev_target, |row, pcol| {
                                                        row.child(action_btn(
                                                            "←",
                                                            pal.input_bg,
                                                            pal.text_primary,
                                                            move |this, _| {
                                                                this.pending_action = Some(
                                                                    TasksWorkspaceAction::MoveKanbanCard {
                                                                        card_id: cid_prev.clone(),
                                                                        column: pcol.clone(),
                                                                    },
                                                                );
                                                            },
                                                            cx,
                                                        ))
                                                    })
                                                    .when_some(next_target, |row, ncol| {
                                                        row.child(action_btn(
                                                            "→",
                                                            pal.input_bg,
                                                            pal.accent,
                                                            move |this, _| {
                                                                this.pending_action = Some(
                                                                    TasksWorkspaceAction::MoveKanbanCard {
                                                                        card_id: cid_next.clone(),
                                                                        column: ncol.clone(),
                                                                    },
                                                                );
                                                            },
                                                            cx,
                                                        ))
                                                    })
                                                    .child(action_btn(
                                                        "×",
                                                        pal.input_bg,
                                                        pal.danger,
                                                        move |this, _| {
                                                            this.pending_action = Some(
                                                                TasksWorkspaceAction::DeleteKanbanCard(
                                                                    cid_del.clone(),
                                                                ),
                                                            );
                                                        },
                                                        cx,
                                                    )),
                                            ),
                                    )
                            }))
                    })),
            ),
        )
        // 2. Pending Dangerous Command Approval Queue (/approve /deny)
        .child(
            studio_section_card(
                tr_str(
                    &lang,
                    "危险命令审批队列 (/approve · /deny)",
                    "Dangerous Command Approval Queue (/approve · /deny)",
                ),
                IconName::Lock,
                pal,
            )
            .when(
                pending_hermes.is_empty() && ws.pending_approvals.is_empty(),
                |el| {
                    el.child(
                        div()
                            .text_xs()
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                &lang,
                                "当前没有等待审批的高危工具命令。",
                                "No dangerous commands waiting for approval.",
                            )),
                    )
                },
            )
            .children(pending_hermes.into_iter().map(|ap| {
                let id_once = ap.approval_id.clone();
                let id_sess = ap.approval_id.clone();
                let id_deny = ap.approval_id.clone();
                div()
                    .p(px(10.0))
                    .rounded(px(9.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.danger))
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
                                    .child(format!("[{}] {}", ap.risk_level, ap.tool_name)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.accent))
                                    .child(ap.command_preview),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(ap.reason),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(action_btn(
                                "/approve once",
                                pal.success,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::DecideHermesApproval {
                                            approval_id: id_once.clone(),
                                            decision: "approved_once".to_string(),
                                        });
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Session",
                                pal.accent,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::DecideHermesApproval {
                                            approval_id: id_sess.clone(),
                                            decision: "approved_session".to_string(),
                                        });
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "/deny",
                                pal.danger,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::DecideHermesApproval {
                                            approval_id: id_deny.clone(),
                                            decision: "denied".to_string(),
                                        });
                                },
                                cx,
                            )),
                    )
            }))
            .children(ws.pending_approvals.iter().map(|req| {
                let rid_allow = req.id.clone();
                let rid_deny = req.id.clone();
                let detail = if !req.summary.trim().is_empty() {
                    req.summary.clone()
                } else {
                    req.arguments_preview.clone()
                };
                div()
                    .p(px(10.0))
                    .rounded(px(9.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
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
                                    .child(format!("{} · {}", req.tool_name, req.risk_level)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(detail),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .child(action_btn(
                                tr_str(&lang, "仅本次", "Once"),
                                pal.success,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ApprovePermission(
                                            rid_allow.clone(),
                                            ToolApprovalDecision::AllowOnce,
                                        ));
                                },
                                cx,
                            ))
                            .child(action_btn(
                                tr_str(&lang, "拒绝", "Deny"),
                                pal.danger,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ApprovePermission(
                                            rid_deny.clone(),
                                            ToolApprovalDecision::Deny,
                                        ));
                                },
                                cx,
                            )),
                    )
            })),
        )
        // 3. Active Subagent Lineage Tree & Legacy ACP Tasks
        .child(
            studio_section_card(
                tr_str(
                    &lang,
                    "子智能体谱系树与后台任务 (Subagent Lineage & ACP Tasks)",
                    "Active Subagent Lineage Tree & ACP Tasks",
                ),
                IconName::BotRobot,
                pal,
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div().flex_1().child(studio_input_box(
                            &ws.task_draft,
                            tr_str(&lang, "输入新后台智能体目标...", "New autonomous ACP task goal..."),
                            StudioInputTarget::NewTask,
                            ws.active_input,
                            pal,
                            false,
                            cx,
                        )),
                    )
                    .child(action_btn(
                        tr_str(&lang, "创建任务", "Create Task"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            let goal = this.task_draft.trim().to_string();
                            if !goal.is_empty() {
                                this.pending_action = Some(TasksWorkspaceAction::CreateTask(goal));
                                this.task_draft.clear();
                            }
                        },
                        cx,
                    )),
            )
            .children(ws.subagent_lineage.iter().take(12).map(|node| {
                let indent = if node.parent_session_id.is_some() { 20.0 } else { 0.0 };
                div()
                    .ml(px(indent))
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(if node.parent_session_id.is_some() {
                                        pal.accent
                                    } else {
                                        pal.success
                                    }))
                                    .child(if node.parent_session_id.is_some() {
                                        "↳ Subagent"
                                    } else {
                                        "● Root"
                                    }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_primary))
                                    .child(node.title.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(pal.text_secondary))
                            .child(format!("{} · {}", node.model, node.status)),
                    )
            }))
            .children(ws.tasks.iter().map(|t| {
                let tid = t.id.clone();
                div()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.window_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(t.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(format!(
                                        "{} · {}",
                                        localize_task_status(&lang, &t.status),
                                        t.schedule.as_deref().unwrap_or("immediate")
                                    )),
                            ),
                    )
                    .when(t.requires_approval, |el| {
                        el.child(action_btn(
                            tr_str(&lang, "批准执行", "Approve"),
                            pal.success,
                            pal.active_button_fg,
                            move |this, _| {
                                this.pending_action =
                                    Some(TasksWorkspaceAction::ApproveTask(tid.clone()));
                            },
                            cx,
                        ))
                    })
            })),
        )
        .into_any_element()
}

// ============================================================================
// Tab 2: Skills Hub (SKILL.md)
// ============================================================================

pub fn render_skills_hub_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();
    let q = ws.skill_search_query.trim().to_ascii_lowercase();
    let filtered: Vec<_> = ws
        .skills
        .iter()
        .filter(|s| {
            q.is_empty()
                || s.name.to_ascii_lowercase().contains(&q)
                || s.description.to_ascii_lowercase().contains(&q)
                || s.category.to_ascii_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        .child(
            studio_section_card(
                tr_str(&lang, "SKILL.md 技能中心", "Hermes Skills Hub (SKILL.md)"),
                IconName::FileDoc,
                pal,
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div().flex_1().min_w(px(180.0)).child(studio_input_box(
                            &ws.skill_search_query,
                            tr_str(&lang, "搜索技能名称或分类...", "Search SKILL.md library..."),
                            StudioInputTarget::SkillSearch,
                            ws.active_input,
                            pal,
                            false,
                            cx,
                        )),
                    )
                    .child(
                        div().w(px(170.0)).child(studio_input_box(
                            &ws.skill_learn_draft,
                            tr_str(&lang, "/learn 提炼技能名...", "/learn skill name..."),
                            StudioInputTarget::SkillLearnName,
                            ws.active_input,
                            pal,
                            false,
                            cx,
                        )),
                    )
                    .child(action_btn(
                        "/learn",
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            let name = this.skill_learn_draft.trim().to_string();
                            if !name.is_empty() {
                                this.pending_action =
                                    Some(TasksWorkspaceAction::LearnSkillFromSession(name));
                                this.skill_learn_draft.clear();
                            }
                        },
                        cx,
                    ))
                    .child(action_btn(
                        if ws.no_bundled_skills {
                            ".no-bundled-skills: ON"
                        } else {
                            ".no-bundled-skills: OFF"
                        },
                        if ws.no_bundled_skills {
                            pal.danger
                        } else {
                            pal.input_bg
                        },
                        if ws.no_bundled_skills {
                            pal.active_button_fg
                        } else {
                            pal.text_primary
                        },
                        |this, _| {
                            this.no_bundled_skills = !this.no_bundled_skills;
                            this.pending_action = Some(
                                TasksWorkspaceAction::ToggleNoBundledSkills(this.no_bundled_skills),
                            );
                        },
                        cx,
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(12.0))
                    // Left skill list
                    .child(
                        div()
                            .w(px(300.0))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .when(filtered.is_empty(), |el| {
                                el.child(
                                    div()
                                        .p(px(12.0))
                                        .rounded(px(8.0))
                                        .bg(rgb(pal.window_bg))
                                        .text_xs()
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(
                                            &lang,
                                            "暂无匹配技能，可使用 /learn 提炼或右侧创建 SKILL.md。",
                                            "No matching skills. Use /learn or create a new SKILL.md on the right.",
                                        )),
                                )
                            })
                            .children(filtered.into_iter().map(|sk| {
                                let is_sel = ws
                                    .selected_skill_name
                                    .as_deref()
                                    == Some(sk.name.as_str());
                                let s_click = sk.name.clone();
                                let s_toggle = sk.name.clone();
                                let next_en = !sk.enabled;
                                let s_approve = sk.name.clone();
                                let s_del = sk.name.clone();
                                let sec_color = match sk.security_status.as_str() {
                                    "critical" | "high" | "rejected" => pal.danger,
                                    "medium" | "pending" | "untrusted" => 0xE67E22,
                                    _ => pal.success,
                                };
                                div()
                                    .p(px(10.0))
                                    .rounded(px(9.0))
                                    .bg(rgb(pal.window_bg))
                                    .border_1()
                                    .border_color(rgb(if is_sel {
                                        pal.accent
                                    } else {
                                        pal.border_fg
                                    }))
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.select_skill(&s_click);
                                            cx.notify();
                                        }),
                                    )
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
                                                    .child(sk.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .px(px(6.0))
                                                    .py(px(1.0))
                                                    .rounded(px(5.0))
                                                    .bg(TdesktopThemePalette::rgba_with_alpha(
                                                        sec_color, 0.16,
                                                    ))
                                                    .text_xs()
                                                    .text_color(rgb(sec_color))
                                                    .child(if sk.security_status.is_empty() {
                                                        "safe".to_string()
                                                    } else {
                                                        sk.security_status.clone()
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .child(sk.description.clone()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(5.0))
                                            .child(action_btn(
                                                if sk.enabled { "Enabled" } else { "Disabled" },
                                                if sk.enabled { pal.success } else { pal.input_bg },
                                                if sk.enabled {
                                                    pal.active_button_fg
                                                } else {
                                                    pal.text_secondary
                                                },
                                                move |this, _| {
                                                    this.pending_action = Some(
                                                        TasksWorkspaceAction::ToggleSkillEnabled {
                                                            name: s_toggle.clone(),
                                                            enabled: next_en,
                                                        },
                                                    );
                                                },
                                                cx,
                                            ))
                                            .when(!sk.approved, |row| {
                                                row.child(action_btn(
                                                    "Approve Diff",
                                                    0xE67E22,
                                                    pal.active_button_fg,
                                                    move |this, _| {
                                                        this.pending_action = Some(
                                                            TasksWorkspaceAction::ApproveSkill(
                                                                s_approve.clone(),
                                                            ),
                                                        );
                                                    },
                                                    cx,
                                                ))
                                            })
                                            .child(action_btn(
                                                "Delete",
                                                pal.input_bg,
                                                pal.danger,
                                                move |this, _| {
                                                    this.pending_action = Some(
                                                        TasksWorkspaceAction::DeleteSkill(
                                                            s_del.clone(),
                                                        ),
                                                    );
                                                },
                                                cx,
                                            )),
                                    )
                            })),
                    )
                    // Right SKILL.md editor
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(300.0))
                            .p(px(12.0))
                            .rounded(px(10.0))
                            .bg(rgb(pal.window_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child("SKILL.md Viewer & Editor"),
                            )
                            .child(studio_input_box(
                                &ws.skill_editor_name,
                                "Skill name (e.g. rust-debugger)",
                                StudioInputTarget::SkillEditorName,
                                ws.active_input,
                                pal,
                                false,
                                cx,
                            ))
                            .child(studio_input_box(
                                &ws.skill_editor_content,
                                "---\nname: my-skill\ndescription: Reusable workflow\n---\n# Instructions",
                                StudioInputTarget::SkillEditorContent,
                                ws.active_input,
                                pal,
                                true,
                                cx,
                            ))
                            .child(
                                div().flex().justify_end().child(action_btn(
                                    tr_str(&lang, "保存 SKILL.md", "Save SKILL.md"),
                                    pal.accent,
                                    pal.active_button_fg,
                                    |this, _| {
                                        let name = this.skill_editor_name.trim().to_string();
                                        if !name.is_empty() {
                                            this.pending_action =
                                                Some(TasksWorkspaceAction::SaveSkill {
                                                    name,
                                                    content: this.skill_editor_content.clone(),
                                                    enabled: true,
                                                });
                                        }
                                    },
                                    cx,
                                )),
                            ),
                    ),
            ),
        )
        .into_any_element()
}

// ============================================================================
// Tab 3: Memory & SOUL.md (+ Honcho Dialectic Profile)
// ============================================================================

pub fn render_memory_soul_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();
    let mem_chars = ws.memory_edit_draft.chars().count();
    let user_chars = ws.user_edit_draft.chars().count();
    let mem_pct = ((mem_chars as f32 / 2200.0).min(1.0) * 100.0).round() as u32;
    let user_pct = ((user_chars as f32 / 1375.0).min(1.0) * 100.0).round() as u32;

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // Frozen Snapshot bar
        .child(
            div()
                .p(px(12.0))
                .rounded(px(10.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(tg_icon(IconName::Lock, 14.0, pal.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(format!(
                                    "Frozen Prompt Snapshot: {} ({} entries)",
                                    ws.snapshot_frozen_label,
                                    ws.memory_state.entries.len()
                                )),
                        ),
                )
                .child(action_btn(
                    tr_str(&lang, "刷新冻结快照 (/reload-memory)", "Refresh Snapshot"),
                    pal.accent,
                    pal.active_button_fg,
                    |this, _| {
                        this.pending_action = Some(TasksWorkspaceAction::ReloadFrozenSnapshot);
                    },
                    cx,
                )),
        )
        // Side-by-side MEMORY.md (2200) and USER.md (1375)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(12.0))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(280.0))
                        .child(
                            studio_section_card(
                                format!("MEMORY.md ({} / 2200 chars · {}%)", mem_chars, mem_pct),
                                IconName::Edit,
                                pal,
                            )
                            .child(
                                div()
                                    .h(px(6.0))
                                    .w_full()
                                    .rounded(px(999.0))
                                    .bg(rgb(pal.input_bg))
                                    .child(
                                        div()
                                            .h_full()
                                            .w(relative((mem_chars as f32 / 2200.0).min(1.0)))
                                            .rounded(px(999.0))
                                            .bg(rgb(if mem_chars > 2200 {
                                                pal.danger
                                            } else {
                                                pal.accent
                                            })),
                                    ),
                            )
                            .child(studio_input_box(
                                &ws.memory_edit_draft,
                                "Bounded agent notes separated by § (max 2200 chars)",
                                StudioInputTarget::MemoryMarkdown,
                                ws.active_input,
                                pal,
                                true,
                                cx,
                            ))
                            .child(
                                div().flex().justify_end().child(action_btn(
                                    tr_str(&lang, "保存 MEMORY.md", "Save MEMORY.md"),
                                    pal.accent,
                                    pal.active_button_fg,
                                    |this, _| {
                                        this.pending_action =
                                            Some(TasksWorkspaceAction::SaveMemoryRaw {
                                                scope: "memory".to_string(),
                                                content: this.memory_edit_draft.clone(),
                                            });
                                    },
                                    cx,
                                )),
                            ),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(280.0))
                        .child(
                            studio_section_card(
                                format!("USER.md ({} / 1375 chars · {}%)", user_chars, user_pct),
                                IconName::ContactsUsers,
                                pal,
                            )
                            .child(
                                div()
                                    .h(px(6.0))
                                    .w_full()
                                    .rounded(px(999.0))
                                    .bg(rgb(pal.input_bg))
                                    .child(
                                        div()
                                            .h_full()
                                            .w(relative((user_chars as f32 / 1375.0).min(1.0)))
                                            .rounded(px(999.0))
                                            .bg(rgb(if user_chars > 1375 {
                                                pal.danger
                                            } else {
                                                pal.success
                                            })),
                                    ),
                            )
                            .child(studio_input_box(
                                &ws.user_edit_draft,
                                "Bounded user preferences & traits (max 1375 chars)",
                                StudioInputTarget::UserMarkdown,
                                ws.active_input,
                                pal,
                                true,
                                cx,
                            ))
                            .child(
                                div().flex().justify_end().child(action_btn(
                                    tr_str(&lang, "保存 USER.md", "Save USER.md"),
                                    pal.success,
                                    pal.active_button_fg,
                                    |this, _| {
                                        this.pending_action =
                                            Some(TasksWorkspaceAction::SaveMemoryRaw {
                                                scope: "user".to_string(),
                                                content: this.user_edit_draft.clone(),
                                            });
                                    },
                                    cx,
                                )),
                            ),
                        ),
                ),
        )
        // SOUL.md Persona Editor & Honcho Dialectic Profile
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(12.0))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(280.0))
                        .child(
                            studio_section_card("Bot SOUL.md Persona", IconName::BotRobot, pal)
                                .child(studio_input_box(
                                    &ws.soul_persona_draft,
                                    "Persona display name (e.g. Hermes)",
                                    StudioInputTarget::SoulPersonaName,
                                    ws.active_input,
                                    pal,
                                    false,
                                    cx,
                                ))
                                .child(studio_input_box(
                                    &ws.soul_content_draft,
                                    "# SOUL.md\nTone, boundaries, style, and voice personality...",
                                    StudioInputTarget::SoulContent,
                                    ws.active_input,
                                    pal,
                                    true,
                                    cx,
                                ))
                                .child(
                                    div().flex().justify_end().child(action_btn(
                                        tr_str(&lang, "保存 SOUL.md", "Save SOUL.md"),
                                        pal.accent,
                                        pal.active_button_fg,
                                        |this, _| {
                                            let mut soul = this.soul_state.clone();
                                            soul.persona_name =
                                                this.soul_persona_draft.trim().to_string();
                                            soul.content = this.soul_content_draft.clone();
                                            this.pending_action =
                                                Some(TasksWorkspaceAction::SaveSoul(soul));
                                        },
                                        cx,
                                    )),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(280.0))
                        .child(
                            studio_section_card(
                                "Honcho Dialectic User Profile",
                                IconName::Search,
                                pal,
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .child(format!(
                                                "Peer: {}",
                                                ws.honcho_state
                                                    .peer_name
                                                    .as_deref()
                                                    .unwrap_or("Local User")
                                            )),
                                    )
                                    .child(action_btn(
                                        if ws.honcho_state.enabled {
                                            "Honcho: ON"
                                        } else {
                                            "Honcho: OFF"
                                        },
                                        if ws.honcho_state.enabled {
                                            pal.success
                                        } else {
                                            pal.input_bg
                                        },
                                        if ws.honcho_state.enabled {
                                            pal.active_button_fg
                                        } else {
                                            pal.text_secondary
                                        },
                                        |this, _| {
                                            let next = !this.honcho_state.enabled;
                                            this.honcho_state.enabled = next;
                                            this.pending_action =
                                                Some(TasksWorkspaceAction::ToggleHoncho(next));
                                        },
                                        cx,
                                    )),
                            )
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.window_bg))
                                    .text_xs()
                                    .text_color(rgb(pal.text_primary))
                                    .child(if ws.honcho_state.profile_summary.trim().is_empty() {
                                        "No dialectic profile insights recorded yet. Ask Honcho below to query user reasoning traits."
                                            .to_string()
                                    } else {
                                        ws.honcho_state.profile_summary.clone()
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(
                                        div().flex_1().child(studio_input_box(
                                            &ws.honcho_query_draft,
                                            "Query dialectic profile (e.g. coding style?)",
                                            StudioInputTarget::HonchoQuery,
                                            ws.active_input,
                                            pal,
                                            false,
                                            cx,
                                        )),
                                    )
                                    .child(action_btn(
                                        "Query",
                                        pal.accent,
                                        pal.active_button_fg,
                                        |this, _| {
                                            let q = this.honcho_query_draft.trim().to_string();
                                            if !q.is_empty() {
                                                this.pending_action =
                                                    Some(TasksWorkspaceAction::QueryHoncho(q));
                                            }
                                        },
                                        cx,
                                    )),
                            ),
                        ),
                ),
        )
        .into_any_element()
}

// ============================================================================
// Tab 4: Toolsets & Terminal (28 Toolsets + 6 Backends + Bg Processes + MCP)
// ============================================================================

pub fn render_toolsets_terminal_tab(
    ws: &GlobalTasksWorkspace,
    cx: &mut Context<GlobalTasksWorkspace>,
) -> AnyElement {
    let pal = ws.palette;
    let lang = ws.language_id.clone();
    let presets = ["hermes-telegram", "hermes-cli", "hermes-acp", "safe", "full", "minimal"];
    let backends = ["local", "docker", "ssh", "singularity", "modal", "daytona"];

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // 1. 28 Hermes Toolsets Toggle Grid + Preset Selector
        .child(
            studio_section_card(
                format!("28 Hermes Toolsets (Preset: {})", ws.active_toolset_preset),
                IconName::TerminalMcp,
                pal,
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
                            .child("Presets:"),
                    )
                    .children(presets.into_iter().map(|preset| {
                        let sel = ws.active_toolset_preset == preset;
                        let p_str = preset.to_string();
                        div()
                            .px(px(9.0))
                            .py(px(4.0))
                            .rounded(px(7.0))
                            .bg(rgb(if sel { pal.accent } else { pal.input_bg }))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(if sel {
                                pal.active_button_fg
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.active_toolset_preset = p_str.clone();
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ApplyToolsetPreset(p_str.clone()));
                                    cx.notify();
                                }),
                            )
                            .child(preset)
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.0))
                    .children(ws.toolsets.iter().map(|ts| {
                        let tname = ts.name.clone();
                        let next_en = !ts.enabled;
                        div()
                            .w(px(180.0))
                            .p(px(9.0))
                            .rounded(px(9.0))
                            .bg(rgb(pal.window_bg))
                            .border_1()
                            .border_color(rgb(if ts.enabled {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    if let Some(item) =
                                        this.toolsets.iter_mut().find(|x| x.name == tname)
                                    {
                                        item.enabled = next_en;
                                    }
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::ToggleToolset {
                                            name: tname.clone(),
                                            enabled: next_en,
                                        });
                                    cx.notify();
                                }),
                            )
                            .flex()
                            .flex_col()
                            .gap(px(3.0))
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
                                            .child(ts.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(if ts.enabled {
                                                pal.success
                                            } else {
                                                pal.text_secondary
                                            }))
                                            .child(if ts.enabled { "ON" } else { "OFF" }),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child(ts.description.clone()),
                            )
                    })),
            ),
        )
        // 2. 6 Terminal Backends & Live Background Process Table
        .child(
            studio_section_card(
                tr_str(
                    &lang,
                    "终端后端与后台进程监控",
                    "Terminal Execution Backends & Live Background Processes",
                ),
                IconName::ComputerDesktop,
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
                            .flex_wrap()
                            .items_center()
                            .gap(px(6.0))
                            .children(backends.into_iter().map(|b| {
                                let sel = ws.terminal_backend == b;
                                let b_str = b.to_string();
                                div()
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(px(7.0))
                                    .bg(rgb(if sel { pal.accent } else { pal.input_bg }))
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(if sel {
                                        pal.active_button_fg
                                    } else {
                                        pal.text_primary
                                    }))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.terminal_backend = b_str.clone();
                                            this.pending_action =
                                                Some(TasksWorkspaceAction::SetTerminalBackend(
                                                    b_str.clone(),
                                                ));
                                            cx.notify();
                                        }),
                                    )
                                    .child(b.to_uppercase())
                            })),
                    )
                    .child(action_btn(
                        tr_str(&lang, "轮询进程", "Poll Processes"),
                        pal.input_bg,
                        pal.accent,
                        |this, _| {
                            this.pending_action = Some(TasksWorkspaceAction::PollTerminalProcesses);
                        },
                        cx,
                    )),
            )
            .when(ws.terminal_processes.is_empty(), |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(pal.text_secondary))
                        .child("No active background terminal processes."),
                )
            })
            .children(ws.terminal_processes.iter().map(|proc_item| {
                let pid_poll = proc_item.process_id.clone();
                let pid_kill = proc_item.process_id.clone();
                div()
                    .p(px(9.0))
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
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!(
                                        "{} [{}] (PID {:?})",
                                        proc_item.command, proc_item.status, proc_item.pid
                                    )),
                            )
                            .when_some(proc_item.output_tail.clone(), |d, tail| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tail),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .child(action_btn(
                                "Poll",
                                pal.input_bg,
                                pal.accent,
                                move |this, _| {
                                    let _ = &pid_poll;
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::PollTerminalProcesses);
                                },
                                cx,
                            ))
                            .child(action_btn(
                                "Kill",
                                pal.danger,
                                pal.active_button_fg,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::KillTerminalProcess(
                                            pid_kill.clone(),
                                        ));
                                },
                                cx,
                            )),
                    )
            })),
        )
        // 3. Connected MCP Servers & Tools
        .child(
            studio_section_card(
                tr_str(&lang, "MCP 工具服务器状态", "MCP Servers & Tools Status"),
                IconName::SettingsGear,
                pal,
            )
            .when(ws.mcp_servers.is_empty(), |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(pal.text_secondary))
                        .child("No external MCP servers connected."),
                )
            })
            .children({
                let mut srv_cards: Vec<AnyElement> = Vec::with_capacity(ws.mcp_servers.len());
                for srv in &ws.mcp_servers {
                    let mut tool_btns: Vec<AnyElement> = Vec::with_capacity(srv.tools.len());
                    for tool in &srv.tools {
                        let sid = srv.id.clone();
                        let tname = tool.clone();
                        tool_btns.push(
                            action_btn(
                                tool.clone(),
                                pal.input_bg,
                                pal.accent,
                                move |this, _| {
                                    this.pending_action =
                                        Some(TasksWorkspaceAction::CallMcpTool(
                                            sid.clone(),
                                            tname.clone(),
                                        ));
                                },
                                cx,
                            )
                            .into_any_element(),
                        );
                    }
                    srv_cards.push(
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
                                            .child(format!("{} ({})", srv.name, srv.endpoint)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(if srv.is_connected {
                                                pal.success
                                            } else {
                                                pal.danger
                                            }))
                                            .child(if srv.is_connected { "Connected" } else { "Offline" }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(6.0))
                                    .children(tool_btns),
                            )
                            .into_any_element(),
                    );
                }
                srv_cards
            }),
        )
        .into_any_element()
}

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
        "local_hermes",
    ];

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
        // 2. API Mode, Fallback Chain & Providers
        .child(
            studio_section_card(
                tr_str(&lang, "API 协议模式与模型供应商", "API Modes, Providers & Fallback Chain"),
                IconName::SettingsGear,
                pal,
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
