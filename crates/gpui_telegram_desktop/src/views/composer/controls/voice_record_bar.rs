//! Voice recording bar component with gesture tracking, duration timer, waveform display, and view-once toggle.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp`
//! Style reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Update interval for voice recording duration and audio level (ms)
/// Upstream: `constexpr auto kAudioVoiceUpdateView = crl::time(200);` (line 60)
pub const AUDIO_VOICE_UPDATE_VIEW_MS: u64 = 200;

/// Maximum recording duration in seconds (100 minutes = 6,000 seconds)
/// Upstream: `constexpr auto kAudioVoiceMaxLength = 100 * 60;` (line 61)
pub const AUDIO_VOICE_MAX_LENGTH_SECS: u64 = 6000;

/// Threshold ratio above which upward vertical drag immediately triggers the lock animation
/// Upstream: `constexpr auto kAnimationThreshold = 0.35;` (line 2690)
pub const ANIMATION_THRESHOLD: f32 = 0.35;

/// Lock arc curvature angle in degrees
/// Upstream: `constexpr auto kLockArcAngle = 15.;` (line 69)
pub const LOCK_ARC_ANGLE_DEG: f32 = 15.0;

/// Normalized amplitude level quantization precision
/// Upstream: `constexpr auto kPrecision = 10;` (line 67)
pub const LEVEL_PRECISION: u16 = 10;

/// Boundary tolerance for trimming audio playback
/// Upstream: `constexpr auto kTrimPlaybackEpsilon = 0.0001;` (line 72)
pub const TRIM_PLAYBACK_EPSILON: f64 = 0.0001;

/// Style metrics from `history_view_voice_record_bar.style`
pub const SIGNAL_RADIUS_PX: f32 = 5.0;
pub const DURATION_SKIP_PX: f32 = 12.0;
pub const LOCK_WIDTH_PX: f32 = 75.0;
pub const LOCK_HEIGHT_PX: f32 = 133.0;
pub const LOCK_ICON_WIDTH_PX: f32 = 14.0;
pub const LOCK_ICON_HEIGHT_PX: f32 = 17.0;
pub const CANCEL_BUTTON_WIDTH_PX: f32 = 100.0;
pub const WAVEFORM_BG_RADIUS_PX: f32 = 7.0;
pub const WAVEFORM_BAR_WIDTH_PX: f32 = 3.0;
pub const SLIDE_CANCEL_THRESHOLD_PX: f32 = -80.0;

/// Voice recording state machine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoiceRecordState {
    Idle,
    Holding {
        duration_ms: u64,
        is_video: bool,
        level: u16,
        samples: Vec<u8>,
        drag_x: f32,
        drag_y: f32,
        lock_progress: f32,
    },
    Locked {
        duration_ms: u64,
        is_video: bool,
        is_paused: bool,
        level: u16,
        samples: Vec<u8>,
    },
    Listening {
        duration_ms: u64,
        playback_pos: f64,
        trim_start: f64,
        trim_end: f64,
        samples: Vec<u8>,
    },
    Sent {
        duration_ms: u64,
        waveform: Vec<u8>,
        is_video: bool,
        view_once: bool,
    },
    Discarded {
        reason: String,
    },
}

impl Default for VoiceRecordState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Stop action requested for active voice recording
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopType {
    Cancel,
    Send,
    Listen,
}

/// Authentic Voice Record Bar Component
pub struct VoiceRecordBar {
    pub state: VoiceRecordState,
    pub is_video: bool,
    pub view_once: bool,
    pub lock_height_px: f32,
    pub cancel_threshold_px: f32,
    pub max_duration_secs: u64,
}

impl VoiceRecordBar {
    pub fn new() -> Self {
        Self {
            state: VoiceRecordState::Idle,
            is_video: false,
            view_once: false,
            lock_height_px: LOCK_HEIGHT_PX,
            cancel_threshold_px: SLIDE_CANCEL_THRESHOLD_PX,
            max_duration_secs: AUDIO_VOICE_MAX_LENGTH_SECS,
        }
    }

