# Subsystem Shard: `payments`

**Total Source Files**: 53  
**Total Source Lines**: 10,790  
**Primary Feature IDs**: CAP-06, TCG-N14, TCG-N24  
**JSON Shard Path**: `docs/specs/ledger/shards/payments.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/payments/payments_checkout_process.cpp` | `CAP-06` | `PaymentsCheckoutProcess` | `crates/gpui_telegram_proto/src/payments/payments_checkout_process.rs` | 1008 | `tests/test_cap_06_payments_checkout_process.rs` |
| `Telegram/SourceFiles/payments/payments_checkout_process.h` | `CAP-06` | `PaymentsCheckoutProcess` | `crates/gpui_telegram_proto/src/payments/payments_checkout_process.rs` | 208 | `tests/test_cap_06_payments_checkout_process.rs` |
| `Telegram/SourceFiles/payments/payments_form.cpp` | `CAP-06` | `PaymentsForm` | `crates/gpui_telegram_proto/src/payments/payments_form.rs` | 1357 | `tests/test_cap_06_payments_form.rs` |
| `Telegram/SourceFiles/payments/payments_form.h` | `CAP-06` | `PaymentsForm` | `crates/gpui_telegram_proto/src/payments/payments_form.rs` | 424 | `tests/test_cap_06_payments_form.rs` |
| `Telegram/SourceFiles/payments/payments_non_panel_process.cpp` | `CAP-06` | `PaymentsNonPanelProcess` | `crates/gpui_telegram_proto/src/payments/payments_non_panel_process.rs` | 194 | `tests/test_cap_06_payments_non_panel_process.rs` |
| `Telegram/SourceFiles/payments/payments_non_panel_process.h` | `CAP-06` | `PaymentsNonPanelProcess` | `crates/gpui_telegram_proto/src/payments/payments_non_panel_process.rs` | 50 | `tests/test_cap_06_payments_non_panel_process.rs` |
| `Telegram/SourceFiles/payments/payments_reaction_process.cpp` | `TCG-N14` | `PaymentsReactionProcess` | `crates/gpui_telegram_proto/src/payments/payments_reaction_process.rs` | 309 | `tests/test_tcg_n14_payments_reaction_process.rs` |
| `Telegram/SourceFiles/payments/payments_reaction_process.h` | `TCG-N14` | `PaymentsReactionProcess` | `crates/gpui_telegram_proto/src/payments/payments_reaction_process.rs` | 54 | `tests/test_tcg_n14_payments_reaction_process.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_api_client.cpp` | `CAP-06` | `SmartglocalApiClientSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_api_client.rs` | 195 | `tests/test_cap_06_smartglocal_api_client.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_api_client.h` | `CAP-06` | `SmartglocalApiClientSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_api_client.rs` | 50 | `tests/test_cap_06_smartglocal_api_client.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_callbacks.h` | `TCG-N24` | `SmartglocalCallbacksSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_callbacks.rs` | 19 | `tests/test_tcg_n24_smartglocal_callbacks.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_card.cpp` | `CAP-06` | `SmartglocalCardSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_card.rs` | 63 | `tests/test_cap_06_smartglocal_card.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_card.h` | `CAP-06` | `SmartglocalCardSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_card.rs` | 51 | `tests/test_cap_06_smartglocal_card.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_error.cpp` | `CAP-06` | `SmartglocalErrorSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_error.rs` | 69 | `tests/test_cap_06_smartglocal_error.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_error.h` | `CAP-06` | `SmartglocalErrorSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_error.rs` | 59 | `tests/test_cap_06_smartglocal_error.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_token.cpp` | `CAP-06` | `SmartglocalTokenSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_token.rs` | 46 | `tests/test_cap_06_smartglocal_token.rs` |
| `Telegram/SourceFiles/payments/smartglocal/smartglocal_token.h` | `CAP-06` | `SmartglocalTokenSmartglocal` | `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_token.rs` | 47 | `tests/test_cap_06_smartglocal_token.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_address.h` | `CAP-06` | `StripeAddressStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_address.rs` | 18 | `tests/test_cap_06_stripe_address.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_api_client.cpp` | `CAP-06` | `StripeApiClientStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_api_client.rs` | 167 | `tests/test_cap_06_stripe_api_client.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_api_client.h` | `CAP-06` | `StripeApiClientStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_api_client.rs` | 45 | `tests/test_cap_06_stripe_api_client.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_callbacks.h` | `TCG-N24` | `StripeCallbacksStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_callbacks.rs` | 19 | `tests/test_tcg_n24_stripe_callbacks.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card.cpp` | `CAP-06` | `StripeCardStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card.rs` | 188 | `tests/test_cap_06_stripe_card.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card.h` | `CAP-06` | `StripeCardStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card.rs` | 99 | `tests/test_cap_06_stripe_card.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card_params.cpp` | `CAP-06` | `StripeCardParamsStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card_params.rs` | 33 | `tests/test_cap_06_stripe_card_params.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card_params.h` | `CAP-06` | `StripeCardParamsStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card_params.rs` | 32 | `tests/test_cap_06_stripe_card_params.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card_validator.cpp` | `CAP-06` | `StripeCardValidatorStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card_validator.rs` | 304 | `tests/test_cap_06_stripe_card_validator.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_card_validator.h` | `CAP-06` | `StripeCardValidatorStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_card_validator.rs` | 57 | `tests/test_cap_06_stripe_card_validator.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_decode.cpp` | `CAP-06` | `StripeDecodeStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_decode.rs` | 23 | `tests/test_cap_06_stripe_decode.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_decode.h` | `CAP-06` | `StripeDecodeStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_decode.rs` | 19 | `tests/test_cap_06_stripe_decode.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_error.cpp` | `CAP-06` | `StripeErrorStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_error.rs` | 107 | `tests/test_cap_06_stripe_error.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_error.h` | `CAP-06` | `StripeErrorStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_error.rs` | 66 | `tests/test_cap_06_stripe_error.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_form_encodable.h` | `CAP-06` | `StripeFormEncodableStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_form_encodable.rs` | 39 | `tests/test_cap_06_stripe_form_encodable.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_form_encoder.cpp` | `CAP-06` | `StripeFormEncoderStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_form_encoder.rs` | 41 | `tests/test_cap_06_stripe_form_encoder.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_form_encoder.h` | `CAP-06` | `StripeFormEncoderStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_form_encoder.rs` | 21 | `tests/test_cap_06_stripe_form_encoder.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_payment_configuration.h` | `CAP-06` | `StripePaymentConfigurationStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_payment_configuration.rs` | 29 | `tests/test_cap_06_stripe_payment_configuration.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_pch.h` | `CAP-06` | `StripePchStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_pch.rs` | 13 | `tests/test_cap_06_stripe_pch.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_token.cpp` | `CAP-06` | `StripeTokenStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_token.rs` | 65 | `tests/test_cap_06_stripe_token.rs` |
| `Telegram/SourceFiles/payments/stripe/stripe_token.h` | `CAP-06` | `StripeTokenStripe` | `crates/gpui_telegram_proto/src/payments/stripe/stripe_token.rs` | 49 | `tests/test_cap_06_stripe_token.rs` |
| `Telegram/SourceFiles/payments/ui/payments.style` | `CAP-06` | `PaymentsUiStyleSheet` | `crates/gpui_telegram_desktop/src/theme/payments/payments_style.rs` | 167 | `tests/test_cap_06_payments.rs` |
| `Telegram/SourceFiles/payments/ui/payments_edit_card.cpp` | `CAP-06` | `PaymentsEditCardUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_edit_card.rs` | 439 | `tests/test_cap_06_payments_edit_card.rs` |
| `Telegram/SourceFiles/payments/ui/payments_edit_card.h` | `CAP-06` | `PaymentsEditCardUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_edit_card.rs` | 72 | `tests/test_cap_06_payments_edit_card.rs` |
| `Telegram/SourceFiles/payments/ui/payments_edit_information.cpp` | `CAP-06` | `PaymentsEditInformationUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_edit_information.rs` | 284 | `tests/test_cap_06_payments_edit_information.rs` |
| `Telegram/SourceFiles/payments/ui/payments_edit_information.h` | `CAP-06` | `PaymentsEditInformationUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_edit_information.rs` | 80 | `tests/test_cap_06_payments_edit_information.rs` |
| `Telegram/SourceFiles/payments/ui/payments_field.cpp` | `CAP-06` | `PaymentsFieldUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_field.rs` | 746 | `tests/test_cap_06_payments_field.rs` |
| `Telegram/SourceFiles/payments/ui/payments_field.h` | `CAP-06` | `PaymentsFieldUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_field.rs` | 143 | `tests/test_cap_06_payments_field.rs` |
| `Telegram/SourceFiles/payments/ui/payments_form_summary.cpp` | `CAP-06` | `PaymentsFormSummaryUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_form_summary.rs` | 609 | `tests/test_cap_06_payments_form_summary.rs` |
| `Telegram/SourceFiles/payments/ui/payments_form_summary.h` | `CAP-06` | `PaymentsFormSummaryUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_form_summary.rs` | 84 | `tests/test_cap_06_payments_form_summary.rs` |
| `Telegram/SourceFiles/payments/ui/payments_panel.cpp` | `CAP-06` | `PaymentsPanelUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_panel.rs` | 960 | `tests/test_cap_06_payments_panel.rs` |
| `Telegram/SourceFiles/payments/ui/payments_panel.h` | `CAP-06` | `PaymentsPanelUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_panel.rs` | 146 | `tests/test_cap_06_payments_panel.rs` |
| `Telegram/SourceFiles/payments/ui/payments_panel_data.h` | `CAP-06` | `PaymentsPanelDataUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_panel_data.rs` | 204 | `tests/test_cap_06_payments_panel_data.rs` |
| `Telegram/SourceFiles/payments/ui/payments_panel_delegate.h` | `CAP-06` | `PaymentsPanelDelegateUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_panel_delegate.rs` | 69 | `tests/test_cap_06_payments_panel_delegate.rs` |
| `Telegram/SourceFiles/payments/ui/payments_reaction_box.cpp` | `TCG-N14` | `PaymentsReactionBoxUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_reaction_box.rs` | 1030 | `tests/test_tcg_n14_payments_reaction_box.rs` |
| `Telegram/SourceFiles/payments/ui/payments_reaction_box.h` | `TCG-N14` | `PaymentsReactionBoxUi` | `crates/gpui_telegram_proto/src/payments/ui/payments_reaction_box.rs` | 120 | `tests/test_tcg_n14_payments_reaction_box.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/payments/payments_checkout_process.cpp`
- **Classes & Structs**: SessionProcesses, InfoField, CardField, Field
- **Core Methods & Signals**: CheckoutProcess::Start, CheckoutProcess::InvoicePaid, CheckoutProcess::ClearAll, Webview::InsideBlockingPopup, CheckoutProcess::close, Webview::RunWhenBlockingPopupFinished
- **Target**: `PaymentsCheckoutProcess` in `crates/gpui_telegram_proto/src/payments/payments_checkout_process.rs`

