//! 1:1 Telegram Desktop global & categorized sidebar search header and results.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{
    CategorizedSearchResult, SidebarCatalogAppItem, TELEGRAM_SEARCH_CATEGORIES,
    TELEGRAM_SEARCH_TAGS,
};
use crate::views::drawer::bot_computer_inspector::format_bot_mau_subtitle;
use crate::views::sidebar::ui::chat_search_empty::ChatSearchEmptyUi;
use crate::views::sidebar::ui::chat_search_in::ChatSearchInUi;
use crate::views::sidebar::ui::top_peers_strip::TopPeerItem;

fn split_highlight_segments(text: &str, query: &str) -> (String, String, String) {
    let q_chars: Vec<char> = query.trim().chars().flat_map(|c| c.to_lowercase()).collect();
    if q_chars.is_empty() {
        return (text.to_string(), String::new(), String::new());
    }
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for start_idx in 0..chars.len() {
        let mut cursor = start_idx;
        let mut matched = true;
        let mut q_pos = 0usize;
        while q_pos < q_chars.len() {
            if cursor >= chars.len() {
                matched = false;
                break;
            }
            let lower: Vec<char> = chars[cursor].1.to_lowercase().collect();
            if q_pos + lower.len() > q_chars.len() || lower.as_slice() != &q_chars[q_pos..q_pos + lower.len()] {
                matched = false;
                break;
            }
            q_pos += lower.len();
            cursor += 1;
        }
        if matched {
            let b_start = chars[start_idx].0;
            let b_end = if cursor < chars.len() { chars[cursor].0 } else { text.len() };
            return (text[..b_start].to_string(), text[b_start..b_end].to_string(), text[b_end..].to_string());
        }
    }
    (text.to_string(), String::new(), String::new())
}

fn tag_icon_for(tag_id: &str) -> IconName {
    match tag_id {
        "bot" => IconName::BotRobot,
        "unread" => IconName::SelectCheck,
        "pinned" => IconName::Pin,
        "media" => IconName::Camera,
        "links" => IconName::Search,
        _ => IconName::FolderAll,
    }
}

fn tag_en_label(tag_id: &str) -> &'static str {
    match tag_id {
        "bot" => "AI Bot",
        "unread" => "Unread",
        "pinned" => "Pinned",
        "media" => "Media",
        "links" => "Links",
        _ => "All",
    }
}

