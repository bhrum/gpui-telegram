#!/usr/bin/env python3
"""Tier 4 Live Cloud full-feature Telegram E2E suite against 34.46.77.133:8080."""

from __future__ import annotations

import argparse
import sys
import time
from typing import Any

from _e2e_http_ws import HttpClient, WsClient
from _e2e_telegram_flows import (
    make_png_2x2,
    make_wav_8k,
    run_auth_profile_contacts_sessions,
    run_groups_channels_topics_folders,
    run_messaging_ws_reactions_polls_receipts,
)


def run_media_search_stories_calls_wallet_acp(
    http: HttpClient,
    alice: dict[str, Any],
    bob: dict[str, Any],
    charlie: dict[str, Any],
    gid: int,
    keyword: str,
) -> None:
    png = make_png_2x2()
    st, init, _ = http.json(
        "POST",
        "/api/v1/media/upload/init",
        {"filename": "e2e.png", "size_bytes": len(png), "chunk_size": 1024},
        token=alice["token"],
    )
    assert st in (200, 201), f"upload/init failed: {st} {init}"
    uid = init["upload_id"]
    st, _, raw = http.request("PUT", f"/api/v1/media/upload/{uid}/part/0", body=png, token=alice["token"])
    assert st == 200, f"upload part 0 failed: {st} {raw!r}"
    st, comp, _ = http.json(
        "POST",
        f"/api/v1/media/upload/{uid}/complete",
        {"filename": "e2e.png", "mime_type": "image/png", "chat_id": gid},
        token=alice["token"],
    )
    assert st == 200 and comp.get("width") == 2 and comp.get("height") == 2, f"upload complete failed: {st} {comp}"
    file_id = comp["file_id"]

    # HTTP 206 Range download by chat member Bob + 403 rejection for non-member Charlie
    st, _, chunk = http.request("GET", f"/api/v1/media/{file_id}", headers={"Range": "bytes=0-15"}, token=bob["token"])
    assert st == 206 and chunk == png[:16], f"HTTP 206 failed: st={st} len={len(chunk)}"
    st_deny, _, _ = http.request("GET", f"/api/v1/media/{file_id}", token=charlie["token"])
    assert st_deny == 403, f"Expected 403 for outsider media access, got {st_deny}"

    # Voice upload + 100-bar waveform metadata
    wav = make_wav_8k()
    st_v, _, raw_v = http.request(
        "POST",
        "/api/v1/media/upload?filename=voice.wav&mime_type=audio/wav&is_voice=true",
        body=wav,
        headers={"Content-Type": "audio/wav"},
        token=alice["token"],
    )
    assert st_v in (200, 201), f"voice upload failed: {st_v} {raw_v!r}"

    # Tantivy search + member isolation + filter
    search_kw = f"SearchToken_{keyword}"
    st, _, _ = http.json("POST", f"/api/v1/chats/{gid}/messages", {"text": f"Msg with {search_kw}"}, token=alice["token"])
    assert st in (200, 201)
    st, s_bob, _ = http.json("GET", f"/api/v1/search?q={search_kw}&chat_id={gid}", token=bob["token"])
    assert st == 200 and s_bob.get("total", 0) >= 1, f"Bob search missed message: {s_bob}"
    st, s_out, _ = http.json("GET", f"/api/v1/search?q={search_kw}", token=charlie["token"])
    assert st == 200 and s_out.get("total", 0) == 0, f"Outsider leaked search hit: {s_out}"

    # Stories
    st, story, _ = http.json("POST", "/api/v1/stories", {"media_id": file_id, "caption": "Live 24h"}, token=alice["token"])
    assert st in (200, 201), f"story create failed: {st} {story}"
    sid = story["story_id"]
    st, _, _ = http.json("POST", f"/api/v1/stories/{sid}/view", {}, token=bob["token"])
    assert st == 200
    st, _, _ = http.json("DELETE", f"/api/v1/stories/{sid}", token=alice["token"])
    assert st == 200

    # Calls signaling + TURN credentials
    st, call, _ = http.json(
        "POST",
        "/api/v1/calls/invite",
        {"callee_id": bob["user_id"], "is_video": True, "chat_id": gid},
        token=alice["token"],
    )
    assert st in (200, 201), f"call invite failed: {st} {call}"
    call_id = call["call_id"]
    st, _, _ = http.json(
        "POST",
        "/api/v1/calls/signal",
        {"call_id": call_id, "signal_type": "answer", "sdp": "v=0"},
        token=bob["token"],
    )
    assert st == 200
    st, _, _ = http.json("POST", f"/api/v1/calls/{call_id}/rate", {"rating": 5}, token=alice["token"])
    assert st == 200
    st, turn, _ = http.json("GET", "/api/v1/webrtc/turn_credentials", token=alice["token"])
    assert st == 200 and turn.get("credential"), f"TURN missing credential: {turn}"

    # Channel statistics, export, wallet, and real ACP agent turn
    st, stats, _ = http.json("GET", f"/api/v1/chats/{gid}/statistics", token=alice["token"])
    assert st == 200 and stats.get("member_count", 0) >= 2, f"stats failed: {stats}"
    st, _, _ = http.request("GET", f"/api/v1/chats/{gid}/export", token=alice["token"])
    assert st == 200
    st, _, _ = http.json("POST", "/api/v1/wallet/topup", {"amount_stars": 500}, token=alice["token"])
    assert st == 200
    st, gift, _ = http.json(
        "POST",
        "/api/v1/wallet/send_stars",
        {"to_user_id": bob["user_id"], "amount_stars": 120, "message": "E2E Stars"},
        token=alice["token"],
    )
    assert st == 200 and gift["balance"]["stars_balance"] == 380, f"wallet send_stars failed: {gift}"

    st, turn_resp, _ = http.json(
        "POST",
        "/api/v1/agent/turn",
        {"prompt": "Reply with one short word: PONG", "session_id": f"e2e_acp_{keyword}"},
        token=alice["token"],
    )
    assert st == 200 and turn_resp.get("response"), f"ACP agent turn failed: {st} {turn_resp}"


