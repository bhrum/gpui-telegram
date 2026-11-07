use gpui::*;

pub struct DialogRow {
    pub title: String,
    pub preview: String,
    pub unread_count: u32,
    pub is_selected: bool,
}

impl DialogRow {
    pub fn new(title: String, preview: String, unread_count: u32, is_selected: bool) -> Self {
        Self {
            title,
            preview,
            unread_count,
            is_selected,
        }
    }
}

impl Render for DialogRow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg_color = if self.is_selected {
            rgb(0x2AABEE)
        } else {
            rgb(0xFFFFFF)
        };

        div()
            .h(px(64.0))
            .w_full()
            .bg(bg_color)
            .px(px(12.0))
            .flex()
            .items_center()
            .child(
                div()
                    .flex_1()
                    .child(div().font_weight(FontWeight::BOLD).child(self.title.clone()))
                    .child(div().text_size(px(12.0)).child(self.preview.clone())),
            )
    }
}
