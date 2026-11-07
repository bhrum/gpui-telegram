# Subsystem Shard: `lang`

**Total Source Files**: 25  
**Total Source Lines**: 4,812  
**Primary Feature IDs**: TCG-N44  
**JSON Shard Path**: `docs/specs/ledger/shards/lang.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/lang/lang_cloud_manager.cpp` | `TCG-N44` | `LangCloudManager` | `crates/gpui_telegram_desktop/src/i18n/lang_cloud_manager.rs` | 701 | `tests/test_tcg_n44_lang_cloud_manager.rs` |
| `Telegram/SourceFiles/lang/lang_cloud_manager.h` | `TCG-N44` | `LangCloudManager` | `crates/gpui_telegram_desktop/src/i18n/lang_cloud_manager.rs` | 114 | `tests/test_tcg_n44_lang_cloud_manager.rs` |
| `Telegram/SourceFiles/lang/lang_file_parser.cpp` | `TCG-N44` | `LangFileParser` | `crates/gpui_telegram_desktop/src/i18n/lang_file_parser.rs` | 212 | `tests/test_tcg_n44_lang_file_parser.rs` |
| `Telegram/SourceFiles/lang/lang_file_parser.h` | `TCG-N44` | `LangFileParser` | `crates/gpui_telegram_desktop/src/i18n/lang_file_parser.rs` | 56 | `tests/test_tcg_n44_lang_file_parser.rs` |
| `Telegram/SourceFiles/lang/lang_hardcoded.h` | `TCG-N44` | `LangHardcoded` | `crates/gpui_telegram_desktop/src/i18n/lang_hardcoded.rs` | 74 | `tests/test_tcg_n44_lang_hardcoded.rs` |
| `Telegram/SourceFiles/lang/lang_instance.cpp` | `TCG-N44` | `LangInstance` | `crates/gpui_telegram_desktop/src/i18n/lang_instance.rs` | 824 | `tests/test_tcg_n44_lang_instance.rs` |
| `Telegram/SourceFiles/lang/lang_instance.h` | `TCG-N44` | `LangInstance` | `crates/gpui_telegram_desktop/src/i18n/lang_instance.rs` | 161 | `tests/test_tcg_n44_lang_instance.rs` |
| `Telegram/SourceFiles/lang/lang_keys.cpp` | `TCG-N44` | `LangKeys` | `crates/gpui_telegram_desktop/src/i18n/lang_keys.rs` | 298 | `tests/test_tcg_n44_lang_keys.rs` |
| `Telegram/SourceFiles/lang/lang_keys.h` | `TCG-N44` | `LangKeys` | `crates/gpui_telegram_desktop/src/i18n/lang_keys.rs` | 55 | `tests/test_tcg_n44_lang_keys.rs` |
| `Telegram/SourceFiles/lang/lang_numbers_animation.cpp` | `TCG-N44` | `LangNumbersAnimation` | `crates/gpui_telegram_desktop/src/i18n/lang_numbers_animation.rs` | 30 | `tests/test_tcg_n44_lang_numbers_animation.rs` |
| `Telegram/SourceFiles/lang/lang_numbers_animation.h` | `TCG-N44` | `LangNumbersAnimation` | `crates/gpui_telegram_desktop/src/i18n/lang_numbers_animation.rs` | 35 | `tests/test_tcg_n44_lang_numbers_animation.rs` |
| `Telegram/SourceFiles/lang/lang_pch.h` | `TCG-N44` | `LangPch` | `crates/gpui_telegram_desktop/src/i18n/lang_pch.rs` | 14 | `tests/test_tcg_n44_lang_pch.rs` |
| `Telegram/SourceFiles/lang/lang_tag.cpp` | `TCG-N44` | `LangTag` | `crates/gpui_telegram_desktop/src/i18n/lang_tag.rs` | 1059 | `tests/test_tcg_n44_lang_tag.rs` |
| `Telegram/SourceFiles/lang/lang_tag.h` | `TCG-N44` | `LangTag` | `crates/gpui_telegram_desktop/src/i18n/lang_tag.rs` | 76 | `tests/test_tcg_n44_lang_tag.rs` |
| `Telegram/SourceFiles/lang/lang_text_entity.cpp` | `TCG-N44` | `LangTextEntity` | `crates/gpui_telegram_desktop/src/i18n/lang_text_entity.rs` | 75 | `tests/test_tcg_n44_lang_text_entity.rs` |
| `Telegram/SourceFiles/lang/lang_text_entity.h` | `TCG-N44` | `LangTextEntity` | `crates/gpui_telegram_desktop/src/i18n/lang_text_entity.rs` | 225 | `tests/test_tcg_n44_lang_text_entity.rs` |
| `Telegram/SourceFiles/lang/lang_translator.cpp` | `TCG-N44` | `LangTranslator` | `crates/gpui_telegram_desktop/src/i18n/lang_translator.rs` | 54 | `tests/test_tcg_n44_lang_translator.rs` |
| `Telegram/SourceFiles/lang/lang_translator.h` | `TCG-N44` | `LangTranslator` | `crates/gpui_telegram_desktop/src/i18n/lang_translator.rs` | 20 | `tests/test_tcg_n44_lang_translator.rs` |
| `Telegram/SourceFiles/lang/lang_values.h` | `TCG-N44` | `LangValues` | `crates/gpui_telegram_desktop/src/i18n/lang_values.rs` | 175 | `tests/test_tcg_n44_lang_values.rs` |
| `Telegram/SourceFiles/lang/translate_mtproto_provider.cpp` | `TCG-N44` | `TranslateMtprotoProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_mtproto_provider.rs` | 167 | `tests/test_tcg_n44_translate_mtproto_provider.rs` |
| `Telegram/SourceFiles/lang/translate_mtproto_provider.h` | `TCG-N44` | `TranslateMtprotoProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_mtproto_provider.rs` | 21 | `tests/test_tcg_n44_translate_mtproto_provider.rs` |
| `Telegram/SourceFiles/lang/translate_provider.cpp` | `TCG-N44` | `TranslateProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_provider.rs` | 70 | `tests/test_tcg_n44_translate_provider.rs` |
| `Telegram/SourceFiles/lang/translate_provider.h` | `TCG-N44` | `TranslateProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_provider.rs` | 30 | `tests/test_tcg_n44_translate_provider.rs` |
| `Telegram/SourceFiles/lang/translate_url_provider.cpp` | `TCG-N44` | `TranslateUrlProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_url_provider.rs` | 249 | `tests/test_tcg_n44_translate_url_provider.rs` |
| `Telegram/SourceFiles/lang/translate_url_provider.h` | `TCG-N44` | `TranslateUrlProvider` | `crates/gpui_telegram_desktop/src/i18n/translate_url_provider.rs` | 17 | `tests/test_tcg_n44_translate_url_provider.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/lang/lang_cloud_manager.cpp`
- **Classes & Structs**: ConfirmSwitchBox, NotReadyBox
- **Core Methods & Signals**: Ui::hideLayer, ConfirmSwitchBox::ConfirmSwitchBox, ConfirmSwitchBox::prepare, tr::lng_language_switch_title, tr::lng_language_switch_link, tr::lng_language_switch_apply
- **Target**: `LangCloudManager` in `crates/gpui_telegram_desktop/src/i18n/lang_cloud_manager.rs`

