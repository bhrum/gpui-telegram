//! CanonicalComposer view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//!
//! Sub-components & Capabilities:
//! 1. Multi-line text input with IME safety state machine (`ImeSafetyMachine`).
//! 2. Voice hold-to-record gesture state machine (`VoiceGestureMachine`).
//! 3. Attachments drawer (`AttachmentsDrawer`).
//! 4. Formatting bar (`FormattingBar`).
//! 5. Emoji & sticker picker button.

use std::path::PathBuf;

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_telegram_client::net::hermes_client::HermesCommandDescriptor;

use crate::theme::TdesktopThemePalette;
use crate::views::composer::composer_ui::{
    render_composer_context_banner, render_text_input_strip, render_voice_recording_strip,
    ComposerBarAction,
};
use crate::views::composer::editor_state::{detect_autocomplete_trigger, ComposerEditorState};
use crate::views::composer::field_autocomplete::AutocompleteChooseMethod;
use crate::views::composer::popups_ui::{
    render_attachments_drawer, render_autocomplete_popup, render_formatting_toolbar,
    render_send_options_popup, ComposerPopupAction,
};
use crate::views::composer::selector_ui::{render_tabbed_selector_popup, SelectorUiAction};
use crate::views::composer::{
    AttachmentKind, AttachmentsDrawer, EmojiListWidget, FieldAutocomplete, FormattingBar,
    GifsListWidget, ImeSafetyMachine, KeyAction, StickersListWidget, TabbedSelector, TextStyleKind,
    VoiceGestureMachine,
};

pub const MIN_COMPOSER_HEIGHT: f32 = 44.0;
pub const MAX_COMPOSER_HEIGHT: f32 = 192.0;
pub const LINE_HEIGHT: f32 = 20.0;
pub const ACTION_BTN_WIDTH: f32 = 44.0;
pub const ACTION_BTN_HEIGHT: f32 = 46.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HermesSlashOption {
    pub name: String,
    pub slash_name: String,
    pub aliases: Vec<String>,
    pub syntax: String,
    pub category: String,
    pub description: String,
    pub accepts_args: bool,
}

