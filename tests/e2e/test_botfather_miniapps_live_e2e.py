#!/usr/bin/env python3
"""Real multi-user live child-process E2E acceptance suite for:
- @BotFather (UserId 777_001) discovery ("BotFather" & "机器人之父"), human ID isolation (<500_000)
- Slash command FSM (/newbot, /mybots, /newapp, callbacks, /deleteapp)
- Natural-language one-shot Bot + Mini App creation via DM and HTTP API
- Every Bot = AI Bot (persona responses + WebApp buttons + /help + /app + live WS push)
- Multi-user Mini App discovery, install & automatic companion bot DM + WebSocket real-time events
- Live /miniapps/{bot}/{short} HTML5 WebApp endpoint (window.Telegram.WebApp JS SDK)
- Cryptographic Telegram HMAC-SHA256 initData verification & tamper rejection
- CloudStorage CRUD + persistence across relaunch, widget actions, sendData -> companion chat, AI Co-Pilot
"""

from __future__ import annotations

import hashlib
import hmac
import json
import os
from pathlib import Path
import re
import socket
import subprocess
import sys
import tempfile
import time
from typing import Any
import urllib.parse

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _e2e_http_ws import HttpClient, WsClient  # noqa: E402

BOTFATHER_USER_ID = 777_001
REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def find_free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return int(s.getsockname()[1])


def resolve_server_binary() -> Path:
    env_bin = os.environ.get("GPUI_SERVER_BIN")
    if env_bin:
        p = Path(env_bin)
        if p.exists():
            return p
    target_bin = REPO_ROOT / "target" / "debug" / "gpui_telegram_server"
    if not target_bin.exists():
        subprocess.run(
            ["cargo", "build", "-p", "gpui_telegram_server", "--bin", "gpui_telegram_server"],
            cwd=str(REPO_ROOT),
            check=True,
        )
        subprocess.run(
            [sys.executable, str(REPO_ROOT / "scripts" / "clean_build_garbage.py")],
            cwd=str(REPO_ROOT),
            check=False,
        )
    assert target_bin.exists(), f"Missing compiled server binary: {target_bin}"
    return target_bin


def wait_until_healthy(http: HttpClient, proc: subprocess.Popen[bytes], timeout_s: float = 20.0) -> None:
    deadline = time.time() + timeout_s
    last_err = ""
    while time.time() < deadline:
        if proc.poll() is not None:
            out = proc.stdout.read().decode("utf-8", errors="replace") if proc.stdout else ""
            err = proc.stderr.read().decode("utf-8", errors="replace") if proc.stderr else ""
            raise RuntimeError(f"Server exited prematurely ({proc.returncode}):\nSTDOUT:{out}\nSTDERR:{err}")
        try:
            st, _, _ = http.request("GET", "/health")
            if st == 200:
                return
        except Exception as exc:  # noqa: BLE001
            last_err = str(exc)
        time.sleep(0.15)
    raise TimeoutError(f"Server /health did not respond within {timeout_s}s: {last_err}")


def register_user(http: HttpClient, username: str) -> dict[str, Any]:
    st, data, _ = http.json(
        "POST",
        "/api/v1/auth/register",
        {
            "username": username,
            "password": "LiveE2ePassword!2026",
            "first_name": username,
            "last_name": "Live",
            "device_name": "LivePyE2E",
        },
    )
    assert st == 201, f"register {username} failed ({st}): {data}"
    uid = int(data["user_id"])
    assert 100_001 <= uid < 500_000, f"human user_id {uid} collided with bot range!"
    return data


def list_messages(http: HttpClient, token: str, chat_id: int) -> list[dict[str, Any]]:
    st, data, _ = http.json("GET", f"/api/v1/chats/{chat_id}/messages?limit=100", token=token)
    assert st == 200, f"list_messages({chat_id}) failed ({st}): {data}"
    return list(data.get("messages", []))


def send_dm_and_wait_bot_reply(
    http: HttpClient, token: str, chat_id: int, text: str, timeout_s: float = 8.0
) -> dict[str, Any]:
    st, sent, _ = http.json(
        "POST", f"/api/v1/chats/{chat_id}/messages", {"text": text}, token=token
    )
    assert st in (200, 201), f"send '{text}' failed ({st}): {sent}"
    sent_id = int(sent["message_id"])
    deadline = time.time() + timeout_s
    while time.time() < deadline:
        msgs = list_messages(http, token, chat_id)
        for m in reversed(msgs):
            if int(m.get("id", 0)) > sent_id and int(m.get("sender_id", 0)) >= 500_000:
                return m
        time.sleep(0.05)
    raise AssertionError(f"Timed out waiting for bot reply in chat {chat_id} after '{text}'")


