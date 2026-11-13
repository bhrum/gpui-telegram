//! Subsystem Unit Tests for Module 13 Wallet Part 5
//! Covers all 20 targets across `gpui_telegram_agent` and `gpui_telegram_desktop`

use gpui_telegram_agent::wallet::wallet_session::*;
use gpui_telegram_agent::wallet::wallet_session_deposit::*;
use gpui_telegram_agent::wallet::wallet_session_gift_moves::*;
use gpui_telegram_agent::wallet::wallet_session_waits::*;
use gpui_telegram_agent::wallet::wallet_stream::*;
use gpui_telegram_agent::wallet::wallet_ton_connect::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_box::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_claims::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_emulation::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_link::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_login::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_request::*;
use gpui_telegram_agent::wallet::wallet_ton_connect_request_box::*;
use gpui_telegram_agent::wallet::wallet_transfer_messages::*;
use gpui_telegram_agent::wallet::wallet_transfer_store::*;
use gpui_telegram_agent::wallet::wallet_unlock::*;
use gpui_telegram_agent::wallet::wallet_user_addresses::*;
use gpui_telegram_agent::wallet::wallet_vault::*;
use gpui_telegram_agent::wallet::wallet_window_transition::*;
use gpui_telegram_desktop::theme::wallet::WalletStyleSheet;

#[test]
fn test_wallet_session_lifecycle() {
    let mut session = WalletSession::with_address("EQCGScrZe1xsobBeuqbpR9TLbk0mQmtZP29liBpxmmVUjWPB");
    assert_eq!(session.balance(), 0);
    assert!(session.is_active());

    session.set_balance(500_000_000); // 0.5 TON
    assert_eq!(session.available_balance(), 500_000_000);

    // Minimum transfer bound check (0.1 TON = 100_000_000)
    assert!(!session.can_transfer_amount(50_000_000));
    assert!(session.can_transfer_amount(150_000_000));
    assert!(!session.can_transfer_amount(600_000_000));

    assert_eq!(session.load_more_collectibles(5).unwrap(), 5);
    assert_eq!(session.collectibles_count, 5);
}

#[test]
fn test_wallet_session_deposit_and_gift_moves() {
    let mut deposit = WalletSessionDeposit::new();
    let intent = GiftDepositIntent {
        gift_id: "gift_42".into(),
        recipient_address: "EQ_WALLET_ADDR".into(),
        amount_nanos: 200_000_000,
        payload_boc: vec![0x01, 0x02],
        created_at_ms: 1000,
    };
    assert_eq!(deposit.submit_intent(intent).unwrap(), 1);
    assert_eq!(deposit.pending_count(), 1);

    let confirmed = deposit.confirm_deposit("gift_42");
    assert!(confirmed.is_some());
    assert_eq!(deposit.total_deposited_nanos, 200_000_000);
    assert_eq!(deposit.pending_count(), 0);

    let mut gift_moves = WalletSessionGiftMoves::new();
    let mv_id = gift_moves.enqueue_move("nft_item_1", "EQ_DEST", 2000);
    assert!(gift_moves.mark_status(&mv_id, GiftMoveStatus::Confirmed));
    assert_eq!(gift_moves.completed_count, 1);
    assert_eq!(gift_moves.prune_confirmed(), 1);
}

#[test]
fn test_wallet_session_waits_and_stream() {
    let mut waits = WalletSessionWaits::new();
    waits.add_wait("EQ_NFT_1", "owner_1", 1000);
    assert!(waits.is_waiting("EQ_NFT_1"));

    // Timeout is 60_000ms
    assert_eq!(waits.prune_expired(1000 + 30000), 0);
    assert_eq!(waits.prune_expired(1000 + 70000), 1);
    assert!(!waits.is_waiting("EQ_NFT_1"));

    let mut stream = WalletStream::with_endpoint("wss://ton-stream.org/ws");
    assert_eq!(stream.state, StreamConnectionState::Connecting);
    stream.on_connected();
    assert_eq!(stream.state, StreamConnectionState::Connected);

    stream.push_event(StreamEvent::BalanceUpdate {
        address: "EQ_ADDR".into(),
        new_balance_nanos: 1_000_000_000,
    }, 5000);
    assert_eq!(stream.drain_events().len(), 1);
}

#[test]
fn test_ton_connect_coordinator_and_link() {
    let mut tc = WalletTonConnect::new();
    let manifest = TonConnectManifest {
        url: "https://fragment.com/tonconnect-manifest.json".into(),
        name: "Fragment".into(),
        icon_url: "https://fragment.com/icon.png".into(),
        terms_of_use_url: None,
        privacy_policy_url: None,
    };
    tc.cache_manifest(manifest.clone());

    let session = tc.connect_session("sess_1", "pubkey_hex_32bytes", manifest, 1000).unwrap();
    assert_eq!(session.session_id, "sess_1");
    assert_eq!(tc.count_active(), 1);

    let mut link_parser = WalletTonConnectLink::new();
    let raw_link = "tc://?v=2&id=abcd1234ef56&r=https%3A%2F%2Fapp.org%2Fmanifest.json&ret=tg%3A%2F%2Fresolve";
    let parsed = link_parser.parse(raw_link).unwrap();
    assert_eq!(parsed.protocol_version, 2);
    assert_eq!(parsed.client_session_id, "abcd1234ef56");
    assert!(link_parser.build_return_url(true).unwrap().contains("status=success"));
}
#[test]
fn test_ton_connect_claims_and_emulation() {
    let mut claims = WalletTonConnectClaims::new();
    claims.grant_claim("https://fragment.com", 0b0011, 1000, Some(3600_000));
    assert!(claims.has_permission("https://fragment.com", 0b0001, 2000));
    assert!(!claims.has_permission("https://fragment.com", 0b0100, 2000));
    assert!(!claims.has_permission("https://fragment.com", 0b0001, 1000 + 4000_000));

    let mut emu = WalletTonConnectEmulation::new();
    let actions = vec![
        EmulationAction {
            destination_address: "EQ_DEST_1".into(),
            amount_nanos: 100_000_000,
            gas_fee_nanos: 15_000_000,
            payload_type: "transfer".into(),
        },
    ];
    let summary = emu.simulate_actions(actions, 200_000_000);
    assert!(summary.is_success);
    assert_eq!(summary.total_amount_nanos, 100_000_000);
    assert_eq!(summary.total_fee_nanos, 15_000_000);
    assert!(emu.can_afford(200_000_000));
    assert!(!emu.can_afford(100_000_000));
}

