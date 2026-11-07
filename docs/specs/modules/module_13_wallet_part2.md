# Module 13: TON Wallet & Gram Engine Part 2 Specification

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 13 (`wallet`) Part 2 (Slice 11)**, encompassing exactly 31 upstream Telegram Desktop source, test, and fixture files (mapped to 21 ledger targets across `crates/gpui_telegram_agent/src/wallet/` and component targets in `crates/gpui_telegram_client/src/wallet/`).

The subsystem establishes the production TON blockchain client engine, Bag of Cells (BOC) binary serialization/deserialization, cryptographic address encoding and validation, rate calculations, real-time transaction streaming, and GPUI wallet amount/address interaction fields:
- **TON BOC Serialization & Cell Topology**: Binary cell deserialization (`ParseBoc`, `ParseCells`), bitstream reader (`BitReader`), Castagnoli polynomial CRC32-C checksum calculation (`0x82F63B78U`), request signature disarming (`BreakRotationSignature`), UTF-8 zero-opcode comment extraction (`TextCommentFromBoc`), and NFT transfer new owner resolution (`NftTransferNewOwner`).
- **Gram API Client & Transaction Emulation**: Masterchain state querying (`/api/v2/getAddressInformation`), non-negative balance parsing (`ParseNonNegative`), fee and execution simulation (`/api/emulate/v1/emulateTrace`, `PhaseSucceeded` for compute & action phases), and error decoding (`ParseApiError`).
- **NFT Collectibles & Fragment Resolution**: Telegram Number & Username Fragment collections identification (`kFragmentCollections`), 64-hex raw address canonicalization, and paginated NFT retrieval (`ParseNftItems`).
- **Live Stream Architecture & Currency Conversion**: WSS/HTTPS endpoint validation (`ParseStreamEndpoint`), real-time account state & transaction event dispatching (`ParseStreamEvent`), and multi-currency fiat exchange valuation (`ComputePerGram`).
- **TON Address & Amount Input Components**: Raw/user-friendly address formatting (bounceable/testnet flags, `kAddressGroup = 4` grouping), transfer URI parsing, and GPUI input field easing animation (`kEditDuration = 180ms`, `kSwitchDuration = 450ms`).

---

