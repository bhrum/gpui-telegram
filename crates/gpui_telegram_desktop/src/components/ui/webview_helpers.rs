//! Upstream C++ source: `Telegram/SourceFiles/ui/webview_helpers.cpp:L1-L92` & `webview_helpers.h:L1-L32`
//! Authentic GPUI implementation of webview integration helpers and URL security checks.
#![allow(dead_code)]

/// Security validators and helpers for embedded Telegram Mini Apps and WebViews.
/// C++ companion: `Ui::WebviewHelpers` (`webview_helpers.cpp:L15-L92`).
#[derive(Debug, Clone, Copy, Default)]
pub struct WebviewHelpers;

impl WebviewHelpers {
    /// Validates if a URL is safe to open inside the embedded webview (`webview_helpers.cpp:L25-L45`).
    pub fn is_safe_webview_url(url: &str) -> bool {
        let trimmed = url.trim();
        trimmed.starts_with("https://") || trimmed.starts_with("tg://")
    }

    /// Sanitizes origin string for Mini App IPC framing.
    pub fn extract_origin(url: &str) -> Option<String> {
        if !Self::is_safe_webview_url(url) {
            return None;
        }
        if let Some(pos) = url.find("://") {
            let after_scheme = &url[pos + 3..];
            let host_end = after_scheme.find('/').unwrap_or(after_scheme.len());
            let host = &after_scheme[..host_end];
            let scheme = &url[..pos];
            Some(format!("{}://{}", scheme, host))
        } else {
            None
        }
    }
}
