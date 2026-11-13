//! Target Symbol: TranslateProvider
//! Telegram Desktop 1:1 Submodule: `TranslateProvider`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/translate_provider.h`, `translate_provider.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::i18n::lang_text_entity::TextWithEntities;

pub const OPTION_TRANSLATE_URL_TEMPLATE: &str = "translate-url-template";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslateProviderError {
    Unknown,
    Network,
    RateLimit,
    UnsupportedLanguage,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TranslateProviderRequest {
    pub peer_id: u64,
    pub msg_id: i32,
    pub text: TextWithEntities,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TranslateProviderResult {
    pub text: Option<TextWithEntities>,
    pub error: Option<TranslateProviderError>,
}

/// Abstract machine translation backend interface.
/// Matches upstream class `Ui::TranslateProvider`.
pub trait TranslateProvider: Send + Sync {
    /// Returns true if provider supports server-side message ID resolution.
    fn supports_message_id(&self) -> bool;

    /// Dispatches a translation request.
    fn translate(
        &self,
        request: TranslateProviderRequest,
        to_lang: &str,
    ) -> TranslateProviderResult;
}

/// Prepares translation request by normalizing message ID versus raw text payload.
/// Upstream: `Ui::PrepareTranslateProviderRequest`
pub fn prepare_translate_provider_request(
    provider: &dyn TranslateProvider,
    peer_id: u64,
    msg_id: i32,
    text: TextWithEntities,
) -> TranslateProviderRequest {
    let mut req = TranslateProviderRequest {
        peer_id,
        msg_id,
        text,
    };

    if !provider.supports_message_id() {
        req.msg_id = 0;
    }

    req
}
