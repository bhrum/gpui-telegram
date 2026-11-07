//! Edit Privacy Settings and Exceptions Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/edit_privacy_box.h:L1-L193`
//! - `Telegram/SourceFiles/boxes/edit_privacy_box.cpp:L1-L1417`
//! - `Telegram/SourceFiles/boxes/edit_privacy_box.cpp:L52-L54` (`kPremiumsRowId`, `kMiniAppsRowId`, `kDefaultPrivateMessagesPrice`)
//! - `Telegram/SourceFiles/boxes/edit_privacy_box.cpp:L1160-L1175` (`kOptionAll`, `kOptionContacts`, `kOptionNobody`, `kOptionPremium`, `kOptionCharge`)
//! - `Telegram/SourceFiles/boxes/edit_privacy_box.cpp:L1356` (`kDirectParam`)
//!
//! Subsystem: boxes, Feature: TCG-N03

use gpui::{px, Pixels};
use std::collections::HashSet;

pub const K_PREMIUMS_ROW_ID: u64 = 0x7fff_ffff_0000_0001;
pub const K_MINI_APPS_ROW_ID: u64 = 0x7fff_ffff_0000_0002;
pub const K_DEFAULT_PRIVATE_MESSAGES_PRICE: u32 = 10;
pub const K_DIRECT_PARAM: &str = "?direct";

/// Privacy option rule value (edit_privacy_box.cpp:L1160-L1175).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyRuleOption {
    Everyone = 0,
    Contacts = 1,
    Nobody = 2,
    Premium = 3,
    ChargeStars = 4,
}

/// Category of privacy setting being configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyCategory {
    LastSeen,
    PhoneNumber,
    ProfilePhoto,
    Forwards,
    Calls,
    VoiceMessages,
    Bio,
    GroupsAndChannels,
}

/// Modal dialog box for configuring peer privacy rules and exception whitelists/blacklists.
/// Target symbol in ledger: `EditPrivacyBox`.
#[derive(Debug, Clone)]
pub struct EditPrivacyBox {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub category: PrivacyCategory,
    pub selected_rule: PrivacyRuleOption,
    pub stars_price: u32,
    pub always_allow: HashSet<u64>,
    pub never_allow: HashSet<u64>,
    pub allow_premium_override: bool,
}

impl Default for EditPrivacyBox {
    fn default() -> Self {
        Self::new(PrivacyCategory::LastSeen)
    }
}

impl EditPrivacyBox {
    pub fn new(category: PrivacyCategory) -> Self {
        Self {
            is_visible: true,
            width: px(420.0),
            height: px(520.0),
            category,
            selected_rule: PrivacyRuleOption::Contacts,
            stars_price: K_DEFAULT_PRIVATE_MESSAGES_PRICE,
            always_allow: HashSet::new(),
            never_allow: HashSet::new(),
            allow_premium_override: false,
        }
    }

    pub fn set_rule(&mut self, rule: PrivacyRuleOption) {
        self.selected_rule = rule;
        if rule == PrivacyRuleOption::ChargeStars && self.stars_price == 0 {
            self.stars_price = K_DEFAULT_PRIVATE_MESSAGES_PRICE;
        }
    }

    pub fn add_always_allow(&mut self, peer_id: u64) {
        self.never_allow.remove(&peer_id);
        self.always_allow.insert(peer_id);
    }

    pub fn add_never_allow(&mut self, peer_id: u64) {
        self.always_allow.remove(&peer_id);
        self.never_allow.insert(peer_id);
    }

    pub fn remove_exception(&mut self, peer_id: u64) {
        self.always_allow.remove(&peer_id);
        self.never_allow.remove(&peer_id);
    }

    pub fn is_allowed_for(&self, peer_id: u64, is_contact: bool, is_premium: bool) -> bool {
        if self.never_allow.contains(&peer_id) {
            return false;
        }
        if self.always_allow.contains(&peer_id) {
            return true;
        }
        match self.selected_rule {
            PrivacyRuleOption::Everyone => true,
            PrivacyRuleOption::Contacts => is_contact,
            PrivacyRuleOption::Nobody => false,
            PrivacyRuleOption::Premium => is_premium || (self.allow_premium_override && is_contact),
            PrivacyRuleOption::ChargeStars => true,
        }
    }
}
