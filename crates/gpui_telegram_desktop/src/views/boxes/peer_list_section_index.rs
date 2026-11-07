//! Peer List Section Alphabetical Fast-Scroller Index
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_list_section_index.h:L1-L68`
//! - `Telegram/SourceFiles/boxes/peer_list_section_index.cpp:L1-L291`
//! - `Telegram/SourceFiles/boxes/peer_list_section_index.cpp:L16-L19` (`kMaxBoost`, `kEase`, `kFisheyeReachSlots`, `kBulgeReserveSlots`)
//! - `Telegram/SourceFiles/boxes/peer_list_section_index.cpp:L114-L140` (Fisheye distance falloff)
//! - `Telegram/SourceFiles/boxes/peer_list_section_index.cpp:L160-L165` (Scale easing)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub const K_MAX_BOOST: f32 = 1.8;
pub const K_EASE: f32 = 0.6;
pub const K_FISHEYE_REACH_SLOTS: f32 = 2.2;
pub const K_BULGE_RESERVE_SLOTS: f32 = 2.5;

/// Single letter entry in the section index bar.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionIndexLetter {
    pub letter: char,
    pub content_top: Pixels,
}

/// Alphabetical slot representation with dynamic fisheye scale.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionIndexSlot {
    pub letter: char,
    pub source_index: usize,
    pub center_y: Pixels,
    pub current_scale: f32,
    pub target_scale: f32,
}

/// Alphabetical fast-scroller index sidebar widget.
/// Target symbol in ledger: `PeerListSectionIndex`.
#[derive(Debug, Clone)]
pub struct PeerListSectionIndex {
    pub is_visible: bool,
    pub is_pressed: bool,
    pub letters: Vec<SectionIndexLetter>,
    pub slots: Vec<SectionIndexSlot>,
    pub pitch: Pixels,
    pub item_height: Pixels,
    pub full_height: Pixels,
    pub active_slot: Option<usize>,
}

impl Default for PeerListSectionIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerListSectionIndex {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            is_pressed: false,
            letters: Vec::new(),
            slots: Vec::new(),
            pitch: px(16.0),
            item_height: px(14.0),
            full_height: px(400.0),
            active_slot: None,
        }
    }

    /// Rebuilds letter slots from section entries (peer_list_section_index.cpp:L70-L110).
    pub fn set_letters(&mut self, letters: Vec<SectionIndexLetter>) {
        self.letters = letters;
        let pitch_f = f32::from(self.pitch);
        self.slots = self
            .letters
            .iter()
            .enumerate()
            .map(|(idx, l)| SectionIndexSlot {
                letter: l.letter,
                source_index: idx,
                center_y: px(idx as f32 * pitch_f + pitch_f / 2.0),
                current_scale: 1.0,
                target_scale: 1.0,
            })
            .collect();
    }

    /// Calculates fisheye magnification for mouse position (peer_list_section_index.cpp:L114-L140).
    pub fn update_fisheye(&mut self, mouse_y: Pixels) {
        let pitch_f = f32::from(self.pitch);
        let reach = K_FISHEYE_REACH_SLOTS * pitch_f;
        if reach <= 0.0 {
            return;
        }

        let mouse_y_f = f32::from(mouse_y);
        for slot in &mut self.slots {
            let dist = (mouse_y_f - f32::from(slot.center_y)).abs();
            let vertical_falloff = (1.0f32 - (dist / reach)).clamp(0.0f32, 1.0f32);
            slot.target_scale = 1.0 + (K_MAX_BOOST - 1.0) * vertical_falloff;
            // Ease towards target scale (peer_list_section_index.cpp:L162)
            slot.current_scale += (slot.target_scale - slot.current_scale) * K_EASE;
        }
    }

    /// Resolves the nearest slot to a click/drag position (peer_list_section_index.cpp:L185-L205).
    pub fn jump_at_y(&mut self, mouse_y: Pixels) -> Option<Pixels> {
        if self.slots.is_empty() {
            return None;
        }
        let pitch_f = f32::from(self.pitch);
        let slot_idx = (f32::from(mouse_y) / pitch_f)
            .floor()
            .clamp(0.0, (self.slots.len() - 1) as f32) as usize;
        self.active_slot = Some(slot_idx);
        let source_idx = self.slots[slot_idx].source_index;
        self.letters.get(source_idx).map(|l| l.content_top)
    }

    pub fn reset_scales(&mut self) {
        for slot in &mut self.slots {
            slot.target_scale = 1.0;
            slot.current_scale = 1.0;
        }
        self.active_slot = None;
    }
}
