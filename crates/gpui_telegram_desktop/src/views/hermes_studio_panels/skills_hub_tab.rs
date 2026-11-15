use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::TdesktopThemePalette;
use crate::views::global_tasks_workspace::{
    GlobalTasksWorkspace, StudioInputTarget, TasksWorkspaceAction,
};

use super::{action_btn, studio_input_box, studio_section_card};

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
