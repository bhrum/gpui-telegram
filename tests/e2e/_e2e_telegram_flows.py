"""Modular authentic Telegram E2E flow assertions (HTTP + RFC 6455 WS)."""

from __future__ import annotations

import struct
import time
import zlib
from typing import Any

from _e2e_http_ws import HttpClient, WsClient


def make_png_2x2() -> bytes:
    def chunk(tag: bytes, data: bytes) -> bytes:
        crc = zlib.crc32(tag + data) & 0xFFFFFFFF
        return struct.pack("!I", len(data)) + tag + data + struct.pack("!I", crc)

    ihdr = struct.pack("!IIBBBBB", 2, 2, 8, 6, 0, 0, 0)
    raw = b"".join(b"\x00" + b"\x2a\xab\xee\xff" * 2 for _ in range(2))
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b"")


def make_wav_8k() -> bytes:
    sr, n = 8000, 1600
    pcm = b"".join(struct.pack("<h", int(12000 * ((i % 40) / 20.0 - 1.0))) for i in range(n))
    hdr = struct.pack(
        "<4sI4s4sIHHIIHH4sI",
        b"RIFF",
        36 + len(pcm),
        b"WAVE",
        b"fmt ",
        16,
        1,
        1,
        sr,
        sr * 2,
        2,
        16,
        b"data",
        len(pcm),
    )
    return hdr + pcm


def run_auth_profile_contacts_sessions(http: HttpClient, suffix: str) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    st, health, _ = http.json("GET", "/health")
    assert st == 200 and health, f"/health failed: {st} {health}"
    st, status, _ = http.json("GET", "/api/v1/status")
    assert st == 200, f"/api/v1/status failed: {st} {status}"
    assert status.get("version") == "1.0.0", f"Expected version 1.0.0, got {status}"
    caps = status.get("capabilities", [])
    assert "phone_otp" in caps and "polls" in caps, f"Missing capabilities: {caps}"

    users: list[dict[str, Any]] = []
    for role in ("alice", "bob", "charlie"):
        uname = f"e2e_{role}_{suffix}"
        pw = "StrongE2ePass!9"
        st, reg, _ = http.json(
            "POST",
            "/api/v1/auth/register",
            {"username": uname, "password": pw, "first_name": role.title(), "device_name": f"{role}-mac"},
        )
        assert st in (200, 201), f"register {uname} failed: {st} {reg}"
        st, chats0, _ = http.json("GET", "/api/v1/chats", token=reg["token"])
        assert st == 200 and chats0.get("total") == 0 and chats0.get("chats") == [], (
            f"Expected empty initial chats on register for {uname}, got: {chats0}"
        )
        users.append({"user_id": reg["user_id"], "username": uname, "token": reg["token"], "password": pw})

    alice, bob, charlie = users
    st, w_resp, _ = http.json("POST", "/api/v1/chats/welcome", {}, token=alice["token"])
    assert st in (200, 201) and w_resp.get("chat_id"), f"welcome opt-in failed: {st} {w_resp}"

    st, login_bob, _ = http.json(
        "POST",
        "/api/v1/auth/login",
        {"username": bob["username"], "password": bob["password"], "device_name": "bob-ipad"},
    )
    assert st == 200 and login_bob.get("token"), f"bob login failed: {st} {login_bob}"
    st, sess, _ = http.json("GET", "/api/v1/auth/sessions", token=bob["token"])
    assert st == 200 and len(sess.get("sessions", [])) >= 2, f"Expected >=2 sessions: {sess}"

    st, prof, _ = http.json(
        "PUT",
        "/api/v1/users/me",
        {"first_name": "Alice", "last_name": "E2E", "bio": "GPUI verifier", "emoji_status": "rocket"},
        token=alice["token"],
    )
    assert st == 200 and prof.get("bio") == "GPUI verifier", f"profile update failed: {st} {prof}"
    st, resolved, _ = http.json("GET", f"/api/v1/peers/resolve?username=@{alice['username']}", token=bob["token"])
    assert st == 200 and resolved["user"]["user_id"] == alice["user_id"], f"peer resolve failed: {resolved}"

    st, _ = http.json(
        "POST",
        "/api/v1/contacts",
        {"username": bob["username"], "first_name": "BobContact"},
        token=alice["token"],
    )[:2]
    assert st in (200, 201), f"add contact failed: {st}"
    st, cl, _ = http.json("GET", "/api/v1/contacts", token=alice["token"])
    assert st == 200 and any(c["user_id"] == bob["user_id"] for c in cl.get("contacts", [])), f"contact missing: {cl}"
    st, _, _ = http.json("DELETE", f"/api/v1/contacts/{bob['user_id']}", token=alice["token"])
    assert st == 200, f"delete contact failed: {st}"
    return alice, bob, charlie


