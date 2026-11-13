use gpui::*;

pub struct ToolPermissionDock {
    pub pending_count: usize,
}

impl ToolPermissionDock {
    pub fn new(pending_count: usize) -> Self {
        Self { pending_count }
    }
}

impl Render for ToolPermissionDock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .bg(rgb(0xFFF3E0))
            .child(format!("{} Pending Tool Approvals", self.pending_count))
    }
}
