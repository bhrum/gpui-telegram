//! Upstream Source: Telegram/SourceFiles/webauthn/cable_scanner_win.cpp
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::cable_core::ADVERT_SIZE;

pub const SERVICE_DATA_AD_TYPE: u8 = 0x16;
pub const GOOGLE_CABLE_UUID16: u16 = 0xFDE2;
pub const FIDO_CABLE_UUID16: u16 = 0xFFF9;

#[derive(Debug, Clone, Default)]
pub struct CableScannerWin {
    pub is_watching: bool,
}

impl CableScannerWin {
    pub fn new() -> Self {
        Self { is_watching: false }
    }

    pub fn start(&mut self) -> bool {
        self.is_watching = true;
        true
    }

    pub fn stop(&mut self) {
        self.is_watching = false;
    }

    pub fn matches_uuid16(uuid: u16) -> bool {
        uuid == GOOGLE_CABLE_UUID16 || uuid == FIDO_CABLE_UUID16
    }

    pub fn parse_advertisement(ad_type: u8, uuid: u16, payload: &[u8]) -> Option<Vec<u8>> {
        if ad_type != SERVICE_DATA_AD_TYPE || !Self::matches_uuid16(uuid) {
            return None;
        }
        if payload.len() >= ADVERT_SIZE {
            Some(payload[..ADVERT_SIZE].to_vec())
        } else {
            None
        }
    }
}
