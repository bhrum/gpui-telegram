//! Unified Theme-Aware GPUI Instant View Reader (`IvViewer`).

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::iv_controller::IvController;
use super::iv_data::{site_name_from_url, IvData};
use super::iv_rich_page::{to_roman_numeral, IvRichPage, OrderedMarkerType, RichBlock};
use super::iv_search_bar::IvSearchBar;
use super::iv_zoom_controls::{IvZoomControls, ZoomStepKind};

pub struct IvViewer {
    pub controller: IvController,
    pub page: IvRichPage,
    pub search_bar: IvSearchBar,
    pub zoom_controls: IvZoomControls,
    pub data: Option<IvData>,
    pub palette: TdesktopThemePalette,
    pub lang_id: String,
    pub should_close: bool,
    pub pending_share_url: Option<String>,
    pub copied_notice: bool,
    pub focus_handle: Option<FocusHandle>,
}

impl IvViewer {
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        let u = url.into();
        let t = title.into();
        let mut controller = IvController::new();
        controller.navigate_to(u.clone());
        controller.page_title = if t.is_empty() { site_name_from_url(&u) } else { t.clone() };
        controller.is_active = true;

        let mut page = IvRichPage::default();
        page.title = controller.page_title.clone();
        let data = Some(IvData::new(1, 0, u, false));

        Self {
            controller,
            page,
            search_bar: IvSearchBar::default(),
            zoom_controls: IvZoomControls::new(),
            data,
            palette: TdesktopThemePalette::default(),
            lang_id: "en".to_string(),
            should_close: false,
            pending_share_url: None,
            copied_notice: false,
            focus_handle: None,
        }
    }

    pub fn from_page(url: impl Into<String>, page: IvRichPage) -> Self {
        let u = url.into();
        let mut v = Self::new(u, page.title.clone());
        v.page = page;
        v
    }

    pub fn open_page(&mut self, url: impl Into<String>, title: impl Into<String>, blocks: Vec<RichBlock>) {
        let u = url.into();
        let t = title.into();
        self.controller.navigate_to(u.clone());
        self.controller.page_title = t.clone();
        self.page.title = t;
        self.page.blocks = blocks;
        self.data = Some(IvData::new(self.page.page_id, 0, u, false));
        self.recompute_search();
    }

    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        self.palette = palette;
        self.lang_id = lang_id.into();
    }

    fn recompute_search(&mut self) {
        let q = self.search_bar.query.trim().to_lowercase();
        if q.is_empty() {
            self.search_bar.total_matches = 0;
            self.search_bar.current_match = 0;
            return;
        }
        let hits = self
            .page
            .blocks
            .iter()
            .filter(|b| block_plain_text(b).to_lowercase().contains(&q))
            .count();
        self.search_bar.total_matches = hits;
        self.search_bar.current_match = if hits > 0 { 1 } else { 0 };
    }
}

fn block_plain_text(block: &RichBlock) -> String {
    match block {
        RichBlock::Heading { text, .. } | RichBlock::Paragraph { text } | RichBlock::Blockquote { text } | RichBlock::Code { text, .. } => text.clone(),
        RichBlock::List { items, .. } => items.join(" "),
        RichBlock::Table { headers, rows } => format!("{} {}", headers.join(" "), rows.iter().map(|r| r.join(" ")).collect::<Vec<_>>().join(" ")),
        RichBlock::Divider => String::new(),
    }
}