### `Telegram/SourceFiles/payments/payments_checkout_process.h`
- **Classes & Structs**: HistoryItem, PasscodeBox, CloudPasswordState, Session, GenericBox, Panel, InformationField, CardField, Form, FormUpdate, Error, InvoiceCredits
- **Core Methods & Signals**: ClearAll, RegisterPaymentStart, UnregisterPaymentStart, requestActivate, closeAndReactivate, close
- **Target**: `PaymentsCheckoutProcess` in `crates/gpui_telegram_proto/src/payments/payments_checkout_process.rs`

### `Telegram/SourceFiles/payments/payments_form.cpp`
- **Classes & Structs**: Flag, Size, Type
- **Core Methods & Signals**: views::all, views::transform, Stripe::CardBrandToString, SmartGlocal::Last4, Form::Form, Form::~Form
- **Target**: `PaymentsForm` in `crates/gpui_telegram_proto/src/payments/payments_form.rs`

### `Telegram/SourceFiles/payments/payments_form.h`
- **Classes & Structs**: Image, QJsonObject, CloudPasswordResult, APIClient, Session, PhotoMedia, Mode, FormDetails, ThumbnailLoadProcess, SavedCredentials, NewCredentials, StripePaymentMethod
- **Core Methods & Signals**: v::is_null, valid, Form, validateInformation, validateCard, setPaymentCredentials
- **Target**: `PaymentsForm` in `crates/gpui_telegram_proto/src/payments/payments_form.rs`

