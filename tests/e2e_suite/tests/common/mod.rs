use futures::{SinkExt, StreamExt};
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
pub use gpui_telegram_test_harness::temp_dir::TempTestDir;
#[allow(unused_imports)]
pub use gpui_telegram_test_harness::tier4_e2e::{E2eSystemDriver, EphemeralTestServer};
use reqwest::Method;
use serde_json::json;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use uuid::Uuid;

pub mod mock_agents;
pub mod mock_mcp;
pub mod mock_network;

#[allow(unused_imports)]
pub use mock_agents::*;
#[allow(unused_imports)]
pub use mock_mcp::*;
#[allow(unused_imports)]
pub use mock_network::*;

/// Allocates an isolated RAII temporary test directory with the required prefix `gpui_tg_e2e_suite_`.
pub fn new_test_dir(prefix: &str) -> TempTestDir {
    let sanitized = format!("e2e_suite_{prefix}_{}", Uuid::new_v4().simple());
    TempTestDir::new(&sanitized).expect("allocate temp test directory")
}

/// Boots an ephemeral full-featured Telegram backend server on `127.0.0.1:0`.
pub async fn boot_test_server() -> anyhow::Result<EphemeralTestServer> {
    EphemeralTestServer::start().await
}

/// Boots an ephemeral server, registers two users (`alice` and `bob`), creates a group chat,
/// and adds `bob` as a member. Returns `(server, driver, (alice_id, alice_tok), (bob_id, bob_tok), chat_id)`.
pub async fn boot_and_seed_two_users() -> anyhow::Result<(
    EphemeralTestServer,
    E2eSystemDriver,
    (u64, String),
    (u64, String),
    i64,
)> {
    let server = boot_test_server().await?;
    let driver = E2eSystemDriver::new(server.base_url());
    let alice = driver.register_user("alice", "pass1234").await?;
    let bob = driver.register_user("bob", "pass1234").await?;

    let (st, chat_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/chats",
            Some(&alice.1),
            Some(json!({
                "title": "E2E Seeded Group",
                "chat_type": "group",
                "member_ids": [bob.0]
            })),
        )
        .await?;
    anyhow::ensure!(st == 201 || st == 200, "create seeded chat failed {st}: {chat_val}");
    let chat_id = chat_val["chat_id"]
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("missing chat_id: {chat_val}"))?;
    Ok((server, driver, alice, bob, chat_id))
}

/// Connects an authenticated WebSocket client to `/api/v1/ws?token=...`.
pub async fn connect_ws(
    server: &EphemeralTestServer,
    token: &str,
) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let (ws, _) = connect_async(server.ws_url(token)).await?;
    Ok(ws)
}

/// Sends a `WsClientFrame` serialized as JSON text over the WebSocket.
#[allow(dead_code)]
pub async fn send_ws_frame(
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    frame: &WsClientFrame,
) -> anyhow::Result<()> {
    let txt = serde_json::to_string(frame)?;
    ws.send(Message::Text(txt.into())).await?;
    Ok(())
}

/// Reads WebSocket frames until `pred` matches or `timeout` elapses.
#[allow(dead_code)]
pub async fn recv_ws_frame_matching<F>(
    ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    timeout: Duration,
    mut pred: F,
) -> anyhow::Result<WsServerFrame>
where
    F: FnMut(&WsServerFrame) -> bool,
{
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let now = tokio::time::Instant::now();
        if now >= deadline {
            anyhow::bail!("Timed out waiting for matching WsServerFrame");
        }
        let rem = deadline - now;
        match tokio::time::timeout(rem, ws.next()).await {
            Ok(Some(Ok(Message::Text(txt)))) => {
                if let Ok(frame) = serde_json::from_str::<WsServerFrame>(&txt) {
                    if pred(&frame) {
                        return Ok(frame);
                    }
                }
            }
            Ok(Some(Ok(Message::Ping(p)))) => {
                let _ = ws.send(Message::Pong(p)).await;
            }
            Ok(Some(Ok(Message::Close(_)))) | Ok(None) => {
                anyhow::bail!("WebSocket closed before matching frame arrived");
            }
            Ok(Some(Err(e))) => anyhow::bail!("WebSocket read error: {e}"),
            Ok(Some(Ok(_))) => {}
            Err(_) => anyhow::bail!("Timed out waiting for matching WsServerFrame"),
        }
    }
}

/// Helper function generating a valid 1x1 PNG image for media and viewport testing.
#[allow(dead_code)]
pub fn create_test_png(r: u8, g: u8, b: u8, a: u8) -> Vec<u8> {
    let mut img = image::RgbaImage::new(1, 1);
    img.put_pixel(0, 0, image::Rgba([r, g, b, a]));
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .expect("encode test png");
    out.into_inner()
}