### `Telegram/SourceFiles/lang/lang_cloud_manager.h`
- **Classes & Structs**: Instance, Pack, Language, CloudManager, ValueRequest, Languages
- **Core Methods & Signals**: ParseLanguage, CloudManager, requestLanguageList, requestLangPackDifference, applyLangPackDifference, setCurrentVersions
- **Target**: `LangCloudManager` in `crates/gpui_telegram_desktop/src/i18n/lang_cloud_manager.rs`

### `Telegram/SourceFiles/lang/lang_file_parser.h`
- **Classes & Structs**: FileParser, Result
- **Core Methods & Signals**: ReadFile, found, parse, error, warning, readKeyValue
- **Target**: `LangFileParser` in `crates/gpui_telegram_desktop/src/i18n/lang_file_parser.rs`

### `Telegram/SourceFiles/lang/lang_instance.cpp`
- **Classes & Structs**: ValueParser, Instance
- **Core Methods & Signals**: ValueParser::ValueParser, ValueParser::appendToResult, QString::fromUtf8, ValueParser::logError, ValueParser::readTag, ValueParser::parse
- **Target**: `LangInstance` in `crates/gpui_telegram_desktop/src/i18n/lang_instance.rs`

### `Telegram/SourceFiles/lang/lang_instance.h`
- **Classes & Structs**: Language, Instance, Pack, PrivateTag
- **Core Methods & Signals**: CloudLangPackName, CustomLanguageId, DefaultLanguage, Instance, switchToId, switchToCustomFile
- **Target**: `LangInstance` in `crates/gpui_telegram_desktop/src/i18n/lang_instance.rs`

### `Telegram/SourceFiles/lang/lang_numbers_animation.cpp`
- **Classes & Structs**: namespace Lang
- **Target**: `LangNumbersAnimation` in `crates/gpui_telegram_desktop/src/i18n/lang_numbers_animation.rs`

### `Telegram/SourceFiles/lang/lang_numbers_animation.h`
- **Classes & Structs**: StartReplacements, ReplaceTag
- **Core Methods & Signals**: Call
- **Target**: `LangNumbersAnimation` in `crates/gpui_telegram_desktop/src/i18n/lang_numbers_animation.rs`

### `Telegram/SourceFiles/lang/lang_pch.h`
- **Style Tokens**: #include <QtCore/QString>, #include <QtCore/QDateTime>, #include <rpl/rpl.h>
- **Target**: `LangPch` in `crates/gpui_telegram_desktop/src/i18n/lang_pch.rs`

### `Telegram/SourceFiles/lang/lang_tag.cpp`
- **Classes & Structs**: PluralsKey, ChoosePluralMethod
- **Core Methods & Signals**: QChar::toLower, PluralsKey::PluralsKey, QString::number, details::IsNonDefaultPlural, ChoosePlural1, ChoosePlural2fil
- **Target**: `LangTag` in `crates/gpui_telegram_desktop/src/i18n/lang_tag.rs`

### `Telegram/SourceFiles/lang/lang_tag.h`
- **Classes & Structs**: CreditsAmount, lngtag_count, ShortenedCount, PluralResult, StartReplacements, ReplaceTag
- **Core Methods & Signals**: Plural, UpdatePluralRules, Call, Replace
- **Target**: `LangTag` in `crates/gpui_telegram_desktop/src/i18n/lang_tag.rs`

