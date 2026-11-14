//! Themed Tabbed Emoji, Stickers & GIFs Selector Popup (`selector_ui`).

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::stage::rich_meta::resolve_image_source;
use super::emoji_list_widget::{EmojiListWidget, EmojiSection};
use super::gifs_list_widget::GifsListWidget;
use super::stickers_list_widget::StickersListWidget;

#[derive(Debug, Clone)]
pub enum SelectorUiAction {
    SwitchTab(u8),
    SelectEmojiSection(EmojiSection),
    PickEmoji(String),
    PickSticker(String),
    PickDice(String),
    PickGif(u64, String),
    SelectStickerSet(u64),
    FocusSearch,
    ClearSearch,
}

const BUILTIN_EMOJIS: &[(EmojiSection, &str, &str)] = &[
    (EmojiSection::Recent, "👍", "thumbsup ok"), (EmojiSection::Recent, "❤️", "heart love"),
    (EmojiSection::Recent, "🔥", "fire hot"), (EmojiSection::Recent, "😂", "joy laugh"),
    (EmojiSection::Recent, "🎉", "party tada"), (EmojiSection::Recent, "🙏", "pray thanks"),
    (EmojiSection::Recent, "😍", "heart_eyes love"), (EmojiSection::Recent, "😭", "sob cry"),
    (EmojiSection::People, "😀", "grin smile"), (EmojiSection::People, "😃", "smiley happy"),
    (EmojiSection::People, "😄", "smile"), (EmojiSection::People, "😁", "grinning"),
    (EmojiSection::People, "😅", "sweat_smile"), (EmojiSection::People, "🤣", "rofl"),
    (EmojiSection::People, "😊", "blush"), (EmojiSection::People, "😉", "wink"),
    (EmojiSection::People, "😎", "sunglasses cool"), (EmojiSection::People, "🤔", "thinking"),
    (EmojiSection::People, "🥳", "partying"), (EmojiSection::People, "🤩", "star_struck"),
    (EmojiSection::People, "🥹", "pleading"), (EmojiSection::People, "👏", "clap"),
    (EmojiSection::People, "🤝", "handshake"), (EmojiSection::People, "👌", "ok_hand"),
    (EmojiSection::Nature, "🌟", "star"), (EmojiSection::Nature, "🌈", "rainbow"),
    (EmojiSection::Nature, "🌸", "blossom"), (EmojiSection::Nature, "🌙", "moon"),
    (EmojiSection::Food, "☕", "coffee"), (EmojiSection::Food, "🍕", "pizza"),
    (EmojiSection::Food, "🍔", "burger"), (EmojiSection::Food, "🍰", "cake"),
    (EmojiSection::Activity, "⚽", "soccer"), (EmojiSection::Activity, "🏀", "basketball"),
    (EmojiSection::Activity, "🎯", "dart"), (EmojiSection::Activity, "🎮", "game"),
    (EmojiSection::Travel, "✈️", "plane"), (EmojiSection::Travel, "🚀", "rocket"),
    (EmojiSection::Objects, "💻", "laptop"), (EmojiSection::Objects, "📱", "phone"),
    (EmojiSection::Objects, "📎", "paperclip"), (EmojiSection::Objects, "🔒", "lock"),
    (EmojiSection::Symbols, "✅", "check"), (EmojiSection::Symbols, "💯", "100"),
    (EmojiSection::Symbols, "✨", "sparkles"), (EmojiSection::Symbols, "⚡", "zap"),
];

