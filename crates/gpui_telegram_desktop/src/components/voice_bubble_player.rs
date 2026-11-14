use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::*;
use std::sync::Arc;

pub struct VoiceBubblePlayer {
    pub duration_secs: u32,
    pub is_playing: bool,
    pub progress: f32,
    pub waveform: Vec<u8>,
    pub palette: TdesktopThemePalette,
    pub on_toggle_play: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl VoiceBubblePlayer {
    pub fn new(duration_secs: u32) -> Self {
        Self {
            duration_secs,
            is_playing: false,
            progress: 0.0,
            waveform: vec![
                6, 12, 18, 10, 22, 16, 8, 14, 24, 20, 12, 18, 26, 15, 9, 14, 20, 17, 11, 7,
            ],
            palette: TdesktopThemePalette::default(),
            on_toggle_play: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
        if let Some(ref cb) = self.on_toggle_play {
            cb(self.is_playing);
        }
    }
}

impl Render for VoiceBubblePlayer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let bar_count = self.waveform.len().max(1);
        let filled_bars = ((self.progress.clamp(0.0, 1.0)) * bar_count as f32).round() as usize;
        let icon = if self.is_playing {
            IconName::Pause
        } else {
            IconName::Play
        };

        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .p(px(8.0))
            .child(
                div()
                    .cursor_pointer()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(pal.accent))
                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.toggle_play();
                            cx.notify();
                        }),
                    )
                    .child(tg_icon(icon, 16.0, pal.active_button_fg)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .flex()
                            .items_end()
                            .gap(px(2.0))
                            .h(px(22.0))
                            .children(self.waveform.iter().enumerate().map(|(idx, amp)| {
                                let is_active = idx < filled_bars;
                                let h = ((*amp as f32).clamp(4.0, 26.0) / 26.0) * 20.0;
                                div()
                                    .w(px(3.0))
                                    .h(px(h.max(3.0)))
                                    .rounded_full()
                                    .bg(if is_active {
                                        rgb(pal.accent)
                                    } else {
                                        TdesktopThemePalette::rgba_with_alpha(pal.text_secondary, 0.45)
                                    })
                            })),
                    )
                    .child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!(
                                "{}:{:02}",
                                self.duration_secs / 60,
                                self.duration_secs % 60
                            )),
                    ),
            )
    }
}
