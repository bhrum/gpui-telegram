use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::components::tool_permission_dock::ToolApprovalDecision;
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::TdesktopThemePalette;
use crate::views::global_tasks_workspace::{
    localize_task_status, GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

use super::{action_btn, studio_input_box, studio_section_card};

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
