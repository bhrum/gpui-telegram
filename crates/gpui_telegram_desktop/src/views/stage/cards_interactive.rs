//! Themed 1:1 GPUI Renderers for Poll/Quiz (with explanation), Location, Call, Story, Invoice & BotKeyboard.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::cards::models::{
    CallDirection, CallType, InlineButtonType, InlineKeyboardButton, InvoiceStatus, PollOption,
    PollType,
};

#[derive(Debug, Clone)]
pub enum InteractiveCardAction {
    VotePoll(u64, u32),
    CallBack(u64, bool),
    OpenStory(u64, String),
    PayInvoice(u64, String, u64),
    BotButton(u64, InlineButtonType),
    GenerativeButton(u64, String, String),
}

pub fn render_poll_card<V: 'static>(
    msg_id: u64,
    question: &str,
    poll_type: PollType,
    options: &[PollOption],
    total_votes: u32,
    is_closed: bool,
    selected_option_id: Option<u32>,
    quiz_explanation: Option<&str>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let show_results = is_closed || selected_option_id.is_some();
    let is_quiz = matches!(poll_type, PollType::Quiz);
    let type_label = match poll_type {
        PollType::Regular => tr_key_or(lang_id, "lng_polls_poll", "Poll"),
        PollType::Quiz => tr_key_or(lang_id, "lng_polls_quiz", "Quiz"),
        PollType::Anonymous => tr_key_or(lang_id, "lng_polls_anonymous", "Anonymous Poll"),
    };

    let mut card = div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .min_w(px(250.0))
        .max_w(px(360.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(question.to_string()),
                )
                .child(tg_icon(IconName::Poll, 15.0, palette.accent)),
        )
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(palette.text_secondary))
                .child(if is_closed {
                    format!("{} · Closed", type_label)
                } else {
                    type_label.to_string()
                }),
        );

    for opt in options {
        let oid = opt.id;
        let act = on_action.clone();
        let chosen = opt.is_selected || selected_option_id == Some(oid);
        let fill_color = if show_results && is_quiz {
            if opt.is_correct {
                palette.success
            } else if chosen {
                palette.danger
            } else {
                palette.accent
            }
        } else {
            palette.accent
        };
        let fill_pct = if show_results { opt.percentage.clamp(0.0, 100.0) } else { 0.0 };

        card = card.child(
            div()
                .id(SharedString::from(format!("poll-{}-{}", msg_id, oid)))
                .relative()
                .w_full()
                .px(px(10.0))
                .py(px(7.0))
                .rounded(px(8.0))
                .overflow_hidden()
                .bg(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.25))
                .border_1()
                .border_color(if chosen {
                    rgb(fill_color)
                } else {
                    TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.5)
                })
                .when(!is_closed, |el| {
                    el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                        act(this, InteractiveCardAction::VotePoll(msg_id, oid), cx);
                    }))
                })
                .when(show_results && fill_pct > 0.0, |el| {
                    el.child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left_0()
                            .w(relative(fill_pct / 100.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(fill_color, 0.20)),
                    )
                })
                .child(
                    div()
                        .relative()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .w(px(16.0))
                                        .h(px(16.0))
                                        .rounded_full()
                                        .border_1()
                                        .border_color(rgb(if chosen { fill_color } else { palette.text_secondary }))
                                        .when(chosen || (show_results && is_quiz && opt.is_correct), |c| {
                                            c.bg(rgb(fill_color))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(tg_icon(IconName::SelectCheck, 10.0, 0xFFFFFF))
                                        }),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(rgb(palette.text_primary))
                                        .child(opt.text.clone()),
                                ),
                        )
                        .when(show_results, |el| {
                            el.child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(if chosen { fill_color } else { palette.text_secondary }))
                                    .child(format!("{:.0}%", opt.percentage)),
                            )
                        }),
                ),
        );
    }

    card = card
        .when(show_results && is_quiz && quiz_explanation.is_some(), |el| {
            el.child(
                div()
                    .mt(px(2.0))
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.14))
                    .border_l_2()
                    .border_color(rgb(palette.accent))
                    .text_size(px(12.0))
                    .text_color(rgb(palette.text_primary))
                    .child(format!("💡 {}", quiz_explanation.unwrap_or_default())),
            )
        })
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(palette.text_secondary))
                .child(format!("{} votes", total_votes)),
        );

    card.into_any_element()
}

