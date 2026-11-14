use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalkModeState {
    Connecting,
    Listening,
    Thinking,
    Speaking,
    Muted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub speaker: String,
    pub text: String,
    pub is_user: bool,
}

pub struct CallOverlay {
    pub peer_name: String,
    pub persona_name: String,
    pub state: TalkModeState,
    pub is_connected: bool,
    pub transcript: Vec<TranscriptEntry>,
    pub is_mic_muted: bool,
    pub latency_ms: u32,
    pub should_close: bool,
    pub e2ee_emojis: String,
    pub is_camera_on: bool,
    pub is_screen_sharing: bool,
    pub noise_suppression_on: bool,
    pub is_group_call_grid: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub avatar_path: Option<String>,
    pub duration_secs: u64,
    pub signal_bars: u8,
    pub is_speaker_on: bool,
    pub available_outputs: Vec<String>,
    pub selected_output_device: String,
    pub show_rate_call: bool,
    pub call_rating: u8,
    pub pending_hangup: bool,
    pub pending_rating_submit: Option<u8>,
}

impl CallOverlay {
    pub fn new(peer_name: String) -> Self {
        let persona_name = if peer_name.trim().is_empty() { "Voice Call".to_string() } else { peer_name.clone() };
        let emoji_pool = ["🔒", "🚀", "💎", "🔥", "🌈", "⚡", "🎯", "🐳", "🍀", "🛰️", "🪐", "🎸", "🏔️", "🦊", "🌊", "🔮"];
        let seed = peer_name.bytes().fold(0x9E3779B9usize, |acc, b| acc.wrapping_mul(31).wrapping_add(b as usize));
        let e2ee_emojis = (0..4).map(|i| emoji_pool[(seed.wrapping_shr((i * 4) as u32)) % emoji_pool.len()]).collect::<Vec<_>>().join(" ");
        Self {
            peer_name,
            persona_name,
            state: TalkModeState::Listening,
            is_connected: true,
            transcript: Vec::new(),
            is_mic_muted: false,
            latency_ms: 0,
            should_close: false,
            e2ee_emojis,
            is_camera_on: false,
            is_screen_sharing: false,
            noise_suppression_on: true,
            is_group_call_grid: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".to_string(),
            avatar_path: None,
            duration_secs: 0,
            signal_bars: 4,
            is_speaker_on: true,
            available_outputs: vec!["Default Output".into(), "Headphones".into(), "Speakers".into()],
            selected_output_device: "Default Output".into(),
            show_rate_call: false,
            call_rating: 0,
            pending_hangup: false,
            pending_rating_submit: None,
        }
    }

