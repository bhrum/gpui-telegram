//! Adversarial Test Suite for Module 13: TON Crypto, BOC/Cell, TonConnect, and Telegram Passport
//! Executed by challenger_module_13_2 under Mandate #15-18 & AGENTS.md hygiene mandate.
//!
//! Subsystems covered:
//! 1. TON BIP39 Mnemonic Derivation & Seed Generation Invariants
//! 2. TON Cell Tree Structure & Bit/Ref Overflow Boundary Limits
//! 3. Bag of Cells (BOC) Deserialization Security, CRC32-C Validation & Cyclic Cell Protection
//! 4. TonConnect 2.0 Protocol Injection, Deep Link Schemes & Manifest Invariants
//! 5. TonConnect Proof Authentication, Expiration Bounds & Replay Defenses
//! 6. Telegram Passport End-to-End Cryptography: Modulo-255 Checksum & Ciphertext Integrity

use std::collections::BTreeMap;
use gpui_telegram_client::wallet::core::ton_account_id::TonAccountId;
use gpui_telegram_client::wallet::core::ton_bip39::{find_word_index, is_valid_word, BIP39_ENGLISH};
use gpui_telegram_client::wallet::core::ton_cell::{TonCell, MAX_CELL_REFS};
use gpui_telegram_client::wallet::core::ton_hash::{crc16_xmodem, crc32c, sha256, sha512};
use gpui_telegram_client::wallet::core::ton_keypair::TonKeyPair;
use gpui_telegram_client::wallet::core::ton_mnemonic::{TonMnemonic, TON_MNEMONIC_WORDS_COUNT};
use gpui_telegram_agent::wallet::gram_boc::{crc32c as boc_crc32c, parse_boc, text_comment_from_boc, GramBoc, BOC_MAGIC};
use gpui_telegram_agent::wallet::wallet_ton_connect::{TonConnectManifest, WalletTonConnect, APP_NAME_LIMIT};
use gpui_telegram_agent::wallet::wallet_ton_connect_link::WalletTonConnectLink;
use gpui_telegram_agent::wallet::wallet_ton_connect_login::{WalletTonConnectLogin, TON_PROOF_MAX_AGE_SECONDS};
use gpui_telegram_agent::wallet::wallet_ton_connect_request::{SendTransactionRpcParams, TonConnectMessage, WalletTonConnectRequest};
use gpui_telegram_core::passport::passport_encryption::{PassportEncryption, ALIGN_TO, MIN_PADDING, SECRET_SIZE};

