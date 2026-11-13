# Module 08 Specification: UI Text Layout & Formatting Engine (`ui/text`)

## Overview

Module 08 Slice 3 (`ui/text`) delivers the complete, high-performance text rendering, layout, formatting, entity parsing, and localized value representation subsystem for Telegram Desktop reconstructed in pure, idiomatic Rust with GPUI.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eliminated across `crates/gpui_telegram_desktop/src/components/ui/text/`.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the immutable reading checklist. The target architecture provides fine-grained, authentic GPUI rendering components and text layout modules.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, layout algorithm, and formatting routine cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`) and header definitions (`.h:L<start>-L<end>`), extracting anonymous-namespace `k...` constants, pixel math, and non-obvious edge cases.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: 100% of the assigned upstream C++ source and header files are explicitly present, documented, and accounted for in the Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Maintained under modular chunk size constraints (<200 lines per call).

---

## 1. 100% Per-File Reading Manifest (All Assigned Upstream Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/ui/text/format_song_document_name.cpp` | `L1-L91` | `FormatSongNameFor`, `FormatDownloadsName`, `FormatVoiceName`, `tr::lng_in_dlg_video`, `tr::lng_in_dlg_audio`, `tr::lng_in_dlg_video_message`, `tr::lng_in_dlg_sticker`, `tr::lng_in_dlg_sticker_emoji`, `tr::lng_from_you`, `tr::lng_player_message_today`, `tr::lng_player_message_yesterday`, `tr::lng_player_message_date`, `tr::lng_media_round`, `tr::lng_media_audio` | Document type identification (video, voice, video message, sticker, audio), sticker alt emoji fallback, relative voice date formatting (today, yesterday, full date with `langDayOfMonthFull`), `setNoDash(true)` for voice items | `crates/gpui_telegram_desktop/src/components/ui/text/format_song_document_name.rs` (`FormatSongDocumentNameText`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/ui/text/format_song_document_name.h` | `L1-L27` | `FormatSongNameFor`, `FormatDownloadsName`, `FormatVoiceName`, `DocumentData` | Header prototypes for document audio/voice/download title and entity formatting | `crates/gpui_telegram_desktop/src/components/ui/text/format_song_document_name.rs` (`FormatSongDocumentNameText`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/ui/text/format_song_name.cpp` | `L1-L83` | `FormatSongName`, `ComputeComposedName`, `composedName`, `string`, `setNoDash`, `textWithEntities` | Default fallback `u"Unknown Track"_q`, separator dash logic: empty if title/performer missing, `" "` if `_noDash`, en-dash `" \u{2013} "` (`\xe2\x80\x93`) by default; `Semibold` entity applied to performer | `crates/gpui_telegram_desktop/src/components/ui/text/format_song_name.rs` (`FormatSongNameText`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/ui/text/format_song_name.h` | `L1-L38` | `FormatSongName`, `ComposedName` | Structure definition: `ComposedName { title, performer }`, `_noDash` state flag | `crates/gpui_telegram_desktop/src/components/ui/text/format_song_name.rs` (`FormatSongNameText`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/ui/text/format_values.cpp` | `L1-L728` | `kSecondsInYear = 31536000`, `kRecentlyInSeconds = 20 * 3600`, `FormatSizeText`, `FormatDownloadText`, `FormatProgressText`, `FormatDateTime`, `FormatDateTimeSavedFrom`, `FormatDurationText`, `FormatDurationWords`, `FormatDurationWordsSlowmode`, `FormatDurationAndSizeText`, `FormatGifAndSizeText`, `FormatPlayedText`, `FillAmountAndCurrency`, `LookupCurrencyRule`, `KnownCurrency`, `CurrencyName`, `CurrencyEnglishName`, `FormatWithSeparators`, `FormatImageSizeText`, `FormatPhone`, `FormatTTL`, `FormatTTLAfter`, `FormatTTLTiny`, `FormatMuteFor`, `FormatMuteForTiny`, `FormatResetCloudPasswordIn`, `FormatDialogsDate` | Exact byte/KB/MB formatting with 1 decimal digit; `hh:mm:ss` duration math; Telegram Stars `⭐` (`0x2B50` / `kCreditsCurrency = "XTR"`); 100+ international currency rules (decimal, thousands separator, exponent, left/right placement); TTL threshold stages (<24h hours, <7d days, <31d weeks, <11m months, >=1y years); dialogs 20h recent threshold | `crates/gpui_telegram_desktop/src/components/ui/text/format_values.rs` (`FormatValuesText`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/ui/text/format_values.h` | `L1-L64` | `FileStatusSizeReady = 0xFFFFFFF0LL`, `FileStatusSizeLoaded = 0xFFFFFFF1LL`, `FileStatusSizeFailed = 0xFFFFFFF2LL`, `kCreditsCurrency = "XTR"`, `CurrencyRule` | Special file size status constants; Currency rule specification with exponent and separator flags | `crates/gpui_telegram_desktop/src/components/ui/text/format_values.rs` (`FormatValuesText`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/ui/text/text_lottie_custom_emoji.cpp` | `L1-L122` | `LottieCustomEmoji`, `LottieEmojiData`, `LottieEmoji`, `LottieEmojiContext` | Custom emoji integration with vector icon/lottie frames; frame jump on pause (`forceFirstFrame`, `overrideFirstWithLastFrame`, `forceLastFrame`); animation loop bounds `0..framesCount - 1`; text color tinting; repaint callback dispatch | `crates/gpui_telegram_desktop/src/components/ui/text/text_lottie_custom_emoji.rs` (`TextLottieCustomEmojiText`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/ui/text/text_lottie_custom_emoji.h` | `L1-L47` | `LottieCustomEmoji`, `IconDescriptor`, `Icon` | Custom emoji interface definitions, entity data string extraction, paint with context | `crates/gpui_telegram_desktop/src/components/ui/text/text_lottie_custom_emoji.rs` (`TextLottieCustomEmojiText`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/ui/text/text_options.cpp` | `L1-L154` | `HistoryTextOptions`, `HistoryBotOptions`, `HistoryServiceOptions`, `HistoryTextNoMonoOptions`, `HistoryBotNoMonoOptions`, `TextNameOptions`, `TextDialogOptions`, `WebpageTitleOptions`, `WebpageDescriptionOptions`, `InitTextOptions` | Text parse bitmask flags (`TextParseLinks`, `Mentions`, `Hashtags`, `BotCommands`, `Multiline`, `Markdown`, `Colorized`); max dimensions (`TextNameOptions: maxw=4096, maxh=1`; `TextDialogOptions: maxw=columnMaximalWidthLeft*2, maxh=1`; Webpage title/description quotes padding subtraction) | `crates/gpui_telegram_desktop/src/components/ui/text/text_options.rs` (`TextOptionsText`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/ui/text/text_options.h` | `L1-L29` | `InitTextOptions`, `ItemTextDefaultOptions`, `ItemTextBotDefaultOptions`, `ItemTextNoMonoOptions`, `ItemTextBotNoMonoOptions`, `ItemTextServiceOptions`, `WebpageTextTitleOptions`, `WebpageTextDescriptionOptions`, `NameTextOptions`, `DialogTextOptions` | Standard preset accessors for chat history items, bot items, service notifications, dialog list items, and webpage previews | `crates/gpui_telegram_desktop/src/components/ui/text/text_options.rs` (`TextOptionsText`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/tests/test_text.cpp` | `L1-L854` | `RenderTextPadding`, `RenderTextOffscreen`, `SymbolHitBounds`, `SymbolRangeHitBounds`, `FormulaLikeObject`, `CustomEmojiVerticalMetrics`, `CustomEmojiSemantics`, `TextForMimeDataTag`, `TextSelection`, `kMarkupTextOptions` | Test harness for rich text shaping, offscreen rendering verification, hit-testing bounds lookup (`SymbolHitBounds`), formula/custom-emoji replacement object, vertical metrics ascent/descent calculation (`descent = max(height / 5, 1)`), selection bounds adjustment | `crates/gpui_telegram_desktop/src/components/ui/text/text_layout.rs` (`UiTextString`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/ui/item_text_options.cpp` | `L1-L52` | `ItemTextOptions`, `st::msgMinWidth`, `st::msgMaxWidth` | History item text options builder: sets parse flags, clamps bounds to message minimum/maximum widths | `crates/gpui_telegram_desktop/src/components/ui/item_text_options.rs` (`ItemTextOptions`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/ui/item_text_options.h` | `L1-L21` | `ItemTextOptions` | Prototype for item text options generator | `crates/gpui_telegram_desktop/src/components/ui/item_text_options.rs` (`ItemTextOptions`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/ui/controls/text_copied_toast.cpp` | `L1-L63` | `ShowTextCopiedToast`, `Toast::Show` | Transient notification toast indicating copied text or link to clipboard | `crates/gpui_telegram_desktop/src/components/ui/controls/text_copied_toast.rs` (`TextCopiedToast`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/ui/controls/text_copied_toast.h` | `L1-L21` | `ShowTextCopiedToast` | Interface for copying feedback popup | `crates/gpui_telegram_desktop/src/components/ui/controls/text_copied_toast.rs` (`TextCopiedToast`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/lang/lang_text_entity.cpp` | `L1-L135` | `Lang::ReplaceTag`, `Lang::ReplaceEntities`, `Lang::FormatString` | Localization string entity parser: substitutes `{tag}` placeholders while preserving and re-offsetting text entities | `crates/gpui_telegram_desktop/src/components/ui/text/text_entity.rs` (`TextEntityLayout`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/lang/lang_text_entity.h` | `L1-L42` | `LangTextEntity` | Struct for localized template token substitution | `crates/gpui_telegram_desktop/src/components/ui/text/text_entity.rs` (`TextEntityLayout`, `gpui_ui_1to1`) |

---
## 2. Subsystem Architectural Details & Algorithms

### 2.1 Currency Formatting Rules & Math (`format_values.cpp:L176-L212`, `L216-L380`, `L566-L600`)

The currency formatter handles over 100 world currencies with precise placement, spacing, precision exponent, and thousands separators:
- **Telegram Stars (Credits)**:
  `currency == kCreditsCurrency ("XTR")`: returns `QChar(0x2B50)` ('⭐') followed by decimal count (`Lang::FormatCountDecimal`).
- **Negative Amount Prefix**:
  Negative values prepend Unicode minus sign `"−"` (`\xe2\x88\x92`, `U+2212`).
- **Exponent Scaling**:
  `value = |amount| / 10^rule.exponent`.
- **Precision**:
  If `(!rule.stripDotZero && !forceStripDotZero) || floor(value) != value`, precision is `rule.exponent`, otherwise `0`.
- **Grouping and Separator**:
  Standard 3-digit grouping (`\3`). If thousands separator is `'\0'`, spaces or commas are stripped.
- **Placement**:
  If `rule.left` is true: `[prefix][name][optional space][formatted_value]`.
  If `rule.left` is false: `[prefix][formatted_value][optional space][name]`.

```rust
pub struct CurrencyRule {
    pub international: &'static str,
    pub thousands: char,
    pub decimal: char,
    pub left: bool,
    pub space: bool,
    pub exponent: u8,
    pub strip_dot_zero: bool,
}
```

### 2.2 Date and Time Relative Thresholds (`format_values.cpp:L84-L135`, `L709-L725`)

- **Dialogs List Time Format Threshold**:
  `kRecentlyInSeconds = 20 * 3600` (20 hours).
  If the message date is today OR within the last 20 hours (`|lastTime - now| < 20h`), format as ShortFormat time (`hh:mm`).
  Else if within 7 days, format as day of week (`Monday`, `Tuesday`, etc.).
  Otherwise, format as localized date (`dd.MM.yy` or `MM/dd/yy`).
- **Saved From Timestamp Threshold**:
  `kSecondsInYear = 31536000` (365 days).
  If date is today -> `"Today at hh:mm"`.
  If date is yesterday -> `"Yesterday at hh:mm"`.
  If within 1 year -> month and day (`"MMM d at hh:mm"`).
  If >= 1 year -> full date (`"d MMMM yyyy at hh:mm"`).

### 2.3 Song & Voice Track Naming Logic (`format_song_name.cpp:L14-L80`, `format_song_document_name.cpp:L22-L88`)

- Fallback title: `"Unknown Track"` when both `songTitle` and `songPerformer` are empty.
- Separator:
  - If title or performer is empty -> `""`
  - Else if `no_dash == true` -> `" "`
  - Else -> `" – "` (Unicode en-dash `\u{2013}` with surrounding spaces).
- Entity formatting:
  - When `boldOnlyPerformer == false` or `performer` is non-empty, a `Semibold` entity is placed at offset 0 covering the performer string (or the entire title if performer is empty).
- Voice message formatting:
  - Displays `"You"` (`lng_from_you`) or sender name (`fromOriginal()->name()`).
  - Subtitle displays date: `"Today at hh:mm"`, `"Yesterday at hh:mm"`, or full localized date.
  - Automatically sets `no_dash = true`.
### 2.4 Text Options & Presets (`text_options.cpp:L16-L152`)

Text parsing options control which inline elements are detected and how line breaking is applied:
- `HistoryTextOptions`: Links | Mentions | Hashtags | Multiline | Markdown.
- `HistoryBotOptions`: Links | Mentions | Hashtags | BotCommands | Multiline | Markdown.
- `HistoryServiceOptions`: Links | Mentions | Hashtags | Markdown (single-line / auto layout direction).
- `HistoryTextNoMonoOptions`: Links | Mentions | Hashtags | Multiline (omits Markdown backticks).
- `TextNameOptions`: flags = 0, `maxw = 4096`, `maxh = 1`. Single-line clamped text for user/chat titles.
- `TextDialogOptions`: Colorized | Markdown, `maxw = columnMaximalWidthLeft * 2`, `maxh = 1`.
- `WebpageTitleOptions`: Multiline, `maxh = webPageTitleFont->height * 2`, `maxw` adjusted for message bubble and quote padding:
  `msgMaxWidth - msgPadding.left - quote.padding.left - quote.padding.right - msgPadding.right`.

### 2.5 Lottie & Vector Custom Emoji Rendering (`text_lottie_custom_emoji.cpp:L15-L120`, `test_text.cpp:L300-L356`)

Custom emoji are inline objects replacing a placeholder glyph (`\u{FFFC}` Object Replacement Character or custom shortcode):
- **Metrics Calculation**:
  - `height`: Matches font line height or specified size override.
  - `descent = max(height / 5, 1)`.
  - `ascent = height - descent`.
- **Playback States**:
  - `paused`: Holds first frame or specified `jumpTo` frame (`forceLastFrame`, `forceFirstFrame`).
  - Active animation: loops frames from `0` to `framesCount - 1` triggering continuous view invalidation/repaints.
- **Isolated Emoji Sizing**:
  - Standalone single emoji in message: rendered at `3x` scale (jumbo emoji, ~64px).
  - Two to three standalone emoji: rendered at `2x` scale (~44px).
  - Regular in-line text emoji: rendered at `1x` scale matching font size (16–20px).

---

## 3. Implementation Verification & Test Strategy

Comprehensive subsystem tests in `crates/gpui_telegram_desktop/tests/ui_text_subsystem_tests.rs`:
1. `test_format_values_size_formatting`: Tests exact boundary handling for Bytes, KB, and MB with 1-decimal rounding.
2. `test_format_values_durations`: Tests `hh:mm:ss`, `mm:ss`, seconds, slowmode words, and played/total formatters.
3. `test_format_values_currency_international`: Verifies Telegram Stars (`⭐`), USD (`$`), EUR (`€`), GBP (`£`), JPY (`¥`), and separator configurations.
4. `test_format_values_ttl_and_dialog_dates`: Tests TTL stages and the 20-hour relative dialog date rule (`kRecentlyInSeconds = 20 * 3600`).
5. `test_format_song_name_composition`: Tests "Unknown Track", custom performer en-dash separator (` \u{2013} `), `no_dash` space separator, and Semibold entity offsets.
6. `test_format_song_document_name_voice`: Tests voice message date generation (`Today at`, `Yesterday at`) and document type labels.
7. `test_text_options_and_flags`: Tests all parse flag bitmasks, name/dialog line limits, and webpage quote padding subtraction.
8. `test_text_lottie_custom_emoji_metrics`: Tests custom emoji ascent/descent metrics math and frame state transitions.
9. `test_text_layout_isolated_emoji_scaling`: Tests 1x, 2x, 3x isolated emoji scale factors.
10. `test_text_layout_word_wrapping_and_breaks`: Tests newline preservation, word wrapping within available width, and text selection bounds.