    pub fn new_live(peer_name: String, avatar_path: Option<String>, e2ee_emojis: Option<String>, palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new(peer_name);
        s.state = TalkModeState::Connecting;
        s.is_connected = false;
        s.signal_bars = 0;
        s.avatar_path = avatar_path;
        if let Some(emojis) = e2ee_emojis.filter(|e| !e.trim().is_empty()) { s.e2ee_emojis = emojis; }
        s.palette = palette;
        s.language_id = language_id.to_string();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn set_signal_bars(&mut self, bars: u8) { self.signal_bars = bars.min(4); }
    pub fn set_latency(&mut self, ms: u32) { self.latency_ms = ms; }
    pub fn set_duration(&mut self, secs: u64) { self.duration_secs = secs; }
    pub fn toggle_camera(&mut self) { self.is_camera_on = !self.is_camera_on; }
    pub fn toggle_screen_share(&mut self) { self.is_screen_sharing = !self.is_screen_sharing; }
    pub fn toggle_speaker(&mut self) { self.is_speaker_on = !self.is_speaker_on; }
    pub fn enter_rate_call(&mut self) { self.show_rate_call = true; self.is_connected = false; }
    pub fn take_rating_submission(&mut self) -> Option<u8> { self.pending_rating_submit.take() }

    pub fn cycle_audio_output(&mut self) {
        if let Some(idx) = self.available_outputs.iter().position(|d| d == &self.selected_output_device) {
            self.selected_output_device = self.available_outputs[(idx + 1) % self.available_outputs.len()].clone();
        }
    }

    pub fn submit_rating(&mut self, stars: u8) {
        self.call_rating = stars.clamp(1, 5);
        self.pending_rating_submit = Some(self.call_rating);
        self.should_close = true;
    }

    pub fn hangup(&mut self) {
        self.pending_hangup = true;
        if self.duration_secs > 0 && !self.show_rate_call { self.enter_rate_call(); } else { self.should_close = true; }
    }

    pub fn set_state(&mut self, state: TalkModeState) {
        self.state = state;
        self.is_mic_muted = state == TalkModeState::Muted;
    }

    pub fn toggle_mute(&mut self) {
        self.is_mic_muted = !self.is_mic_muted;
        self.state = if self.is_mic_muted { TalkModeState::Muted } else { TalkModeState::Listening };
    }

    pub fn add_transcript(&mut self, speaker: &str, text: &str, is_user: bool) {
        self.transcript.push(TranscriptEntry { speaker: speaker.to_string(), text: text.to_string(), is_user });
    }
}

impl Default for CallOverlay { fn default() -> Self { Self::new(String::new()) } }

impl Render for CallOverlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let (state_color, state_label) = match self.state {
            TalkModeState::Connecting => (pal.text_secondary, tr_str(&lang, "正在连接...", "Connecting...")),
            TalkModeState::Listening => (pal.accent, tr_str(&lang, "正在通话", "Connected")),
            TalkModeState::Thinking => (0x9b51e0, tr_str(&lang, "正在处理...", "Processing...")),
            TalkModeState::Speaking => (pal.success, tr_str(&lang, "语音传输中", "Audio Active")),
            TalkModeState::Muted => (pal.danger, tr_str(&lang, "已静音", "Microphone Muted")),
        };
        let duration_text = if self.duration_secs > 0 { format!("{:02}:{:02}", self.duration_secs / 60, self.duration_secs % 60) } else { state_label.to_string() };
        let latency_suffix = if self.latency_ms > 0 { format!(" · {} ms", self.latency_ms) } else { String::new() };
        let initial = self.persona_name.chars().next().unwrap_or('T').to_uppercase().to_string();
        let (av_top, _) = pal.peer_avatar_gradient(self.peer_name.len() as i64);
        let transcript_slice: Vec<TranscriptEntry> = self.transcript.iter().rev().take(2).rev().cloned().collect();

        let btn = |icon: IconName, active: bool, danger: bool, p: &TdesktopThemePalette| {
            let bg = if danger { p.danger } else if active { p.accent } else { p.input_bg };
            let fg = if danger || active { p.active_button_fg } else { p.icon_active };
            div().cursor_pointer().size(px(44.0)).rounded_full().bg(rgb(bg)).flex().items_center().justify_center().child(tg_icon(icon, 18.0, fg))
        };

