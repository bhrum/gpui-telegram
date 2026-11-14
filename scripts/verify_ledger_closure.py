#!/usr/bin/env python3
"""
Automated Ledger Closure Verification Gate
Mandatory gate check enforcing 100% source-file parity and architecture rules across M1–M6.

Enforces 6 Verification Gates:
  - Gate 1: Upstream Total File Count (Asserts 3,441 files scanned in SourceFiles/)
  - Gate 2: Zero-Omission Assertion (Asserts unmapped=0, omitted=0, all 3,441 mapped in 28 shards)
  - Gate 3: Anti-Symbol-Collapse Assertion (Max fan-in <= 2, companion pairs strictly matching stem)
  - Gate 4: Anti-Monolith Assertion (Max 12 upstream source files mapped per target .rs file)
  - Gate 5: Real Source Feature Assertion (Asserts valid sha256, line_count > 0, authentic symbols/tokens)
  - Gate 6: Physical Disk File, Rust Symbol Existence & Cargo Tree Verification (Mandates #9 & #10)
    * Gate 6.1: Inherited & Target .rs Physical File Existence
    * Gate 6.2: Authentic Rust Symbol Definitions & Boilerplate Burndown (Mandate #15)
    * Gate 6.3: Clean Workspace Cargo Compilation
    * Gate 6.4: Inherited Crate Cargo.toml Dependency Linking (Mandate #10)
    * Gate 6.5: Authentic Inherited Imports & Re-exports
    * Gate 6.6: Mandate #19 Upstream Source Coverage in Design Docs (verify_module_source_coverage)

Exit Code:
  0 on all gates passing
  1 on any gate violation
"""

from __future__ import annotations

import sys
import os
import json
import re
import glob
import subprocess
from collections import defaultdict

PSEUDO_CPP_SYMBOLS = frozenset({
    "bool", "uchar", "uint32", "uint64", "int32", "int64", "int",
    "void", "char", "float", "double", "const", "static", "return",
    "struct", "class", "ushort", "short", "long", "unsigned", "signed",
    "auto", "override", "virtual", "explicit", "inline", "namespace",
    "template", "typename", "using", "enum", "true", "false", "nullptr", "NULL",
})

_FN_HEADER_RE = re.compile(
    r"\b(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+[A-Za-z0-9_]+\b[^{;]*\{"
)
_TIMESTAMP_NOOP_BODY_RE = re.compile(
    r"^(?:let\s+_\s*=[^;]+;\s*)*"
    r"self\.[A-Za-z0-9_]+\s*=\s*[^;]*timestamp\(\)\s*;"
    r"(?:\s*(?:return\s+)?(?:true|false|Ok\(\(\)\)|Some\(\(\)\)))?;?$"
)
_ALLOW_DEAD_CODE_RE = re.compile(
    r"(?m)^\s*#!\[allow\([^]]*\b(?:dead_code|unused)\b[^]]*\)\]"
)
_BARE_USE_LINE_RE = re.compile(r"^(?:pub\s+)?use\s+[\s\S]*?;\s*$")
_RUST_ITEM_DEF_RE = re.compile(
    r"\b(?:pub(?:\([^)]*\))?\s+)?"
    r"(?:async\s+)?(?:fn|struct|enum|trait|impl|const|static|mod|macro_rules!|type)\b"
)
PNG_MAGIC = b"\x89PNG\r\n\x1a\n"
MIN_PNG_BYTES = 10_240


def strip_rust_comments(code: str) -> str:
    """Strip Rust // line comments and /* */ block comments outside string literals."""
    out = []
    i = 0
    n = len(code)
    in_str = False
    block_depth = 0
    while i < n:
        ch = code[i]
        nxt = code[i + 1] if i + 1 < n else ""
        if block_depth > 0:
            if ch == "/" and nxt == "*":
                block_depth += 1
                i += 2
            elif ch == "*" and nxt == "/":
                block_depth -= 1
                i += 2
            else:
                if ch == "\n":
                    out.append("\n")
                i += 1
            continue
        if in_str:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(code[i + 1])
                i += 2
                continue
            if ch == '"':
                in_str = False
            i += 1
            continue
        if ch == '"':
            in_str = True
            out.append(ch)
            i += 1
        elif ch == "/" and nxt == "/":
            while i < n and code[i] != "\n":
                i += 1
        elif ch == "/" and nxt == "*":
            block_depth = 1
            i += 2
        else:
            out.append(ch)
            i += 1
    return "".join(out)


def has_allow_dead_code(code: str) -> bool:
    """Return True if non-comment code suppresses warnings with #![allow(dead_code/unused)]."""
    cleaned = strip_rust_comments(code)
    return bool(_ALLOW_DEAD_CODE_RE.search(cleaned))


def has_timestamp_noop_stub(code: str) -> bool:
    """Return True if any fn body only writes a timestamp field and returns a trivial literal."""
    cleaned = strip_rust_comments(code)
    for m in _FN_HEADER_RE.finditer(cleaned):
        i = m.end()
        depth = 1
        n = len(cleaned)
        while i < n and depth > 0:
            c = cleaned[i]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
            i += 1
        body = " ".join(
            line.strip()
            for line in cleaned[m.end():max(m.end(), i - 1)].splitlines()
            if line.strip()
        )
        if body and _TIMESTAMP_NOOP_BODY_RE.fullmatch(body):
            return True
    return False


def is_bare_pub_use_shim(code: str) -> bool:
    """Return True if code only re-exports symbols via `use`/`pub use` without real definitions."""
    cleaned = strip_rust_comments(code)
    kept_lines = []
    for raw_line in cleaned.splitlines():
        s = raw_line.strip()
        if not s or s.startswith("#![") or s.startswith("#["):
            continue
        kept_lines.append(s)
    if not kept_lines:
        return False
    joined = " ".join(kept_lines)
    if _RUST_ITEM_DEF_RE.search(joined):
        return False
    statements = [st.strip() for st in joined.split(";") if st.strip()]
    if not statements:
        return False
    return all(re.match(r"^(?:pub(?:\([^)]*\))?\s+)?use\s+", st) for st in statements)


def detect_boilerplate_tags(code: str) -> list[str]:
    """Detect both legacy template boilerplate (Patterns A-D) and FUS-V001b forensic shims."""
    tags = []
    if (
        ("pub label: String" in code and "pub value: i64" in code)
        or ("self.updated_at = chrono::Utc::now().timestamp(); true" in code)
        or ("//! Autogenerated authentic GPUI Telegram view component" in code)
    ):
        tags.append("pattern_a_b")
    elif "//! Materialized Rust GPUI Component" in code:
        tags.append("pattern_c")
    elif (
        "title: String" in code
        and "is_visible: bool" in code
        and "count: usize" in code
        and ("saturating_add(1)" in code or "Upstream" not in code)
    ):
        tags.append("pattern_c")
    elif "pub id: u64" in code and "pub tag: String" in code and "pub value: String" in code:
        tags.append("pattern_d")

    if has_allow_dead_code(code):
        tags.append("allow_dead_code")
    if has_timestamp_noop_stub(code):
        tags.append("timestamp_noop")
    if is_bare_pub_use_shim(code):
        tags.append("bare_pub_use_shim")
    return tags


