# Subsystem Shard: `mtproto`

**Total Source Files**: 76  
**Total Source Lines**: 25,288  
**Primary Feature IDs**: TCG-N02, TCG-N20, TCG-N35  
**JSON Shard Path**: `docs/specs/ledger/shards/mtproto.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/mtproto/config_loader.cpp` | `TCG-N20` | `ConfigLoader` | `crates/gpui_telegram_core/src/mtproto/config_loader.rs` | 242 | `tests/test_tcg_n20_config_loader.rs` |
| `Telegram/SourceFiles/mtproto/config_loader.h` | `TCG-N20` | `ConfigLoader` | `crates/gpui_telegram_core/src/mtproto/config_loader.rs` | 84 | `tests/test_tcg_n20_config_loader.rs` |
| `Telegram/SourceFiles/mtproto/connection_abstract.cpp` | `TCG-N20` | `ConnectionAbstract` | `crates/gpui_telegram_core/src/mtproto/connection_abstract.rs` | 224 | `tests/test_tcg_n20_connection_abstract.rs` |
| `Telegram/SourceFiles/mtproto/connection_abstract.h` | `TCG-N20` | `ConnectionAbstract` | `crates/gpui_telegram_core/src/mtproto/connection_abstract.rs` | 214 | `tests/test_tcg_n20_connection_abstract.rs` |
| `Telegram/SourceFiles/mtproto/connection_http.cpp` | `TCG-N20` | `ConnectionHttp` | `crates/gpui_telegram_core/src/mtproto/connection_http.rs` | 289 | `tests/test_tcg_n20_connection_http.rs` |
| `Telegram/SourceFiles/mtproto/connection_http.h` | `TCG-N20` | `ConnectionHttp` | `crates/gpui_telegram_core/src/mtproto/connection_http.rs` | 69 | `tests/test_tcg_n20_connection_http.rs` |
| `Telegram/SourceFiles/mtproto/connection_resolving.cpp` | `TCG-N20` | `ConnectionResolving` | `crates/gpui_telegram_core/src/mtproto/connection_resolving.rs` | 250 | `tests/test_tcg_n20_connection_resolving.rs` |
| `Telegram/SourceFiles/mtproto/connection_resolving.h` | `TCG-N20` | `ConnectionResolving` | `crates/gpui_telegram_core/src/mtproto/connection_resolving.rs` | 72 | `tests/test_tcg_n20_connection_resolving.rs` |
| `Telegram/SourceFiles/mtproto/connection_tcp.cpp` | `TCG-N20` | `ConnectionTcp` | `crates/gpui_telegram_core/src/mtproto/connection_tcp.rs` | 680 | `tests/test_tcg_n20_connection_tcp.rs` |
| `Telegram/SourceFiles/mtproto/connection_tcp.h` | `TCG-N20` | `ConnectionTcp` | `crates/gpui_telegram_core/src/mtproto/connection_tcp.rs` | 98 | `tests/test_tcg_n20_connection_tcp.rs` |
| `Telegram/SourceFiles/mtproto/core_types.h` | `TCG-N20` | `CoreTypes` | `crates/gpui_telegram_core/src/mtproto/core_types.rs` | 319 | `tests/test_tcg_n20_core_types.rs` |
| `Telegram/SourceFiles/mtproto/dedicated_file_loader.cpp` | `TCG-N20` | `DedicatedFileLoader` | `crates/gpui_telegram_core/src/mtproto/dedicated_file_loader.rs` | 512 | `tests/test_tcg_n20_dedicated_file_loader.rs` |
| `Telegram/SourceFiles/mtproto/dedicated_file_loader.h` | `TCG-N20` | `DedicatedFileLoader` | `crates/gpui_telegram_core/src/mtproto/dedicated_file_loader.rs` | 219 | `tests/test_tcg_n20_dedicated_file_loader.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_abstract_socket.cpp` | `TCG-N20` | `MtprotoAbstractSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_abstract_socket.rs` | 71 | `tests/test_tcg_n20_mtproto_abstract_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_abstract_socket.h` | `TCG-N20` | `MtprotoAbstractSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_abstract_socket.rs` | 76 | `tests/test_tcg_n20_mtproto_abstract_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_bound_key_creator.cpp` | `TCG-N20` | `MtprotoBoundKeyCreatorDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_bound_key_creator.rs` | 92 | `tests/test_tcg_n20_mtproto_bound_key_creator.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_bound_key_creator.h` | `TCG-N20` | `MtprotoBoundKeyCreatorDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_bound_key_creator.rs` | 56 | `tests/test_tcg_n20_mtproto_bound_key_creator.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dc_key_binder.cpp` | `TCG-N20` | `MtprotoDcKeyBinderDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dc_key_binder.rs` | 132 | `tests/test_tcg_n20_mtproto_dc_key_binder.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dc_key_binder.h` | `TCG-N20` | `MtprotoDcKeyBinderDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dc_key_binder.rs` | 42 | `tests/test_tcg_n20_mtproto_dc_key_binder.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dc_key_creator.cpp` | `TCG-N20` | `MtprotoDcKeyCreatorDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dc_key_creator.rs` | 855 | `tests/test_tcg_n20_mtproto_dc_key_creator.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dc_key_creator.h` | `TCG-N20` | `MtprotoDcKeyCreatorDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dc_key_creator.rs` | 139 | `tests/test_tcg_n20_mtproto_dc_key_creator.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dcenter.cpp` | `TCG-N20` | `MtprotoDcenterDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dcenter.rs` | 166 | `tests/test_tcg_n20_mtproto_dcenter.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dcenter.h` | `TCG-N20` | `MtprotoDcenterDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dcenter.rs` | 70 | `tests/test_tcg_n20_mtproto_dcenter.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_domain_resolver.cpp` | `TCG-N20` | `MtprotoDomainResolverDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_domain_resolver.rs` | 367 | `tests/test_tcg_n20_mtproto_domain_resolver.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_domain_resolver.h` | `TCG-N20` | `MtprotoDomainResolverDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_domain_resolver.rs` | 106 | `tests/test_tcg_n20_mtproto_domain_resolver.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dump_to_text.cpp` | `TCG-N20` | `MtprotoDumpToTextDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dump_to_text.rs` | 172 | `tests/test_tcg_n20_mtproto_dump_to_text.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_dump_to_text.h` | `TCG-N20` | `MtprotoDumpToTextDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_dump_to_text.rs` | 82 | `tests/test_tcg_n20_mtproto_dump_to_text.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_received_ids_manager.cpp` | `TCG-N20` | `MtprotoReceivedIdsManagerDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_received_ids_manager.rs` | 55 | `tests/test_tcg_n20_mtproto_received_ids_manager.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_received_ids_manager.h` | `TCG-N20` | `MtprotoReceivedIdsManagerDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_received_ids_manager.rs` | 43 | `tests/test_tcg_n20_mtproto_received_ids_manager.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_rsa_public_key.cpp` | `TCG-N20` | `MtprotoRsaPublicKeyDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_rsa_public_key.rs` | 266 | `tests/test_tcg_n20_mtproto_rsa_public_key.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_rsa_public_key.h` | `TCG-N20` | `MtprotoRsaPublicKeyDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_rsa_public_key.rs` | 49 | `tests/test_tcg_n20_mtproto_rsa_public_key.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_serialized_request.cpp` | `TCG-N20` | `MtprotoSerializedRequestDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_serialized_request.rs` | 156 | `tests/test_tcg_n20_mtproto_serialized_request.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_serialized_request.h` | `TCG-N20` | `MtprotoSerializedRequestDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_serialized_request.rs` | 107 | `tests/test_tcg_n20_mtproto_serialized_request.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_tcp_socket.cpp` | `TCG-N20` | `MtprotoTcpSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_tcp_socket.rs` | 127 | `tests/test_tcg_n20_mtproto_tcp_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_tcp_socket.h` | `TCG-N20` | `MtprotoTcpSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_tcp_socket.rs` | 39 | `tests/test_tcg_n20_mtproto_tcp_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_tls_socket.cpp` | `TCG-N20` | `MtprotoTlsSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_tls_socket.rs` | 959 | `tests/test_tcg_n20_mtproto_tls_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_tls_socket.h` | `TCG-N20` | `MtprotoTlsSocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_tls_socket.rs` | 68 | `tests/test_tcg_n20_mtproto_tls_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_web_proxy_socket.cpp` | `TCG-N35` | `MtprotoWebProxySocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_web_proxy_socket.rs` | 163 | `tests/test_tcg_n35_mtproto_web_proxy_socket.rs` |
| `Telegram/SourceFiles/mtproto/details/mtproto_web_proxy_socket.h` | `TCG-N35` | `MtprotoWebProxySocketDetails` | `crates/gpui_telegram_core/src/mtproto/details/mtproto_web_proxy_socket.rs` | 61 | `tests/test_tcg_n35_mtproto_web_proxy_socket.rs` |
| `Telegram/SourceFiles/mtproto/facade.cpp` | `TCG-N20` | `Facade` | `crates/gpui_telegram_core/src/mtproto/facade.rs` | 43 | `tests/test_tcg_n20_facade.rs` |
| `Telegram/SourceFiles/mtproto/facade.h` | `TCG-N20` | `Facade` | `crates/gpui_telegram_core/src/mtproto/facade.rs` | 128 | `tests/test_tcg_n20_facade.rs` |
| `Telegram/SourceFiles/mtproto/mtp_instance.cpp` | `TCG-N20` | `MtpInstance` | `crates/gpui_telegram_core/src/mtproto/mtp_instance.rs` | 2223 | `tests/test_tcg_n20_mtp_instance.rs` |
| `Telegram/SourceFiles/mtproto/mtp_instance.h` | `TCG-N20` | `MtpInstance` | `crates/gpui_telegram_core/src/mtproto/mtp_instance.rs` | 247 | `tests/test_tcg_n20_mtp_instance.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_auth_key.cpp` | `TCG-N20` | `MtprotoAuthKey` | `crates/gpui_telegram_core/src/mtproto/mtproto_auth_key.rs` | 190 | `tests/test_tcg_n20_mtproto_auth_key.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_auth_key.h` | `TCG-N20` | `MtprotoAuthKey` | `crates/gpui_telegram_core/src/mtproto/mtproto_auth_key.rs` | 125 | `tests/test_tcg_n20_mtproto_auth_key.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_concurrent_sender.cpp` | `TCG-N20` | `MtprotoConcurrentSender` | `crates/gpui_telegram_core/src/mtproto/mtproto_concurrent_sender.rs` | 200 | `tests/test_tcg_n20_mtproto_concurrent_sender.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_concurrent_sender.h` | `TCG-N20` | `MtprotoConcurrentSender` | `crates/gpui_telegram_core/src/mtproto/mtproto_concurrent_sender.rs` | 433 | `tests/test_tcg_n20_mtproto_concurrent_sender.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_config.cpp` | `TCG-N20` | `MtprotoConfig` | `crates/gpui_telegram_core/src/mtproto/mtproto_config.rs` | 317 | `tests/test_tcg_n20_mtproto_config.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_config.h` | `TCG-N20` | `MtprotoConfig` | `crates/gpui_telegram_core/src/mtproto/mtproto_config.rs` | 95 | `tests/test_tcg_n20_mtproto_config.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_dc_options.cpp` | `TCG-N20` | `MtprotoDcOptions` | `crates/gpui_telegram_core/src/mtproto/mtproto_dc_options.rs` | 845 | `tests/test_tcg_n20_mtproto_dc_options.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_dc_options.h` | `TCG-N20` | `MtprotoDcOptions` | `crates/gpui_telegram_core/src/mtproto/mtproto_dc_options.rs` | 167 | `tests/test_tcg_n20_mtproto_dc_options.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_dh_utils.cpp` | `TCG-N20` | `MtprotoDhUtils` | `crates/gpui_telegram_core/src/mtproto/mtproto_dh_utils.rs` | 175 | `tests/test_tcg_n20_mtproto_dh_utils.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_dh_utils.h` | `TCG-N20` | `MtprotoDhUtils` | `crates/gpui_telegram_core/src/mtproto/mtproto_dh_utils.rs` | 38 | `tests/test_tcg_n20_mtproto_dh_utils.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_pch.h` | `TCG-N20` | `MtprotoPch` | `crates/gpui_telegram_core/src/mtproto/mtproto_pch.rs` | 23 | `tests/test_tcg_n20_mtproto_pch.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_proxy_data.cpp` | `TCG-N35` | `MtprotoProxyData` | `crates/gpui_telegram_core/src/mtproto/mtproto_proxy_data.rs` | 592 | `tests/test_tcg_n35_mtproto_proxy_data.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_proxy_data.h` | `TCG-N35` | `MtprotoProxyData` | `crates/gpui_telegram_core/src/mtproto/mtproto_proxy_data.rs` | 80 | `tests/test_tcg_n35_mtproto_proxy_data.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_response.cpp` | `TCG-N20` | `MtprotoResponse` | `crates/gpui_telegram_core/src/mtproto/mtproto_response.rs` | 91 | `tests/test_tcg_n20_mtproto_response.rs` |
| `Telegram/SourceFiles/mtproto/mtproto_response.h` | `TCG-N20` | `MtprotoResponse` | `crates/gpui_telegram_core/src/mtproto/mtproto_response.rs` | 87 | `tests/test_tcg_n20_mtproto_response.rs` |
| `Telegram/SourceFiles/mtproto/proxy_check.cpp` | `TCG-N35` | `ProxyCheck` | `crates/gpui_telegram_core/src/mtproto/proxy_check.rs` | 114 | `tests/test_tcg_n35_proxy_check.rs` |
| `Telegram/SourceFiles/mtproto/proxy_check.h` | `TCG-N35` | `ProxyCheck` | `crates/gpui_telegram_core/src/mtproto/proxy_check.rs` | 35 | `tests/test_tcg_n35_proxy_check.rs` |
| `Telegram/SourceFiles/mtproto/scheme/api.tl` | `TCG-N20` | `ApiSchemeTlAsset` | `crates/gpui_telegram_core/assets/api_tl.rs` | 3269 | `tests/test_tcg_n20_api.rs` |
| `Telegram/SourceFiles/mtproto/scheme/mtproto.tl` | `TCG-N20` | `MtprotoSchemeTlAsset` | `crates/gpui_telegram_core/assets/mtproto_tl.rs` | 128 | `tests/test_tcg_n20_mtproto.rs` |
| `Telegram/SourceFiles/mtproto/sender.h` | `TCG-N20` | `Sender` | `crates/gpui_telegram_core/src/mtproto/sender.rs` | 477 | `tests/test_tcg_n20_sender.rs` |
| `Telegram/SourceFiles/mtproto/session.cpp` | `TCG-N02` | `Session` | `crates/gpui_telegram_core/src/mtproto/session.rs` | 590 | `tests/test_tcg_n02_session.rs` |
| `Telegram/SourceFiles/mtproto/session.h` | `TCG-N02` | `Session` | `crates/gpui_telegram_core/src/mtproto/session.rs` | 222 | `tests/test_tcg_n02_session.rs` |
| `Telegram/SourceFiles/mtproto/session_private.cpp` | `TCG-N02` | `SessionPrivate` | `crates/gpui_telegram_core/src/mtproto/session_private.rs` | 2811 | `tests/test_tcg_n02_session_private.rs` |
| `Telegram/SourceFiles/mtproto/session_private.h` | `TCG-N02` | `SessionPrivate` | `crates/gpui_telegram_core/src/mtproto/session_private.rs` | 251 | `tests/test_tcg_n02_session_private.rs` |
| `Telegram/SourceFiles/mtproto/special_config_request.cpp` | `TCG-N20` | `SpecialConfigRequest` | `crates/gpui_telegram_core/src/mtproto/special_config_request.rs` | 550 | `tests/test_tcg_n20_special_config_request.rs` |
| `Telegram/SourceFiles/mtproto/special_config_request.h` | `TCG-N20` | `SpecialConfigRequest` | `crates/gpui_telegram_core/src/mtproto/special_config_request.rs` | 85 | `tests/test_tcg_n20_special_config_request.rs` |
| `Telegram/SourceFiles/mtproto/type_utils.h` | `TCG-N20` | `TypeUtils` | `crates/gpui_telegram_core/src/mtproto/type_utils.rs` | 19 | `tests/test_tcg_n20_type_utils.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_frame.cpp` | `TCG-N35` | `WebProxyFrameWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_frame.rs` | 94 | `tests/test_tcg_n35_web_proxy_frame.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_frame.h` | `TCG-N35` | `WebProxyFrameWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_frame.rs` | 51 | `tests/test_tcg_n35_web_proxy_frame.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_transport.cpp` | `TCG-N35` | `WebProxyTransportWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_transport.rs` | 2056 | `tests/test_tcg_n35_web_proxy_transport.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_transport.h` | `TCG-N35` | `WebProxyTransportWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_transport.rs` | 83 | `tests/test_tcg_n35_web_proxy_transport.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_webview.cpp` | `TCG-N35` | `WebProxyWebviewWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_webview.rs` | 468 | `tests/test_tcg_n35_web_proxy_webview.rs` |
| `Telegram/SourceFiles/mtproto/web_proxy/web_proxy_webview.h` | `TCG-N35` | `WebProxyWebviewWebProxy` | `crates/gpui_telegram_core/src/mtproto/web_proxy/web_proxy_webview.rs` | 95 | `tests/test_tcg_n35_web_proxy_webview.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/mtproto/config_loader.cpp`
- **Classes & Structs**: Flag
- **Core Methods & Signals**: ConfigLoader::ConfigLoader, ConfigLoader::load, ConfigLoader::sendRequest, ConfigLoader::specialToRealDcId, ConfigLoader::terminateRequest, MTP::configDcId
- **Target**: `ConfigLoader` in `crates/gpui_telegram_core/src/mtproto/config_loader.rs`