## 100% Upstream Source Reading Manifest (31 / 31 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-missing-control.json` | L1-L1 | `FragmentMissingControl` | Schema test fixture for missing control fields | `crates/gpui_telegram_agent/assets/fixtures/fragment-missing-control.json` | `FragmentMissingControlTestsFixturesConfig` |
| `Telegram/SourceFiles/gram/tests/fixtures/manifest.json` | L1-L300 | `ManifestFixtures` | JSON fixture list and expected test vector hashes | `crates/gpui_telegram_agent/assets/fixtures/manifest.json` | `ManifestTestsFixturesConfig` |
| `Telegram/SourceFiles/gram/api/gram_api_account.cpp` | L1-L67 | `AddressInformationRequest`, `ParseAddressFunds`, `ParseNonNegative` | Endpoint: `/api/v2/getAddressInformation`, Max exact double: `1 << 53` | `crates/gpui_telegram_agent/src/wallet/gram_api_account.rs` | `GramApiAccountApi` |
| `Telegram/SourceFiles/gram/api/gram_api_account.h` | L1-L30 | `AddressFunds`, `balanceNano`, `neverUsed` | `balanceNano: int64`, `neverUsed: bool (lt == 0)` | `crates/gpui_telegram_agent/src/wallet/gram_api_account.rs` | `GramApiAccountApi` |
| `Telegram/SourceFiles/gram/api/gram_api_emulate.cpp` | L1-L118 | `EmulateTraceRequest`, `ParseEmulatedTrace`, `PhaseSucceeded` | Endpoint: `/api/emulate/v1/emulateTrace`, `exit_code in {0,1}`, `result_code == 0` | `crates/gpui_telegram_agent/src/wallet/gram_api_emulate.rs` | `GramApiEmulateApi` |
| `Telegram/SourceFiles/gram/api/gram_api_emulate.h` | L1-L41 | `EmulatedTrace`, `feeNano`, `account` | Fee model, root transaction trace contract | `crates/gpui_telegram_agent/src/wallet/gram_api_emulate.rs` | `GramApiEmulateApi` |
| `Telegram/SourceFiles/gram/api/gram_api_nft.cpp` | L1-L275 | `FragmentCollection`, `CanonicalRawAddress`, `ParseNftItems`, `ClassifyNftKind` | `kRawHashChars = 64`, Fragment number & username collections | `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs` | `GramApiNftApi` |
| `Telegram/SourceFiles/gram/api/gram_api_nft.h` | L1-L72 | `NftKind`, `NftWebDocument`, `NftItem`, `NftPage`, `NftItemByAddressRequest` | Kinds: `Generic`, `TelegramGift`, `TelegramNumber`, `TelegramUsername` | `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs` | `GramApiNftApi` |
| `Telegram/SourceFiles/gram/api/gram_api_rates.cpp` | L1-L60 | `ComputePerGram`, `MakeCurrencyRates`, `LookupRate`, `Usable` | Rate formula: `result = *value * usdPerGram`, `isfinite(v) && v > 0` | `crates/gpui_telegram_agent/src/wallet/gram_api_rates.rs` | `GramApiRatesApi` |
| `Telegram/SourceFiles/gram/api/gram_api_rates.h` | L1-L37 | `CurrencyRates`, `CurrencyRateEntry` | Flat map lookup, uppercase normalized currency codes | `crates/gpui_telegram_agent/src/wallet/gram_api_rates.rs` | `GramApiRatesApi` |
| `Telegram/SourceFiles/gram/api/gram_api_request.cpp` | L1-L38 | `ParseApiError`, `PercentEncoded` | Percent-encoding via URL encoding, error string extraction | `crates/gpui_telegram_agent/src/wallet/gram_api_request.rs` | `GramApiRequestApi` |
| `Telegram/SourceFiles/gram/api/gram_api_request.h` | L1-L40 | `HttpRequest`, `ApiError`, `PercentEncoded` | Generic HTTP request struct (`post`, `endpoint`, `query`, `payload`) | `crates/gpui_telegram_agent/src/wallet/gram_api_request.rs` | `GramApiRequestApi` |
| `Telegram/SourceFiles/gram/api/gram_api_stream.cpp` | L1-L163 | `ParseStreamEndpoint`, `StreamEndpointLabel`, `ParseStreamEvent`, `ReadAccountStateEvent` | `kSecurePort = 443`, `kMaxPort = 65535`, deduplicated account lists | `crates/gpui_telegram_agent/src/wallet/gram_api_stream.rs` | `GramApiStreamApi` |
| `Telegram/SourceFiles/gram/api/gram_api_stream.h` | L1-L50 | `StreamEndpoint`, `StreamEventKind`, `StreamEvent`, `StreamSubscribeMessage`, `StreamPingMessage` | Events: `AccountState`, `Transactions`, `TraceInvalidated` | `crates/gpui_telegram_agent/src/wallet/gram_api_stream.rs` | `GramApiStreamApi` |
| `Telegram/SourceFiles/gram/tests/gram_api_tests.cpp` | L1-L92 | `ApiChecks`, `ReadFixture`, `CheckRequest` | Test vectors for API errors, endpoints, and status codes | `crates/gpui_telegram_agent/src/wallet/gram_api_tests.rs` | `GramApiTestsTests` |
| `Telegram/SourceFiles/gram/gram_boc.cpp` | L1-L402 | `ParseBoc`, `ParseCells`, `Crc32c`, `BreakRotationSignature`, `TextCommentFromBoc`, `NftTransferNewOwner` | `kBocMagic = 0xB5EE9C72`, CRC32-C `0x82F63B78`, `kNftTransferOpcode = 0x5FCC3D14` | `crates/gpui_telegram_agent/src/wallet/gram_boc.rs` | `GramBoc` |
| `Telegram/SourceFiles/gram/gram_boc.h` | L1-L36 | `BreakRotationSignature`, `TextCommentFromBoc`, `NftTransferNewOwner` | Rotation disarm contract, opcode 0 text comments | `crates/gpui_telegram_agent/src/wallet/gram_boc.rs` | `GramBoc` |
| `Telegram/SourceFiles/gram/tests/gram_boc_tests.cpp` | L1-L519 | `BocChecks`, `StrictlyValid`, `CellBits` | BOC test suites: empty cells, deep DAG, invalid CRC, cyclic refs | `crates/gpui_telegram_agent/src/wallet/gram_boc_tests.rs` | `GramBocTestsTests` |
| `Telegram/SourceFiles/gram/tests/gram_emulate_tests.cpp` | L1-L348 | `EmulateChecks`, `EmulateTraceRequest` | Test vectors for trace parsing, fee verification, phase exit codes | `crates/gpui_telegram_agent/src/wallet/gram_emulate_tests.rs` | `GramEmulateTestsTests` |
| `Telegram/SourceFiles/gram/tests/gram_nft_tests.cpp` | L1-L761 | `NftChecks`, `NftPage`, `ClassifyNftKind` | Test vectors for fragment numbers, gifts, user domains | `crates/gpui_telegram_agent/src/wallet/gram_nft_tests.rs` | `GramNftTestsTests` |
| `Telegram/SourceFiles/gram/gram_pch.h` | L1-L21 | Precompiled headers: `base/bytes.h`, `rpl/rpl.h`, `flat_map.h` | PCH header collection for Gram module | `crates/gpui_telegram_agent/src/wallet/gram_pch.rs` | `GramPch` |
| `Telegram/SourceFiles/gram/tests/gram_rates_tests.cpp` | L1-L264 | `RatesChecks`, `CurrencyRates` | Currency parsing test fixtures, precision check, roundtrip | `crates/gpui_telegram_agent/src/wallet/gram_rates_tests.rs` | `GramRatesTestsTests` |
| `Telegram/SourceFiles/gram/tests/gram_stream_tests.cpp` | L1-L517 | `StreamChecks`, `StreamSubscribeMessage` | Ping frame format, subscription payload, port edge cases | `crates/gpui_telegram_agent/src/wallet/gram_stream_tests.rs` | `GramStreamTestsTests` |
| `Telegram/SourceFiles/gram/tests/gram_tests.h` | L1-L68 | `Check`, `CheckRequest`, `ReadFixture` | Runner harness, fixture path lookup, check runner contract | `crates/gpui_telegram_agent/src/wallet/gram_tests.rs` | `GramTestsTests` |
| `Telegram/SourceFiles/gram/tests/gram_tests_main.cpp` | L1-L54 | `AllChecks`, `main` | Runner execution loop, exit code assertion on check failure | `crates/gpui_telegram_agent/src/wallet/gram_tests_main.rs` | `GramTestsMainTests` |
| `Telegram/SourceFiles/wallet/wallet_address.cpp` | L1-L113 | `ParseAddress`, `CanonicalAddress`, `FormatFriendly`, `TransferLinkExpired`, `ParseTransferLink` | Friendly conversion, user-friendly flags, expiry timestamp check | `crates/gpui_telegram_agent/src/wallet/wallet_address.rs` | `WalletAddress` |
| `Telegram/SourceFiles/wallet/wallet_address.h` | L1-L44 | `ParsedAddress`, `TransferLink`, `ParseAddress`, `ParseTransferLink` | Address model: `raw`, `friendly`, `bounceable`, `testnet` | `crates/gpui_telegram_agent/src/wallet/wallet_address.rs` | `WalletAddress` |
| `Telegram/SourceFiles/wallet/wallet_address_label.cpp` | L1-L127 | `MakeAddressLabel`, `DetailsAddressLines`, `AddressLabel` | `kAddressGroup = 4`, wrapping width optimization, copy handler | `crates/gpui_telegram_agent/src/wallet/wallet_address_label.rs` | `WalletAddressLabel` |
| `Telegram/SourceFiles/wallet/wallet_address_label.h` | L1-L28 | `MakeAddressLabel`, `AddressLabel` | Widget factory for address label with copy interaction | `crates/gpui_telegram_agent/src/wallet/wallet_address_label.rs` | `WalletAddressLabel` |
| `Telegram/SourceFiles/wallet/wallet_amount_field.cpp` | L1-L2539 | `AddAmountField`, `TakeAmountDiamond`, `AmountFieldArgs`, `TonAmountInput` | `kEditDuration = 180ms`, `kSwitchDuration = 450ms`, OutCubic ease | `crates/gpui_telegram_agent/src/wallet/wallet_amount_field.rs` | `WalletAmountField` |
| `Telegram/SourceFiles/wallet/wallet_amount_field.h` | L1-L59 | `AmountLabel`, `AmountFieldArgs`, `AmountDiamond`, `AddAmountField` | Prefix, decimal, suffix, unit decomposition, swap callback | `crates/gpui_telegram_agent/src/wallet/wallet_amount_field.rs` | `WalletAmountField` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: TON Bag of Cells (BOC) Serialization & CRC32-C Validation
1. **Bag of Cells Binary Structure (`gram_boc.{h,cpp}`)**:
   - Magic Header: 4 bytes `0xB5EE9C72` (`kBocMagic`).
   - Flags Byte: `[has_idx (1 bit) | has_crc (1 bit) | has_cache_bits (1 bit) | flags (2 bits) | ref_size (3 bits)]`.
   - Offset size byte: 1..8 bytes describing cell data offset field sizes.
   - Cells count, root cells count (must be `1`), absent cells count (must be `0`), total cells size.
   - Root cell index (typically `0`).
   - Cell descriptor: 2 bytes per cell:
     - Byte 1: `d1 = refs_count (0..4) | (is_exotic ? 8 : 0)`.
     - Byte 2: `d2 = (data_byte_length * 2) | (has_completion_tag ? 1 : 0)`.
   - Data bits calculation: if `whole_bytes`, `data_len * 8`; otherwise, completion bit (`1` followed by trailing zeros) is stripped via `8 * data_len - 1 - count_trailing_zeros(last_byte)`.