pub fn render_location_card(
    latitude: f64,
    longitude: f64,
    title: &str,
    address: &str,
    palette: &TdesktopThemePalette,
) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .w(px(280.0))
        .rounded(px(10.0))
        .overflow_hidden()
        .border_1()
        .border_color(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.6))
        .child(
            div()
                .relative()
                .h(px(118.0))
                .bg(if palette.mode.is_fabushi_style() {
                    if palette.mode.is_dark() { rgb(0x1C1E22) } else { rgb(0xEAECEF) }
                } else if palette.mode.is_dark() {
                    rgb(0x1E2F40)
                } else {
                    rgb(0xDCE7EE)
                })
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w(px(36.0))
                        .h(px(36.0))
                        .rounded_full()
                        .bg(rgb(palette.danger))
                        .flex()
                        .items_center()
                        .justify_center()
                        .shadow_md()
                        .child(tg_icon(IconName::LocationPin, 18.0, 0xFFFFFF)),
                )
                .child(
                    div()
                        .absolute()
                        .bottom(px(6.0))
                        .right(px(8.0))
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.55))
                        .text_size(px(10.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(format!("{:.4}, {:.4}", latitude, longitude)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .px(px(10.0))
                .py(px(8.0))
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(title.to_string()),
                )
                .when(!address.is_empty(), |el| {
                    el.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(palette.text_secondary))
                            .child(address.to_string()),
                    )
                }),
        )
        .into_any_element()
}

pub fn render_call_card<V: 'static>(
    msg_id: u64,
    call_type: CallType,
    direction: CallDirection,
    duration_secs: u32,
    status_text: &str,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + 'static,
) -> AnyElement {
    let is_video = matches!(call_type, CallType::Video);
    let is_missed = matches!(direction, CallDirection::Missed);
    let label = match direction {
        CallDirection::Outgoing => if is_video { "Outgoing Video Call" } else { "Outgoing Call" },
        CallDirection::Incoming => if is_video { "Incoming Video Call" } else { "Incoming Call" },
        CallDirection::Missed => if is_video { "Missed Video Call" } else { "Missed Call" },
    };
    let sub = if !status_text.is_empty() {
        status_text.to_string()
    } else if duration_secs >= 60 {
        format!("{} min {} sec", duration_secs / 60, duration_secs % 60)
    } else if duration_secs > 0 {
        format!("{} sec", duration_secs)
    } else {
        "Tap to call back".to_string()
    };
    let tint = if is_missed { palette.danger } else { palette.accent };

    div()
        .id(SharedString::from(format!("call-card-{}", msg_id)))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.0))
        .min_w(px(210.0))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            on_action(this, InteractiveCardAction::CallBack(msg_id, is_video), cx);
        }))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(label),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(if is_missed { palette.danger } else { palette.text_secondary }))
                        .child(sub),
                ),
        )
        .child(
            div()
                .w(px(34.0))
                .h(px(34.0))
                .rounded_full()
                .bg(TdesktopThemePalette::rgba_with_alpha(tint, 0.16))
                .flex()
                .items_center()
                .justify_center()
                .child(tg_icon(
                    if is_video { IconName::VideoCall } else { IconName::PhoneCall },
                    16.0,
                    tint,
                )),
        )
        .into_any_element()
}