    /// Start holding down to record (voice or round video note)
    pub fn start_holding(&mut self, is_video: bool) {
        self.is_video = is_video;
        self.state = VoiceRecordState::Holding {
            duration_ms: 0,
            is_video,
            level: 0,
            samples: Vec::new(),
            drag_x: 0.0,
            drag_y: 0.0,
            lock_progress: 0.0,
        };
    }

    /// Calculate lock progress ratio from upward vertical drag distance (0.0 to 1.0)
    /// Upstream: `calcLockProgress(QPoint globalPos)` (lines 3244-3249)
    pub fn calc_lock_progress(&self, drag_y: f32) -> f32 {
        if drag_y >= 0.0 {
            0.0
        } else {
            (-drag_y / self.lock_height_px).clamp(0.0, 1.0)
        }
    }

    /// Handle drag delta during holding:
    /// - Sliding up past lock_height_px (or capturing past ANIMATION_THRESHOLD) locks recording
    /// - Sliding left past cancel_threshold_px cancels recording
    pub fn handle_drag(&mut self, delta_x: f32, delta_y: f32) {
        let progress = self.calc_lock_progress(delta_y);
        if let VoiceRecordState::Holding {
            duration_ms,
            is_video,
            level,
            ref samples,
            ref mut drag_x,
            ref mut drag_y,
            ref mut lock_progress,
        } = self.state
        {
            *drag_x = delta_x;
            *drag_y = delta_y;
            *lock_progress = progress;

            // Upstream: lock if progress >= 1.0 or captured by input over animation threshold
            if *lock_progress >= 1.0 || *lock_progress > ANIMATION_THRESHOLD && delta_y < -50.0 {
                self.state = VoiceRecordState::Locked {
                    duration_ms,
                    is_video,
                    is_paused: false,
                    level,
                    samples: samples.clone(),
                };
                return;
            }

            // Upstream: cancel if slid past cancel threshold
            if delta_x <= self.cancel_threshold_px {
                self.state = VoiceRecordState::Discarded {
                    reason: "Slid left to cancel".to_string(),
                };
            }
        }
    }

    /// Release pointer during holding
    pub fn handle_release(&mut self) {
        if let VoiceRecordState::Holding {
            duration_ms,
            samples,
            is_video,
            drag_x,
            ..
        } = &self.state
        {
            if *drag_x <= self.cancel_threshold_px || *duration_ms < 500 {
                // Too short (<500ms) or canceled
                self.state = VoiceRecordState::Discarded {
                    reason: if *duration_ms < 500 {
                        "Recording too short (<500ms)".to_string()
                    } else {
                        "Canceled by gesture".to_string()
                    },
                };
            } else {
                self.state = VoiceRecordState::Sent {
                    duration_ms: *duration_ms,
                    waveform: samples.clone(),
                    is_video: *is_video,
                    view_once: self.view_once,
                };
            }
        }
    }

    /// Advance recording duration by elapsed milliseconds (capped at 100 minutes)
    pub fn tick_duration(&mut self, elapsed_ms: u64) {
        let max_ms = self.max_duration_secs * 1000;
        match &mut self.state {
            VoiceRecordState::Holding { duration_ms, .. } => {
                *duration_ms = (*duration_ms + elapsed_ms).min(max_ms);
            }
            VoiceRecordState::Locked {
                duration_ms,
                is_paused,
                ..
            } => {
                if !*is_paused {
                    *duration_ms = (*duration_ms + elapsed_ms).min(max_ms);
                }
            }
            _ => {}
        }
    }

    /// Push audio sample and normalized level
    pub fn record_updated(&mut self, level: u16, sample: u8) {
        match &mut self.state {
            VoiceRecordState::Holding {
                level: l, samples, ..
            }
            | VoiceRecordState::Locked {
                level: l, samples, ..
            } => {
                *l = level;
                samples.push(sample);
            }
            _ => {}
        }
    }

