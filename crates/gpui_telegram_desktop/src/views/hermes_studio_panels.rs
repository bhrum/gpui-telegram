use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::global_tasks_workspace::{GlobalTasksWorkspace, StudioInputTarget};

pub mod cron_heartbeat_tab;
pub mod kanban_approvals_tab;
pub mod memory_soul_tab;
pub mod sessions_providers_tab;
pub mod skills_hub_tab;
pub mod toolsets_terminal_tab;

pub use cron_heartbeat_tab::render_cron_heartbeat_tab;
pub use kanban_approvals_tab::render_tasks_kanban_approvals_tab;
pub use memory_soul_tab::render_memory_soul_tab;
pub use sessions_providers_tab::render_sessions_providers_tab;
pub use skills_hub_tab::render_skills_hub_tab;
pub use toolsets_terminal_tab::render_toolsets_terminal_tab;

pub(crate) fn studio_input_box(
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

pub(crate) fn studio_section_card(
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

pub(crate) fn action_btn(
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
