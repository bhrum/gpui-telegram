//! Authentic Telegram Desktop Chat Wallpaper & Empty-State Rendering.

use gpui::*;
use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

pub fn render_themed_wallpaper(palette: &TdesktopThemePalette) -> AnyElement {
    let gradient_bg = linear_gradient(
        145.0,
        linear_color_stop(rgb(palette.wallpaper_top), 0.0),
        linear_color_stop(rgb(palette.wallpaper_bottom), 1.0),
    );
    let pattern_rgba = TdesktopThemePalette::rgba_with_alpha(
        palette.wallpaper_pattern,
        if palette.mode.is_dark() { 0.14 } else { 0.10 },
    );

    // Seamless full-area diagonal gradient + subtle geometric Telegram pattern motif
    div()
        .absolute()
        .inset_0()
        .overflow_hidden()
        .bg(gradient_bg)
        .child(
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
                        .opacity(0.45)
                        .child(tg_icon(IconName::TelegramPlaneLogo, 22.0, palette.wallpaper_pattern))
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
                        .child(tg_icon(IconName::TelegramPlaneLogo, 18.0, palette.wallpaper_pattern))
                        .child(tg_icon(IconName::PremiumStar, 14.0, palette.wallpaper_pattern))
                })),
        )
        .into_any_element()
}

pub fn render_empty_no_chat_pill(palette: &TdesktopThemePalette, lang_id: &str) -> AnyElement {
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
                .child(tg_icon(IconName::TelegramPlaneLogo, 38.0, palette.accent))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.service_pill_fg))
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
