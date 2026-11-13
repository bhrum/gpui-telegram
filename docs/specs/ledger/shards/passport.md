# Subsystem Shard: `passport`

**Total Source Files**: 29  
**Total Source Lines**: 11,533  
**Primary Feature IDs**: TCG-N02  
**JSON Shard Path**: `docs/specs/ledger/shards/passport.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/passport/passport.style` | `TCG-N02` | `PassportStyleSheet` | `crates/gpui_telegram_desktop/src/theme/passport/passport_style.rs` | 197 | `tests/test_tcg_n02_passport.rs` |
| `Telegram/SourceFiles/passport/passport_edit_identity_box.cpp` | `TCG-N02` | `PassportEditIdentityBox` | `crates/gpui_telegram_core/src/passport/passport_edit_identity_box.rs` | 307 | `tests/test_tcg_n02_passport_edit_identity_box.rs` |
| `Telegram/SourceFiles/passport/passport_edit_identity_box.h` | `TCG-N02` | `PassportEditIdentityBox` | `crates/gpui_telegram_core/src/passport/passport_edit_identity_box.rs` | 64 | `tests/test_tcg_n02_passport_edit_identity_box.rs` |
| `Telegram/SourceFiles/passport/passport_encryption.cpp` | `TCG-N02` | `PassportEncryption` | `crates/gpui_telegram_core/src/passport/passport_encryption.rs` | 362 | `tests/test_tcg_n02_passport_encryption.rs` |
| `Telegram/SourceFiles/passport/passport_encryption.h` | `TCG-N02` | `PassportEncryption` | `crates/gpui_telegram_core/src/passport/passport_encryption.rs` | 57 | `tests/test_tcg_n02_passport_encryption.rs` |
| `Telegram/SourceFiles/passport/passport_form_controller.cpp` | `TCG-N02` | `PassportFormController` | `crates/gpui_telegram_core/src/passport/passport_form_controller.rs` | 2768 | `tests/test_tcg_n02_passport_form_controller.rs` |
| `Telegram/SourceFiles/passport/passport_form_controller.h` | `TCG-N02` | `PassportFormController` | `crates/gpui_telegram_core/src/passport/passport_form_controller.rs` | 603 | `tests/test_tcg_n02_passport_form_controller.rs` |
| `Telegram/SourceFiles/passport/passport_form_row.cpp` | `TCG-N02` | `PassportFormRow` | `crates/gpui_telegram_core/src/passport/passport_form_row.rs` | 91 | `tests/test_tcg_n02_passport_form_row.rs` |
| `Telegram/SourceFiles/passport/passport_form_row.h` | `TCG-N02` | `PassportFormRow` | `crates/gpui_telegram_core/src/passport/passport_form_row.rs` | 19 | `tests/test_tcg_n02_passport_form_row.rs` |
| `Telegram/SourceFiles/passport/passport_form_view_controller.cpp` | `TCG-N02` | `PassportFormViewController` | `crates/gpui_telegram_core/src/passport/passport_form_view_controller.rs` | 560 | `tests/test_tcg_n02_passport_form_view_controller.rs` |
| `Telegram/SourceFiles/passport/passport_form_view_controller.h` | `TCG-N02` | `PassportFormViewController` | `crates/gpui_telegram_core/src/passport/passport_form_view_controller.rs` | 77 | `tests/test_tcg_n02_passport_form_view_controller.rs` |
| `Telegram/SourceFiles/passport/passport_panel.cpp` | `TCG-N02` | `PassportPanel` | `crates/gpui_telegram_core/src/passport/passport_panel.rs` | 109 | `tests/test_tcg_n02_passport_panel.rs` |
| `Telegram/SourceFiles/passport/passport_panel.h` | `TCG-N02` | `PassportPanel` | `crates/gpui_telegram_core/src/passport/passport_panel.rs` | 52 | `tests/test_tcg_n02_passport_panel.rs` |
| `Telegram/SourceFiles/passport/passport_panel_controller.cpp` | `TCG-N02` | `PassportPanelController` | `crates/gpui_telegram_core/src/passport/passport_panel_controller.rs` | 1474 | `tests/test_tcg_n02_passport_panel_controller.rs` |
| `Telegram/SourceFiles/passport/passport_panel_controller.h` | `TCG-N02` | `PassportPanelController` | `crates/gpui_telegram_core/src/passport/passport_panel_controller.rs` | 190 | `tests/test_tcg_n02_passport_panel_controller.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_contact.cpp` | `TCG-N02` | `PassportPanelEditContact` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_contact.rs` | 453 | `tests/test_tcg_n02_passport_panel_edit_contact.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_contact.h` | `TCG-N02` | `PassportPanelEditContact` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_contact.rs` | 94 | `tests/test_tcg_n02_passport_panel_edit_contact.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_document.cpp` | `TCG-N02` | `PassportPanelEditDocument` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_document.rs` | 722 | `tests/test_tcg_n02_passport_panel_edit_document.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_document.h` | `TCG-N02` | `PassportPanelEditDocument` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_document.rs` | 183 | `tests/test_tcg_n02_passport_panel_edit_document.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_scans.cpp` | `TCG-N02` | `PassportPanelEditScans` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_scans.rs` | 981 | `tests/test_tcg_n02_passport_panel_edit_scans.rs` |
| `Telegram/SourceFiles/passport/passport_panel_edit_scans.h` | `TCG-N02` | `PassportPanelEditScans` | `crates/gpui_telegram_core/src/passport/passport_panel_edit_scans.rs` | 153 | `tests/test_tcg_n02_passport_panel_edit_scans.rs` |
| `Telegram/SourceFiles/passport/passport_panel_form.cpp` | `TCG-N02` | `PassportPanelForm` | `crates/gpui_telegram_core/src/passport/passport_panel_form.rs` | 180 | `tests/test_tcg_n02_passport_panel_form.rs` |
| `Telegram/SourceFiles/passport/passport_panel_form.h` | `TCG-N02` | `PassportPanelForm` | `crates/gpui_telegram_core/src/passport/passport_panel_form.rs` | 58 | `tests/test_tcg_n02_passport_panel_form.rs` |
| `Telegram/SourceFiles/passport/passport_panel_password.cpp` | `TCG-N02` | `PassportPanelPassword` | `crates/gpui_telegram_core/src/passport/passport_panel_password.rs` | 267 | `tests/test_tcg_n02_passport_panel_password.rs` |
| `Telegram/SourceFiles/passport/passport_panel_password.h` | `TCG-N02` | `PassportPanelPassword` | `crates/gpui_telegram_core/src/passport/passport_panel_password.rs` | 74 | `tests/test_tcg_n02_passport_panel_password.rs` |
| `Telegram/SourceFiles/passport/ui/passport_details_row.cpp` | `TCG-N02` | `PassportDetailsRowUi` | `crates/gpui_telegram_core/src/passport/ui/passport_details_row.rs` | 1179 | `tests/test_tcg_n02_passport_details_row.rs` |
| `Telegram/SourceFiles/passport/ui/passport_details_row.h` | `TCG-N02` | `PassportDetailsRowUi` | `crates/gpui_telegram_core/src/passport/ui/passport_details_row.rs` | 85 | `tests/test_tcg_n02_passport_details_row.rs` |
| `Telegram/SourceFiles/passport/ui/passport_form_row.cpp` | `TCG-N02` | `PassportFormRowUi` | `crates/gpui_telegram_core/src/passport/ui/passport_form_row.rs` | 126 | `tests/test_tcg_n02_passport_form_row.rs` |
| `Telegram/SourceFiles/passport/ui/passport_form_row.h` | `TCG-N02` | `PassportFormRowUi` | `crates/gpui_telegram_core/src/passport/ui/passport_form_row.rs` | 48 | `tests/test_tcg_n02_passport_form_row.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/passport/passport.style`
- **Core Methods & Signals**: margins, FlatLabel, align, TextStyle, font, size
- **Style Tokens**: passportPasswordPadding: margins, passportPasswordLabel: FlatLabel, minWidth: 275px, align: align, passportPasswordLabelBold: FlatLabel, style: TextStyle
- **Target**: `PassportStyleSheet` in `crates/gpui_telegram_desktop/src/theme/passport/passport_style.rs`

### `Telegram/SourceFiles/passport/passport_edit_identity_box.cpp`
- **Classes & Structs**: ScanButton
- **Core Methods & Signals**: ScanButton::ScanButton, Ui::NameTextOptions, ScanButton::setImage, ScanButton::setDescription, ScanButton::resizeGetHeight, ScanButton::countAvailableWidth
- **Target**: `PassportEditIdentityBox` in `crates/gpui_telegram_core/src/passport/passport_edit_identity_box.rs`

### `Telegram/SourceFiles/passport/passport_edit_identity_box.h`
- **Classes & Structs**: LinkButton, InputField, PanelController, ScanInfo, ScanButton, IdentityData, IdentityBox
- **Core Methods & Signals**: IdentityBox, prepare, setInnerFocus, resizeEvent, chooseScan, encryptScan
- **Target**: `PassportEditIdentityBox` in `crates/gpui_telegram_core/src/passport/passport_edit_identity_box.rs`

### `Telegram/SourceFiles/passport/passport_encryption.cpp`
- **Classes & Structs**: AesParams
- **Core Methods & Signals**: bytes::make_vector, openssl::Sha512, bytes::vector, ranges::accumulate, bytes::set_random, QByteArray::fromRawData
- **Target**: `PassportEncryption` in `crates/gpui_telegram_core/src/passport/passport_encryption.rs`

### `Telegram/SourceFiles/passport/passport_encryption.h`
- **Classes & Structs**: EncryptedData
- **Core Methods & Signals**: GenerateSecretBytes, EncryptSecureSecret, DecryptSecureSecret, SerializeData, DeserializeData, EncryptData
- **Target**: `PassportEncryption` in `crates/gpui_telegram_core/src/passport/passport_encryption.rs`

### `Telegram/SourceFiles/passport/passport_form_controller.cpp`
- **Classes & Structs**: Type, Flag, Scheme
- **Core Methods & Signals**: Images::Read, QByteArray::fromRawData, QString::fromUtf8, qthelp::validate_url, QJsonDocument::fromJson, FormRequest::FormRequest
- **Target**: `PassportFormController` in `crates/gpui_telegram_core/src/passport/passport_form_controller.rs`

### `Telegram/SourceFiles/passport/passport_form_controller.h`
- **Classes & Structs**: mtpFileLoader, UploadSecureDone, UploadSecureProgress, SessionController, Session, SentCodeCall, EditDocumentCountry, SavedCredentials, ViewController, FormRequest, LoadStatus, Status
- **Core Methods & Signals**: v::is_null, NonceNameByScope, set, offset, status, UploadScanDataPointer
- **Target**: `PassportFormController` in `crates/gpui_telegram_core/src/passport/passport_form_controller.rs`

### `Telegram/SourceFiles/passport/passport_form_row.h`
- **Classes & Structs**: FadeWrapScaled
- **Target**: `PassportFormRow` in `crates/gpui_telegram_core/src/passport/passport_form_row.rs`

### `Telegram/SourceFiles/passport/passport_form_view_controller.cpp`
- **Classes & Structs**: Pair, Type, ValueClass
- **Core Methods & Signals**: ranges::count_if, ranges::any_of, Scope::Scope, ranges::accumulate, tr::lng_passport_identity_passport, tr::lng_passport_identity_passport_upload
- **Target**: `PassportFormViewController` in `crates/gpui_telegram_core/src/passport/passport_form_view_controller.rs`

### `Telegram/SourceFiles/passport/passport_form_view_controller.h`
- **Classes & Structs**: Scope, Type, ScopeRow, ViewController
- **Core Methods & Signals**: Scope, CanHaveErrors, ValidateForm, ComputeScopes, ComputeScopeRowReadyString, ComputeScopeRow
- **Target**: `PassportFormViewController` in `crates/gpui_telegram_core/src/passport/passport_form_view_controller.rs`

