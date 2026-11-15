use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{
    ACCENT_SWATCHES, TdesktopThemeMode, TdesktopThemePalette, WALLPAPER_PRESETS,
};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
    settings_toggle_row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatThemePreset {
    Classic,
    Day,
    Tinted,
    Night,
}

impl ChatThemePreset {
    pub fn to_mode(self) -> TdesktopThemeMode {
        match self {
            Self::Classic => TdesktopThemeMode::Classic,
            Self::Day => TdesktopThemeMode::Day,
            Self::Tinted => TdesktopThemeMode::Tinted,
            Self::Night => TdesktopThemeMode::Night,
        }
    }
}

pub struct SettingsChatSections {
    pub font_size: u32,
    pub bubble_radius: u32,
    pub theme_preset: ChatThemePreset,
    pub accent_color: u32,
    pub wallpaper_name: String,
    pub blur_wallpaper: bool,
    pub send_on_enter: bool,
    pub suggest_stickers_by_emoji: bool,
    pub loop_animated_stickers: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    theme_dirty: bool,
    chat_prefs_dirty: bool,
}

impl SettingsChatSections {
    pub fn new() -> Self {
        let palette = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12);
        Self {
            font_size: 13,
            bubble_radius: 12,
            theme_preset: ChatThemePreset::Day,
            accent_color: 0x3390ec,
            wallpaper_name: WALLPAPER_PRESETS[0].id.to_string(),
            blur_wallpaper: false,
            send_on_enter: true,
            suggest_stickers_by_emoji: true,
            loop_animated_stickers: true,
            palette,
            language_id: "en".to_string(),
            theme_dirty: false,
            chat_prefs_dirty: false,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let theme_preset = match palette.mode {
            TdesktopThemeMode::Classic => ChatThemePreset::Classic,
            TdesktopThemeMode::Day => ChatThemePreset::Day,
            TdesktopThemeMode::Tinted => ChatThemePreset::Tinted,
            TdesktopThemeMode::Night => ChatThemePreset::Night,
        };
        Self {
            font_size: palette.font_size_px,
            bubble_radius: palette.bubble_radius_px,
            theme_preset,
            accent_color: palette.accent,
            wallpaper_name: WALLPAPER_PRESETS[0].id.to_string(),
            blur_wallpaper: palette.blur_wallpaper,
            send_on_enter: true,
            suggest_stickers_by_emoji: true,
            loop_animated_stickers: true,
            palette,
            language_id: language_id.to_string(),
            theme_dirty: false,
            chat_prefs_dirty: false,
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.theme_preset = match palette.mode {
            TdesktopThemeMode::Classic => ChatThemePreset::Classic,
            TdesktopThemeMode::Day => ChatThemePreset::Day,
            TdesktopThemeMode::Tinted => ChatThemePreset::Tinted,
            TdesktopThemeMode::Night => ChatThemePreset::Night,
        };
        self.accent_color = palette.accent;
        self.font_size = palette.font_size_px;
        self.bubble_radius = palette.bubble_radius_px;
        self.blur_wallpaper = palette.blur_wallpaper;
        self.palette = palette;
    }

    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn set_font_size(&mut self, size: u32) { self.font_size = size.clamp(12, 30); self.rebuild_palette(); self.chat_prefs_dirty = true; }
    pub fn set_bubble_radius(&mut self, radius: u32) { self.bubble_radius = radius.clamp(2, 18); self.rebuild_palette(); self.chat_prefs_dirty = true; }
    pub fn set_theme_preset(&mut self, preset: ChatThemePreset) { self.theme_preset = preset; self.rebuild_palette(); self.theme_dirty = true; }
    pub fn set_accent_color(&mut self, color: u32) { self.accent_color = color; self.rebuild_palette(); self.theme_dirty = true; }
    pub fn set_wallpaper(&mut self, id: &str) { self.wallpaper_name = id.to_string(); self.palette = self.palette.clone().with_wallpaper(id); self.theme_dirty = true; }
    pub fn toggle_send_on_enter(&mut self) { self.send_on_enter = !self.send_on_enter; self.chat_prefs_dirty = true; }
    pub fn toggle_loop_animated_stickers(&mut self) { self.loop_animated_stickers = !self.loop_animated_stickers; self.chat_prefs_dirty = true; }
    pub fn toggle_blur_wallpaper(&mut self) { self.blur_wallpaper = !self.blur_wallpaper; self.rebuild_palette(); self.chat_prefs_dirty = true; }

    fn rebuild_palette(&mut self) {
        let ui_scale_pct = self.palette.ui_scale_pct;
        self.palette = TdesktopThemePalette::build(self.theme_preset.to_mode(), self.accent_color, self.font_size.clamp(12, 20), self.bubble_radius.clamp(2, 18))
            .with_wallpaper(&self.wallpaper_name)
            .with_ui_scale(ui_scale_pct)
            .with_blur_wallpaper(self.blur_wallpaper);
    }

    pub fn take_theme_change(&mut self) -> Option<(TdesktopThemeMode, u32, String)> {
        if self.theme_dirty { self.theme_dirty = false; Some((self.theme_preset.to_mode(), self.accent_color, self.wallpaper_name.clone())) } else { None }
    }

    pub fn take_chat_prefs_change(&mut self) -> Option<(u32, u32, bool, bool, bool, bool)> {
        if self.chat_prefs_dirty {
            self.chat_prefs_dirty = false;
            Some((
                self.font_size,
                self.bubble_radius,
                self.send_on_enter,
                self.suggest_stickers_by_emoji,
                self.loop_animated_stickers,
                self.blur_wallpaper,
            ))
        } else {
            None
        }
    }

    pub fn theme_name(&self) -> &'static str { self.theme_preset.to_mode().as_name() }
    pub fn theme_name_localized(&self, lang: &str) -> &'static str {
        match self.theme_preset {
            ChatThemePreset::Night => tr_str(lang, "夜间", "Night"),
            ChatThemePreset::Tinted => tr_str(lang, "暗调", "Tinted"),
            ChatThemePreset::Day => tr_str(lang, "日间", "Day"),
            ChatThemePreset::Classic => tr_str(lang, "经典", "Classic"),
        }
    }
}

