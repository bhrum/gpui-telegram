//! macOS Native Event Filter and Key Typing Classifier
//! Upstream: Telegram/SourceFiles/platform/mac/native_event_mac.h, Telegram/SourceFiles/platform/mac/native_event_mac.mm
//! Evaluates NSEvent keystrokes to identify genuine text typing vs system hotkeys.

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// macOS NSEvent modifier bitflags matching AppKit constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacModifierFlags(pub u32);

impl MacModifierFlags {
    pub const ALPHA_SHIFT: u32 = 1 << 16;
    pub const SHIFT: u32 = 1 << 17;
    pub const CONTROL: u32 = 1 << 18;
    pub const ALTERNATE: u32 = 1 << 19;
    pub const COMMAND: u32 = 1 << 20;
    pub const NUMERIC_PAD: u32 = 1 << 21;
    pub const HELP: u32 = 1 << 22;
    pub const FUNCTION: u32 = 1 << 23;

    pub const DEVICE_INDEPENDENT_FLAGS_MASK: u32 = 0xffff0000;

    pub fn new(flags: u32) -> Self {
        Self(flags)
    }

    pub fn device_independent(self) -> u32 {
        self.0 & Self::DEVICE_INDEPENDENT_FLAGS_MASK
    }

    pub fn has_command(self) -> bool {
        (self.0 & Self::COMMAND) != 0
    }

    pub fn has_control(self) -> bool {
        (self.0 & Self::CONTROL) != 0
    }

    pub fn has_alternate(self) -> bool {
        (self.0 & Self::ALTERNATE) != 0
    }

    pub fn has_shift(self) -> bool {
        (self.0 & Self::SHIFT) != 0
    }
}

/// Simulated or native macOS key down event representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacKeyEvent {
    pub event_type: u32,
    pub key_code: u16,
    pub characters: String,
    pub characters_ignoring_modifiers: String,
    pub modifiers: MacModifierFlags,
    pub is_repeat: bool,
}

impl MacKeyEvent {
    pub const NS_EVENT_TYPE_KEY_DOWN: u32 = 10;
    pub const NS_EVENT_TYPE_KEY_UP: u32 = 11;
    pub const NS_EVENT_TYPE_FLAGS_CHANGED: u32 = 12;

    pub fn new_key_down(characters: impl Into<String>, modifiers: u32) -> Self {
        let chars = characters.into();
        Self {
            event_type: Self::NS_EVENT_TYPE_KEY_DOWN,
            key_code: 0,
            characters: chars.clone(),
            characters_ignoring_modifiers: chars,
            modifiers: MacModifierFlags::new(modifiers),
            is_repeat: false,
        }
    }
}

/// Native macOS event analyzer and typing event evaluator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeEventMacMac {
    pub processed_typing_events: usize,
    pub ignored_shortcut_events: usize,
    pub shift_only_allowed: bool,
}

impl NativeEventMacMac {
    pub fn new() -> Self {
        Self {
            processed_typing_events: 0,
            ignored_shortcut_events: 0,
            shift_only_allowed: true,
        }
    }

    /// Evaluates whether an event is a genuine text typing event (and not a hotkey shortcut).
    pub fn possibly_text_typing_event(&mut self, event: &MacKeyEvent) -> Result<bool> {
        if event.event_type != MacKeyEvent::NS_EVENT_TYPE_KEY_DOWN {
            return Ok(false);
        }

        let flags = event.modifiers.device_independent();
        if self.shift_only_allowed && (flags & !MacModifierFlags::SHIFT) != 0 {
            self.ignored_shortcut_events += 1;
            return Ok(false);
        }

        for ch in event.characters.chars() {
            let u16_val = ch as u32;
            if u16_val >= 32 && ch != '?' {
                self.processed_typing_events += 1;
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn reset_metrics(&mut self) {
        self.processed_typing_events = 0;
        self.ignored_shortcut_events = 0;
    }
}

impl Default for NativeEventMacMac {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NativeEventMacMac {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(rgb(0x1E293B))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0xF8FAFC))
                            .child("macOS Native Key Event Filter"),
                    ),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x94A3B8))
                    .child(format!("Typing: {} | Shortcuts: {}", self.processed_typing_events, self.ignored_shortcut_events)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typing_event_detection() {
        let mut analyzer = NativeEventMacMac::new();

        // Plain character 'a'
        let evt_plain = MacKeyEvent::new_key_down("a", 0);
        assert_eq!(analyzer.possibly_text_typing_event(&evt_plain).unwrap(), true);

        // Shifted character 'A'
        let evt_shift = MacKeyEvent::new_key_down("A", MacModifierFlags::SHIFT);
        assert_eq!(analyzer.possibly_text_typing_event(&evt_shift).unwrap(), true);

        // Command + S (shortcut)
        let evt_cmd = MacKeyEvent::new_key_down("s", MacModifierFlags::COMMAND);
        assert_eq!(analyzer.possibly_text_typing_event(&evt_cmd).unwrap(), false);

        // Control character (tab, newline)
        let evt_ctrl_char = MacKeyEvent::new_key_down("\t", 0);
        assert_eq!(analyzer.possibly_text_typing_event(&evt_ctrl_char).unwrap(), false);

        assert_eq!(analyzer.processed_typing_events, 2);
        assert_eq!(analyzer.ignored_shortcut_events, 1);
    }
}
