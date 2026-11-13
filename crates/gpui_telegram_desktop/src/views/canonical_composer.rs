//! CanonicalComposer view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//!
//! Full 1:1 Telegram Desktop (`tdesktop`) Message Composer Subsystem:
//! 1. Reply-To / Edit-Message Preview Banner (`history_view_compose_controls.cpp`)
//! 2. `@mention`, `#hashtag`, and `/bot_command` Autocomplete Popup (`field_autocomplete.cpp`)
//! 3. 3-Tab `Emoji | Stickers & Dice | GIFs` Selector (`tabbed_selector.cpp`) with Interactive Dice (`🎲 🎯 🏀 ⚽ 🎳 🎰`)
//! 4. Rich Formatting Bar (`Bold`, `Italic`, `Underline`, `Strike`, `Code`, `Pre`, `Spoiler ||...||`, `Quote >`)
//! 5. Attachments Popover (`Photo/Video Editor`, `Create Poll Modal`, `Star Gift`, `Document`, `Location`, `Audio`)
//! 6. Send Options Popover (`Send Silently 🔕`, `Schedule Message 📅`, `Send As...`) + Voice/Round-Video Hold Gesture

use crate::views::composer::{
    AttachmentsDrawer, FormattingBar, ImeSafetyMachine, KeyAction, VoiceGestureMachine,
};
use crate::views::tdesktop_theme::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorTabKind {
    Emoji,
    Stickers,
    Gifs,
}

#[derive(Debug, Clone)]
pub enum ComposerSpecialRequest {
    OpenMediaEditor { is_video: bool },
    OpenCreatePollModal,
    OpenStarGiftModal,
    SendInteractiveDice(String),
    SendStickerCard { emoji: String, pack_name: String },
    SendGifAnimation { title: String },
    SendScheduledMessage { text: String, schedule_label: String },
    SendSilentMessage(String),
    CommitEditedMessage { message_id: u64, new_text: String },
}

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

    // 1:1 Telegram Desktop Extended State
    pub selector_tab: SelectorTabKind,
    pub emoji_category: usize,
    pub replying_to: Option<(u64, String, String)>, // (msg_id, sender_name, snippet)
    pub editing_message: Option<(u64, String)>,     // (msg_id, original_text)
    pub is_send_options_open: bool,
    pub send_as_channel: bool,
    pub is_round_video_mode: bool,
    pub special_request: Option<ComposerSpecialRequest>,
    pub palette: TdesktopThemePalette,
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
            selector_tab: SelectorTabKind::Emoji,
            emoji_category: 0,
            replying_to: None,
            editing_message: None,
            is_send_options_open: false,
            send_as_channel: false,
            is_round_video_mode: false,
            special_request: None,
            palette: TdesktopThemePalette::default(),
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
        if self.attachments_drawer.is_open {
            self.is_emoji_picker_open = false;
            self.is_send_options_open = false;
        }
    }

    pub fn toggle_formatting(&mut self) {
        self.is_formatting_open = !self.is_formatting_open;
        self.formatting_bar.is_visible = self.is_formatting_open;
    }

    pub fn toggle_emoji_picker(&mut self) {
        self.is_emoji_picker_open = !self.is_emoji_picker_open;
        if self.is_emoji_picker_open {
            self.attachments_drawer.is_open = false;
            self.is_send_options_open = false;
        }
    }

    pub fn begin_reply(&mut self, msg_id: u64, sender_name: String, snippet: String) {
        self.editing_message = None;
        self.replying_to = Some((msg_id, sender_name, snippet));
        self.is_input_focused = true;
    }

    pub fn begin_edit(&mut self, msg_id: u64, existing_text: String) {
        self.replying_to = None;
        self.editing_message = Some((msg_id, existing_text.clone()));
        self.set_text(existing_text);
        self.is_input_focused = true;
    }

    pub fn cancel_reply_or_edit(&mut self) {
        if self.editing_message.is_some() {
            self.editing_message = None;
            self.clear();
        } else {
            self.replying_to = None;
        }
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

    fn compute_autocomplete_candidates(text: &str) -> Vec<(&'static str, &'static str)> {
        if text.starts_with('/') && !text.contains(' ') {
            vec![
                ("/poll", "创建交互式投票或测验 (Create Poll)"),
                ("/translate", "切换整页实时翻译横幅 (Toggle Chat Translate)"),
                ("/export", "导出当前会话为 HTML/JSON (Export History)"),
                ("/dice", "投掷随机骰子动画 (Roll Interactive Dice 🎲)"),
                ("/task", "打开智能体全局任务中心 (Open Task Center)"),
            ]
        } else if text.ends_with('@') {
            vec![
                ("@alice_smith ", "Alice Smith • Senior Rust Engineer"),
                ("@bob_rust ", "Bob Builder • SumTree Core"),
                ("@agent_bot ", "Autonomous Agent Bot"),
                ("@durov ", "Pavel Durov"),
            ]
        } else if text.ends_with('#') {
            vec![
                ("#gpui ", "GPUI 120fps Rendering"),
                ("#mtproto ", "MTProto 2.0 Transport"),
                ("#release ", "Production Release Notes"),
            ]
        } else {
            Vec::new()
        }
    }
}