#[test]
fn test_adversarial_ton_bip39_and_mnemonic_invariants() {
    // 1. Boundary word counts must be rejected
    let invalid_counts = [0, 1, 2, 11, 12, 16, 23, 25, 48];
    for &count in &invalid_counts {
        let words: Vec<String> = (0..count).map(|_| "abandon".to_string()).collect();
        assert!(
            TonMnemonic::from_words(words).is_none(),
            "Mnemonic with invalid count {count} must be rejected"
        );
    }

    // 2. Exact 24 words with valid BIP-39 dictionary entries must be accepted
    let valid_24: Vec<String> = (0..TON_MNEMONIC_WORDS_COUNT).map(|_| "abandon".to_string()).collect();
    let mnemonic = TonMnemonic::from_words(valid_24).expect("24 valid words accepted");
    assert_eq!(mnemonic.words.len(), 24);

    // 3. Unknown / non-BIP39 words must be rejected immediately
    let mut bad_words: Vec<String> = (0..TON_MNEMONIC_WORDS_COUNT).map(|_| "abandon".to_string()).collect();
    bad_words[12] = "not_a_bip39_word".to_string();
    assert!(TonMnemonic::from_words(bad_words).is_none());

    let mut bad_words_crypto: Vec<String> = (0..TON_MNEMONIC_WORDS_COUNT).map(|_| "abandon".to_string()).collect();
    bad_words_crypto[0] = "toncoin".to_string();
    assert!(TonMnemonic::from_words(bad_words_crypto).is_none());

    // 4. Test first and last words in BIP-39 wordlist
    assert_eq!(find_word_index("abandon"), Some(0));
    assert_eq!(find_word_index("zoo"), Some(BIP39_ENGLISH.len() - 1));
    assert!(is_valid_word("abandon"));
    assert!(is_valid_word("zoo"));
    assert!(!is_valid_word("telegram"));
    assert!(!is_valid_word(""));

    // Case insensitivity and whitespace handling
    assert_eq!(find_word_index("  ABANDON  "), Some(0));
    assert_eq!(find_word_index("Zoo\n"), Some(BIP39_ENGLISH.len() - 1));

    // 5. Seed derivation determinism and salt permutation invariants
    let phrase_str = mnemonic.to_string_phrase();
    assert_eq!(phrase_str.split_whitespace().count(), 24);

    let seed_default_1 = mnemonic.to_seed("");
    let seed_default_2 = mnemonic.to_seed("");
    assert_eq!(seed_default_1, seed_default_2, "Seed derivation must be 100% deterministic");
    assert_eq!(seed_default_1.len(), 64);

    // Empty passphrase uses "TON default seed", non-empty passphrase uses "TON seed <passphrase>"
    let seed_pass_a = mnemonic.to_seed("passphrase123");
    let seed_pass_b = mnemonic.to_seed("passphrase124");
    assert_ne!(seed_default_1, seed_pass_a, "Empty passphrase must diverge from custom passphrase");
    assert_ne!(seed_pass_a, seed_pass_b, "Different passphrases must produce completely distinct seeds");

    // 6. Cryptographic Keypair & Signature Verification from Derived Seed
    let keypair = TonKeyPair::from_secret_seed(&seed_default_1);
    assert_eq!(keypair.public_key.len(), 32);
    assert_eq!(keypair.secret_key.len(), 32);
    assert_eq!(keypair.public_key_hex().len(), 64);

    let msg1 = b"TON Transfer 100 TON to EQCGScrZe1xsobBeuqbpR9TLbk0mQmtZP29liBpxmmVUjWPB";
    let msg2 = b"TON Transfer 100 TON to EQCGScrZe1xsobBeuqbpR9TLbk0mQmtZP29liBpxmmVUjWPC"; // 1-bit delta
    let sig1 = keypair.sign_message(msg1);
    let sig2 = keypair.sign_message(msg2);
    assert_eq!(sig1.len(), 64);
    assert_eq!(sig2.len(), 64);
    assert_ne!(sig1, sig2, "Different messages must produce entirely distinct signatures");

    // Account ID formatting invariants
    let account = TonAccountId::new(0, keypair.public_key);
    assert_eq!(account.workchain, 0);
    assert!(account.to_raw().starts_with("0:"));
    let uf_bounceable = account.to_user_friendly(true, false);
    let uf_non_bounceable = account.to_user_friendly(false, false);
    assert_ne!(uf_bounceable, uf_non_bounceable);
    assert_eq!(uf_bounceable.len(), 48); // Standard base64url length with CRC16
}