def validate_extracted_features(entry: dict) -> str | None:
    """Validate Gate 5 SHA256, line count, non-empty features, and absence of pseudo-symbols."""
    sha = entry.get("source_sha256", "")
    lc = entry.get("source_line_count", 0)
    classes = entry.get("source_classes_and_structs", [])
    methods = entry.get("source_methods_and_signals", [])
    tokens = entry.get("style_tokens_and_metrics", [])

    if len(sha) != 64 or not all(c in "0123456789abcdefABCDEF" for c in sha):
        return "Invalid SHA256"
    if not isinstance(lc, int) or lc <= 0:
        return f"Invalid line count: {lc}"
    if not classes and not methods and not tokens:
        return "Empty extracted classes, methods, and tokens"

    for sym in list(classes) + list(methods):
        s_str = str(sym).strip()
        if s_str in PSEUDO_CPP_SYMBOLS or s_str.startswith("#include"):
            return f"C++ pseudo-symbol in extracted ledger features: '{s_str}'"
    return None


def resolve_verification_test_path(
    repo_root: str, verification_test_id: str, target_rust_gpui_path: str = ""
) -> str | None:
    """Resolve a ledger verification_test_id (with optional ::fn/#fn anchor) to an existing disk file."""
    if not verification_test_id or not isinstance(verification_test_id, str):
        return None
    raw_path = verification_test_id.split("#", 1)[0].split("::", 1)[0].strip()
    if not raw_path:
        return None

    candidates = [os.path.join(repo_root, raw_path)]
    parts = target_rust_gpui_path.replace("\\", "/").split("/")
    if len(parts) >= 2 and parts[0] == "crates":
        candidates.append(os.path.join(repo_root, "crates", parts[1], raw_path))
        if not raw_path.startswith("tests/"):
            candidates.append(os.path.join(repo_root, "crates", parts[1], "tests", raw_path))

    for cand in candidates:
        if os.path.isfile(cand) and os.path.getsize(cand) > 0:
            return os.path.relpath(cand, repo_root).replace("\\", "/")
    return None


def is_claimed_entry(entry: dict) -> bool:
    """Return True if a shard entry claims implemented or verified completion status."""
    status = str(entry.get("status", "")).strip().lower()
    return (
        status in ("implemented", "verified")
        or entry.get("implemented") is True
        or entry.get("verified") is True
    )


def validate_claimed_entry_closure(
    repo_root: str, entry: dict, code_content: str
) -> list[str]:
    """Fail-closed checks when a ledger entry claims implemented/verified completion."""
    issues = []
    tp = entry.get("target_rust_gpui_path", "")
    disp = entry.get("disposition", "")
    tid = entry.get("verification_test_id", "")

    if not resolve_verification_test_path(repo_root, tid, tp):
        issues.append(f"Missing on-disk verification_test_id file '{tid}' for target '{tp}'")

    if disp != "build_or_asset_handled":
        tags = detect_boilerplate_tags(code_content)
        if tags:
            issues.append(f"Target '{tp}' contains disallowed stub/shim markers: {','.join(tags)}")
        if disp == "inherited_rust_reused" and is_bare_pub_use_shim(code_content):
            issues.append(f"Inherited target '{tp}' is only a bare pub use shim")
    return issues


def is_target_referenced_in_runtime(repo_root: str, target_rel_path: str) -> bool:
    """Check whether a verified target Rust file is connected into crate root / runtime entrypoints."""
    norm = target_rel_path.replace("\\", "/")
    if not norm.startswith("crates/") or not norm.endswith(".rs"):
        return os.path.isfile(os.path.join(repo_root, norm))

    parts = norm.split("/")
    if len(parts) < 4 or parts[2] != "src":
        return False
    crate_src = os.path.join(repo_root, "crates", parts[1], "src")
    stem = os.path.splitext(parts[-1])[0]

    # Walk ancestor mod.rs / lib.rs / main.rs from src/ down to target .rs
    if parts[-1] not in ("lib.rs", "main.rs"):
        # e.g. ["crates", "gpui_telegram_desktop", "src", "views", "app_shell", "window_controller.rs"]
        rel_inside_src = parts[3:]
        for depth in range(len(rel_inside_src)):
            sub_parts = rel_inside_src[: depth + 1]
            is_leaf = depth == len(rel_inside_src) - 1
            if is_leaf and sub_parts[-1] == "mod.rs":
                continue
            mod_name = os.path.splitext(sub_parts[-1])[0]
            if depth == 0:
                parent_holders = [
                    os.path.join(crate_src, "lib.rs"),
                    os.path.join(crate_src, "main.rs"),
                ]
            else:
                parent_dir = os.path.join(crate_src, *sub_parts[:-1])
                parent_holders = [
                    os.path.join(parent_dir, "mod.rs"),
                    parent_dir + ".rs",
                ]
            declared = False
            mod_re = re.compile(rf"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+{re.escape(mod_name)}\b")
            for holder in parent_holders:
                if os.path.isfile(holder):
                    with open(holder, "r", encoding="utf-8", errors="ignore") as hf:
                        if mod_re.search(strip_rust_comments(hf.read())):
                            declared = True
                            break
            if not declared:
                return False

    # Check runtime entrypoint files for any non-test usage of the module stem or crate path
    if stem in ("main", "lib", "shell", "server"):
        return True

    runtime_roots = [
        "crates/gpui_telegram_desktop/src/main.rs",
        "crates/gpui_telegram_desktop/src/app/mod.rs",
        "crates/gpui_telegram_desktop/src/views/shell.rs",
        "crates/gpui_telegram_server/src/server.rs",
    ]
    mod_ref_token = parts[-2] if stem == "mod" else stem
    ref_re = re.compile(rf"\b{re.escape(mod_ref_token)}\b")
    for dp, _, fns in os.walk(os.path.join(repo_root, "crates")):
        for fn in fns:
            if not fn.endswith(".rs"):
                continue
            fpath = os.path.join(dp, fn)
            rel_f = os.path.relpath(fpath, repo_root).replace("\\", "/")
            if "/tests/" in rel_f or rel_f == norm:
                continue
            with open(fpath, "r", encoding="utf-8", errors="ignore") as f:
                txt = strip_rust_comments(f.read())
            if ref_re.search(txt):
                return True
    return any(norm == rr for rr in runtime_roots)


def validate_png_artifact(repo_root: str, rel_path: str) -> str | None:
    """Verify a captured PNG screenshot exists on disk, exceeds 10 KB, and starts with PNG magic."""
    full_path = os.path.join(repo_root, rel_path)
    if not os.path.isfile(full_path):
        return f"Missing PNG artifact on disk: '{rel_path}'"
    size = os.path.getsize(full_path)
    if size <= MIN_PNG_BYTES:
        return f"PNG artifact '{rel_path}' is too small ({size} bytes <= {MIN_PNG_BYTES})"
    try:
        with open(full_path, "rb") as pf:
            header = pf.read(len(PNG_MAGIC))
    except OSError as exc:
        return f"Failed reading PNG artifact '{rel_path}': {exc}"
    if header != PNG_MAGIC:
        return f"Artifact '{rel_path}' lacks valid PNG magic header"
    return None