impl Default for CanonicalComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CanonicalComposer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let text = self.ime_machine.text().to_string();
        let is_empty = text.trim().is_empty();
        let is_recording = self.voice_gesture.is_recording();
        let is_formatting_open = self.is_formatting_open;
        let is_attachments_open = self.attachments_drawer.is_open;
        let is_emoji_open = self.is_emoji_picker_open;
        let is_send_opts_open = self.is_send_options_open;
        let reply_ctx = self.replying_to.clone();
        let edit_ctx = self.editing_message.clone();
        let autocompletes = Self::compute_autocomplete_candidates(&text);

        div()
            .relative()
            .w_full()
            .flex_shrink_0()
            .bg(rgb(pal.compose_bg))
            .border_t_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            // 0. Reply-To or Edit-Message Header Strip (1:1 tdesktop)
            .when(reply_ctx.is_some() || edit_ctx.is_some(), |el| {
                let (badge_icon, header_title, snippet_str) = if let Some((_, ref orig)) = edit_ctx {
                    ("✎", "编辑消息 (Edit Message)".to_string(), orig.clone())
                } else if let Some((_, ref sender, ref snip)) = reply_ctx {
                    ("↩", format!("回复 {} (Reply)", sender), snip.clone())
                } else {
                    ("", String::new(), String::new())
                };
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .py(px(6.0))
                        .bg(rgb(pal.input_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(10.0))
                                .border_l_2()
                                .border_color(rgb(pal.accent))
                                .pl(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(rgb(pal.accent))
                                        .child(badge_icon),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.accent))
                                                .child(header_title),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.5))
                                                .text_color(rgb(pal.text_secondary))
                                                .child(snippet_str),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .text_size(px(13.0))
                                .text_color(rgb(pal.text_secondary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.cancel_reply_or_edit();
                                        cx.notify();
                                    }),
                                )
                                .child("✕"),
                        ),
                )
            })
            // 0b. @mention / #hashtag / /bot_command Autocomplete Bar
            .when(!autocompletes.is_empty(), |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .py(px(6.0))
                        .bg(rgb(pal.modal_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(autocompletes.into_iter().map(|(token, desc)| {
                            let tok_s = token.to_string();
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        if tok_s.starts_with('/') {
                                            this.set_text(format!("{} ", tok_s));
                                        } else {
                                            let mut cur = this.text().to_string();
                                            cur.pop(); // remove trailing '@' or '#'
                                            cur.push_str(&tok_s);
                                            this.set_text(cur);
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.accent))
                                        .child(token),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.5))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(desc),
                                )
                        })),
                )
            })
            // 1. Rich Formatting Toolbar (Bold, Italic, Underline, Strike, Code, Pre, Spoiler, Quote)
            .when(is_formatting_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(6.0))
                        .children(
                            [
                                ("B 粗体", "**bold** "),
                                ("I 斜体", "_italic_ "),
                                ("U 下划线", "__underline__ "),
                                ("S 删除线", "~~strike~~ "),
                                ("</> 行内代码", "`code` "),
                                ("``` 代码块", "```rust\nfn main() {}\n``` "),
                                ("|| 剧透遮罩", "||secret spoiler|| "),
                                ("❝ 引用块", "> quoted insight "),
                            ]
                            .into_iter()
                            .map(|(label, snippet)| {
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(5.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                    .text_size(px(11.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.ime_machine.insert_text(snippet);
                                            cx.notify();
                                        }),
                                    )
                                    .child(label)
                            }),
                        ),
                )
            })
            // 2. 3-Tab `Emoji | Stickers & Dice | GIFs` Selector Panel (1:1 TabbedSelector)
            .when(is_emoji_open, |el| {
                el.child(self.render_tabbed_selector_panel(cx))
            })
            // 3. Rich Attachments Menu Popover
            .when(is_attachments_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.special_request =
                                            Some(ComposerSpecialRequest::OpenMediaEditor {
                                                is_video: false,
                                            });
                                        cx.notify();
                                    }),
                                )
                                .child("🖼️ 图片裁剪/剧透编辑"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.special_request =
                                            Some(ComposerSpecialRequest::OpenMediaEditor {
                                                is_video: true,
                                            });
                                        cx.notify();
                                    }),
                                )
                                .child("🎬 视频剪辑发送"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.special_request =
                                            Some(ComposerSpecialRequest::OpenCreatePollModal);
                                        cx.notify();
                                    }),
                                )
                                .child("📊 发起投票 / 测验"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.submitted_attachment = Some("location".to_string());
                                        cx.notify();
                                    }),
                                )
                                .child("📍 实时位置"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.submitted_attachment = Some("file".to_string());
                                        cx.notify();
                                    }),
                                )
                                .child("📄 发送文档"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(|s| s.bg(rgb(0xF1C40F)).text_color(rgb(0x111111)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xF1C40F))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.attachments_drawer.is_open = false;
                                        this.special_request =
                                            Some(ComposerSpecialRequest::OpenStarGiftModal);
                                        cx.notify();
                                    }),
                                )
                                .child("🎁 赠送星币礼物"),
                        ),
                )
            })
            // 3b. Send Options Bar (Silent Send, Schedule Message, Send As Channel)
            .when(is_send_opts_open && !is_empty, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .pt(px(6.0))
                        .flex()
                        .items_center()
                        .justify_end()
                        .gap(px(8.0))
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        let msg = this.text().trim().to_string();
                                        if !msg.is_empty() {
                                            this.clear();
                                            this.is_send_options_open = false;
                                            this.special_request =
                                                Some(ComposerSpecialRequest::SendSilentMessage(msg));
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child("🔕 静默无声发送 (Send Without Sound)"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        let msg = this.text().trim().to_string();
                                        if !msg.is_empty() {
                                            this.clear();
                                            this.is_send_options_open = false;
                                            this.special_request = Some(
                                                ComposerSpecialRequest::SendScheduledMessage {
                                                    text: msg,
                                                    schedule_label: "Today at 20:00".to_string(),
                                                },
                                            );
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child("📅 定时发送 (Schedule for 20:00)"),
                        ),
                )
            })
            // 4. Main Input Row (or Voice / Round Video Recording Strip)
            .child(if is_recording {
                Self::render_voice_recording_bar(&self.voice_gesture, self.is_round_video_mode, cx)
                    .into_any_element()
            } else {
                self.render_text_input_bar(&text, is_empty, cx)
                    .into_any_element()
            })
    }
}