#[test]
fn test_adversarial_ton_cell_and_bit_overflow_boundaries() {
    let mut cell = TonCell::new();
    assert_eq!(cell.bit_len, 0);
    assert_eq!(cell.references.len(), 0);

    // Write exactly 127 bytes = 1016 bits (capacity is 1023 bits)
    let fill_127 = vec![0x5A; 127];
    assert!(cell.write_bytes(&fill_127).is_ok());
    assert_eq!(cell.bit_len, 1016);
    assert_eq!(cell.data.len(), 127);

    // Attempting to write 1 more byte (8 bits) -> 1016 + 8 = 1024 > 1023 (Overflow!)
    let overflow_byte = [0xFF];
    let err = cell.write_bytes(&overflow_byte);
    assert_eq!(err, Err("Cell bit length overflow"));
    // Verify atomicity: cell bit_len and data must remain unchanged
    assert_eq!(cell.bit_len, 1016);
    assert_eq!(cell.data.len(), 127);

    // Attempting to write a u32 (32 bits) -> 1016 + 32 = 1048 > 1023
    assert_eq!(cell.write_u32(0xDEADBEEF), Err("Cell bit length overflow"));

    // Cell reference overflow: maximum allowed references is 4
    for i in 0..MAX_CELL_REFS {
        let child = TonCell::with_data(vec![i as u8], 8);
        assert!(cell.add_reference(child).is_ok());
    }
    assert_eq!(cell.references.len(), 4);

    // Adding 5th reference must be rejected
    let fifth_child = TonCell::with_data(vec![0xAA], 8);
    let ref_err = cell.add_reference(fifth_child);
    assert_eq!(ref_err, Err("Cell cannot exceed 4 references"));
    assert_eq!(cell.references.len(), 4);
}