pub fn render_search_panel<V: 'static>(
    active_cat: &str,
    active_tag: &str,
    top_peers: &[TopPeerItem],
    search_in: &ChatSearchInUi,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_cat: impl Fn(&mut V, String, &mut Context<V>) + Clone + 'static,
    on_tag: impl Fn(&mut V, String, &mut Context<V>) + Clone + 'static,
    on_peer: impl Fn(&mut V, u64, &mut Context<V>) + Clone + 'static,
    on_clear_scope: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let hover_primary = rgb(palette.text_primary);
    let cat_tabs: Vec<AnyElement> = TELEGRAM_SEARCH_CATEGORIES
        .iter()
        .map(|(cat_id, cat_label)| {
            let is_active = active_cat == *cat_id;
            let cid = (*cat_id).to_string();
            let cb = on_cat.clone();
            div()
                .cursor_pointer()
                .relative()
                .px(px(6.0))
                .py(px(7.0))
                .text_size(px(12.0))
                .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::NORMAL })
                .text_color(if is_active { rgb(palette.accent) } else { rgb(palette.text_secondary) })
                .hover(move |s| if is_active { s } else { s.text_color(hover_primary) })
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, cid.clone(), cx);
                }))
                .child(tr_str(lang_id, *cat_label, *cat_id))
                .when(is_active, |el| el.child(div().absolute().bottom_0().left_0().right_0().h(px(2.0)).bg(rgb(palette.accent))))
                .into_any_element()
        })
        .collect();

    let peer_cards: Vec<AnyElement> = top_peers
        .iter()
        .map(|p| {
            let pid = p.id;
            let cb = on_peer.clone();
            div()
                .cursor_pointer()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, pid, cx);
                }))
                .child(
                    div()
                        .relative()
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded_full()
                        .bg(rgb(p.avatar_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(p.avatar_initials.clone())
                        .when(p.is_online, |el| {
                            el.child(div().absolute().bottom_0().right_0().w(px(10.0)).h(px(10.0)).rounded_full().bg(rgb(palette.online_dot_fg)).border_2().border_color(rgb(palette.dialogs_bg)))
                        }),
                )
                .child(div().text_size(px(11.0)).text_color(rgb(palette.text_primary)).child(p.name.clone()))
                .into_any_element()
        })
        .collect();

    let tag_chips: Vec<AnyElement> = TELEGRAM_SEARCH_TAGS
        .iter()
        .map(|(tag_id, tag_label)| {
            let is_active = active_tag == *tag_id;
            let tid = (*tag_id).to_string();
            let cb = on_tag.clone();
            div()
                .cursor_pointer()
                .px(px(7.0))
                .py(px(3.0))
                .rounded_full()
                .bg(if is_active { rgb(palette.accent) } else { rgb(palette.composer_bg) })
                .border_1()
                .border_color(if is_active { rgb(palette.accent) } else { rgb(palette.border_fg) })
                .flex()
                .items_center()
                .gap(px(4.0))
                .text_size(px(10.5))
                .text_color(if is_active { rgb(palette.active_button_fg) } else { rgb(palette.text_secondary) })
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, tid.clone(), cx);
                }))
                .child(tg_icon(tag_icon_for(tag_id), 11.0, if is_active { palette.active_button_fg } else { palette.icon_fg }))
                .child(tr_str(lang_id, *tag_label, tag_en_label(tag_id)))
                .into_any_element()
        })
        .collect();

    div()
        .id("sidebar-categorized-search-header")
        .w_full()
        .bg(rgb(palette.dialogs_bg))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .flex_col()
        .child(div().id("sidebar-search-category-tabs").w_full().px(px(6.0)).border_b_1().border_color(rgb(palette.border_fg)).flex().flex_wrap().items_center().gap(px(2.0)).children(cat_tabs))
        .when_some(search_in.peer_name.clone(), |el, scope_name| {
            el.child(
                div().w_full().px(px(10.0)).py(px(5.0)).bg(rgb(palette.composer_bg)).flex().items_center().justify_between()
                    .child(div().text_size(px(11.5)).text_color(rgb(palette.accent)).child(format!("{}: {}", tr_str(lang_id, "限定对话", "In chat"), scope_name)))
                    .child(div().cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| on_clear_scope(this, cx))).child(tg_icon(IconName::Close, 12.0, palette.icon_fg))),
            )
        })
        .when(!peer_cards.is_empty(), |el| {
            el.child(
                div().w_full().px(px(12.0)).py(px(8.0)).border_b_1().border_color(rgb(palette.border_fg)).flex().flex_col().gap(px(8.0))
                    .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(palette.text_secondary)).child(tr_str(lang_id, "常用联系人", "Recent")))
                    .child(div().w_full().flex().items_center().gap(px(12.0)).children(peer_cards)),
            )
        })
        .child(div().w_full().px(px(8.0)).py(px(6.0)).flex().flex_wrap().items_center().gap(px(5.0)).children(tag_chips))
        .into_any_element()
}

fn filter_catalog_apps<'a>(
    apps: &'a [SidebarCatalogAppItem],
    q_trim: &str,
) -> Vec<&'a SidebarCatalogAppItem> {
    apps.iter()
        .filter(|app| {
            q_trim.is_empty()
                || app.title.to_lowercase().contains(q_trim)
                || app.short_name.to_lowercase().contains(q_trim)
                || app.bot_username.to_lowercase().contains(q_trim)
                || app.description.to_lowercase().contains(q_trim)
        })
        .collect()
}

