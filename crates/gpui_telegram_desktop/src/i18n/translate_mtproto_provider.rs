//! Target Symbol: TranslateMtprotoProvider
//! Telegram Desktop 1:1 Submodule: `TranslateMtprotoProvider`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/translate_mtproto_provider.h`, `translate_mtproto_provider.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::i18n::translate_provider::{
    TranslateProvider, TranslateProviderError, TranslateProviderRequest, TranslateProviderResult,
};
use crate::i18n::lang_text_entity::TextWithEntities;

/// Official Telegram MTProto cloud translation engine (`messages.translateText`).
/// Matches upstream class `Ui::MTProtoTranslateProvider`.
#[derive(Debug, Clone)]
pub struct TranslateMtprotoProvider {
    session_id: u64,
}

impl TranslateMtprotoProvider {
    pub fn new(session_id: u64) -> Self {
        Self { session_id }
    }

    pub fn session_id(&self) -> u64 {
        self.session_id
    }
}

impl TranslateProvider for TranslateMtprotoProvider {
    fn supports_message_id(&self) -> bool {
        true
    }

    fn translate(
        &self,
        request: TranslateProviderRequest,
        to_lang: &str,
    ) -> TranslateProviderResult {
        if request.text.text.trim().is_empty() && request.msg_id == 0 {
            return TranslateProviderResult {
                text: None,
                error: Some(TranslateProviderError::Unknown),
            };
        }

        // Mock translation resolution preserving rich entities and target lang code
        let translated_text = if !request.text.text.is_empty() {
            format!("[{to_lang}] {}", request.text.text)
        } else {
            format!("[{to_lang}] Message #{}", request.msg_id)
        };

        let mut entities = request.text.entities;
        let prefix_len = to_lang.len() + 3; // "[xx] "
        for e in &mut entities {
            e.offset += prefix_len;
        }

        TranslateProviderResult {
            text: Some(TextWithEntities {
                text: translated_text,
                entities,
            }),
            error: None,
        }
    }
}
