#!/usr/bin/env python3
"""
Wire materialized modules into mod.rs and lib.rs for gpui_telegram_core and gpui_telegram_proto.
Uses explicit symbol re-exports to avoid ambiguous glob re-export warnings.
"""

import os
import glob
import re

REPO_ROOT = "/Users/bhrum/teamwork_projects/gpui_telegram"

def extract_primary_symbol(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
    # Find the main struct/enum/trait/type/fn
    m = re.search(r"//! Telegram Desktop 1:1 Parity Submodule: `(\w+)`", content)
    if m:
        return m.group(1)
    m = re.search(r"\bpub\s+(?:struct|enum|trait|fn|type)\s+(\w+)\b", content)
    if m:
        return m.group(1)
    return None

def generate_mod_rs(dir_path, submodules=None):
    submodules = submodules or []
    files = sorted(glob.glob(os.path.join(dir_path, "*.rs")))
    mod_lines = []
    mod_lines.append("//! Auto-generated module tree")
    mod_lines.append("")

    for sub in submodules:
        mod_lines.append(f"pub mod {sub};")
        mod_lines.append(f"pub use {sub}::*;")

    for f in files:
        base = os.path.basename(f)
        if base == "mod.rs":
            continue
        stem = os.path.splitext(base)[0]
        sym = extract_primary_symbol(f)
        mod_lines.append(f"pub mod {stem};")
        if sym:
            mod_lines.append(f"pub use {stem}::{sym};")

    mod_path = os.path.join(dir_path, "mod.rs")
    with open(mod_path, "w", encoding="utf-8") as mf:
        mf.write("\n".join(mod_lines) + "\n")
    print(f"Wrote {mod_path}")

def main():
    # 1. deep_links
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/core/deep_links"))

    # 2. core
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/core"), submodules=["deep_links"])

    # 3. mtproto details
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/mtproto/details"))

    # 4. mtproto web_proxy
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/mtproto/web_proxy"))

    # 5. mtproto
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/mtproto"), submodules=["details", "web_proxy"])

    # 6. gpui_telegram_proto api
    generate_mod_rs(os.path.join(REPO_ROOT, "crates/gpui_telegram_proto/src/api"))

    # Update gpui_telegram_core/src/lib.rs
    core_lib_path = os.path.join(REPO_ROOT, "crates/gpui_telegram_core/src/lib.rs")
    with open(core_lib_path, "r", encoding="utf-8") as f:
        core_lib = f.read()
    if "pub mod core;" not in core_lib:
        core_lib += "\npub mod core;\npub mod mtproto;\n"
        with open(core_lib_path, "w", encoding="utf-8") as f:
            f.write(core_lib)
        print("Updated crates/gpui_telegram_core/src/lib.rs")

    # Update gpui_telegram_proto/src/lib.rs
    proto_lib_path = os.path.join(REPO_ROOT, "crates/gpui_telegram_proto/src/lib.rs")
    with open(proto_lib_path, "r", encoding="utf-8") as f:
        proto_lib = f.read()
    if "pub mod api;" not in proto_lib:
        proto_lib += "\npub mod api;\npub use api::*;\n"
        with open(proto_lib_path, "w", encoding="utf-8") as f:
            f.write(proto_lib)
        print("Updated crates/gpui_telegram_proto/src/lib.rs")

if __name__ == "__main__":
    main()
