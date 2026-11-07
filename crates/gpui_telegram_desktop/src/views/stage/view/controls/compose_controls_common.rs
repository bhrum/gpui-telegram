//! Compose controls shared types and interface contracts.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/compose_controls_common.h`

use gpui::*;
use serde::{Deserialize, Serialize};

pub const OPTION_MAC_CMD_REPLY_IMMEDIATELY: &str = "MacCmdReplyImmediately";

/// Send progress notification type sent over MTProto
/// Upstream: `Api::SendProgressType` (`compose_controls_common.h:L14`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SendProgressType {
    Typing,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    RecordRound,
    UploadRound,
    ChooseSticker,
    Cancel,
}

impl Default for SendProgressType {
    fn default() -> Self {
        Self::Typing
    }
}

/// Message edition payload
/// Upstream: `struct MessageToEdit` (`compose_controls_common.h:L28-34`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageToEdit {
    pub full_id: (i64, i32), // (peer_id, msg_id)
    pub text: String,
    pub spoilered: bool,
    pub invert_media: bool,
}

/// Voice or video note payload to send
/// Upstream: `struct VoiceToSend` (`compose_controls_common.h:L35-41`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceToSend {
    pub bytes: Vec<u8>,
    pub waveform: Vec<u8>,
    pub duration_ms: u64,
    pub is_video: bool,
    pub view_once: bool,
}

/// Send action update notification
/// Upstream: `struct SendActionUpdate` (`compose_controls_common.h:L42-46`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendActionUpdate {
    pub progress_type: SendProgressType,
    pub progress: i32,
    pub cancel: bool,
}

/// Write restriction classification
/// Upstream: `enum class WriteRestrictionType` (`compose_controls_common.h:L48-54`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteRestrictionType {
    None,
    Rights,
    PremiumRequired,
    Frozen,
    Hidden,
}

impl Default for WriteRestrictionType {
    fn default() -> Self {
        Self::None
    }
}

/// Channel/Chat write restriction state
/// Upstream: `struct WriteRestriction` (`compose_controls_common.h:L56-74`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteRestriction {
    pub text: String,
    pub button: String,
    pub restriction_type: WriteRestrictionType,
    pub boosts_to_lift: i32,
}

impl Default for WriteRestriction {
    fn default() -> Self {
        Self {
            text: String::new(),
            button: String::new(),
            restriction_type: WriteRestrictionType::None,
            boosts_to_lift: 0,
        }
    }
}

impl WriteRestriction {
    pub fn is_empty(&self) -> bool {
        self.restriction_type == WriteRestrictionType::None
    }
}

/// Direction for iterating through consecutive replies
/// Upstream: `struct ReplyNextRequest` (`compose_controls_common.h:L93-100`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplyDirection {
    Next,
    Previous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyNextRequest {
    pub reply_msg_id: i32,
    pub direction: ReplyDirection,
}

/// Toggle comments button state in forum channels
/// Upstream: `enum class ToggleCommentsState` (`compose_controls_common.h:L102-107`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToggleCommentsState {
    Empty,
    Shown,
    Hidden,
    WithNew,
}

impl Default for ToggleCommentsState {
    fn default() -> Self {
        Self::Empty
    }
}

/// Star reaction effect payload
/// Upstream: `struct SendStarButtonEffect` (`compose_controls_common.h:L109-112`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendStarButtonEffect {
    pub from_peer_id: i64,
    pub stars: i32,
}

/// Shared compose controls state wrapper and GPUI view
pub struct ComposeControlsCommonViewControls {
    pub send_progress: SendProgressType,
    pub restriction: WriteRestriction,
    pub comments_state: ToggleCommentsState,
    pub slowmode_seconds: i32,
    pub can_send_texts: bool,
    pub is_active: bool,
}

impl ComposeControlsCommonViewControls {
    pub fn new() -> Self {
        Self {
            send_progress: SendProgressType::Typing,
            restriction: WriteRestriction::default(),
            comments_state: ToggleCommentsState::Empty,
            slowmode_seconds: 0,
            can_send_texts: true,
            is_active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_restriction(&mut self, restriction: WriteRestriction) {
        self.restriction = restriction;
    }

    pub fn set_slowmode(&mut self, seconds: i32) {
        self.slowmode_seconds = seconds;
    }
}

impl Default for ComposeControlsCommonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ComposeControlsCommonViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_restricted = !self.restriction.is_empty();

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .px_3()
            .py_1()
            .bg(if is_restricted {
                rgb(0x2d1a1a)
            } else {
                rgb(0x18222d)
            })
            .text_xs()
            .text_color(if is_restricted {
                rgb(0xff5252)
            } else {
                rgb(0x8e8e93)
            })
            .child(if is_restricted {
                self.restriction.text.clone()
            } else if self.slowmode_seconds > 0 {
                format!("Slowmode: {}s left", self.slowmode_seconds)
            } else {
                "Compose common ready".to_string()
            })
    }
}
