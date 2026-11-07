# Subsystem Shard: `wallet`

**Total Source Files**: 195  
**Total Source Lines**: 78,692  
**Primary Feature IDs**: CAP-06, TCG-N15, TCG-N31, TCG-N35  
**JSON Shard Path**: `docs/specs/ledger/shards/wallet.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/gram/api/gram_api_account.cpp` | `CAP-06` | `GramApiAccountApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_account.rs` | 67 | `tests/test_cap_06_gram_api_account.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_account.h` | `CAP-06` | `GramApiAccountApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_account.rs` | 30 | `tests/test_cap_06_gram_api_account.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_emulate.cpp` | `CAP-06` | `GramApiEmulateApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_emulate.rs` | 118 | `tests/test_cap_06_gram_api_emulate.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_emulate.h` | `CAP-06` | `GramApiEmulateApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_emulate.rs` | 41 | `tests/test_cap_06_gram_api_emulate.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_nft.cpp` | `CAP-06` | `GramApiNftApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs` | 275 | `tests/test_cap_06_gram_api_nft.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_nft.h` | `CAP-06` | `GramApiNftApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs` | 72 | `tests/test_cap_06_gram_api_nft.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_rates.cpp` | `CAP-06` | `GramApiRatesApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_rates.rs` | 60 | `tests/test_cap_06_gram_api_rates.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_rates.h` | `CAP-06` | `GramApiRatesApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_rates.rs` | 37 | `tests/test_cap_06_gram_api_rates.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_request.cpp` | `CAP-06` | `GramApiRequestApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_request.rs` | 38 | `tests/test_cap_06_gram_api_request.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_request.h` | `CAP-06` | `GramApiRequestApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_request.rs` | 40 | `tests/test_cap_06_gram_api_request.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_stream.cpp` | `TCG-N52` | `GramApiStreamApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_stream.rs` | 163 | `tests/test_tcg_n52_gram_api_stream.rs` |
| `Telegram/SourceFiles/gram/api/gram_api_stream.h` | `TCG-N52` | `GramApiStreamApi` | `crates/gpui_telegram_agent/src/wallet/gram_api_stream.rs` | 50 | `tests/test_tcg_n52_gram_api_stream.rs` |
| `Telegram/SourceFiles/gram/gram_boc.cpp` | `CAP-06` | `GramBoc` | `crates/gpui_telegram_agent/src/wallet/gram_boc.rs` | 402 | `tests/test_cap_06_gram_boc.rs` |
| `Telegram/SourceFiles/gram/gram_boc.h` | `CAP-06` | `GramBoc` | `crates/gpui_telegram_agent/src/wallet/gram_boc.rs` | 36 | `tests/test_cap_06_gram_boc.rs` |
| `Telegram/SourceFiles/gram/gram_pch.h` | `CAP-06` | `GramPch` | `crates/gpui_telegram_agent/src/wallet/gram_pch.rs` | 21 | `tests/test_cap_06_gram_pch.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-02-traces.json` | `CAP-06` | `Api02TracesTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-02-traces_config.rs` | 1 | `tests/test_cap_06_api_02_traces.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-04-emulateTrace.json` | `CAP-06` | `Api04EmulatetraceTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-04-emulateTrace_config.rs` | 1 | `tests/test_cap_06_api_04_emulateTrace.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-05-badAddress.json` | `CAP-06` | `Api05BadaddressTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-05-badAddress_config.rs` | 1 | `tests/test_cap_06_api_05_badAddress.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-06-unknownEndpoint.json` | `CAP-06` | `Api06UnknownendpointTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-06-unknownEndpoint_config.rs` | 1 | `tests/test_cap_06_api_06_unknownEndpoint.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-07-badPostBody.json` | `CAP-06` | `Api07BadpostbodyTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-07-badPostBody_config.rs` | 1 | `tests/test_cap_06_api_07_badPostBody.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-09-tracesAlt.json` | `CAP-06` | `Api09TracesaltTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-09-tracesAlt_config.rs` | 1 | `tests/test_cap_06_api_09_tracesAlt.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-10-emulateV3.json` | `CAP-06` | `Api10Emulatev3TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-10-emulateV3_config.rs` | 1 | `tests/test_cap_06_api_10_emulateV3.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-13-emulateV3Get.json` | `CAP-06` | `Api13Emulatev3getTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-13-emulateV3Get_config.rs` | 1 | `tests/test_cap_06_api_13_emulateV3Get.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-14-msgHashRaw.json` | `CAP-06` | `Api14MsghashrawTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-14-msgHashRaw_config.rs` | 1 | `tests/test_cap_06_api_14_msgHashRaw.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-15-msgHashEncoded.json` | `CAP-06` | `Api15MsghashencodedTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-15-msgHashEncoded_config.rs` | 1 | `tests/test_cap_06_api_15_msgHashEncoded.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-camel.json` | `CAP-06` | `ApiDnsRecordsCamelTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-camel_config.rs` | 1 | `tests/test_cap_06_api_dns_records_camel.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-domain-pumpanddump.json` | `CAP-06` | `ApiDnsRecordsDomainPumpanddumpTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-domain-pumpanddump_config.rs` | 1 | `tests/test_cap_06_api_dns_records_domain_pumpanddump.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-domain-saint.json` | `CAP-06` | `ApiDnsRecordsDomainSaintTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-domain-saint_config.rs` | 1 | `tests/test_cap_06_api_dns_records_domain_saint.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-domain-tolya.json` | `CAP-06` | `ApiDnsRecordsDomainTolyaTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-domain-tolya_config.rs` | 1 | `tests/test_cap_06_api_dns_records_domain_tolya.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-wallet-owner1.json` | `CAP-06` | `ApiDnsRecordsWalletOwner1TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-wallet-owner1_config.rs` | 1 | `tests/test_cap_06_api_dns_records_wallet_owner1.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-dns-records-wallet-owner3.json` | `CAP-06` | `ApiDnsRecordsWalletOwner3TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-dns-records-wallet-owner3_config.rs` | 1 | `tests/test_cap_06_api_dns_records_wallet_owner3.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-jetton-masters-usdt.json` | `CAP-06` | `ApiJettonMastersUsdtTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-jetton-masters-usdt_config.rs` | 1 | `tests/test_cap_06_api_jetton_masters_usdt.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-jetton-wallets-owner1.json` | `CAP-06` | `ApiJettonWalletsOwner1TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-jetton-wallets-owner1_config.rs` | 1 | `tests/test_cap_06_api_jetton_wallets_owner1.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-masterchain-info.json` | `CAP-06` | `ApiMasterchainInfoTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-masterchain-info_config.rs` | 1 | `tests/test_cap_06_api_masterchain_info.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-metadata-address-gift.json` | `TCG-N15` | `ApiMetadataAddressGiftTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-metadata-address-gift_config.rs` | 1 | `tests/test_tcg_n15_api_metadata_address_gift.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-collections-gifts.json` | `TCG-N15` | `ApiNftCollectionsGiftsTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-collections-gifts_config.rs` | 1 | `tests/test_tcg_n15_api_nft_collections_gifts.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-address-gift.json` | `TCG-N15` | `ApiNftItemsAddressGiftTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-address-gift_config.rs` | 1 | `tests/test_tcg_n15_api_nft_items_address_gift.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-camel.json` | `CAP-06` | `ApiNftItemsCamelTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-camel_config.rs` | 1 | `tests/test_cap_06_api_nft_items_camel.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-collection-numbers.json` | `CAP-06` | `ApiNftItemsCollectionNumbersTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-collection-numbers_config.rs` | 1 | `tests/test_cap_06_api_nft_items_collection_numbers.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-collection-usernames.json` | `CAP-06` | `ApiNftItemsCollectionUsernamesTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-collection-usernames_config.rs` | 1 | `tests/test_cap_06_api_nft_items_collection_usernames.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-owner1-limit5.json` | `CAP-06` | `ApiNftItemsOwner1Limit5TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-owner1-limit5_config.rs` | 1 | `tests/test_cap_06_api_nft_items_owner1_limit5.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-owner1-metadata-flag.json` | `CAP-06` | `ApiNftItemsOwner1MetadataFlagTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-owner1-metadata-flag_config.rs` | 1 | `tests/test_cap_06_api_nft_items_owner1_metadata_flag.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-owner1-paged.json` | `CAP-06` | `ApiNftItemsOwner1PagedTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-owner1-paged_config.rs` | 1 | `tests/test_cap_06_api_nft_items_owner1_paged.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-owner1.json` | `CAP-06` | `ApiNftItemsOwner1TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-owner1_config.rs` | 1 | `tests/test_cap_06_api_nft_items_owner1.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-items-owner2.json` | `CAP-06` | `ApiNftItemsOwner2TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-items-owner2_config.rs` | 1 | `tests/test_cap_06_api_nft_items_owner2.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-camel.json` | `CAP-06` | `ApiNftTransfersCamelTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-camel_config.rs` | 1 | `tests/test_cap_06_api_nft_transfers_camel.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-item-gift.json` | `TCG-N15` | `ApiNftTransfersItemGiftTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-item-gift_config.rs` | 1 | `tests/test_tcg_n15_api_nft_transfers_item_gift.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-owner1-in.json` | `CAP-06` | `ApiNftTransfersOwner1InTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-owner1-in_config.rs` | 1 | `tests/test_cap_06_api_nft_transfers_owner1_in.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-owner1-paged.json` | `CAP-06` | `ApiNftTransfersOwner1PagedTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-owner1-paged_config.rs` | 1 | `tests/test_cap_06_api_nft_transfers_owner1_paged.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-owner1.json` | `CAP-06` | `ApiNftTransfersOwner1TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-owner1_config.rs` | 1 | `tests/test_cap_06_api_nft_transfers_owner1.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-nft-transfers-owner2.json` | `CAP-06` | `ApiNftTransfersOwner2TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-nft-transfers-owner2_config.rs` | 1 | `tests/test_cap_06_api_nft_transfers_owner2.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-traces-item-gift.json` | `TCG-N15` | `ApiTracesItemGiftTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-traces-item-gift_config.rs` | 1 | `tests/test_tcg_n15_api_traces_item_gift.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-traces-owner1-limit5.json` | `CAP-06` | `ApiTracesOwner1Limit5TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-traces-owner1-limit5_config.rs` | 1 | `tests/test_cap_06_api_traces_owner1_limit5.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-traces-owner2.json` | `CAP-06` | `ApiTracesOwner2TestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-traces-owner2_config.rs` | 1 | `tests/test_cap_06_api_traces_owner2.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-transactions-item-gift.json` | `TCG-N15` | `ApiTransactionsItemGiftTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-transactions-item-gift_config.rs` | 1 | `tests/test_tcg_n15_api_transactions_item_gift.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/api-transactions-latest.json` | `CAP-06` | `ApiTransactionsLatestTestsFixturesConfig` | `crates/gpui_telegram_agent/config/api-transactions-latest_config.rs` | 1 | `tests/test_cap_06_api_transactions_latest.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-collection-deskcalendar.json` | `CAP-06` | `FragmentCollectionDeskcalendarTestsFixturesConfig` | `crates/gpui_telegram_agent/config/fragment-collection-deskcalendar_config.rs` | 1 | `tests/test_cap_06_fragment_collection_deskcalendar.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-gift-deskcalendar-45754-image.webp` | `TCG-N15` | `FragmentGiftDeskcalendar45754ImageTestsFixturesWebpAsset` | `crates/gpui_telegram_agent/assets/fragment-gift-deskcalendar-45754-image_webp.rs` | 1948 | `tests/test_tcg_n15_fragment_gift_deskcalendar_45754_image.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-gift-deskcalendar-45754.json` | `TCG-N15` | `FragmentGiftDeskcalendar45754TestsFixturesConfig` | `crates/gpui_telegram_agent/config/fragment-gift-deskcalendar-45754_config.rs` | 1 | `tests/test_tcg_n15_fragment_gift_deskcalendar_45754.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-imgproxy-deskcalendar-small.bin` | `TCG-N35` | `FragmentImgproxyDeskcalendarSmallTestsFixturesBinAsset` | `crates/gpui_telegram_agent/assets/fragment-imgproxy-deskcalendar-small_bin.rs` | 522 | `tests/test_tcg_n35_fragment_imgproxy_deskcalendar_small.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/fragment-missing-control.json` | `CAP-06` | `FragmentMissingControlTestsFixturesConfig` | `crates/gpui_telegram_agent/config/fragment-missing-control_config.rs` | 1 | `tests/test_cap_06_fragment_missing_control.rs` |
| `Telegram/SourceFiles/gram/tests/fixtures/manifest.json` | `CAP-06` | `ManifestTestsFixturesConfig` | `crates/gpui_telegram_agent/config/manifest_config.rs` | 300 | `tests/test_cap_06_manifest.rs` |
| `Telegram/SourceFiles/gram/tests/gram_api_tests.cpp` | `CAP-06` | `GramApiTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_api_tests.rs` | 92 | `tests/test_cap_06_gram_api_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_boc_tests.cpp` | `CAP-06` | `GramBocTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_boc_tests.rs` | 519 | `tests/test_cap_06_gram_boc_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_emulate_tests.cpp` | `CAP-06` | `GramEmulateTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_emulate_tests.rs` | 348 | `tests/test_cap_06_gram_emulate_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_nft_tests.cpp` | `CAP-06` | `GramNftTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_nft_tests.rs` | 761 | `tests/test_cap_06_gram_nft_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_rates_tests.cpp` | `CAP-06` | `GramRatesTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_rates_tests.rs` | 264 | `tests/test_cap_06_gram_rates_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_stream_tests.cpp` | `TCG-N52` | `GramStreamTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_stream_tests.rs` | 517 | `tests/test_tcg_n52_gram_stream_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_tests.h` | `CAP-06` | `GramTestsTests` | `crates/gpui_telegram_agent/src/wallet/gram_tests.rs` | 68 | `tests/test_cap_06_gram_tests.rs` |
| `Telegram/SourceFiles/gram/tests/gram_tests_main.cpp` | `CAP-06` | `GramTestsMainTests` | `crates/gpui_telegram_agent/src/wallet/gram_tests_main.rs` | 54 | `tests/test_cap_06_gram_tests_main.rs` |
| `Telegram/SourceFiles/wallet/wallet.style` | `CAP-06` | `WalletStyleSheet` | `crates/gpui_telegram_desktop/src/theme/wallet/wallet_style.rs` | 851 | `tests/test_cap_06_wallet.rs` |
| `Telegram/SourceFiles/wallet/wallet_address.cpp` | `CAP-06` | `WalletAddress` | `crates/gpui_telegram_agent/src/wallet/wallet_address.rs` | 113 | `tests/test_cap_06_wallet_address.rs` |
| `Telegram/SourceFiles/wallet/wallet_address.h` | `CAP-06` | `WalletAddress` | `crates/gpui_telegram_agent/src/wallet/wallet_address.rs` | 44 | `tests/test_cap_06_wallet_address.rs` |
| `Telegram/SourceFiles/wallet/wallet_address_label.cpp` | `CAP-06` | `WalletAddressLabel` | `crates/gpui_telegram_agent/src/wallet/wallet_address_label.rs` | 127 | `tests/test_cap_06_wallet_address_label.rs` |
| `Telegram/SourceFiles/wallet/wallet_address_label.h` | `CAP-06` | `WalletAddressLabel` | `crates/gpui_telegram_agent/src/wallet/wallet_address_label.rs` | 28 | `tests/test_cap_06_wallet_address_label.rs` |
| `Telegram/SourceFiles/wallet/wallet_amount_field.cpp` | `CAP-06` | `WalletAmountField` | `crates/gpui_telegram_agent/src/wallet/wallet_amount_field.rs` | 2539 | `tests/test_cap_06_wallet_amount_field.rs` |
| `Telegram/SourceFiles/wallet/wallet_amount_field.h` | `CAP-06` | `WalletAmountField` | `crates/gpui_telegram_agent/src/wallet/wallet_amount_field.rs` | 59 | `tests/test_cap_06_wallet_amount_field.rs` |
| `Telegram/SourceFiles/wallet/wallet_amount_painter.cpp` | `CAP-06` | `WalletAmountPainter` | `crates/gpui_telegram_agent/src/wallet/wallet_amount_painter.rs` | 258 | `tests/test_cap_06_wallet_amount_painter.rs` |
| `Telegram/SourceFiles/wallet/wallet_amount_painter.h` | `CAP-06` | `WalletAmountPainter` | `crates/gpui_telegram_agent/src/wallet/wallet_amount_painter.rs` | 96 | `tests/test_cap_06_wallet_amount_painter.rs` |
| `Telegram/SourceFiles/wallet/wallet_api.cpp` | `CAP-06` | `WalletApi` | `crates/gpui_telegram_agent/src/wallet/wallet_api.rs` | 225 | `tests/test_cap_06_wallet_api.rs` |
| `Telegram/SourceFiles/wallet/wallet_api.h` | `CAP-06` | `WalletApi` | `crates/gpui_telegram_agent/src/wallet/wallet_api.rs` | 60 | `tests/test_cap_06_wallet_api.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_angle.cpp` | `CAP-06` | `WalletCardAngle` | `crates/gpui_telegram_agent/src/wallet/wallet_card_angle.rs` | 307 | `tests/test_cap_06_wallet_card_angle.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_angle.h` | `CAP-06` | `WalletCardAngle` | `crates/gpui_telegram_agent/src/wallet/wallet_card_angle.rs` | 73 | `tests/test_cap_06_wallet_card_angle.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_colors.cpp` | `CAP-06` | `WalletCardColors` | `crates/gpui_telegram_agent/src/wallet/wallet_card_colors.rs` | 283 | `tests/test_cap_06_wallet_card_colors.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_colors.h` | `CAP-06` | `WalletCardColors` | `crates/gpui_telegram_agent/src/wallet/wallet_card_colors.rs` | 81 | `tests/test_cap_06_wallet_card_colors.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_drag.cpp` | `CAP-06` | `WalletCardDrag` | `crates/gpui_telegram_agent/src/wallet/wallet_card_drag.rs` | 502 | `tests/test_cap_06_wallet_card_drag.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_drag.h` | `CAP-06` | `WalletCardDrag` | `crates/gpui_telegram_agent/src/wallet/wallet_card_drag.rs` | 118 | `tests/test_cap_06_wallet_card_drag.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_editor.cpp` | `CAP-06` | `WalletCardEditor` | `crates/gpui_telegram_agent/src/wallet/wallet_card_editor.rs` | 1097 | `tests/test_cap_06_wallet_card_editor.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_editor.h` | `CAP-06` | `WalletCardEditor` | `crates/gpui_telegram_agent/src/wallet/wallet_card_editor.rs` | 134 | `tests/test_cap_06_wallet_card_editor.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_flight.cpp` | `CAP-06` | `WalletCardFlight` | `crates/gpui_telegram_agent/src/wallet/wallet_card_flight.rs` | 298 | `tests/test_cap_06_wallet_card_flight.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_flight.h` | `CAP-06` | `WalletCardFlight` | `crates/gpui_telegram_agent/src/wallet/wallet_card_flight.rs` | 71 | `tests/test_cap_06_wallet_card_flight.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_float.cpp` | `CAP-06` | `WalletCardFloat` | `crates/gpui_telegram_agent/src/wallet/wallet_card_float.rs` | 365 | `tests/test_cap_06_wallet_card_float.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_float.h` | `CAP-06` | `WalletCardFloat` | `crates/gpui_telegram_agent/src/wallet/wallet_card_float.rs` | 78 | `tests/test_cap_06_wallet_card_float.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_foreground.cpp` | `CAP-06` | `WalletCardForeground` | `crates/gpui_telegram_agent/src/wallet/wallet_card_foreground.rs` | 230 | `tests/test_cap_06_wallet_card_foreground.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_foreground.h` | `CAP-06` | `WalletCardForeground` | `crates/gpui_telegram_agent/src/wallet/wallet_card_foreground.rs` | 60 | `tests/test_cap_06_wallet_card_foreground.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_gradient.cpp` | `CAP-06` | `WalletCardGradient` | `crates/gpui_telegram_agent/src/wallet/wallet_card_gradient.rs` | 301 | `tests/test_cap_06_wallet_card_gradient.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_gradient.h` | `CAP-06` | `WalletCardGradient` | `crates/gpui_telegram_agent/src/wallet/wallet_card_gradient.rs` | 43 | `tests/test_cap_06_wallet_card_gradient.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_look.cpp` | `CAP-06` | `WalletCardLook` | `crates/gpui_telegram_agent/src/wallet/wallet_card_look.rs` | 229 | `tests/test_cap_06_wallet_card_look.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_look.h` | `CAP-06` | `WalletCardLook` | `crates/gpui_telegram_agent/src/wallet/wallet_card_look.rs` | 74 | `tests/test_cap_06_wallet_card_look.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_space.cpp` | `CAP-06` | `WalletCardSpace` | `crates/gpui_telegram_agent/src/wallet/wallet_card_space.rs` | 1202 | `tests/test_cap_06_wallet_card_space.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_space.h` | `CAP-06` | `WalletCardSpace` | `crates/gpui_telegram_agent/src/wallet/wallet_card_space.rs` | 85 | `tests/test_cap_06_wallet_card_space.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_stickers.cpp` | `TCG-N15` | `WalletCardStickers` | `crates/gpui_telegram_agent/src/wallet/wallet_card_stickers.rs` | 1450 | `tests/test_tcg_n15_wallet_card_stickers.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_stickers.h` | `TCG-N15` | `WalletCardStickers` | `crates/gpui_telegram_agent/src/wallet/wallet_card_stickers.rs` | 179 | `tests/test_tcg_n15_wallet_card_stickers.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_tint.cpp` | `CAP-06` | `WalletCardTint` | `crates/gpui_telegram_agent/src/wallet/wallet_card_tint.rs` | 333 | `tests/test_cap_06_wallet_card_tint.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_tint.h` | `CAP-06` | `WalletCardTint` | `crates/gpui_telegram_agent/src/wallet/wallet_card_tint.rs` | 57 | `tests/test_cap_06_wallet_card_tint.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_transition.cpp` | `CAP-06` | `WalletCardTransition` | `crates/gpui_telegram_agent/src/wallet/wallet_card_transition.rs` | 531 | `tests/test_cap_06_wallet_card_transition.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_transition.h` | `CAP-06` | `WalletCardTransition` | `crates/gpui_telegram_agent/src/wallet/wallet_card_transition.rs` | 123 | `tests/test_cap_06_wallet_card_transition.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_view.cpp` | `CAP-06` | `WalletCardView` | `crates/gpui_telegram_agent/src/wallet/wallet_card_view.rs` | 1849 | `tests/test_cap_06_wallet_card_view.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_view.h` | `CAP-06` | `WalletCardView` | `crates/gpui_telegram_agent/src/wallet/wallet_card_view.rs` | 27 | `tests/test_cap_06_wallet_card_view.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_view_motion.cpp` | `CAP-06` | `WalletCardViewMotion` | `crates/gpui_telegram_agent/src/wallet/wallet_card_view_motion.rs` | 530 | `tests/test_cap_06_wallet_card_view_motion.rs` |
| `Telegram/SourceFiles/wallet/wallet_card_view_motion.h` | `CAP-06` | `WalletCardViewMotion` | `crates/gpui_telegram_agent/src/wallet/wallet_card_view_motion.rs` | 101 | `tests/test_cap_06_wallet_card_view_motion.rs` |
| `Telegram/SourceFiles/wallet/wallet_chat_show.cpp` | `CAP-06` | `WalletChatShow` | `crates/gpui_telegram_agent/src/wallet/wallet_chat_show.rs` | 142 | `tests/test_cap_06_wallet_chat_show.rs` |
| `Telegram/SourceFiles/wallet/wallet_chat_show.h` | `CAP-06` | `WalletChatShow` | `crates/gpui_telegram_agent/src/wallet/wallet_chat_show.rs` | 27 | `tests/test_cap_06_wallet_chat_show.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectible_media.cpp` | `CAP-06` | `WalletCollectibleMedia` | `crates/gpui_telegram_agent/src/wallet/wallet_collectible_media.rs` | 668 | `tests/test_cap_06_wallet_collectible_media.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectible_media.h` | `CAP-06` | `WalletCollectibleMedia` | `crates/gpui_telegram_agent/src/wallet/wallet_collectible_media.rs` | 148 | `tests/test_cap_06_wallet_collectible_media.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectible_record.cpp` | `CAP-06` | `WalletCollectibleRecord` | `crates/gpui_telegram_agent/src/wallet/wallet_collectible_record.rs` | 48 | `tests/test_cap_06_wallet_collectible_record.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectible_record.h` | `CAP-06` | `WalletCollectibleRecord` | `crates/gpui_telegram_agent/src/wallet/wallet_collectible_record.rs` | 41 | `tests/test_cap_06_wallet_collectible_record.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles.cpp` | `CAP-06` | `WalletCollectibles` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles.rs` | 2320 | `tests/test_cap_06_wallet_collectibles.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles.h` | `CAP-06` | `WalletCollectibles` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles.rs` | 41 | `tests/test_cap_06_wallet_collectibles.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles_hint.cpp` | `CAP-06` | `WalletCollectiblesHint` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles_hint.rs` | 105 | `tests/test_cap_06_wallet_collectibles_hint.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles_hint.h` | `CAP-06` | `WalletCollectiblesHint` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles_hint.rs` | 50 | `tests/test_cap_06_wallet_collectibles_hint.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles_viewport.cpp` | `CAP-06` | `WalletCollectiblesViewport` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles_viewport.rs` | 69 | `tests/test_cap_06_wallet_collectibles_viewport.rs` |
| `Telegram/SourceFiles/wallet/wallet_collectibles_viewport.h` | `CAP-06` | `WalletCollectiblesViewport` | `crates/gpui_telegram_agent/src/wallet/wallet_collectibles_viewport.rs` | 45 | `tests/test_cap_06_wallet_collectibles_viewport.rs` |
| `Telegram/SourceFiles/wallet/wallet_comment.cpp` | `CAP-06` | `WalletComment` | `crates/gpui_telegram_agent/src/wallet/wallet_comment.rs` | 422 | `tests/test_cap_06_wallet_comment.rs` |
| `Telegram/SourceFiles/wallet/wallet_comment.h` | `CAP-06` | `WalletComment` | `crates/gpui_telegram_agent/src/wallet/wallet_comment.rs` | 76 | `tests/test_cap_06_wallet_comment.rs` |
| `Telegram/SourceFiles/wallet/wallet_content.cpp` | `CAP-06` | `WalletContent` | `crates/gpui_telegram_agent/src/wallet/wallet_content.rs` | 16865 | `tests/test_cap_06_wallet_content.rs` |
| `Telegram/SourceFiles/wallet/wallet_content.h` | `CAP-06` | `WalletContent` | `crates/gpui_telegram_agent/src/wallet/wallet_content.rs` | 238 | `tests/test_cap_06_wallet_content.rs` |
| `Telegram/SourceFiles/wallet/wallet_custody.cpp` | `CAP-06` | `WalletCustody` | `crates/gpui_telegram_agent/src/wallet/wallet_custody.rs` | 242 | `tests/test_cap_06_wallet_custody.rs` |
| `Telegram/SourceFiles/wallet/wallet_custody.h` | `CAP-06` | `WalletCustody` | `crates/gpui_telegram_agent/src/wallet/wallet_custody.rs` | 98 | `tests/test_cap_06_wallet_custody.rs` |
| `Telegram/SourceFiles/wallet/wallet_diamond_flight.cpp` | `CAP-06` | `WalletDiamondFlight` | `crates/gpui_telegram_agent/src/wallet/wallet_diamond_flight.rs` | 191 | `tests/test_cap_06_wallet_diamond_flight.rs` |
| `Telegram/SourceFiles/wallet/wallet_diamond_flight.h` | `CAP-06` | `WalletDiamondFlight` | `crates/gpui_telegram_agent/src/wallet/wallet_diamond_flight.rs` | 69 | `tests/test_cap_06_wallet_diamond_flight.rs` |
| `Telegram/SourceFiles/wallet/wallet_engine.cpp` | `CAP-06` | `WalletEngine` | `crates/gpui_telegram_agent/src/wallet/wallet_engine.rs` | 1450 | `tests/test_cap_06_wallet_engine.rs` |
| `Telegram/SourceFiles/wallet/wallet_engine.h` | `CAP-06` | `WalletEngine` | `crates/gpui_telegram_agent/src/wallet/wallet_engine.rs` | 440 | `tests/test_cap_06_wallet_engine.rs` |
| `Telegram/SourceFiles/wallet/wallet_fiat.cpp` | `CAP-06` | `WalletFiat` | `crates/gpui_telegram_agent/src/wallet/wallet_fiat.rs` | 230 | `tests/test_cap_06_wallet_fiat.rs` |
| `Telegram/SourceFiles/wallet/wallet_fiat.h` | `CAP-06` | `WalletFiat` | `crates/gpui_telegram_agent/src/wallet/wallet_fiat.rs` | 67 | `tests/test_cap_06_wallet_fiat.rs` |
| `Telegram/SourceFiles/wallet/wallet_gift_move_presentation.cpp` | `TCG-N15` | `WalletGiftMovePresentation` | `crates/gpui_telegram_agent/src/wallet/wallet_gift_move_presentation.rs` | 576 | `tests/test_tcg_n15_wallet_gift_move_presentation.rs` |
| `Telegram/SourceFiles/wallet/wallet_gift_move_presentation.h` | `TCG-N15` | `WalletGiftMovePresentation` | `crates/gpui_telegram_agent/src/wallet/wallet_gift_move_presentation.rs` | 78 | `tests/test_tcg_n15_wallet_gift_move_presentation.rs` |
| `Telegram/SourceFiles/wallet/wallet_key_protection.cpp` | `CAP-06` | `WalletKeyProtection` | `crates/gpui_telegram_agent/src/wallet/wallet_key_protection.rs` | 1611 | `tests/test_cap_06_wallet_key_protection.rs` |
| `Telegram/SourceFiles/wallet/wallet_key_protection.h` | `CAP-06` | `WalletKeyProtection` | `crates/gpui_telegram_agent/src/wallet/wallet_key_protection.rs` | 220 | `tests/test_cap_06_wallet_key_protection.rs` |
| `Telegram/SourceFiles/wallet/wallet_move_to_gifts.cpp` | `TCG-N15` | `WalletMoveToGifts` | `crates/gpui_telegram_agent/src/wallet/wallet_move_to_gifts.rs` | 852 | `tests/test_tcg_n15_wallet_move_to_gifts.rs` |
| `Telegram/SourceFiles/wallet/wallet_move_to_gifts.h` | `TCG-N15` | `WalletMoveToGifts` | `crates/gpui_telegram_agent/src/wallet/wallet_move_to_gifts.rs` | 41 | `tests/test_tcg_n15_wallet_move_to_gifts.rs` |
| `Telegram/SourceFiles/wallet/wallet_onramp.cpp` | `CAP-06` | `WalletOnramp` | `crates/gpui_telegram_agent/src/wallet/wallet_onramp.rs` | 126 | `tests/test_cap_06_wallet_onramp.rs` |
| `Telegram/SourceFiles/wallet/wallet_onramp.h` | `CAP-06` | `WalletOnramp` | `crates/gpui_telegram_agent/src/wallet/wallet_onramp.rs` | 48 | `tests/test_cap_06_wallet_onramp.rs` |
| `Telegram/SourceFiles/wallet/wallet_own_card.cpp` | `CAP-06` | `WalletOwnCard` | `crates/gpui_telegram_agent/src/wallet/wallet_own_card.rs` | 268 | `tests/test_cap_06_wallet_own_card.rs` |
| `Telegram/SourceFiles/wallet/wallet_own_card.h` | `CAP-06` | `WalletOwnCard` | `crates/gpui_telegram_agent/src/wallet/wallet_own_card.rs` | 88 | `tests/test_cap_06_wallet_own_card.rs` |
| `Telegram/SourceFiles/wallet/wallet_palette.cpp` | `TCG-N31` | `WalletPalette` | `crates/gpui_telegram_agent/src/wallet/wallet_palette.rs` | 300 | `tests/test_tcg_n31_wallet_palette.rs` |
| `Telegram/SourceFiles/wallet/wallet_palette.h` | `TCG-N31` | `WalletPalette` | `crates/gpui_telegram_agent/src/wallet/wallet_palette.rs` | 72 | `tests/test_tcg_n31_wallet_palette.rs` |
| `Telegram/SourceFiles/wallet/wallet_panel.cpp` | `CAP-06` | `WalletPanel` | `crates/gpui_telegram_agent/src/wallet/wallet_panel.rs` | 273 | `tests/test_cap_06_wallet_panel.rs` |
| `Telegram/SourceFiles/wallet/wallet_panel.h` | `CAP-06` | `WalletPanel` | `crates/gpui_telegram_agent/src/wallet/wallet_panel.rs` | 77 | `tests/test_cap_06_wallet_panel.rs` |
| `Telegram/SourceFiles/wallet/wallet_phrase_shares.cpp` | `CAP-06` | `WalletPhraseShares` | `crates/gpui_telegram_agent/src/wallet/wallet_phrase_shares.rs` | 147 | `tests/test_cap_06_wallet_phrase_shares.rs` |
| `Telegram/SourceFiles/wallet/wallet_phrase_shares.h` | `CAP-06` | `WalletPhraseShares` | `crates/gpui_telegram_agent/src/wallet/wallet_phrase_shares.rs` | 47 | `tests/test_cap_06_wallet_phrase_shares.rs` |
| `Telegram/SourceFiles/wallet/wallet_rates.cpp` | `CAP-06` | `WalletRates` | `crates/gpui_telegram_agent/src/wallet/wallet_rates.rs` | 155 | `tests/test_cap_06_wallet_rates.rs` |
| `Telegram/SourceFiles/wallet/wallet_rates.h` | `CAP-06` | `WalletRates` | `crates/gpui_telegram_agent/src/wallet/wallet_rates.rs` | 45 | `tests/test_cap_06_wallet_rates.rs` |
| `Telegram/SourceFiles/wallet/wallet_send_preparation.cpp` | `CAP-06` | `WalletSendPreparation` | `crates/gpui_telegram_agent/src/wallet/wallet_send_preparation.rs` | 306 | `tests/test_cap_06_wallet_send_preparation.rs` |
| `Telegram/SourceFiles/wallet/wallet_send_preparation.h` | `CAP-06` | `WalletSendPreparation` | `crates/gpui_telegram_agent/src/wallet/wallet_send_preparation.rs` | 106 | `tests/test_cap_06_wallet_send_preparation.rs` |
| `Telegram/SourceFiles/wallet/wallet_sending_effects.cpp` | `CAP-06` | `WalletSendingEffects` | `crates/gpui_telegram_agent/src/wallet/wallet_sending_effects.rs` | 133 | `tests/test_cap_06_wallet_sending_effects.rs` |
| `Telegram/SourceFiles/wallet/wallet_sending_effects.h` | `CAP-06` | `WalletSendingEffects` | `crates/gpui_telegram_agent/src/wallet/wallet_sending_effects.rs` | 69 | `tests/test_cap_06_wallet_sending_effects.rs` |
| `Telegram/SourceFiles/wallet/wallet_session.cpp` | `CAP-06` | `WalletSession` | `crates/gpui_telegram_agent/src/wallet/wallet_session.rs` | 11427 | `tests/test_cap_06_wallet_session.rs` |
| `Telegram/SourceFiles/wallet/wallet_session.h` | `CAP-06` | `WalletSession` | `crates/gpui_telegram_agent/src/wallet/wallet_session.rs` | 1396 | `tests/test_cap_06_wallet_session.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_deposit.cpp` | `CAP-06` | `WalletSessionDeposit` | `crates/gpui_telegram_agent/src/wallet/wallet_session_deposit.rs` | 265 | `tests/test_cap_06_wallet_session_deposit.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_deposit.h` | `CAP-06` | `WalletSessionDeposit` | `crates/gpui_telegram_agent/src/wallet/wallet_session_deposit.rs` | 65 | `tests/test_cap_06_wallet_session_deposit.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_gift_moves.cpp` | `TCG-N15` | `WalletSessionGiftMoves` | `crates/gpui_telegram_agent/src/wallet/wallet_session_gift_moves.rs` | 313 | `tests/test_tcg_n15_wallet_session_gift_moves.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_gift_moves.h` | `TCG-N15` | `WalletSessionGiftMoves` | `crates/gpui_telegram_agent/src/wallet/wallet_session_gift_moves.rs` | 42 | `tests/test_tcg_n15_wallet_session_gift_moves.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_waits.cpp` | `CAP-06` | `WalletSessionWaits` | `crates/gpui_telegram_agent/src/wallet/wallet_session_waits.rs` | 84 | `tests/test_cap_06_wallet_session_waits.rs` |
| `Telegram/SourceFiles/wallet/wallet_session_waits.h` | `CAP-06` | `WalletSessionWaits` | `crates/gpui_telegram_agent/src/wallet/wallet_session_waits.rs` | 45 | `tests/test_cap_06_wallet_session_waits.rs` |
| `Telegram/SourceFiles/wallet/wallet_stream.cpp` | `TCG-N52` | `WalletStream` | `crates/gpui_telegram_agent/src/wallet/wallet_stream.rs` | 299 | `tests/test_tcg_n52_wallet_stream.rs` |
| `Telegram/SourceFiles/wallet/wallet_stream.h` | `TCG-N52` | `WalletStream` | `crates/gpui_telegram_agent/src/wallet/wallet_stream.rs` | 86 | `tests/test_tcg_n52_wallet_stream.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect.cpp` | `CAP-06` | `WalletTonConnect` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect.rs` | 1686 | `tests/test_cap_06_wallet_ton_connect.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect.h` | `CAP-06` | `WalletTonConnect` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect.rs` | 329 | `tests/test_cap_06_wallet_ton_connect.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_box.cpp` | `CAP-06` | `WalletTonConnectBox` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_box.rs` | 656 | `tests/test_cap_06_wallet_ton_connect_box.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_box.h` | `CAP-06` | `WalletTonConnectBox` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_box.rs` | 88 | `tests/test_cap_06_wallet_ton_connect_box.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_claims.cpp` | `CAP-06` | `WalletTonConnectClaims` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_claims.rs` | 197 | `tests/test_cap_06_wallet_ton_connect_claims.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_claims.h` | `CAP-06` | `WalletTonConnectClaims` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_claims.rs` | 69 | `tests/test_cap_06_wallet_ton_connect_claims.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_emulation.cpp` | `CAP-06` | `WalletTonConnectEmulation` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_emulation.rs` | 259 | `tests/test_cap_06_wallet_ton_connect_emulation.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_emulation.h` | `CAP-06` | `WalletTonConnectEmulation` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_emulation.rs` | 73 | `tests/test_cap_06_wallet_ton_connect_emulation.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_link.cpp` | `CAP-06` | `WalletTonConnectLink` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_link.rs` | 182 | `tests/test_cap_06_wallet_ton_connect_link.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_link.h` | `CAP-06` | `WalletTonConnectLink` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_link.rs` | 42 | `tests/test_cap_06_wallet_ton_connect_link.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_login.cpp` | `CAP-06` | `WalletTonConnectLogin` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_login.rs` | 647 | `tests/test_cap_06_wallet_ton_connect_login.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_login.h` | `CAP-06` | `WalletTonConnectLogin` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_login.rs` | 146 | `tests/test_cap_06_wallet_ton_connect_login.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_request.cpp` | `CAP-06` | `WalletTonConnectRequest` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request.rs` | 2253 | `tests/test_cap_06_wallet_ton_connect_request.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_request.h` | `CAP-06` | `WalletTonConnectRequest` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request.rs` | 119 | `tests/test_cap_06_wallet_ton_connect_request.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_request_box.cpp` | `CAP-06` | `WalletTonConnectRequestBox` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request_box.rs` | 1032 | `tests/test_cap_06_wallet_ton_connect_request_box.rs` |
| `Telegram/SourceFiles/wallet/wallet_ton_connect_request_box.h` | `CAP-06` | `WalletTonConnectRequestBox` | `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request_box.rs` | 71 | `tests/test_cap_06_wallet_ton_connect_request_box.rs` |
| `Telegram/SourceFiles/wallet/wallet_transfer_messages.cpp` | `CAP-06` | `WalletTransferMessages` | `crates/gpui_telegram_agent/src/wallet/wallet_transfer_messages.rs` | 286 | `tests/test_cap_06_wallet_transfer_messages.rs` |
| `Telegram/SourceFiles/wallet/wallet_transfer_messages.h` | `CAP-06` | `WalletTransferMessages` | `crates/gpui_telegram_agent/src/wallet/wallet_transfer_messages.rs` | 61 | `tests/test_cap_06_wallet_transfer_messages.rs` |
| `Telegram/SourceFiles/wallet/wallet_transfer_store.cpp` | `CAP-06` | `WalletTransferStore` | `crates/gpui_telegram_agent/src/wallet/wallet_transfer_store.rs` | 428 | `tests/test_cap_06_wallet_transfer_store.rs` |
| `Telegram/SourceFiles/wallet/wallet_transfer_store.h` | `CAP-06` | `WalletTransferStore` | `crates/gpui_telegram_agent/src/wallet/wallet_transfer_store.rs` | 129 | `tests/test_cap_06_wallet_transfer_store.rs` |
| `Telegram/SourceFiles/wallet/wallet_unlock.cpp` | `CAP-06` | `WalletUnlock` | `crates/gpui_telegram_agent/src/wallet/wallet_unlock.rs` | 871 | `tests/test_cap_06_wallet_unlock.rs` |
| `Telegram/SourceFiles/wallet/wallet_unlock.h` | `CAP-06` | `WalletUnlock` | `crates/gpui_telegram_agent/src/wallet/wallet_unlock.rs` | 131 | `tests/test_cap_06_wallet_unlock.rs` |
| `Telegram/SourceFiles/wallet/wallet_user_addresses.cpp` | `CAP-06` | `WalletUserAddresses` | `crates/gpui_telegram_agent/src/wallet/wallet_user_addresses.rs` | 424 | `tests/test_cap_06_wallet_user_addresses.rs` |
| `Telegram/SourceFiles/wallet/wallet_user_addresses.h` | `CAP-06` | `WalletUserAddresses` | `crates/gpui_telegram_agent/src/wallet/wallet_user_addresses.rs` | 148 | `tests/test_cap_06_wallet_user_addresses.rs` |
| `Telegram/SourceFiles/wallet/wallet_vault.cpp` | `CAP-06` | `WalletVault` | `crates/gpui_telegram_agent/src/wallet/wallet_vault.rs` | 1410 | `tests/test_cap_06_wallet_vault.rs` |
| `Telegram/SourceFiles/wallet/wallet_vault.h` | `CAP-06` | `WalletVault` | `crates/gpui_telegram_agent/src/wallet/wallet_vault.rs` | 333 | `tests/test_cap_06_wallet_vault.rs` |
| `Telegram/SourceFiles/wallet/wallet_window_transition.cpp` | `CAP-06` | `WalletWindowTransition` | `crates/gpui_telegram_agent/src/wallet/wallet_window_transition.rs` | 638 | `tests/test_cap_06_wallet_window_transition.rs` |
| `Telegram/SourceFiles/wallet/wallet_window_transition.h` | `CAP-06` | `WalletWindowTransition` | `crates/gpui_telegram_agent/src/wallet/wallet_window_transition.rs` | 134 | `tests/test_cap_06_wallet_window_transition.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/gram/api/gram_api_account.h`
- **Classes & Structs**: AddressFunds
- **Target**: `GramApiAccountApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_account.rs`