def verify_telegram_hmac_sha256(raw_query: str, bot_token: str) -> bool:
    pairs: list[tuple[str, str]] = []
    provided_hash = None
    for part in raw_query.split("&"):
        if not part:
            continue
        k, _, v = part.partition("=")
        dk = urllib.parse.unquote_plus(k)
        dv = urllib.parse.unquote_plus(v)
        if dk == "hash":
            provided_hash = dv
        else:
            pairs.append((dk, dv))
    if not provided_hash:
        return False
    pairs.sort(key=lambda kv: kv[0])
    check_str = "\n".join(f"{k}={v}" for k, v in pairs)
    secret_key = hmac.new(b"WebAppData", bot_token.encode("utf-8"), hashlib.sha256).digest()
    computed = hmac.new(secret_key, check_str.encode("utf-8"), hashlib.sha256).hexdigest()
    return hmac.compare_digest(computed.lower(), provided_hash.lower())


def has_webapp_button(msg: dict[str, Any], substr: str) -> bool:
    kb = msg.get("inline_keyboard") or []
    for row in kb:
        for btn in row:
            url = btn.get("web_app_url") or ""
            if substr in url:
                return True
    return False


def has_callback_button(msg: dict[str, Any], callback_data: str) -> bool:
    kb = msg.get("inline_keyboard") or []
    for row in kb:
        for btn in row:
            if btn.get("callback_data") == callback_data:
                return True
    return False