fn append_apps_catalog_group<V: 'static>(
    out: &mut Vec<AnyElement>,
    id_prefix: &'static str,
    header_label: &str,
    items: &[&SidebarCatalogAppItem],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_select_app: impl Fn(&mut V, SidebarCatalogAppItem, &mut Context<V>) + Clone + 'static,
) {
    if items.is_empty() {
        return;
    }
    let hover = rgb(palette.dialogs_hover_bg);
    out.push(
        div()
            .id(SharedString::from(format!("{id_prefix}-header")))
            .w_full()
            .px(px(12.0))
            .py(px(6.0))
            .bg(rgb(palette.composer_bg))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .text_size(px(11.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(palette.text_secondary))
            .child(header_label.to_string())
            .into_any_element(),
    );

    for (idx, app) in items.iter().enumerate() {
        let app_row = (*app).clone();
        let cb_row = on_select_app.clone();
        let icon_col = if app.avatar_color != 0 {
            app.avatar_color
        } else {
            palette.accent
        };
        let initials = if app.icon_initials.is_empty() {
            app.title.chars().take(2).collect::<String>().to_uppercase()
        } else {
            app.icon_initials.clone()
        };
        let mau_subtitle = format_bot_mau_subtitle(app.mau_count as i64, lang_id);
        let title_text = if !app.title.trim().is_empty() {
            app.title.clone()
        } else {
            app.bot_username.clone()
        };

        out.push(
            div()
                .id(SharedString::from(format!(
                    "{id_prefix}-card-{idx}-{}",
                    app.bot_username
                )))
                .cursor_pointer()
                .w_full()
                .px(px(10.0))
                .py(px(9.0))
                .bg(rgb(palette.dialogs_bg))
                .hover(move |s| s.bg(hover))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .flex()
                .items_center()
                .gap(px(10.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cb_row(this, app_row.clone(), cx);
                    }),
                )
                .child(
                    div()
                        .w(px(40.0))
                        .h(px(40.0))
                        .flex_shrink_0()
                        .rounded_full()
                        .bg(rgb(icon_col))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(13.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(if initials.is_empty() {
                            "MA".to_string()
                        } else {
                            initials
                        }),
                )
                .child(
                    div()
                        .flex_1()
                        .overflow_hidden()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(palette.text_primary))
                                        .truncate()
                                        .child(title_text),
                                )
                                .when(app.is_verified, |row| {
                                    row.child(tg_icon(IconName::Verified, 13.0, palette.accent))
                                }),
                        )
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(palette.text_secondary))
                                .truncate()
                                .child(mau_subtitle),
                        ),
                )
                .into_any_element(),
        );
    }
}

pub fn render_apps_catalog_section_with_recent<V: 'static>(
    query: &str,
    recent_apps: &[SidebarCatalogAppItem],
    apps: &[SidebarCatalogAppItem],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_open_botfather: impl Fn(&mut V, &mut Context<V>) + Clone + 'static,
    on_open_bot_profile: impl Fn(&mut V, SidebarCatalogAppItem, &mut Context<V>) + Clone + 'static,
) -> Vec<AnyElement> {
    let q_trim = query.trim().to_lowercase();
    let hover = rgb(palette.dialogs_hover_bg);
    let mut out: Vec<AnyElement> = Vec::new();

    let show_botfather = q_trim.is_empty()
        || "botfather".contains(&q_trim)
        || "机器人之父".contains(query.trim())
        || q_trim.contains("bot")
        || q_trim.contains("app")
        || query.trim().contains("机器人")
        || query.trim().contains("小程序");

    if show_botfather {
        let bf_open_row = on_open_botfather.clone();
        let bf_open_btn = on_open_botfather;
        out.push(
            div()
                .id("apps-catalog-botfather-card")
                .cursor_pointer()
                .w_full()
                .px(px(10.0))
                .py(px(9.0))
                .bg(rgb(palette.dialogs_bg))
                .hover(move |s| s.bg(hover))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .flex()
                .items_center()
                .gap(px(10.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                    bf_open_row(this, cx);
                }))
                .child(
                    div()
                        .w(px(40.0))
                        .h(px(40.0))
                        .flex_shrink_0()
                        .rounded_full()
                        .bg(rgb(palette.accent))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(18.0))
                        .child("🤖"),
                )
                .child(
                    div()
                        .flex_1()
                        .overflow_hidden()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(5.0))
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(palette.text_primary))
                                        .child(tr_str(lang_id, "BotFather (机器人之父)", "BotFather")),
                                )
                                .child(tg_icon(IconName::Verified, 13.0, palette.accent)),
                        )
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(palette.text_secondary))
                                .child(tr_str(
                                    lang_id,
                                    "@BotFather · 创建 AI 机器人与小程序",
                                    "@BotFather · Create Bots & Mini Apps",
                                )),
                        ),
                )
                .child(
                    div()
                        .id("apps-catalog-open-botfather-btn")
                        .cursor_pointer()
                        .flex_shrink_0()
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(999.0))
                        .bg(rgb(palette.accent))
                        .text_size(px(11.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xFFFFFF))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            bf_open_btn(this, cx);
                        }))
                        .child(tr_str(lang_id, "打开", "Open")),
                )
                .into_any_element(),
        );
    }

    let filtered_recent = filter_catalog_apps(recent_apps, &q_trim);
    let filtered_popular = filter_catalog_apps(apps, &q_trim);

    append_apps_catalog_group(
        &mut out,
        "apps-recent",
        tr_str(lang_id, "您用过的应用", "Apps You Used"),
        &filtered_recent,
        palette,
        lang_id,
        cx,
        on_open_bot_profile.clone(),
    );
    append_apps_catalog_group(
        &mut out,
        "apps-popular",
        tr_str(lang_id, "畅销应用", "Popular Apps"),
        &filtered_popular,
        palette,
        lang_id,
        cx,
        on_open_bot_profile,
    );

    out
}