impl Default for SettingsChatSections {
    fn default() -> Self { Self::new() }
}

fn wallpaper_label_ink(bg_hex: u32) -> u32 {
    let linear = |channel: u32| -> f32 {
        let s = (channel & 0xFF) as f32 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    let lum = 0.2126 * linear(bg_hex >> 16)
        + 0.7152 * linear(bg_hex >> 8)
        + 0.0722 * linear(bg_hex);
    if lum > 0.179 {
        0x18222D
    } else {
        0xFFFFFF
    }
}

impl Render for SettingsChatSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let rad = self.bubble_radius as f32;
        let presets = [ChatThemePreset::Night, ChatThemePreset::Tinted, ChatThemePreset::Day, ChatThemePreset::Classic];
        settings_container(pal.clone())
            .child(settings_section_header(IconName::NightMoon, tr_str(&lang, "聊天设置", "Chat Settings"), Some(&format!("{} pt · {}", self.font_size, self.theme_name_localized(&lang))), pal.clone()))
            .child(
                div().flex().flex_col().gap(px(8.0)).p(px(14.0)).rounded(px(10.0)).bg(rgb(pal.wallpaper_top)).border_1().border_color(rgb(pal.border_fg))
                    .child(div().flex().justify_start().child(div().px(px(12.0)).py(px(8.0)).rounded(px(pal.scaled_px(rad))).bg(rgb(pal.msg_in_bg)).text_color(rgb(pal.text_primary)).text_size(px(pal.scaled_px(self.font_size as f32))).child(tr_str(&lang, "早上好！新版聊天界面体验怎么样？", "Good morning! How is the new update?"))))
                    .child(div().flex().justify_end().child(div().px(px(12.0)).py(px(8.0)).rounded(px(pal.scaled_px(rad))).bg(rgb(pal.msg_out_bg)).text_color(rgb(pal.text_primary)).text_size(px(pal.scaled_px(self.font_size as f32))).child(tr_str(&lang, "原生丝滑，在任何主题下都非常清晰。", "Clean, native, and readable in every theme.")))),
            )
            .child(settings_subsection_title(tr_str(&lang, "颜色主题与强调色", "Color Theme & Accent"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(div().flex().flex_row().gap(px(8.0)).children(presets.into_iter().map(|preset| {
                        let active = preset == self.theme_preset;
                        let preset_label = match preset {
                            ChatThemePreset::Night => tr_str(&lang, "夜间", "Night"),
                            ChatThemePreset::Tinted => tr_str(&lang, "暗调", "Tinted"),
                            ChatThemePreset::Day => tr_str(&lang, "日间", "Day"),
                            ChatThemePreset::Classic => tr_str(&lang, "经典", "Classic"),
                        };
                        div().flex_1().py(px(8.0)).rounded(px(8.0)).flex().items_center().justify_center().bg(rgb(if active { pal.accent } else { pal.input_bg })).text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.set_theme_preset(preset); cx.notify(); })).child(preset_label)
                    })))
                    .child(div().pt(px(6.0)).flex().flex_row().items_center().gap(px(8.0)).children(ACCENT_SWATCHES.iter().map(|sw| {
                        let sel = sw.hex == self.accent_color;
                        div().size(px(24.0)).rounded_full().bg(rgb(sw.hex)).border_2().border_color(rgb(if sel { pal.text_primary } else { pal.card_bg })).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener({ let hex = sw.hex; move |this, _, _, cx| { this.set_accent_color(hex); cx.notify(); } }))
                    }))),
            )
            .child(settings_subsection_title(tr_str(&lang, "聊天背景与字体", "Wallpaper & Typography"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(div().flex().flex_row().flex_wrap().gap(px(6.0)).children(WALLPAPER_PRESETS.iter().map(|wp| {
                        let sel = wp.id == self.wallpaper_name || wp.name == self.wallpaper_name;
                        let label_ink = wallpaper_label_ink(wp.top_hex);
                        div().px(px(10.0)).py(px(5.0)).rounded(px(6.0)).bg(rgb(wp.top_hex)).border_1().border_color(rgb(if sel { pal.accent } else { pal.border_fg })).text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(label_ink)).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener({ let id = wp.id; move |this, _, _, cx| { this.set_wallpaper(id); cx.notify(); } })).child(wp.name)
                    })))
                    .child(div().pt(px(6.0)).flex().flex_row().items_center().justify_between().child(div().text_sm().text_color(rgb(pal.text_primary)).child(format!("{}: {} pt", tr_str(&lang, "字体大小", "Text Size"), self.font_size))).child(div().flex().flex_row().gap(px(6.0))
                        .child(div().px(px(10.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.input_bg)).text_color(rgb(pal.text_primary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.set_font_size(this.font_size.saturating_sub(1)); cx.notify(); })).child("-"))
                        .child(div().px(px(10.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.input_bg)).text_color(rgb(pal.text_primary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.set_font_size(this.font_size + 1); cx.notify(); })).child("+"))))
                    .child(div().pt(px(4.0)).flex().flex_row().items_center().justify_between().child(div().text_sm().text_color(rgb(pal.text_primary)).child(format!("{}: {} px", tr_str(&lang, "气泡圆角", "Bubble Corners"), self.bubble_radius))).child(div().flex().flex_row().gap(px(6.0))
                        .child(div().px(px(10.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.input_bg)).text_color(rgb(pal.text_primary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.set_bubble_radius(this.bubble_radius.saturating_sub(2).max(2)); cx.notify(); })).child("-"))
                        .child(div().px(px(10.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.input_bg)).text_color(rgb(pal.text_primary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.set_bubble_radius((this.bubble_radius + 2).min(18)); cx.notify(); })).child("+")))),
            )
            .child(settings_card(pal.clone())
                .child(settings_toggle_row(IconName::SendPlane, tr_str(&lang, "回车发送", "Send by Enter"), None, self.send_on_enter, pal.clone(), |this| this.toggle_send_on_enter(), cx))
                .child(settings_toggle_row(IconName::Sticker, tr_str(&lang, "按表情推荐贴纸", "Suggest Stickers by Emoji"), None, self.suggest_stickers_by_emoji, pal.clone(), |this| { this.suggest_stickers_by_emoji = !this.suggest_stickers_by_emoji; this.chat_prefs_dirty = true; }, cx))
                .child(settings_toggle_row(IconName::Sticker, tr_str(&lang, "循环播放动态贴纸", "Loop Animated Stickers"), None, self.loop_animated_stickers, pal.clone(), |this| this.toggle_loop_animated_stickers(), cx))
                .child(settings_toggle_row(IconName::SunDay, tr_str(&lang, "模糊背景", "Blur Wallpaper"), None, self.blur_wallpaper, pal, |this| this.toggle_blur_wallpaper(), cx))
                .when(false, |el| el.child(tg_icon(IconName::SunDay, 12.0, 0))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[std::prelude::v1::test]
    fn test_wallpaper_label_ink_matches_preset_brightness() {
        for wp in WALLPAPER_PRESETS {
            let expected = if wp.is_dark { 0xFFFFFF } else { 0x18222D };
            assert_eq!(
                wallpaper_label_ink(wp.top_hex),
                expected,
                "unexpected contrast ink for wallpaper {} (0x{:06X})",
                wp.name,
                wp.top_hex
            );
        }
    }
}
