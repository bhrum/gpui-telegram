//! Voice Hold-to-Record Gesture State Machine (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//! Upstream reference: `tdesktop/Telegram/SourceFiles/chat_helpers/message_field.cpp`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoiceGestureState {
    /// Idle: Microphone button ready to press
    Idle,
    /// Holding: User holds the mic button. Sliding left cancels, sliding up locks
    Holding {
        elapsed_secs: f32,
        amplitude: f32,
        start_x: f32,
        start_y: f32,
        current_x: f32,
        current_y: f32,
    },
    /// Locked: Hands-free recording mode. User can tap Stop or Send without holding
    Locked {
        elapsed_secs: f32,
        amplitude: f32,
    },
    /// SlideCanceling: User slid past -100px threshold to cancel
    SlideCanceling {
        offset_x: f32,
    },
    /// Completed: Finished recording successfully with audio duration and waveform
    Completed {
        duration_secs: f32,
        waveform: Vec<u8>,
    },
    /// Discarded: Recording was canceled or too short (<0.5s tap)
    Discarded,
}

impl Default for VoiceGestureState {
    fn default() -> Self {
        Self::Idle
    }
}

pub struct VoiceGestureMachine {
    pub state: VoiceGestureState,
    pub waveform_samples: Vec<u8>,
    pub min_duration_threshold_secs: f32,
    pub cancel_slide_threshold_px: f32,
    pub lock_slide_threshold_px: f32,
}

impl VoiceGestureMachine {
    pub fn new() -> Self {
        Self {
            state: VoiceGestureState::Idle,
            waveform_samples: Vec::new(),
            min_duration_threshold_secs: 0.5,
            cancel_slide_threshold_px: -80.0,
            lock_slide_threshold_px: -50.0,
        }
    }

    /// Pointer down on mic button
    pub fn on_press_start(&mut self, x: f32, y: f32) {
        self.waveform_samples.clear();
        self.state = VoiceGestureState::Holding {
            elapsed_secs: 0.0,
            amplitude: 0.0,
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
        };
    }

    /// Pointer dragged while holding
    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        if let VoiceGestureState::Holding {
            elapsed_secs,
            amplitude,
            start_x,
            start_y,
            ..
        } = self.state
        {
            let delta_x = x - start_x;
            let delta_y = y - start_y;

            // Slide left to cancel threshold
            if delta_x < self.cancel_slide_threshold_px {
                self.state = VoiceGestureState::SlideCanceling { offset_x: delta_x };
                return;
            }

            // Slide up to lock threshold
            if delta_y < self.lock_slide_threshold_px {
                self.state = VoiceGestureState::Locked {
                    elapsed_secs,
                    amplitude,
                };
                return;
            }

            self.state = VoiceGestureState::Holding {
                elapsed_secs,
                amplitude,
                start_x,
                start_y,
                current_x: x,
                current_y: y,
            };
        }
    }

    /// Pointer released
    pub fn on_press_end(&mut self) {
        match self.state {
            VoiceGestureState::Holding { elapsed_secs, .. } => {
                if elapsed_secs >= self.min_duration_threshold_secs {
                    self.state = VoiceGestureState::Completed {
                        duration_secs: elapsed_secs,
                        waveform: self.waveform_samples.clone(),
                    };
                } else {
                    // Tap was too short
                    self.state = VoiceGestureState::Discarded;
                }
            }
            VoiceGestureState::SlideCanceling { .. } => {
                self.state = VoiceGestureState::Discarded;
            }
            VoiceGestureState::Locked { .. } => {
                // In locked mode, releasing mouse does NOT stop recording
            }
            _ => {
                self.state = VoiceGestureState::Idle;
            }
        }
    }

    /// Lock button clicked (hands-free)
    pub fn lock(&mut self) {
        if let VoiceGestureState::Holding { elapsed_secs, amplitude, .. } = self.state {
            self.state = VoiceGestureState::Locked {
                elapsed_secs,
                amplitude,
            };
        }
    }

    /// Cancel / Trash recording
    pub fn cancel(&mut self) {
        self.state = VoiceGestureState::Discarded;
    }

    /// Explicit send when locked
    pub fn stop_and_send(&mut self) {
        match self.state {
            VoiceGestureState::Locked { elapsed_secs, .. } | VoiceGestureState::Holding { elapsed_secs, .. } => {
                self.state = VoiceGestureState::Completed {
                    duration_secs: elapsed_secs,
                    waveform: self.waveform_samples.clone(),
                };
            }
            _ => {}
        }
    }

    /// Periodic audio tick from capture thread (accumulates duration and 5-bit sample)
    pub fn tick(&mut self, dt_secs: f32, sample_amplitude: f32) {
        // Quantize amplitude to 5-bit (0..31)
        let sample_5bit = ((sample_amplitude * 31.0).clamp(0.0, 31.0)) as u8;
        self.waveform_samples.push(sample_5bit);

        match &mut self.state {
            VoiceGestureState::Holding { elapsed_secs, amplitude, .. } => {
                *elapsed_secs += dt_secs;
                *amplitude = sample_amplitude;
            }
            VoiceGestureState::Locked { elapsed_secs, amplitude } => {
                *elapsed_secs += dt_secs;
                *amplitude = sample_amplitude;
            }
            _ => {}
        }
    }

    pub fn is_recording(&self) -> bool {
        matches!(
            self.state,
            VoiceGestureState::Holding { .. }
                | VoiceGestureState::Locked { .. }
                | VoiceGestureState::SlideCanceling { .. }
        )
    }

    pub fn is_locked(&self) -> bool {
        matches!(self.state, VoiceGestureState::Locked { .. })
    }

    pub fn duration_secs(&self) -> f32 {
        match self.state {
            VoiceGestureState::Holding { elapsed_secs, .. }
            | VoiceGestureState::Locked { elapsed_secs, .. } => elapsed_secs,
            VoiceGestureState::Completed { duration_secs, .. } => duration_secs,
            _ => 0.0,
        }
    }
}

impl Default for VoiceGestureMachine {
    fn default() -> Self {
        Self::new()
    }
}