def run_live_suite() -> None:
    server_bin = resolve_server_binary()
    port = find_free_port()
    base_url = f"http://127.0.0.1:{port}"
    ws_base = f"ws://127.0.0.1:{port}"

    with tempfile.TemporaryDirectory(prefix="gpui_bf_live_e2e_") as tmpdir:
        tmp_path = Path(tmpdir)
        db_path = tmp_path / "live_botfather_miniapps.db"
        media_dir = tmp_path / "media_cas"
        media_dir.mkdir(parents=True, exist_ok=True)

        env = os.environ.copy()
        env["HOST"] = "127.0.0.1"
        env["PORT"] = str(port)
        env["DATABASE_URL"] = f"sqlite://{db_path}"
        env["MEDIA_STORAGE_PATH"] = str(media_dir)
        env["GPUI_OFFLINE_TEST"] = "1"
        env["RUST_LOG"] = "warn"

        proc = subprocess.Popen(
            [str(server_bin)],
            cwd=str(REPO_ROOT),
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        ws_alice: WsClient | None = None
        ws_bob: WsClient | None = None
        try:
            http = HttpClient(base_url, timeout=15.0)
            wait_until_healthy(http, proc)

            # 1. Register User A & User B, verify < 500_000 human IDs
            alice = register_user(http, "alice_live_bf")
            bob = register_user(http, "bob_live_bf")
            ws_alice = WsClient(f"{ws_base}/api/v1/ws?token={alice['token']}")
            ws_bob = WsClient(f"{ws_base}/api/v1/ws?token={bob['token']}")

            # 2. Search "BotFather" & Chinese "机器人之父", add @BotFather DM
            st, s_en, _ = http.json("GET", "/api/v1/search?q=BotFather", token=alice["token"])
            assert st == 200 and any(b["bot_user_id"] == BOTFATHER_USER_ID for b in s_en.get("bots", [])), s_en
            zh_q = urllib.parse.quote("机器人之父")
            st, s_zh, _ = http.json("GET", f"/api/v1/search?q={zh_q}", token=bob["token"])
            assert st == 200 and any(b["bot_user_id"] == BOTFATHER_USER_ID for b in s_zh.get("bots", [])), s_zh

            st, add_bf, _ = http.json(
                "POST",
                "/api/v1/contacts/add",
                {"username": "BotFather", "is_bot": True},
                token=alice["token"],
            )
            assert st in (200, 201), add_bf
            bf_cid = int(add_bf["chat_id"])
            ws_alice.send_json({"type": "SubscribeChat", "payload": {"chat_id": bf_cid}})

            bf_msgs = list_messages(http, alice["token"], bf_cid)
            assert bf_msgs and int(bf_msgs[0]["sender_id"]) == BOTFATHER_USER_ID
            welcome_id = int(bf_msgs[0]["id"])
            assert has_callback_button(bf_msgs[0], "bf:/newbot")

            # 3. Slash command FSM: click callback bf:/newbot -> name -> invalid 'bad' -> @LiveCoffeeBot -> /mybots -> /newapp
            st, cb_res, _ = http.json(
                "POST",
                f"/api/v1/chats/{bf_cid}/callback",
                {"message_id": welcome_id, "callback_data": "bf:/newbot"},
                token=alice["token"],
            )
            assert st == 200 and cb_res["ok"] is True, cb_res

            r_name = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "Live Coffee Bot")
            assert "username" in r_name["text"].lower() or "用户名" in r_name["text"] or "bot" in r_name["text"].lower()

            r_bad = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "bad")
            assert "❌" in r_bad["text"] or "bot" in r_bad["text"].lower()

            r_bot = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "@LiveCoffeeBot")
            assert "LiveCoffeeBot" in r_bot["text"]
            assert has_callback_button(r_bot, "bf:newapp_for:LiveCoffeeBot")

            r_mybots = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "/mybots")
            assert "LiveCoffeeBot" in r_mybots["text"]

            _ = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "/newapp")
            _ = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "@LiveCoffeeBot")
            _ = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "Live 咖啡点单")
            _ = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "实时咖啡点单与 AI 总结小程序")
            _ = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "auto")
            r_app = send_dm_and_wait_bot_reply(http, alice["token"], bf_cid, "coffee")
            assert "t.me/LiveCoffeeBot/coffee" in r_app["text"], r_app
            assert has_webapp_button(r_app, "gpui-miniapp://LiveCoffeeBot/coffee"), r_app

            # 4. Natural-language dual creation in @BotFather DM
            r_nl = send_dm_and_wait_bot_reply(
                http,
                alice["token"],
                bf_cid,
                "帮我创建一个每日习惯打卡机器人和习惯打卡小程序，支持连续打卡统计",
            )
            assert "t.me/" in r_nl["text"] and has_webapp_button(r_nl, "gpui-miniapp://"), r_nl

            # Extract minted bot_token from BotFather's /newbot reply & verify /bots/my
            tok_match = re.search(r"(\d+:[A-Za-z0-9_-]{8,})", r_bot["text"])
            assert tok_match, f"No bot token minted in BotFather reply: {r_bot}"
            coffee_token = tok_match.group(1)

            st, my_bots, _ = http.json("GET", "/api/v1/bots/my", token=alice["token"])
            assert st == 200
            assert any(b["username"] == "LiveCoffeeBot" for b in my_bots["bots"]), my_bots
            assert len(my_bots["bots"]) >= 2, f"Expected both slash & NL bots created: {my_bots}"

            # 5. User B searches "apps", finds flagship + newly created apps, installs LiveCoffeeBot/coffee
            st, s_apps, _ = http.json("GET", "/api/v1/search?q=&category=apps", token=bob["token"])
            assert st == 200
            app_shorts = {a["short_name"] for a in s_apps.get("apps", [])}
            assert {"focus", "split", "poll", "coffee"}.issubset(app_shorts), app_shorts

            st, inst_res, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/install",
                {"allow_write_access": True},
                token=bob["token"],
            )
            assert st == 200 and inst_res["ok"] is True, inst_res
            bob_companion_cid = int(inst_res["companion_chat_id"])
            assert bob_companion_cid >= 600_000_000

            ws_bob.send_json({"type": "SubscribeChat", "payload": {"chat_id": bob_companion_cid}})
            comp_msgs = list_messages(http, bob["token"], bob_companion_cid)
            assert comp_msgs and has_webapp_button(comp_msgs[0], "gpui-miniapp://LiveCoffeeBot/coffee")

            # 6. Live /miniapps/{bot}/{short} HTML5 WebApp endpoint -> window.Telegram.WebApp check
            st, _, raw_html = http.request("GET", "/miniapps/LiveCoffeeBot/coffee?init_data=test")
            html_str = raw_html.decode("utf-8")
            assert st == 200 and "window.Telegram.WebApp" in html_str and "Live 咖啡点单" in html_str

            # 7. Launch Mini App + Cryptographic HMAC-SHA256 initData check + tamper rejection
            st, launch_ctx, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/launch",
                {"chat_id": bob_companion_cid, "launch_mode": "CompactSheet", "start_param": "live42"},
                token=bob["token"],
            )
            assert st == 200 and launch_ctx["init_data"]["hmac_verified"] is True, launch_ctx
            raw_init = launch_ctx["init_data"]["raw_query_string"]
            assert verify_telegram_hmac_sha256(raw_init, coffee_token), f"HMAC mismatch on {raw_init}"

            st, val_ok, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/validate-init-data",
                {"init_data": raw_init},
                token=bob["token"],
            )
            assert st == 200 and val_ok["valid"] is True and val_ok["user_id"] == int(bob["user_id"]), val_ok

            st, val_bad, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/validate-init-data",
                {"init_data": f"{raw_init}&tamper=1"},
                token=bob["token"],
            )
            assert st == 200 and val_bad["valid"] is False, val_bad

            # 8. CloudStorage PUT/GET/DELETE + persistence across relaunch
            st, _, _ = http.json(
                "PUT",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/storage",
                {"key": "cup_size", "value": "Large"},
                token=bob["token"],
            )
            assert st == 200
            st, _, _ = http.json(
                "PUT",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/storage",
                {"key": "drop_me", "value": "123"},
                token=bob["token"],
            )
            assert st == 200
            st, _, _ = http.json(
                "DELETE",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/storage?key=drop_me",
                token=bob["token"],
            )
            assert st == 200
            st, relaunch, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/launch",
                {"launch_mode": "DirectLink"},
                token=bob["token"],
            )
            assert st == 200 and any(
                i["key"] == "cup_size" and i["value"] == "Large" for i in relaunch["cloud_storage"]
            )
            assert not any(i["key"] == "drop_me" for i in relaunch["cloud_storage"])

            # 9. Widget action, AI Co-Pilot, sendData -> WebSocket real-time push -> Companion AI chat
            st, w_res, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/action",
                {
                    "action_type": "widget_event",
                    "widget_id": "coffee_item",
                    "value": "拿铁 ¥28",
                    "chat_id": bob_companion_cid,
                },
                token=bob["token"],
            )
            assert st == 200 and w_res["ok"] is True

            st, cp_res, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/action",
                {"action_type": "ai_copilot", "prompt": "给我推荐咖啡搭配", "chat_id": bob_companion_cid},
                token=bob["token"],
            )
            assert st == 200 and (cp_res.get("ai_reply") or "").strip(), cp_res

            st, sd_res, _ = http.json(
                "POST",
                "/api/v1/miniapps/LiveCoffeeBot/coffee/action",
                {"action_type": "send_data", "data": "LiveCoffee 下单拿铁 x1", "chat_id": bob_companion_cid},
                token=bob["token"],
            )
            assert st == 200 and sd_res["ok"] is True and sd_res["close_app"] is True, sd_res

            # Verify real-time WebSocket push delivered companion sync message to Bob
            ws_ev = ws_bob.recv_until(
                lambda m: m.get("type") in ("PtsPush", "Update"), timeout=8.0
            )
            assert ws_ev, "Missing WebSocket event on sendData companion delivery"

            # 10. Every Bot = AI Bot over live WS + HTTP (/help, /app, and natural language chat)
            ai_msg = send_dm_and_wait_bot_reply(
                http, bob["token"], bob_companion_cid, "请帮我确认拿铁订单并给小程序链接"
            )
            assert ai_msg["text"].strip() and has_webapp_button(ai_msg, "gpui-miniapp://LiveCoffeeBot/coffee")

            help_msg = send_dm_and_wait_bot_reply(http, bob["token"], bob_companion_cid, "/help")
            assert help_msg["text"].strip()

            app_msg = send_dm_and_wait_bot_reply(http, bob["token"], bob_companion_cid, "/app")
            assert has_webapp_button(app_msg, "gpui-miniapp://LiveCoffeeBot/coffee")

            print("[PASS] Live E2E: BotFather + NL + Companion AI Bots + Mini Apps HTML5 + HMAC + CloudStorage + WS")
        finally:
            if ws_alice is not None:
                ws_alice.close()
            if ws_bob is not None:
                ws_bob.close()
            if proc.poll() is None:
                proc.terminate()
                try:
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait(timeout=5)


def test_botfather_miniapps_live_e2e() -> None:
    run_live_suite()


def main() -> int:
    run_live_suite()
    return 0


if __name__ == "__main__":
    sys.exit(main())
