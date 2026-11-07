//! CanonicalComposer view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//!
//! Sub-components & Capabilities:
//! 1. Multi-line text input with IME safety state machine (`ImeSafetyMachine`).
//! 2. Voice hold-to-record gesture state machine (`VoiceGestureMachine`).
//! 3. Attachments drawer (`AttachmentsDrawer`).
//! 4. Formatting bar (`FormattingBar`).
//! 5. Emoji & sticker picker button.

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::composer::{
    AttachmentsDrawer, FormattingBar, ImeSafetyMachine, KeyAction, VoiceGestureMachine,
    VoiceGestureState,
};

pub struct CanonicalComposer {
    pub ime_machine: ImeSafetyMachine,
    pub voice_gesture: VoiceGestureMachine,
    pub formatting_bar: FormattingBar,
    pub attachments_drawer: AttachmentsDrawer,
    pub is_emoji_picker_open: bool,
    pub is_formatting_open: bool,
    pub submitted_message: Option<String>,
    pub submitted_attachment: Option<String>,
    pub is_input_focused: bool,
}

impl CanonicalComposer {
    pub fn new() -> Self {
        Self {
            ime_machine: ImeSafetyMachine::new(),
            voice_gesture: VoiceGestureMachine::new(),
            formatting_bar: FormattingBar::new(),
            attachments_drawer: AttachmentsDrawer::new(),
            is_emoji_picker_open: false,
            is_formatting_open: false,
            submitted_message: None,
            submitted_attachment: None,
            is_input_focused: true,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.ime_machine = ImeSafetyMachine::with_text(text);
    }

    pub fn text(&self) -> &str {
        self.ime_machine.text()
    }

    pub fn clear(&mut self) {
        self.ime_machine.clear();
    }

    pub fn handle_enter(&mut self, is_shift: bool) -> KeyAction {
        self.ime_machine.handle_enter_key(is_shift)
    }

    pub fn toggle_attachments(&mut self) {
        self.attachments_drawer.toggle();
    }

    pub fn toggle_formatting(&mut self) {
        self.is_formatting_open = !self.is_formatting_open;
        self.formatting_bar.is_visible = self.is_formatting_open;
    }

    pub fn toggle_emoji_picker(&mut self) {
        self.is_emoji_picker_open = !self.is_emoji_picker_open;
    }

    pub fn start_voice_recording(&mut self, x: f32, y: f32) {
        self.voice_gesture.on_press_start(x, y);
    }

    pub fn move_voice_recording(&mut self, x: f32, y: f32) {
        self.voice_gesture.on_pointer_move(x, y);
    }

    pub fn end_voice_recording(&mut self) {
        self.voice_gesture.on_press_end();
    }

    pub fn lock_voice_recording(&mut self) {
        self.voice_gesture.lock();
    }

    pub fn cancel_voice_recording(&mut self) {
        self.voice_gesture.cancel();
    }

    pub fn is_recording_voice(&self) -> bool {
        self.voice_gesture.is_recording()
    }
}

impl Default for CanonicalComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CanonicalComposer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.ime_machine.text().to_string();
        let is_empty = text.trim().is_empty();
        let is_recording = self.voice_gesture.is_recording();
        let is_formatting_open = self.is_formatting_open;
        let is_attachments_open = self.attachments_drawer.is_open;
        let is_emoji_open = self.is_emoji_picker_open;

        div()
            .relative()
            .w_full()
            .bg(rgb(0x17212B)) // Upstream composer background
            .border_t_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            // 1. Formatting Bar (shown when formatting toggle is active)
            .when(is_formatting_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .children([
                            ("B", "**bold** "),
                            ("I", "_italic_ "),
                            ("U", "__underline__ "),
                            ("S", "~~strike~~ "),
                            ("</>", "`code` "),
                            ("||", "||spoiler|| "),
                        ].into_iter().map(|(label, snippet)| {
                            div()
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2AABEE)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.ime_machine.insert_text(snippet);
                                    cx.notify();
                                }))
                                .child(label)
                        })),
                )
            })
            // 2. Quick Emoji Strip (shown when emoji picker toggle is active)
            .when(is_emoji_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .children(["👍", "❤️", "🔥", "🎉", "🚀", "🤖", "✨", "🦀", "✅", "⚡"].into_iter().map(|emoji| {
                            div()
                                .cursor_pointer()
                                .w(px(30.0))
                                .h(px(30.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2B5278)))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(16.0))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.ime_machine.insert_text(emoji);
                                    cx.notify();
                                }))
                                .child(emoji)
                        })),
                )
            })
            // 3. Attachments Drawer Popover
            .when(is_attachments_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .children([
                            ("📷 Send Photo", "photo"),
                            ("🎬 Send Video", "video"),
                            ("📍 Share Location", "location"),
                            ("📊 Create Poll", "poll"),
                        ].into_iter().map(|(label, kind)| {
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x242F3D))
                                .hover(|s| s.bg(rgb(0x2AABEE)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.attachments_drawer.is_open = false;
                                    this.submitted_attachment = Some(kind.to_string());
                                    cx.notify();
                                }))
                                .child(label)
                        })),
                )
            })
            // 4. Main Input Row (or Voice Recording Strip)
            .child(
                if is_recording {
                    Self::render_voice_recording_bar(&self.voice_gesture, cx).into_any_element()
                } else {
                    self.render_text_input_bar(&text, is_empty, cx).into_any_element()
                },
            )
    }
}