#[test]
fn test_adversarial_boc_deserialization_and_cyclic_protection() {
    // 1. Truncated BOC inputs
    assert!(parse_boc(b"").is_none());
    assert!(parse_boc(&[0xB5, 0xEE, 0x9C]).is_none()); // 3 bytes
    assert!(parse_boc(&[0xB5, 0xEE, 0x9C, 0x72]).is_none()); // 4 bytes (magic only)
    assert!(parse_boc(&[0xB5, 0xEE, 0x9C, 0x72, 0x01, 0x01, 0x01, 0x01, 0x00]).is_none()); // 9 bytes (< 10)

    // 2. Corrupted Magic
    let mut bad_magic = vec![0x00; 20];
    bad_magic[0..4].copy_from_slice(&0xDEADBEEFu32.to_be_bytes());
    assert!(parse_boc(&bad_magic).is_none());

    // 3. CRC32-C (Castagnoli) known test vectors
    assert_eq!(boc_crc32c(b""), 0);
    assert_eq!(boc_crc32c(b"123456789"), 0xE3069283);
    assert_eq!(crc32c(b"123456789"), 0xE3069283);

    // CRC16-XMODEM known test vector
    assert_eq!(crc16_xmodem(b"123456789"), 0x31C3);

    // 4. Construct a valid minimal 1-cell BOC with text comment
    // Magic: 0xB5EE9C72
    // Flags byte: has_index=0, has_crc=0, has_cache=0, ref_size=1 -> 0x01
    // Offset size: 1 -> 0x01
    // cells_count: 1 byte = 1 -> 0x01
    // roots_count: 1 byte = 1 -> 0x01
    // absent_count: 1 byte = 0 -> 0x00
    // total_size: 1 byte (offset_size=1) = cell_header(2) + 4(opcode) + 5("hello") = 11 -> 0x0B
    // root_index: 1 byte = 0 -> 0x00
    // Cell 0:
    //   first byte: refs=0 -> 0x00
    //   second byte: data_size=9 bytes -> ((9 << 1) | 0) = 18 = 0x12 (whole bytes)
    //   data: opcode 0x00000000 (4 bytes) + "hello" (5 bytes)
    let mut valid_boc = Vec::new();
    valid_boc.extend_from_slice(&BOC_MAGIC.to_be_bytes());
    valid_boc.push(0x01); // flags: ref_size=1
    valid_boc.push(0x01); // offset_size=1
    valid_boc.push(0x01); // cells_count = 1
    valid_boc.push(0x01); // roots_count = 1
    valid_boc.push(0x00); // absent_count = 0
    valid_boc.push(11);   // total_size = 11
    valid_boc.push(0x00); // root_index = 0
    // Cell 0 payload:
    valid_boc.push(0x00); // 0 refs
    valid_boc.push(18);   // 9 bytes data (whole_bytes = true)
    valid_boc.extend_from_slice(&0u32.to_be_bytes()); // comment opcode
    valid_boc.extend_from_slice(b"hello");

    let parsed = parse_boc(&valid_boc).expect("Valid minimal BOC must parse");
    assert_eq!(parsed.cells.len(), 1);
    assert_eq!(parsed.root, 0);

    let comment = text_comment_from_boc(&valid_boc).expect("Comment must extract");
    assert_eq!(comment, "hello");

    // GramBoc high-level interface
    let mut gram_boc = GramBoc::new();
    assert!(gram_boc.parse(&valid_boc).is_some());
    assert_eq!(gram_boc.extract_comment(), Some("hello".to_string()));
    assert_ne!(gram_boc.compute_crc(), 0);

    // 5. Adversarial: Malicious Ref Count in Cell (> 4 refs)
    let mut invalid_refs_boc = valid_boc.clone();
    invalid_refs_boc[11] = 0x05; // refs = 5 (> 4 MAX_CELL_REFS)
    assert!(parse_boc(&invalid_refs_boc).is_none(), "Cell with > 4 references must be rejected");

    // 6. Adversarial: Non-Comment Opcode (e.g. 0x00000001)
    let mut non_comment_boc = valid_boc.clone();
    non_comment_boc[13..17].copy_from_slice(&1u32.to_be_bytes());
    assert!(text_comment_from_boc(&non_comment_boc).is_none(), "Non-zero opcode must not parse as text comment");

    // 7. Adversarial: Truncated Payload length mismatch
    let mut truncated_payload_boc = valid_boc.clone();
    truncated_payload_boc.pop(); // Remove last byte
    assert!(parse_boc(&truncated_payload_boc).is_none(), "Length mismatch must reject");

    // 8. Adversarial: Cyclic Cell References (Root cell 0 points to cell 1, cell 1 points back to cell 0)
    // cells_count = 2, roots_count = 1, absent_count = 0, ref_size = 1, offset_size = 1
    // Cell 0: refs=1, data_size=4 (opcode 0), ref[0]=1. header 2 + 4 + 1 = 7 bytes.
    // Cell 1: refs=1, data_size=5 ("cycle"), ref[0]=0. header 2 + 5 + 1 = 8 bytes.
    // Total size = 7 + 8 = 15 bytes.
    let mut cyclic_boc = Vec::new();
    cyclic_boc.extend_from_slice(&BOC_MAGIC.to_be_bytes());
    cyclic_boc.push(0x01); // ref_size = 1
    cyclic_boc.push(0x01); // offset_size = 1
    cyclic_boc.push(0x02); // cells_count = 2
    cyclic_boc.push(0x01); // roots_count = 1
    cyclic_boc.push(0x00); // absent_count = 0
    cyclic_boc.push(15);   // total_size = 15
    cyclic_boc.push(0x00); // root_index = 0
    // Cell 0: refs=1, data_size=4
    cyclic_boc.push(0x01); // 1 ref
    cyclic_boc.push(8);    // 4 bytes data
    cyclic_boc.extend_from_slice(&0u32.to_be_bytes()); // opcode 0
    cyclic_boc.push(0x01); // points to cell 1
    // Cell 1: refs=1, data_size=5 ("cycle")
    cyclic_boc.push(0x01); // 1 ref
    cyclic_boc.push(10);   // 5 bytes data
    cyclic_boc.extend_from_slice(b"cycle");
    cyclic_boc.push(0x00); // points back to cell 0! (CYCLE)

    let parsed_cyclic = parse_boc(&cyclic_boc).expect("Cyclic BOC syntax itself is valid");
    assert_eq!(parsed_cyclic.cells.len(), 2);

    // Comment extraction MUST safely detect the loop and return None rather than infinite looping or stack overflow
    let cyclic_result = text_comment_from_boc(&cyclic_boc);
    assert_eq!(cyclic_result, None, "Cyclic cell reference must be safely detected and terminated with None");
}

