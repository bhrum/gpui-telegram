//! Composer Keybindings and Formatting Shortcuts
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/global_shortcuts.h:L1-L85`
//! - `Telegram/SourceFiles/chat_helpers/message_field.cpp:L540-L580`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use super::input_field::FormattingTag;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerAction {
    SendMessage,
    InsertNewline,
    ApplyFormatting(FormattingTag),
    OpenEditLinkDialog,
    EditLastMessage,
    CancelAction,
    NavigateAutocompleteUp,
    NavigateAutocompleteDown,
    ConfirmAutocomplete,
    ToggleTabbedPanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifier {
    pub ctrl: bool,
    pub cmd: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifier {
    pub const NONE: Self = Self { ctrl: false, cmd: false, alt: false, shift: false };
    pub const CMD: Self = Self { ctrl: false, cmd: true, alt: false, shift: false };
    pub const CMD_SHIFT: Self = Self { ctrl: false, cmd: true, alt: false, shift: true };
    pub const SHIFT: Self = Self { ctrl: false, cmd: false, alt: false, shift: true };
    pub const CTRL: Self = Self { ctrl: true, cmd: false, alt: false, shift: false };
}

#[derive(Debug, Clone)]
pub struct GlobalShortcuts {
    pub send_on_enter: bool,
}

impl Default for GlobalShortcuts {
    fn default() -> Self {
        Self {
            send_on_enter: true,
        }
    }
}

impl GlobalShortcuts {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve key press into a composer shortcut action
    pub fn resolve_key(
        &self,
        key: &str,
        modifiers: KeyModifier,
        is_empty_field: bool,
        is_autocomplete_open: bool,
    ) -> Option<ComposerAction> {
        let meta = modifiers.cmd || modifiers.ctrl;

        // Autocomplete navigation intercepts Up, Down, Tab, Enter
        if is_autocomplete_open {
            match key {
                "Up" | "ArrowUp" => return Some(ComposerAction::NavigateAutocompleteUp),
                "Down" | "ArrowDown" => return Some(ComposerAction::NavigateAutocompleteDown),
                "Tab" | "Enter" if !modifiers.shift => return Some(ComposerAction::ConfirmAutocomplete),
                "Escape" => return Some(ComposerAction::CancelAction),
                _ => {}
            }
        }

        // Formatting Shortcuts (Cmd / Ctrl based)
        if meta && !modifiers.alt {
            match (key, modifiers.shift) {
                ("b" | "B", false) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Bold)),
                ("i" | "I", false) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Italic)),
                ("u" | "U", false) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Underline)),
                ("x" | "X", true) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Strike)),
                ("m" | "M", true) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Monospace)),
                ("p" | "P", true) => return Some(ComposerAction::ApplyFormatting(FormattingTag::Spoiler)),
                ("k" | "K", false) => return Some(ComposerAction::OpenEditLinkDialog),
                _ => {}
            }
        }

        // Send vs Newline
        if key == "Enter" || key == "Return" {
            if modifiers.shift {
                return Some(ComposerAction::InsertNewline);
            } else if self.send_on_enter {
                return Some(ComposerAction::SendMessage);
            } else {
                return Some(ComposerAction::InsertNewline);
            }
        }

        // Edit last message
        if (key == "Up" || key == "ArrowUp") && is_empty_field && modifiers == KeyModifier::NONE {
            return Some(ComposerAction::EditLastMessage);
        }

        // Escape cancels edit or closes panel
        if key == "Escape" {
            return Some(ComposerAction::CancelAction);
        }

        None
    }
}