        if self.show_rate_call {
            return div().w(px(380.0)).rounded(px(18.0)).bg(rgb(pal.modal_bg)).border_1().border_color(rgb(pal.border_fg)).p(px(24.0)).flex().flex_col().items_center().gap(px(14.0))
                .child(tg_icon(IconName::PhoneCall, 28.0, pal.accent))
                .child(div().text_base().font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(tr_str(&lang, "通话质量评价", "Rate This Call")))
                .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(format!("{} · {}", self.persona_name, duration_text)))
                .child(div().flex().flex_row().gap(px(8.0)).children((1_u8..=5).map(|star| {
                    let filled = star <= self.call_rating;
                    div().cursor_pointer().p(px(6.0)).on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.call_rating = star; cx.notify(); }))
                        .child(tg_icon(IconName::PremiumStar, 26.0, if filled { pal.accent } else { pal.muted_badge_bg }))
                })))
                .child(div().flex().flex_row().gap(px(10.0)).w_full().pt(px(6.0))
                    .child(div().flex_1().py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.input_bg)).flex().items_center().justify_center().text_xs().text_color(rgb(pal.text_secondary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.should_close = true; cx.notify(); })).child(tr_str(&lang, "跳过", "Skip")))
                    .when(self.call_rating > 0, |el| el.child(div().flex_1().py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.accent)).flex().items_center().justify_center().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { let r = this.call_rating; this.submit_rating(r); cx.notify(); })).child(tr_str(&lang, "提交评价", "Submit")))))
                .into_any_element();
        }

        div().w(px(380.0)).h(px(530.0)).rounded(px(18.0)).bg(rgb(pal.wallpaper_top)).border_1().border_color(rgb(pal.border_fg)).p(px(20.0)).flex().flex_col().justify_between()
            .child(div().flex().items_center().justify_between()
                .child(div().flex().flex_row().items_center().gap(px(8.0))
                    .child(div().flex().flex_row().items_end().gap(px(2.0)).children((1_u8..=4).map(|b| div().w(px(3.0)).h(px((b as f32) * 3.0 + 2.0)).rounded(px(1.0)).bg(rgb(if b <= self.signal_bars { pal.success } else { pal.muted_badge_bg })))))
                    .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(format!("{}{}", duration_text, latency_suffix))))
                .child(div().flex().flex_row().items_center().gap(px(8.0))
                    .child(div().px(px(8.0)).py(px(3.0)).rounded_full().bg(TdesktopThemePalette::rgba_with_alpha(pal.card_bg, 0.72)).flex().flex_row().items_center().gap(px(4.0)).child(tg_icon(IconName::Lock, 11.0, pal.text_secondary)).child(div().text_xs().text_color(rgb(pal.text_primary)).child(self.e2ee_emojis.clone())))
                    .child(div().cursor_pointer().p(px(5.0)).rounded_full().bg(rgb(pal.input_bg)).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.should_close = true; cx.notify(); })).child(tg_icon(IconName::Close, 13.0, pal.icon_fg)))))
            .child(div().flex().flex_col().items_center().gap(px(10.0))
                .child(div().size(px(124.0)).rounded_full().bg(TdesktopThemePalette::rgba_with_alpha(state_color, 0.20)).flex().items_center().justify_center().child(if let Some(ref path) = self.avatar_path {
                    div().size(px(96.0)).rounded_full().overflow_hidden().child(img(path.clone()).size_full().object_fit(ObjectFit::Cover)).into_any_element()
                } else {
                    div().size(px(96.0)).rounded_full().bg(rgb(av_top)).flex().items_center().justify_center().text_size(px(32.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg)).child(initial).into_any_element()
                }))
                .child(div().text_lg().font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(self.persona_name.clone()))
                .child(div().text_xs().text_color(rgb(state_color)).child(if self.is_camera_on { format!("{} · Video On", state_label) } else { state_label.to_string() }))
                .child(div().px(px(10.0)).py(px(4.0)).rounded_full().bg(rgb(pal.card_bg)).border_1().border_color(rgb(pal.border_fg)).text_xs().text_color(rgb(pal.text_secondary)).cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.cycle_audio_output(); cx.notify(); })).child(self.selected_output_device.clone())))
            .when(!transcript_slice.is_empty(), |el| el.child(div().p(px(10.0)).rounded(px(10.0)).bg(rgb(pal.card_bg)).border_1().border_color(rgb(pal.border_fg)).flex().flex_col().gap(px(6.0)).children(transcript_slice.into_iter().map(|t| {
                div().flex().flex_col().child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(if t.is_user { pal.accent } else { pal.success })).child(t.speaker)).child(div().text_xs().text_color(rgb(pal.text_primary)).child(t.text))
            }))))
            .child(div().flex().items_center().justify_center().gap(px(12.0))
                .child(btn(IconName::Speaker, self.is_speaker_on, false, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_speaker(); cx.notify(); })))
                .child(btn(IconName::VideoCall, self.is_camera_on, false, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_camera(); cx.notify(); })))
                .child(btn(IconName::ScreenShare, self.is_screen_sharing, false, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_screen_share(); cx.notify(); })))
                .child(btn(if self.is_mic_muted { IconName::Mute } else { IconName::Mic }, false, self.is_mic_muted, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_mute(); cx.notify(); })))
                .child(btn(IconName::Hangup, false, true, &pal).on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.hangup(); cx.notify(); }))))
            .into_any_element()
    }
}