#[test]
fn test_adversarial_ton_connect_deep_link_and_manifest_injection() {
    let mut link_parser = WalletTonConnectLink::new();

    // 1. Rejected URL schemes
    let bad_schemes = [
        "http://evil.com/ton-connect?v=2&id=123",
        "https://evil.com/ton-connect?v=2&id=123",
        "ftp://app.org/connect?v=2&id=123",
        "javascript:alert(1)?v=2&id=123",
        "data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pg==",
    ];
    for &bad in &bad_schemes {
        assert!(link_parser.parse(bad).is_err(), "Scheme '{bad}' must be rejected");
    }

    // 2. Missing query string
    assert!(link_parser.parse("tc://").is_err());
    assert!(link_parser.parse("tc://?").is_err());

    // 3. Missing client session id
    assert!(link_parser.parse("tc://?v=2&r=https%3A%2F%2Fapp.org%2Fmanifest.json").is_err());

    // 4. Valid TON Connect URLs (tc://, ton://connect, tonkeeper)
    let valid_link = "tc://?v=2&id=sess_abcd1234&r=https%3A%2F%2Ffragment.com%2Fmanifest.json&ret=tg%3A%2F%2Fresolve";
    let parsed = link_parser.parse(valid_link).expect("Valid tc:// link must parse");
    assert_eq!(parsed.protocol_version, 2);
    assert_eq!(parsed.client_session_id, "sess_abcd1234");
    assert_eq!(parsed.return_url, Some("tg%3A%2F%2Fresolve".to_string()));

    let ret_success = link_parser.build_return_url(true).unwrap();
    assert_eq!(ret_success, "tg%3A%2F%2Fresolve?status=success");
    let ret_cancel = link_parser.build_return_url(false).unwrap();
    assert_eq!(ret_cancel, "tg%3A%2F%2Fresolve?status=canceled");

    // 5. Manifest Name Length Overflow Attack
    let mut tc_coordinator = WalletTonConnect::new();
    let super_long_name = "A".repeat(256);
    let manifest_oversize = TonConnectManifest {
        url: "https://evil.dapp/manifest.json".into(),
        name: super_long_name,
        icon_url: "https://evil.dapp/icon.png".into(),
        terms_of_use_url: None,
        privacy_policy_url: None,
    };
    tc_coordinator.cache_manifest(manifest_oversize);
    let cached = tc_coordinator.cached_manifests.get("https://evil.dapp/manifest.json").unwrap().clone();
    assert_eq!(cached.name.len(), APP_NAME_LIMIT, "Manifest name must be clamped to APP_NAME_LIMIT (64)");

    // 6. Session Connection with Empty Session Public Key Attack
    let empty_pk_err = tc_coordinator.connect_session(
        "sess_test_1",
        "", // Empty public key
        cached.clone(),
        1000,
    );
    assert!(empty_pk_err.is_err(), "Empty DApp public key must be rejected");

    // Valid session connection
    let session = tc_coordinator.connect_session(
        "sess_test_1",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        cached.clone(),
        1000,
    ).expect("Valid session connects");
    assert!(session.is_active);
    assert_eq!(tc_coordinator.count_active(), 1);

    // Disconnect session
    assert!(tc_coordinator.disconnect_session("sess_test_1"));
    assert_eq!(tc_coordinator.count_active(), 0);
    assert!(!tc_coordinator.disconnect_session("non_existent"));
}

