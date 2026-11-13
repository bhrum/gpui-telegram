use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::MessageItemModel;
use super::text_card::{render_bubble_tail, BubbleRounding, MSG_MAX_WIDTH, MSG_MIN_WIDTH};

pub struct VoiceMessageView;

impl VoiceMessageView {
    pub fn render_default(
        message: &MessageItemModel,
        duration_secs: u32,
        waveform_5bit: &[u8],
        playback_position_secs: f32,
        is_playing: bool,
        playback_speed: f32,
    ) -> impl IntoElement {
        Self::render(
            message,
            duration_secs,
            waveform_5bit,
            playback_position_secs,
            is_playing,
            playback_speed,
            BubbleRounding::default(),
        )
    }

    pub fn render(
        message: &MessageItemModel,
        duration_secs: u32,
        waveform_5bit: &[u8],
        playback_position_secs: f32,
        is_playing: bool,
        playback_speed: f32,
        rounding: BubbleRounding,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        // Progress ratio in [0.0, 1.0]
        let total_dur = (duration_secs as f32).max(1.0);
        let progress_ratio = (playback_position_secs / total_dur).clamp(0.0, 1.0);
        let current_sec = playback_position_secs as u32;
        let played_bars_count = ((waveform_5bit.len() as f32) * progress_ratio).round() as usize;

        div()
            .relative()
            .flex()
            .items_center()
            .gap(px(12.0))
            .min_w(px(MSG_MIN_WIDTH))
            .max_w(px(MSG_MAX_WIDTH))
            .w(px(320.0))
            .rounded_tl(px(rounding.top_left))
            .rounded_tr(px(rounding.top_right))
            .rounded_bl(px(rounding.bottom_left))
            .rounded_br(px(rounding.bottom_right))
            .bg(bg_color)
            .pl(px(11.0))
            .pr(px(11.0))
            .pt(px(8.0))
            .pb(px(8.0))
            // 1. Play / Pause Circular Button (36px)
            .child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(14.0))
                    .child(if is_playing { "❚❚" } else { "▶" }),
            )
            // 2. Waveform + Time metadata
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    // Waveform Bars Container (height 24px)
                    .child(
                        div()
                            .h(px(24.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .gap(px(1.5))
                            .children(waveform_5bit.iter().enumerate().map(|(idx, &sample)| {
                                // 5-bit sample range: 0..31
                                let sample_val = (sample & 0x1F) as f32;
                                let bar_height = (2.0 + (sample_val / 31.0) * 20.0).clamp(2.0, 22.0);
                                let is_played = idx < played_bars_count;
                                let bar_color = if is_played {
                                    rgb(0x2AABEE) // Played accent
                                } else {
                                    rgb(0x6B808D) // Unplayed muted
                                };

                                div()
                                    .w(px(2.0))
                                    .h(px(bar_height))
                                    .rounded(px(1.0))
                                    .bg(bar_color)
                            })),
                    )
                    // Footer: Duration timer + 2X speed pill + message timestamp
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .child(format!(
                                                "{:02}:{:02} / {:02}:{:02}",
                                                current_sec / 60,
                                                current_sec % 60,
                                                duration_secs / 60,
                                                duration_secs % 60
                                            )),
                                    )
                                    // Speed button (e.g. 2X)
                                    .child(
                                        div()
                                            .px(px(4.0))
                                            .py(px(1.0))
                                            .rounded(px(3.0))
                                            .bg(rgba(0x00000033))
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if playback_speed > 1.0 { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                                            .child(format!("{:.0}X", playback_speed)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(message.timestamp.clone()),
                            ),
                    ),
            )
            .when(rounding.has_tail, |el| {
                el.child(render_bubble_tail(message.direction, bg_color))
            })
    }
}