pub fn render_apps_catalog_section<V: 'static>(
    query: &str,
    apps: &[SidebarCatalogAppItem],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_open_botfather: impl Fn(&mut V, &mut Context<V>) + Clone + 'static,
    _on_install_open_app: impl Fn(&mut V, SidebarCatalogAppItem, &mut Context<V>) + Clone + 'static,
    on_open_app_only: impl Fn(&mut V, SidebarCatalogAppItem, &mut Context<V>) + Clone + 'static,
) -> Vec<AnyElement> {
    render_apps_catalog_section_with_recent(
        query,
        &[],
        apps,
        palette,
        lang_id,
        cx,
        on_open_botfather,
        on_open_app_only,
    )
}

pub fn render_search_results_list<V: 'static>(
    query: &str,
    results: Vec<CategorizedSearchResult>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_select: impl Fn(&mut V, u64, &mut Context<V>) + Clone + 'static,
) -> Vec<AnyElement> {
    let has_apps = results.iter().any(|r| r.category.eq_ignore_ascii_case("Apps"));
    if (query.trim().is_empty() && !has_apps) || results.is_empty() {
        let empty = if query.trim().is_empty() {
            ChatSearchEmptyUi::new()
        } else {
            ChatSearchEmptyUi::for_no_results(query, true)
        };
        let subtitle = if empty.query_preview.is_empty() {
            tr_str(lang_id, "将显示在此处。", "Recent search results will appear here.").to_string()
        } else {
            format!("\"{}\"", empty.query_preview)
        };
        return vec![div()
            .id("sidebar-search-empty-state")
            .w_full()
            .py(px(32.0))
            .px(px(16.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(10.0))
            .child(tg_icon(IconName::Search, 36.0, palette.text_secondary))
            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(palette.text_primary)).child(if query.trim().is_empty() { tr_str(lang_id, "最近搜索结果", &empty.title).to_string() } else { tr_str(lang_id, "未找到匹配结果", &empty.title).to_string() }))
            .child(div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(subtitle))
            .into_any_element()];
    }

    let hover = rgb(palette.dialogs_hover_bg);
    results
        .into_iter()
        .map(|item| {
            let cid = item.chat_id;
            let cb = on_select.clone();
            let (t0, t1, t2) = split_highlight_segments(&item.title, query);
            let (s0, s1, s2) = split_highlight_segments(&item.subtitle, query);
            div()
                .cursor_pointer()
                .h(px(62.0))
                .flex_shrink_0()
                .w_full()
                .px(px(10.0))
                .py(px(8.0))
                .bg(rgb(palette.dialogs_bg))
                .hover(move |s| s.bg(hover))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .flex()
                .items_center()
                .gap(px(12.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, cid, cx);
                }))
                .child(div().w(px(42.0)).h(px(42.0)).flex_shrink_0().rounded_full().bg(rgb(item.avatar_color)).flex().items_center().justify_center().text_color(rgb(0xFFFFFF)).font_weight(FontWeight::BOLD).text_size(px(14.0)).child(item.avatar_initials))
                .child(
                    div().flex_1().h_full().overflow_hidden().flex().flex_col().justify_between()
                        .child(
                            div().w_full().flex().items_center().justify_between().gap(px(4.0))
                                .child(
                                    div().flex().items_center().gap(px(5.0)).overflow_hidden()
                                        .child(div().flex().items_center().text_size(px(13.5)).font_weight(FontWeight::BOLD).text_color(rgb(palette.text_primary)).child(t0).when(!t1.is_empty(), |el| el.child(div().text_color(rgb(palette.accent)).child(t1))).child(t2))
                                        .child(div().px(px(5.0)).py(px(1.0)).rounded(px(4.0)).bg(rgb(palette.composer_bg)).text_size(px(10.0)).text_color(rgb(palette.accent)).child(item.category)),
                                )
                                .child(div().text_size(px(11.0)).text_color(rgb(palette.msg_in_date_fg)).child(item.timestamp)),
                        )
                        .child(div().flex().items_center().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(s0).when(!s1.is_empty(), |el| el.child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgb(palette.accent)).child(s1))).child(s2)),
                )
                .into_any_element()
        })
        .collect()
}
