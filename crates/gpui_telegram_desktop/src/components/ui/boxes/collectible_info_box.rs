//! Authentic GPUI TON Collectible Info Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/collectible_info_box.cpp:L35-L283
//! Reference: Telegram/SourceFiles/ui/boxes/collectible_info_box.h:L15-L37
//! Reference: Telegram/SourceFiles/ui/boxes/collectible_info_box.style:L1-L28

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectibleType {
    Phone,
    Username,
}

/// Detects collectible type based on leading character.
/// Reference: `collectible_info_box.cpp:L40-L48`
pub fn detect_collectible_type(entity: &str) -> CollectibleType {
    let trimmed = entity.trim();
    if trimmed.starts_with('+') || trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        CollectibleType::Phone
    } else {
        CollectibleType::Username
    }
}

/// Formats entity display: phone non-digit replacement or username `@` prefix.
/// Reference: `collectible_info_box.cpp:L42-L47`
pub fn format_collectible_entity(c_type: CollectibleType, entity: &str) -> String {
    match c_type {
        CollectibleType::Phone => {
            let digits: String = entity.chars().filter(|c| c.is_ascii_digit()).collect();
            format!("+{}", digits)
        }
        CollectibleType::Username => {
            let trimmed = entity.trim();
            if trimmed.starts_with('@') {
                trimmed.to_string()
            } else {
                format!("@{}", trimmed)
            }
        }
    }
}

/// Target symbol for collectible info dialog box.
/// Reference: `collectible_info_box.h:L20-L37`
#[derive(Debug, Clone)]
pub struct CollectibleInfoBoxBoxes {
    pub entity: String,
    pub collectible_type: CollectibleType,
    pub formatted_entity: String,
    pub owner_name: String,
    pub crypto_amount_nano_ton: u64,
    pub crypto_currency: String,
    pub url: String,
    pub date: i64,
}

impl Default for CollectibleInfoBoxBoxes {
    fn default() -> Self {
        Self::new("88801234567")
    }
}

impl CollectibleInfoBoxBoxes {
    pub fn new(entity: impl Into<String>) -> Self {
        let raw = entity.into();
        let c_type = detect_collectible_type(&raw);
        let formatted = format_collectible_entity(c_type, &raw);
        Self {
            entity: raw,
            collectible_type: c_type,
            formatted_entity: formatted,
            owner_name: "Anonymous Owner".to_string(),
            crypto_amount_nano_ton: 10_500_000_000, // 10.5 TON
            crypto_currency: "TON".to_string(),
            url: "https://fragment.com".to_string(),
            date: 1794025000,
        }
    }

    /// Formats TON crypto amount with decimal fraction.
    /// Reference: `collectible_info_box.cpp:L55-L65`
    pub fn formatted_ton_price(&self) -> String {
        let whole = self.crypto_amount_nano_ton / 1_000_000_000;
        let frac = (self.crypto_amount_nano_ton % 1_000_000_000) / 10_000_000;
        format!("{}.{:02} TON", whole, frac)
    }
}