impl CanonicalComposer {
    fn render_text_input_bar(&self, text: &str, is_empty: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let preedit = self.ime_machine.preedit_text().map(|s| s.to_string());
        let is_focused = self.is_input_focused;

        div()
            .h(px(56.0))
            .w_full()
            .px(px(16.0))
            .flex()
            .items_center()
            .gap(px(10.0))
            // Paperclip Attachment Button
            .child(
                div()
                    .cursor_pointer()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .hover(|s| s.bg(rgb(0x242F3D)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.0))
                    .text_color(if self.attachments_drawer.is_open { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.toggle_attachments();
                        cx.notify();
                    }))
                    .child("📎"),
            )
            // Formatting Toggle Button (T)
            .child(
                div()
                    .cursor_pointer()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded(px(4.0))
                    .hover(|s| s.bg(rgb(0x242F3D)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if self.is_formatting_open { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.toggle_formatting();
                        cx.notify();
                    }))
                    .child("T"),
            )
            // Multi-line Text Input Field Container
            .child(
                div()
                    .cursor_pointer()
                    .flex_1()
                    .h(px(38.0))
                    .rounded(px(19.0))
                    .bg(rgb(0x242F3D))
                    .border_1()
                    .border_color(if is_focused { rgb(0x2AABEE) } else { rgba(0x00000000) })
                    .px(px(14.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.is_input_focused = true;
                        cx.notify();
                    }))
                    // Draft text or placeholder
                    .child(
                        if is_empty {
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(0x6B808D))
                                .child("Write a message... (Type on keyboard & press Enter)")
                        } else {
                            div()
                                .flex()
                                .items_center()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(text.to_string()),
                                )
                                // Active IME preedit underline
                                .when_some(preedit, |el, p| {
                                    el.child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(rgb(0x2AABEE))
                                            .border_b_1()
                                            .border_color(rgb(0x2AABEE))
                                            .child(p),
                                    )
                                })
                        },
                    )
                    // Emoji / Sticker Picker Icon
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(16.0))
                            .text_color(if self.is_emoji_picker_open { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_emoji_picker();
                                cx.notify();
                            }))
                            .child("😊"),
                    ),
            )
            // Right Action: Send Button (if text) or Microphone (if empty)
            .child(
                if is_empty {
                    div()
                        .cursor_pointer()
                        .w(px(38.0))
                        .h(px(38.0))
                        .rounded_full()
                        .hover(|s| s.bg(rgb(0x242F3D)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(18.0))
                        .text_color(rgb(0x8A9AA8))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.start_voice_recording(100.0, 200.0);
                            this.voice_gesture.tick(1.4, 0.85);
                            cx.notify();
                        }))
                        .child("🎤")
                } else {
                    div()
                        .cursor_pointer()
                        .w(px(38.0))
                        .h(px(38.0))
                        .rounded_full()
                        .bg(rgb(0x2AABEE))
                        .hover(|s| s.bg(rgb(0x1E96D8)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(16.0))
                        .text_color(rgb(0xFFFFFF))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            let msg = this.text().trim().to_string();
                            if !msg.is_empty() {
                                this.clear();
                                this.submitted_message = Some(msg);
                                cx.notify();
                            }
                        }))
                        .child("➤")
                },
            )
    }

    fn render_voice_recording_bar(voice_gesture: &VoiceGestureMachine, cx: &mut Context<Self>) -> impl IntoElement {
        let dur = voice_gesture.duration_secs();
        let mins = (dur / 60.0) as u32;
        let secs = (dur % 60.0) as u32;
        let millis = ((dur % 1.0) * 10.0) as u32;
        let is_locked = voice_gesture.is_locked();

        div()
            .h(px(56.0))
            .w_full()
            .px(px(16.0))
            .flex()
            .items_center()
            .justify_between()
            .bg(rgb(0x1B2836))
            // Left: Red recording dot + Live Timer
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded_full()
                            .bg(rgb(0xE53935)), // Red pulse
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(format!("{:02}:{:02}.{}", mins, secs, millis)),
                    ),
            )
            // Center: Gesture hints / Cancel & Lock buttons
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .bg(rgba(0xE5393522))
                            .text_size(px(12.0))
                            .text_color(rgb(0xE53935))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.cancel_voice_recording();
                                cx.notify();
                            }))
                            .child("✕ Cancel"),
                    )
                    .when(!is_locked, |el| {
                        el.child(
                            div()
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(rgba(0x2AABEE22))
                                .text_size(px(12.0))
                                .text_color(rgb(0x2AABEE))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.lock_voice_recording();
                                    cx.notify();
                                }))
                                .child("🔒 Lock"),
                        )
                    }),
            )
            // Right: Stop & Send button
            .child(
                div()
                    .cursor_pointer()
                    .w(px(38.0))
                    .h(px(38.0))
                    .rounded_full()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(16.0))
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.end_voice_recording();
                        this.submitted_attachment = Some("voice".to_string());
                        cx.notify();
                    }))
                    .child("➤"),
            )
    }
}
