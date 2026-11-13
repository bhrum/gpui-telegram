use gpui::*;

pub struct GlobalTasksWorkspace {
    pub active_tab: usize,
}

impl GlobalTasksWorkspace {
    pub fn new() -> Self {
        Self { active_tab: 0 }
    }
}

impl Default for GlobalTasksWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for GlobalTasksWorkspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .p(px(24.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Global Tasks & Automations Workspace"),
            )
    }
}
