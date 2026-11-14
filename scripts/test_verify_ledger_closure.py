import json
import os
import sys
import tempfile
import unittest

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, ".."))
if SCRIPT_DIR not in sys.path:
    sys.path.insert(0, SCRIPT_DIR)

import verify_ledger_closure as vlc


class TestVerifyLedgerClosure(unittest.TestCase):
    def test_validate_extracted_features_rejects_cpp_pseudo_symbols(self):
        base = {
            "source_sha256": "a" * 64,
            "source_line_count": 42,
            "source_classes_and_structs": ["MainWindow"],
            "source_methods_and_signals": ["bool"],
            "style_tokens_and_metrics": [],
        }
        err = vlc.validate_extracted_features(base)
        self.assertIsNotNone(err)
        self.assertIn("pseudo-symbol", err)

        inc = dict(base, source_methods_and_signals=["#include <foo.h>"])
        self.assertIsNotNone(vlc.validate_extracted_features(inc))

        ok = dict(base, source_methods_and_signals=["init", "#define APP_VER"])
        self.assertIsNone(vlc.validate_extracted_features(ok))

    def test_forensic_detectors(self):
        self.assertTrue(vlc.has_allow_dead_code("#![allow(dead_code)]\npub struct X;"))
        self.assertTrue(vlc.has_allow_dead_code("#![allow(unused, dead_code)]\n"))
        self.assertFalse(vlc.has_allow_dead_code("// #![allow(dead_code)]\npub struct X;"))

        noop = "impl Foo {\n  pub fn tick(&mut self) -> Result<(), ()> {\n    self.updated_at = chrono::Utc::now().timestamp();\n    Ok(())\n  }\n}"
        self.assertTrue(vlc.has_timestamp_noop_stub(noop))

        real = "impl Foo {\n  pub fn apply(&mut self, v: i32) {\n    self.count = v;\n    self.updated_at = chrono::Utc::now().timestamp();\n  }\n}"
        self.assertFalse(vlc.has_timestamp_noop_stub(real))

        shim = "// comment\n#[allow(unused_imports)]\nuse fabushi_messaging_core::*;\npub use fabushi_messaging_core::{dialog, message};"
        self.assertTrue(vlc.is_bare_pub_use_shim(shim))
        self.assertFalse(vlc.is_bare_pub_use_shim(shim + "\npub fn attach_domain() {}"))

    def test_claimed_entry_fails_closed_without_real_test_or_clean_impl(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "crates/demo/src"), exist_ok=True)
            os.makedirs(os.path.join(tmp, "crates/demo/tests"), exist_ok=True)
            real_test = os.path.join(tmp, "crates/demo/tests/real_test.rs")
            with open(real_test, "w", encoding="utf-8") as f:
                f.write("#[test]\nfn works() {}\n")

            entry = {
                "target_rust_gpui_path": "crates/demo/src/foo.rs",
                "disposition": "rust_domain_or_server_1to1",
                "verification_test_id": "tests/missing_test.rs",
            }
            issues = vlc.validate_claimed_entry_closure(tmp, entry, "pub struct Foo;")
            self.assertEqual(len(issues), 1)
            self.assertIn("Missing on-disk verification_test_id", issues[0])

            entry["verification_test_id"] = "tests/real_test.rs::works"
            self.assertEqual(vlc.validate_claimed_entry_closure(tmp, entry, "pub struct Foo;"), [])
            issues_shim = vlc.validate_claimed_entry_closure(tmp, entry, "pub use other::*;")
            self.assertTrue(any("bare_pub_use_shim" in x for x in issues_shim))

    def test_runtime_reachability_and_capability_coverage_fail_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            src = os.path.join(tmp, "crates/gpui_telegram_desktop/src")
            os.makedirs(os.path.join(src, "views"), exist_ok=True)
            with open(os.path.join(src, "main.rs"), "w", encoding="utf-8") as f:
                f.write("mod views;\nfn main() { let _ = views::shell::AppShell; }\n")
            with open(os.path.join(src, "views/mod.rs"), "w", encoding="utf-8") as f:
                f.write("pub mod shell;\n")
            with open(os.path.join(src, "views/shell.rs"), "w", encoding="utf-8") as f:
                f.write("pub struct AppShell;\n")
            with open(os.path.join(src, "views/orphan.rs"), "w", encoding="utf-8") as f:
                f.write("pub struct Orphan;\n")

            self.assertTrue(vlc.is_target_referenced_in_runtime(tmp, "crates/gpui_telegram_desktop/src/views/shell.rs"))
            self.assertFalse(vlc.is_target_referenced_in_runtime(tmp, "crates/gpui_telegram_desktop/src/views/orphan.rs"))

            cov_dir = os.path.join(tmp, "docs/verification/fusion-20261108")
            os.makedirs(cov_dir, exist_ok=True)
            cov = {
                "schema_version": 2,
                "stage_summary": {
                    "file_mapped": {"verified": 1},
                    "entrypoint_reachable": {"verified": 1},
                    "behavior_tested": {"unverified": 1, "verified": 0},
                    "ui_evidence_captured": {"unverified": 1, "verified": 0},
                    "packaged_accepted": {"unverified": 1, "verified": 0},
                },
                "capabilities": [{
                    "capability_id": "CAP-01",
                    "evidence_stages": {
                        "file_mapped": {"status": "verified", "existing_shards": ["main.json"], "missing_shards": []},
                        "entrypoint_reachable": {"status": "verified", "verified_entrypoints": ["crates/gpui_telegram_desktop/src/views/orphan.rs"]},
                        "behavior_tested": {"status": "unverified", "verified_runs": []},
                        "ui_evidence_captured": {"status": "unverified", "artifacts": []},
                        "packaged_accepted": {"status": "unverified", "artifacts": []},
                    },
                }],
            }
            with open(os.path.join(cov_dir, "capability-coverage.json"), "w", encoding="utf-8") as f:
                json.dump(cov, f)
            viols = vlc.validate_capability_coverage(tmp, {"shards": [{"subsystem": "main"}]})
            self.assertTrue(any("unreachable from runtime" in v for v in viols))

    def test_live_repository_capability_coverage_validates_cleanly(self):
        with open(os.path.join(REPO_ROOT, "docs/specs/ledger/index.json"), "r", encoding="utf-8") as f:
            master_index = json.load(f)
        self.assertEqual(vlc.validate_capability_coverage(REPO_ROOT, master_index), [])


if __name__ == "__main__":
    unittest.main()
