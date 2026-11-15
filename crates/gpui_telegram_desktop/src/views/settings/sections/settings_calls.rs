use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
    settings_toggle_row,
};

pub const K_MIC_TEST_UPDATE_INTERVAL_MS: u64 = 100;
pub const K_MIC_TEST_ANIMATION_DURATION_MS: u64 = 200;

pub struct SettingsCallsSections {
    pub input_device: String,
    pub output_device: String,
    pub camera_device: String,
    pub available_inputs: Vec<String>,
    pub available_outputs: Vec<String>,
    pub available_cameras: Vec<String>,
    pub mic_level: f32,
    pub mic_test_active: bool,
    pub p2p_mode: String,
    pub accept_calls: bool,
    pub pending_save_settings: bool,
    pub pending_prefs_sync: bool,
    pub pending_mic_test_change: Option<bool>,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsCallsSections {
    pub fn new() -> Self {
        Self {
            input_device: "Default Microphone".into(),
            output_device: "Default Output Device".into(),
            camera_device: "Default Camera".into(),
            available_inputs: vec!["Default Microphone".into(), "Built-in Mic".into()],
            available_outputs: vec!["Default Output Device".into(), "Speakers".into()],
            available_cameras: vec!["Default Camera".into(), "FaceTime HD Camera".into()],
            mic_level: 0.0,
            mic_test_active: false,
            p2p_mode: "Contacts".into(),
            accept_calls: true,
            pending_save_settings: false,
            pending_prefs_sync: false,
            pending_mic_test_change: None,
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

    pub fn sync_devices(&mut self, inputs: Vec<String>, outputs: Vec<String>, cameras: Vec<String>) {
        if !inputs.is_empty() {
            if !inputs.contains(&self.input_device) {
                self.input_device = inputs[0].clone();
            }
            self.available_inputs = inputs;
        }
        if !outputs.is_empty() {
            if !outputs.contains(&self.output_device) {
                self.output_device = outputs[0].clone();
            }
            self.available_outputs = outputs;
        }
        if !cameras.is_empty() {
            if !cameras.contains(&self.camera_device) {
                self.camera_device = cameras[0].clone();
            }
            self.available_cameras = cameras;
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn calls_id(&self) -> &'static str { "settings_calls" }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }

    pub fn calculate_normalized_level(&self, rms: f32, min_rms: f32, max_rms: f32) -> f32 {
        if max_rms <= min_rms { return 0.0; }
        ((rms - min_rms) / (max_rms - min_rms)).clamp(0.0, 1.0)
    }

    pub fn toggle_mic_test(&mut self) {
        self.mic_test_active = !self.mic_test_active;
        if !self.mic_test_active {
            self.mic_level = 0.0;
        }
        self.pending_mic_test_change = Some(self.mic_test_active);
    }

    pub fn set_mic_level_normalized(&mut self, level: f32) {
        self.mic_level = level.clamp(0.0, 1.0);
    }

    pub fn stop_mic_test_silent(&mut self) {
        self.mic_test_active = false;
        self.mic_level = 0.0;
        self.pending_mic_test_change = None;
    }

    pub fn update_mic_level(&mut self, rms: f32) { self.mic_level = self.calculate_normalized_level(rms, -60.0, 0.0); }
    pub fn set_p2p_mode(&mut self, mode: &str) { self.p2p_mode = mode.into(); self.pending_save_settings = true; }
    pub fn choose_playback_device(&mut self, device: &str) { self.output_device = device.into(); }
    pub fn choose_capture_device(&mut self, device: &str) { self.input_device = device.into(); }
    pub fn choose_camera_device(&mut self, device: &str) { self.camera_device = device.into(); }
    pub fn lng_group_call_speakers(&self) -> &'static str { "Speakers" }
    pub fn lng_settings_call_accept_calls(&self) -> bool { self.accept_calls }
    pub fn lng_settings_call_section_output(&self) -> &'static str { "Speakers and Headphones" }
    pub fn lng_settings_call_output_device(&self) -> &str { &self.output_device }
    pub fn lng_settings_call_camera(&self) -> &str { &self.camera_device }
    pub fn playback_device_name_value(&self) -> String { self.output_device.clone() }
    pub fn capture_device_name_value(&self) -> String { self.input_device.clone() }
    pub fn camera_device_name_value(&self) -> String { self.camera_device.clone() }

    fn cycle(list: &[String], current: &str) -> String {
        let idx = list.iter().position(|x| x == current).map(|i| (i + 1) % list.len()).unwrap_or(0);
        list.get(idx).cloned().unwrap_or_else(|| current.to_string())
    }
}

impl Default for SettingsCallsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsCallsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let mic_pct = (self.mic_level * 120.0).clamp(0.0, 120.0);
        settings_container(pal.clone())
            .child(settings_section_header(IconName::PhoneCall, tr_str(&lang, "通话设置", "Call Settings"), Some(tr_str(&lang, "管理麦克风、扬声器、摄像头及点对点连接策略", "Audio input/output devices, camera, and peer-to-peer preference")), pal.clone()))
            .child(settings_subsection_title(tr_str(&lang, "音频设备", "Audio Devices"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(div().flex().flex_row().justify_between().items_center().py(px(6.0)).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.input_device = Self::cycle(&this.available_inputs, &this.input_device); this.pending_prefs_sync = true; cx.notify(); }))
                        .child(div().flex().flex_row().items_center().gap(px(8.0)).child(tg_icon(IconName::Mic, 16.0, pal.icon_fg)).child(div().text_sm().text_color(rgb(pal.text_primary)).child(tr_str(&lang, "麦克风", "Microphone"))))
                        .child(div().text_xs().text_color(rgb(pal.text_link)).child(self.input_device.clone())))
                    .child(div().flex().flex_row().justify_between().items_center().py(px(6.0)).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.output_device = Self::cycle(&this.available_outputs, &this.output_device); this.pending_prefs_sync = true; cx.notify(); }))
                        .child(div().flex().flex_row().items_center().gap(px(8.0)).child(tg_icon(IconName::Speaker, 16.0, pal.icon_fg)).child(div().text_sm().text_color(rgb(pal.text_primary)).child(tr_str(&lang, "扬声器与耳机", self.lng_settings_call_section_output()))))
                        .child(div().text_xs().text_color(rgb(pal.text_link)).child(self.output_device.clone())))
                    .child(div().pt(px(4.0)).flex().flex_row().justify_between().items_center()
                        .child(div().id("settings-calls-mic-test-btn").px(px(10.0)).py(px(5.0)).rounded(px(6.0)).bg(rgb(if self.mic_test_active { pal.danger } else { pal.accent })).text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.active_button_fg)).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.toggle_mic_test(); cx.notify(); }))
                            .child(if self.mic_test_active { tr_str(&lang, "停止麦克风测试", "Stop Mic Test") } else { tr_str(&lang, "测试麦克风", "Test Microphone") }))
                        .child(div().w(px(120.0)).h(px(6.0)).rounded_full().bg(rgb(pal.input_bg)).child(div().w(px(mic_pct)).h_full().rounded_full().bg(rgb(pal.accent))))),
            )
            .child(settings_subsection_title(tr_str(&lang, "视频与端到端设置", "Video & Peer-to-Peer"), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(div().flex().flex_row().justify_between().items_center().py(px(6.0)).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.camera_device = Self::cycle(&this.available_cameras, &this.camera_device); this.pending_prefs_sync = true; cx.notify(); }))
                        .child(div().flex().flex_row().items_center().gap(px(8.0)).child(tg_icon(IconName::VideoCall, 16.0, pal.icon_fg)).child(div().text_sm().text_color(rgb(pal.text_primary)).child(tr_str(&lang, "摄像头", "Camera"))))
                        .child(div().text_xs().text_color(rgb(pal.text_link)).child(self.camera_device.clone())))
                    .child(div().pt(px(4.0)).flex().flex_row().gap(px(6.0)).children(["Everybody", "Contacts", "Nobody"].into_iter().map(|mode| {
                        let active = self.p2p_mode == mode;
                        let mode_lbl = match mode {
                            "Everybody" => tr_str(&lang, "所有人", "Everybody"),
                            "Contacts" => tr_str(&lang, "联系人", "Contacts"),
                            _ => tr_str(&lang, "没有人", "Nobody"),
                        };
                        div().flex_1().py(px(6.0)).rounded(px(6.0)).flex().items_center().justify_center().bg(rgb(if active { pal.accent } else { pal.input_bg })).text_xs().text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.set_p2p_mode(mode); cx.notify(); })).child(format!("P2P: {}", mode_lbl))
                    })))
                    .child(settings_toggle_row(IconName::PhoneCall, tr_str(&lang, "允许在此设备接听通话", "Accept Calls on This Device"), None, self.accept_calls, pal, |this| { this.accept_calls = !this.accept_calls; this.pending_save_settings = true; }, cx)),
            )
    }
}
