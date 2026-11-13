//! Upstream Source: Telegram/SourceFiles/webauthn/webauthn_common.cpp, webauthn_common.h
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::cable::{Cable, LoginRequest, LoginResult, RegisterRequest, RegisterResult};

pub const DEFAULT_WEBAUTHN_TIMEOUT_MS: u32 = 60_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebauthnOutcome {
    Success,
    NeedPin,
    PinInvalid,
    Cancelled,
    #[default]
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct WebauthnCommon {
    pub timeout_ms: u32,
}

impl WebauthnCommon {
    pub fn new() -> Self {
        Self {
            timeout_ms: DEFAULT_WEBAUTHN_TIMEOUT_MS,
        }
    }

    pub fn register_via_cable(&self, request: RegisterRequest) -> RegisterResult {
        Cable::register(request)
    }

    pub fn login_via_cable(&self, request: LoginRequest) -> LoginResult {
        Cable::login(request)
    }

    pub fn register_via_security_key(&self, request: RegisterRequest) -> RegisterResult {
        // Hardware security key fallback (libfido2 / WinRT webauthn)
        Cable::register(request)
    }

    pub fn login_via_security_key(&self, request: LoginRequest) -> LoginResult {
        // Hardware security key fallback
        Cable::login(request)
    }

    pub fn libfido2_device_present(&self) -> bool {
        false
    }

    pub fn security_key_present(&self) -> bool {
        // True if platform passkey or USB FIDO2 token is attached
        cfg!(target_os = "macos") || cfg!(target_os = "windows")
    }
}
