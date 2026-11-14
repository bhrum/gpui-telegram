//! Autocomplete Popup, Attachments Drawer, Formatting Toolbar & Send Options Menu (`popups_ui`).

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::attachments_drawer::AttachmentKind;
use super::field_autocomplete::{AutocompleteType, FieldAutocomplete};
use super::formatting_bar::{FormattingBar, TextStyleKind};

#[derive(Debug, Clone)]
pub enum ComposerPopupAction {
    ChooseAutocomplete(String),
    ChooseAttachment(AttachmentKind),
    ApplyFormat(TextStyleKind),
    SendSilent,
    ScheduleMessage,
    ClosePopups,
}

const EMOJI_SHORTCODES: &[(&str, &str)] = &[
    (":thumbsup:", "👍"), (":+1:", "👍"), (":heart:", "❤️"), (":fire:", "🔥"),
    (":joy:", "😂"), (":smile:", "😄"), (":rocket:", "🚀"), (":party:", "🎉"),
    (":check:", "✅"), (":100:", "💯"), (":thinking:", "🤔"), (":eyes:", "👀"),
];

pub fn render_autocomplete_popup<V: 'static>(
    ac: &FieldAutocomplete,
    emoji_token: Option<&str>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerPopupAction, &mut Context<V>) + Clone + 'static,
) -> Option<AnyElement> {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    if let Some(etok) = emoji_token {
        let q = etok.to_lowercase();
        for &(code, glyph) in EMOJI_SHORTCODES {
            if code.starts_with(&q) || code.contains(&q[1..]) {
                rows.push((glyph.to_string(), code.to_string(), glyph.to_string()));
            }
        }
    } else if ac.is_open {
        match ac.autocomplete_type {
            AutocompleteType::Mentions => {
                for m in &ac.mention_rows {
                    rows.push((format!("@{}", m.username), m.name.clone(), format!("@{} ", m.username)));
                }
            }
            AutocompleteType::Hashtags => {
                for h in &ac.hashtag_rows {
                    rows.push((format!("#{}", h.tag), "Hashtag".to_string(), format!("#{} ", h.tag)));
                }
            }
            AutocompleteType::BotCommands => {
                for c in &ac.bot_command_rows {
                    rows.push((format!("/{}", c.command), c.description.clone(), format!("/{} ", c.command)));
                }
            }
            AutocompleteType::Stickers => {
                for s in &ac.sticker_rows {
                    rows.push((s.emoji.clone(), "Sticker".to_string(), s.emoji.clone()));
                }
            }
        }
    }

    if rows.is_empty() {
        return None;
    }

    let sel_idx = ac.selected_index.max(0) as usize;
    let hbg = palette.menu_hover_bg;
    let mut box_el = div()
        .id("composer-autocomplete-popup")
        .w(px(340.0))
        .max_h(px(220.0))
        .overflow_y_scroll()
        .mb(px(6.0))
        .p(px(6.0))
        .rounded(px(10.0))
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(2.0));

    for (idx, (primary, secondary, insert)) in rows.into_iter().enumerate() {
        let act = on_action.clone();
        let active = idx == sel_idx;
        box_el = box_el.child(
            div()
                .id(SharedString::from(format!("ac-row-{}", idx)))
                .px(px(10.0))
                .py(px(6.0))
                .rounded(px(7.0))
                .when(active, |el| el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.20)))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hbg)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    act(this, ComposerPopupAction::ChooseAutocomplete(insert.clone()), cx);
                }))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(10.0))
                .child(div().text_size(px(13.0)).font_weight(FontWeight::BOLD).text_color(rgb(palette.accent)).child(primary))
                .child(div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(secondary)),
        );
    }

    Some(box_el.into_any_element())
}