impl Render for IvViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_handle.is_none() {
            self.focus_handle = Some(cx.focus_handle());
        }
        let focus = self.focus_handle.clone().unwrap();
        let zoom = self.zoom_controls.zoom_factor;
        let q = self.search_bar.query.trim().to_lowercase();
        let pal = &self.palette;
        let hbg = pal.menu_hover_bg;

        div()
            .id("iv-viewer-overlay")
            .track_focus(&focus)
            .key_context("IvViewer")
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
                let key = ev.keystroke.key.as_str();
                if key == "escape" {
                    if this.search_bar.is_shown { this.search_bar.is_shown = false; } else { this.should_close = true; }
                } else if ev.keystroke.modifiers.platform && key == "f" {
                    this.search_bar.is_shown = true;
                } else if this.search_bar.is_shown {
                    if key == "backspace" { this.search_bar.query.pop(); }
                    else if let Some(ref ch) = ev.keystroke.key_char { this.search_bar.query.push_str(ch); }
                    this.recompute_search();
                }
                cx.notify();
            }))
            .absolute().inset_0().bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.72)).flex().items_center().justify_center().p(px(24.0))
            .child(
                div().w(px(820.0)).h(px(680.0)).rounded(px(14.0)).bg(rgb(pal.window_bg)).border_1().border_color(rgb(pal.border_fg)).shadow_lg().flex().flex_col().overflow_hidden()
                    .child(
                        div().h(px(52.0)).px(px(16.0)).bg(rgb(pal.topbar_bg)).border_b_1().border_color(rgb(pal.border_fg)).flex().items_center().justify_between()
                            .child(
                                div().flex().items_center().gap(px(8.0))
                                    .child(iv_btn("iv-back", IconName::BackArrow, pal.icon_fg, hbg, cx, |this, cx| { this.controller.go_back(); cx.notify(); }))
                                    .child(iv_btn("iv-fwd-nav", IconName::ChevronRight, pal.icon_fg, hbg, cx, |this, cx| { this.controller.go_forward(); cx.notify(); }))
                                    .child(div().flex().flex_col()
                                        .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(self.page.title.clone()))
                                        .child(div().text_size(px(11.5)).text_color(rgb(pal.text_secondary)).child(self.controller.current_url.clone()))),
                            )
                            .child(
                                div().flex().items_center().gap(px(6.0))
                                    .child(iv_btn("iv-search", IconName::Search, pal.icon_fg, hbg, cx, |this, cx| { this.search_bar.is_shown = !this.search_bar.is_shown; cx.notify(); }))
                                    .child(div().id("iv-zout").px(px(8.0)).py(px(4.0)).rounded(px(6.0)).cursor_pointer().hover(move |s| s.bg(rgb(hbg))).text_size(px(12.0)).text_color(rgb(pal.text_primary)).on_click(cx.listener(|t, _, _, cx| { t.zoom_controls.zoom_out(ZoomStepKind::Normal); cx.notify(); })).child("A−"))
                                    .child(div().text_size(px(11.5)).text_color(rgb(pal.text_secondary)).child(format!("{}%", self.zoom_controls.zoom_percentage())))
                                    .child(div().id("iv-zin").px(px(8.0)).py(px(4.0)).rounded(px(6.0)).cursor_pointer().hover(move |s| s.bg(rgb(hbg))).text_size(px(12.0)).text_color(rgb(pal.text_primary)).on_click(cx.listener(|t, _, _, cx| { t.zoom_controls.zoom_in(ZoomStepKind::Normal); cx.notify(); })).child("A+"))
                                    .child(iv_btn("iv-share", IconName::Forward, pal.icon_fg, hbg, cx, |t, cx| { t.pending_share_url = Some(t.controller.current_url.clone()); cx.notify(); }))
                                    .child(iv_btn("iv-close", IconName::Close, pal.icon_fg, hbg, cx, |t, cx| { t.should_close = true; cx.notify(); })),
                            ),
                    )
                    .when(self.search_bar.is_shown, |el| {
                        el.child(
                            div().h(px(38.0)).px(px(16.0)).bg(rgb(pal.compose_bg)).border_b_1().border_color(rgb(pal.border_fg)).flex().items_center().justify_between()
                                .child(div().text_size(px(12.5)).text_color(rgb(pal.text_primary)).child(if self.search_bar.query.is_empty() { tr_key_or(&self.lang_id, "lng_dlg_filter", "Type to search article...").to_string() } else { self.search_bar.query.clone() }))
                                .child(div().text_size(px(11.5)).text_color(rgb(pal.text_secondary)).child(format!("{} matches", self.search_bar.total_matches))),
                        )
                    })
                    .child(
                        div().id("iv-article-scroll").flex_1().overflow_y_scroll().p(px(28.0)).flex().justify_center().child(
                            div().w(px(620.0 * zoom)).flex().flex_col().gap(px(14.0))
                                .child(div().text_size(px(24.0 * zoom)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(self.page.title.clone()))
                                .when(!self.page.author.is_empty(), |el| el.child(div().text_size(px(12.5)).text_color(rgb(pal.text_secondary)).child(format!("{} • {}", self.page.author, self.page.date))))
                                .children(self.page.blocks.iter().enumerate().map(|(bi, blk)| render_iv_block(bi, blk, zoom, &q, pal, cx))),
                        ),
                    ),
            )
            .into_any_element()
    }
}

