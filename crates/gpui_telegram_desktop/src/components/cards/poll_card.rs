use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::cards::PollCard;
use std::sync::Arc;

pub struct PollCardView {
    pub poll: PollCard,
    pub palette: TdesktopThemePalette,
    pub on_vote: Option<Arc<dyn Fn(u8) + Send + Sync>>,
}

impl PollCardView {
    pub fn new(poll: PollCard) -> Self {
        Self {
            poll,
            palette: TdesktopThemePalette::default(),
            on_vote: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn vote_option(&mut self, option_id: u8) {
        if self.poll.is_closed {
            return;
        }
        let mut changed = false;
        for opt in &mut self.poll.options {
            if opt.option_id == option_id {
                if !opt.is_chosen {
                    opt.is_chosen = true;
                    opt.vote_count += 1;
                    self.poll.total_voters += 1;
                    changed = true;
                }
            } else if !self.poll.allows_multiple_answers && opt.is_chosen {
                opt.is_chosen = false;
                opt.vote_count = opt.vote_count.saturating_sub(1);
                self.poll.total_voters = self.poll.total_voters.saturating_sub(1);
            }
        }
        if changed {
            if let Some(ref cb) = self.on_vote {
                cb(option_id);
            }
        }
    }
}

impl Render for PollCardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let total = self.poll.total_voters.max(1) as f32;
        let kind_label = if self.poll.is_quiz {
            "Quiz"
        } else if self.poll.is_anonymous {
            "Anonymous Poll"
        } else {
            "Poll"
        };

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
                    .gap(px(6.0))
                    .child(tg_icon(IconName::Poll, 15.0, pal.accent))
                    .child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(kind_label),
                    ),
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.0))
                    .text_color(rgb(pal.text_primary))
                    .child(self.poll.question.clone()),
            )
            .children(self.poll.options.iter().map(|opt| {
                let opt_id = opt.option_id;
                let pct = ((opt.vote_count as f32 / total) * 100.0).round() as u32;
                let is_chosen = opt.is_chosen;
                div()
                    .cursor_pointer()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(if is_chosen {
                        TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.18)
                    } else {
                        rgb(pal.input_bg)
                    })
                    .border_1()
                    .border_color(rgb(if is_chosen { pal.accent } else { pal.border_fg }))
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _e, _w, cx| {
                            this.vote_option(opt_id);
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .when(is_chosen, |el| el.child(tg_icon(IconName::SelectCheck, 13.0, pal.accent)))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .child(opt.text.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!("{}%", pct)),
                    )
            }))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_secondary))
                    .child(format!("{} votes", self.poll.total_voters)),
            )
    }
}
