//! Upstream Source: Telegram/SourceFiles/webauthn/cable_scanner_linux.cpp
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::cable_core::ADVERT_SIZE;

pub const GOOGLE_CABLE_UUID: &str = "0000fde2-0000-1000-8000-00805f9b34fb";
pub const FIDO_CABLE_UUID: &str = "0000fff9-0000-1000-8000-00805f9b34fb";
pub const BLUEZ_SERVICE: &str = "org.bluez";
pub const BLUEZ_OBJECT_PATH: &str = "/";

#[derive(Debug, Clone, Default)]
pub struct CableScannerLinux {
    pub is_running: bool,
}

impl CableScannerLinux {
    pub fn new() -> Self {
        Self { is_running: false }
    }

    pub fn start(&mut self) -> bool {
        self.is_running = true;
        true
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_cable_uuid(uuid: &str) -> bool {
        uuid.eq_ignore_ascii_case(GOOGLE_CABLE_UUID) || uuid.eq_ignore_ascii_case(FIDO_CABLE_UUID)
    }

    pub fn parse_service_data(data: &[u8]) -> Option<Vec<u8>> {
        if data.len() >= ADVERT_SIZE {
            Some(data[..ADVERT_SIZE].to_vec())
        } else {
            None
        }
    }
}
