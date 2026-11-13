//! Target Symbol: LangHardcoded
//! Telegram Desktop 1:1 Submodule: `LangHardcoded`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_hardcoded.h`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

/// System-level hardcoded error strings, fallback notices, and passport security dialogs.
/// Matches upstream namespace `Lang::Hard`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangHardcoded;

impl LangHardcoded {
    /// Upstream: `FavedSetTitle` -> "Favorite stickers"
    pub fn faved_set_title() -> &'static str {
        "Favorite stickers"
    }

    /// Upstream: `CallErrorIncompatible` -> "{user}'s app is using an incompatible protocol. They need to update their app before you can call them."
    pub fn call_error_incompatible(user: &str) -> String {
        format!("{user}'s app is using an incompatible protocol. They need to update their app before you can call them.")
    }

    /// Upstream: `ServerError` -> "Internal server error."
    pub fn server_error() -> &'static str {
        "Internal server error."
    }

    /// Upstream: `ClearPathFailed` -> "Clear failed :("
    pub fn clear_path_failed() -> &'static str {
        "Clear failed :("
    }

    /// Upstream: `ProxyConfigError` -> "The proxy you are using is not configured correctly and will be disabled. Please find another one."
    pub fn proxy_config_error() -> &'static str {
        "The proxy you are using is not configured correctly and will be disabled. Please find another one."
    }

    /// Upstream: `NoAuthorizationBot` -> "Could not get authorization bot."
    pub fn no_authorization_bot() -> &'static str {
        "Could not get authorization bot."
    }

    /// Upstream: `SecureSaveError` -> "Error saving value."
    pub fn secure_save_error() -> &'static str {
        "Error saving value."
    }

    /// Upstream: `SecureAcceptError` -> "Error accepting form."
    pub fn secure_accept_error() -> &'static str {
        "Error accepting form."
    }

    /// Upstream: `PassportCorrupted` -> "It seems your Telegram Passport data was corrupted.\n\nYou can reset your Telegram Passport and restart this authorization."
    pub fn passport_corrupted() -> &'static str {
        "It seems your Telegram Passport data was corrupted.\n\nYou can reset your Telegram Passport and restart this authorization."
    }

    /// Upstream: `PassportCorruptedChange` -> "It seems your Telegram Passport data was corrupted.\n\nYou can reset your Telegram Passport and change your cloud password."
    pub fn passport_corrupted_change() -> &'static str {
        "It seems your Telegram Passport data was corrupted.\n\nYou can reset your Telegram Passport and change your cloud password."
    }

    /// Upstream: `PassportCorruptedReset` -> "Reset"
    pub fn passport_corrupted_reset() -> &'static str {
        "Reset"
    }

    /// Upstream: `PassportCorruptedResetSure` -> "Are you sure you want to reset your Telegram Passport data?"
    pub fn passport_corrupted_reset_sure() -> &'static str {
        "Are you sure you want to reset your Telegram Passport data?"
    }

    /// Upstream: `UnknownSecureScanError` -> "Unknown scan read error."
    pub fn unknown_secure_scan_error() -> &'static str {
        "Unknown scan read error."
    }

    /// Upstream: `EmailConfirmationExpired` -> "This email confirmation has expired. Please setup two-step verification once again."
    pub fn email_confirmation_expired() -> &'static str {
        "This email confirmation has expired. Please setup two-step verification once again."
    }

    /// Upstream: `AutostartEnableError` -> "Could not register for autostart."
    pub fn autostart_enable_error() -> &'static str {
        "Could not register for autostart."
    }
}
