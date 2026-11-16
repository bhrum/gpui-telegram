#!/usr/bin/env python3
"""Builds `crates/fabushi_miniapp_wasm` to `wasm32-unknown-unknown`, verifies WASM
magic & C-ABI exports, stages assets into `crates/fabushi_miniapp/assets/`, and
runs mandatory post-build hygiene (`scripts/clean_build_garbage.py`).
"""

import os
import shutil
import subprocess
import sys
from typing import Dict, Tuple

REQUIRED_EXPORTS = (
    "memory",
    "wasm_alloc",
    "wasm_dealloc",
    "fabushi_wasm_init",
    "fabushi_wasm_tick",
    "fabushi_wasm_dispatch",
    "fabushi_wasm_apply_http",
    "fabushi_wasm_result_len",
    "fabushi_wasm_globe_slices_ptr",
    "fabushi_wasm_set_initial_tab",
)


def read_leb128_u32(data: bytes, offset: int) -> Tuple[int, int]:
    result = 0
    shift = 0
    while True:
        if offset >= len(data):
            raise ValueError("unexpected EOF reading LEB128")
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        if (byte & 0x80) == 0:
            return result, offset
        shift += 7
        if shift > 35:
            raise ValueError("LEB128 overflow")


def parse_wasm_exports(wasm_bytes: bytes) -> Dict[str, int]:
    if len(wasm_bytes) < 8 or wasm_bytes[:8] != b"\x00asm\x01\x00\x00\x00":
        raise ValueError("invalid WASM magic/version header")
    offset = 8
    exports: Dict[str, int] = {}
    while offset < len(wasm_bytes):
        sec_id = wasm_bytes[offset]
        offset += 1
        sec_len, offset = read_leb128_u32(wasm_bytes, offset)
        sec_end = offset + sec_len
        if sec_end > len(wasm_bytes):
            raise ValueError("truncated WASM section")
        if sec_id == 7:
            pos = offset
            count, pos = read_leb128_u32(wasm_bytes, pos)
            for _ in range(count):
                name_len, pos = read_leb128_u32(wasm_bytes, pos)
                name = wasm_bytes[pos : pos + name_len].decode("utf-8")
                pos += name_len
                kind = wasm_bytes[pos]
                pos += 1
                _, pos = read_leb128_u32(wasm_bytes, pos)
                exports[name] = kind
        offset = sec_end
    return exports


def main() -> int:
    repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    manifest = os.path.join(repo_root, "crates", "fabushi_miniapp_wasm", "Cargo.toml")
    shell_src = os.path.join(repo_root, "crates", "fabushi_miniapp_wasm", "host_shell.html")
    target_dir = os.path.join(repo_root, "target")
    wasm_built = os.path.join(
        target_dir, "wasm32-unknown-unknown", "release", "fabushi_miniapp_wasm.wasm"
    )
    assets_dir = os.path.join(repo_root, "crates", "fabushi_miniapp", "assets")
    wasm_dst = os.path.join(assets_dir, "fabushi_miniapp.wasm")
    html_dst = os.path.join(assets_dir, "hosted_bundle.html")
    clean_script = os.path.join(repo_root, "scripts", "clean_build_garbage.py")

    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = target_dir

    cmd = [
        "cargo",
        "build",
        "--manifest-path",
        manifest,
        "--target",
        "wasm32-unknown-unknown",
        "--release",
    ]
    print(f"[build_fabushi_miniapp_wasm] Running: {' '.join(cmd)}")
    try:
        subprocess.run(cmd, cwd=repo_root, env=env, check=True)
    finally:
        subprocess.run([sys.executable, clean_script], cwd=repo_root, check=True)

    with open(wasm_built, "rb") as f:
        wasm_bytes = f.read()
    exports = parse_wasm_exports(wasm_bytes)
    missing = [name for name in REQUIRED_EXPORTS if name not in exports]
    if missing:
        raise RuntimeError(f"missing required WASM exports: {missing}")

    with open(shell_src, "r", encoding="utf-8") as f:
        html_text = f.read()
    for token in ("<html", "Telegram.WebApp", "__FABUSHI_TEST_HOOK__", "/api/v1/fabushi-miniapp/app.wasm"):
        if token not in html_text:
            raise RuntimeError(f"host_shell.html missing required token: {token}")

    os.makedirs(assets_dir, exist_ok=True)
    shutil.copyfile(wasm_built, wasm_dst)
    shutil.copyfile(shell_src, html_dst)

    print(
        f"[build_fabushi_miniapp_wasm] OK: {len(wasm_bytes)} bytes, "
        f"{len(exports)} exports verified -> {wasm_dst} & {html_dst}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
