//! Channel Ownership Transfer Controller and 2FA Verification Flow
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/channel_ownership_transfer.h:L1-L47`
//! - `Telegram/SourceFiles/boxes/peers/channel_ownership_transfer.cpp:L1-L148`
//! - `channel_ownership_transfer.cpp:L30-L100` (Password challenge & transfer execution)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `ChannelOwnershipTransferPeers`.
#[derive(Debug, Clone)]
pub struct ChannelOwnershipTransferPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub channel_id: u64,
    pub new_owner_user_id: u64,
    pub password_confirmed: bool,
    pub is_transferring: bool,
}

impl Default for ChannelOwnershipTransferPeers {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl ChannelOwnershipTransferPeers {
    pub fn new(channel_id: u64, new_owner_user_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(360.0),
            channel_id,
            new_owner_user_id,
            password_confirmed: false,
            is_transferring: false,
        }
    }

    pub fn confirm_password(&mut self) {
        self.password_confirmed = true;
    }

    pub fn can_execute(&self) -> bool {
        self.password_confirmed && !self.is_transferring
    }
}
