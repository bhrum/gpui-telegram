#!/usr/bin/env python3
"""Capture live macOS GPUI window screenshots against the Cloud server.

Seeds real Alice & Bob accounts, contacts, group/topic, poll, call, and ACP task
on `http://34.46.77.133:8080`, drives the real packaged `GPUI Telegram.app`
binary (`1280x820`) via macOS `CGEvent` shortcuts/clicks/keystrokes, captures
all 17 window PNGs using `/usr/sbin/screencapture -x -o -l <wid>`, validates PNG
headers and byte sizes (> 10 KB), verifies live composer roundtrip persistence
via HTTP GET, and cleans up all temporary processes/dirs/input-source state.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_APP_BIN = (
    REPO_ROOT / "dist" / "GPUI Telegram.app" / "Contents" / "MacOS" / "gpui_telegram_desktop"
)
DEFAULT_OUT_DIR = REPO_ROOT / "docs" / "verification" / "fusion-20261108" / "ui-evidence"
DEFAULT_SERVER_URL = "http://34.46.77.133:8080"
PNG_MAGIC = b"\x89PNG\r\n\x1a\n"
MIN_PNG_BYTES = 10_240
WINDOW_CONTENT_WIDTH = 1280.0
WINDOW_CONTENT_HEIGHT = 820.0

# Exact macOS Swift helper compiled once per run inside TemporaryDirectory.
SWIFT_DRIVER_SOURCE = r'''
import AppKit
import ApplicationServices
import Carbon
import CoreGraphics
import Foundation

func jsonOut(_ obj: [String: Any]) {
    if let data = try? JSONSerialization.data(withJSONObject: obj, options: []),
       let s = String(data: data, encoding: .utf8) {
        print(s)
    }
}

func findMainWindow(pid: Int32) -> [String: Any]? {
    let list = CGWindowListCopyWindowInfo([.optionAll], kCGNullWindowID) as? [[String: Any]] ?? []
    var best: [String: Any]? = nil
    var bestArea: Double = 0.0
    for w in list {
        guard let ownerPid = w[kCGWindowOwnerPID as String] as? Int32, ownerPid == pid else { continue }
        let layer = (w[kCGWindowLayer as String] as? Int) ?? -1
        if layer != 0 { continue }
        guard let bounds = w[kCGWindowBounds as String] as? [String: Any],
              let width = bounds["Width"] as? Double,
              let height = bounds["Height"] as? Double,
              let x = bounds["X"] as? Double,
              let y = bounds["Y"] as? Double,
              let wid = w[kCGWindowNumber as String] as? Int else { continue }
        if width >= 800.0 && height >= 560.0 {
            let area = width * height
            if area > bestArea {
                bestArea = area
                best = ["wid": wid, "x": x, "y": y, "width": width, "height": height]
            }
        }
    }
    return best
}

func currentInputSourceId() -> String? {
    let src = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
    guard let raw = TISGetInputSourceProperty(src, kTISPropertyInputSourceID) else { return nil }
    return Unmanaged<CFString>.fromOpaque(raw).takeUnretainedValue() as String
}

func selectInputSource(id targetId: String) -> Bool {
    let filter = [kTISPropertyInputSourceID as String: targetId] as CFDictionary
    guard let arr = TISCreateInputSourceList(filter, false)?.takeRetainedValue() as? [TISInputSource],
          let first = arr.first else {
        return false
    }
    return TISSelectInputSource(first) == noErr
}

func asciiKeySpec(_ ch: Character) -> (CGKeyCode, Bool)? {
    switch ch {
    case "a": return (0, false)
    case "s": return (1, false)
    case "d": return (2, false)
    case "f": return (3, false)
    case "h": return (4, false)
    case "g": return (5, false)
    case "z": return (6, false)
    case "x": return (7, false)
    case "c": return (8, false)
    case "v": return (9, false)
    case "b": return (11, false)
    case "q": return (12, false)
    case "w": return (13, false)
    case "e": return (14, false)
    case "r": return (15, false)
    case "y": return (16, false)
    case "t": return (17, false)
    case "1": return (18, false)
    case "2": return (19, false)
    case "3": return (20, false)
    case "4": return (21, false)
    case "6": return (22, false)
    case "5": return (23, false)
    case "=": return (24, false)
    case "9": return (25, false)
    case "7": return (26, false)
    case "-": return (27, false)
    case "8": return (28, false)
    case "0": return (29, false)
    case "]": return (30, false)
    case "o": return (31, false)
    case "u": return (32, false)
    case "[": return (33, false)
    case "i": return (34, false)
    case "p": return (35, false)
    case "l": return (37, false)
    case "j": return (38, false)
    case "'": return (39, false)
    case "k": return (40, false)
    case ";": return (41, false)
    case "\\": return (42, false)
    case ",": return (43, false)
    case "/": return (44, false)
    case "n": return (45, false)
    case "m": return (46, false)
    case ".": return (47, false)
    case " ": return (49, false)
    case "`": return (50, false)
    case "A": return (0, true)
    case "S": return (1, true)
    case "D": return (2, true)
    case "F": return (3, true)
    case "H": return (4, true)
    case "G": return (5, true)
    case "Z": return (6, true)
    case "X": return (7, true)
    case "C": return (8, true)
    case "V": return (9, true)
    case "B": return (11, true)
    case "Q": return (12, true)
    case "W": return (13, true)
    case "E": return (14, true)
    case "R": return (15, true)
    case "Y": return (16, true)
    case "T": return (17, true)
    case "!": return (18, true)
    case "@": return (19, true)
    case "#": return (20, true)
    case "$": return (21, true)
    case "^": return (22, true)
    case "%": return (23, true)
    case "+": return (24, true)
    case "(": return (25, true)
    case "&": return (26, true)
    case "_": return (27, true)
    case "*": return (28, true)
    case ")": return (29, true)
    case "}": return (30, true)
    case "O": return (31, true)
    case "U": return (32, true)
    case "{": return (33, true)
    case "I": return (34, true)
    case "P": return (35, true)
    case "L": return (37, true)
    case "J": return (38, true)
    case "\"": return (39, true)
    case "K": return (40, true)
    case ":": return (41, true)
    case "|": return (42, true)
    case "<": return (43, true)
    case "?": return (44, true)
    case "N": return (45, true)
    case "M": return (46, true)
    case ">": return (47, true)
    case "~": return (50, true)
    default: return nil
    }
}

func postKey(code: CGKeyCode, flags: CGEventFlags) {
    let src = CGEventSource(stateID: .privateState)
    guard let down = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: true),
          let up = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: false) else {
        fputs("Failed to create CGEvent for key \(code)\n", stderr)
        exit(2)
    }
    down.flags = flags
    up.flags = flags
    down.post(tap: .cghidEventTap)
    usleep(35_000)
    up.post(tap: .cghidEventTap)
    usleep(35_000)
}

func parseFlags(_ raw: String) -> CGEventFlags {
    var flags: CGEventFlags = []
    for part in raw.split(separator: ",") {
        switch part.lowercased() {
        case "cmd": flags.insert(.maskCommand)
        case "shift": flags.insert(.maskShift)
        case "ctrl": flags.insert(.maskControl)
        case "alt": flags.insert(.maskAlternate)
        default: break
        }
    }
    return flags
}

let args = CommandLine.arguments
if args.count < 2 {
    fputs("Usage: helper <cmd> ...\n", stderr)
    exit(1)
}

switch args[1] {
case "preflight":
    jsonOut([
        "screen_capture": CGPreflightScreenCaptureAccess(),
        "post_event": CGPreflightPostEventAccess(),
        "ax_trusted": AXIsProcessTrusted(),
    ])
case "find-window":
    guard args.count >= 3, let pid = Int32(args[2]) else { exit(1) }
    if let w = findMainWindow(pid: pid) {
        jsonOut(w)
    } else {
        exit(1)
    }
case "activate":
    guard args.count >= 3, let pid = Int32(args[2]) else { exit(1) }
    if let app = NSRunningApplication(processIdentifier: pid) {
        _ = app.activate(options: [.activateAllWindows])
    }
    let axApp = AXUIElementCreateApplication(pid)
    var winsRef: CFTypeRef?
    if AXUIElementCopyAttributeValue(axApp, kAXWindowsAttribute as CFString, &winsRef) == .success,
       let wins = winsRef as? [AXUIElement], let first = wins.first {
        _ = AXUIElementPerformAction(first, kAXRaiseAction as CFString)
        _ = AXUIElementSetAttributeValue(first, kAXMainAttribute as CFString, kCFBooleanTrue)
    }
case "key":
    guard args.count >= 4, let code = UInt16(args[3]) else { exit(1) }
    let flags = parseFlags(args[2])
    postKey(code: code, flags: flags)
case "click":
    guard args.count >= 4, let x = Double(args[2]), let y = Double(args[3]) else { exit(1) }
    let pt = CGPoint(x: x, y: y)
    let src = CGEventSource(stateID: .privateState)
    if let mv = CGEvent(mouseEventSource: src, mouseType: .mouseMoved, mouseCursorPosition: pt, mouseButton: .left) {
        mv.post(tap: .cghidEventTap)
        usleep(25_000)
    }
    guard let dn = CGEvent(mouseEventSource: src, mouseType: .leftMouseDown, mouseCursorPosition: pt, mouseButton: .left),
          let up = CGEvent(mouseEventSource: src, mouseType: .leftMouseUp, mouseCursorPosition: pt, mouseButton: .left) else {
        exit(2)
    }
    dn.post(tap: .cghidEventTap)
    usleep(40_000)
    up.post(tap: .cghidEventTap)
    usleep(40_000)
case "type":
    guard args.count >= 3 else { exit(1) }
    for ch in args[2] {
        if let (code, shift) = asciiKeySpec(ch) {
            postKey(code: code, flags: shift ? .maskShift : [])
        } else {
            var chars = Array(String(ch).utf16)
            let src = CGEventSource(stateID: .privateState)
            if let dn = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: true),
               let up = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: false) {
                dn.keyboardSetUnicodeString(stringLength: chars.count, unicodeString: &chars)
                up.keyboardSetUnicodeString(stringLength: chars.count, unicodeString: &chars)
                dn.post(tap: .cghidEventTap)
                usleep(25_000)
                up.post(tap: .cghidEventTap)
                usleep(25_000)
            }
        }
    }
case "ensure-ascii":
    let prev = currentInputSourceId() ?? ""
    let ascii = TISCopyCurrentASCIICapableKeyboardInputSource().takeRetainedValue()
    let asciiId: String
    if let raw = TISGetInputSourceProperty(ascii, kTISPropertyInputSourceID) {
        asciiId = Unmanaged<CFString>.fromOpaque(raw).takeUnretainedValue() as String
    } else {
        asciiId = "com.apple.keylayout.ABC"
    }
    let ok = (prev == asciiId) || (TISSelectInputSource(ascii) == noErr)
    jsonOut(["previous": prev, "ascii": asciiId, "switched": ok])
case "select-source":
    guard args.count >= 3 else { exit(1) }
    let ok = selectInputSource(id: args[2])
    jsonOut(["selected": args[2], "ok": ok])
default:
    fputs("Unknown command: \(args[1])\n", stderr)
    exit(1)
}
'''

# macOS Carbon virtual keycodes matching handle_shortcut_key / handle_non_char_key
VK = {
    "d": 2,
    "e": 14,
    "i": 34,
    "j": 38,
    "l": 37,
    "n": 45,
    "p": 35,
    "1": 18,
    "3": 20,
    "4": 21,
    ",": 43,
    "return": 36,
    "escape": 53,
}


class CloudApi:
    """Minimal stdlib HTTP JSON helper for live Cloud seeding and verification."""

    def __init__(self, base_url: str) -> None:
        self.base_url = base_url.rstrip("/")

    def request(
        self,
        method: str,
        path: str,
        body: dict[str, Any] | None = None,
        token: str | None = None,
        timeout: float = 15.0,
    ) -> tuple[int, Any]:
        url = f"{self.base_url}{path}"
        headers = {"Accept": "application/json"}
        data: bytes | None = None
        if body is not None:
            headers["Content-Type"] = "application/json"
            data = json.dumps(body).encode("utf-8")
        if token:
            headers["Authorization"] = f"Bearer {token}"
        req = urllib.request.Request(url, data=data, headers=headers, method=method)
        try:
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                raw = resp.read().decode("utf-8", errors="replace")
                if not raw:
                    return resp.status, None
                try:
                    return resp.status, json.loads(raw)
                except json.JSONDecodeError:
                    return resp.status, raw
        except urllib.error.HTTPError as err:
            raw = err.read().decode("utf-8", errors="replace")
            try:
                parsed = json.loads(raw) if raw else None
            except json.JSONDecodeError:
                parsed = raw
            return err.code, parsed


class MacGuiDriver:
    """Compiled Swift CGEvent + WindowServer helper wrapper."""

    def __init__(self, tmp_dir: Path) -> None:
        src_path = tmp_dir / "gui_driver.swift"
        self.bin_path = tmp_dir / "gui_driver"
        src_path.write_text(SWIFT_DRIVER_SOURCE, encoding="utf-8")
        subprocess.run(
            ["swiftc", "-O", str(src_path), "-o", str(self.bin_path)],
            check=True,
        )

    def _run(self, *args: str) -> str:
        res = subprocess.run(
            [str(self.bin_path), *args],
            check=True,
            text=True,
            capture_output=True,
        )
        return res.stdout.strip()

    def check_permissions(self) -> dict[str, bool]:
        return json.loads(self._run("preflight"))

    def ensure_ascii_input_source(self) -> str:
        info = json.loads(self._run("ensure-ascii"))
        return str(info.get("previous") or "")

    def restore_input_source(self, source_id: str) -> None:
        if source_id:
            self._run("select-source", source_id)

    def wait_for_window(self, pid: int, timeout_secs: float = 20.0) -> dict[str, Any]:
        deadline = time.time() + timeout_secs
        while time.time() < deadline:
            res = subprocess.run(
                [str(self.bin_path), "find-window", str(pid)],
                text=True,
                capture_output=True,
            )
            if res.returncode == 0 and res.stdout.strip():
                return json.loads(res.stdout.strip())
            time.sleep(0.25)
        raise TimeoutError(f"Timed out waiting for main window of pid={pid}")

    def activate(self, pid: int) -> None:
        self._run("activate", str(pid))

    def send_key(self, name: str, flags: str = "none") -> None:
        self._run("key", flags, str(VK[name]))

    def click_viewport(self, win: dict[str, Any], vx: float, vy: float) -> None:
        titlebar_h = max(0.0, float(win["height"]) - WINDOW_CONTENT_HEIGHT)
        sx = float(win["x"]) + vx
        sy = float(win["y"]) + titlebar_h + vy
        self._run("click", f"{sx:.1f}", f"{sy:.1f}")

    def type_text(self, text: str) -> None:
        self._run("type", text)


def capture_and_validate(wid: int, out_path: Path) -> int:
    out_path.parent.mkdir(parents=True, exist_ok=True)
    if out_path.exists():
        out_path.unlink()
    subprocess.run(
        ["/usr/sbin/screencapture", "-x", "-o", "-l", str(wid), str(out_path)],
        check=True,
    )
    if not out_path.is_file():
        raise RuntimeError(f"screencapture did not create file: {out_path}")
    data = out_path.read_bytes()
    if len(data) <= MIN_PNG_BYTES:
        raise RuntimeError(f"PNG too small ({len(data)} bytes <= {MIN_PNG_BYTES}): {out_path}")
    if not data.startswith(PNG_MAGIC):
        raise RuntimeError(f"Invalid PNG header in {out_path}")
    print(f"[CAPTURED] {out_path.name} ({len(data)} bytes)")
    return len(data)


def seed_live_cloud_world(api: CloudApi) -> dict[str, Any]:
    st, health = api.request("GET", "/health")
    if st != 200:
        raise RuntimeError(f"Cloud /health failed: {st} {health}")
    st, status = api.request("GET", "/api/v1/status")
    if st != 200:
        raise RuntimeError(f"Cloud /api/v1/status failed: {st} {status}")

    suffix = str(int(time.time() * 1000))
    pw = "UiParityAudit!2026"
    alice_name = f"ui_alice_{suffix}"
    bob_name = f"ui_bob_{suffix}"

    st, alice = api.request(
        "POST",
        "/api/v1/auth/register",
        {"username": alice_name, "password": pw, "first_name": "Alice", "device_name": "ui-audit-mac"},
    )
    if st not in (200, 201) or not alice.get("token"):
        raise RuntimeError(f"Failed to register Alice: {st} {alice}")

    st, bob = api.request(
        "POST",
        "/api/v1/auth/register",
        {"username": bob_name, "password": pw, "first_name": "Bob", "device_name": "ui-audit-peer"},
    )
    if st not in (200, 201) or not bob.get("token"):
        raise RuntimeError(f"Failed to register Bob: {st} {bob}")

    # Update Alice profile & add Bob contact
    api.request(
        "PUT",
        "/api/v1/users/me",
        {"first_name": "Alice", "last_name": "Auditor", "bio": "GPUI 1:1 Desktop Parity"},
        token=alice["token"],
    )
    st, contact_res = api.request(
        "POST",
        "/api/v1/contacts",
        {"username": bob_name, "first_name": "Bob Developer"},
        token=alice["token"],
    )
    if st not in (200, 201):
        raise RuntimeError(f"Failed to add contact Bob: {st} {contact_res}")

    # Create primary rich group + invite Bob + create topic
    st, grp = api.request(
        "POST",
        "/api/v1/chats",
        {"title": "GPUI Telegram Engineering", "chat_type": "group", "is_forum": True},
        token=alice["token"],
    )
    if st not in (200, 201) or not grp.get("chat_id"):
        raise RuntimeError(f"Failed to create group: {st} {grp}")
    gid = int(grp["chat_id"])

    st, inv = api.request("POST", f"/api/v1/chats/{gid}/invites", {"title": "Audit Join"}, token=alice["token"])
    if st not in (200, 201) or not inv.get("code"):
        raise RuntimeError(f"Failed to create invite: {st} {inv}")
    st, join_res = api.request("POST", "/api/v1/chats/join", {"code": inv["code"]}, token=bob["token"])
    if st != 200:
        raise RuntimeError(f"Failed for Bob to join group: {st} {join_res}")

    api.request(
        "POST",
        f"/api/v1/chats/{gid}/topics",
        {"title": "Visual QA", "icon_color": 0x6FB9F0},
        token=alice["token"],
    )

    # Create a channel & folder so NavRail / Sidebar tabs show real server entities
    st, chan = api.request(
        "POST",
        "/api/v1/chats",
        {"title": "GPUI Release Notes", "chat_type": "channel"},
        token=alice["token"],
    )
    if st in (200, 201) and chan.get("chat_id"):
        api.request(
            "POST",
            f"/api/v1/chats/{int(chan['chat_id'])}/messages",
            {"text": "GPUI Telegram Desktop v1.0.0 live Cloud release verified."},
            token=alice["token"],
        )
        api.request(
            "POST",
            "/api/v1/folders",
            {"title": "Work", "include_chats": [gid, int(chan["chat_id"])], "emoticon": "work"},
            token=alice["token"],
        )

    # Seed live messages, reply, edit, reaction, and interactive Poll inside primary group `gid`
    st, m1 = api.request(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {"text": "Starting live macOS Metal 1:1 Telegram Desktop UI verification."},
        token=alice["token"],
    )
    if st not in (200, 201):
        raise RuntimeError(f"Failed to seed initial message: {st} {m1}")
    mid1 = int(m1["message_id"])

    api.request(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {"text": "All 62 vector SVGs and 4 tdesktop palettes look crisp!", "reply_to": mid1},
        token=bob["token"],
    )
    api.request(
        "POST",
        f"/api/v1/chats/{gid}/messages/{mid1}/reactions",
        {"emoji": "🔥", "action": "toggle"},
        token=bob["token"],
    )

    st, poll_res = api.request(
        "POST",
        f"/api/v1/chats/{gid}/polls",
        {
            "question": "Which tdesktop theme renders sharpest in GPUI?",
            "options": ["Night", "Tinted", "Day", "Classic"],
            "is_anonymous": False,
            "is_quiz": False,
        },
        token=alice["token"],
    )
    if st in (200, 201) and poll_res.get("poll", {}).get("id"):
        api.request(
            "POST",
            f"/api/v1/polls/{poll_res['poll']['id']}/vote",
            {"option_ids": [0]},
            token=bob["token"],
        )

    st, m_latest = api.request(
        "POST",
        f"/api/v1/chats/{gid}/messages",
        {"text": "Ready for live composer roundtrip and theme parity check."},
        token=alice["token"],
    )
    if st not in (200, 201):
        raise RuntimeError(f"Failed to seed final group message: {st} {m_latest}")

    # Seed Call history + ACP Task for Calls & Global Task Center panels
    st, call = api.request(
        "POST",
        "/api/v1/calls/invite",
        {"callee_id": int(bob["user_id"]), "is_video": True, "chat_id": gid},
        token=alice["token"],
    )
    if st in (200, 201) and call.get("call_id"):
        api.request(
            "POST",
            "/api/v1/calls/signal",
            {"call_id": call["call_id"], "signal_type": "answer", "sdp": "v=0"},
            token=bob["token"],
        )

    api.request(
        "POST",
        "/api/v1/tasks",
        {
            "task_id": f"task_ui_{suffix}",
            "title": "Live 1:1 GPUI Visual Verification",
            "goal": "Audit Night/Tinted/Day/Classic + NavRail + Modals + Settings + Roundtrip",
        },
        token=alice["token"],
    )

    # Determine exact row index of `gid` in Alice's chat list
    st, chat_list = api.request("GET", "/api/v1/chats", token=alice["token"])
    if st != 200 or not isinstance(chat_list, dict):
        raise RuntimeError(f"Failed to fetch Alice chat list: {st} {chat_list}")
    chats = chat_list.get("chats", [])
    gid_row_index = 0
    for idx, ch in enumerate(chats):
        if int(ch.get("chat_id", ch.get("id", -1))) == gid:
            gid_row_index = idx
            break

    print(
        f"[SEEDED] alice={alice_name} (id={alice['user_id']}) bob={bob_name} "
        f"gid={gid} row_index={gid_row_index} total_chats={len(chats)}"
    )
    return {
        "suffix": suffix,
        "alice": alice,
        "bob": bob,
        "gid": gid,
        "gid_row_index": gid_row_index,
        "chat_ids": [int(c.get("chat_id", c.get("id", 0))) for c in chats],
    }


def launch_isolated_app(
    app_bin: Path,
    server_url: str,
    config_dir: Path,
    xdg_data_dir: Path,
    log_path: Path,
) -> subprocess.Popen[str]:
    env = os.environ.copy()
    env["GPUI_TELEGRAM_SERVER_URL"] = server_url
    env["GPUI_TELEGRAM_CONFIG_DIR"] = str(config_dir)
    env["XDG_DATA_HOME"] = str(xdg_data_dir)
    log_file = open(log_path, "w", encoding="utf-8")
    proc = subprocess.Popen(
        [str(app_bin), "--server-url", server_url],
        env=env,
        stdout=log_file,
        stderr=subprocess.STDOUT,
        text=True,
    )
    # Keep handle attached so we can close it on termination
    setattr(proc, "_audit_log_handle", log_file)
    return proc


def terminate_proc(proc: subprocess.Popen[str] | None) -> None:
    if proc is None:
        return
    try:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=5.0)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait(timeout=3.0)
    finally:
        fh = getattr(proc, "_audit_log_handle", None)
        if fh is not None:
            fh.close()


def _extract_msg_text(msg: dict[str, Any]) -> str:
    text = msg.get("text") or ""
    if not text and isinstance(msg.get("content"), dict):
        content = msg["content"]
        if isinstance(content.get("data"), dict):
            text = content["data"].get("text") or ""
        if not text:
            text = content.get("text") or ""
    return str(text)


def wait_for_cloud_message(
    api: CloudApi,
    token: str,
    chat_ids: list[int],
    needle: str,
    timeout_secs: float = 10.0,
) -> tuple[int, int]:
    deadline = time.time() + timeout_secs
    while time.time() < deadline:
        for cid in chat_ids:
            st, body = api.request("GET", f"/api/v1/chats/{cid}/messages?limit=50", token=token)
            if st != 200:
                continue
            msgs = body.get("messages", []) if isinstance(body, dict) else (body or [])
            exact_matches = [m for m in msgs if _extract_msg_text(m) == needle]
            single_char_msgs = [m for m in msgs if len(_extract_msg_text(m)) == 1]
            if single_char_msgs:
                raise AssertionError(
                    f"Detected {len(single_char_msgs)} single-char split messages in chat {cid}: "
                    f"{[_extract_msg_text(m) for m in single_char_msgs[:10]]}"
                )
            if len(exact_matches) > 1:
                raise AssertionError(
                    f"Detected {len(exact_matches)} duplicate roundtrip messages for {needle!r} in chat {cid}"
                )
            if len(exact_matches) == 1:
                m = exact_matches[0]
                return cid, int(m.get("message_id", m.get("id", 0)))
        time.sleep(0.35)
    raise AssertionError(f"Roundtrip message {needle!r} was not persisted in Cloud chats {chat_ids}")


def run_capture(app_bin: Path, server_url: str, out_dir: Path) -> int:
    if not app_bin.is_file():
        raise FileNotFoundError(f"Missing packaged desktop binary: {app_bin}")

    out_dir.mkdir(parents=True, exist_ok=True)
    api = CloudApi(server_url)
    seeded = seed_live_cloud_world(api)

    procs: list[subprocess.Popen[str]] = []
    driver: MacGuiDriver | None = None
    prev_input_source = ""

    with tempfile.TemporaryDirectory(prefix="gpui_tg_ui_audit_") as tmp_root_str:
        tmp_root = Path(tmp_root_str)
        driver_dir = tmp_root / "driver"
        login_cfg = tmp_root / "login_cfg"
        login_xdg = tmp_root / "login_xdg"
        auth_cfg = tmp_root / "auth_cfg"
        auth_xdg = tmp_root / "auth_xdg"
        for d in (driver_dir, login_cfg, login_xdg, auth_cfg, auth_xdg):
            d.mkdir(parents=True, exist_ok=True)

        driver = MacGuiDriver(driver_dir)
        perms = driver.check_permissions()
        if not (perms.get("screen_capture") and perms.get("post_event") and perms.get("ax_trusted")):
            raise RuntimeError(f"Missing macOS TCC permissions: {perms}")

        try:
            subprocess.run(["caffeinate", "-u", "-t", "5"], check=False)
            prev_input_source = driver.ensure_ascii_input_source()

            # ------------------------------------------------------------------
            # Part 1: Unauthenticated Login Wizard (Night zh-CN -> Day en-US)
            # ------------------------------------------------------------------
            login_proc = launch_isolated_app(
                app_bin, server_url, login_cfg, login_xdg, tmp_root / "login_app.log"
            )
            procs.append(login_proc)
            win = driver.wait_for_window(login_proc.pid, timeout_secs=20.0)
            driver.activate(login_proc.pid)
            time.sleep(1.5)
            win = driver.wait_for_window(login_proc.pid, timeout_secs=5.0)
            capture_and_validate(int(win["wid"]), out_dir / "01-login-night-zh.png")

            # Night -> Tinted -> Day (`Cmd+D` x2), then zh-CN -> en-US (`Cmd+L`)
            driver.activate(login_proc.pid)
            driver.send_key("d", "cmd")
            time.sleep(0.6)
            driver.send_key("d", "cmd")
            time.sleep(0.6)
            driver.send_key("l", "cmd")
            time.sleep(0.7)
            capture_and_validate(int(win["wid"]), out_dir / "02-login-day-en.png")

            terminate_proc(login_proc)

            # ------------------------------------------------------------------
            # Part 2: Authenticated Workspace + Themes + Modals + Settings + E2E
            # ------------------------------------------------------------------
            alice = seeded["alice"]
            session_payload = {
                "server_url": server_url.rstrip("/"),
                "user_id": int(alice["user_id"]),
                "username": alice["username"],
                "token": alice["token"],
                "session_id": alice["session_id"],
                "created_at_ms": int(time.time() * 1000),
            }
            session_path = auth_cfg / "session.json"
            session_path.write_text(json.dumps(session_payload, indent=2), encoding="utf-8")
            os.chmod(session_path, 0o600)

            auth_proc = launch_isolated_app(
                app_bin, server_url, auth_cfg, auth_xdg, tmp_root / "auth_app.log"
            )
            procs.append(auth_proc)
            win = driver.wait_for_window(auth_proc.pid, timeout_secs=20.0)
            driver.activate(auth_proc.pid)
            # Wait for `/auth/me`, `/chats`, and initial WebSocket sync
            time.sleep(2.5)
            win = driver.wait_for_window(auth_proc.pid, timeout_secs=5.0)

            # Click the seeded group row in ChatSidebar (NavRail=52, Sidebar=288 -> center x=196;
            # Sidebar header=48, folder tabs=36 -> first row starts y=84, row pitch=62).
            row_vy = 84.0 + (float(seeded["gid_row_index"]) * 62.0) + 31.0
            driver.click_viewport(win, 196.0, row_vy)
            time.sleep(1.2)
            capture_and_validate(int(win["wid"]), out_dir / "03-workspace-night.png")

            # Cycle themes: Night -> Tinted -> Day -> Classic (`Cmd+D` x3)
            for fname in ("04-theme-tinted.png", "05-theme-day.png", "06-theme-classic.png"):
                driver.activate(auth_proc.pid)
                driver.send_key("d", "cmd")
                time.sleep(0.55)
                capture_and_validate(int(win["wid"]), out_dir / fname)

            # Classic -> Night (`Cmd+D`) + open ContextInspector (`Cmd+I`)
            driver.send_key("d", "cmd")
            time.sleep(0.35)
            driver.send_key("i", "cmd")
            time.sleep(0.55)
            capture_and_validate(int(win["wid"]), out_dir / "07-context-inspector.png")

            # Esc closes Inspector -> `Cmd+E` opens MainMenuDrawer
            driver.send_key("escape")
            time.sleep(0.25)
            driver.send_key("e", "cmd")
            time.sleep(0.45)
            capture_and_validate(int(win["wid"]), out_dir / "08-main-menu-drawer.png")

            # Esc closes Drawer -> `Cmd+N` opens New Group modal
            driver.send_key("escape")
            time.sleep(0.25)
            driver.send_key("n", "cmd")
            time.sleep(0.45)
            capture_and_validate(int(win["wid"]), out_dir / "09-modal-new-group.png")

            # Esc closes Modal -> `Cmd+P` opens Create Poll modal
            driver.send_key("escape")
            time.sleep(0.25)
            driver.send_key("p", "cmd")
            time.sleep(0.45)
            capture_and_validate(int(win["wid"]), out_dir / "10-modal-create-poll.png")

            # Esc closes Modal -> `Cmd+J` opens Contacts modal
            driver.send_key("escape")
            time.sleep(0.25)
            driver.send_key("j", "cmd")
            time.sleep(0.6)
            capture_and_validate(int(win["wid"]), out_dir / "11-modal-contacts.png")

            # Esc closes Contacts modal -> `Cmd+,` opens Settings Main
            driver.send_key("escape")
            time.sleep(0.25)
            driver.send_key(",", "cmd")
            time.sleep(0.55)
            capture_and_validate(int(win["wid"]), out_dir / "12-settings-main.png")

            # Click "Chat Settings" row (row 3 in SettingsMainSections, center ~ (700, 390))
            driver.click_viewport(win, 700.0, 428.0)
            time.sleep(0.5)
            capture_and_validate(int(win["wid"]), out_dir / "13-settings-chat.png")

            # Esc goes back to Settings Main -> click "Privacy & Security" row (row 2, center ~ (700, 382))
            driver.send_key("escape")
            time.sleep(0.3)
            driver.click_viewport(win, 700.0, 382.0)
            time.sleep(0.5)
            capture_and_validate(int(win["wid"]), out_dir / "14-settings-privacy.png")

            # Esc back to Main, Esc back to Chats -> `Cmd+3` Calls panel
            driver.send_key("escape")
            time.sleep(0.2)
            driver.send_key("escape")
            time.sleep(0.2)
            driver.send_key("3", "cmd")
            time.sleep(0.75)
            capture_and_validate(int(win["wid"]), out_dir / "15-calls-workspace.png")

            # `Cmd+4` Global Task Center panel
            driver.send_key("4", "cmd")
            time.sleep(0.85)
            capture_and_validate(int(win["wid"]), out_dir / "16-global-tasks-workspace.png")

            # `Cmd+1` Chats panel -> click composer input -> type unique message -> Return -> verify Cloud persistence
            driver.activate(auth_proc.pid)
            driver.send_key("1", "cmd")
            time.sleep(0.55)
            driver.click_viewport(win, 720.0, 790.0)
            time.sleep(0.35)
            roundtrip_msg = f"Live GPUI Roundtrip {seeded['suffix']}"
            driver.type_text(roundtrip_msg)
            time.sleep(0.35)
            driver.send_key("return")

            persisted_cid, persisted_mid = wait_for_cloud_message(
                api, alice["token"], seeded["chat_ids"], roundtrip_msg, timeout_secs=10.0
            )
            time.sleep(0.7)
            capture_and_validate(int(win["wid"]), out_dir / "17-live-composer-roundtrip.png")
            print(
                f"[VERIFIED] Live composer HTTP persistence confirmed: "
                f"chat_id={persisted_cid} message_id={persisted_mid} text={roundtrip_msg!r}"
            )
            return 0
        finally:
            for p in procs:
                terminate_proc(p)
            if driver is not None and prev_input_source:
                try:
                    driver.restore_input_source(prev_input_source)
                except Exception as exc:
                    print(f"[WARN] Failed to restore input source {prev_input_source}: {exc}", file=sys.stderr)


def main() -> int:
    parser = argparse.ArgumentParser(description="Capture 17 live GPUI Telegram Desktop UI screenshots")
    parser.add_argument("--server-url", default=DEFAULT_SERVER_URL)
    parser.add_argument("--app-bin", type=Path, default=DEFAULT_APP_BIN)
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT_DIR)
    args = parser.parse_args()
    return run_capture(args.app_bin, args.server_url, args.out_dir)


if __name__ == "__main__":
    sys.exit(main())