fn default_hermes_slash_commands() -> Vec<HermesSlashOption> {
    let entries: &[(&str, &[&str], &str, &str, &str, bool)] = &[
        ("help", &["h", "?"], "/help [command]", "Session", "List Hermes slash commands & help", true),
        ("new", &["reset"], "/new [title]", "Session", "Start a new Hermes agent session", true),
        ("clear", &[], "/clear", "Session", "Clear messages in the active session", false),
        ("compact", &["compress"], "/compact [focus]", "Session", "Compress conversation context window", true),
        ("undo", &[], "/undo", "Session", "Undo last turn and restore session state", false),
        ("rollback", &[], "/rollback [steps]", "Safety & Control", "Rollback file checkpoints from recent turns", true),
        ("branch", &[], "/branch [name]", "Session", "Fork current session into a new branch", true),
        ("branches", &[], "/branches", "Session", "List all branches in current session", false),
        ("checkout", &[], "/checkout <branch>", "Session", "Switch to an existing session branch", true),
        ("search", &[], "/search <query>", "Inspection & Export", "FTS5 search across past Hermes sessions", true),
        ("history", &[], "/history [limit]", "Inspection & Export", "Show current session message history", true),
        ("sessions", &[], "/sessions", "Session", "List recent Hermes sessions", false),
        ("export", &["save"], "/export [sharegpt|openai|anthropic]", "Inspection & Export", "Export session transcript", true),
        ("usage", &["cost"], "/usage", "Inspection & Export", "Show token usage & estimated cost summary", false),
        ("insights", &[], "/insights [days]", "Inspection & Export", "Show workspace productivity & tool insights", true),
        ("status", &[], "/status", "Inspection & Export", "Show Hermes runtime health & active config", false),
        ("doctor", &[], "/doctor", "Inspection & Export", "Run Hermes diagnostics & provider checks", false),
        ("model", &[], "/model <name>", "Model & Provider", "Switch active LLM model", true),
        ("provider", &[], "/provider <id>", "Model & Provider", "Switch active provider / fallback chain", true),
        ("api-mode", &[], "/api-mode <chat_completions|codex_responses|anthropic_messages|local_hermes>", "Model & Provider", "Select API transport mode", true),
        ("toolsets", &[], "/toolsets [preset|toggle]", "Tools & Skills", "Inspect or toggle the 28 Hermes toolsets", true),
        ("tools", &[], "/tools", "Tools & Skills", "List active tools in current session", false),
        ("skills", &[], "/skills [filter]", "Tools & Skills", "Browse installed SKILL.md skills", true),
        ("skill", &[], "/skill <view|create|enable|disable> <name>", "Tools & Skills", "Inspect or manage a specific SKILL.md", true),
        ("learn", &[], "/learn <skill_name>", "Tools & Skills", "Extract reusable skill from current session", true),
        ("memory", &[], "/memory [add|replace|remove|clear]", "Memory & Persona", "View or edit MEMORY.md bounded facts", true),
        ("soul", &[], "/soul [prompt]", "Memory & Persona", "Inspect or update SOUL.md bot persona", true),
        ("user", &[], "/user [facts]", "Memory & Persona", "View or edit USER.md profile memory", true),
        ("reload-memory", &[], "/reload-memory", "Memory & Persona", "Refresh frozen session memory snapshot", false),
        ("honcho", &[], "/honcho [query|on|off]", "Memory & Persona", "Query or toggle Honcho dialectic memory", true),
        ("cron", &["jobs"], "/cron [list|create|run|delete]", "Workflow & Automation", "Manage scheduled cron automations", true),
        ("heartbeat", &[], "/heartbeat [on|off|secs]", "Workflow & Automation", "Configure periodic heartbeat check-ins", true),
        ("kanban", &["board"], "/kanban [board|add|move]", "Workflow & Automation", "Open or manage agent Kanban task board", true),
        ("goal", &[], "/goal <objective>", "Workflow & Automation", "Run autonomous multi-step /goal loop", true),
        ("moa", &[], "/moa <prompt>", "Workflow & Automation", "Mixture-of-Agents multi-model synthesis", true),
        ("approve", &[], "/approve <id> [once|session|permanent]", "Safety & Control", "Approve a pending dangerous command request", true),
        ("deny", &[], "/deny <id> [reason]", "Safety & Control", "Deny a pending dangerous command request", true),
        ("yolo", &[], "/yolo [on|off]", "Safety & Control", "Toggle autonomous tool approval bypass", true),
        ("permissions", &[], "/permissions", "Safety & Control", "Show pending command approvals & policy", false),
        ("queue", &[], "/queue <prompt>", "Safety & Control", "Queue follow-up prompt after active turn", true),
        ("steer", &[], "/steer <guidance>", "Safety & Control", "Steer active agent turn mid-flight", true),
        ("interrupt", &["stop"], "/interrupt", "Safety & Control", "Interrupt current agent turn or TTS", false),
        ("voice", &["talk"], "/talk", "Modes & UI", "Launch full-duplex AI Talk Mode", false),
        ("computer", &[], "/computer", "Modes & UI", "Open Remote Computer Workbench", false),
        ("browser", &[], "/browser [url]", "Modes & UI", "Inspect or control browser automation", true),
        ("mcp", &[], "/mcp [status|call]", "Tools & Skills", "Inspect connected MCP servers & tools", true),
        ("diff", &[], "/diff", "Inspection & Export", "Show workspace git diff summary", false),
        ("context", &[], "/context", "Inspection & Export", "Show context budget & @file/@dir bindings", false),
        ("verbose", &[], "/verbose [on|off]", "Modes & UI", "Toggle verbose tool call streaming", true),
        ("reasoning", &[], "/reasoning [on|off|budget]", "Model & Provider", "Configure chain-of-thought reasoning", true),
        ("debug", &[], "/debug [on|off]", "Inspection & Export", "Toggle raw turn debug telemetry", true),
        ("config", &[], "/config", "Modes & UI", "Open Hermes Agent Studio configuration", false),
        ("version", &[], "/version", "Inspection & Export", "Print Hermes Rust engine build info", false),
    ];
    let mut out = Vec::with_capacity(64);
    for (name, aliases, syntax, cat, desc, accepts_args) in entries {
        out.push(HermesSlashOption {
            name: (*name).to_string(),
            slash_name: format!("/{}", name),
            aliases: aliases.iter().map(|s| (*s).to_string()).collect(),
            syntax: (*syntax).to_string(),
            category: (*cat).to_string(),
            description: (*desc).to_string(),
            accepts_args: *accepts_args,
        });
        for alias in *aliases {
            if *alias == "?" || *alias == "h" {
                continue;
            }
            out.push(HermesSlashOption {
                name: (*alias).to_string(),
                slash_name: format!("/{}", alias),
                aliases: vec![(*name).to_string()],
                syntax: format!("/{} (alias for /{})", alias, name),
                category: (*cat).to_string(),
                description: (*desc).to_string(),
                accepts_args: *accepts_args,
            });
        }
    }
    out
}

