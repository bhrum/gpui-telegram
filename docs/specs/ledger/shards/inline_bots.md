# Subsystem Shard: `inline_bots`

**Total Source Files**: 20  
**Total Source Lines**: 10,653  
**Primary Feature IDs**: TCG-N27  
**JSON Shard Path**: `docs/specs/ledger/shards/inline_bots.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp` | `TCG-N27` | `BotAttachWebView` | `crates/gpui_telegram_desktop/src/components/bots/bot_attach_web_view.rs` | 3230 | `tests/test_tcg_n27_bot_attach_web_view.rs` |
| `Telegram/SourceFiles/inline_bots/bot_attach_web_view.h` | `TCG-N27` | `BotAttachWebView` | `crates/gpui_telegram_desktop/src/components/bots/bot_attach_web_view.rs` | 530 | `tests/test_tcg_n27_bot_attach_web_view.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.cpp` | `TCG-N27` | `InlineBotConfirmPrepared` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_confirm_prepared.rs` | 240 | `tests/test_tcg_n27_inline_bot_confirm_prepared.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.h` | `TCG-N27` | `InlineBotConfirmPrepared` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_confirm_prepared.rs` | 31 | `tests/test_tcg_n27_inline_bot_confirm_prepared.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_downloads.cpp` | `TCG-N27` | `InlineBotDownloads` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_downloads.rs` | 429 | `tests/test_tcg_n27_inline_bot_downloads.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_downloads.h` | `TCG-N27` | `InlineBotDownloads` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_downloads.rs` | 105 | `tests/test_tcg_n27_inline_bot_downloads.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.cpp` | `TCG-N27` | `InlineBotLayoutInternal` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_internal.rs` | 2007 | `tests/test_tcg_n27_inline_bot_layout_internal.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.h` | `TCG-N27` | `InlineBotLayoutInternal` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_internal.rs` | 487 | `tests/test_tcg_n27_inline_bot_layout_internal.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.cpp` | `TCG-N27` | `InlineBotLayoutItem` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_item.rs` | 302 | `tests/test_tcg_n27_inline_bot_layout_item.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.h` | `TCG-N27` | `InlineBotLayoutItem` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_item.rs` | 168 | `tests/test_tcg_n27_inline_bot_layout_item.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_result.cpp` | `TCG-N27` | `InlineBotResult` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_result.rs` | 530 | `tests/test_tcg_n27_inline_bot_result.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_result.h` | `TCG-N27` | `InlineBotResult` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_result.rs` | 148 | `tests/test_tcg_n27_inline_bot_result.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_send_data.cpp` | `TCG-N27` | `InlineBotSendData` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_send_data.rs` | 197 | `tests/test_tcg_n27_inline_bot_send_data.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_send_data.h` | `TCG-N27` | `InlineBotSendData` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_send_data.rs` | 373 | `tests/test_tcg_n27_inline_bot_send_data.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_storage.cpp` | `TCG-N27` | `InlineBotStorage` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_storage.rs` | 181 | `tests/test_tcg_n27_inline_bot_storage.rs` |
| `Telegram/SourceFiles/inline_bots/inline_bot_storage.h` | `TCG-N27` | `InlineBotStorage` | `crates/gpui_telegram_desktop/src/components/bots/inline_bot_storage.rs` | 52 | `tests/test_tcg_n27_inline_bot_storage.rs` |
| `Telegram/SourceFiles/inline_bots/inline_results_inner.cpp` | `TCG-N27` | `InlineResultsInner` | `crates/gpui_telegram_desktop/src/components/bots/inline_results_inner.rs` | 754 | `tests/test_tcg_n27_inline_results_inner.rs` |
| `Telegram/SourceFiles/inline_bots/inline_results_inner.h` | `TCG-N27` | `InlineResultsInner` | `crates/gpui_telegram_desktop/src/components/bots/inline_results_inner.rs` | 192 | `tests/test_tcg_n27_inline_results_inner.rs` |
| `Telegram/SourceFiles/inline_bots/inline_results_widget.cpp` | `TCG-N27` | `InlineResultsWidget` | `crates/gpui_telegram_desktop/src/components/bots/inline_results_widget.rs` | 534 | `tests/test_tcg_n27_inline_results_widget.rs` |
| `Telegram/SourceFiles/inline_bots/inline_results_widget.h` | `TCG-N27` | `InlineResultsWidget` | `crates/gpui_telegram_desktop/src/components/bots/inline_results_widget.rs` | 163 | `tests/test_tcg_n27_inline_results_widget.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp`
- **Classes & Structs**: BotAction, State, Show, MoneyState, Section, Result, Flag, Button, CreditsFormDataPtr, CreditsReceiptPtr, UniqueLayer, ObjectBox
- **Core Methods & Signals**: style::DevicePixelRatio, Data::FileOrigin, tr::lng_action_you_joined_by_request_channel, tr::lng_action_you_joined_by_request, tr::lng_group_request_declined_channel, tr::lng_group_request_declined
- **Target**: `BotAttachWebView` in `crates/gpui_telegram_desktop/src/components/bots/bot_attach_web_view.rs`

### `Telegram/SourceFiles/inline_bots/bot_attach_web_view.h`
- **Classes & Structs**: Thread, Show, GenericBox, DropdownMenu, Panel, DownloadsEntry, Session, SessionShow, SessionController, DocumentMedia, NonPanelPaymentForm, CheckoutResult
- **Core Methods & Signals**: Api::SendAction, SendMenu::Details, WebViewInstance, activate, close, resolve
- **Target**: `BotAttachWebView` in `crates/gpui_telegram_desktop/src/components/bots/bot_attach_web_view.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.cpp`
- **Classes & Structs**: PreviewDelegate, PreviewWrap
- **Core Methods & Signals**: PreviewDelegate::PreviewDelegate, PreviewDelegate::elementAnimationsPaused, PreviewDelegate::elementPathShiftGradient, PreviewDelegate::elementContext, PreviewWrap::PreviewWrap, Theme::DefaultChatThemeOn
- **Target**: `InlineBotConfirmPrepared` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_confirm_prepared.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.h`
- **Classes & Structs**: Thread, Session, GenericBox
- **Target**: `InlineBotConfirmPrepared` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_confirm_prepared.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_downloads.h`
- **Classes & Structs**: webFileLoader, Session, GenericBox, Downloads, StartArgs, List, Loader, DownloadBoxArgs, DownloadId
- **Core Methods & Signals**: Downloads, start, action, read, write, load
- **Target**: `InlineBotDownloads` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_downloads.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.cpp`
- **Classes & Structs**: TextState, PhotoSize
- **Core Methods & Signals**: FileBase::FileBase, FileBase::getShownDocument, FileBase::content_width, style::ConvertScale, FileBase::content_height, FileBase::content_duration
- **Target**: `InlineBotLayoutInternal` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_internal.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_layout_internal.h`
- **Classes & Structs**: SinglePlayer, PhotoMedia, DocumentMedia, FileBase, DeleteSavedGifClickHandler, Gif, StateFlag, AnimationData, Photo, Sticker, Video, CancelFileClickHandler
- **Core Methods & Signals**: FileBase, content_width, content_height, content_duration, onClickImpl, Gif
- **Target**: `InlineBotLayoutInternal` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_internal.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.cpp`
- **Classes & Structs**: Type
- **Core Methods & Signals**: ItemBase::getResult, ItemBase::getDocument, ItemBase::getPhoto, ItemBase::getPreviewDocument, ItemBase::getPreviewPhoto, ItemBase::preload
- **Target**: `InlineBotLayoutItem` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_item.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.h`
- **Classes & Structs**: Image, PathShiftGradient, Result, ItemBase, PaintContext, SendClickHandler, OpenFileClickHandler, Context, DocumentItems
- **Core Methods & Signals**: lastRow, onClick, inlineItemLayoutChanged, inlineItemVisible, inlineItemRepaint, inlineItemFileOrigin
- **Target**: `InlineBotLayoutItem` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_layout_item.rs`

### `Telegram/SourceFiles/inline_bots/inline_bot_result.cpp`
- **Classes & Structs**: Type, Flag
- **Core Methods & Signals**: Result::Result, Result::Create, Images::FromWebDocument, Api::EntitiesFromMTP, style::DevicePixelRatio, Result::onChoose
- **Target**: `InlineBotResult` in `crates/gpui_telegram_desktop/src/components/bots/inline_bot_result.rs`

