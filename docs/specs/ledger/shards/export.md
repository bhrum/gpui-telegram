# Subsystem Shard: `export`

**Total Source Files**: 35  
**Total Source Lines**: 24,279  
**Primary Feature IDs**: TCG-N43  
**JSON Shard Path**: `docs/specs/ledger/shards/export.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/export/data/export_data_types.cpp` | `TCG-N43` | `ExportDataTypesData` | `crates/gpui_telegram_client/src/export/data/export_data_types.rs` | 3614 | `tests/test_tcg_n43_export_data_types.rs` |
| `Telegram/SourceFiles/export/data/export_data_types.h` | `TCG-N43` | `ExportDataTypesData` | `crates/gpui_telegram_client/src/export/data/export_data_types.rs` | 1463 | `tests/test_tcg_n43_export_data_types.rs` |
| `Telegram/SourceFiles/export/export_api_wrap.cpp` | `TCG-N43` | `ExportApiWrap` | `crates/gpui_telegram_client/src/export/export_api_wrap.rs` | 3811 | `tests/test_tcg_n43_export_api_wrap.rs` |
| `Telegram/SourceFiles/export/export_api_wrap.h` | `TCG-N43` | `ExportApiWrap` | `crates/gpui_telegram_client/src/export/export_api_wrap.rs` | 361 | `tests/test_tcg_n43_export_api_wrap.rs` |
| `Telegram/SourceFiles/export/export_controller.cpp` | `TCG-N43` | `ExportController` | `crates/gpui_telegram_client/src/export/export_controller.rs` | 924 | `tests/test_tcg_n43_export_controller.rs` |
| `Telegram/SourceFiles/export/export_controller.h` | `TCG-N43` | `ExportController` | `crates/gpui_telegram_client/src/export/export_controller.rs` | 154 | `tests/test_tcg_n43_export_controller.rs` |
| `Telegram/SourceFiles/export/export_manager.cpp` | `TCG-N43` | `ExportManager` | `crates/gpui_telegram_client/src/export/export_manager.rs` | 116 | `tests/test_tcg_n43_export_manager.rs` |
| `Telegram/SourceFiles/export/export_manager.h` | `TCG-N43` | `ExportManager` | `crates/gpui_telegram_client/src/export/export_manager.rs` | 57 | `tests/test_tcg_n43_export_manager.rs` |
| `Telegram/SourceFiles/export/export_pch.h` | `TCG-N43` | `ExportPch` | `crates/gpui_telegram_client/src/export/export_pch.rs` | 35 | `tests/test_tcg_n43_export_pch.rs` |
| `Telegram/SourceFiles/export/export_settings.cpp` | `TCG-N43` | `ExportSettings` | `crates/gpui_telegram_client/src/export/export_settings.rs` | 50 | `tests/test_tcg_n43_export_settings.rs` |
| `Telegram/SourceFiles/export/export_settings.h` | `TCG-N43` | `ExportSettings` | `crates/gpui_telegram_client/src/export/export_settings.rs` | 133 | `tests/test_tcg_n43_export_settings.rs` |
| `Telegram/SourceFiles/export/output/export_output_abstract.cpp` | `TCG-N43` | `ExportOutputAbstractOutput` | `crates/gpui_telegram_client/src/export/output/export_output_abstract.rs` | 58 | `tests/test_tcg_n43_export_output_abstract.rs` |
| `Telegram/SourceFiles/export/output/export_output_abstract.h` | `TCG-N43` | `ExportOutputAbstractOutput` | `crates/gpui_telegram_client/src/export/output/export_output_abstract.rs` | 104 | `tests/test_tcg_n43_export_output_abstract.rs` |
| `Telegram/SourceFiles/export/output/export_output_file.cpp` | `TCG-N43` | `ExportOutputFileOutput` | `crates/gpui_telegram_client/src/export/output/export_output_file.rs` | 140 | `tests/test_tcg_n43_export_output_file.rs` |
| `Telegram/SourceFiles/export/output/export_output_file.h` | `TCG-N43` | `ExportOutputFileOutput` | `crates/gpui_telegram_client/src/export/output/export_output_file.rs` | 57 | `tests/test_tcg_n43_export_output_file.rs` |
| `Telegram/SourceFiles/export/output/export_output_html.cpp` | `TCG-N43` | `ExportOutputHtmlOutput` | `crates/gpui_telegram_client/src/export/output/export_output_html.rs` | 6814 | `tests/test_tcg_n43_export_output_html.rs` |
| `Telegram/SourceFiles/export/output/export_output_html.h` | `TCG-N43` | `ExportOutputHtmlOutput` | `crates/gpui_telegram_client/src/export/output/export_output_html.rs` | 186 | `tests/test_tcg_n43_export_output_html.rs` |
| `Telegram/SourceFiles/export/output/export_output_html_and_json.cpp` | `TCG-N43` | `ExportOutputHtmlAndJsonOutput` | `crates/gpui_telegram_client/src/export/output/export_output_html_and_json.rs` | 166 | `tests/test_tcg_n43_export_output_html_and_json.rs` |
| `Telegram/SourceFiles/export/output/export_output_html_and_json.h` | `TCG-N43` | `ExportOutputHtmlAndJsonOutput` | `crates/gpui_telegram_client/src/export/output/export_output_html_and_json.rs` | 69 | `tests/test_tcg_n43_export_output_html_and_json.rs` |
| `Telegram/SourceFiles/export/output/export_output_json.cpp` | `TCG-N43` | `ExportOutputJsonOutput` | `crates/gpui_telegram_client/src/export/output/export_output_json.rs` | 3093 | `tests/test_tcg_n43_export_output_json.rs` |
| `Telegram/SourceFiles/export/output/export_output_json.h` | `TCG-N43` | `ExportOutputJsonOutput` | `crates/gpui_telegram_client/src/export/output/export_output_json.rs` | 114 | `tests/test_tcg_n43_export_output_json.rs` |
| `Telegram/SourceFiles/export/output/export_output_result.h` | `TCG-N43` | `ExportOutputResultOutput` | `crates/gpui_telegram_client/src/export/output/export_output_result.rs` | 48 | `tests/test_tcg_n43_export_output_result.rs` |
| `Telegram/SourceFiles/export/output/export_output_stats.cpp` | `TCG-N43` | `ExportOutputStatsOutput` | `crates/gpui_telegram_client/src/export/output/export_output_stats.rs` | 35 | `tests/test_tcg_n43_export_output_stats.rs` |
| `Telegram/SourceFiles/export/output/export_output_stats.h` | `TCG-N43` | `ExportOutputStatsOutput` | `crates/gpui_telegram_client/src/export/output/export_output_stats.rs` | 33 | `tests/test_tcg_n43_export_output_stats.rs` |
| `Telegram/SourceFiles/export/view/export.style` | `TCG-N43` | `ExportViewStyleSheet` | `crates/gpui_telegram_desktop/src/theme/export/export_style.rs` | 104 | `tests/test_tcg_n43_export.rs` |
| `Telegram/SourceFiles/export/view/export_view_content.cpp` | `TCG-N43` | `ExportViewContentView` | `crates/gpui_telegram_client/src/export/view/export_view_content.rs` | 197 | `tests/test_tcg_n43_export_view_content.rs` |
| `Telegram/SourceFiles/export/view/export_view_content.h` | `TCG-N43` | `ExportViewContentView` | `crates/gpui_telegram_client/src/export/view/export_view_content.rs` | 57 | `tests/test_tcg_n43_export_view_content.rs` |
| `Telegram/SourceFiles/export/view/export_view_panel_controller.cpp` | `TCG-N43` | `ExportViewPanelControllerView` | `crates/gpui_telegram_client/src/export/view/export_view_panel_controller.rs` | 438 | `tests/test_tcg_n43_export_view_panel_controller.rs` |
| `Telegram/SourceFiles/export/view/export_view_panel_controller.h` | `TCG-N43` | `ExportViewPanelControllerView` | `crates/gpui_telegram_client/src/export/view/export_view_panel_controller.rs` | 90 | `tests/test_tcg_n43_export_view_panel_controller.rs` |
| `Telegram/SourceFiles/export/view/export_view_progress.cpp` | `TCG-N43` | `ExportViewProgressView` | `crates/gpui_telegram_client/src/export/view/export_view_progress.rs` | 382 | `tests/test_tcg_n43_export_view_progress.rs` |
| `Telegram/SourceFiles/export/view/export_view_progress.h` | `TCG-N43` | `ExportViewProgressView` | `crates/gpui_telegram_client/src/export/view/export_view_progress.rs` | 62 | `tests/test_tcg_n43_export_view_progress.rs` |
| `Telegram/SourceFiles/export/view/export_view_settings.cpp` | `TCG-N43` | `ExportViewSettingsView` | `crates/gpui_telegram_client/src/export/view/export_view_settings.rs` | 989 | `tests/test_tcg_n43_export_view_settings.rs` |
| `Telegram/SourceFiles/export/view/export_view_settings.h` | `TCG-N43` | `ExportViewSettingsView` | `crates/gpui_telegram_client/src/export/view/export_view_settings.rs` | 135 | `tests/test_tcg_n43_export_view_settings.rs` |
| `Telegram/SourceFiles/export/view/export_view_top_bar.cpp` | `TCG-N43` | `ExportViewTopBarView` | `crates/gpui_telegram_client/src/export/view/export_view_top_bar.rs` | 134 | `tests/test_tcg_n43_export_view_top_bar.rs` |
| `Telegram/SourceFiles/export/view/export_view_top_bar.h` | `TCG-N43` | `ExportViewTopBarView` | `crates/gpui_telegram_client/src/export/view/export_view_top_bar.rs` | 56 | `tests/test_tcg_n43_export_view_top_bar.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/export/data/export_data_types.cpp`
- **Classes & Structs**: Type, Style, Alignment, Source, Kind, State, DialogType
- **Core Methods & Signals**: QString::fromUtf8, QString::number, QByteArray::number, InlineButtonAction::TypeToString, HistoryMessageMarkupButton::TypeToString, Reaction::TypeToString
- **Target**: `ExportDataTypesData` in `crates/gpui_telegram_client/src/export/data/export_data_types.rs`