def run_groups_channels_topics_folders(http: HttpClient, alice: dict[str, Any], bob: dict[str, Any]) -> tuple[int, int, int]:
    st, grp, _ = http.json(
        "POST",
        "/api/v1/chats",
        {"title": "E2E Forum Group", "chat_type": "group", "is_forum": True},
        token=alice["token"],
    )
    assert st in (200, 201), f"create group failed: {st} {grp}"
    gid = grp["chat_id"]

    st, inv, _ = http.json("POST", f"/api/v1/chats/{gid}/invites", {"title": "E2E Invite"}, token=alice["token"])
    assert st in (200, 201), f"create invite failed: {st} {inv}"
    st, joined, _ = http.json("POST", "/api/v1/chats/join", {"code": inv["code"]}, token=bob["token"])
    assert st == 200 and joined["chat_id"] == gid, f"join invite failed: {st} {joined}"

    st, chan, _ = http.json(
        "POST",
        "/api/v1/chats",
        {"title": "E2E News Channel", "chat_type": "channel"},
        token=alice["token"],
    )
    assert st in (200, 201), f"create channel failed: {st} {chan}"
    chan_id = chan["chat_id"]

    st, topic, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/topics",
        {"title": "Engineering", "icon_color": 0x6FB9F0},
        token=alice["token"],
    )
    assert st in (200, 201) and topic.get("topic_id", 0) > 0, f"create topic failed: {st} {topic}"
    tid = topic["topic_id"]

    st, folder, _ = http.json(
        "POST",
        "/api/v1/folders",
        {"title": "E2E Work", "include_chats": [gid, chan_id], "emoticon": "briefcase"},
        token=alice["token"],
    )
    assert st in (200, 201), f"create folder failed: {st} {folder}"
    fid = folder["id"]
    st, fl, _ = http.json("GET", "/api/v1/folders", token=alice["token"])
    assert st == 200 and any(f["id"] == fid for f in fl.get("folders", [])), f"folder missing: {fl}"
    st, _, _ = http.json("DELETE", f"/api/v1/folders/{fid}", token=alice["token"])
    assert st == 200
    return gid, chan_id, tid


