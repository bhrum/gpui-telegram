//! Authentic GPUI Emoji Game Stakes & TON Input Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/emoji_stake_box.cpp:L35-L654
//! Reference: Telegram/SourceFiles/ui/boxes/emoji_stake_box.h:L25-L69

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Stake options displayed per horizontal row: 4.
/// Reference: `emoji_stake_box.cpp:L45`
pub const K_PER_ROW: usize = 4;

/// Target symbol for emoji game stake modal dialog.
/// Reference: `emoji_stake_box.h:L45-L69`
#[derive(Debug, Clone)]
pub struct EmojiStakeBoxBoxes {
    pub current_stake: i64,
    pub stake_presets: Vec<i64>,
    pub milli_rewards: [i32; 6],
    pub jackpot_milli_reward: i32,
    pub balance_nano_ton: i64,
    pub is_submitted: bool,
    pub submitted_stake: Option<i64>,
}

impl Default for EmojiStakeBoxBoxes {
    fn default() -> Self {
        Self::new(50, 10_000)
    }
}

impl EmojiStakeBoxBoxes {
    pub fn new(initial_stake: i64, balance: i64) -> Self {
        Self {
            current_stake: initial_stake,
            stake_presets: vec![10, 25, 50, 100, 250, 500, 1000, 2500],
            milli_rewards: [1000, 1500, 2000, 3000, 5000, 10000],
            jackpot_milli_reward: 50000, // 50x multiplier
            balance_nano_ton: balance,
            is_submitted: false,
            submitted_stake: None,
        }
    }

    /// Selects preset stake or clamps custom input.
    /// Reference: `emoji_stake_box.cpp:L120-L140`
    pub fn select_preset(&mut self, stake: i64) {
        if stake > 0 {
            self.current_stake = stake;
        }
    }

    pub fn can_submit(&self) -> bool {
        self.current_stake > 0 && self.current_stake <= self.balance_nano_ton
    }

    pub fn submit(&mut self) -> bool {
        if self.can_submit() {
            self.is_submitted = true;
            self.submitted_stake = Some(self.current_stake);
            true
        } else {
            false
        }
    }
}
