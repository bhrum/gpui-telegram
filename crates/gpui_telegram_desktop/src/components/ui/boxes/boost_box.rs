//! Authentic GPUI Channel & Group Boost Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/boost_box.cpp:L45-L996
//! Reference: Telegram/SourceFiles/ui/boxes/boost_box.h:L20-L154

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BoostCounters {
    pub level: i32,
    pub boosts: i32,
    pub this_level_boosts: i32,
    pub next_level_boosts: i32,
    pub mine: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AskBoostReason {
    ChannelColor { required_level: i32 },
    Autotranslate { required_level: i32 },
    Wallpaper { required_level: i32, is_group: bool },
    EmojiStatus { required_level: i32, is_group: bool },
    EmojiPack { required_level: i32 },
    CustomReactions { count: i32 },
    Cpm { required_level: i32 },
    WearCollectible { required_level: i32 },
}

/// Target symbol for channel/group boost box.
/// Reference: `boost_box.h:L64-L120`
#[derive(Debug, Clone)]
pub struct BoostBoxBoxes {
    pub name: String,
    pub counters: BoostCounters,
    pub is_group: bool,
    pub allow_multi: bool,
    pub lifting: i32,
    pub fireworks_active: bool,
    pub selected_reason: Option<AskBoostReason>,
    pub link: String,
}

impl Default for BoostBoxBoxes {
    fn default() -> Self {
        Self::new("Telegram Channel", false)
    }
}

impl BoostBoxBoxes {
    pub fn new(name: impl Into<String>, is_group: bool) -> Self {
        Self {
            name: name.into(),
            counters: BoostCounters {
                level: 1,
                boosts: 3,
                this_level_boosts: 0,
                next_level_boosts: 5,
                mine: 0,
            },
            is_group,
            allow_multi: true,
            lifting: 0,
            fireworks_active: false,
            selected_reason: None,
            link: "https://t.me/boost/channel".to_string(),
        }
    }

    /// Computes fractional progress within the current boost level.
    /// Reference: `boost_box.cpp:L310-L318`
    pub fn progress_fraction(&self) -> f32 {
        if self.counters.next_level_boosts <= self.counters.this_level_boosts {
            1.0
        } else {
            let num = (self.counters.boosts - self.counters.this_level_boosts) as f32;
            let den = (self.counters.next_level_boosts - self.counters.this_level_boosts) as f32;
            (num / den).clamp(0.0, 1.0)
        }
    }

    pub fn apply_boost(&mut self) -> bool {
        self.counters.boosts += 1;
        self.counters.mine += 1;
        self.fireworks_active = true;
        if self.counters.next_level_boosts > 0 && self.counters.boosts >= self.counters.next_level_boosts {
            self.counters.level += 1;
            self.counters.this_level_boosts = self.counters.next_level_boosts;
            self.counters.next_level_boosts += 5;
        }
        true
    }
}
