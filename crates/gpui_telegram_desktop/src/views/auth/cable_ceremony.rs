//! Upstream Source: Telegram/SourceFiles/webauthn/cable_ceremony.cpp
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::cable_box::CableSheet;

pub const CEREMONY_TIMEOUT_MS: u64 = 180_000;
pub const TUNNEL_PORT: u16 = 443;

#[derive(Debug, Clone)]
pub struct CableCeremony {
    pub tunnel_url: String,
    pub timeout_ms: u64,
    pub is_active: bool,
    pub current_sheet: CableSheet,
    pub elapsed_ms: u64,
}

impl Default for CableCeremony {
    fn default() -> Self {
        Self::new()
    }
}

impl CableCeremony {
    pub fn new() -> Self {
        Self {
            tunnel_url: format!("wss://cable.auth.telegram.org:{}/tunnel", TUNNEL_PORT),
            timeout_ms: CEREMONY_TIMEOUT_MS,
            is_active: false,
            current_sheet: CableSheet::Qr,
            elapsed_ms: 0,
        }
    }

    pub fn start(&mut self, rp_id: &str) -> String {
        self.is_active = true;
        self.elapsed_ms = 0;
        self.current_sheet = CableSheet::Qr;
        format!("FIDO:/{}/{}", rp_id, hex::encode([0xde, 0xad, 0xbe, 0xef]))
    }

    pub fn advance_sheet(&mut self, next: CableSheet) {
        self.current_sheet = next;
    }

    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if !self.is_active {
            return false;
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
        if self.elapsed_ms >= self.timeout_ms {
            self.cancel();
            true
        } else {
            false
        }
    }

    pub fn cancel(&mut self) {
        self.is_active = false;
        self.current_sheet = CableSheet::Error;
    }
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}