pub fn extract_context_reference_badges(text: &str) -> Vec<(&'static str, String, u32)> {
    let mut badges = Vec::new();
    for raw in text.split_whitespace() {
        let tok = raw.trim_matches(|c: char| matches!(c, ',' | ';' | ')' | ']' | '"' | '\''));
        if let Some(rest) = tok.strip_prefix("@file") {
            if rest.is_empty() || rest.starts_with(':') {
                badges.push(("FILE", tok.to_string(), 0x2AABEE));
            }
        } else if let Some(rest) = tok.strip_prefix("@dir") {
            if rest.is_empty() || rest.starts_with(':') {
                badges.push(("DIR", tok.to_string(), 0x27AE60));
            }
        } else if tok == "@diff" || tok == "@staged" || tok.starts_with("@git:") {
            badges.push(("GIT", tok.to_string(), 0xE67E22));
        } else if let Some(rest) = tok.strip_prefix("@url") {
            if rest.is_empty() || rest.starts_with(':') {
                badges.push(("URL", tok.to_string(), 0x9B51E0));
            }
        }
    }
    badges
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorTabKind {
    Emoji,
    Stickers,
    Gifs,
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
    pub replying_to: Option<(u64, String, String)>,
    pub editing_message: Option<(u64, String)>,
    pub selector_tab: SelectorTabKind,
    pub palette: TdesktopThemePalette,
    pub lang_id: String,
    pub editor: ComposerEditorState,
    pub autocomplete: FieldAutocomplete,
    pub tabbed_selector: TabbedSelector,
    pub emoji_list: EmojiListWidget,
    pub stickers_list: StickersListWidget,
    pub gifs_list: GifsListWidget,
    pub focus_handle: Option<FocusHandle>,
    pub pending_send_silent: Option<String>,
    pub pending_schedule_message: Option<String>,
    pub pending_edit_commit: Option<(u64, String)>,
    pub pending_attached_files: Option<Vec<PathBuf>>,
    pub pending_send_sticker_emoji: Option<String>,
    pub pending_send_dice: Option<String>,
    pub pending_send_gif: Option<(u64, String)>,
    pub pending_open_poll_modal: bool,
    pub pending_open_location_modal: bool,
    pub pending_open_contact_modal: bool,
    pub hermes_commands: Vec<HermesSlashOption>,
    pub hermes_ac_visible: bool,
    pub hermes_ac_matches: Vec<HermesSlashOption>,
    pub hermes_ac_selected: usize,
    pub context_ref_hint_open: bool,
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
            replying_to: None,
            editing_message: None,
            selector_tab: SelectorTabKind::Emoji,
            palette: TdesktopThemePalette::default(),
            lang_id: "en".to_string(),
            editor: ComposerEditorState::new(),
            autocomplete: FieldAutocomplete::new(),
            tabbed_selector: TabbedSelector::new(),
            emoji_list: EmojiListWidget::new(),
            stickers_list: StickersListWidget::new(),
            gifs_list: GifsListWidget::new(),
            focus_handle: None,
            pending_send_silent: None,
            pending_schedule_message: None,
            pending_edit_commit: None,
            pending_attached_files: None,
            pending_send_sticker_emoji: None,
            pending_send_dice: None,
            pending_send_gif: None,
            pending_open_poll_modal: false,
            pending_open_location_modal: false,
            pending_open_contact_modal: false,
            hermes_commands: default_hermes_slash_commands(),
            hermes_ac_visible: false,
            hermes_ac_matches: Vec::new(),
            hermes_ac_selected: 0,
            context_ref_hint_open: false,
        }
    }

    pub fn set_hermes_commands(&mut self, descriptors: Vec<HermesCommandDescriptor>) {
        if descriptors.is_empty() {
            return;
        }
        self.hermes_commands = descriptors
            .into_iter()
            .map(|d| HermesSlashOption {
                name: d.name,
                slash_name: d.slash_name,
                aliases: d.aliases,
                syntax: d.syntax,
                category: d.category,
                description: d.description,
                accepts_args: d.accepts_args,
            })
            .collect();
        self.sync_autocomplete();
    }

    pub fn begin_reply(&mut self, msg_id: u64, sender_name: String, snippet: String) {
        self.editing_message = None;
        self.replying_to = Some((msg_id, sender_name, snippet));
    }

    pub fn begin_edit(&mut self, msg_id: u64, existing_text: String) {
        self.replying_to = None;
        self.editing_message = Some((msg_id, existing_text));
    }

    pub fn cancel_reply_or_edit(&mut self) {
        self.replying_to = None;
        self.editing_message = None;
    }

    /// Calculate effective line count considering explicit newlines and line length wrapping
    pub fn calculate_line_count(&self) -> usize {
        let text = self.ime_machine.text();
        if text.is_empty() {
            return 1;
        }
        let explicit_lines = text.split('\n').count();
        let mut total_lines = 0;
        for line in text.split('\n') {
            let char_count = line.chars().count();
            let wrapped = char_count / 48;
            total_lines += 1 + wrapped;
        }
        total_lines.max(explicit_lines).max(1)
    }

    /// Dynamically compute composer container height clamped to [44.0, 192.0]
    pub fn calculate_content_height(&self) -> Pixels {
        let lines = self.calculate_line_count();
        let computed = MIN_COMPOSER_HEIGHT + (lines.saturating_sub(1) as f32) * LINE_HEIGHT;
        px(computed.clamp(MIN_COMPOSER_HEIGHT, MAX_COMPOSER_HEIGHT))
    }

    /// Whether the text buffer exceeds maximum view bounds and requires internal scrolling
    pub fn is_overflow_scroll_active(&self) -> bool {
        self.calculate_line_count() > 8
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.ime_machine = ImeSafetyMachine::with_text(text);
    }

    pub fn text(&self) -> &str {
        self.ime_machine.text()
    }

    pub fn clear(&mut self) {
        self.ime_machine.clear();
        self.editor.selection_anchor = None;
        self.autocomplete.hide();
    }

    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        self.palette = palette;
        self.lang_id = lang_id.into();
    }

    pub fn sync_autocomplete(&mut self) {
        let cur = self.ime_machine.cursor_index;
        let text = self.ime_machine.text();
        if let Some((trigger, token, _)) = detect_autocomplete_trigger(text, cur) {
            if trigger == '/' {
                let q = token.trim_start_matches('/').to_ascii_lowercase();
                let mut exact = Vec::new();
                let mut alias = Vec::new();
                let mut substr = Vec::new();
                for cmd in &self.hermes_commands {
                    if q.is_empty() || cmd.name.starts_with(&q) {
                        exact.push(cmd.clone());
                    } else if cmd.aliases.iter().any(|a| a.to_ascii_lowercase().starts_with(&q)) {
                        alias.push(cmd.clone());
                    } else if cmd.name.contains(&q)
                        || cmd.description.to_ascii_lowercase().contains(&q)
                        || cmd.category.to_ascii_lowercase().contains(&q)
                    {
                        substr.push(cmd.clone());
                    }
                }
                exact.extend(alias);
                exact.extend(substr);
                self.hermes_ac_matches = exact;
                if self.hermes_ac_selected >= self.hermes_ac_matches.len() {
                    self.hermes_ac_selected = 0;
                }
                self.hermes_ac_visible = !self.hermes_ac_matches.is_empty();
                self.context_ref_hint_open = false;
                self.autocomplete.bot_command_rows = self
                    .hermes_ac_matches
                    .iter()
                    .take(12)
                    .map(|c| crate::views::composer::field_autocomplete::BotCommandRow {
                        command: c.slash_name.clone(),
                        description: format!("[{}] {}", c.category, c.description),
                        bot_user_id: 0,
                    })
                    .collect();
                self.autocomplete.show_filtered(&token);
            } else if trigger == '@' {
                self.hermes_ac_visible = false;
                self.hermes_ac_matches.clear();
                self.context_ref_hint_open = true;
                self.autocomplete.show_filtered(&token);
            } else if trigger == '#' {
                self.hermes_ac_visible = false;
                self.hermes_ac_matches.clear();
                self.context_ref_hint_open = false;
                self.autocomplete.show_filtered(&token);
            } else {
                self.hermes_ac_visible = false;
                self.hermes_ac_matches.clear();
                self.context_ref_hint_open = false;
                self.autocomplete.hide();
            }
        } else {
            self.hermes_ac_visible = false;
            self.hermes_ac_matches.clear();
            self.context_ref_hint_open = false;
            self.autocomplete.hide();
        }
    }

    pub fn select_hermes_autocomplete_index(&mut self, idx: usize) {
        if let Some(cmd) = self.hermes_ac_matches.get(idx).cloned() {
            let suffix = if cmd.accepts_args { " " } else { "" };
            self.replace_active_trigger_token(&format!("{}{}", cmd.slash_name, suffix));
            self.hermes_ac_visible = false;
            self.hermes_ac_matches.clear();
            self.hermes_ac_selected = 0;
        }
    }

    pub fn handle_hermes_autocomplete_nav(&mut self, key: &str, is_shift: bool) -> bool {
        if self.hermes_ac_visible && !self.hermes_ac_matches.is_empty() {
            let len = self.hermes_ac_matches.len();
            match key {
                "up" => {
                    self.hermes_ac_selected = if self.hermes_ac_selected == 0 {
                        len.saturating_sub(1)
                    } else {
                        self.hermes_ac_selected - 1
                    };
                    self.autocomplete.move_selection("Up");
                    return true;
                }
                "down" => {
                    self.hermes_ac_selected = (self.hermes_ac_selected + 1) % len;
                    self.autocomplete.move_selection("Down");
                    return true;
                }
                "tab" => {
                    let idx = self.hermes_ac_selected.min(len.saturating_sub(1));
                    self.select_hermes_autocomplete_index(idx);
                    return true;
                }
                "enter" if !is_shift => {
                    let idx = self.hermes_ac_selected.min(len.saturating_sub(1));
                    self.select_hermes_autocomplete_index(idx);
                    return true;
                }
                "escape" => {
                    self.hermes_ac_visible = false;
                    self.hermes_ac_matches.clear();
                    self.context_ref_hint_open = false;
                    self.autocomplete.hide();
                    return true;
                }
                _ => {}
            }
        } else if self.context_ref_hint_open && key == "escape" {
            self.context_ref_hint_open = false;
            self.autocomplete.hide();
            return true;
        }
        false
    }

    pub fn handle_enter(&mut self, is_shift: bool) -> KeyAction {
        self.ime_machine.handle_enter_key(is_shift)
    }

    pub fn toggle_attachments(&mut self) {
        self.attachments_drawer.toggle();
        if self.attachments_drawer.is_open {
            self.is_emoji_picker_open = false;
            self.editor.send_options_open = false;
        }
    }

    pub fn toggle_formatting(&mut self) {
        self.is_formatting_open = !self.is_formatting_open;
        self.formatting_bar.is_visible = self.is_formatting_open;
    }

    pub fn toggle_emoji_picker(&mut self) {
        self.is_emoji_picker_open = !self.is_emoji_picker_open;
        if self.is_emoji_picker_open {
            self.attachments_drawer.close();
            self.editor.send_options_open = false;
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

    fn apply_style_wrap(&mut self, style: TextStyleKind) {
        let (pre, suf) = match style {
            TextStyleKind::Bold => ("**", "**"),
            TextStyleKind::Italic => ("__", "__"),
            TextStyleKind::Underline => ("<u>", "</u>"),
            TextStyleKind::Strikethrough => ("~~", "~~"),
            TextStyleKind::Monospace => ("`", "`"),
            TextStyleKind::Spoiler => ("||", "||"),
            TextStyleKind::Link => ("[", "](https://)"),
        };
        self.editor.wrap_selection(&mut self.ime_machine, pre, suf);
        self.formatting_bar.toggle_style(style);
    }

    fn replace_active_trigger_token(&mut self, replacement: &str) {
        let cur = self.ime_machine.cursor_index;
        let text = self.ime_machine.text().to_string();
        if let Some((_trig, _tok, start)) = detect_autocomplete_trigger(&text, cur) {
            self.editor.push_undo(&self.ime_machine);
            self.ime_machine.text_buffer.replace_range(start..cur, replacement);
            self.ime_machine.cursor_index = start + replacement.len();
            self.editor.selection_anchor = None;
        } else {
            self.editor.insert_at_cursor(&mut self.ime_machine, replacement);
        }
        self.autocomplete.hide();
    }

    pub fn submit_primary(&mut self) {
        let msg = self.text().trim().to_string();
        if let Some((edit_id, _)) = self.editing_message.take() {
            if !msg.is_empty() {
                self.pending_edit_commit = Some((edit_id, msg));
            }
            self.clear();
            return;
        }
        if !msg.is_empty() {
            self.submitted_message = Some(msg);
            self.clear();
        }
        if !self.editor.attachments.is_empty() {
            let paths = std::mem::take(&mut self.editor.attachments);
            self.submitted_attachment = Some(paths[0].display().to_string());
            self.pending_attached_files = Some(paths);
        }
    }

    fn prompt_native_files(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });
        cx.spawn(async move |weak_this: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(Ok(Some(paths))) = rx.await {
                let _ = weak_this.update(app, |this, cx| {
                    if !paths.is_empty() {
                        this.submitted_attachment = Some(paths[0].display().to_string());
                        this.editor.attachments.extend(paths.clone());
                        this.pending_attached_files = Some(paths);
                        cx.notify();
                    }
                });
            }
        })
        .detach();
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let ks = &event.keystroke;
        let cmd = ks.modifiers.platform || ks.modifiers.control;
        let shift = ks.modifiers.shift;
        let alt = ks.modifiers.alt;

        if self.editor.selector_search_focused && self.is_emoji_picker_open {
            if ks.key == "escape" {
                self.editor.selector_search_focused = false;
                self.is_emoji_picker_open = false;
            } else if ks.key == "backspace" {
                let mut q = self.tabbed_selector.search_query.clone();
                q.pop();
                self.tabbed_selector.set_search_query(&q);
                self.emoji_list.search(&q);
                self.stickers_list.set_search_query(&q);
                self.gifs_list.set_search_query(&q);
            } else if !cmd {
                if let Some(ref ch) = ks.key_char {
                    let q = format!("{}{}", self.tabbed_selector.search_query, ch);
                    self.tabbed_selector.set_search_query(&q);
                    self.emoji_list.search(&q);
                    self.stickers_list.set_search_query(&q);
                    self.gifs_list.set_search_query(&q);
                } else {
                    return;
                }
            } else {
                return;
            }
            cx.notify();
            cx.stop_propagation();
            return;
        }

        if !self.is_input_focused {
            return;
        }

        if cmd {
            match ks.key.as_str() {
                "a" => self.editor.select_all(&mut self.ime_machine),
                "c" => {
                    if let Some(sel) = self.editor.selected_text(&self.ime_machine) {
                        cx.write_to_clipboard(ClipboardItem::new_string(sel.to_string()));
                    }
                }
                "x" => {
                    if let Some(sel) = self.editor.selected_text(&self.ime_machine) {
                        cx.write_to_clipboard(ClipboardItem::new_string(sel.to_string()));
                        self.editor.backspace_or_delete_sel(&mut self.ime_machine);
                    }
                }
                "v" => {
                    if let Some(clip) = cx.read_from_clipboard().and_then(|c| c.text()) {
                        self.editor.insert_at_cursor(&mut self.ime_machine, &clip);
                    }
                }
                "z" => {
                    if shift {
                        self.editor.redo(&mut self.ime_machine);
                    } else {
                        self.editor.undo(&mut self.ime_machine);
                    }
                }
                "b" => self.apply_style_wrap(TextStyleKind::Bold),
                "i" => self.apply_style_wrap(TextStyleKind::Italic),
                "u" => self.apply_style_wrap(TextStyleKind::Underline),
                "k" => self.apply_style_wrap(TextStyleKind::Link),
                "m" => self.apply_style_wrap(TextStyleKind::Monospace),
                _ => return,
            }
            self.sync_autocomplete();
            cx.notify();
            cx.stop_propagation();
            return;
        }

        if self.hermes_ac_visible && !self.hermes_ac_matches.is_empty() {
            let len = self.hermes_ac_matches.len();
            match ks.key.as_str() {
                "up" => {
                    self.hermes_ac_selected = if self.hermes_ac_selected == 0 {
                        len.saturating_sub(1)
                    } else {
                        self.hermes_ac_selected - 1
                    };
                    self.autocomplete.move_selection("Up");
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                "down" => {
                    self.hermes_ac_selected = (self.hermes_ac_selected + 1) % len;
                    self.autocomplete.move_selection("Down");
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                "tab" => {
                    let idx = self.hermes_ac_selected.min(len.saturating_sub(1));
                    self.select_hermes_autocomplete_index(idx);
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                _ => {}
            }
        } else if self.autocomplete.is_open {
            match ks.key.as_str() {
                "up" | "down" => {
                    self.autocomplete.move_selection(if ks.key == "up" { "Up" } else { "Down" });
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                "tab" => {
                    if let Some(chosen) = self.autocomplete.choose_selected(AutocompleteChooseMethod::ByTab) {
                        self.replace_active_trigger_token(&format!("{} ", chosen));
                        cx.notify();
                        cx.stop_propagation();
                        return;
                    }
                }
                _ => {}
            }
        }

        match ks.key.as_str() {
            "left" => self.editor.move_left(&mut self.ime_machine, shift, alt),
            "right" => self.editor.move_right(&mut self.ime_machine, shift, alt),
            "home" => self.editor.move_home(&mut self.ime_machine, shift),
            "end" => self.editor.move_end(&mut self.ime_machine, shift),
            "backspace" => self.editor.backspace_or_delete_sel(&mut self.ime_machine),
            "delete" => self.editor.delete_forward(&mut self.ime_machine),
            "enter" => {
                if self.hermes_ac_visible && !self.hermes_ac_matches.is_empty() && !shift {
                    let idx = self.hermes_ac_selected.min(self.hermes_ac_matches.len().saturating_sub(1));
                    self.select_hermes_autocomplete_index(idx);
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                if self.autocomplete.is_open && !shift {
                    if let Some(chosen) = self.autocomplete.choose_selected(AutocompleteChooseMethod::ByEnter) {
                        self.replace_active_trigger_token(&format!("{} ", chosen));
                        cx.notify();
                        cx.stop_propagation();
                        return;
                    }
                }
                if self.handle_enter(shift) == KeyAction::SendMessage {
                    self.submit_primary();
                }
            }
            "escape" => {
                self.editor.send_options_open = false;
                self.attachments_drawer.close();
                self.is_emoji_picker_open = false;
                self.hermes_ac_visible = false;
                self.hermes_ac_matches.clear();
                self.context_ref_hint_open = false;
                self.autocomplete.hide();
                self.cancel_reply_or_edit();
            }
            // Printable characters and IME composition flow solely through AppShell::EntityInputHandler
            // so macOS NSTextInputClient never double-inserts characters alongside ks.key_char.
            _ => return,
        }
        self.sync_autocomplete();
        cx.notify();
        cx.stop_propagation();
    }

    fn handle_bar_action(&mut self, action: ComposerBarAction, cx: &mut Context<Self>) {
        match action {
            ComposerBarAction::CancelContext => self.cancel_reply_or_edit(),
            ComposerBarAction::ClearAttachments => {
                self.editor.attachments.clear();
                self.submitted_attachment = None;
            }
            ComposerBarAction::ToggleAttachments => self.toggle_attachments(),
            ComposerBarAction::ToggleFormatting => self.toggle_formatting(),
            ComposerBarAction::ToggleEmojiPicker => self.toggle_emoji_picker(),
            ComposerBarAction::FocusInput => {
                self.is_input_focused = true;
                self.editor.selector_search_focused = false;
            }
            ComposerBarAction::SendOrSave => self.submit_primary(),
            ComposerBarAction::OpenSendOptions => {
                self.editor.send_options_open = !self.editor.send_options_open;
            }
            ComposerBarAction::StartVoice(pos) => {
                self.start_voice_recording(f32::from(pos.x), f32::from(pos.y));
                self.voice_gesture.tick(0.2, 0.45);
            }
            ComposerBarAction::MoveVoice(pos) => {
                if self.voice_gesture.is_recording() {
                    self.move_voice_recording(f32::from(pos.x), f32::from(pos.y));
                    self.voice_gesture.tick(0.05, 0.5);
                }
            }
            ComposerBarAction::EndVoice => {
                let cancelled = !self.voice_gesture.is_recording();
                self.end_voice_recording();
                if !cancelled {
                    self.submitted_attachment = Some("voice".to_string());
                }
            }
            ComposerBarAction::LockVoice => self.lock_voice_recording(),
            ComposerBarAction::CancelVoice => self.cancel_voice_recording(),
        }
        cx.notify();
    }

    fn handle_popup_action(&mut self, action: ComposerPopupAction, cx: &mut Context<Self>) {
        match action {
            ComposerPopupAction::ChooseAutocomplete(txt) => self.replace_active_trigger_token(&txt),
            ComposerPopupAction::ChooseAttachment(kind) => {
                self.attachments_drawer.close();
                match kind {
                    AttachmentKind::PhotoOrVideo
                    | AttachmentKind::DocumentOrFile
                    | AttachmentKind::AudioOrMusic => self.prompt_native_files(cx),
                    AttachmentKind::CreatePoll => self.pending_open_poll_modal = true,
                    AttachmentKind::SendLocation => self.pending_open_location_modal = true,
                    AttachmentKind::ShareContact => self.pending_open_contact_modal = true,
                }
            }
            ComposerPopupAction::ApplyFormat(style) => self.apply_style_wrap(style),
            ComposerPopupAction::SendSilent => {
                self.editor.send_options_open = false;
                let msg = self.text().trim().to_string();
                if !msg.is_empty() {
                    self.pending_send_silent = Some(msg);
                    self.clear();
                }
            }
            ComposerPopupAction::ScheduleMessage => {
                self.editor.send_options_open = false;
                let msg = self.text().trim().to_string();
                if !msg.is_empty() {
                    self.pending_schedule_message = Some(msg);
                    self.clear();
                }
            }
            ComposerPopupAction::ClosePopups => {
                self.editor.send_options_open = false;
                self.attachments_drawer.close();
                self.autocomplete.hide();
            }
        }
        cx.notify();
    }

    fn handle_selector_action(&mut self, action: SelectorUiAction, cx: &mut Context<Self>) {
        match action {
            SelectorUiAction::SwitchTab(t) => {
                self.selector_tab = match t {
                    0 => SelectorTabKind::Emoji,
                    1 => SelectorTabKind::Stickers,
                    _ => SelectorTabKind::Gifs,
                };
            }
            SelectorUiAction::SelectEmojiSection(sec) => self.emoji_list.switch_section(sec),
            SelectorUiAction::PickEmoji(em) => {
                self.editor.insert_at_cursor(&mut self.ime_machine, &em);
            }
            SelectorUiAction::PickSticker(em) => {
                self.pending_send_sticker_emoji = Some(em.clone());
                self.submitted_attachment = Some(format!("sticker:{}", em));
                self.is_emoji_picker_open = false;
            }
            SelectorUiAction::PickDice(d) => {
                self.pending_send_dice = Some(d.clone());
                self.submitted_attachment = Some(format!("dice:{}", d));
                self.is_emoji_picker_open = false;
            }
            SelectorUiAction::PickGif(id, url) => {
                self.pending_send_gif = Some((id, url.clone()));
                self.submitted_attachment = Some(format!("gif:{}", id));
                self.is_emoji_picker_open = false;
            }
            SelectorUiAction::SelectStickerSet(id) => self.stickers_list.show_sticker_set(id),
            SelectorUiAction::FocusSearch => self.editor.selector_search_focused = true,
            SelectorUiAction::ClearSearch => {
                self.tabbed_selector.set_search_query("");
                self.emoji_list.search("");
                self.stickers_list.set_search_query("");
                self.gifs_list.set_search_query("");
            }
        }
        cx.notify();
    }
}

impl Default for CanonicalComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CanonicalComposer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_handle.is_none() {
            self.focus_handle = Some(cx.focus_handle());
        }
        let focus = self.focus_handle.clone().unwrap();

        let text = self.ime_machine.text().to_string();
        let cursor = self.ime_machine.cursor_index;
        let selection = self.editor.selected_range(cursor);
        let preedit = self.ime_machine.preedit_text().map(str::to_string);
        let is_recording = self.voice_gesture.is_recording();

        let context_badges = extract_context_reference_badges(&text);
        let active_emoji_token = detect_autocomplete_trigger(&text, cursor)
            .and_then(|(ch, tok, _)| if ch == ':' { Some(tok) } else { None });
        let ac_popup = if self.hermes_ac_visible && !self.hermes_ac_matches.is_empty() {
            let pal = self.palette;
            let matches = self.hermes_ac_matches.clone();
            let total_matches = matches.len();
            let sel = self.hermes_ac_selected.min(total_matches.saturating_sub(1));
            let window_start = if total_matches > 8 && sel >= 5 {
                (sel - 4).min(total_matches - 8)
            } else {
                0
            };
            let visible_rows: Vec<(usize, HermesSlashOption)> = matches
                .into_iter()
                .enumerate()
                .skip(window_start)
                .take(8)
                .collect();
            Some(
                div()
                    .mb(px(6.0))
                    .rounded(px(12.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .p(px(6.0))
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.accent))
                                    .child(format!(
                                        "Hermes Slash Commands ({}/{})",
                                        sel + 1,
                                        total_matches
                                    )),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(pal.text_secondary))
                                    .child("↑↓ Select · Tab/Enter Complete · Esc Close"),
                            ),
                    )
                    .children(visible_rows.into_iter().map(|(idx, cmd)| {
                        let is_sel = idx == sel;
                        let cat_color = match cmd.category.as_str() {
                            "Session" => 0x3498DB,
                            "Model & Provider" => 0x9B51E0,
                            "Tools & Skills" => 0x27AE60,
                            "Memory & Persona" => 0xE67E22,
                            "Workflow & Automation" => 0x16A085,
                            "Safety & Control" => 0xE74C3C,
                            "Inspection & Export" => 0x5C7CFA,
                            _ => pal.accent,
                        };
                        div()
                            .px(px(8.0))
                            .py(px(5.0))
                            .rounded(px(8.0))
                            .bg(rgb(if is_sel { pal.input_bg } else { pal.card_bg }))
                            .when(is_sel, |d| d.border_1().border_color(rgb(pal.accent)))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.select_hermes_autocomplete_index(idx);
                                    cx.notify();
                                }),
                            )
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .min_w(px(0.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(if is_sel {
                                                pal.accent
                                            } else {
                                                pal.text_primary
                                            }))
                                            .child(cmd.slash_name.clone()),
                                    )
                                    .child(
                                        div()
                                            .px(px(5.0))
                                            .py(px(1.0))
                                            .rounded(px(4.0))
                                            .bg(rgb(pal.window_bg))
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .child(cmd.syntax.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(pal.text_secondary))
                                            .truncate()
                                            .child(cmd.description.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(6.0))
                                    .bg(TdesktopThemePalette::rgba_with_alpha(cat_color, 0.16))
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(cat_color))
                                    .child(cmd.category.clone()),
                            )
                    }))
                    .into_any_element(),
            )
        } else {
            render_autocomplete_popup(
                &self.autocomplete,
                active_emoji_token.as_deref(),
                &self.palette,
                cx,
                |this, act, cx| this.handle_popup_action(act, cx),
            )
        };
        let context_ref_strip = if !context_badges.is_empty() || self.context_ref_hint_open {
            let pal = self.palette;
            let quick_refs = [("@file:", 0x2AABEE), ("@dir:", 0x27AE60), ("@diff ", 0xE67E22), ("@url:", 0x9B51E0)];
            Some(
                div()
                    .px(px(14.0))
                    .py(px(4.0))
                    .flex()
                    .items_center()
                    .flex_wrap()
                    .gap(px(6.0))
                    .when(!context_badges.is_empty(), |row| {
                        row.child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.text_secondary))
                                .child("Context Refs:"),
                        )
                        .children(context_badges.into_iter().map(|(kind, tok, col)| {
                            div()
                                .px(px(7.0))
                                .py(px(2.0))
                                .rounded(px(999.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(col, 0.16))
                                .border_1()
                                .border_color(TdesktopThemePalette::rgba_with_alpha(col, 0.45))
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(col))
                                .child(format!("[{}] {}", kind, tok))
                        }))
                    })
                    .when(self.context_ref_hint_open, |row| {
                        row.children(quick_refs.into_iter().map(|(ins, col)| {
                            let val = ins.to_string();
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(TdesktopThemePalette::rgba_with_alpha(col, 0.5))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(col))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.replace_active_trigger_token(&val);
                                        this.context_ref_hint_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(format!("+ {}", ins.trim()))
                        }))
                    })
                    .into_any_element(),
            )
        } else {
            None
        };
        let banner = render_composer_context_banner(
            self.replying_to.as_ref(),
            self.editing_message.as_ref(),
            &self.editor.attachments,
            &self.palette,
            &self.lang_id,
            cx,
            |this, act, cx| this.handle_bar_action(act, cx),
        );

        let tab_idx = match self.selector_tab {
            SelectorTabKind::Emoji => 0,
            SelectorTabKind::Stickers => 1,
            SelectorTabKind::Gifs => 2,
        };

        div()
            .id("canonical-composer-root")
            .track_focus(&focus)
            .key_context("CanonicalComposer")
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
                this.handle_key_down(ev, cx);
            }))
            .on_mouse_move(cx.listener(|this, ev: &MouseMoveEvent, _, cx| {
                if this.voice_gesture.is_recording() && !this.voice_gesture.is_locked() {
                    this.handle_bar_action(ComposerBarAction::MoveVoice(ev.position), cx);
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _, _, cx| {
                if this.voice_gesture.is_recording() && !this.voice_gesture.is_locked() {
                    this.handle_bar_action(ComposerBarAction::EndVoice, cx);
                }
            }))
            .on_drop::<gpui::ExternalPaths>(cx.listener(|this, paths: &gpui::ExternalPaths, _, cx| {
                let files = paths.paths().to_vec();
                if !files.is_empty() {
                    this.submitted_attachment = Some(files[0].display().to_string());
                    this.editor.attachments.extend(files.clone());
                    this.pending_attached_files = Some(files);
                    cx.notify();
                }
            }))
            .relative()
            .w_full()
            .flex_shrink_0()
            .flex()
            .flex_col()
            .when_some(ac_popup, |el, pop| el.child(div().px(px(14.0)).child(pop)))
            .when(self.attachments_drawer.is_open, |el| {
                el.child(
                    div().px(px(14.0)).child(render_attachments_drawer(
                        &self.palette,
                        &self.lang_id,
                        cx,
                        |this, act, cx| this.handle_popup_action(act, cx),
                    )),
                )
            })
            .when(self.is_emoji_picker_open, |el| {
                el.child(
                    div().px(px(14.0)).flex().justify_end().child(render_tabbed_selector_popup(
                        tab_idx,
                        &self.tabbed_selector.search_query,
                        self.editor.selector_search_focused,
                        &self.emoji_list,
                        &self.stickers_list,
                        &self.gifs_list,
                        &self.palette,
                        &self.lang_id,
                        cx,
                        |this, act, cx| this.handle_selector_action(act, cx),
                    )),
                )
            })
            .when(self.editor.send_options_open, |el| {
                el.child(
                    div().px(px(14.0)).flex().justify_end().child(render_send_options_popup(
                        &self.palette,
                        &self.lang_id,
                        cx,
                        |this, act, cx| this.handle_popup_action(act, cx),
                    )),
                )
            })
            .when(self.is_formatting_open, |el| {
                el.child(
                    div().px(px(14.0)).child(render_formatting_toolbar(
                        &self.formatting_bar,
                        &self.palette,
                        cx,
                        |this, act, cx| this.handle_popup_action(act, cx),
                    )),
                )
            })
            .when_some(banner, |el, b| el.child(b))
            .when_some(context_ref_strip, |el, strip| el.child(strip))
            .child(if is_recording {
                render_voice_recording_strip(&self.voice_gesture, &self.palette, &self.lang_id, cx, |this, act, cx| {
                    this.handle_bar_action(act, cx);
                })
            } else {
                render_text_input_strip(
                    &text,
                    cursor,
                    selection,
                    preedit.as_deref(),
                    self.is_input_focused,
                    self.editing_message.is_some(),
                    !self.editor.attachments.is_empty(),
                    self.is_formatting_open,
                    self.is_emoji_picker_open,
                    self.calculate_content_height(),
                    self.is_overflow_scroll_active(),
                    &self.palette,
                    &self.lang_id,
                    cx,
                    |this, act, cx| this.handle_bar_action(act, cx),
                )
            })
    }
}