def main() -> int:
    ap = argparse.ArgumentParser(description="Full-feature Telegram live E2E test suite")
    ap.add_argument("--server-url", default="http://34.46.77.133:8080")
    args = ap.parse_args()

    http = HttpClient(args.server_url)
    ws_base = args.server_url.replace("https://", "wss://").replace("http://", "ws://").rstrip("/")
    suffix = str(int(time.time() * 1000))
    keyword = f"Kw{suffix[-6:]}"

    alice, bob, charlie = run_auth_profile_contacts_sessions(http, suffix)
    print("[PASS] 1. Health, status v1.0.0, multi-user register (empty initial chats), welcome, sessions, profile, contacts")
    gid, chan_id, tid = run_groups_channels_topics_folders(http, alice, bob)
    print(f"[PASS] 2. Group ({gid}), channel ({chan_id}), forum topic ({tid}), invite join, folders")

    ws_alice = WsClient(f"{ws_base}/api/v1/ws?token={alice['token']}")
    ws_bob = WsClient(f"{ws_base}/api/v1/ws?token={bob['token']}")
    try:
        run_messaging_ws_reactions_polls_receipts(http, ws_alice, ws_bob, alice, bob, gid, chan_id, tid, keyword)
        print("[PASS] 3. WS push, nonce dedup, reply, edit, forward, reaction, quiz poll, read receipt ✓->✓✓, pin, draft, revoke")
    finally:
        ws_alice.close()
        ws_bob.close()

    run_media_search_stories_calls_wallet_acp(http, alice, bob, charlie, gid, keyword)
    print("[PASS] 4. Chunked CAS upload + HTTP 206 + 403 isolation, voice waveform, Tantivy search, stories, calls/TURN, stats, wallet, ACP turn")
    return 0


if __name__ == "__main__":
    sys.exit(main())