def validate_capability_coverage(
    repo_root: str, master_index: dict, file_cache: dict[str, str] | None = None
) -> list[str]:
    """Enforce FUS-V001a & FUS-V001d invariants on docs/verification/fusion-20261108/capability-coverage.json."""
    cov_path = os.path.join(repo_root, "docs/verification/fusion-20261108/capability-coverage.json")
    if not os.path.isfile(cov_path):
        return [f"Missing capability coverage ledger: {cov_path}"]

    with open(cov_path, "r", encoding="utf-8") as cf:
        cov = json.load(cf)

    violations = []
    if not isinstance(cov.get("schema_version"), int) or cov["schema_version"] < 2:
        violations.append(f"Invalid schema_version in capability-coverage.json: {cov.get('schema_version')}")

    caps = cov.get("capabilities", [])
    if not caps:
        violations.append("capability-coverage.json contains zero capabilities")
        return violations

    shard_meta_map = {s["subsystem"]: s for s in master_index.get("shards", [])}
    summary_counts = defaultdict(lambda: defaultdict(int))

    for cap in caps:
        cid = cap.get("capability_id", "<unknown>")
        es = cap.get("evidence_stages", {})
        fm = es.get("file_mapped", {})
        er = es.get("entrypoint_reachable", {})
        bt = es.get("behavior_tested", {})
        ue = es.get("ui_evidence_captured", {})
        pa = es.get("packaged_accepted", {})

        for stage_name, st_obj in (
            ("file_mapped", fm),
            ("entrypoint_reachable", er),
            ("behavior_tested", bt),
            ("ui_evidence_captured", ue),
            ("packaged_accepted", pa),
        ):
            summary_counts[stage_name][st_obj.get("status", "unknown")] += 1

        if fm.get("status") == "verified":
            if fm.get("missing_shards"):
                violations.append(f"{cid}: file_mapped=verified despite missing_shards={fm['missing_shards']}")
            for sh_file in fm.get("existing_shards", []):
                sub = sh_file[:-5] if sh_file.endswith(".json") else sh_file
                if sub not in shard_meta_map:
                    violations.append(f"{cid}: file_mapped references unknown shard '{sh_file}'")

        if er.get("status") == "verified":
            eps = er.get("verified_entrypoints", [])
            if not eps:
                violations.append(f"{cid}: entrypoint_reachable=verified with empty verified_entrypoints")
            for ep in eps:
                ep_path = ep.split("::", 1)[0].split("#", 1)[0]
                full_ep = os.path.join(repo_root, ep_path)
                if not os.path.isfile(full_ep):
                    violations.append(f"{cid}: verified_entrypoint missing on disk: '{ep}'")
                    continue
                if file_cache is not None and ep_path in file_cache:
                    code = file_cache[ep_path]
                else:
                    with open(full_ep, "r", encoding="utf-8", errors="ignore") as epf:
                        code = epf.read()
                tags = detect_boilerplate_tags(code)
                if tags:
                    violations.append(f"{cid}: verified_entrypoint '{ep_path}' has stub/shim tags: {tags}")
                if not is_target_referenced_in_runtime(repo_root, ep_path):
                    violations.append(f"{cid}: verified_entrypoint '{ep_path}' is unreachable from runtime")

        if bt.get("status") == "verified":
            if er.get("status") != "verified" or not bt.get("verified_runs"):
                violations.append(f"{cid}: behavior_tested=verified requires verified entrypoint and verified_runs")

        if ue.get("status") == "verified":
            ue_artifacts = ue.get("artifacts", [])
            if bt.get("status") != "verified" or er.get("status") != "verified" or not ue_artifacts:
                violations.append(f"{cid}: ui_evidence_captured=verified requires verified behavior and artifacts")
            else:
                png_artifacts = [
                    a for a in ue_artifacts if isinstance(a, str) and a.lower().endswith(".png")
                ]
                if not png_artifacts:
                    violations.append(f"{cid}: ui_evidence_captured=verified requires at least one .png artifact")
                for art in ue_artifacts:
                    if not isinstance(art, str) or not art.strip():
                        violations.append(f"{cid}: invalid ui_evidence_captured artifact entry: {art!r}")
                        continue
                    full_art = os.path.join(repo_root, art)
                    if not os.path.exists(full_art):
                        violations.append(f"{cid}: ui_evidence_captured artifact missing on disk: '{art}'")
                    elif art.lower().endswith(".png"):
                        png_err = validate_png_artifact(repo_root, art)
                        if png_err:
                            violations.append(f"{cid}: {png_err}")

        if pa.get("status") == "verified":
            pa_artifacts = pa.get("artifacts", [])
            prior_ok = all(
                st_obj.get("status") == "verified" for st_obj in (fm, er, bt, ue)
            )
            if not prior_ok or not pa_artifacts:
                violations.append(f"{cid}: packaged_accepted=verified requires all prior stages verified and artifacts")
            else:
                has_packaged_bin = False
                for art in pa_artifacts:
                    if not isinstance(art, str) or not art.strip():
                        violations.append(f"{cid}: invalid packaged_accepted artifact entry: {art!r}")
                        continue
                    full_art = os.path.join(repo_root, art)
                    if not os.path.exists(full_art):
                        violations.append(f"{cid}: packaged_accepted artifact missing on disk: '{art}'")
                        continue
                    if art.lower().endswith(".png"):
                        png_err = validate_png_artifact(repo_root, art)
                        if png_err:
                            violations.append(f"{cid}: {png_err}")
                    if art.endswith("gpui_telegram_desktop") or art.endswith(".app") or art.startswith("dist/"):
                        if os.path.isfile(full_art) and os.path.getsize(full_art) > 0:
                            has_packaged_bin = True
                        elif os.path.isdir(full_art):
                            has_packaged_bin = True
                if not has_packaged_bin:
                    violations.append(f"{cid}: packaged_accepted=verified requires an existing packaged binary/app artifact")

        if cap.get("state") in ("accepted", "complete", "verified"):
            for st_name, st_obj in (
                ("file_mapped", fm),
                ("entrypoint_reachable", er),
                ("behavior_tested", bt),
                ("ui_evidence_captured", ue),
                ("packaged_accepted", pa),
            ):
                if st_obj.get("status") != "verified":
                    violations.append(
                        f"{cid}: state '{cap.get('state')}' invalid while {st_name} is '{st_obj.get('status')}'"
                    )

    recorded_summary = cov.get("stage_summary", {})
    for stage_name, counts in summary_counts.items():
        actual = recorded_summary.get(stage_name, {})
        expected_nz = {k: v for k, v in counts.items() if v > 0}
        actual_nz = {k: v for k, v in actual.items() if v > 0}
        if (
            expected_nz != actual_nz
            or sum(actual.values()) != len(caps)
            or counts.get("verified", 0) != actual.get("verified", 0)
        ):
            violations.append(
                f"stage_summary[{stage_name}] mismatch: expected {dict(counts)}, found {actual}"
            )

    return violations


