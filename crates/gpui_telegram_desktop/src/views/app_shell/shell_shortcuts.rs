//! Keyboard shortcuts, window key dispatch, inspector toggle, language toggle, and nav switching.

use gpui::{Context, KeyDownEvent, Window};

use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::global_tasks_workspace::TasksWorkspaceTab;
use crate::views::nav_rail::NavDestination;
use crate::views::settings::settings_type::SettingsSectionType;
use crate::views::shell::{
    nav_destination_for_shortcut_digit, next_tdesktop_theme_cycle, toggled_language_code,
    ActiveWorkspacePanel, AppShell,
};
use crate::views::tdesktop_theme::tr_str;

impl AppShell {
    /// Synchronizes `ContextInspector` open state with `ConversationStage::is_inspector_open`.
    pub fn set_inspector_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.inspector.update(cx, |insp, insp_cx| {
            if open {
                insp.open();
            } else {
                insp.close();
            }
            insp_cx.notify();
        });
        self.stage.update(cx, |st, st_cx| {
            st.is_inspector_open = open;
            st_cx.notify();
        });
        cx.notify();
    }

    /// Toggles the active interface language (`zh-CN` <-> `en-US`) and persists preferences.
    pub fn toggle_language(&mut self, cx: &mut Context<Self>) {
        let next_lang = toggled_language_code(&self.preferences.localization.language_code);
        self.update_preferences(cx, |p| {
            p.localization.language_code = next_lang.to_string();
        });
    }

    /// Switches to a `NavDestination` panel and synchronizes `NavRail` + workspace data cleanly.
    pub fn activate_nav_destination(&mut self, dest: NavDestination, cx: &mut Context<Self>) {
        self.last_handled_nav_dest = Some(dest);
        self.nav_rail.update(cx, |r, r_cx| {
            r.set_active_destination(dest);
            r_cx.notify();
        });
        match dest {
            NavDestination::Chats | NavDestination::Folders | NavDestination::SavedMessages => {
                self.active_panel = ActiveWorkspacePanel::Chats;
                let target_saved = if dest == NavDestination::SavedMessages {
                    let sb = self.sidebar.read(cx);
                    sb.saved_messages_id.or_else(|| {
                        let uid = self.session_sync.user_id?;
                        sb.dialogs.iter().find(|d| d.id == uid).map(|d| d.id)
                    })
                } else {
                    None
                };
                self.sidebar.update(cx, |sb, sb_cx| {
                    match dest {
                        NavDestination::Chats => sb.select_folder("All"),
                        NavDestination::Folders => {
                            if let Some(first_folder) = sb
                                .folders
                                .iter()
                                .find(|(f, _)| f != "All")
                                .map(|(f, _)| f.clone())
                            {
                                sb.select_folder(first_folder);
                            }
                        }
                        NavDestination::SavedMessages => {
                            if let Some(saved_id) = target_saved {
                                sb.select_dialog(saved_id);
                            }
                        }
                        _ => {}
                    }
                    sb_cx.notify();
                });
                if let Some(saved_id) = target_saved {
                    self.sync_selected_chat_inspector_and_stage(saved_id, cx);
                }
            }
            NavDestination::Calls => {
                let shell = &mut *self;
                shell.active_panel = ActiveWorkspacePanel::Calls;
                shell.sync_calls_history(cx);
            }
            NavDestination::TalkMode => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(false, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    let name = self.stage.read(cx).chat_title.clone();
                    self.start_webrtc_call(pid, name, false, cx);
                } else {
                    self.show_talk_mode(cx);
                }
            }
            NavDestination::RemoteComputer => {
                self.open_computer_workbench_live(cx);
            }
            NavDestination::TaskCenter => {
                self.active_panel = ActiveWorkspacePanel::TaskCenter;
                self.tasks_workspace
                    .update(cx, |w, _| w.set_tab(TasksWorkspaceTab::Tasks));
                self.refresh_tasks_and_mcp(cx);
            }
            NavDestination::McpTools => {
                self.active_panel = ActiveWorkspacePanel::McpTools;
                self.tasks_workspace
                    .update(cx, |w, _| w.set_tab(TasksWorkspaceTab::McpTools));
                self.refresh_tasks_and_mcp(cx);
            }
            NavDestination::Settings => {
                if self.active_panel == ActiveWorkspacePanel::Settings {
                    self.settings_router.update(cx, |sr, scx| {
                        sr.settings_stack.clear();
                        sr.navigate_to(SettingsSectionType::Main, false, scx);
                    });
                } else {
                    self.navigate_settings_to(SettingsSectionType::Main, cx);
                }
            }
        }
        if matches!(
            dest,
            NavDestination::Calls
                | NavDestination::TaskCenter
                | NavDestination::McpTools
                | NavDestination::Settings
        ) {
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.is_plus_menu_open = false;
                sb.is_account_popover_open = false;
                sb_cx.notify();
            });
            self.stage.update(cx, |st, st_cx| {
                st.more_menu_open = false;
                st.ttl_submenu_open = false;
                st_cx.notify();
            });
        }
        cx.notify();
    }

    /// Processes Cmd/Ctrl-prefixed shell and workspace keyboard shortcuts.
    pub fn handle_shortcut_key(&mut self, key: &str, is_shift: bool, cx: &mut Context<Self>) -> bool {
        let key_lower = key.to_ascii_lowercase();
        if self.is_authenticated && self.overlays.is_passcode_locked {
            match key_lower.as_str() {
                "v" => {
                    if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                        self.insert_committed_text(&clip_text, cx);
                    }
                    return true;
                }
                "backspace" | "a" | "u" => {
                    self.clear_active_field(cx);
                    return true;
                }
                _ => return true,
            }
        }
        if self.is_authenticated {
            self.record_user_activity();
        }
        match key_lower.as_str() {
            "v" => {
                if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                    self.insert_committed_text(&clip_text, cx);
                }
                return true;
            }
            "backspace" | "a" | "u" => {
                self.clear_active_field(cx);
                return true;
            }
            "d" => {
                let next = next_tdesktop_theme_cycle(self.theme_state.theme_mode);
                self.apply_theme_selection(next, None, None, cx);
                return true;
            }
            "l" => {
                if is_shift && self.is_authenticated && self.has_local_passcode_configured() {
                    self.lock_app_now(cx);
                } else {
                    self.toggle_language(cx);
                }
                return true;
            }
            _ => {}
        }

        if !self.is_authenticated || !self.preferences.shortcuts_enabled {
            return false;
        }

        let lang = self.active_lang_id();
        match key_lower.as_str() {
            "k" => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.focus_search();
                    sb_cx.notify();
                });
                self.composer.update(cx, |c, c_cx| {
                    c.is_input_focused = false;
                    c_cx.notify();
                });
                cx.notify();
                true
            }
            "," if self.is_authenticated => {
                self.navigate_settings_to(SettingsSectionType::Main, cx);
                true
            }
            "[" if self.active_panel == ActiveWorkspacePanel::Settings => {
                self.navigate_settings_back(cx);
                true
            }
            "i" => {
                let next_open = !self.inspector.read(cx).is_open;
                self.set_inspector_open(next_open, cx);
                true
            }
            "b" | "\\" => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.toggle_collapsed_from_user();
                    sb_cx.notify();
                });
                true
            }
            "e" => {
                self.sidebar.update(cx, |s, s_cx| {
                    s.is_account_popover_open = !s.is_account_popover_open;
                    s.is_plus_menu_open = false;
                    s_cx.notify();
                });
                cx.notify();
                true
            }
            "j" => {
                self.refresh_contacts(cx);
                self.open_modal_dialog(
                    tr_str(lang, "联系人", "Contacts"),
                    ModalDialogKind::Contacts,
                    420.0,
                    500.0,
                    cx,
                );
                true
            }
            "n" => {
                if is_shift {
                    self.open_modal_dialog(
                        tr_str(lang, "新建频道", "New Channel"),
                        ModalDialogKind::NewChannel,
                        420.0,
                        380.0,
                        cx,
                    );
                } else {
                    self.open_modal_dialog(
                        tr_str(lang, "新建群组", "New Group"),
                        ModalDialogKind::NewGroup,
                        420.0,
                        380.0,
                        cx,
                    );
                }
                true
            }
            "p" => {
                self.open_modal_dialog(
                    tr_str(lang, "创建投票", "Create Poll"),
                    ModalDialogKind::CreatePoll,
                    420.0,
                    480.0,
                    cx,
                );
                true
            }
            digit => {
                if let Some(dest) = nav_destination_for_shortcut_digit(digit) {
                    self.activate_nav_destination(dest, cx);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Handles window-level key down events without swallowing printable IME character input.
    pub fn handle_window_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();

        // 1. Handle Cmd / Ctrl shortcuts (Theme, Language, Navigation, Modals, Search, Paste)
        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            if self.handle_shortcut_key(key, event.keystroke.modifiers.shift, cx) {
                cx.stop_propagation();
            }
            return;
        }

        // 2. Handle non-character control keys (Enter, Backspace/Alt+Backspace, Tab, Escape)
        if self.handle_non_char_key_with_alt(
            key,
            event.keystroke.modifiers.shift,
            event.keystroke.modifiers.alt,
            cx,
        ) {
            cx.stop_propagation();
        }
        // 3. Printable characters are NOT stopped here so GPUI's macOS NSTextInputClient
        //    routes both English keys (`insertText:`) and Chinese Pinyin (`setMarkedText:`)
        //    through `EntityInputHandler` without duplication.
    }

    /// Dispatches a synthetic or view-local `KeyDownEvent` (used by tests and fallback key handlers).
    pub fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();

        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            self.handle_shortcut_key(key, event.keystroke.modifiers.shift, cx);
            return;
        }

        if self.handle_non_char_key_with_alt(
            key,
            event.keystroke.modifiers.shift,
            event.keystroke.modifiers.alt,
            cx,
        ) {
            return;
        }

        if key == "space" {
            self.insert_committed_text(" ", cx);
        } else if let Some(ref ch) = event.keystroke.key_char {
            self.insert_committed_text(ch, cx);
        }
    }
}
