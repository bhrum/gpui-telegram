//! Full-Duplex Talk Mode Orb Overlay & Call Modal (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.6, CAP-05).
//!
//! Sub-components:
//! 1. Glowing Full-Duplex Orb (states: Connecting, Listening, Thinking, Speaking, Muted).
//! 2. Live Speech-to-Speech Transcript Stream.
//! 3. Agent Persona Selector.
//! 4. Control Toolbar (Mute, Output Device, End Session).

use gpui::*;
use serde::{Deserialize, Serialize};

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
    pub speaker: String, // "You" or Agent Name
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
}

impl CallOverlay {
    pub fn new(peer_name: String) -> Self {
        let persona_name = if peer_name.trim().is_empty() {
            "Voice Call".to_string()
        } else {
            peer_name.clone()
        };
        Self {
            peer_name,
            persona_name,
            state: TalkModeState::Listening,
            is_connected: true,
            transcript: Vec::new(),
            is_mic_muted: false,
            latency_ms: 0,
            should_close: false,
        }
    }

    pub fn set_state(&mut self, state: TalkModeState) {
        self.state = state;
        if state == TalkModeState::Muted {
            self.is_mic_muted = true;
        } else if self.is_mic_muted {
            self.is_mic_muted = false;
        }
    }

    pub fn toggle_mute(&mut self) {
        self.is_mic_muted = !self.is_mic_muted;
        self.state = if self.is_mic_muted {
            TalkModeState::Muted
        } else {
            TalkModeState::Listening
        };
    }

    pub fn add_transcript(&mut self, speaker: &str, text: &str, is_user: bool) {
        self.transcript.push(TranscriptEntry {
            speaker: speaker.to_string(),
            text: text.to_string(),
            is_user,
        });
    }
}

impl Default for CallOverlay {
    fn default() -> Self {
        Self::new("Alice Smith".to_string())
    }
}

impl Render for CallOverlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (orb_color, orb_glow, state_label) = match self.state {
            TalkModeState::Connecting => (rgb(0x8A9AA8), rgba(0x8A9AA833), "Connecting WebRTC..."),
            TalkModeState::Listening => (rgb(0x2AABEE), rgba(0x2AABEE44), "Connected"),
            TalkModeState::Thinking => (rgb(0x9B59B6), rgba(0x9B59B644), "Processing..."),
            TalkModeState::Speaking => (rgb(0x1ABC9C), rgba(0x1ABC9C44), "Audio Active"),
            TalkModeState::Muted => (rgb(0xE53935), rgba(0xE5393533), "Microphone Muted"),
        };

        let transcript_slice: Vec<TranscriptEntry> = self.transcript.iter().rev().take(3).rev().cloned().collect();

        div()
            .w(px(380.0))
            .h(px(520.0))
            .rounded(px(20.0))
            .bg(rgb(0x101923))
            .border_1()
            .border_color(rgb(0x233344))
            .p(px(20.0))
            .flex()
            .flex_col()
            .justify_between()
            // 1. Top Header: Session Info & Connection Latency
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(self.persona_name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(if self.latency_ms > 0 {
                                        format!("WebRTC P2P • {}ms latency", self.latency_ms)
                                    } else if self.is_connected {
                                        "WebRTC P2P • Connected".to_string()
                                    } else {
                                        "WebRTC P2P • Connecting...".to_string()
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded_full()
                            .bg(rgba(0xE5393533))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xE53935))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.should_close = true;
                                cx.notify();
                            }))
                            .child("✕ Close"),
                    ),
            )
            // 2. Center: Glowing Full-Duplex Pulsing Orb (Clickable to advance AI conversation turn)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    // Outer Ripple Ring
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(140.0))
                            .h(px(140.0))
                            .rounded_full()
                            .bg(orb_glow)
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_mute();
                                cx.notify();
                            }))
                            // Inner Pulsing Orb
                            .child(
                                div()
                                    .w(px(100.0))
                                    .h(px(100.0))
                                    .rounded_full()
                                    .bg(orb_color)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(36.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .child(match self.state {
                                        TalkModeState::Listening => "🎙️",
                                        TalkModeState::Thinking => "⚡",
                                        TalkModeState::Speaking => "🔊",
                                        TalkModeState::Muted => "🔇",
                                        TalkModeState::Connecting => "⏳",
                                    }),
                            ),
                    )
                    // State status label
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(orb_color)
                            .child(state_label),
                    ),
            )
            // 3. Live Speech Transcript Box
            .child(
                div()
                    .h(px(120.0))
                    .w_full()
                    .rounded(px(12.0))
                    .bg(rgb(0x17222E))
                    .p(px(10.0))
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(if transcript_slice.is_empty() {
                        vec![
                            div()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .text_color(rgb(0x8A9AA8))
                                .child("Speech transcription will appear here when active.")
                                .into_any_element()
                        ]
                    } else {
                        transcript_slice.into_iter().map(|t| {
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if t.is_user { rgb(0x2AABEE) } else { rgb(0x1ABC9C) })
                                        .child(t.speaker),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(t.text),
                                )
                                .into_any_element()
                        }).collect()
                    }),
            )
            // 4. Bottom Controls: Mute, End Call, Speaker
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(18.0))
                    // Mute Button
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(46.0))
                            .h(px(46.0))
                            .rounded_full()
                            .bg(if self.is_mic_muted { rgb(0xE53935) } else { rgb(0x243447) })
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_mute();
                                cx.notify();
                            }))
                            .child(if self.is_mic_muted { "🔇" } else { "🎙️" }),
                    )
                    // End Session Button
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(54.0))
                            .h(px(54.0))
                            .rounded_full()
                            .bg(rgb(0xE53935))
                            .hover(|s| s.bg(rgb(0xC62828)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(22.0))
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.should_close = true;
                                cx.notify();
                            }))
                            .child("📞"),
                    )
                    // Output Audio Switch
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(46.0))
                            .h(px(46.0))
                            .rounded_full()
                            .bg(rgb(0x243447))
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event, _window, _cx| {
                                // Speaker output toggle placeholder (preserved for hardware routing)
                            }))
                            .child("🔊"),
                    ),
            )
    }
}
