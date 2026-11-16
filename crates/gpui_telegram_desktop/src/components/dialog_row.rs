use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub struct DialogRow {
    pub title: String,
    pub preview: String,
    pub unread_count: u32,
    pub is_selected: bool,
    pub is_muted: bool,
    pub is_pinned: bool,
    pub is_verified: bool,
    pub timestamp: String,
    pub avatar_color: u32,
    pub palette: TdesktopThemePalette,
}

impl DialogRow {
    pub fn new(title: String, preview: String, unread_count: u32, is_selected: bool) -> Self {
        Self {
            title,
            preview,
            unread_count,
            is_selected,
            is_muted: false,
            is_pinned: false,
            is_verified: false,
            timestamp: String::new(),
            avatar_color: 0x2AABEE,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }
}

impl Render for DialogRow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let use_inverted_sel = self.is_selected && !pal.mode.is_fabushi_style();
        let bg_color = if self.is_selected {
            rgb(pal.dialogs_active_bg)
        } else {
            rgb(pal.dialogs_bg)
        };
        let title_color = if use_inverted_sel {
            rgb(pal.active_button_fg)
        } else {
            rgb(pal.text_primary)
        };
        let sub_color = if use_inverted_sel {
            rgb(pal.active_button_fg)
        } else {
            rgb(pal.text_secondary)
        };
        let initial: String = self.title.chars().take(2).collect();

        div()
            .h(px(64.0))
            .w_full()
            .bg(bg_color)
            .when(!self.is_selected, |el| el.hover(move |s| s.bg(rgb(pal.dialogs_hover_bg))))
            .px(px(12.0))
            .flex()
            .items_center()
            .gap(px(10.0))
            .child(
                div()
                    .w(px(46.0))
                    .h(px(46.0))
                    .rounded_full()
                    .bg(rgb(self.avatar_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.active_button_fg))
                    .child(initial),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .text_size(px(13.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(title_color)
                                            .child(self.title.clone()),
                                    )
                                    .when(self.is_verified, |el| {
                                        el.child(tg_icon(IconName::Verified, 14.0, pal.accent))
                                    })
                                    .when(self.is_muted, |el| {
                                        el.child(tg_icon(IconName::Mute, 12.0, pal.muted_badge_bg))
                                    }),
                            )
                            .when(!self.timestamp.is_empty(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(sub_color)
                                        .child(self.timestamp.clone()),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w(px(0.0))
                                    .text_size(px(12.5))
                                    .text_color(sub_color)
                                    .overflow_hidden()
                                    .child(self.preview.clone()),
                            )
                            .when(self.unread_count > 0, |el| {
                                el.child(
                                    div()
                                        .min_w(px(20.0))
                                        .h(px(20.0))
                                        .px(px(6.0))
                                        .rounded_full()
                                        .bg(rgb(if self.is_muted {
                                            pal.muted_badge_bg
                                        } else {
                                            pal.unread_badge_bg
                                        }))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.unread_badge_fg))
                                        .child(self.unread_count.to_string()),
                                )
                            })
                            .when(self.unread_count == 0 && self.is_pinned, |el| {
                                el.child(tg_icon(IconName::Pin, 13.0, pal.icon_fg))
                            }),
                    ),
            )
    }
}