### `Telegram/SourceFiles/gram/api/gram_api_emulate.h`
- **Classes & Structs**: EmulatedTrace
- **Target**: `GramApiEmulateApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_emulate.rs`

### `Telegram/SourceFiles/gram/api/gram_api_nft.cpp`
- **Classes & Structs**: FragmentCollection
- **Core Methods & Signals**: QString::number, QString::fromUtf8, ApiDetails::PercentEncoded, QJsonDocument::fromJson, QString, ClassifyNftKind
- **Target**: `GramApiNftApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs`

### `Telegram/SourceFiles/gram/api/gram_api_nft.h`
- **Classes & Structs**: NftKind, NftWebDocument, NftItem, NftPage
- **Core Methods & Signals**: ClassifyNftKind
- **Target**: `GramApiNftApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_nft.rs`

### `Telegram/SourceFiles/gram/api/gram_api_rates.h`
- **Classes & Structs**: CurrencyRates, CurrencyRateEntry
- **Target**: `GramApiRatesApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_rates.rs`

### `Telegram/SourceFiles/gram/api/gram_api_request.h`
- **Classes & Structs**: HttpRequest, ApiError
- **Target**: `GramApiRequestApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_request.rs`

### `Telegram/SourceFiles/gram/api/gram_api_stream.h`
- **Classes & Structs**: StreamEndpoint, StreamEventKind, StreamEvent
- **Core Methods & Signals**: Stream::mine
- **Target**: `GramApiStreamApi` in `crates/gpui_telegram_agent/src/wallet/gram_api_stream.rs`

### `Telegram/SourceFiles/gram/gram_boc.cpp`
- **Classes & Structs**: Cell, ParsedBoc, BitReader
- **Core Methods & Signals**: BitReader::BitReader, BitReader::read, QByteArray::fromBase64, QString::fromLatin1, QString::fromUtf8, QString::number
- **Target**: `GramBoc` in `crates/gpui_telegram_agent/src/wallet/gram_boc.rs`

### `Telegram/SourceFiles/gram/gram_boc.h`
- **Classes & Structs**: namespace Gram
- **Style Tokens**: #include <QtCore/QString>, #include <optional>
- **Target**: `GramBoc` in `crates/gpui_telegram_agent/src/wallet/gram_boc.rs`

### `Telegram/SourceFiles/gram/gram_pch.h`
- **Style Tokens**: #include <QtCore/QString>, #include <QtCore/QByteArray>, #include <crl/crl.h>, #include <rpl/rpl.h>, #include <vector>, #include <optional>
- **Target**: `GramPch` in `crates/gpui_telegram_agent/src/wallet/gram_pch.rs`

