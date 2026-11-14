use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
    settings_toggle_row,
};

pub const K_FORCE_DISABLE_TOOLTIP_DURATION_MS: u64 = 3_000;

pub struct SettingsPowerSaving {
    pub battery_saver_enabled: bool,
    pub battery_threshold_percent: u32,
    pub animate_stickers_in_chat: bool,
    pub animate_stickers_in_panel: bool,
    pub animate_emoji_in_panel: bool,
    pub autoplay_gifs: bool,
    pub autoplay_videos: bool,
    pub has_divider: bool,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsPowerSaving {
    pub fn new() -> Self {
        Self {
            battery_saver_enabled: false,
            battery_threshold_percent: 20,
            animate_stickers_in_chat: true,
            animate_stickers_in_panel: true,
            animate_emoji_in_panel: true,
            autoplay_gifs: true,
            autoplay_videos: true,
            has_divider: false,
            is_visible: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.palette = palette;
        s.language_id = language_id.into();
        s.is_visible = true;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn margins(&self) -> (f32, f32, f32, f32) { (22.0, 8.0, 22.0, 8.0) }
    pub fn lng_settings_power_stickers_panel(&self) -> &'static str { "Animate stickers in panel" }
    pub fn add_divider(&mut self) { self.has_divider = true; }
    pub fn lng_settings_power_emoji_panel(&self) -> &'static str { "Animate emoji in panel" }
    pub fn app(&self) -> &'static str { "Telegram Desktop" }
    pub fn lng_settings_power_auto(&self) -> String { format!("Enable when battery drops below {}%", self.battery_threshold_percent) }
}

impl Default for SettingsPowerSaving {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsPowerSaving {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();

        settings_container(pal.clone())
            .child(settings_section_header(IconName::NightMoon, tr_str(&lang, "省电模式", "Power Saving"), Some(&self.lng_settings_power_auto()), pal.clone()))
            .child(settings_card(pal.clone()).child(settings_toggle_row(IconName::NightMoon, tr_str(&lang, "启用省电模式", "Enable Power Saving Mode"), Some(&self.lng_settings_power_auto()), self.battery_saver_enabled, pal.clone(), |this| this.battery_saver_enabled = !this.battery_saver_enabled, cx)))
            .child(settings_subsection_title(tr_str(&lang, "电量阈值", "Battery Threshold"), pal.clone()))
            .child(div().flex().flex_row().gap(px(8.0)).children([10_u32, 20, 30, 50].into_iter().map(|pct| {
                let active = self.battery_threshold_percent == pct;
                div().flex_1().py(px(8.0)).rounded(px(8.0)).flex().items_center().justify_center().bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.battery_threshold_percent = pct; cx.notify(); })).child(format!("{}%", pct))
            })))
            .child(settings_subsection_title(tr_str(&lang, "动画与自动播放", "Animations & Autoplay"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(settings_toggle_row(IconName::Sticker, "Animate stickers in chat", None, self.animate_stickers_in_chat, pal.clone(), |this| this.animate_stickers_in_chat = !this.animate_stickers_in_chat, cx))
                    .child(settings_toggle_row(IconName::Sticker, self.lng_settings_power_stickers_panel(), None, self.animate_stickers_in_panel, pal.clone(), |this| this.animate_stickers_in_panel = !this.animate_stickers_in_panel, cx))
                    .child(settings_toggle_row(IconName::SmileEmoji, self.lng_settings_power_emoji_panel(), None, self.animate_emoji_in_panel, pal.clone(), |this| this.animate_emoji_in_panel = !this.animate_emoji_in_panel, cx))
                    .child(settings_toggle_row(IconName::Gif, "Autoplay GIFs", None, self.autoplay_gifs, pal.clone(), |this| this.autoplay_gifs = !this.autoplay_gifs, cx))
                    .child(settings_toggle_row(IconName::VideoCall, "Autoplay Videos", None, self.autoplay_videos, pal, |this| this.autoplay_videos = !this.autoplay_videos, cx)),
            )
    }
}
