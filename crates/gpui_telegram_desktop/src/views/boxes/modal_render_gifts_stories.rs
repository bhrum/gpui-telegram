use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header, DialogClose,
    DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::boxes::send_credits_box::SendCreditsBox;
use crate::views::boxes::star_gift_box::StarGiftBox;
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_star_gifts<V: 'static>(
    dlg: &ActiveModalDialog,
    peer_name: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let catalog = StarGiftBox::default_catalog();

    modal_card(460.0, pal)
        .child(modal_header(
            IconName::WalletStars,
            &format!("{} {}", tr_str(lang, "赠送礼物给", "Send Gift to"), peer_name),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(10.0))
                .children(catalog.into_iter().map(|item| {
                    let emoji = item.icon_emoji.clone();
                    let title = item.title.clone();
                    let stars = item.stars_price;
                    div()
                        .cursor_pointer()
                        .w(px(130.0))
                        .p(px(12.0))
                        .rounded(px(10.0))
                        .bg(rgb(pal.input_bg))
                        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(6.0))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).pending_outcome = Some(ModalActionOutcome::SendStarGift {
                                    emoji: emoji.clone(),
                                    title: title.clone(),
                                    stars,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(div().text_size(px(28.0)).child(item.icon_emoji))
                        .child(div().text_size(px(11.5)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(item.title))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .child(tg_icon(IconName::WalletStars, 12.0, pal.accent))
                                .child(div().text_size(px(11.5)).text_color(rgb(pal.accent)).child(stars.to_string())),
                        )
                })),
        )
        .into_any_element()
}

pub fn render_send_credits<V: 'static>(
    dlg: &ActiveModalDialog,
    invoice_title: String,
    stars_amount: u64,
    balance_stars: u64,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let box_state = SendCreditsBox::new(invoice_title.clone(), stars_amount, balance_stars, None);
    let can_pay = box_state.has_sufficient_balance();

    modal_card(390.0, pal)
        .child(modal_header(
            IconName::WalletStars,
            tr_str(lang, "确认星币支付", "Confirm Telegram Stars"),
            pal,
            on_close,
            cx,
        ))
        .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(invoice_title.clone()))
        .child(
            div()
                .p(px(12.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .flex()
                .justify_between()
                .child(div().text_size(px(13.0)).text_color(rgb(pal.text_secondary)).child(tr_str(lang, "需支付 / 当前余额", "Amount / Balance")))
                .child(div().text_size(px(13.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.accent)).child(format!("{} / {}", stars_amount, balance_stars))),
        )
        .map(|el| {
            if !can_pay {
                el.child(modal_error_banner(Some(&format!("Insufficient Stars (need {} more)", box_state.missing_stars())), pal))
            } else {
                el
            }
        })
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(if can_pay { pal.accent } else { pal.muted_badge_bg }))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                if can_pay {
                                    access(this).pending_outcome = Some(ModalActionOutcome::ConfirmSendCredits {
                                        invoice_title: invoice_title.clone(),
                                        stars_amount,
                                    });
                                    on_close(this);
                                    cx.notify();
                                }
                            }),
                        )
                        .child(format!("{} {}", tr_str(lang, "支付", "Pay"), stars_amount)),
                ),
        )
        .into_any_element()
}

pub fn render_stories_viewer<V: 'static>(
    dlg: &ActiveModalDialog,
    peer_name: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    if dlg.stories.is_empty() {
        return modal_card(380.0, pal)
            .child(modal_header(IconName::Camera, &peer_name, pal, on_close, cx))
            .child(
                div()
                    .py(px(36.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .child(tg_icon(IconName::Camera, 36.0, pal.text_secondary))
                    .child(div().text_size(px(13.0)).text_color(rgb(pal.text_secondary)).child(format!("{} {}", peer_name, tr_str(lang, "暂无动态", "has no active stories")))),
            )
            .into_any_element();
    }

    let total = dlg.stories.len();
    let idx = dlg.story_slide_idx % total;
    let slide = &dlg.stories[idx];
    let peer_for_send = peer_name.clone();

    div()
        .w(px(380.0))
        .h(px(580.0))
        .rounded(px(16.0))
        .bg(rgb(slide.bg_color))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .p(px(18.0))
        .flex()
        .flex_col()
        .justify_between()
        .on_mouse_down(MouseButton::Left, |_e, _w, cx| cx.stop_propagation())
        .child(modal_header(IconName::Camera, &format!("{} ({}/{})", peer_name, idx + 1, total), pal, on_close, cx))
        .child(div().text_size(px(16.0)).text_color(rgb(pal.active_button_fg)).child(slide.caption.clone()))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(modal_field(
                    tr_str(lang, "回复动态", "Reply"),
                    &dlg.primary_input,
                    tr_str(lang, "发送私信回复...", "Send a reply..."),
                    ActiveTextField::Primary,
                    dlg.active_field == ActiveTextField::Primary,
                    pal,
                    access,
                    cx,
                ))
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(12.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                    access(this).story_slide_idx += 1;
                                    cx.notify();
                                }))
                                .child(tr_str(lang, "下一条", "Next").to_string()),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(14.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.accent))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.active_button_fg))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                    let d = access(this);
                                    let text = d.primary_input.trim().to_string();
                                    if !text.is_empty() {
                                        d.pending_outcome = Some(ModalActionOutcome::SendStoryReply {
                                            peer_name: peer_for_send.clone(),
                                            text,
                                        });
                                        on_close(this);
                                    }
                                    cx.notify();
                                }))
                                .child(tr_str(lang, "发送", "Send").to_string()),
                        ),
                ),
        )
        .into_any_element()
}