### `Telegram/SourceFiles/mtproto/config_loader.h`
- **Classes & Structs**: Instance, SpecialConfigRequest, ConfigLoader, SpecialEndpoint
- **Core Methods & Signals**: load, setPhone, setProxyEnabled, sendRequest, addSpecialEndpoint, sendSpecialRequest
- **Target**: `ConfigLoader` in `crates/gpui_telegram_core/src/mtproto/config_loader.rs`

### `Telegram/SourceFiles/mtproto/connection_abstract.h`
- **Classes & Structs**: Instance, ConnectionOptions, AbstractConnection, ConnectionPointer, BuffersQueue
- **Core Methods & Signals**: AbstractConnection::prepareNotSecurePacket, tl::count_length, bytes::set_random, Logs::DebugEnabled, ConnectionPointer, New
- **Target**: `ConnectionAbstract` in `crates/gpui_telegram_core/src/mtproto/connection_abstract.rs`

### `Telegram/SourceFiles/mtproto/connection_http.h`
- **Classes & Structs**: HttpConnection, Status
- **Core Methods & Signals**: HttpConnection, clone, pingTime, fullConnectTimeout, sendData, disconnectFromServer
- **Target**: `ConnectionHttp` in `crates/gpui_telegram_core/src/mtproto/connection_http.rs`