    /// Toggle pause state in locked mode
    pub fn toggle_pause(&mut self) {
        if let VoiceRecordState::Locked {
            ref mut is_paused, ..
        } = self.state
        {
            *is_paused = !*is_paused;
        }
    }

    /// Transition to listen/review state before sending
    pub fn enter_listening(&mut self) {
        if let VoiceRecordState::Locked {
            duration_ms,
            ref samples,
            ..
        } = self.state
        {
            self.state = VoiceRecordState::Listening {
                duration_ms,
                playback_pos: 0.0,
                trim_start: 0.0,
                trim_end: 1.0,
                samples: samples.clone(),
            };
        }
    }

    /// Explicitly stop recording with a target StopType
    pub fn stop(&mut self, stop_type: StopType) {
        match stop_type {
            StopType::Cancel => {
                self.state = VoiceRecordState::Discarded {
                    reason: "User canceled".to_string(),
                };
            }
            StopType::Send => {
                let is_video = self.is_video;
                let view_once = self.view_once;
                match &self.state {
                    VoiceRecordState::Holding {
                        duration_ms,
                        samples,
                        ..
                    }
                    | VoiceRecordState::Locked {
                        duration_ms,
                        samples,
                        ..
                    }
                    | VoiceRecordState::Listening {
                        duration_ms,
                        samples,
                        ..
                    } => {
                        self.state = VoiceRecordState::Sent {
                            duration_ms: *duration_ms,
                            waveform: samples.clone(),
                            is_video,
                            view_once,
                        };
                    }
                    _ => {}
                }
            }
            StopType::Listen => {
                self.enter_listening();
            }
        }
    }

    /// Toggle view-once TTL setting
    pub fn toggle_view_once(&mut self) {
        self.view_once = !self.view_once;
    }

    /// Format duration string as "M:SS" or "H:MM:SS"
    pub fn formatted_duration(&self) -> String {
        let total_secs = match &self.state {
            VoiceRecordState::Holding { duration_ms, .. }
            | VoiceRecordState::Locked { duration_ms, .. }
            | VoiceRecordState::Listening { duration_ms, .. } => duration_ms / 1000,
            VoiceRecordState::Sent { duration_ms, .. } => duration_ms / 1000,
            _ => 0,
        };
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            VoiceRecordState::Holding { .. }
                | VoiceRecordState::Locked { .. }
                | VoiceRecordState::Listening { .. }
        )
    }

    pub fn is_locked(&self) -> bool {
        matches!(self.state, VoiceRecordState::Locked { .. })
    }
}

impl Default for VoiceRecordBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for VoiceRecordBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.is_active();
        let duration_text = self.formatted_duration();
        let is_locked = self.is_locked();
        let is_video = self.is_video;
        let view_once = self.view_once;

        div()
            .flex()
            .items_center()
            .justify_between()
            .h_10()
            .px_3()
            .bg(rgb(0x18222d))
            .rounded_md()
            .text_sm()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Red recording signal dot
                    .child(
                        div()
                            .w(px(SIGNAL_RADIUS_PX * 2.0))
                            .h(px(SIGNAL_RADIUS_PX * 2.0))
                            .rounded_full()
                            .bg(if is_active { rgb(0xff3b30) } else { rgb(0x8e8e93) }),
                    )
                    // Duration counter
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(duration_text),
                    )
                    // Slide to cancel / Lock hint
                    .child(
                        div()
                            .text_color(rgb(0x8e8e93))
                            .text_xs()
                            .child(if is_locked {
                                "Tap stop or send".to_string()
                            } else if is_active {
                                "< Slide to cancel | ^ Swipe to lock".to_string()
                            } else {
                                "Hold to record".to_string()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // TTL View-once toggle
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .bg(if view_once { rgb(0x2481cc) } else { rgb(0x2a394a) })
                            .text_color(rgb(0xffffff))
                            .child(if is_video { "1x Video" } else { "1x Voice" }),
                    )
                    // Mode indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(if is_video { "Video Note" } else { "Voice Note" }),
                    ),
            )
    }
}
