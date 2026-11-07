//! macOS Root TouchBar Coordinator and Manager
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// High-level RootTouchBar target selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootTouchBarTarget {
    Blank,
    AudioPlayer,
    MainChat,
}

/// Root macOS TouchBar manager and GPUI Render dispatcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarManagerMacTouchbar {
    pub has_active_session: bool,
    pub is_passcode_locked: bool,
    pub is_audio_playing: bool,
    pub is_voice_recording: bool,
    pub active_target: RootTouchBarTarget,
}

impl MacTouchbarManagerMacTouchbar {
    pub const MAIN_ITEM_ID: &'static str = "touchbarMain";
    pub const AUDIO_ITEM_ID: &'static str = "touchbarAudio";

    pub fn new() -> Self {
        Self {
            has_active_session: true,
            is_passcode_locked: false,
            is_audio_playing: false,
            is_voice_recording: false,
            active_target: RootTouchBarTarget::MainChat,
        }
    }

    /// Evaluates current active touchbar based on 4 upstream inputs.
    pub fn evaluate_state(&mut self) -> RootTouchBarTarget {
        let target = if self.is_passcode_locked || self.is_voice_recording {
            RootTouchBarTarget::Blank
        } else if self.is_audio_playing {
            RootTouchBarTarget::AudioPlayer
        } else if self.has_active_session {
            RootTouchBarTarget::MainChat
        } else {
            RootTouchBarTarget::Blank
        };
        self.active_target = target;
        target
    }

    pub fn set_passcode_lock(&mut self, locked: bool) -> Result<()> {
        self.is_passcode_locked = locked;
        self.evaluate_state();
        Ok(())
    }

    pub fn set_audio_playing(&mut self, playing: bool) -> Result<()> {
        self.is_audio_playing = playing;
        self.evaluate_state();
        Ok(())
    }

    pub fn set_voice_recording(&mut self, recording: bool) -> Result<()> {
        self.is_voice_recording = recording;
        self.evaluate_state();
        Ok(())
    }

    pub fn set_has_active_session(&mut self, has_session: bool) -> Result<()> {
        self.has_active_session = has_session;
        self.evaluate_state();
        Ok(())
    }

    pub fn active_item_identifiers(&self) -> Vec<&'static str> {
        match self.active_target {
            RootTouchBarTarget::Blank => vec![],
            RootTouchBarTarget::AudioPlayer => vec![Self::AUDIO_ITEM_ID],
            RootTouchBarTarget::MainChat => vec![Self::MAIN_ITEM_ID],
        }
    }
}

impl Default for MacTouchbarManagerMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacTouchbarManagerMacTouchbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.active_target {
            RootTouchBarTarget::Blank => "TouchBar: [Inactive / Locked]",
            RootTouchBarTarget::AudioPlayer => "TouchBar: [Music Player Bar]",
            RootTouchBarTarget::MainChat => "TouchBar: [Main Chat Controls]",
        };

        div()
            .h(px(30.0))
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x121212))
            .rounded_md()
            .text_xs()
            .text_color(rgb(0x8e8e93))
            .child(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_touchbar_dispatch_logic() {
        let mut mgr = MacTouchbarManagerMacTouchbar::new();
        assert_eq!(mgr.evaluate_state(), RootTouchBarTarget::MainChat);
        assert_eq!(mgr.active_item_identifiers(), vec!["touchbarMain"]);

        // When audio starts playing, switch to audio bar
        mgr.set_audio_playing(true).unwrap();
        assert_eq!(mgr.active_target, RootTouchBarTarget::AudioPlayer);
        assert_eq!(mgr.active_item_identifiers(), vec!["touchbarAudio"]);

        // When passcode lock turns on, blank out
        mgr.set_passcode_lock(true).unwrap();
        assert_eq!(mgr.active_target, RootTouchBarTarget::Blank);
        assert!(mgr.active_item_identifiers().is_empty());

        // When unlocked and voice recording active, blank out
        mgr.set_passcode_lock(false).unwrap();
        mgr.set_voice_recording(true).unwrap();
        assert_eq!(mgr.active_target, RootTouchBarTarget::Blank);

        // When recording ends, returns to audio player if still playing
        mgr.set_voice_recording(false).unwrap();
        assert_eq!(mgr.active_target, RootTouchBarTarget::AudioPlayer);
    }
}