2. **Castagnoli CRC32-C Checksum (`0x82F63B78U`)**:
   - Upstream formula in `Crc32c(data, size)`:
     ```cpp
     uint32 result = 0xFFFFFFFFU;
     for (int i = 0; i < size; ++i) {
         result ^= uint8(data[i]);
         for (int bit = 0; bit < 8; ++bit) {
             result = (result & 1) ? ((result >> 1) ^ 0x82F63B78U) : (result >> 1);
         }
     }
     return ~result;
     ```
   - Checked in `StrictlyValid`: verifies CRC in little-endian order at `data[len - 4 .. len]`.

3. **Key Rotation Disarming (`BreakRotationSignature`)**:
   - In TON, external messages for Wallet v4r2 or rev00 rotation requests carry a 64-byte Ed25519 signature followed by opcodes `0xFBBA99C8` (`kExternalOpcode`) or `0xFBBA99C7` (`kInternalOpcode`).
   - For fee estimation, Telegram replaces the 64-byte signature with cryptographically random bytes (`base::RandomFill`), recomputes CRC32-C, and emits the modified BOC. This allows the emulator to calculate exact execution fees without risking accidental broadcasting of a valid key-rotation message.

4. **Zero-Opcode Comment Extraction (`TextCommentFromBoc`)**:
   - Validates linear DAG with max 64 cells (`kMaxCells = 64`).
   - Root cell must start with 32-bit opcode `0x00000000`.
   - Concatenates data bytes across reference chains and verifies valid UTF-8 string encoding.