### `Telegram/SourceFiles/export/data/export_data_types.h`
- **Classes & Structs**: Settings, TextPart, Type, InlineButtonPeerType, InlineButtonAction, RichButtonStyle, RichButtonAlignment, RichButtonPayload, RichText, RichCaption, RichBlock, RichTaskState
- **Core Methods & Signals**: PeerColorIndex, PeerToBareId, ApplicationColorIndex, DomainApplicationId, ParseString, FillLeft
- **Target**: `ExportDataTypesData` in `crates/gpui_telegram_client/src/export/data/export_data_types.rs`

### `Telegram/SourceFiles/export/export_api_wrap.cpp`
- **Classes & Structs**: LocationKey, RichMessageCallbacks, ApiWrap, Step, Request, DialogType, Type, Kind, ListContent, QuoteContent, Location, Original
- **Core Methods & Signals**: Settings::Type, v::match, Data::RefreshFileReference, MediaSettings::Type, LoadedFileCache::LoadedFileCache, LoadedFileCache::save
- **Target**: `ExportApiWrap` in `crates/gpui_telegram_client/src/export/export_api_wrap.rs`

### `Telegram/SourceFiles/export/export_api_wrap.h`
- **Classes & Structs**: File, Chat, Document, FileLocation, PersonalInfo, UserpicsInfo, UserpicsSlice, StoriesInfo, StoriesSlice, ProfileMusicInfo, ProfileMusicSlice, ContactsList
- **Core Methods & Signals**: errors, ioErrors, skipFile, cancelExportFast, sendNextStartRequest, requestUserpicsCount
- **Target**: `ExportApiWrap` in `crates/gpui_telegram_client/src/export/export_api_wrap.rs`