pub fn render_tabbed_selector_popup<V: 'static>(
    active_tab: u8,
    search_query: &str,
    search_focused: bool,
    emoji_list: &EmojiListWidget,
    stickers_list: &StickersListWidget,
    gifs_list: &GifsListWidget,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, SelectorUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let a_focus = on_action.clone();
    let a_clr = on_action.clone();
    let a_t0 = on_action.clone();
    let a_t1 = on_action.clone();
    let a_t2 = on_action.clone();

    let body = match active_tab {
        0 => render_emoji_body(search_query, emoji_list, palette, cx, on_action),
        1 => render_stickers_body(search_query, stickers_list, palette, lang_id, cx, on_action),
        _ => render_gifs_body(search_query, gifs_list, palette, lang_id, cx, on_action),
    };

    div()
        .id("tabbed-selector-popup")
        .w(px(346.0))
        .h(px(332.0))
        .mb(px(8.0))
        .rounded(px(12.0))
        .bg(rgb(palette.modal_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(
            div()
                .px(px(10.0))
                .py(px(8.0))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .child(
                    div()
                        .id("sel-search-box")
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(8.0))
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(rgb(if search_focused { palette.accent } else { palette.border_fg }))
                        .cursor_text()
                        .on_click(cx.listener(move |this, _, _, cx| a_focus(this, SelectorUiAction::FocusSearch, cx)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(6.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(tg_icon(IconName::Search, 14.0, palette.icon_fg))
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .text_color(rgb(if search_query.is_empty() { palette.text_secondary } else { palette.text_primary }))
                                        .child(if search_query.is_empty() {
                                            tr_key_or(lang_id, "lng_dlg_filter", "Search...").to_string()
                                        } else {
                                            search_query.to_string()
                                        }),
                                ),
                        )
                        .when(!search_query.is_empty(), |el| {
                            el.child(
                                div()
                                    .id("sel-search-clear")
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| a_clr(this, SelectorUiAction::ClearSearch, cx)))
                                    .child(tg_icon(IconName::Close, 12.0, palette.icon_fg)),
                            )
                        }),
                ),
        )
        .child(div().id("sel-scroll-body").flex_1().overflow_y_scroll().child(body))
        .child(
            div()
                .h(px(40.0))
                .px(px(10.0))
                .border_t_1()
                .border_color(rgb(palette.border_fg))
                .bg(rgb(palette.topbar_bg))
                .flex()
                .items_center()
                .justify_around()
                .child(tab_btn("sel-tab-emoji", IconName::SmileEmoji, "Emoji", active_tab == 0, palette, cx, move |this, cx| a_t0(this, SelectorUiAction::SwitchTab(0), cx)))
                .child(tab_btn("sel-tab-stickers", IconName::Sticker, "Stickers", active_tab == 1, palette, cx, move |this, cx| a_t1(this, SelectorUiAction::SwitchTab(1), cx)))
                .child(tab_btn("sel-tab-gifs", IconName::Gif, "GIFs", active_tab == 2, palette, cx, move |this, cx| a_t2(this, SelectorUiAction::SwitchTab(2), cx))),
        )
        .into_any_element()
}

fn tab_btn<V: 'static>(
    id: &'static str, icon: IconName, label: &'static str, active: bool, palette: &TdesktopThemePalette,
    cx: &mut Context<V>, on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let fg = if active { palette.accent } else { palette.icon_fg };
    let hbg = palette.menu_hover_bg;
    div()
        .id(id)
        .px(px(12.0))
        .py(px(5.0))
        .rounded(px(8.0))
        .cursor_pointer()
        .hover(move |s| s.bg(rgb(hbg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .flex()
        .items_center()
        .gap(px(5.0))
        .child(tg_icon(icon, 15.0, fg))
        .child(div().text_size(px(12.0)).font_weight(if active { FontWeight::BOLD } else { FontWeight::NORMAL }).text_color(rgb(fg)).child(label))
        .into_any_element()
}

fn render_emoji_body<V: 'static>(
    query: &str, emoji_list: &EmojiListWidget, palette: &TdesktopThemePalette,
    cx: &mut Context<V>, on_action: impl Fn(&mut V, SelectorUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let q = query.trim().to_lowercase();
    let mut grid = Vec::new();
    for (idx, &(sec, glyph, kw)) in BUILTIN_EMOJIS.iter().enumerate() {
        let matches = if q.is_empty() { sec == emoji_list.active_section || emoji_list.active_section == EmojiSection::Recent && idx < 16 } else { kw.contains(&q) || glyph.contains(&q) };
        if matches {
            let em = glyph.to_string();
            let a = on_action.clone();
            grid.push(
                div().id(SharedString::from(format!("em-btn-{}", idx))).w(px(36.0)).h(px(36.0)).rounded(px(8.0)).cursor_pointer()
                    .hover(|s| s.bg(rgb(palette.menu_hover_bg))).flex().items_center().justify_center().text_size(px(20.0))
                    .on_click(cx.listener(move |this, _, _, cx| a(this, SelectorUiAction::PickEmoji(em.clone()), cx)))
                    .child(glyph).into_any_element(),
            );
        }
    }
    let cats = [
        (EmojiSection::Recent, "🕒"), (EmojiSection::People, "😀"), (EmojiSection::Nature, "🌿"),
        (EmojiSection::Food, "🍕"), (EmojiSection::Activity, "⚽"), (EmojiSection::Travel, "✈️"),
        (EmojiSection::Objects, "💡"), (EmojiSection::Symbols, "✅"),
    ];
    let mut cat_bar = div().flex().items_center().justify_between().px(px(8.0)).py(px(4.0)).border_b_1().border_color(rgb(palette.border_fg));
    for (ci, &(sec, icon)) in cats.iter().enumerate() {
        let active = emoji_list.active_section == sec;
        let a = on_action.clone();
        cat_bar = cat_bar.child(
            div().id(SharedString::from(format!("em-cat-{}", ci))).px(px(6.0)).py(px(3.0)).rounded(px(6.0)).cursor_pointer()
                .when(active, |el| el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.18)))
                .on_click(cx.listener(move |this, _, _, cx| a(this, SelectorUiAction::SelectEmojiSection(sec), cx)))
                .text_size(px(14.0)).child(icon),
        );
    }
    div().flex().flex_col().child(cat_bar).child(div().p(px(8.0)).flex().flex_wrap().gap(px(3.0)).children(grid)).into_any_element()
}

