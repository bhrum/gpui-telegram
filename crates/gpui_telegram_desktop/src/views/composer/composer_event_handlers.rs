//! Event, keyboard, autocomplete, action handlers, and Hermes overlay builders for `CanonicalComposer`.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::theme::TdesktopThemePalette;
use crate::views::canonical_composer::{CanonicalComposer, HermesSlashOption, SelectorTabKind};
use crate::views::composer::composer_ui::ComposerBarAction;
use crate::views::composer::editor_state::detect_autocomplete_trigger;
use crate::views::composer::field_autocomplete::{AutocompleteChooseMethod, BotCommandRow};
use crate::views::composer::popups_ui::{render_autocomplete_popup, ComposerPopupAction};
use crate::views::composer::selector_ui::SelectorUiAction;
use crate::views::composer::{AttachmentKind, KeyAction, TextStyleKind};

impl CanonicalComposer {
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
                    .map(|c| BotCommandRow {
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
            let trimmed = text.trim();
            if self.suggest_stickers_by_emoji
                && !trimmed.is_empty()
                && trimmed.chars().count() <= 3
                && trimmed.chars().all(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace())
            {
                let mut rows: Vec<crate::views::composer::field_autocomplete::StickerRow> = self
                    .stickers_list
                    .recent_stickers
                    .iter()
                    .chain(self.stickers_list.favorite_stickers.iter())
                    .filter(|s| s.alt_emoticon == trimmed)
                    .map(|s| crate::views::composer::field_autocomplete::StickerRow {
                        document_id: if s.document_id != 0 { s.document_id } else { s.id },
                        emoji: s.alt_emoticon.clone(),
                    })
                    .collect();
                if rows.is_empty() {
                    rows.push(crate::views::composer::field_autocomplete::StickerRow {
                        document_id: 1,
                        emoji: trimmed.to_string(),
                    });
                }
                self.autocomplete.sticker_rows = rows;
                self.autocomplete.autocomplete_type =
                    crate::views::composer::field_autocomplete::AutocompleteType::Stickers;
                self.autocomplete.filter = trimmed.to_string();
                self.autocomplete.is_open = true;
                self.autocomplete.selected_index = 0;
            } else {
                self.autocomplete.hide();
            }
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

    pub(crate) fn replace_active_trigger_token(&mut self, replacement: &str) {
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

    pub(crate) fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
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

    pub(crate) fn handle_bar_action(&mut self, action: ComposerBarAction, cx: &mut Context<Self>) {
        match action {
            ComposerBarAction::CancelContext => self.cancel_reply_or_edit(),
            ComposerBarAction::ClearAttachments => {
                self.editor.attachments.clear();
                self.submitted_attachment = None;
            }
            ComposerBarAction::ToggleAttachments => self.toggle_attachments(),
            ComposerBarAction::ToggleFormatting => self.toggle_formatting(),
            ComposerBarAction::ToggleEmojiPicker => self.toggle_emoji_picker(),
            ComposerBarAction::OpenBotMenu => {
                if self.bot_menu_controls.is_active {
                    self.pending_launch_miniapp = Some((
                        self.bot_menu_controls.bot_id,
                        self.bot_menu_controls.bot_username.clone(),
                        self.bot_menu_controls.text.clone(),
                        self.bot_menu_controls.menu_url.clone(),
                    ));
                }
            }
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

    pub(crate) fn handle_popup_action(&mut self, action: ComposerPopupAction, cx: &mut Context<Self>) {
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
            ComposerPopupAction::ChooseMiniApp(app) => {
                self.attachments_drawer.close();
                self.pending_launch_miniapp = Some((
                    app.bot_id,
                    app.bot_username,
                    app.title,
                    app.webview_url,
                ));
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

    pub(crate) fn handle_selector_action(&mut self, action: SelectorUiAction, cx: &mut Context<Self>) {
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

    pub(crate) fn render_autocomplete_overlay(
        &self,
        text: &str,
        cursor: usize,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let active_emoji_token = if self.suggest_stickers_by_emoji {
            detect_autocomplete_trigger(text, cursor)
                .and_then(|(ch, tok, _)| if ch == ':' { Some(tok) } else { None })
        } else {
            None
        };
        if self.hermes_ac_visible && !self.hermes_ac_matches.is_empty() {
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
        }
    }

    pub(crate) fn render_context_ref_strip(
        &self,
        context_badges: Vec<(&'static str, String, u32)>,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if context_badges.is_empty() && !self.context_ref_hint_open {
            return None;
        }
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
    }
}
