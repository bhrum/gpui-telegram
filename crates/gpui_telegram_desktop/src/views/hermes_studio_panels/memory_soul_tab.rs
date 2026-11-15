use gpui::*;

use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::views::global_tasks_workspace::{
    GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

use super::{action_btn, studio_input_box, studio_section_card};

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