#[test]
fn test_adversarial_ton_connect_login_proof_and_replay_defense() {
    let mut login = WalletTonConnectLogin::new();

    // 1. Proof Domain Injection / Malformation
    let empty_domain_proof = login.create_proof_payload("", "payload", 1000, "sig_hex_123");
    assert!(login.verify_proof(&empty_domain_proof, 1050).is_err());

    // Domain length spoofing attack (domain_len claims 10 but domain is "abc" of length 3)
    let mut spoofed_domain_proof = login.create_proof_payload("abc", "payload", 1000, "sig_hex_123");
    spoofed_domain_proof.domain_len = 10;
    let err = login.verify_proof(&spoofed_domain_proof, 1050);
    assert!(err.unwrap_err().to_string().contains("Proof domain length mismatch"));

    // 2. Replay Attack via Stale Timestamps (> 300s)
    let proof_base = login.create_proof_payload("fragment.com", "nonce_abc", 1000, "sig_hex_123");
    // Boundary check: exactly 300s -> should succeed
    assert!(login.verify_proof(&proof_base, 1000 + TON_PROOF_MAX_AGE_SECONDS).is_ok());
    // 301s -> must be rejected as expired replay attack
    let expired_err = login.verify_proof(&proof_base, 1000 + TON_PROOF_MAX_AGE_SECONDS + 1);
    assert!(expired_err.unwrap_err().to_string().contains("Proof timestamp expired"));

    // 3. Future Timestamp Attack (clock desync / future timestamp spoofing)
    let future_err = login.verify_proof(&proof_base, 900);
    assert!(future_err.unwrap_err().to_string().contains("Proof timestamp in the future"));

    // 4. Missing Signature Attack
    let mut no_sig_proof = proof_base.clone();
    no_sig_proof.signature_hex = "".to_string();
    let sig_err = login.verify_proof(&no_sig_proof, 1050);
    assert!(sig_err.unwrap_err().to_string().contains("Proof signature is missing"));

    // 5. Transaction RPC Request Validation Attacks
    let mut req_mgr = WalletTonConnectRequest::new();
    let valid_until = 2000;

    // Expired transaction request
    let expired_tx = SendTransactionRpcParams {
        valid_until,
        from: None,
        messages: vec![TonConnectMessage {
            address: "EQCGScrZe1xsobBeuqbpR9TLbk0mQmtZP29liBpxmmVUjWPB".into(),
            amount_nanos_str: "50000000".into(),
            payload_b64: None,
            state_init_b64: None,
        }],
    };
    let enqueue_expired = req_mgr.enqueue_send_transaction("req_expired", expired_tx, 2001);
    assert!(enqueue_expired.unwrap_err().to_string().contains("Transaction request has already expired"));

    // Empty messages attack
    let empty_msg_tx = SendTransactionRpcParams {
        valid_until: 3000,
        from: None,
        messages: Vec::new(),
    };
    let enqueue_empty = req_mgr.enqueue_send_transaction("req_empty", empty_msg_tx, 2000);
    assert!(enqueue_empty.unwrap_err().to_string().contains("Transaction messages list cannot be empty"));

    // Empty recipient address attack
    let empty_addr_tx = SendTransactionRpcParams {
        valid_until: 3000,
        from: None,
        messages: vec![TonConnectMessage {
            address: "".into(),
            amount_nanos_str: "10000000".into(),
            payload_b64: None,
            state_init_b64: None,
        }],
    };
    let enqueue_empty_addr = req_mgr.enqueue_send_transaction("req_empty_addr", empty_addr_tx, 2000);
    assert!(enqueue_empty_addr.unwrap_err().to_string().contains("Destination address cannot be empty"));

    // Valid multi-message transaction and pending amount summation
    let valid_multi_tx = SendTransactionRpcParams {
        valid_until: 4000,
        from: None,
        messages: vec![
            TonConnectMessage {
                address: "EQ_RECIPIENT_1".into(),
                amount_nanos_str: "250000000".into(), // 0.25 TON
                payload_b64: None,
                state_init_b64: None,
            },
            TonConnectMessage {
                address: "EQ_RECIPIENT_2".into(),
                amount_nanos_str: "750000000".into(), // 0.75 TON
                payload_b64: None,
                state_init_b64: None,
            },
        ],
    };
    assert!(req_mgr.enqueue_send_transaction("req_multi", valid_multi_tx, 2000).is_ok());
    assert_eq!(req_mgr.total_pending_amount_nanos(), 1_000_000_000); // 1.0 TON total
    assert!(req_mgr.pop_next().is_some());
    assert_eq!(req_mgr.total_pending_amount_nanos(), 0);
}