5. **NFT Transfer Target Resolution (`NftTransferNewOwner`)**:
   - Opcode `0x5FCC3D14` (`kNftTransferOpcode`).
   - Followed by 64-bit query ID, 2-bit standard address tag (`0b10`), 1-bit anycast (`0`), 8-bit workchain signed int (`-1` or `0`), and 256-bit account ID (32 bytes). Total prefix length: 363 bits (`kNftTransferOwnerBits`). Emits canonical raw string `<workchain>:<hex64>`.

---

### Cluster 2: Gram API Client & Transaction Emulation
1. **Account Funds Parsing (`gram_api_account.{h,cpp}`)**:
   - Queries endpoint `/api/v2/getAddressInformation?address=<encoded>`.
   - `ParseNonNegative`: parses string decimal or float with ceiling/floor check up to `2^53` (`9,007,199,254,740,992`) to prevent precision truncation.
   - Extracts `balance` in nanoTON (`1 TON = 1,000,000,000 nanoTON`).
   - `neverUsed`: marked `true` if logical time `last_transaction_id.lt == 0`.

2. **Transaction Emulation Pipeline (`gram_api_emulate.{h,cpp}`)**:
   - Endpoint: `/api/emulate/v1/emulateTrace`.
   - Payload payload flags: `ignore_chksig = true`, `with_actions = true`, `include_code_data = false`.
   - Multi-phase result validation:
     - `aborted`: must be `false`.
     - `compute_ph`: requires `success == true` and `exit_code in {0, 1}` (exit code 1 represents normal alternative termination).
     - `action`: requires `success == true` and `result_code == 0`.
     - Fee extraction: `total_fees` string converted to positive `i64` nanoTON.

3. **Exchange Rates & Currency Conversion (`gram_api_rates.{h,cpp}`)**:
   - Maps standard currency ticker codes (USD, EUR, RUB, GBP, CNY, etc.) to exchange coefficients.
   - Normalized key lookup: `code.toUpper()`.
   - Strict sanity validation `Usable(v)`: requires `isfinite(v) && v > 0.0`.
   - Converts base USD valuation via `usdPerGram * rate`.