### `Telegram/SourceFiles/export/export_controller.cpp`
- **Classes & Structs**: ControllerObject, Step, DownloadProgress, Type
- **Core Methods & Signals**: ControllerObject::ControllerObject, ControllerObject::state, ControllerObject::stopped, ControllerObject::setState, ControllerObject::ioError, ControllerObject::ioCatchError
- **Target**: `ExportController` in `crates/gpui_telegram_client/src/export/export_controller.rs`

### `Telegram/SourceFiles/export/export_controller.h`
- **Classes & Structs**: Instance, ControllerObject, Settings, Environment, PasswordCheckState, ProcessingState, Step, EntityType, ApiErrorState, OutputErrorState, CancelledState, FinishedState
- **Core Methods & Signals**: Controller, state, startExport, skipFile, cancelExportFast, rpl::state
- **Target**: `ExportController` in `crates/gpui_telegram_client/src/export/export_controller.rs`

### `Telegram/SourceFiles/export/export_manager.h`
- **Classes & Structs**: PeerData, BoxContent, Session, Controller, PanelController, Manager
- **Core Methods & Signals**: Manager, start, startTopic, stop, setupPanel, rpl::currentView
- **Target**: `ExportManager` in `crates/gpui_telegram_client/src/export/export_manager.rs`

### `Telegram/SourceFiles/export/export_pch.h`
- **Style Tokens**: #include <QtCore/QFile>, #include <QtCore/QFileInfo>, #include <QtCore/QDir>, #include <QtCore/QSize>, #include <QtCore/QTextStream>, #include <QtCore/QDateTime>
- **Target**: `ExportPch` in `crates/gpui_telegram_client/src/export/export_pch.rs`

### `Telegram/SourceFiles/export/export_settings.cpp`
- **Classes & Structs**: Format
- **Core Methods & Signals**: MediaSettings::validate, Settings::validate
- **Target**: `ExportSettings` in `crates/gpui_telegram_client/src/export/export_settings.rs`

### `Telegram/SourceFiles/export/export_settings.h`
- **Classes & Structs**: Format, MediaSettings, Type, Settings, Environment, Types
- **Core Methods & Signals**: Output::Format, validate, is_flag_type, DefaultTypes, onlySinglePeer, onlySingleTopic
- **Target**: `ExportSettings` in `crates/gpui_telegram_client/src/export/export_settings.rs`