### `Telegram/SourceFiles/mtproto/connection_resolving.h`
- **Classes & Structs**: ResolvingConnection
- **Core Methods & Signals**: ResolvingConnection, clone, pingTime, fullConnectTimeout, sendData, disconnectFromServer
- **Target**: `ConnectionResolving` in `crates/gpui_telegram_core/src/mtproto/connection_resolving.rs`

### `Telegram/SourceFiles/mtproto/connection_tcp.cpp`
- **Classes & Structs**: TcpConnection, Version0, Version1, VersionD
- **Core Methods & Signals**: Version0::id, Version0::supportsArbitraryLength, Version0::prepareKey, bytes::copy, Version0::finalizePacket, bytes::make_span
- **Target**: `ConnectionTcp` in `crates/gpui_telegram_core/src/mtproto/connection_tcp.rs`

### `Telegram/SourceFiles/mtproto/connection_tcp.h`
- **Classes & Structs**: AbstractSocket, TcpConnection, Status, Protocol
- **Core Methods & Signals**: TcpConnection, clone, pingTime, fullConnectTimeout, sendData, disconnectFromServer
- **Target**: `ConnectionTcp` in `crates/gpui_telegram_core/src/mtproto/connection_tcp.rs`

### `Telegram/SourceFiles/mtproto/core_types.h`
- **Classes & Structs**: NotSingleDataTypePlaceholder, Writer, Reader, mtpPrime, mtpRequestId, mtpMsgId, mtpPingId, mtpBuffer, mtpTypeId, DcId, ShiftedDcId, MTPint
- **Core Methods & Signals**: tl::make_int, tl::make_flags, details::zero_flags_helper, tl::make_long, tl::make_int128, tl::make_int256
- **Target**: `CoreTypes` in `crates/gpui_telegram_core/src/mtproto/core_types.rs`

### `Telegram/SourceFiles/mtproto/dedicated_file_loader.cpp`
- **Classes & Structs**: ResolveResult
- **Core Methods & Signals**: WeakInstance::WeakInstance, WeakInstance::session, WeakInstance::valid, WeakInstance::instance, WeakInstance::die, Error::Local
- **Target**: `DedicatedFileLoader` in `crates/gpui_telegram_core/src/mtproto/dedicated_file_loader.rs`

### `Telegram/SourceFiles/mtproto/dedicated_file_loader.h`
- **Classes & Structs**: Session, WeakInstance, AbstractDedicatedLoader, Progress, DedicatedLoader, Location, File, Request, Result
- **Core Methods & Signals**: WeakInstance::send, MTP::IsDefaultHandledError, WeakInstance, die, removeRequest, AbstractDedicatedLoader
- **Target**: `DedicatedFileLoader` in `crates/gpui_telegram_core/src/mtproto/dedicated_file_loader.rs`