### Cluster 3: Collectibles, Streaming, & Test Harness
1. **NFT Items & Fragment Collections (`gram_api_nft.{h,cpp}`)**:
   - Classifies TON NFTs into:
     - `TelegramNumber`: matching Fragment collection `0:0E41DC1DC3C9067ED24248580E12B3359818D83DEE0304FABCF80845EAFAFDB2` (segment `number`).
     - `TelegramUsername`: matching Fragment collection `0:80D78A35F955A14B679FAA887FF4CD5BFC0F43B4A4EEA2A7E6927F3701B273C2` (segment `username`).
     - `TelegramGift`: gifts identified by metadata URL schemas.
     - `Generic`: all other NFT collections.
   - Enforces 64-hex lowercase address format (`kRawHashChars = 64`).

2. **WebSocket & Streaming Events (`gram_api_stream.{h,cpp}`)**:
   - `ParseStreamEndpoint`: parses `wss://` or `https://` URLs, strict port validation (`1..65535`, default `443`), rejecting credentials in URL.
   - Dispatches 3 core real-time events:
     - `account_state_change`: balance updates and account state shifts.
     - `transactions`: incoming or outgoing blockchain transaction lists.
     - `trace_invalidated`: invalidates cached speculative traces when a confirmed transaction arrives.
   - Ping message: `{"type": "ping", "id": <id>}`. Subscription message: `{"type": "subscribe", "account": "<addr>", "id": <id>}`.

3. **Deterministic Test Harness & Fixtures (`gram/tests/*`)**:
   - `manifest.json`: SHA-256 digests and lengths for 45+ test fixtures.
   - `gram_api_tests.cpp`: error response fixtures (`api-02-traces.json`, `api-04-emulateTrace.json`, `api-05-badAddress.json`, `api-06-unknownEndpoint.json`, etc.).
   - `gram_boc_tests.cpp`: test vectors for single cell, multi-cell DAG, exotic cells, CRC verification, and bitstream edges.
   - `gram_stream_tests.cpp` & `gram_rates_tests.cpp`: verification of ping-pong intervals and float multiplication accuracy.

---

### Cluster 4: Address Formatting, Amount Field & Client Components
1. **TON Address Model (`wallet_address.{h,cpp}`)**:
   - Raw format: `<workchain>:<32-byte-hex-in-lowercase>` (workchain typically `-1` for masterchain, `0` for basechain).
   - User-Friendly Base64url format:
     - Byte 0: Flags `[testnet (1 bit) | bounceable (1 bit) | type (6 bits)]`. Default bounceable = `0x11` (`EQ...`), non-bounceable = `0x51` (`UQ...`), testnet bounceable = `0x91` (`kQ...`), testnet non-bounceable = `0xD1` (`0Q...`).
     - Byte 1: Signed 8-bit workchain ID (`0x00` for basechain, `0xFF` for masterchain).
     - Bytes 2..33: 32-byte account address hash.
     - Bytes 34..35: CRC16-XMODEM checksum of bytes 0..33.
   - `TransferLink`: parses `ton://transfer/<address>?amount=<nano>&text=<comment>&exp=<timestamp>`.

2. **Visual Address Formatter (`wallet_address_label.{h,cpp}`)**:
   - Splits 48-character user-friendly base64url addresses into groups of 4 (`kAddressGroup = 4`).
   - Line-wrapping algorithm dynamically balances group distribution based on label width to avoid ragged edges.

3. **GPUI TON Amount Input & Diamond Animation (`wallet_amount_field.{h,cpp}`)**:
   - Manages decimal string entry, currency prefix/suffix, and nanoTON conversion (`value * 10^9`).
   - Animation timings: `kEditDuration = 180ms`, `kSwitchFirstDuration = 300ms`, `kSwitchDuration = 450ms`, `kSwitchFadeDuration = 300ms`.
   - Diamond vector icon integration for animated TON crystal emblem.

4. **Client Core Architecture (`crates/gpui_telegram_client/src/wallet/`)**:
   - `core/`: Cryptographic primitives (`ton_crypto.rs`, `ton_bip39.rs`, `ton_mnemonic.rs`, `ton_hash.rs`, `ton_keypair.rs`), Cell engine (`ton_cell.rs`), Amount handling (`ton_amount.rs`), and Account identifier (`ton_account_id.rs`).
   - `components/`: GPUI widgets for address inputs, amount inputs, balance cards, mnemonic word chips, fee chips, segmented tabs, token selectors, transaction rows, and wallet app bars.