def run_messaging_ws_reactions_polls_receipts(
    http: HttpClient,
    ws_alice: WsClient,
    ws_bob: WsClient,
    alice: dict[str, Any],
    bob: dict[str, Any],
    gid: int,
    chan_id: int,
    tid: int,
    keyword: str,
) -> None:
    ws_alice.send_json({"type": "SubscribeChat", "payload": {"chat_id": gid}})
    ws_bob.send_json({"type": "SubscribeChat", "payload": {"chat_id": gid}})
    ws_bob.send_json({"type": "Ping", "payload": {"nonce": 777}})
    pong = ws_bob.recv_until(lambda m: m.get("type") == "Pong", timeout=8.0)
    assert pong["payload"]["nonce"] == 777, f"Unexpected pong: {pong}"

    nonce = int(time.time() * 1000) % 1_000_000_000
    st, m1, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {
            "text": f"Hello {keyword} message",
            "topic_id": tid,
            "client_nonce": nonce,
            "entities": [{"kind": "bold", "offset": 0, "length": 5}],
        },
        token=alice["token"],
    )
    assert st in (200, 201), f"send m1 failed: {st} {m1}"
    mid1 = m1["message_id"]

    # Idempotent retry
    st_dup, m1_dup, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {"text": f"Hello {keyword} message", "topic_id": tid, "client_nonce": nonce},
        token=alice["token"],
    )
    assert st_dup in (200, 201) and m1_dup["message_id"] == mid1, f"nonce dedup failed: {m1_dup}"

    # Verify WS push arrives on Bob's connection
    ws_msg = ws_bob.recv_until(lambda m: m.get("type") in ("PtsPush", "Update"), timeout=8.0)
    assert ws_msg, "Missing WS push frame for Bob"

    # Reply, edit, forward, reaction
    st, rep, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {"text": "Quoted reply", "reply_to": mid1, "quote_text": keyword},
        token=alice["token"],
    )
    assert st in (200, 201) and rep.get("message_id", 0) > mid1, f"reply failed: {st} {rep}"
    st, ed, _ = http.json("PUT", f"/api/v1/chats/{gid}/messages/{mid1}", {"text": f"Edited {keyword}"}, token=alice["token"])
    assert st == 200 and ed.get("text") == f"Edited {keyword}", f"edit failed: {st} {ed}"
    st, fwd, _ = http.json(
        "POST",
        f"/api/v1/chats/{chan_id}/messages",
        {"text": f"Fwd {keyword}", "forward_from_chat_id": gid, "forward_from_msg_id": mid1},
        token=alice["token"],
    )
    assert st in (200, 201) and fwd.get("message_id", 0) > 0, f"forward failed: {st} {fwd}"

    st, rx, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/messages/{mid1}/reactions",
        {"emoji": "🔥", "action": "toggle"},
        token=bob["token"],
    )
    assert st == 200, f"reaction failed: {st} {rx}"
    rx_val = rx.get("reactions")
    has_fire = (isinstance(rx_val, dict) and rx_val.get("🔥") == 1) or (
        isinstance(rx_val, list) and any(r.get("emoji") == "🔥" and r.get("count") == 1 for r in rx_val)
    )
    assert has_fire, f"Expected 🔥 count=1 in reactions: {rx}"

    # Quiz poll + vote + close
    st, poll_res, _ = http.json(
        "POST",
        f"/api/v1/chats/{gid}/polls",
        {
            "question": "GPUI frame sync?",
            "options": ["Polling", "Event-driven"],
            "is_anonymous": False,
            "is_quiz": True,
            "correct_option_id": 1,
            "solution": "GPUI renders cleanly on invalidation",
        },
        token=alice["token"],
    )
    assert st in (200, 201), f"create poll failed: {st} {poll_res}"
    pid = poll_res["poll"]["id"]
    poll_mid = poll_res["message_id"]
    st, voted, _ = http.json("POST", f"/api/v1/polls/{pid}/vote", {"option_ids": [1]}, token=bob["token"])
    assert st == 200 and voted["poll"]["total_voters"] == 1, f"vote failed: {st} {voted}"
    st, closed, _ = http.json("POST", f"/api/v1/polls/{pid}/close", {}, token=alice["token"])
    assert st == 200 and closed["poll"]["is_closed"] is True, f"close poll failed: {st} {closed}"

    # Single check (✓) -> Double check (✓✓) read receipt verification
    st, chat_before, _ = http.json("GET", f"/api/v1/chats/{gid}", token=alice["token"])
    assert st == 200 and chat_before.get("read_outbox_max_id", 0) < poll_mid, (
        f"Expected unread outbox before Bob reads: {chat_before}"
    )
    st, rd, _ = http.json("POST", f"/api/v1/chats/{gid}/read", {"max_id": poll_mid}, token=bob["token"])
    assert st == 200 and rd.get("unread_count") == 0, f"mark read failed: {st} {rd}"
    st, chat_after, _ = http.json("GET", f"/api/v1/chats/{gid}", token=alice["token"])
    assert st == 200 and chat_after.get("read_outbox_max_id", 0) >= poll_mid, (
        f"Expected read_outbox_max_id >= {poll_mid} after Bob reads: {chat_after}"
    )

    # Pin, draft, delete revoke
    st, pins, _ = http.json("POST", f"/api/v1/chats/{gid}/pins", {"message_id": rep["message_id"]}, token=alice["token"])
    assert st == 200 and rep["message_id"] in pins.get("pinned_message_ids", []), f"pin failed: {pins}"
    st, _, _ = http.json("PUT", f"/api/v1/chats/{gid}/draft", {"text": "Saved draft"}, token=alice["token"])
    assert st == 200
    st, dr, _ = http.json("GET", f"/api/v1/chats/{gid}/draft", token=alice["token"])
    assert st == 200 and dr.get("draft", {}).get("text") == "Saved draft", f"draft failed: {dr}"
    st, del_r, _ = http.json("DELETE", f"/api/v1/chats/{gid}/messages/{mid1}?revoke=true", token=alice["token"])
    assert st == 200 and del_r.get("revoke") is True, f"delete revoke failed: {del_r}"
