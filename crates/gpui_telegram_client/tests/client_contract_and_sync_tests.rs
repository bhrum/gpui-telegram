use gpui_telegram_client::{
    ClientConfig, ClientDomainEvent, LocalStore, TelegramClient,
};
use gpui_telegram_core::types::{ChatId, MessageId, PeerId, UserId};
use gpui_telegram_proto::message::{MessageContent, MessageItem, ReactionCount};
use gpui_telegram_proto::sync::{PtsUpdate, ReadHistoryEvent, ReactionEvent, UpdateEvent};
use gpui_telegram_proto::ws::WsServerFrame;
use gpui_telegram_proto::{DeleteTombstone, PollCard, PollOption};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn read_http_request(stream: &mut tokio::net::TcpStream) -> (String, String, Vec<u8>) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let mut header_end = None;
    while header_end.is_none() {
        let n = stream.read(&mut tmp).await.unwrap();
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            header_end = Some(pos + 4);
        }
    }
    let hend = header_end.unwrap_or(buf.len());
    let headers_str = String::from_utf8_lossy(&buf[..hend]).to_string();
    let content_len = headers_str
        .lines()
        .find_map(|l| {
            let lower = l.to_ascii_lowercase();
            lower
                .strip_prefix("content-length:")
                .and_then(|v| v.trim().parse::<usize>().ok())
        })
        .unwrap_or(0);
    while buf.len() < hend + content_len {
        let n = stream.read(&mut tmp).await.unwrap();
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
    }
    let first_line = headers_str.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    (method, path, buf[hend..].to_vec())
}

#[test]
fn test_chunked_put_upload_and_cas_media_cache_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = ClientConfig::default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let put_parts = Arc::new(AtomicUsize::new(0));
    let downloads = Arc::new(AtomicUsize::new(0));
    let payload: Vec<u8> = (0..(300 * 1024)).map(|i| (i % 251) as u8).collect();
    let expected = payload.clone();

    let addr = rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let put_parts_srv = put_parts.clone();
        let downloads_srv = downloads.clone();
        let bytes_srv = payload.clone();
        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else { break };
                let (method, path, _) = read_http_request(&mut stream).await;
                let (status, ctype, body) = if method == "POST" && path == "/api/v1/media/upload/init" {
                    ("200 OK", "application/json", br#"{"upload_id":"up-1","chunk_size":1048576}"#.to_vec())
                } else if method == "PUT" && path.starts_with("/api/v1/media/upload/up-1/part/") {
                    put_parts_srv.fetch_add(1, Ordering::SeqCst);
                    ("200 OK", "application/json", br#"{"ok":true}"#.to_vec())
                } else if method == "POST" && path == "/api/v1/media/upload/up-1/complete" {
                    ("200 OK", "application/json", br#"{"file_id":"cas-file-1","sha256":"abc","size_bytes":307200,"mime_type":"application/octet-stream"}"#.to_vec())
                } else if method == "GET" && path == "/api/v1/media/stream/remote-file-2" {
                    downloads_srv.fetch_add(1, Ordering::SeqCst);
                    ("200 OK", "application/octet-stream", bytes_srv.clone())
                } else {
                    ("404 Not Found", "application/json", br#"{"error":"not_found"}"#.to_vec())
                };
                let head = format!(
                    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    status,
                    ctype,
                    body.len()
                );
                let _ = stream.write_all(head.as_bytes()).await;
                let _ = stream.write_all(&body).await;
            }
        });
        addr
    });

    let mut cfg = config;
    cfg.server_url = format!("http://{}", addr);
    let client = TelegramClient::open_with_dir(cfg.clone(), dir.path()).expect("open client");
    client.set_token(Some("tok-123".to_string()));

    let uploaded = rt.block_on(client.upload_media("blob.bin", "application/octet-stream", expected.clone())).unwrap();
    assert_eq!(uploaded.file_id, "cas-file-1");
    assert_eq!(put_parts.load(Ordering::SeqCst), 1);
    assert!(client.local_media_path("cas-file-1").expect("cas path").exists());
    assert_eq!(rt.block_on(client.get_media_bytes("cas-file-1")).unwrap(), expected);
    assert_eq!(downloads.load(Ordering::SeqCst), 0);

    let first_fetch = rt.block_on(client.get_media_bytes("remote-file-2")).unwrap();
    assert_eq!(first_fetch, expected);
    assert_eq!(downloads.load(Ordering::SeqCst), 1);

    let reopened = TelegramClient::open_with_dir(cfg, dir.path()).expect("reopen client");
    reopened.set_token(Some("tok-123".to_string()));
    assert_eq!(rt.block_on(reopened.get_media_bytes("remote-file-2")).unwrap(), expected);
    assert_eq!(downloads.load(Ordering::SeqCst), 1);
}