pub fn render_attachments_drawer<V: 'static>(
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerPopupAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let entries = [
        (AttachmentKind::PhotoOrVideo, IconName::Camera, tr_key_or(lang_id, "lng_attach_photo_video", "Photo or Video")),
        (AttachmentKind::DocumentOrFile, IconName::FileDoc, tr_key_or(lang_id, "lng_attach_file", "File")),
        (AttachmentKind::CreatePoll, IconName::Poll, tr_key_or(lang_id, "lng_polls_create", "Poll")),
        (AttachmentKind::SendLocation, IconName::LocationPin, tr_key_or(lang_id, "lng_attach_location", "Location")),
        (AttachmentKind::ShareContact, IconName::ContactsUsers, tr_key_or(lang_id, "lng_attach_contact", "Contact")),
    ];
    let hbg = palette.menu_hover_bg;
    let mut box_el = div()
        .id("composer-attachments-popup")
        .w(px(210.0))
        .mb(px(8.0))
        .p(px(6.0))
        .rounded(px(10.0))
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(2.0));

    for (idx, (kind, icon, label)) in entries.into_iter().enumerate() {
        let act = on_action.clone();
        box_el = box_el.child(
            div()
                .id(SharedString::from(format!("att-opt-{}", idx)))
                .px(px(10.0))
                .py(px(7.0))
                .rounded(px(7.0))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hbg)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    act(this, ComposerPopupAction::ChooseAttachment(kind), cx);
                }))
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(tg_icon(icon, 16.0, palette.accent))
                .child(div().text_size(px(13.0)).text_color(rgb(palette.text_primary)).child(label.to_string())),
        );
    }

    box_el.into_any_element()
}

pub fn render_formatting_toolbar<V: 'static>(
    fmt: &FormattingBar,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerPopupAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let items = [
        (TextStyleKind::Bold, "B"),
        (TextStyleKind::Italic, "I"),
        (TextStyleKind::Underline, "U"),
        (TextStyleKind::Strikethrough, "S"),
        (TextStyleKind::Monospace, "</>"),
        (TextStyleKind::Spoiler, "||"),
        (TextStyleKind::Link, "Link"),
    ];
    let hbg = palette.menu_hover_bg;
    let mut row = div()
        .id("composer-formatting-bar")
        .px(px(8.0))
        .py(px(5.0))
        .mb(px(6.0))
        .rounded(px(8.0))
        .bg(rgb(palette.card_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .gap(px(4.0));

    for (idx, (kind, lbl)) in items.into_iter().enumerate() {
        let active = fmt.has_style(kind);
        let a = on_action.clone();
        row = row.child(
            div()
                .id(SharedString::from(format!("fmt-btn-{}", idx)))
                .px(px(9.0))
                .py(px(3.0))
                .rounded(px(6.0))
                .when(active, |el| el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.22)))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hbg)))
                .on_click(cx.listener(move |this, _, _, cx| a(this, ComposerPopupAction::ApplyFormat(kind), cx)))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(if active { palette.accent } else { palette.text_primary }))
                .child(lbl),
        );
    }
    row.into_any_element()
}

pub fn render_send_options_popup<V: 'static>(
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerPopupAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let a_sil = on_action.clone();
    let a_sch = on_action;
    let hbg = palette.menu_hover_bg;
    div()
        .id("composer-send-opts")
        .w(px(210.0))
        .p(px(6.0))
        .mb(px(6.0))
        .rounded(px(10.0))
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .id("send-silent-opt")
                .px(px(10.0))
                .py(px(7.0))
                .rounded(px(7.0))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hbg)))
                .on_click(cx.listener(move |this, _, _, cx| a_sil(this, ComposerPopupAction::SendSilent, cx)))
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(tg_icon(IconName::Mute, 15.0, palette.icon_fg))
                .child(div().text_size(px(13.0)).text_color(rgb(palette.text_primary)).child(tr_key_or(lang_id, "lng_send_silent_message", "Send Without Sound"))),
        )
        .child(
            div()
                .id("send-schedule-opt")
                .px(px(10.0))
                .py(px(7.0))
                .rounded(px(7.0))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hbg)))
                .on_click(cx.listener(move |this, _, _, cx| a_sch(this, ComposerPopupAction::ScheduleMessage, cx)))
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(tg_icon(IconName::ClockSending, 15.0, palette.icon_fg))
                .child(div().text_size(px(13.0)).text_color(rgb(palette.text_primary)).child(tr_key_or(lang_id, "lng_schedule_message", "Schedule Message"))),
        )
        .into_any_element()
}