fn render_stickers_body<V: 'static>(
    query: &str, stickers_list: &StickersListWidget, palette: &TdesktopThemePalette, lang_id: &str,
    cx: &mut Context<V>, on_action: impl Fn(&mut V, SelectorUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let q = query.trim().to_lowercase();
    let dice_bar = ["🎲", "🎯", "🏀", "⚽", "🎳", "🎰"];
    let mut dice_row = div().flex().items_center().gap(px(6.0)).px(px(10.0)).py(px(6.0)).border_b_1().border_color(rgb(palette.border_fg));
    for (i, &d) in dice_bar.iter().enumerate() {
        let ds = d.to_string();
        let a = on_action.clone();
        dice_row = dice_row.child(
            div().id(SharedString::from(format!("stk-dice-{}", i))).w(px(38.0)).h(px(36.0)).rounded(px(8.0))
                .bg(rgb(palette.input_bg)).cursor_pointer().hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .flex().items_center().justify_center().text_size(px(20.0))
                .on_click(cx.listener(move |this, _, _, cx| a(this, SelectorUiAction::PickDice(ds.clone()), cx)))
                .child(d),
        );
    }
    let items = stickers_list.prepare_stickers(false);
    let mut grid = Vec::new();
    for (idx, st) in items.iter().enumerate() {
        if !q.is_empty() && !st.alt_emoticon.to_lowercase().contains(&q) {
            continue;
        }
        let em = st.alt_emoticon.clone();
        let img_src = st.thumbnail_url.as_deref().and_then(|u| resolve_image_source(u, None));
        let a = on_action.clone();
        grid.push(
            div().id(SharedString::from(format!("stk-it-{}", idx))).w(px(70.0)).h(px(70.0)).rounded(px(8.0)).cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg))).flex().items_center().justify_center().overflow_hidden()
                .on_click(cx.listener(move |this, _, _, cx| a(this, SelectorUiAction::PickSticker(em.clone()), cx)))
                .when_some(img_src, |el, s| el.child(img(s).size_full().object_fit(ObjectFit::Contain)))
                .when(st.thumbnail_url.is_none(), |el| el.text_size(px(32.0)).child(st.alt_emoticon.clone()))
                .into_any_element(),
        );
    }
    div().flex().flex_col().child(dice_row).child(
        div().p(px(10.0)).flex().flex_wrap().gap(px(6.0))
            .when(grid.is_empty(), |el| el.child(div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(tr_key_or(lang_id, "lng_stickers_default_hint", "Tap an animated dice above or add sticker packs"))))
            .children(grid),
    ).into_any_element()
}

fn render_gifs_body<V: 'static>(
    query: &str, gifs_list: &GifsListWidget, palette: &TdesktopThemePalette, lang_id: &str,
    cx: &mut Context<V>, on_action: impl Fn(&mut V, SelectorUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let q = query.trim().to_lowercase();
    let mut cards = Vec::new();
    for (idx, gif) in gifs_list.visible_items().iter().enumerate() {
        if !q.is_empty() && !gif.stream_url.to_lowercase().contains(&q) {
            continue;
        }
        let thumb = resolve_image_source(&gif.thumbnail_url, None);
        let did = gif.document_id;
        let url = gif.stream_url.clone();
        let a = on_action.clone();
        cards.push(
            div().id(SharedString::from(format!("gif-it-{}", idx))).w(px(100.0)).h(px(72.0)).rounded(px(8.0))
                .bg(rgb(palette.input_bg)).overflow_hidden().cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| a(this, SelectorUiAction::PickGif(did, url.clone()), cx)))
                .relative()
                .when_some(thumb, |el, s| el.child(img(s).size_full().object_fit(ObjectFit::Cover)))
                .child(div().absolute().top(px(4.0)).left(px(4.0)).px(px(5.0)).py(px(1.0)).rounded(px(4.0))
                    .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.55)).text_size(px(9.5)).text_color(rgb(0xFFFFFF)).child("GIF"))
                .into_any_element(),
        );
    }
    div().p(px(10.0)).flex().flex_wrap().gap(px(6.0))
        .when(cards.is_empty(), |el| el.child(div().w_full().py(px(32.0)).flex().justify_center().text_size(px(12.5)).text_color(rgb(palette.text_secondary)).child(tr_key_or(lang_id, "lng_gifs_no_saved", "No saved GIFs yet"))))
        .children(cards)
        .into_any_element()
}