fn iv_btn<V: 'static>(id: &'static str, icon: IconName, color: u32, hover_bg: u32, cx: &mut Context<V>, on_click: impl Fn(&mut V, &mut Context<V>) + 'static) -> AnyElement {
    div().id(id).w(px(30.0)).h(px(30.0)).rounded_full().cursor_pointer().hover(move |s| s.bg(rgb(hover_bg))).flex().items_center().justify_center().on_click(cx.listener(move |t, _, _, cx| on_click(t, cx))).child(tg_icon(icon, 15.0, color)).into_any_element()
}

fn render_iv_block(idx: usize, blk: &RichBlock, z: f32, q: &str, pal: &TdesktopThemePalette, cx: &mut Context<IvViewer>) -> AnyElement {
    let hit = !q.is_empty() && block_plain_text(blk).to_lowercase().contains(q);
    let wrap = |el: AnyElement| if hit { div().rounded(px(4.0)).bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)).p(px(4.0)).child(el).into_any_element() } else { el };
    match blk {
        RichBlock::Heading { level, text, .. } => {
            let sz = (22.0 - ((*level as f32).min(4.0) * 2.0)) * z;
            wrap(div().text_size(px(sz)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(text.clone()).into_any_element())
        }
        RichBlock::Paragraph { text } => wrap(div().text_size(px(15.0 * z)).text_color(rgb(pal.text_primary)).child(text.clone()).into_any_element()),
        RichBlock::Blockquote { text } => wrap(div().pl(px(12.0)).py(px(6.0)).border_l_2().border_color(rgb(pal.reply_bar_fg)).bg(TdesktopThemePalette::rgba_with_alpha(pal.reply_bar_bg, 0.25)).text_size(px(14.0 * z)).italic().text_color(rgb(pal.text_primary)).child(text.clone()).into_any_element()),
        RichBlock::Code { lang, text } => {
            let code = text.clone();
            wrap(div().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.code_bg)).flex().flex_col().gap(px(6.0))
                .child(div().flex().items_center().justify_between().child(div().text_size(px(11.0)).text_color(rgb(pal.text_secondary)).child(lang.clone().unwrap_or_else(|| "code".into())))
                    .child(div().id(SharedString::from(format!("iv-cp-{}", idx))).cursor_pointer().text_size(px(11.0)).text_color(rgb(pal.accent)).on_click(cx.listener(move |t, _, _, cx| { cx.write_to_clipboard(ClipboardItem::new_string(code.clone())); t.copied_notice = true; cx.notify(); })).child("Copy")))
                .child(div().text_size(px(13.0 * z)).text_color(rgb(pal.code_fg)).child(text.clone())).into_any_element())
        }
        RichBlock::List { marker, items } => wrap(div().flex().flex_col().gap(px(4.0)).children(items.iter().enumerate().map(|(i, it)| {
            let m = match marker {
                OrderedMarkerType::Numbers => format!("{}.", i + 1),
                OrderedMarkerType::RomanLower => format!("{}.", to_roman_numeral(i + 1, false)),
                OrderedMarkerType::RomanUpper => format!("{}.", to_roman_numeral(i + 1, true)),
                OrderedMarkerType::LettersLower => format!("{}.", (b'a' + (i % 26) as u8) as char),
                OrderedMarkerType::LettersUpper => format!("{}.", (b'A' + (i % 26) as u8) as char),
            };
            div().flex().gap(px(8.0)).text_size(px(14.5 * z)).text_color(rgb(pal.text_primary)).child(div().text_color(rgb(pal.accent)).child(m)).child(it.clone())
        })).into_any_element()),
        RichBlock::Table { headers, rows } => wrap(div().border_1().border_color(rgb(pal.border_fg)).rounded(px(6.0)).flex().flex_col()
            .child(div().flex().bg(rgb(pal.card_bg)).p(px(6.0)).children(headers.iter().map(|h| div().flex_1().text_size(px(12.5)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(h.clone()))))
            .children(rows.iter().map(|r| div().flex().p(px(6.0)).border_t_1().border_color(rgb(pal.border_fg)).children(r.iter().map(|c| div().flex_1().text_size(px(12.5)).text_color(rgb(pal.text_secondary)).child(c.clone()))))).into_any_element()),
        RichBlock::Divider => div().h(px(1.0)).w_full().bg(rgb(pal.border_fg)).into_any_element(),
    }
}
