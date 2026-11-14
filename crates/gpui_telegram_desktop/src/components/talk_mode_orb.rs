use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::*;

pub struct TalkModeOrb {
    pub energy_level: f32,
    pub is_active: bool,
    pub is_muted: bool,
    pub palette: TdesktopThemePalette,
}

impl TalkModeOrb {
    pub fn new() -> Self {
        Self {
            energy_level: 0.0,
            is_active: false,
            is_muted: false,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn set_energy(&mut self, energy_level: f32, is_active: bool) {
        self.energy_level = energy_level.clamp(0.0, 1.0);
        self.is_active = is_active;
    }
}

impl Default for TalkModeOrb {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for TalkModeOrb {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let pulse_scale = if self.is_active && !self.is_muted {
            160.0 + self.energy_level * 28.0
        } else {
            160.0
        };
        let core_color = if self.is_muted {
            pal.danger
        } else {
            pal.accent
        };

        div()
            .w(px(196.0))
            .h(px(196.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(pulse_scale))
                    .h(px(pulse_scale))
                    .rounded_full()
                    .bg(TdesktopThemePalette::rgba_with_alpha(core_color, 0.22))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(px(136.0))
                            .h(px(136.0))
                            .rounded_full()
                            .bg(TdesktopThemePalette::rgba_with_alpha(core_color, 0.40))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .w(px(108.0))
                                    .h(px(108.0))
                                    .rounded_full()
                                    .bg(rgb(core_color))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(tg_icon(
                                        if self.is_muted {
                                            IconName::Mute
                                        } else {
                                            IconName::Mic
                                        },
                                        34.0,
                                        pal.active_button_fg,
                                    )),
                            ),
                    ),
            )
    }
}