#[test]
fn test_adversarial_telegram_passport_encryption_and_integrity() {
    // 1. Modulo-255 Checksum Invariant: (sum(bytes) % 255) == 239
    let secret = PassportEncryption::generate_secret_bytes();
    assert_eq!(secret.len(), SECRET_SIZE);
    assert!(PassportEncryption::check_bytes_mod_255(&secret));
    assert!(PassportEncryption::check_secret_bytes(&secret));

    // Boundary arrays
    assert!(!PassportEncryption::check_bytes_mod_255(&[0u8; 32])); // sum = 0 % 255 = 0 != 239
    assert!(!PassportEncryption::check_bytes_mod_255(&[1u8; 32])); // sum = 32 % 255 = 32 != 239
    assert!(!PassportEncryption::check_secret_bytes(&[239u8; 1]));  // Correct sum mod 255, but wrong length (1 != 32)

    // Bit-flip sensitivity: altering any byte in a valid secret breaks the modulo invariant
    for i in 0..secret.len() {
        let mut tampered_secret = secret.clone();
        tampered_secret[i] ^= 0x01; // flip 1 bit
        assert!(
            !PassportEncryption::check_bytes_mod_255(&tampered_secret),
            "Flipping bit in byte {i} must break modulo-255 check"
        );
    }

    // 2. Secret ID Derivation Invariant
    let secret_id = PassportEncryption::count_secure_secret_id(&secret);
    let sha = sha256(&secret);
    let mut expected_id_bytes = [0u8; 8];
    expected_id_bytes.copy_from_slice(&sha[..8]);
    assert_eq!(secret_id, u64::from_le_bytes(expected_id_bytes));
    assert_eq!(secret_id, PassportEncryption::count_secure_secret_id(&secret)); // Deterministic

    // 3. End-to-End Encryption & Decryption Roundtrip with Padding
    let test_payloads: &[&[u8]] = &[
        b"", // Empty payload
        b"1", // 1 byte
        b"123456789012345", // 15 bytes
        b"1234567890123456", // 16 bytes (block boundary)
        b"Telegram Passport End-to-End Client Cryptography Verification Payload with sensitive identity records",
    ];

    for &payload in test_payloads {
        let encrypted = PassportEncryption::encrypt_data(payload, Some(&secret))
            .expect("Encryption with valid secret succeeds");

        assert_eq!(encrypted.secret, secret);
        assert_eq!(encrypted.hash.len(), 32);
        assert_eq!(encrypted.bytes.len() % ALIGN_TO, 0, "Ciphertext must be aligned to 16 bytes");
        assert!(encrypted.bytes.len() >= MIN_PADDING + payload.len());

        let decrypted = PassportEncryption::decrypt_data(&encrypted.bytes, &encrypted.hash, &encrypted.secret)
            .expect("Decryption succeeds");
        assert_eq!(decrypted, payload, "Decrypted data must match original plaintext");

        // 4. Adversarial Attack: Ciphertext Tampering
        if !encrypted.bytes.is_empty() {
            let mut tampered_ct = encrypted.bytes.clone();
            tampered_ct[0] ^= 0xFF; // Invert first byte
            let decrypt_tampered = PassportEncryption::decrypt_data(&tampered_ct, &encrypted.hash, &encrypted.secret);
            assert!(decrypt_tampered.is_err(), "Tampered ciphertext must fail decryption or integrity check");

            // Truncated block size attack (length not divisible by 16)
            let mut truncated_ct = encrypted.bytes.clone();
            truncated_ct.pop();
            let decrypt_trunc = PassportEncryption::decrypt_data(&truncated_ct, &encrypted.hash, &encrypted.secret);
            assert!(decrypt_trunc.is_err(), "Non-aligned ciphertext length must fail immediately");
        }

        // 5. Adversarial Attack: Data Hash Tampering
        let mut tampered_hash = encrypted.hash.clone();
        tampered_hash[0] ^= 0x01;
        let decrypt_bad_hash = PassportEncryption::decrypt_data(&encrypted.bytes, &tampered_hash, &encrypted.secret);
        assert!(decrypt_bad_hash.is_err(), "Decryption with tampered hash must fail data hash check");

        let bad_len_hash = vec![0x42; 31]; // 31 bytes instead of 32
        assert!(PassportEncryption::decrypt_data(&encrypted.bytes, &bad_len_hash, &encrypted.secret).is_err());

        // 6. Adversarial Attack: Wrong Secret Key
        let another_secret = PassportEncryption::generate_secret_bytes();
        let decrypt_wrong_secret = PassportEncryption::decrypt_data(&encrypted.bytes, &encrypted.hash, &another_secret);
        assert!(decrypt_wrong_secret.is_err(), "Decryption with wrong secret must fail");

        let bad_len_secret = vec![0x55; 31];
        assert!(PassportEncryption::decrypt_data(&encrypted.bytes, &encrypted.hash, &bad_len_secret).is_err());
    }

    // 7. Cloud Password and Master Secret Key Encapsulation
    let password_hash_64 = sha512(b"CloudTwoFactorPasswordSaltedHash123456");
    let enc_secret = PassportEncryption::encrypt_secure_secret(&secret, &password_hash_64)
        .expect("Encrypt secure secret");
    let dec_secret = PassportEncryption::decrypt_secure_secret(&enc_secret, &password_hash_64)
        .expect("Decrypt secure secret");
    assert_eq!(dec_secret, secret);

    // Corrupted password hash must fail decryption
    let bad_pwd_hash_64 = sha512(b"WrongPasswordHash");
    let dec_bad_pwd = PassportEncryption::decrypt_secure_secret(&enc_secret, &bad_pwd_hash_64);
    assert!(dec_bad_pwd.is_err(), "Decryption with wrong password hash must fail mod-255 check");

    // 8. Value Secret Encryption with Value Hash
    let value_secret = PassportEncryption::generate_secret_bytes();
    let value_hash = sha256(b"PassportDocumentRecordHash");
    let enc_val_sec = PassportEncryption::encrypt_value_secret(&value_secret, &secret, &value_hash)
        .expect("Encrypt value secret");
    let dec_val_sec = PassportEncryption::decrypt_value_secret(&enc_val_sec, &secret, &value_hash)
        .expect("Decrypt value secret");
    assert_eq!(dec_val_sec, value_secret);

    // Mismatched value hash
    let altered_val_hash = sha256(b"DifferentDocumentRecordHash");
    let dec_mismatch = PassportEncryption::decrypt_value_secret(&enc_val_sec, &secret, &altered_val_hash)
        .expect("AES-CBC decrypts without padding check");
    assert_ne!(dec_mismatch, value_secret, "Decryption with wrong value hash must not recover value secret");

    // 9. Structured Data Serialization Roundtrip
    let mut doc_fields = BTreeMap::new();
    doc_fields.insert("first_name".to_string(), "Pavel".to_string());
    doc_fields.insert("last_name".to_string(), "Durov".to_string());
    doc_fields.insert("document_no".to_string(), "AB1234567".to_string());
    let serialized = PassportEncryption::serialize_data(&doc_fields).expect("Serialize");
    let deserialized = PassportEncryption::deserialize_data(&serialized).expect("Deserialize");
    assert_eq!(deserialized, doc_fields);
}