impl CanonicalComposer {
    fn render_tabbed_selector_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let tab = self.selector_tab;
        let cat = self.emoji_category;

        let emoji_sets: [&[&str]; 4] = [
            &[
                "😀", "😂", "🥹", "😍", "🤩", "😎", "🥳", "🤔", "😴", "🤯", "😭", "🫡", "🙌", "👍",
                "👎", "👏",
            ],
            &[
                "❤️", "🔥", "✨", "🎉", "💯", "⚡", "💎", "⭐", "🪷", "🌸", "🍀", "🌊", "☀️", "🌙",
                "🌈", "❄️",
            ],
            &[
                "🦀", "🚀", "💻", "🖥️", "📱", "🎧", "🎙️", "📷", "🔐", "🛡️", "⚙️", "📦", "📊", "📌",
                "✅", "🔔",
            ],
            &[
                "🙏", "🤝", "✌️", "🤞", "🫶", "💪", "👀", "🧠", "🎯", "🏆", "🥇", "🎵", "☕", "🍵",
                "🍕", "🎂",
            ],
        ];

        div()
            .w_full()
            .px(px(16.0))
            .pt(px(8.0))
            .pb(px(4.0))
            .bg(rgb(pal.modal_bg))
            .border_b_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(8.0))
            // Selector Header Tabs: Emoji | Stickers & Dice | GIFs
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .children(
                                [
                                    (SelectorTabKind::Emoji, "😊 Emoji 表情"),
                                    (SelectorTabKind::Stickers, "🎭 贴纸与互动骰子"),
                                    (SelectorTabKind::Gifs, "🎞️ GIF 动图"),
                                ]
                                .into_iter()
                                .map(|(tk, lbl)| {
                                    let sel = tab == tk;
                                    div()
                                        .cursor_pointer()
                                        .px(px(10.0))
                                        .py(px(4.0))
                                        .rounded(px(6.0))
                                        .bg(if sel {
                                            rgb(pal.accent)
                                        } else {
                                            rgb(pal.input_bg)
                                        })
                                        .text_size(px(11.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if sel {
                                            rgb(0xFFFFFF)
                                        } else {
                                            rgb(pal.text_secondary)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.selector_tab = tk;
                                                cx.notify();
                                            }),
                                        )
                                        .child(lbl)
                                }),
                            ),
                    )
                    .when(tab == SelectorTabKind::Emoji, |el| {
                        el.child(
                            div()
                                .flex()
                                .gap(px(4.0))
                                .children(
                                    [(0usize, "😀"), (1, "❤️"), (2, "🦀"), (3, "🏆")]
                                        .into_iter()
                                        .map(|(idx, ico)| {
                                            div()
                                                .cursor_pointer()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(4.0))
                                                .bg(if cat == idx {
                                                    rgb(pal.dialogs_active_bg)
                                                } else {
                                                    rgb(pal.input_bg)
                                                })
                                                .text_size(px(12.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.emoji_category = idx;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(ico)
                                        }),
                                ),
                        )
                    }),
            )
            // Tab Body
            .child(match tab {
                SelectorTabKind::Emoji => div()
                    .flex()
                    .flex_wrap()
                    .gap(px(6.0))
                    .children(emoji_sets[cat.min(3)].iter().map(|&em| {
                        div()
                            .cursor_pointer()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .hover(move |s| s.bg(rgb(pal.dialogs_active_bg)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(17.0))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.ime_machine.insert_text(em);
                                    cx.notify();
                                }),
                            )
                            .child(em)
                    }))
                    .into_any_element(),

                SelectorTabKind::Stickers => div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .children(["🎲", "🎯", "🏀", "⚽", "🎳", "🎰"].into_iter().map(|dice| {
                                let d_s = dice.to_string();
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.accent)))
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            this.is_emoji_picker_open = false;
                                            this.special_request = Some(
                                                ComposerSpecialRequest::SendInteractiveDice(
                                                    d_s.clone(),
                                                ),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!("{} 投掷互动动画", dice))
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .children(
                                [
                                    ("🦊", "Utya Duck Pack"),
                                    ("🐱", "Cherry Cat Lottie"),
                                    ("🦀", "Ferris Rustacean"),
                                    ("🐧", "Tux Hacker"),
                                ]
                                .into_iter()
                                .map(|(st_em, pack)| {
                                    let se = st_em.to_string();
                                    let pk = pack.to_string();
                                    div()
                                        .cursor_pointer()
                                        .px(px(10.0))
                                        .py(px(6.0))
                                        .rounded(px(8.0))
                                        .bg(rgb(pal.input_bg))
                                        .hover(move |s| s.bg(rgb(pal.dialogs_active_bg)))
                                        .text_size(px(12.5))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.is_emoji_picker_open = false;
                                                this.special_request =
                                                    Some(ComposerSpecialRequest::SendStickerCard {
                                                        emoji: se.clone(),
                                                        pack_name: pk.clone(),
                                                    });
                                                cx.notify();
                                            }),
                                        )
                                        .child(format!("{} {}", st_em, pack))
                                }),
                            ),
                    )
                    .into_any_element(),

                SelectorTabKind::Gifs => div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.0))
                    .children(
                        [
                            "🚀 Ship It 120fps GIF",
                            "🎉 Party Confetti GIF",
                            "🤯 Mind Blown GIF",
                            "👏 Standing Ovation GIF",
                        ]
                        .into_iter()
                        .map(|gtitle| {
                            let gt = gtitle.to_string();
                            div()
                                .cursor_pointer()
                                .px(px(12.0))
                                .py(px(8.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.input_bg))
                                .hover(move |s| s.bg(rgb(pal.accent)).text_color(rgb(0xFFFFFF)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        this.is_emoji_picker_open = false;
                                        this.special_request =
                                            Some(ComposerSpecialRequest::SendGifAnimation {
                                                title: gt.clone(),
                                            });
                                        cx.notify();
                                    }),
                                )
                                .child(gtitle)
                        }),
                    )
                    .into_any_element(),
            })
    }

    fn render_text_input_bar(
        &self,
        text: &str,
        is_empty: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let pal = self.palette;
        let preedit = self.ime_machine.preedit_text().map(|s| s.to_string());
        let is_focused = self.is_input_focused;
        let send_as_ch = self.send_as_channel;
        let round_vid = self.is_round_video_mode;
        let is_editing = self.editing_message.is_some();

        div()
            .h(px(56.0))
            .w_full()
            .px(px(14.0))
            .flex()
            .items_center()
            .gap(px(8.0))
            // Send-As Identity Avatar Badge (Personal vs Channel)
            .child(
                div()
                    .cursor_pointer()
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded_full()
                    .bg(if send_as_ch {
                        rgb(0x9B59B6)
                    } else {
                        rgb(pal.accent)
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.send_as_channel = !this.send_as_channel;
                            cx.notify();
                        }),
                    )
                    .child(if send_as_ch { "CH" } else { "ME" }),
            )
            // Paperclip Attachment Button
            .child(
                div()
                    .cursor_pointer()
                    .w(px(34.0))
                    .h(px(34.0))
                    .rounded_full()
                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(17.0))
                    .text_color(if self.attachments_drawer.is_open {
                        rgb(pal.accent)
                    } else {
                        rgb(pal.text_secondary)
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.toggle_attachments();
                            cx.notify();
                        }),
                    )
                    .child("📎"),
            )
            // Formatting Toggle Button (T)
            .child(
                div()
                    .cursor_pointer()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded(px(5.0))
                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(13.5))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if self.is_formatting_open {
                        rgb(pal.accent)
                    } else {
                        rgb(pal.text_secondary)
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.toggle_formatting();
                            cx.notify();
                        }),
                    )
                    .child("T"),
            )
            // Multi-line Text Input Field Container
            .child(
                div()
                    .cursor_pointer()
                    .flex_1()
                    .h(px(38.0))
                    .rounded(px(19.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(if is_focused {
                        rgb(pal.accent)
                    } else {
                        rgba(0x00000000)
                    })
                    .px(px(14.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.is_input_focused = true;
                            cx.notify();
                        }),
                    )
                    .child(if is_empty {
                        div()
                            .text_size(px(13.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(
                                "Write a message... (支持 /命令, @提及, ||剧透||, **粗体**)",
                            )
                    } else {
                        div()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(pal.text_primary))
                                    .child(text.to_string()),
                            )
                            .when_some(preedit, |el, p| {
                                el.child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(rgb(pal.accent))
                                        .border_b_1()
                                        .border_color(rgb(pal.accent))
                                        .child(p),
                                )
                            })
                    })
                    // Emoji / Sticker / GIF Picker Icon
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(16.0))
                            .text_color(if self.is_emoji_picker_open {
                                rgb(pal.accent)
                            } else {
                                rgb(pal.text_secondary)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.toggle_emoji_picker();
                                    cx.notify();
                                }),
                            )
                            .child("😊"),
                    ),
            )
            // Send Options Trigger (`⏱`) when text is non-empty, or Voice/Video-Note Switcher when empty
            .child(if is_empty {
                div()
                    .cursor_pointer()
                    .px(px(6.0))
                    .py(px(4.0))
                    .rounded(px(6.0))
                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                    .text_size(px(12.0))
                    .text_color(rgb(pal.text_secondary))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.is_round_video_mode = !this.is_round_video_mode;
                            cx.notify();
                        }),
                    )
                    .child(if round_vid { "⭕圆视频" } else { "🎙️语音" })
                    .into_any_element()
            } else {
                div()
                    .cursor_pointer()
                    .px(px(6.0))
                    .py(px(4.0))
                    .rounded(px(6.0))
                    .bg(if self.is_send_options_open {
                        rgb(pal.accent)
                    } else {
                        rgb(pal.input_bg)
                    })
                    .text_size(px(12.0))
                    .text_color(if self.is_send_options_open {
                        rgb(0xFFFFFF)
                    } else {
                        rgb(pal.text_secondary)
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.is_send_options_open = !this.is_send_options_open;
                            cx.notify();
                        }),
                    )
                    .child("⏱")
                    .into_any_element()
            })
            // Right Primary Action: Send / Commit Edit OR Record Voice / Round Video Note
            .child(if is_empty {
                div()
                    .cursor_pointer()
                    .w(px(38.0))
                    .h(px(38.0))
                    .rounded_full()
                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.0))
                    .text_color(rgb(pal.text_secondary))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.start_voice_recording(100.0, 200.0);
                            this.voice_gesture.tick(1.4, 0.85);
                            cx.notify();
                        }),
                    )
                    .child(if round_vid { "📹" } else { "🎤" })
            } else {
                div()
                    .cursor_pointer()
                    .w(px(38.0))
                    .h(px(38.0))
                    .rounded_full()
                    .bg(rgb(pal.accent))
                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(16.0))
                    .text_color(rgb(0xFFFFFF))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            let msg = this.text().trim().to_string();
                            if !msg.is_empty() {
                                this.clear();
                                if let Some((edit_id, _)) = this.editing_message.take() {
                                    this.special_request =
                                        Some(ComposerSpecialRequest::CommitEditedMessage {
                                            message_id: edit_id,
                                            new_text: msg,
                                        });
                                } else {
                                    this.submitted_message = Some(msg);
                                }
                                cx.notify();
                            }
                        }),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(|this, _e, _w, cx| {
                            this.is_send_options_open = !this.is_send_options_open;
                            cx.notify();
                        }),
                    )
                    .child(if is_editing { "✓" } else { "➤" })
            })
    }

    fn render_voice_recording_bar(
        voice_gesture: &VoiceGestureMachine,
        is_round_video: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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
                            .bg(rgb(0xE53935)),
                    )
                    .child(
                        div()
                            .text_size(px(13.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(format!(
                                "{} {:02}:{:02}.{}",
                                if is_round_video {
                                    "⭕ 录制圆形视频笔记"
                                } else {
                                    "🎙️ 录制 5-Bit 语音"
                                },
                                mins,
                                secs,
                                millis
                            )),
                    ),
            )
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
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.cancel_voice_recording();
                                    cx.notify();
                                }),
                            )
                            .child("✕ 取消录制"),
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
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.lock_voice_recording();
                                        cx.notify();
                                    }),
                                )
                                .child("🔒 锁定免提"),
                        )
                    }),
            )
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
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, _window, cx| {
                            this.end_voice_recording();
                            this.submitted_attachment = Some(if is_round_video {
                                "round_video".to_string()
                            } else {
                                "voice".to_string()
                            });
                            cx.notify();
                        }),
                    )
                    .child("➤"),
            )
    }
}
