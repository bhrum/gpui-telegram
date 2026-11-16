//! Authentic Telegram Desktop Chat Wallpaper & Empty-State Rendering.

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

pub fn render_themed_wallpaper(palette: &TdesktopThemePalette) -> AnyElement {
    let gradient_bg = linear_gradient(
        145.0,
        linear_color_stop(rgb(palette.wallpaper_top), 0.0),
        linear_color_stop(rgb(palette.wallpaper_bottom), 1.0),
    );
    let is_fabushi = palette.mode.is_fabushi_style();
    let pattern_rgba = TdesktopThemePalette::rgba_with_alpha(
        palette.wallpaper_pattern,
        if palette.mode.is_dark() { 0.14 } else { 0.10 },
    );

    let pattern_opacity = if palette.blur_wallpaper { 0.16 } else { 0.45 };

    // Clean flat/gradient Fabushi canvas, or subtle geometric Telegram pattern motif for Classic mode
    div()
        .absolute()
        .inset_0()
        .overflow_hidden()
        .bg(gradient_bg)
        .when(!is_fabushi, |el| {
            el.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .flex_col()
                    .justify_around()
                    .px(px(28.0))
                    .py(px(24.0))
                    .children((0..5).map(move |row| {
                        let offset_left = if row % 2 == 0 { 0.0 } else { 36.0 };
                        div()
                            .flex()
                            .items_center()
                            .justify_around()
                            .pl(px(offset_left))
                            .opacity(pattern_opacity)
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(pattern_rgba),
                            )
                            .child(
                                div()
                                    .w(px(14.0))
                                    .h(px(14.0))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(pattern_rgba),
                            )
                            .child(tg_icon(IconName::HeartReaction, 16.0, palette.wallpaper_pattern))
                            .child(tg_icon(IconName::SmileEmoji, 18.0, palette.wallpaper_pattern))
                            .child(
                                div()
                                    .w(px(8.0))
                                    .h(px(8.0))
                                    .rounded_full()
                                    .bg(pattern_rgba),
                            )
                            .child(tg_icon(IconName::PremiumStar, 14.0, palette.wallpaper_pattern))
                    })),
            )
        })
        .when(palette.blur_wallpaper, |el| {
            let veil = TdesktopThemePalette::rgba_with_alpha(
                palette.window_bg,
                if palette.mode.is_dark() { 0.42 } else { 0.34 },
            );
            el.child(
                div()
                    .id("stage-wallpaper-blur-veil")
                    .absolute()
                    .inset_0()
                    .bg(veil),
            )
        })
        .into_any_element()
}

pub fn render_empty_no_chat_pill(palette: &TdesktopThemePalette, lang_id: &str) -> AnyElement {
    let is_fabushi = palette.mode.is_fabushi_style();
    div()
        .size_full()
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .child(render_themed_wallpaper(palette))
        .child(
            div()
                .relative()
                .px(px(18.0))
                .py(px(8.0))
                .rounded_full()
                .bg(palette.service_pill_rgba())
                .when(is_fabushi, |el| el.border_1().border_color(rgb(palette.border_fg)))
                .shadow_sm()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(palette.service_pill_fg))
                .child(tr_key_or(
                    lang_id,
                    "lng_chat_select_empty",
                    "Select a chat to start messaging",
                )),
        )
        .into_any_element()
}

pub fn render_empty_history_placeholder(
    palette: &TdesktopThemePalette,
    lang_id: &str,
) -> AnyElement {
    let is_fabushi = palette.mode.is_fabushi_style();
    let title_fg = if is_fabushi {
        palette.text_primary
    } else {
        palette.service_pill_fg
    };
    div()
        .flex_1()
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(10.0))
                .max_w(px(260.0))
                .px(px(22.0))
                .py(px(20.0))
                .rounded(px(16.0))
                .bg(palette.service_pill_rgba())
                .when(is_fabushi, |el| el.border_1().border_color(rgb(palette.border_fg)))
                .child(tg_icon(IconName::FolderChat, 38.0, palette.accent))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(title_fg))
                        .child(tr_key_or(lang_id, "lng_empty_history_title", "No messages here yet...")),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(palette.text_secondary))
                        .child(tr_key_or(
                            lang_id,
                            "lng_empty_history_subtitle",
                            "Send a message below to start the conversation.",
                        )),
                ),
        )
        .into_any_element()
}
