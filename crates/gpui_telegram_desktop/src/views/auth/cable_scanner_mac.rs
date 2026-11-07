//! Upstream Source: Telegram/SourceFiles/webauthn/cable_scanner_mac.mm
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::cable_core::ADVERT_SIZE;

pub const CABLE_UUID_GOOGLE: &str = "FDE2";
pub const CABLE_UUID_FIDO: &str = "FFF9";

#[derive(Debug, Clone, Default)]
pub struct CableScannerMac {
    pub is_running: bool,
    pub is_bluetooth_available: bool,
}

impl CableScannerMac {
    pub fn new() -> Self {
        Self {
            is_running: false,
            is_bluetooth_available: true,
        }
    }

    pub fn start(&mut self) -> bool {
        if self.is_bluetooth_available {
            self.is_running = true;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn matches_cbuuid(uuid: &str) -> bool {
        uuid.eq_ignore_ascii_case(CABLE_UUID_GOOGLE) || uuid.eq_ignore_ascii_case(CABLE_UUID_FIDO)
    }

    pub fn extract_advert(uuid: &str, service_data: &[u8]) -> Option<Vec<u8>> {
        if !Self::matches_cbuuid(uuid) {
            return None;
        }
        if service_data.len() >= ADVERT_SIZE {
            Some(service_data[..ADVERT_SIZE].to_vec())
        } else {
            None
        }
    }
}