#[test]
fn test_ton_connect_login_and_request_box() {
    let mut login = WalletTonConnectLogin::new();
    let proof = login.create_proof_payload("fragment.com", "random_nonce_123", 1000, "sig_hex_abcdef");
    assert!(login.verify_proof(&proof, 1050).unwrap());
    assert!(!login.verify_proof(&proof, 1500).is_ok()); // > 300s expired

    let mut box_ui = WalletTonConnectBox::with_manifest(TonConnectManifest {
        url: "https://test.app/manifest.json".into(),
        name: "TestApp".into(),
        icon_url: "".into(),
        terms_of_use_url: None,
        privacy_policy_url: None,
    });
    assert_eq!(box_ui.effective_kind(), TonConnectBoxPhase::Confirm);
    box_ui.set_success();
    assert_eq!(box_ui.phase, TonConnectBoxPhase::Success);

    let mut req_mgr = WalletTonConnectRequest::new();
    let rpc_params = SendTransactionRpcParams {
        valid_until: 2000,
        from: None,
        messages: vec![TonConnectMessage {
            address: "EQ_DEST".into(),
            amount_nanos_str: "100000000".into(),
            payload_b64: None,
            state_init_b64: None,
        }],
    };
    assert!(req_mgr.enqueue_send_transaction("req_1", rpc_params, 1000).is_ok());
    assert_eq!(req_mgr.total_pending_amount_nanos(), 100000000);

    let box_model = WalletTonConnectRequestBox::with_request(
        "req_101",
        vec![DestinationDisplay {
            recipient: "EQ_RECIPIENT".into(),
            amount_nanos: 500_000_000,
            payload_preview: None,
        }],
        15_000_000,
    );
    let cost_str = box_model.format_total_cost();
    assert!(cost_str.contains("TON"));
    assert!(cost_str.contains(MINUS_SIGN_CHAR));
}

#[test]
fn test_wallet_transfer_store_and_messages() {
    let mut store = WalletTransferStore::new();
    store.add_transfer(SubmittedTransferRecord {
        transfer_id: "tx_1".into(),
        recipient_address: "EQ_RECV".into(),
        amount_nanos: 1_000_000_000,
        comment: "Test comment".into(),
        submitted_at_ms: 1000,
        is_confirmed: false,
    });
    assert_eq!(store.pending_transfers().len(), 1);
    assert!(store.mark_confirmed("tx_1"));
    assert_eq!(store.pending_transfers().len(), 0);

    let (valid, near_limit) = WalletTransferStore::validate_comment("Hello Telegram TON");
    assert!(valid);
    assert!(!near_limit);

    let mut msgs = WalletTransferMessages::new();
    msgs.inject_transfer_message(101, 555, 1234, 5678, 2_000_000_000, Some("Gift".into()), 1700000000);
    assert_eq!(msgs.find_by_chat(555).len(), 1);
    assert!(msgs.update_status(101, TransferServiceStatus::Completed, Some("0xabc123".into())));

    let mut user_addrs = WalletUserAddresses::new();
    user_addrs.insert_resolved(12345, "0:1234567890abcdef", "EQCGScrZe1xsobBeuqbpR9TLbk0mQmtZP29liBpxmmVUjWPB", 1000);
    assert!(user_addrs.get_address(12345).is_some());
    assert_eq!(user_addrs.get_address(12345).unwrap().raw_address, "0:1234567890abcdef");
}

#[test]
fn test_wallet_vault_and_unlock() {
    let mut vault = WalletVault::new();
    let key_id = [1u8; VAULT_KEY_ID_SIZE_BYTES];
    let nonce = [2u8; VAULT_NONCE_SIZE_BYTES];
    let tag = [3u8; VAULT_TAG_SIZE_BYTES];
    vault.store_record(key_id, nonce, tag, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(vault.record_count(), 1);
    assert!(vault.find_record(&key_id).is_some());

    let mut unlock = WalletUnlock::new();
    assert!(unlock.is_locked);
    assert!(!unlock.verify_passcode("wrong", "secret_pass", 1000));
    assert!(unlock.verify_passcode("secret_pass", "secret_pass", 2000));
    assert!(!unlock.is_locked);
}

#[test]
fn test_wallet_transition_and_style() {
    let mut trans = WalletWindowTransition::new();
    trans.start_reveal();
    assert!(trans.is_animating);
    trans.tick(320); // half duration
    assert!(trans.progress > 0.4 && trans.progress < 0.6);
    trans.tick(350); // complete duration
    assert_eq!(trans.progress, 1.0);
    assert!(!trans.is_animating);

    let style = WalletStyleSheet::new();
    assert_eq!(style.card_height, 190.0);
    assert_eq!(style.card_margin, 16.0);
    assert_eq!(style.card_radius, 16.0);
    assert_eq!(WalletStyleSheet::wallet_card_height(), 190.0);
    assert_eq!(WalletStyleSheet::wallet_menu_icon(), 16.0);
}
