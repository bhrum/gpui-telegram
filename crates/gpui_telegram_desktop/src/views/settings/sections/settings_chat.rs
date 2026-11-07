// Source: Telegram/SourceFiles/settings/sections/settings_chat.cpp, Telegram/SourceFiles/settings/sections/settings_chat.h
// Subsystem: settings | Feature ID: TCG-N11
// Upstream: SetupThemeOptions, SetupThemeSettings, ColorsPalette::editButton, Theme::EmbeddedThemes

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatThemePreset {
    Classic,
    Day,
    Tinted,
    Night,
}

#[derive(Debug, Clone)]
pub struct SettingsChatSections {
    pub font_size: u32,
    pub theme_preset: ChatThemePreset,
    pub accent_color: u32,
    pub wallpaper_name: String,
    pub blur_wallpaper: bool,
    pub send_on_enter: bool,
    pub suggest_stickers_by_emoji: bool,
    pub loop_animated_stickers: bool,
}

impl SettingsChatSections {
    pub fn new() -> Self {
        Self {
            font_size: 13,
            theme_preset: ChatThemePreset::Day,
            accent_color: 0x2481cc,
            wallpaper_name: "Default".into(),
            blur_wallpaper: false,
            send_on_enter: true,
            suggest_stickers_by_emoji: true,
            loop_animated_stickers: true,
        }
    }

    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size.clamp(12, 30);
    }

    pub fn set_theme_preset(&mut self, preset: ChatThemePreset) {
        self.theme_preset = preset;
    }

    pub fn set_accent_color(&mut self, color: u32) {
        self.accent_color = color;
    }

    pub fn toggle_send_on_enter(&mut self) {
        self.send_on_enter = !self.send_on_enter;
    }

    pub fn toggle_blur_wallpaper(&mut self) {
        self.blur_wallpaper = !self.blur_wallpaper;
    }

    pub fn theme_name(&self) -> &'static str {
        match self.theme_preset {
            ChatThemePreset::Classic => "Classic",
            ChatThemePreset::Day => "Day",
            ChatThemePreset::Tinted => "Tinted",
            ChatThemePreset::Night => "Night",
        }
    }
}

impl Default for SettingsChatSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsChatSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bubble_bg = match self.theme_preset {
            ChatThemePreset::Night => rgb(0x2b5278),
            _ => rgb(0xeffdde),
        };
        let preview_bg = match self.theme_preset {
            ChatThemePreset::Night => rgb(0x18222d),
            _ => rgb(0xe6eef5),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap(px(16.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_xl()
                    .child("Chat Settings")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(16.0))
                    .rounded(px(10.0))
                    .bg(preview_bg)
                    .child(
                        div()
                            .flex()
                            .justify_start()
                            .child(
                                div()
                                    .p(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0xffffff))
                                    .text_size(px(self.font_size as f32))
                                    .child("Good morning! How is the new update?")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .p(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(bubble_bg)
                                    .text_size(px(self.font_size as f32))
                                    .child("It looks stunning and works smoothly! 🚀")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Message Text Size"))
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(format!("{} pt", self.font_size)))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Color Theme"))
                            .child(div().text_color(rgb(0x2481cc)).child(self.theme_name()))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Chat Wallpaper"))
                            .child(div().text_color(rgb(0x2481cc)).child(self.wallpaper_name.clone()))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Send by Enter"))
                            .child(div().text_color(if self.send_on_enter { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.send_on_enter { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Suggest Stickers by Emoji"))
                            .child(div().text_color(if self.suggest_stickers_by_emoji { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.suggest_stickers_by_emoji { "On" } else { "Off" }))
                    )
            )
    }
}
