use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::cards::TodoCard;
use std::sync::Arc;

pub struct TodoCardView {
    pub todo: TodoCard,
    pub palette: TdesktopThemePalette,
    pub on_toggle_item: Option<Arc<dyn Fn(String, bool) + Send + Sync>>,
}

impl TodoCardView {
    pub fn new(todo: TodoCard) -> Self {
        Self {
            todo,
            palette: TdesktopThemePalette::default(),
            on_toggle_item: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn toggle_item(&mut self, item_id: &str) {
        let mut new_state = None;
        for item in &mut self.todo.items {
            if item.id == item_id {
                item.completed = !item.completed;
                new_state = Some(item.completed);
                break;
            }
        }
        self.todo.completed_count = self.todo.items.iter().filter(|i| i.completed).count() as u32;
        if let (Some(done), Some(cb)) = (new_state, &self.on_toggle_item) {
            cb(item_id.to_string(), done);
        }
    }
}

impl Render for TodoCardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;

        div()
            .p(px(12.0))
            .rounded(px(pal.bubble_radius_px as f32))
            .bg(rgb(pal.card_bg))
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
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.0))
                            .text_color(rgb(pal.text_primary))
                            .child(self.todo.title.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!("{}/{}", self.todo.completed_count, self.todo.items.len())),
                    ),
            )
            .children(self.todo.items.iter().map(|item| {
                let item_id = item.id.clone();
                let done = item.completed;
                div()
                    .cursor_pointer()
                    .px(px(8.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(pal.input_bg))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _e, _w, cx| {
                            this.toggle_item(&item_id);
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded(px(4.0))
                            .bg(rgb(if done { pal.accent } else { pal.card_bg }))
                            .border_1()
                            .border_color(rgb(if done { pal.accent } else { pal.border_fg }))
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(done, |el| {
                                el.child(tg_icon(IconName::SelectCheck, 12.0, pal.active_button_fg))
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(if done {
                                pal.text_secondary
                            } else {
                                pal.text_primary
                            }))
                            .child(item.title.clone()),
                    )
            }))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_secondary))
                    .child(format!("{} tasks", self.todo.items.len())),
            )
    }
}
