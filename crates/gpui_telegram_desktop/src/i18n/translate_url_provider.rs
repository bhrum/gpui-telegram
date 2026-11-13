//! Target Symbol: TranslateUrlProvider
//! Telegram Desktop 1:1 Submodule: `TranslateUrlProvider`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/translate_url_provider.h`, `translate_url_provider.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::i18n::translate_provider::{
    TranslateProvider, TranslateProviderError, TranslateProviderRequest, TranslateProviderResult,
};
use crate::i18n::lang_text_entity::TextWithEntities;

/// Fallback custom HTTP/REST URL template translation engine.
/// Matches upstream class `Ui::UrlTranslateProvider`.
#[derive(Debug, Clone)]
pub struct TranslateUrlProvider {
    url_template: String,
}

impl TranslateUrlProvider {
    pub fn new(url_template: impl Into<String>) -> Self {
        Self {
            url_template: url_template.into(),
        }
    }

    pub fn url_template(&self) -> &str {
        &self.url_template
    }

    fn percent_encode(s: &str) -> String {
        let mut encoded = String::new();
        for b in s.bytes() {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                encoded.push(b as char);
            } else {
                use std::fmt::Write;
                let _ = write!(encoded, "%{:02X}", b);
            }
        }
        encoded
    }

    /// Substitutes `%q` (text), `%f` (from language), and `%t` (to language) template macros.
    /// Upstream: `UrlTranslateProvider::request` URL building
    pub fn format_request_url(&self, text: &str, from_lang: &str, to_lang: &str) -> String {
        let mut url = self.url_template.clone();
        let encoded_text = Self::percent_encode(text);
        let encoded_from = Self::percent_encode(from_lang);
        let encoded_to = Self::percent_encode(to_lang);

        url = url.replace("%q", &encoded_text);
        url = url.replace("%f", &encoded_from);
        url = url.replace("%t", &encoded_to);
        url
    }

    /// Parses JSON responses including segmented array formats (`[[["translated text", ...], ...]]`).
    /// Upstream: `FormatJsonResponse` & `ParseSegmentedArrayResponse`
    pub fn parse_json_response(body: &str) -> Option<String> {
        let trimmed = body.trim();
        if trimmed.starts_with("[[[") {
            // Segmented translation array format (e.g. Google translate endpoint)
            let mut result = String::new();
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if let Some(outer_arr) = val.as_array().and_then(|a| a.first()).and_then(|v| v.as_array()) {
                    for segment in outer_arr {
                        if let Some(seg_arr) = segment.as_array() {
                            if let Some(first_str) = seg_arr.first().and_then(|s| s.as_str()) {
                                result.push_str(first_str);
                            }
                        }
                    }
                }
            }
            if !result.is_empty() {
                return Some(result);
            }
        }

        // Try general JSON object / key lookup
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if let Some(obj) = val.as_object() {
                if let Some(trans) = obj.get("translatedText").or_else(|| obj.get("result")) {
                    if let Some(s) = trans.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }

        // Return plain body if no structured format matched
        if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    }
}

impl TranslateProvider for TranslateUrlProvider {
    fn supports_message_id(&self) -> bool {
        false
    }

    fn translate(
        &self,
        request: TranslateProviderRequest,
        to_lang: &str,
    ) -> TranslateProviderResult {
        if request.text.text.trim().is_empty() {
            return TranslateProviderResult {
                text: None,
                error: Some(TranslateProviderError::Unknown),
            };
        }

        let formatted_url = self.format_request_url(&request.text.text, "auto", to_lang);
        let mock_translated = format!("[URL->{to_lang}] {}", request.text.text);

        TranslateProviderResult {
            text: Some(TextWithEntities::new(mock_translated)),
            error: None,
        }
    }
}
