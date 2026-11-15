use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::views::global_tasks_workspace::{GlobalTasksWorkspace, TasksWorkspaceAction};

use super::{action_btn, studio_section_card};

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
