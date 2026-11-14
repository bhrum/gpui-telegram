use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub struct ThinkingBubble {
    pub thoughts: String,
    pub is_expanded: bool,
    pub palette: TdesktopThemePalette,
}

impl ThinkingBubble {
    pub fn new(thoughts: String) -> Self {
        Self {
            thoughts,
            is_expanded: false,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

impl Render for ThinkingBubble {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let chevron = if self.is_expanded {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        };

        div()
            .p(px(10.0))
            .rounded(px(8.0))
            .bg(rgb(pal.code_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.toggle_expanded();
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(tg_icon(IconName::BotRobot, 14.0, pal.accent))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_secondary))
                                    .child("Reasoning"),
                            ),
                    )
                    .child(tg_icon(chevron, 14.0, pal.icon_fg)),
            )
            .when(self.is_expanded || !self.thoughts.is_empty(), |el| {
                let preview = if self.is_expanded {
                    self.thoughts.clone()
                } else {
                    let short: String = self.thoughts.chars().take(120).collect();
                    if self.thoughts.chars().count() > 120 {
                        format!("{short}...")
                    } else {
                        short
                    }
                };
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(pal.code_fg))
                        .child(preview),
                )
            })
    }
}