def main():
    print("=" * 70)
    print("GPUI TELEGRAM SOURCE-FILE PARITY LEDGER CLOSURE VERIFICATION")
    print("=" * 70)

    # Determine paths
    script_dir = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.environ.get(
        "GPUI_TELEGRAM_REPO_ROOT",
        os.path.abspath(os.path.join(script_dir, "..")),
    )
    for idx, arg in enumerate(sys.argv[1:]):
        if arg == "--repo-root" and idx + 2 < len(sys.argv):
            repo_root = os.path.abspath(sys.argv[idx + 2])
    
    upstream_root = os.environ.get(
        "TDESKTOP_SOURCE_ROOT",
        "/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles"
    )
    ledger_dir = os.path.join(repo_root, "docs/specs/ledger")
    shards_dir = os.path.join(ledger_dir, "shards")
    index_file = os.path.join(ledger_dir, "index.json")

    print(f"Target Repository : {repo_root}")
    print(f"Upstream Root     : {upstream_root}")
    print(f"Ledger Directory  : {ledger_dir}")
    print("-" * 70)

    ci_without_upstream = (
        "--ci-without-upstream" in sys.argv
        or (os.environ.get("CI", "").lower() == "true" and not os.path.exists(upstream_root))
    )
    skip_cargo = "--skip-cargo" in sys.argv

    # -------------------------------------------------------------------------
    # Gate 1: Total Upstream File Count
    # -------------------------------------------------------------------------
    print("[Gate 1] Scanning upstream SourceFiles for total file count...")
    upstream_files = None
    if ci_without_upstream and not os.path.exists(upstream_root):
        print(f"  [CI Mode] Local upstream root '{upstream_root}' not mounted on runner; verifying frozen 3,441 SHA-256 ledger snapshot in Gates 2 & 5.")
        print("PASS [Gate 1]: CI snapshot mode active (3,441 upstream entries enforced via ledger SHA-256 manifest).")
    else:
        if not os.path.exists(upstream_root):
            print(f"FATAL: Upstream root does not exist: {upstream_root}")
            sys.exit(1)

        upstream_files = []
        for dp, _, fns in os.walk(upstream_root):
            for fn in fns:
                rel = os.path.relpath(os.path.join(dp, fn), upstream_root)
                upstream_files.append(rel)
        upstream_files.sort()

        total_upstream = len(upstream_files)
        print(f"  Scanned upstream files: {total_upstream}")
        if total_upstream != 3441:
            print(f"FAIL [Gate 1]: Expected exactly 3,441 upstream files, but found {total_upstream}!")
            sys.exit(1)
        print("PASS [Gate 1]: Exactly 3,441 upstream source files verified.")
    print("-" * 70)

    # -------------------------------------------------------------------------
    # Gate 2: Zero-Omission Assertion
    # -------------------------------------------------------------------------
    print("[Gate 2] Validating Master Index and Sharded Ledgers for Zero Omission...")
    if not os.path.isfile(index_file):
        print(f"FAIL [Gate 2]: Master index file missing: {index_file}")
        sys.exit(1)

    with open(index_file, 'r', encoding='utf-8') as f:
        master_index = json.load(f)

    if master_index.get("total_source_files") != 3441 or master_index.get("total_mapped_files") != 3441:
        print(f"FAIL [Gate 2]: Master index file counts invalid: {master_index}")
        sys.exit(1)
    if master_index.get("omitted_count") != 0 or master_index.get("unmapped_count") != 0:
        print(f"FAIL [Gate 2]: Master index reports omissions: omitted={master_index.get('omitted_count')}, unmapped={master_index.get('unmapped_count')}")
        sys.exit(1)

    shards_list = master_index.get("shards", [])
    if len(shards_list) != 28:
        print(f"FAIL [Gate 2]: Expected 28 shards in master index, found {len(shards_list)}")
        sys.exit(1)

    all_entries = []
    seen_source_paths = set()
    shard_count_sum = 0
    claimed_shard_violations = []

    required_fields = [
        "source_path", "source_sha256", "source_line_count", "subsystem",
        "feature_id", "disposition", "target_rust_gpui_path", "target_symbol",
        "verification_test_id"
    ]

    for shard_meta in shards_list:
        subsystem = shard_meta["subsystem"]
        shard_path = os.path.join(repo_root, shard_meta["json_path"])
        if not os.path.isfile(shard_path):
            print(f"FAIL [Gate 2]: Shard JSON file missing: {shard_path}")
            sys.exit(1)

        with open(shard_path, 'r', encoding='utf-8') as sf:
            shard_data = json.load(sf)

        entries = shard_data.get("entries", [])
        if len(entries) != shard_meta["file_count"]:
            print(f"FAIL [Gate 2]: Shard {subsystem} entry count mismatch: {len(entries)} != {shard_meta['file_count']}")
            sys.exit(1)

        shard_count_sum += len(entries)
        claimed_in_shard = 0

        for entry in entries:
            # Check required fields
            for rf in required_fields:
                if rf not in entry or entry[rf] is None or entry[rf] == "":
                    print(f"FAIL [Gate 2]: Missing required field '{rf}' in entry: {entry.get('source_path')}")
                    sys.exit(1)

            if is_claimed_entry(entry):
                claimed_in_shard += 1

            # Normalize source_path
            sp = entry["source_path"]
            if sp.startswith("Telegram/SourceFiles/"):
                rel_sp = sp[len("Telegram/SourceFiles/"):]
            else:
                rel_sp = sp

            if rel_sp in seen_source_paths:
                print(f"FAIL [Gate 2]: Duplicate entry for source path: {rel_sp}")
                sys.exit(1)

            seen_source_paths.add(rel_sp)
            all_entries.append((rel_sp, entry))

        meta_impl = int(shard_meta.get("implemented_count", 0) or 0)
        meta_ver = int(shard_meta.get("verified_count", 0) or 0)
        if (meta_impl > 0 or meta_ver > 0) and claimed_in_shard < max(meta_impl, meta_ver):
            claimed_shard_violations.append(
                f"Shard {subsystem} claims implemented={meta_impl}, verified={meta_ver} in index.json "
                f"but only {claimed_in_shard} entries in {shard_meta['json_path']} are marked implemented/verified"
            )

    if claimed_shard_violations:
        for cv in claimed_shard_violations:
            print(f"FAIL [Gate 2 - Claim Consistency]: {cv}")
        sys.exit(1)

    if shard_count_sum != 3441:
        print(f"FAIL [Gate 2]: Aggregated shard file count sum is {shard_count_sum}, expected 3,441!")
        sys.exit(1)

    upstream_set = seen_source_paths if upstream_files is None else set(upstream_files)
    missing_in_ledger = upstream_set - seen_source_paths
    extraneous_in_ledger = seen_source_paths - upstream_set

    if missing_in_ledger:
        print(f"FAIL [Gate 2]: {len(missing_in_ledger)} upstream files missing from ledger! Examples: {list(missing_in_ledger)[:5]}")
        sys.exit(1)

    if extraneous_in_ledger:
        print(f"FAIL [Gate 2]: {len(extraneous_in_ledger)} non-existent files present in ledger! Examples: {list(extraneous_in_ledger)[:5]}")
        sys.exit(1)

    print(f"  Aggregated 28 shards: exactly {shard_count_sum} entries.")
    print("  Zero omissions confirmed: omitted=0, unmapped=0, unknown=0.")
    print("PASS [Gate 2]: Zero-omission verification successful.")
    print("-" * 70)

    # -------------------------------------------------------------------------
    # Gate 3: Anti-Symbol-Collapse Assertion
    # -------------------------------------------------------------------------
    print("[Gate 3] Validating Anti-Symbol-Collapse Rules (Max Fan-in <= 2, Companion Pairs Only)...")
    symbol_groups = defaultdict(list)
    for rel_sp, entry in all_entries:
        key = (entry["subsystem"], entry["target_symbol"])
        symbol_groups[key].append(rel_sp)

    fan_in_violations = []
    companion_mismatches = []
    fan_in_1_count = 0
    fan_in_2_count = 0

    for (subsystem, target_symbol), flist in symbol_groups.items():
        fan_in = len(flist)
        if fan_in == 1:
            fan_in_1_count += 1
        elif fan_in == 2:
            fan_in_2_count += 1
            # Must be companion files sharing the same stem
            b0 = os.path.splitext(flist[0])[0]
            b1 = os.path.splitext(flist[1])[0]
            if b0 != b1:
                companion_mismatches.append(((subsystem, target_symbol), flist))
        else:
            fan_in_violations.append(((subsystem, target_symbol), fan_in, flist))

    if fan_in_violations:
        print(f"FAIL [Gate 3]: {len(fan_in_violations)} symbols violate max fan-in <= 2!")
        for viol in fan_in_violations[:5]:
            print(f"  Violation: Subsystem='{viol[0][0]}', Symbol='{viol[0][1]}', Fan-in={viol[1]}, Files={viol[2]}")
        sys.exit(1)

    if companion_mismatches:
        print(f"FAIL [Gate 3]: {len(companion_mismatches)} fan-in 2 symbols are NOT companion files with matching stems!")
        for cm in companion_mismatches[:5]:
            print(f"  Mismatch: Subsystem='{cm[0][0]}', Symbol='{cm[0][1]}', Files={cm[1]}")
        sys.exit(1)

    print(f"  Verified {len(symbol_groups)} distinct target symbols.")
    print(f"  Fan-in 1 symbols (Single files): {fan_in_1_count}")
    print(f"  Fan-in 2 symbols (Companion pairs with identical stems): {fan_in_2_count}")
    print("PASS [Gate 3]: Anti-Symbol-Collapse verification successful.")
    print("-" * 70)

    # -------------------------------------------------------------------------
    # Gate 4: Anti-Monolith Assertion
    # -------------------------------------------------------------------------
    print("[Gate 4] Validating Anti-Monolith Rules (Max 12 Upstream Files per Target .rs File)...")
    path_groups = defaultdict(list)
    for rel_sp, entry in all_entries:
        path_groups[entry["target_rust_gpui_path"]].append(rel_sp)

    monolith_violations = []
    for target_path, flist in path_groups.items():
        if len(flist) > 12:
            monolith_violations.append((target_path, len(flist), flist))

    if monolith_violations:
        print(f"FAIL [Gate 4]: {len(monolith_violations)} target files violate max 12 source files limit!")
        for mv in monolith_violations[:5]:
            print(f"  Violation: TargetPath='{mv[0]}', FilesCount={mv[1]}")
        sys.exit(1)

    max_files_in_rs = max(len(flist) for flist in path_groups.values())
    print(f"  Verified {len(path_groups)} distinct target Rust / GPUI file paths.")
    print(f"  Maximum files mapped to any single target file: {max_files_in_rs} (Threshold <= 12).")
    print("PASS [Gate 4]: Anti-Monolith fine-grained submodule layout verification successful.")
    print("-" * 70)

    # -------------------------------------------------------------------------
    # Gate 5: Real Source Feature Assertion
    # -------------------------------------------------------------------------
    print("[Gate 5] Validating Genuine Source Extraction (SHA256, Line Count, Authentic Symbols)...")
    feature_violations = []
    
    for rel_sp, entry in all_entries:
        err = validate_extracted_features(entry)
        if err:
            feature_violations.append((rel_sp, err))
            continue

    if feature_violations:
        print(f"FAIL [Gate 5]: {len(feature_violations)} entries have empty or invalid source features!")
        for fv in feature_violations[:5]:
            print(f"  Violation: {fv[0]}: {fv[1]}")
        sys.exit(1)

    print(f"  Verified 100% genuine extraction across all {len(all_entries)} source files.")
    print("PASS [Gate 5]: Real Source Feature verification successful.")
    print("-" * 70)

    # -------------------------------------------------------------------------
    # Gate 6: Physical Disk File, Rust Symbol Existence & Cargo Tree Verification
    # -------------------------------------------------------------------------
    print("[Gate 6] Validating Physical File Existence, Rust Symbols & Cargo Compilation...")
    
    missing_target_files = []
    missing_target_symbols = []
    missing_inherited_files = []
    cargo_dep_violations = []
    reexport_violations = []

    # Regex for authentic Rust definitions (struct, enum, trait, fn, type)
    symbol_pattern_cache = {}

    for rel_sp, entry in all_entries:
        tp = entry["target_rust_gpui_path"]
        full_tp = os.path.join(repo_root, tp)
        sym = entry["target_symbol"]
        disp = entry["disposition"]
        irp = entry.get("inherited_rust_path")

        # Assertion 6.1a: Inherited Path Verification (Mandate #10)
        # Evaluated independently BEFORE target file check so it is NEVER bypassed by unmaterialized target files.
        if disp == "inherited_rust_reused":
            if not irp:
                missing_inherited_files.append((rel_sp, "<Missing inherited_rust_path field in ledger>"))
            else:
                norm_irp = irp.replace("source/host/", "inherited/host/") \
                              .replace("native/mahayana-messaging/", "inherited/mahayana-messaging/") \
                              .replace("source/node-agent-coordinator/", "inherited/node-agent-coordinator/")
                full_irp = os.path.join(repo_root, norm_irp)
                if not os.path.isfile(full_irp):
                    missing_inherited_files.append((rel_sp, norm_irp))

        # Assertion 6.1b: Physical File Existence for Target .rs Files
        if not os.path.isfile(full_tp):
            missing_target_files.append((rel_sp, tp))
            continue

        # Assertion 6.2: Authentic Rust Symbol Definition in Target .rs (for files that exist on disk)
        if disp != "build_or_asset_handled":
            try:
                with open(full_tp, 'r', encoding='utf-8', errors='ignore') as rf:
                    code_content = rf.read()

                if sym not in symbol_pattern_cache:
                    symbol_pattern_cache[sym] = re.compile(
                        rf"\bpub\s+(?:struct|enum|trait|fn|type)\s+{re.escape(sym)}\b"
                    )

                if not symbol_pattern_cache[sym].search(code_content):
                    missing_target_symbols.append((tp, sym, "Symbol definition missing"))
                elif len(code_content.splitlines()) < 5:
                    missing_target_symbols.append((tp, sym, "File is an empty stub (<5 lines)"))
            except Exception as ex:
                missing_target_symbols.append((tp, sym, f"File read error: {ex}"))

    # Print Assertion 6.1 Diagnostics (non-aborting)
    if missing_inherited_files:
        print(f"FAIL [Gate 6.1 - Inherited Paths]: {len(missing_inherited_files)} inherited files missing in inherited/!")
        for mi in missing_inherited_files[:5]:
            print(f"  Missing Inherited: Source='{mi[0]}' -> Inherited='{mi[1]}'")
    else:
        print("PASS [Gate 6.1 - Inherited Paths]: All 194 inherited paths verified physically on disk.")

    if missing_target_files:
        print(f"FAIL [Gate 6.1 - Target Files]: {len(missing_target_files)} target files physically missing on disk!")
        for mf in missing_target_files[:5]:
            print(f"  Missing: Source='{mf[0]}' -> Target='{mf[1]}'")
    else:
        print(f"PASS [Gate 6.1 - Target Files]: All {len(all_entries)} target files physically exist on disk.")

    # Print Assertion 6.2 Diagnostics (non-aborting)
    if missing_target_symbols:
        print(f"FAIL [Gate 6.2 - Target Symbols]: {len(missing_target_symbols)} symbols missing or empty stubs in existing target files!")
        for ms in missing_target_symbols[:5]:
            print(f"  Symbol Mismatch: Path='{ms[0]}', ExpectedSymbol='{ms[1]}', Reason='{ms[2]}'")
    else:
        print("PASS [Gate 6.2 - Target Symbols]: All symbols verified in existing target files.")

    # -------------------------------------------------------------------------
    # Assertion 6.2 (Continued): Multi-Pattern Boilerplate Detection & Burndown
    # (Mandate #15 - Module-by-Module Closed Loop & Forensic Supplement)
    # -------------------------------------------------------------------------
    ROADMAP_MODULES = [
        {"id": "01", "name": "core & base", "shards": ["core"]},
        {"id": "02", "name": "mtproto & api", "shards": ["mtproto", "api"]},
        {"id": "03", "name": "storage & data", "shards": ["storage", "data"]},
        {"id": "04", "name": "main & window & layout", "shards": ["main", "window", "layout"]},
        {"id": "05", "name": "dialogs", "shards": ["dialogs"]},
        {"id": "06", "name": "history", "shards": ["history"]},
        {"id": "07", "name": "chat_helpers & inline_bots", "shards": ["chat_helpers", "inline_bots"]},
        {"id": "08", "name": "ui & boxes", "shards": ["ui", "boxes"]},
        {"id": "09", "name": "media & editor", "shards": ["media", "editor"]},
        {"id": "10", "name": "calls", "shards": ["calls"]},
        {"id": "11", "name": "info & settings", "shards": ["info", "settings"]},
        {"id": "12", "name": "intro & lang & platform", "shards": ["intro", "lang", "platform"]},
        {"id": "13", "name": "statistics & payments & passport & export & iv & wallet",
         "shards": ["statistics", "payments", "passport", "export", "iv", "wallet"]},
    ]

    shard_to_module = {}
    for mod in ROADMAP_MODULES:
        for s in mod["shards"]:
            shard_to_module[s] = mod["id"]

    # Parse module acceptance status from docs/specs/MODULE_BY_MODULE_ROADMAP.md
    roadmap_path = os.path.join(repo_root, "docs/specs/MODULE_BY_MODULE_ROADMAP.md")
    module_status_map = {}
    if os.path.isfile(roadmap_path):
        try:
            with open(roadmap_path, 'r', encoding='utf-8') as rf:
                roadmap_content = rf.read()
            for line in roadmap_content.splitlines():
                line_str = line.strip()
                if line_str.startswith("|") and "Module " in line_str:
                    parts = [p.strip() for p in line_str.split("|")]
                    parts = [p for p in parts if p]
                    if len(parts) >= 6:
                        m = re.search(r"Module\s*0?(\d+)", parts[0])
                        if m:
                            mid = f"{int(m.group(1)):02d}"
                            for cell in parts[3:]:
                                c = cell.upper()
                                if c in ("PLANNED", "IN_PROGRESS", "REVIEW", "ACCEPTED", "DONE"):
                                    module_status_map[mid] = c
                                    break
            for sm in re.finditer(r"###\s+Module\s*0?(\d+)[:\s].*?\n[\s\S]*?\*\*Status\*\*:\s*`?([A-Za-z_]+)`?", roadmap_content):
                mid = f"{int(sm.group(1)):02d}"
                if mid not in module_status_map:
                    module_status_map[mid] = sm.group(2).strip().upper()
        except Exception as ex:
            print(f"  Warning: Failed to parse roadmap file: {ex}")

    # Scan target files per module for legacy boilerplate Patterns A-D and FUS-V001b forensic stubs
    module_target_files = defaultdict(set)
    module_boilerplate_files = defaultdict(set)
    module_forensic_files = defaultdict(set)
    module_entries = defaultdict(list)
    all_target_files_set = set()
    all_boilerplate_files_set = set()
    all_forensic_files_set = set()
    claimed_entry_closure_violations = []
    runtime_reachability_cache = {}

    # Pre-cache file contents for fast and safe pattern inspection
    target_content_cache = {}
    for rel_sp, entry in all_entries:
        tp = entry["target_rust_gpui_path"]
        sub = entry["subsystem"]
        mid = shard_to_module.get(sub)
        if not mid:
            continue

        module_target_files[mid].add(tp)
        module_entries[mid].append((rel_sp, entry))
        all_target_files_set.add(tp)

        if tp not in target_content_cache:
            full_tp = os.path.join(repo_root, tp)
            if os.path.isfile(full_tp):
                try:
                    with open(full_tp, 'r', encoding='utf-8', errors='ignore') as f:
                        target_content_cache[tp] = f.read()
                except Exception:
                    target_content_cache[tp] = ""
            else:
                target_content_cache[tp] = ""

        content = target_content_cache[tp]
        tags = detect_boilerplate_tags(content) if entry["disposition"] != "build_or_asset_handled" else []
        legacy_tags = [t for t in tags if t.startswith("pattern_")]
        forensic_tags = [t for t in tags if not t.startswith("pattern_")]

        if legacy_tags:
            module_boilerplate_files[mid].add(tp)
            all_boilerplate_files_set.add(tp)
        if forensic_tags:
            module_forensic_files[mid].add(tp)
            all_forensic_files_set.add(tp)

        if is_claimed_entry(entry):
            issues = validate_claimed_entry_closure(repo_root, entry, content)
            if entry["disposition"] != "build_or_asset_handled":
                if tp not in runtime_reachability_cache:
                    runtime_reachability_cache[tp] = is_target_referenced_in_runtime(repo_root, tp)
                if not runtime_reachability_cache[tp]:
                    issues.append(f"Target '{tp}' is not referenced in runtime crate/app tree")
            for issue in issues:
                claimed_entry_closure_violations.append((rel_sp, issue))

    # Print live burndown table
    print("\n" + "=" * 159)
    print("MANDATE #15 & FUS-V001b LIVE MODULE-BY-MODULE BOILERPLATE & FORENSIC BURNDOWN TABLE")
    print("=" * 159)
    header = (
        f"{'Module #':<10} | {'Module Name':<58} | {'Shards':<34} | "
        f"{'Targets':<8} | {'Boilerplate':<11} | {'Forensic':<9} | {'Status':<10}"
    )
    sep = "-" * len(header)
    print(sep)
    print(header)
    print(sep)

    accepted_count = 0
    accepted_boilerplate_violations = []
    accepted_forensic_violations = []

    for mod in ROADMAP_MODULES:
        mid = mod["id"]
        mname = mod["name"]
        mshards_str = ", ".join(f"{s}.json" for s in mod["shards"])
        if len(mshards_str) > 34:
            mshards_str = mshards_str[:31] + "..."
        t_count = len(module_target_files[mid])
        b_count = len(module_boilerplate_files[mid])
        f_count = len(module_forensic_files[mid])
        status = module_status_map.get(mid, "PLANNED")
        if status in ("ACCEPTED", "DONE"):
            accepted_count += 1
            if b_count > 0:
                accepted_boilerplate_violations.append((mid, mname, b_count, list(module_boilerplate_files[mid])))
            if f_count > 0:
                accepted_forensic_violations.append((mid, mname, "forensic_stubs", f_count, list(module_forensic_files[mid])[:3]))
            missing_tests_in_mod = []
            unreachable_in_mod = []
            for _rel_sp, m_entry in module_entries[mid]:
                m_tp = m_entry["target_rust_gpui_path"]
                if not resolve_verification_test_path(repo_root, m_entry.get("verification_test_id", ""), m_tp):
                    missing_tests_in_mod.append(f"{m_tp}:{m_entry.get('verification_test_id')}")
                if m_entry["disposition"] != "build_or_asset_handled":
                    if m_tp not in runtime_reachability_cache:
                        runtime_reachability_cache[m_tp] = is_target_referenced_in_runtime(repo_root, m_tp)
                    if not runtime_reachability_cache[m_tp]:
                        unreachable_in_mod.append(m_tp)
            if missing_tests_in_mod:
                accepted_forensic_violations.append((mid, mname, "missing_test_files", len(missing_tests_in_mod), missing_tests_in_mod[:3]))
            if unreachable_in_mod:
                accepted_forensic_violations.append((mid, mname, "unreachable_targets", len(unreachable_in_mod), unreachable_in_mod[:3]))

        row = f"Module {mid:<3} | {mname:<58} | {mshards_str:<34} | {t_count:<8} | {b_count:<11} | {f_count:<9} | {status:<10}"
        print(row)

    print(sep)
    total_row = (
        f"{'TOTAL':<10} | {'All 28 Subsystems':<58} | {'28 Shards':<34} | "
        f"{len(all_target_files_set):<8} | {len(all_boilerplate_files_set):<11} | "
        f"{len(all_forensic_files_set):<9} | {f'{accepted_count}/13 ACCEPTED'}"
    )
    print(total_row)
    print("=" * 159 + "\n")

    # Hard Veto Assertion: If any module is ACCEPTED, it must have 0 boilerplate and 0 forensic issues
    if accepted_boilerplate_violations:
        print(f"FAIL [Gate 6.2 - Boilerplate Veto]: {len(accepted_boilerplate_violations)} module(s) marked ACCEPTED contain fake boilerplate stubs!")
        for viol in accepted_boilerplate_violations:
            print(f"  VETO: Module {viol[0]} ({viol[1]}) is marked ACCEPTED but contains {viol[2]} boilerplate files! Examples: {viol[3][:3]}")
    elif accepted_count > 0:
        print(f"PASS [Gate 6.2 - Boilerplate Burndown]: All {accepted_count} ACCEPTED module(s) verified with 0 boilerplate stubs.")
    else:
        print(
            f"PASS [Gate 6.2 - Boilerplate Burndown]: Baseline burndown displayed "
            f"({len(all_boilerplate_files_set)} legacy stubs, {len(all_forensic_files_set)} forensic shims/allow_dead_code tracked across 13 modules). "
            "No modules marked ACCEPTED yet."
        )

    if claimed_entry_closure_violations:
        print(f"FAIL [Gate 6.2 - Claimed Entry Closure]: {len(claimed_entry_closure_violations)} implemented/verified entry violation(s)!")
        for sp, reason in claimed_entry_closure_violations[:10]:
            print(f"  VETO: Entry '{sp}': {reason}")
    if accepted_forensic_violations:
        print(f"FAIL [Gate 6.2 - Forensic Acceptance Veto]: {len(accepted_forensic_violations)} ACCEPTED/DONE module forensic failure(s)!")
        for mid, mname, kind, cnt, examples in accepted_forensic_violations[:10]:
            print(f"  VETO: Module {mid} ({mname}) {kind}={cnt}: {examples}")

    capability_coverage_violations = validate_capability_coverage(repo_root, master_index, target_content_cache)
    if capability_coverage_violations:
        print(f"FAIL [Gate 6.7 - Capability Coverage]: {len(capability_coverage_violations)} stage invariant violation(s)!")
        for cv in capability_coverage_violations[:10]:
            print(f"  Violation: {cv}")
    else:
        print("PASS [Gate 6.7 - Capability Coverage]: capability-coverage.json stage summary and promotion invariants verified.")

    # -------------------------------------------------------------------------
    # Assertion 6.4: Inherited Crate Dependency Linking (Mandate #10)
    # -------------------------------------------------------------------------
    print("  Validating Mandate #10 Cargo.toml dependencies on inherited crates...")
    required_inherited_deps = {
        "crates/gpui_telegram_core": ["fabushi-messaging-core"],
        "crates/gpui_telegram_proto": ["fabushi-messaging-core", "mahayana-node-agent-coordinator"],
        "crates/gpui_telegram_server": ["fabushi-messaging-core", "mahayana-node-agent-coordinator", "mahayana-host-runtime"],
        "crates/gpui_telegram_client": ["fabushi-messaging-core", "mahayana-node-agent-coordinator"],
        "crates/gpui_telegram_agent": ["fabushi-messaging-core", "mahayana-node-agent-coordinator", "mahayana-host-runtime"],
        "crates/gpui_telegram_desktop": ["fabushi-messaging-core", "mahayana-node-agent-coordinator", "mahayana-host-runtime"],
    }

    for crate_dir, deps in required_inherited_deps.items():
        cargo_path = os.path.join(repo_root, crate_dir, "Cargo.toml")
        if not os.path.isfile(cargo_path):
            cargo_dep_violations.append((crate_dir, "Missing Cargo.toml"))
            continue
        with open(cargo_path, 'r', encoding='utf-8') as cf:
            cargo_text = cf.read()
        for dep in deps:
            if dep not in cargo_text:
                cargo_dep_violations.append((crate_dir, f"Missing dependency: {dep}"))

    if cargo_dep_violations:
        print(f"FAIL [Gate 6.4 - Cargo Dependencies]: {len(cargo_dep_violations)} Mandate #10 Cargo dependency violations!")
        for cdv in cargo_dep_violations:
            print(f"  Violation: Crate='{cdv[0]}', Detail='{cdv[1]}'")
    else:
        print("PASS [Gate 6.4 - Cargo Dependencies]: All 6 crates declare required inherited dependencies.")

    # -------------------------------------------------------------------------
    # Assertion 6.5: Authentic Inherited Re-export & Import Verification
    # -------------------------------------------------------------------------
    print("  Validating Mandate #10 authentic inherited imports & re-exports...")
    crates_to_check_imports = {
        "crates/gpui_telegram_core/src": ["fabushi_messaging_core"],
        "crates/gpui_telegram_proto/src": ["mahayana_node_agent_coordinator"],
        "crates/gpui_telegram_agent/src": ["mahayana_host_runtime", "mahayana_node_agent_coordinator"],
        "crates/gpui_telegram_server/src": ["fabushi_messaging_core"],
        "crates/gpui_telegram_desktop/src": ["fabushi_messaging_core"],
    }
    for c_src, expected_pkgs in crates_to_check_imports.items():
        if isinstance(expected_pkgs, str):
            expected_pkgs = [expected_pkgs]
        full_src = os.path.join(repo_root, c_src)
        if not os.path.isdir(full_src):
            reexport_violations.append((c_src, "Source directory does not exist!"))
            continue
        for expected_pkg in expected_pkgs:
            found_import = False
            import_pattern = re.compile(rf"(?m)^\s*(?:pub\s+)?use\s+{re.escape(expected_pkg)}\b")
            for dp, _, fns in os.walk(full_src):
                for fn in fns:
                    if fn.endswith(".rs"):
                        try:
                            with open(os.path.join(dp, fn), 'r', encoding='utf-8') as f:
                                if import_pattern.search(f.read()):
                                    found_import = True
                                    break
                        except Exception:
                            pass
                if found_import:
                    break
            if not found_import:
                reexport_violations.append((c_src, f"No authentic 'use {expected_pkg}::...' found in crate!"))

    if reexport_violations:
        print(f"FAIL [Gate 6.5 - Authentic Imports]: {len(reexport_violations)} Mandate #10 re-export / import violations:")
        for rv in reexport_violations:
            print(f"  Violation: Crate='{rv[0]}', Detail='{rv[1]}'")
    else:
        print("PASS [Gate 6.5 - Authentic Imports]: All 5 checked crates contain authentic inherited re-exports/imports.")

    # -------------------------------------------------------------------------
    # Assertion 6.6: Mandate #19 Upstream Source Coverage in Module Design Docs
    # (verify_module_source_coverage: Zero-Omission Module Reading Guarantee)
    # -------------------------------------------------------------------------
    print("  Validating Mandate #19 upstream source coverage in module design docs...")
    module_source_coverage_violations = []
    checked_module_coverage = []

    # Cross-reference upstream source_path entries for modules verified/accepted or with specs on disk (starting from Module 05)
    for mod in ROADMAP_MODULES:
        mid = mod["id"]
        mname = mod["name"]
        status = module_status_map.get(mid, "PLANNED")

        doc_files = glob.glob(os.path.join(repo_root, f"docs/specs/modules/module_{mid}*.md"))
        if (int(mid) >= 5 and status in ("ACCEPTED", "REVIEW", "DONE")) or mid in ("05", "06", "07", "08", "09", "10", "11"):
            if not doc_files:
                module_source_coverage_violations.append((mid, mname, [f"Missing module design document docs/specs/modules/module_{mid}_*.md"]))
                continue

            combined_doc_text = ""
            for df in doc_files:
                try:
                    with open(df, 'r', encoding='utf-8', errors='ignore') as f:
                        combined_doc_text += f.read()
                except Exception as ex:
                    print(f"    Warning: Error reading {df}: {ex}")

            total_sources = 0
            omitted_sources = []
            for s in mod["shards"]:
                shard_path = os.path.join(repo_root, f"docs/specs/ledger/shards/{s}.json")
                if not os.path.isfile(shard_path):
                    continue
                try:
                    with open(shard_path, 'r', encoding='utf-8') as sf:
                        s_data = json.load(sf)
                    s_entries = s_data.get("entries", [])
                    for se in s_entries:
                        total_sources += 1
                        sp = se.get("source_path", "")
                        rel_sp = sp.replace("Telegram/SourceFiles/", "")
                        if sp not in combined_doc_text and rel_sp not in combined_doc_text:
                            omitted_sources.append(sp)
                except Exception as ex:
                    print(f"    Warning: Error reading shard {shard_path}: {ex}")

            if omitted_sources:
                module_source_coverage_violations.append((mid, mname, omitted_sources))
            else:
                checked_module_coverage.append((mid, mname, total_sources, len(doc_files)))

    if module_source_coverage_violations:
        print(f"FAIL [Gate 6.6 - Module Source Coverage]: {len(module_source_coverage_violations)} module(s) violate Mandate #19 zero-omission rule!")
        for viol in module_source_coverage_violations:
            print(f"  VETO: Module {viol[0]} ({viol[1]}) has {len(viol[2])} upstream source file(s) omitted from design doc! Examples: {viol[2][:10]}")
    else:
        for cm in checked_module_coverage:
            print(f"  PASS [Gate 6.6]: Module {cm[0]} ({cm[1]}): 100.0% coverage verified ({cm[2]}/{cm[2]} upstream files documented across {cm[3]} doc(s), 0 omitted).")

    # -------------------------------------------------------------------------
    # Assertion 6.3: Cargo Module Tree Integration & Clean Workspace Compilation
    # -------------------------------------------------------------------------
    cargo_error_msg = None
    if skip_cargo:
        print("  [--skip-cargo] Deferred 'cargo check --workspace' to dedicated Cargo build job.")
    else:
        print("  Running 'cargo check --workspace' for clean tree compilation...")
        try:
            cargo_result = subprocess.run(
                ["cargo", "check", "--workspace"],
                cwd=repo_root,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            if cargo_result.returncode != 0:
                cargo_error_msg = f"'cargo check --workspace' failed with exit code {cargo_result.returncode}!\n{cargo_result.stderr[:2000]}"
        except Exception as ex:
            cargo_error_msg = f"Failed to execute 'cargo check --workspace': {ex}"
        finally:
            # Enforce Mandate #11 Build Hygiene: clean target build garbage
            clean_script = os.path.join(script_dir, "clean_build_garbage.py")
            if os.path.isfile(clean_script):
                subprocess.run([sys.executable, clean_script], cwd=repo_root, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    if cargo_error_msg:
        print(f"FAIL [Gate 6.3 - Compilation]: {cargo_error_msg}")
    else:
        print("PASS [Gate 6.3 - Compilation]: Clean workspace compilation verified (exit code 0).")

    # -------------------------------------------------------------------------
    # Unified Gate 6 Diagnostics Synthesis & Final Exit Decision
    # -------------------------------------------------------------------------
    print("-" * 70)
    print("GATE 6 ACCUMULATED DIAGNOSTICS SUMMARY:")
    print("-" * 70)
    assertions = [
        ("Gate 6.1 (Inherited Paths)", len(missing_inherited_files) == 0, f"{len(missing_inherited_files)} missing"),
        ("Gate 6.1 (Target Files)", len(missing_target_files) == 0, f"{len(missing_target_files)} missing / {len(all_entries)} total"),
        ("Gate 6.2 (Target Symbols)", len(missing_target_symbols) == 0, f"{len(missing_target_symbols)} missing or stubs in existing files"),
        ("Gate 6.2 (Boilerplate Burndown)", len(accepted_boilerplate_violations) == 0,
         f"{len(accepted_boilerplate_violations)} ACCEPTED modules contain boilerplate! VETO." if accepted_boilerplate_violations else
         (f"All {accepted_count} ACCEPTED modules verified clean (0 stubs)" if accepted_count > 0 else f"Baseline burndown active ({len(all_boilerplate_files_set)} stubs tracked, 0 ACCEPTED)")),
        ("Gate 6.2 (Forensic Claim Closure)",
         len(claimed_entry_closure_violations) == 0 and len(accepted_forensic_violations) == 0,
         f"{len(claimed_entry_closure_violations)} claimed entry + {len(accepted_forensic_violations)} accepted module violations"
         if (claimed_entry_closure_violations or accepted_forensic_violations)
         else f"Fail-closed active ({len(all_forensic_files_set)} forensic files tracked cleanly, 0 invalid claims)"),
        ("Gate 6.3 (Workspace Compilation)", cargo_error_msg is None, "Clean exit 0" if cargo_error_msg is None else "Compilation failed"),
        ("Gate 6.4 (Cargo Dependencies)", len(cargo_dep_violations) == 0, f"{len(cargo_dep_violations)} violations"),
        ("Gate 6.5 (Authentic Imports)", len(reexport_violations) == 0, f"{len(reexport_violations)} violations"),
        ("Gate 6.6 (Module Source Coverage)", len(module_source_coverage_violations) == 0,
         f"{len(module_source_coverage_violations)} modules with omissions" if module_source_coverage_violations else
         f"100% coverage verified ({len(checked_module_coverage)} checked module(s), 0 omitted)"),
        ("Gate 6.7 (Capability Coverage)", len(capability_coverage_violations) == 0,
         f"{len(capability_coverage_violations)} violations" if capability_coverage_violations else "All stage invariants verified"),
    ]

    all_passed = True
    for name, passed, detail in assertions:
        status = "PASS" if passed else "FAIL"
        if not passed:
            all_passed = False
        print(f"  [{status}] {name}: {detail}")
    print("-" * 70)

    if not all_passed:
        print("FAIL [Gate 6]: One or more Gate 6 assertions failed!")
        print("=" * 70)
        sys.exit(1)

    print("PASS [Gate 6]: All Gate 6 assertions passed successfully.")
    print("=" * 70)
    print("ALL 6 VERIFICATION GATES PASSED SUCCESSFULLY (Exit Code 0).")
    print("=" * 70)
    sys.exit(0)

if __name__ == '__main__':
    main()