### `Telegram/SourceFiles/payments/payments_non_panel_process.cpp`
- **Classes & Structs**: CreditsFormDataPtr, CreditsReceiptPtr
- **Core Methods & Signals**: tr::lng_gift_sold_out_title, tr::lng_gift_sold_out_text, tr::lng_gift_sent_finished, Ui::ShowGiftErrorToast, Ui::SendStarsForm, Ui::SmallBalanceSourceFromForm
- **Target**: `PaymentsNonPanelProcess` in `crates/gpui_telegram_proto/src/payments/payments_non_panel_process.rs`

### `Telegram/SourceFiles/payments/payments_non_panel_process.h`
- **Classes & Structs**: HistoryItem, SessionShow, SessionController, CheckoutResult, CreditsFormData, CreditsReceiptData, NonPanelPaymentForm
- **Target**: `PaymentsNonPanelProcess` in `crates/gpui_telegram_proto/src/payments/payments_non_panel_process.rs`

### `Telegram/SourceFiles/payments/payments_reaction_process.cpp`
- **Classes & Structs**: State
- **Core Methods & Signals**: ReactionId::Paid, Settings::MaybeRequestBalanceIncrease, Ui::CreditsEmojiSmall, Lang::FormatCountDecimal, tr::lng_paid_react_send, tr::lng_paid_react_anonymous
- **Target**: `PaymentsReactionProcess` in `crates/gpui_telegram_proto/src/payments/payments_reaction_process.rs`

### `Telegram/SourceFiles/payments/payments_reaction_process.h`
- **Classes & Structs**: HistoryReactionSource, HistoryItem, GroupCall, Element, SessionShow, SessionController
- **Core Methods & Signals**: ShowPaidReactionDetails
- **Target**: `PaymentsReactionProcess` in `crates/gpui_telegram_proto/src/payments/payments_reaction_process.rs`

### `Telegram/SourceFiles/payments/smartglocal/smartglocal_api_client.h`
- **Classes & Structs**: PaymentConfiguration, APIClient
- **Core Methods & Signals**: APIClient, createTokenWithCard, createTokenWithData, destroyReplyDelayed
- **Target**: `SmartglocalApiClientSmartglocal` in `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_api_client.rs`

### `Telegram/SourceFiles/payments/smartglocal/smartglocal_callbacks.h`
- **Classes & Structs**: Token, Error, TokenCompletionCallback
- **Target**: `SmartglocalCallbacksSmartglocal` in `crates/gpui_telegram_proto/src/payments/smartglocal/smartglocal_callbacks.rs`

