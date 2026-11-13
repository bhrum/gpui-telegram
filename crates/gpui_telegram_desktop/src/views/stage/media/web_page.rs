//! Rich WebPage, Instant View & Preview Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp:L56-L200`
//! - `Telegram/SourceFiles/history/view/media/history_view_game.cpp:L54-L100`
//! - `Telegram/SourceFiles/history/view/media/history_view_similar_channels.cpp:L79-L105`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_MAX_ORIGINAL_ENTRY_LINES: usize = 8192;
pub const K_FACTCHECK_COLLAPSED_LINES: usize = 3;
pub const K_STICKER_SET_LINES: usize = 3;
pub const K_FACTCHECK_ABOUT_DURATION_MS: u64 = 5_000;
pub const K_SPONSORED_USERPIC_LINES: usize = 2;
pub const K_LOG_ENTRY_PREVIEW_LINES: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebPageType {
    Article,
    Photo,
    Video,
    Document,
    Collage,
    Profile,
    InstantView,
    Theme,
    Story,
    Channel,
    Group,
    Bot,
}

/// Upstream ArticleThumbWidth from `history_view_web_page.cpp:L56-L61`
pub fn article_thumb_width(thumb_w: i32, thumb_h: i32, target_height: i32) -> i32 {
    if thumb_h <= 0 {
        1
    } else {
        let computed = (target_height * thumb_w) / thumb_h;
        computed.clamp(1, target_height)
    }
}

/// Upstream ArticleThumbHeight from `history_view_web_page.cpp:L63-L70`
pub fn article_thumb_height(thumb_w: i32, thumb_h: i32, target_width: i32) -> i32 {
    if thumb_w <= 0 {
        1
    } else {
        let computed = (thumb_h * target_width) / thumb_w;
        computed.max(1)
    }
}

/// Upstream PageToPhrase from `history_view_web_page.cpp:L162-L200`
pub fn page_to_phrase(page_type: WebPageType, has_iv: bool) -> &'static str {
    if has_iv {
        return "INSTANT VIEW";
    }
    match page_type {
        WebPageType::InstantView => "INSTANT VIEW",
        WebPageType::Theme => "VIEW THEME",
        WebPageType::Story => "VIEW STORY",
        WebPageType::Channel => "VIEW CHANNEL",
        WebPageType::Group => "VIEW GROUP",
        WebPageType::Bot => "OPEN BOT",
        WebPageType::Profile => "VIEW PROFILE",
        _ => "OPEN LINK",
    }
}

#[derive(Debug, Clone)]
pub struct FactcheckInfo {
    pub text: String,
    pub country: String,
    pub is_expanded: bool,
}

#[derive(Debug, Clone)]
pub struct RichWebPageCard {
    pub site_name: Option<String>,
    pub title: String,
    pub description: String,
    pub url: String,
    pub page_type: WebPageType,
    pub has_instant_view: bool,
    pub thumbnail_url: Option<String>,
    pub factcheck: Option<FactcheckInfo>,
}

impl RichWebPageCard {
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            site_name: None,
            title: title.into(),
            description: String::new(),
            url: url.into(),
            page_type: WebPageType::Article,
            has_instant_view: false,
            thumbnail_url: None,
            factcheck: None,
        }
    }

    pub fn with_site_name(mut self, name: impl Into<String>) -> Self {
        self.site_name = Some(name.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_instant_view(mut self, has_iv: bool) -> Self {
        self.has_instant_view = has_iv;
        self
    }

    pub fn render(self) -> impl IntoElement {
        let action_phrase = page_to_phrase(self.page_type, self.has_instant_view);
        let site_name = self.site_name;
        let title = self.title;
        let description = self.description;
        let factcheck = self.factcheck;
        let has_instant_view = self.has_instant_view;

        div()
            .flex()
            .flex_col()
            .w(px(380.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .border_l_4()
            .border_color(rgb(0x2AABEE))
            .p(px(12.0))
            .gap(px(6.0))
            // Site name
            .when_some(site_name, |el, site| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x2AABEE))
                        .child(site),
                )
            })
            // Title
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .line_height(px(18.0))
                    .child(title),
            )
            // Description
            .when(!description.is_empty(), |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x8A9AA8))
                        .line_height(px(16.0))
                        .child(description),
                )
            })
            // Factcheck Badge
            .when_some(factcheck, |el, fc| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .mt(px(4.0))
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x233140))
                        .border_1()
                        .border_color(rgb(0x40556B))
                        .gap(px(4.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xE55B5B))
                                .child(format!("⚠️ Fact check ({})", fc.country)),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(0xCCCCCC))
                                .child(fc.text),
                        ),
                )
            })
            // Button (Instant View / Open Link)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .mt(px(6.0))
                    .py(px(6.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .bg(if has_instant_view { rgb(0x2AABEE) } else { rgb(0x233140) })
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(action_phrase),
            )
    }
}
