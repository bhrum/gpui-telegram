use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
};

#[derive(Debug, Clone)]
pub struct PremiumTier {
    pub months: u32,
    pub title: &'static str,
    pub price_per_month: f32,
    pub total_price: f32,
    pub discount_pct: u32,
}

pub struct SettingsPremiumSections {
    pub is_premium_active: bool,
    pub selected_months: u32,
    pub tiers: Vec<PremiumTier>,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_subscribe_months: Option<u32>,
}

impl SettingsPremiumSections {
    pub fn new() -> Self {
        Self {
            is_premium_active: true,
            selected_months: 12,
            tiers: vec![
                PremiumTier { months: 12, title: "Annual", price_per_month: 2.99, total_price: 35.88, discount_pct: 40 },
                PremiumTier { months: 6, title: "Semi-Annual", price_per_month: 3.99, total_price: 23.94, discount_pct: 20 },
                PremiumTier { months: 1, title: "Monthly", price_per_month: 4.99, total_price: 4.99, discount_pct: 0 },
            ],
            is_visible: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_subscribe_months: None,
        }
    }

    pub fn new_live(is_premium_active: bool, palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.is_premium_active = is_premium_active;
        s.palette = palette;
        s.language_id = language_id.into();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn select_tier(&mut self, months: u32) { self.selected_months = months; }

    pub fn subscribe(&mut self, months: u32) {
        self.selected_months = months;
        self.is_premium_active = true;
        self.pending_subscribe_months = Some(months);
    }

    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
}

impl Default for SettingsPremiumSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsPremiumSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let selected = self.selected_months;
        let is_active = self.is_premium_active;
        let premium_accent = 0x9b51e0_u32;

        let perks = [
            (IconName::Paperclip, "4 GB Uploads", "Send documents and media up to 4 GB each"),
            (IconName::SendPlane, "Faster Download Speed", "Priority media and file CDN delivery"),
            (IconName::Mic, "Voice-to-Text Transcription", "Instant transcripts for voice and video notes"),
            (IconName::PremiumStar, "Infinite Custom Reactions", "Animated emoji status and extended reactions"),
            (IconName::FolderAll, "Advanced Chat Management", "Default folders and unlimited Saved Messages tags"),
        ];

        settings_container(pal.clone())
            .child(settings_section_header(
                IconName::PremiumStar,
                tr_str(&lang, "Telegram Premium 会员", "Telegram Premium"),
                Some(if is_active { "Active Subscriber" } else { "Subscribe to unlock exclusive features" }),
                pal.clone(),
            ))
            .child(div().flex().flex_row().gap(px(8.0)).children(self.tiers.iter().map(|tier| {
                let months = tier.months;
                let is_sel = months == selected;
                div().flex_1().p(px(12.0)).rounded(px(10.0))
                    .bg(rgb(if is_sel { pal.dialogs_hover_bg } else { pal.card_bg }))
                    .border_1().border_color(rgb(if is_sel { premium_accent } else { pal.border_fg })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.select_tier(months); cx.notify(); }))
                    .child(div().flex().flex_col().gap(px(4.0))
                        .child(div().flex().flex_row().items_center().justify_between()
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(tier.title))
                            .when(tier.discount_pct > 0, |el| el.child(div().px(px(5.0)).py(px(1.0)).rounded(px(4.0)).bg(rgb(pal.success)).text_xs().text_color(rgb(pal.active_button_fg)).child(format!("-{}%", tier.discount_pct)))))
                        .child(div().text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(premium_accent)).child(format!("${:.2}/mo", tier.price_per_month)))
                        .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(format!("${:.2} total", tier.total_price))))
            })))
            .child(settings_subsection_title(tr_str(&lang, "专属特权", "Included Features"), pal.clone()))
            .child(settings_card(pal.clone()).children(perks.into_iter().map(|(icon, title, desc)| {
                div().flex().flex_row().items_center().gap(px(10.0)).px(px(10.0)).py(px(6.0))
                    .child(tg_icon(icon, 16.0, premium_accent))
                    .child(div().flex().flex_col().child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(pal.text_primary)).child(title)).child(div().text_xs().text_color(rgb(pal.text_secondary)).child(desc)))
            })))
            .child(
                div().w_full().py(px(10.0)).rounded(px(8.0)).bg(rgb(premium_accent)).flex().items_center().justify_center()
                    .text_sm().font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg)).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.subscribe(selected); cx.notify(); }))
                    .child(if is_active { format!("Switch Plan ({} mo)", selected) } else { format!("Subscribe ({} mo)", selected) }),
            )
            .into_any_element()
    }
}