#[test]
fn test_ws_pts_push_dispatch_and_sqlite_local_store() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = LocalStore::open_with_dir(dir.path()).expect("store");
    let client = TelegramClient::open_with_dir(ClientConfig::default(), dir.path()).expect("client");
    let chat_id = ChatId(77);

    let mut msg1 = MessageItem::new(MessageId(10), chat_id, UserId(2), PeerId::Chat(chat_id), MessageContent::plain_text("hello"))
        .with_client_nonce(9001);
    msg1.id = MessageId(10);
    let ev1 = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 1, 1, UpdateEvent::NewMessage(msg1.clone()), 1000)));
    assert!(matches!(ev1.as_slice(), [ClientDomainEvent::NewMessage(_)]));

    let mut edited = msg1.clone();
    edited.content = MessageContent::plain_text("hello edited");
    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 2, 1, UpdateEvent::EditMessage(edited), 1001)));

    let react_ev = ReactionEvent::new(chat_id, MessageId(10), UserId(3), vec![ReactionCount::new("🔥", 1, false)]);
    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 3, 1, UpdateEvent::ReactionUpdated(react_ev), 1002)));

    let gap_msg = MessageItem::new(MessageId(12), chat_id, UserId(2), PeerId::Chat(chat_id), MessageContent::plain_text("after gap"));
    let gap_evs = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 5, 1, UpdateEvent::NewMessage(gap_msg), 1005)));
    assert!(matches!(gap_evs.as_slice(), [ClientDomainEvent::SyncGapDetected { chat_id: ChatId(77), from_pts: 3 }]));
    assert_eq!(client.pending_gap_chats(), vec![ChatId(77)]);

    let miss_msg = MessageItem::new(MessageId(11), chat_id, UserId(2), PeerId::Chat(chat_id), MessageContent::plain_text("filled gap"));
    let poll = PollCard::new("501", "Vote?", vec![PollOption::new(0, "Yes"), PollOption::new(1, "No")], true);
    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 4, 1, UpdateEvent::NewMessage(miss_msg), 1004)));
    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 5, 0, UpdateEvent::PollUpdated { chat_id, message_id: MessageId(12), poll }, 1006)));
    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(chat_id, 5, 0, UpdateEvent::ReadHistory(ReadHistoryEvent::new(chat_id, MessageId(12), false, 0)), 1007)));

    let msgs = client.get_messages(chat_id).unwrap();
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[0].reactions[0].emoji, "🔥");
    assert_eq!(client.local_store.read_inbox_max_id(chat_id), Some(MessageId(12)));

    let _ = client.process_ws_frame(WsServerFrame::PtsPush(PtsUpdate::new(
        chat_id,
        6,
        1,
        UpdateEvent::DeleteMessages(DeleteTombstone::new(PeerId::Chat(chat_id), vec![MessageId(11)], true)),
        1008,
    )));
    assert_eq!(client.get_messages(chat_id).unwrap().len(), 2);
    drop(store);
    let reloaded = LocalStore::open_with_dir(dir.path()).unwrap();
    assert_eq!(reloaded.get_messages(chat_id).unwrap().len(), 2);
}
