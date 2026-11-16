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
use crate::views::composer::attachments_drawer::ComposerAttachMiniApp;
use crate::views::composer::composer_ui::{
    render_composer_context_banner, render_text_input_strip_with_bot_menu,
    render_voice_recording_strip, ComposerBarAction,
};
use crate::views::composer::editor_state::ComposerEditorState;
use crate::views::composer::popups_ui::{
    render_attachments_drawer_with_apps, render_formatting_toolbar, render_send_options_popup,
};
use crate::views::stage::view::controls::history_view_bot_menu_button::HistoryViewBotMenuButtonViewControls;
use crate::views::composer::selector_ui::render_tabbed_selector_popup;
use crate::views::composer::{
    AttachmentsDrawer, EmojiListWidget, FieldAutocomplete, FormattingBar, GifsListWidget,
    ImeSafetyMachine, KeyAction, StickersListWidget, TabbedSelector, VoiceGestureMachine,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotSlashCommandItem {
    pub command: String,
    pub description: String,
    pub bot_username: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
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
    pub hermes_slash_enabled: bool,
    pub bot_slash_commands: Vec<BotSlashCommandItem>,
    pub bot_ac_visible: bool,
    pub bot_ac_matches: Vec<BotSlashCommandItem>,
    pub bot_ac_selected: usize,
    pub(crate) bot_ac_pinned_by_button: bool,
    pub context_ref_hint_open: bool,
    pub send_on_enter: bool,
    pub suggest_stickers_by_emoji: bool,
    pub loop_animated_stickers: bool,
    pub attach_mini_apps: Vec<ComposerAttachMiniApp>,
    pub current_peer_type_mask: u8,
    pub bot_menu_controls: HistoryViewBotMenuButtonViewControls,
    pub bot_menu_entity: Option<Entity<HistoryViewBotMenuButtonViewControls>>,
    pub pending_launch_miniapp: Option<(u64, String, String, String)>,
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
            hermes_slash_enabled: false,
            bot_slash_commands: Vec::new(),
            bot_ac_visible: false,
            bot_ac_matches: Vec::new(),
            bot_ac_selected: 0,
            bot_ac_pinned_by_button: false,
            context_ref_hint_open: false,
            send_on_enter: true,
            suggest_stickers_by_emoji: true,
            loop_animated_stickers: true,
            attach_mini_apps: Vec::new(),
            current_peer_type_mask: 0,
            bot_menu_controls: HistoryViewBotMenuButtonViewControls::new(),
            bot_menu_entity: None,
            pending_launch_miniapp: None,
        }
    }

    pub fn ensure_bot_menu_entity(&mut self, cx: &mut Context<Self>) -> Entity<HistoryViewBotMenuButtonViewControls> {
        if let Some(ent) = self.bot_menu_entity.clone() {
            return ent;
        }
        let bot_id = self.bot_menu_controls.bot_id;
        let bot_username = self.bot_menu_controls.bot_username.clone();
        let text = self.bot_menu_controls.text.clone();
        let url = self.bot_menu_controls.menu_url.clone();
        let is_active = self.bot_menu_controls.is_active;
        let accent = self.palette.accent;
        let ent = cx.new(|_| {
            let mut ctrl = HistoryViewBotMenuButtonViewControls::new();
            ctrl.accent_color = accent;
            if is_active {
                ctrl.set_bot_menu(bot_id, bot_username, text, url);
            }
            ctrl
        });
        cx.observe(&ent, |comp: &mut CanonicalComposer, e: Entity<HistoryViewBotMenuButtonViewControls>, cx: &mut Context<CanonicalComposer>| {
            if let Some(launch) = e.update(cx, |m, _| m.take_launch()) {
                comp.pending_launch_miniapp = Some(launch);
                cx.notify();
            }
        }).detach();
        self.bot_menu_entity = Some(ent.clone());
        ent
    }

    pub fn set_bot_menu(
        &mut self,
        bot_id: u64,
        bot_username: impl Into<String>,
        text: impl Into<String>,
        menu_url: impl Into<String>,
    ) {
        self.bot_menu_controls.accent_color = self.palette.accent;
        self.bot_menu_controls.set_bot_menu(bot_id, bot_username, text, menu_url);
    }

    pub fn sync_bot_menu_with_cx(
        &mut self,
        bot_id: u64,
        bot_username: String,
        text: String,
        menu_url: String,
        cx: &mut Context<Self>,
    ) {
        self.set_bot_menu(bot_id, bot_username.clone(), text.clone(), menu_url.clone());
        let accent = self.palette.accent;
        let ent = self.ensure_bot_menu_entity(cx);
        ent.update(cx, |ctrl, c_cx| {
            ctrl.accent_color = accent;
            ctrl.set_bot_menu(bot_id, bot_username, text, menu_url);
            c_cx.notify();
        });
    }

    pub fn clear_bot_menu(&mut self, cx: Option<&mut Context<Self>>) {
        self.bot_menu_controls.clear_bot_menu();
        if let (Some(ent), Some(c_cx)) = (self.bot_menu_entity.clone(), cx) {
            ent.update(c_cx, |ctrl, ctrl_cx| {
                ctrl.clear_bot_menu();
                ctrl_cx.notify();
            });
        }
    }

    pub fn set_attach_mini_apps(&mut self, apps: Vec<ComposerAttachMiniApp>, peer_type_mask: u8) {
        self.attach_mini_apps = apps;
        self.current_peer_type_mask = peer_type_mask;
    }

    pub fn take_launch_miniapp(&mut self) -> Option<(u64, String, String, String)> {
        self.pending_launch_miniapp.take()
    }

    pub fn prefill_inline_bot_query(&mut self, bot_username: &str, query: &str) {
        let uname = bot_username.trim().trim_start_matches('@');
        let prefill = if query.trim().is_empty() {
            format!("@{} ", uname)
        } else {
            format!("@{} {} ", uname, query.trim())
        };
        self.set_text(prefill);
        self.is_input_focused = true;
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

    pub fn set_bot_slash_commands(&mut self, commands: Vec<BotSlashCommandItem>) {
        self.bot_slash_commands = commands;
        self.bot_ac_visible = false;
        self.bot_ac_matches.clear();
        self.bot_ac_selected = 0;
        self.bot_ac_pinned_by_button = false;
        self.sync_autocomplete();
    }

    pub fn clear_bot_slash_commands(&mut self) {
        self.bot_slash_commands.clear();
        self.bot_ac_visible = false;
        self.bot_ac_matches.clear();
        self.bot_ac_selected = 0;
        self.bot_ac_pinned_by_button = false;
        self.sync_autocomplete();
    }

    pub fn set_hermes_slash_enabled(&mut self, enabled: bool) {
        self.hermes_slash_enabled = enabled;
        if !enabled {
            self.hermes_ac_visible = false;
            self.hermes_ac_matches.clear();
            self.hermes_ac_selected = 0;
        }
        self.sync_autocomplete();
    }

    pub fn toggle_bot_commands_popup(&mut self) {
        if self.bot_slash_commands.is_empty() {
            self.bot_ac_visible = false;
            self.bot_ac_matches.clear();
            self.bot_ac_pinned_by_button = false;
            return;
        }
        if self.bot_ac_visible {
            self.bot_ac_visible = false;
            self.bot_ac_matches.clear();
            self.bot_ac_selected = 0;
            self.bot_ac_pinned_by_button = false;
        } else {
            self.bot_ac_matches = self.bot_slash_commands.clone();
            self.bot_ac_selected = 0;
            self.bot_ac_visible = true;
            self.bot_ac_pinned_by_button = true;
            self.hermes_ac_visible = false;
            self.hermes_ac_matches.clear();
            self.autocomplete.hide();
            self.is_input_focused = true;
        }
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
        self.bot_ac_visible = false;
        self.bot_ac_matches.clear();
        self.bot_ac_selected = 0;
        self.bot_ac_pinned_by_button = false;
        self.hermes_ac_visible = false;
        self.hermes_ac_matches.clear();
        self.hermes_ac_selected = 0;
        self.autocomplete.hide();
    }

    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        self.palette = palette;
        self.lang_id = lang_id.into();
    }

    pub fn handle_enter(&mut self, is_shift: bool) -> KeyAction {
        let insert_newline = if self.send_on_enter {
            is_shift
        } else {
            !is_shift
        };
        self.ime_machine.handle_enter_key(insert_newline)
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
        let ac_popup = self.render_autocomplete_overlay(&text, cursor, cx);
        let context_ref_strip = self.render_context_ref_strip(context_badges, cx);
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
                    div().px(px(14.0)).child(render_attachments_drawer_with_apps(
                        &self.attach_mini_apps,
                        self.current_peer_type_mask,
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
                let bot_menu_view = if self.bot_menu_controls.is_active {
                    Some(self.ensure_bot_menu_entity(cx).into_any_element())
                } else if !self.bot_slash_commands.is_empty() {
                    Some(self.render_bot_slash_toggle_button(cx))
                } else {
                    None
                };
                render_text_input_strip_with_bot_menu(
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
                    bot_menu_view,
                    &self.palette,
                    &self.lang_id,
                    cx,
                    |this, act, cx| this.handle_bar_action(act, cx),
                )
            })
    }
}