pub fn render_story_card<V: 'static>(
    msg_id: u64,
    user_name: &str,
    avatar_initials: &str,
    caption: &str,
    preview_color: u32,
    expires_in: &str,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + 'static,
) -> AnyElement {
    let auth_owned = user_name.to_string();
    let is_expired = expires_in.eq_ignore_ascii_case("expired");
    div()
        .id(SharedString::from(format!("story-card-{}", msg_id)))
        .flex()
        .items_center()
        .gap(px(10.0))
        .p(px(8.0))
        .rounded(px(10.0))
        .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.10))
        .border_l_2()
        .border_color(if is_expired { rgb(palette.text_secondary) } else { rgb(0xA855F7) })
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            on_action(this, InteractiveCardAction::OpenStory(msg_id, auth_owned.clone()), cx);
        }))
        .child(
            div()
                .w(px(38.0))
                .h(px(46.0))
                .rounded(px(8.0))
                .border_2()
                .border_color(if is_expired { rgb(palette.text_secondary) } else { rgb(0xEC4899) })
                .bg(rgb(preview_color))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xFFFFFF))
                .child(avatar_initials.to_string()),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(if is_expired { palette.text_secondary } else { palette.accent }))
                        .child(if expires_in.is_empty() {
                            format!("Story · {}", user_name)
                        } else {
                            format!("Story · {} ({})", user_name, expires_in)
                        }),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(palette.text_primary))
                        .child(caption.to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_invoice_card<V: 'static>(
    msg_id: u64,
    title: &str,
    description: &str,
    amount_stars: u64,
    currency: &str,
    status: InvoiceStatus,
    receipt_id: Option<&str>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + 'static,
) -> AnyElement {
    let is_paid = matches!(status, InvoiceStatus::Paid);
    let is_refunded = matches!(status, InvoiceStatus::Refunded);
    let price_str = if currency.eq_ignore_ascii_case("XTR") || currency.eq_ignore_ascii_case("STARS") {
        format!("{} Stars", amount_stars)
    } else {
        format!("{:.2} {}", amount_stars as f64 / 100.0, currency)
    };
    let cur_owned = currency.to_string();

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .min_w(px(240.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xF59E0B))
                        .child(tg_icon(IconName::PremiumStar, 14.0, 0xF59E0B))
                        .child(price_str.clone()),
                )
                .when_some(receipt_id, |el, rid| {
                    el.child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(palette.text_secondary))
                            .child(format!("#{}", rid)),
                    )
                }),
        )
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(palette.text_primary))
                .child(title.to_string()),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(palette.text_secondary))
                .child(description.to_string()),
        )
        .child(
            div()
                .id(SharedString::from(format!("invoice-pay-{}", msg_id)))
                .w_full()
                .py(px(7.0))
                .rounded(px(8.0))
                .bg(rgb(if is_paid { palette.success } else if is_refunded { palette.text_secondary } else { palette.accent }))
                .flex()
                .items_center()
                .justify_center()
                .when(!is_paid && !is_refunded, |el| {
                    el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                        on_action(this, InteractiveCardAction::PayInvoice(msg_id, cur_owned.clone(), amount_stars), cx);
                    }))
                })
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xFFFFFF))
                .child(if is_paid {
                    format!("✓ Paid ({})", price_str)
                } else if is_refunded {
                    format!("Refunded ({})", price_str)
                } else {
                    format!("Pay {}", price_str)
                }),
        )
        .into_any_element()
}

pub fn render_inline_bot_keyboard<V: 'static>(
    msg_id: u64,
    rows: &[Vec<InlineKeyboardButton>],
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let mut grid = div().flex().flex_col().flex_shrink_0().gap(px(4.0)).mt(px(4.0)).w_full();
    for (r_idx, row) in rows.iter().enumerate() {
        let mut row_el = div().flex().flex_row().h(px(34.0)).flex_shrink_0().gap(px(4.0)).w_full();
        for (c_idx, btn) in row.iter().enumerate() {
            let label = btn.text.clone();
            let btype = btn.button_type.clone();
            let is_ext = matches!(btype, InlineButtonType::Url(_) | InlineButtonType::WebApp(_));
            let act = on_action.clone();
            row_el = row_el.child(
                div()
                    .id(SharedString::from(format!("bot-btn-{}-{}-{}", msg_id, r_idx, c_idx)))
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .h(px(34.0))
                    .px(px(8.0))
                    .rounded(px(8.0))
                    .bg(palette.service_pill_rgba())
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.85))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        act(this, InteractiveCardAction::BotButton(msg_id, btype.clone()), cx);
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .min_w(px(0.0))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(palette.service_pill_fg))
                            .truncate()
                            .child(label),
                    )
                    .when(is_ext, |el| {
                        el.child(
                            div()
                                .text_size(px(10.0))
                                .text_color(rgb(palette.service_pill_fg))
                                .child("↗"),
                        )
                    }),
            );
        }
        grid = grid.child(row_el);
    }
    grid.into_any_element()
}
