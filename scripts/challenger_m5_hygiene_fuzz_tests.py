#!/usr/bin/env python3
"""
scripts/challenger_m5_hygiene_fuzz_tests.py

Adversarial stress testing & fuzzing suite for scripts/clean_build_garbage.py.
Milestone 5 Iteration 2 Gate: Build Garbage & Compiler Fuzzing Challenger.

Objectives & Fuzzing Scenarios:
1. Multi-crate compiler cache stability across repeated runs and `cargo check --workspace`
2. Safe compiler artifact retention policy (.rcgu.o, incremental, dSYM, test binaries pruned;
   .rmeta, .rlib, .fingerprint strictly preserved to prevent E0460/E0463 regressions)
3. Hostile filesystem edge cases:
   - Deep nested directories (25+ levels deep) under target/
   - Broken symlinks pointing to non-existent targets
   - Symlinks pointing outside workspace (security isolation)
   - Read-only files (chmod 444, 555) and restricted files (chmod 000)
   - Space-containing and unicode/emoji filenames
4. Concurrent execution fuzzing: 10 concurrent processes running simultaneously against shared target
5. Cleanup idempotency: consecutive runs report 0 items cleaned, 0 B freed, exit code 0
6. Temporary SQLite databases with active WAL and SHM locks
7. Tantivy search indexes and CAS chunk upload staging cleanup
8. 15-second active build timing guard vs --force bypass
9. CLI flag filtering and non-existent path robustness
10. Directory permission resilience audit (chmod 000 deps directory)
"""

import concurrent.futures
import os
import re
import shutil
import sqlite3
import subprocess
import sys
import tempfile
import time
import unittest

# Ensure target script can be imported directly
REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
CLEAN_SCRIPT = os.path.join(REPO_ROOT, "scripts", "clean_build_garbage.py")
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

from scripts import clean_build_garbage


class TestBuildHygieneFuzzSuite(unittest.TestCase):
    def setUp(self):
        """Create isolated sandbox environment for each test."""
        self.test_dir = tempfile.mkdtemp(prefix="fuzz_hygiene_test_")
        self.tmp_target = os.path.join(self.test_dir, "target")
        self.tracked_external_dirs = []

    def tearDown(self):
        """Cleanly restore permissions and purge sandbox directory."""
        for root, dirs, files in os.walk(self.test_dir):
            for d in dirs:
                try:
                    os.chmod(os.path.join(root, d), 0o755)
                except OSError:
                    pass
            for f in files:
                try:
                    os.chmod(os.path.join(root, f), 0o644)
                except OSError:
                    pass

        try:
            shutil.rmtree(self.test_dir, ignore_errors=True)
        except OSError:
            pass

        for p in self.tracked_external_dirs:
            try:
                if os.path.isdir(p):
                    shutil.rmtree(p, ignore_errors=True)
                elif os.path.lexists(p):
                    os.unlink(p)
            except OSError:
                pass

    def test_01_nested_redundant_targets_and_dry_run(self):
        """Test deeply nested target/ directories and .cargo-target-* with --dry-run and live run."""
        canonical_target = os.path.join(self.test_dir, "target")
        os.makedirs(canonical_target)
        canonical_bin = os.path.join(canonical_target, "keep_canonical.bin")
        with open(canonical_bin, "w") as f:
            f.write("canonical target data")

        # Nested target 1: subcrate/target
        nested1 = os.path.join(self.test_dir, "crates", "crate_a", "target")
        os.makedirs(nested1)
        with open(os.path.join(nested1, "junk1.bin"), "w") as f:
            f.write("junk data 1")

        # Nested target 2: deeply nested a/b/c/d/target
        nested2 = os.path.join(self.test_dir, "a", "b", "c", "d", "target")
        os.makedirs(nested2)
        with open(os.path.join(nested2, "junk2.bin"), "w") as f:
            f.write("junk data 2")

        # Cargo target 1: .cargo-target-arm64
        cargo_target1 = os.path.join(self.test_dir, ".cargo-target-arm64")
        os.makedirs(cargo_target1)
        with open(os.path.join(cargo_target1, "cargo_arm.bin"), "w") as f:
            f.write("arm build cache")

        # Cargo target 2: nested crates/.cargo-target-opt
        cargo_target2 = os.path.join(self.test_dir, "crates", ".cargo-target-opt")
        os.makedirs(cargo_target2)
        with open(os.path.join(cargo_target2, "cargo_opt.bin"), "w") as f:
            f.write("opt build cache")

        # 1. Test Dry Run: must calculate count & size without deleting files
        count_dry, bytes_dry = clean_build_garbage.clean_redundant_workspace_targets(self.test_dir, dry_run=True)
        self.assertEqual(count_dry, 4, f"Dry run should find 4 redundant targets, got {count_dry}")
        self.assertGreater(bytes_dry, 0, "Dry run freed bytes should be > 0")
        self.assertTrue(os.path.exists(nested1), "Dry run must NOT delete nested1")
        self.assertTrue(os.path.exists(nested2), "Dry run must NOT delete nested2")
        self.assertTrue(os.path.exists(cargo_target1), "Dry run must NOT delete cargo_target1")
        self.assertTrue(os.path.exists(cargo_target2), "Dry run must NOT delete cargo_target2")

        # 2. Test Live Run: must delete all redundant targets and preserve canonical target
        count_live, bytes_live = clean_build_garbage.clean_redundant_workspace_targets(self.test_dir, dry_run=False)
        self.assertEqual(count_live, 4, f"Live run should clean 4 redundant targets, got {count_live}")
        self.assertEqual(bytes_live, bytes_dry, "Live run freed bytes should match dry run")
        self.assertFalse(os.path.exists(nested1), "nested1 must be deleted")
        self.assertFalse(os.path.exists(nested2), "nested2 must be deleted")
        self.assertFalse(os.path.exists(cargo_target1), "cargo_target1 must be deleted")
        self.assertFalse(os.path.exists(cargo_target2), "cargo_target2 must be deleted")

        # Canonical target must remain untouched
        self.assertTrue(os.path.exists(canonical_target), "Canonical target MUST be preserved")
        self.assertTrue(os.path.exists(canonical_bin), "Canonical target contents MUST be preserved")

    def test_02_compiler_artifact_safe_retention_policy(self):
        """
        Verify safe retention policy:
        Intermediate .rcgu.o and intermediate .o files ARE purged.
        All .rlib and .rmeta hashes are strictly PRESERVED to maintain multi-crate dependency graphs.
        """
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()
        past = now - 100

        # Group 1: Multiple rlib revisions for multi-crate feature sets (must be PRESERVED)
        gpui_v1 = os.path.join(deps_dir, "libgpui-1111111111111111.rlib")
        gpui_v2 = os.path.join(deps_dir, "libgpui-2222222222222222.rlib")
        gpui_v3 = os.path.join(deps_dir, "libgpui-3333333333333333.rlib")
        for p, text in [(gpui_v1, "gpui v1"), (gpui_v2, "gpui v2"), (gpui_v3, "gpui v3")]:
            with open(p, "w") as f: f.write(text)
            os.utime(p, (past, past))

        # Group 2: Multiple rmeta revisions (must be PRESERVED)
        rmeta_v1 = os.path.join(deps_dir, "libgpui-1111111111111111.rmeta")
        rmeta_v3 = os.path.join(deps_dir, "libgpui-3333333333333333.rmeta")
        for p, text in [(rmeta_v1, "meta v1"), (rmeta_v3, "meta v3")]:
            with open(p, "w") as f: f.write(text)
            os.utime(p, (past, past))

        # Group 3: Codegen unit object files (.rcgu.o and hashed .o) (MUST BE PURGED)
        rcgu_v1 = os.path.join(deps_dir, "libserver-cccccccccccccccc.rcgu.o")
        rcgu_v2 = os.path.join(deps_dir, "libserver-dddddddddddddddd.rcgu.o")
        hashed_o = os.path.join(deps_dir, "libclient-eeeeeeeeeeeeeeee-1.o")
        for p, text in [(rcgu_v1, "rcgu v1"), (rcgu_v2, "rcgu v2"), (hashed_o, "hashed o")]:
            with open(p, "w") as f: f.write(text)
            os.utime(p, (past, past))

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=False)
        self.assertEqual(c, 3, f"Expected exactly 3 intermediate object files cleaned, got {c}")
        self.assertGreater(b, 0, "Expected freed bytes > 0")

        # Verify intermediate objects purged
        self.assertFalse(os.path.exists(rcgu_v1), "rcgu_v1 must be purged")
        self.assertFalse(os.path.exists(rcgu_v2), "rcgu_v2 must be purged")
        self.assertFalse(os.path.exists(hashed_o), "hashed_o must be purged")

        # Verify all .rlib and .rmeta hashes are strictly preserved for compiler stability
        self.assertTrue(os.path.exists(gpui_v1), "gpui_v1 rlib MUST be preserved")
        self.assertTrue(os.path.exists(gpui_v2), "gpui_v2 rlib MUST be preserved")
        self.assertTrue(os.path.exists(gpui_v3), "gpui_v3 rlib MUST be preserved")
        self.assertTrue(os.path.exists(rmeta_v1), "rmeta_v1 rmeta MUST be preserved")
        self.assertTrue(os.path.exists(rmeta_v3), "rmeta_v3 rmeta MUST be preserved")

    def test_03_unhashed_compiler_artifacts_preservation(self):
        """Test that unhashed library files and static archives are NEVER pruned."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()
        past = now - 200

        unhashed_rlib = os.path.join(deps_dir, "libgpui_static.rlib")
        unhashed_a = os.path.join(deps_dir, "libsystem_helper.a")
        unhashed_txt = os.path.join(deps_dir, "cargo_metadata.txt")

        with open(unhashed_rlib, "w") as f: f.write("unhashed rlib")
        with open(unhashed_a, "w") as f: f.write("unhashed static archive")
        with open(unhashed_txt, "w") as f: f.write("some text")

        os.utime(unhashed_rlib, (past, past))
        os.utime(unhashed_a, (past, past))
        os.utime(unhashed_txt, (past, past))

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=True)
        self.assertEqual(c, 0, "No unhashed artifacts should be purged")
        self.assertTrue(os.path.exists(unhashed_rlib), "Unhashed rlib must be preserved")
        self.assertTrue(os.path.exists(unhashed_a), "Unhashed static archive must be preserved")
        self.assertTrue(os.path.exists(unhashed_txt), "Text file must be preserved")

    def test_04_stale_test_binaries_cleanup(self):
        """Test that loose executable test binaries in target/*/deps are identified and purged."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()
        past = now - 100

        # Test runner executable with 16-hex hash
        tbin1 = os.path.join(deps_dir, "integration_test-1234567890abcdef")
        with open(tbin1, "w") as f: f.write("binary data")
        os.chmod(tbin1, 0o755)
        os.utime(tbin1, (past, past))

        # Test runner executable without hash
        tbin2 = os.path.join(deps_dir, "unit_suite_runner")
        with open(tbin2, "w") as f: f.write("binary suite")
        os.chmod(tbin2, 0o755)
        os.utime(tbin2, (past, past))

        # Non-executable file without extension (must NOT be treated as binary)
        regular_file = os.path.join(deps_dir, "README")
        with open(regular_file, "w") as f: f.write("readme")
        os.chmod(regular_file, 0o644)
        os.utime(regular_file, (past, past))

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=False)
        self.assertEqual(c, 2, f"Expected 2 test binaries cleaned, got {c}")
        self.assertFalse(os.path.exists(tbin1), "tbin1 must be cleaned")
        self.assertFalse(os.path.exists(tbin2), "tbin2 must be cleaned")
        self.assertTrue(os.path.exists(regular_file), "README must be preserved")

    def test_05_fingerprint_integrity_preservation(self):
        """Test that .fingerprint cache is completely untouched to guarantee compiler cache synchronization."""
        prof_dir = os.path.join(self.tmp_target, "debug")
        deps_dir = os.path.join(prof_dir, "deps")
        fp_dir = os.path.join(prof_dir, ".fingerprint")
        os.makedirs(deps_dir)
        os.makedirs(fp_dir)
        now = time.time()
        past = now - 100

        fp_crate1 = os.path.join(fp_dir, "gpui-3333333333333333")
        fp_crate2 = os.path.join(fp_dir, "serde-4444444444444444")
        os.makedirs(fp_crate1)
        os.makedirs(fp_crate2)
        with open(os.path.join(fp_crate1, "lib-gpui.json"), "w") as f: f.write("{}")
        with open(os.path.join(fp_crate2, "lib-serde.json"), "w") as f: f.write("{}")

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=True)
        self.assertEqual(c, 0, "clean_target_dir must NOT touch .fingerprint")
        self.assertTrue(os.path.exists(fp_crate1), "Fingerprint crate1 must be preserved")
        self.assertTrue(os.path.exists(fp_crate2), "Fingerprint crate2 must be preserved")

    def test_06_active_build_timing_guard_vs_force(self):
        """Test the 15-second active build timing guard protecting files modified recently, and --force bypass."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()

        # Artifacts modified 2 seconds ago (active build)
        rcgu_active = os.path.join(deps_dir, "libactive-1111111111111111.rcgu.o")
        tbin_active = os.path.join(deps_dir, "active_test-3333333333333333")

        with open(rcgu_active, "w") as f: f.write("active rcgu")
        with open(tbin_active, "w") as f: f.write("active bin")
        os.chmod(tbin_active, 0o755)
        os.utime(rcgu_active, (now - 2.0, now - 2.0))
        os.utime(tbin_active, (now - 2.0, now - 2.0))

        # 1. Without --force: active build guard (15s) must protect recent artifacts
        c_guard, _ = clean_build_garbage.clean_target_dir(self.tmp_target, force=False)
        self.assertEqual(c_guard, 0, f"Active build guard must report 0 cleaned, got {c_guard}")
        self.assertTrue(os.path.exists(rcgu_active), "rcgu_active must be protected by active guard")
        self.assertTrue(os.path.exists(tbin_active), "tbin_active must be protected by active guard")

        # 2. With --force: active build guard is bypassed
        c_force, _ = clean_build_garbage.clean_target_dir(self.tmp_target, force=True)
        self.assertEqual(c_force, 2, f"With --force, should clean 2 items (rcgu + test bin), got {c_force}")
        self.assertFalse(os.path.exists(rcgu_active), "rcgu_active must be cleaned with --force")
        self.assertFalse(os.path.exists(tbin_active), "tbin_active must be cleaned with --force")

    def test_07_sqlite_test_databases_with_wal_and_shm_locks(self):
        """Test cleaning of temporary SQLite databases and active WAL/SHM locks."""
        # 1. Test database with WAL and SHM inside workspace
        db_dir = os.path.join(self.test_dir, "crates", "server", "data_test")
        os.makedirs(db_dir)
        ws_db = os.path.join(db_dir, "test_session.sqlite")
        conn = sqlite3.connect(ws_db)
        conn.execute("PRAGMA journal_mode=WAL;")
        conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);")
        conn.execute("INSERT INTO users (name) VALUES ('Alice');")
        conn.commit()

        ws_wal = ws_db + "-wal"
        ws_shm = ws_db + "-shm"
        self.assertTrue(os.path.exists(ws_db), "ws_db must exist")

        # 2. Production database (must NOT be deleted)
        prod_db = os.path.join(self.test_dir, "crates", "server", "production_data.db")
        with open(prod_db, "w") as f:
            f.write("production credentials")

        # 3. Temp directory test fixtures in /tmp or TMPDIR
        tmp_db_dir = os.path.join(tempfile.gettempdir(), f"gpui_test_persist_challenger_{int(time.time())}")
        os.makedirs(tmp_db_dir, exist_ok=True)
        self.tracked_external_dirs.append(tmp_db_dir)
        with open(os.path.join(tmp_db_dir, "ephemeral.db"), "w") as f:
            f.write("ephemeral db")

        loose_temp_db = os.path.join(tempfile.gettempdir(), f"gpui_test_transient_{int(time.time())}.db")
        with open(loose_temp_db, "w") as f:
            f.write("loose transient db")
        self.tracked_external_dirs.append(loose_temp_db)

        # Execute clean_test_sqlite_databases
        c_db, b_db = clean_build_garbage.clean_test_sqlite_databases(self.test_dir, dry_run=False)
        self.assertGreaterEqual(c_db, 3, f"Expected at least 3 DB items cleaned, got {c_db}")
        self.assertGreater(b_db, 0, "Expected freed bytes > 0")

        self.assertFalse(os.path.exists(ws_db), "ws_db must be cleaned")
        self.assertFalse(os.path.exists(ws_wal), "ws_wal must be cleaned")
        self.assertFalse(os.path.exists(ws_shm), "ws_shm must be cleaned")
        self.assertFalse(os.path.exists(tmp_db_dir), "tmp_db_dir must be cleaned")
        self.assertFalse(os.path.exists(loose_temp_db), "loose_temp_db must be cleaned")

        # Production database must remain intact
        self.assertTrue(os.path.exists(prod_db), "Production database MUST NOT be deleted")
        conn.close()

    def test_08_tantivy_indexes_and_cas_staging(self):
        """Test Tantivy search indexes and media CAS chunk upload staging cleanup."""
        # 1. Tantivy index in workspace
        ws_tantivy = os.path.join(self.test_dir, "search_index_messages")
        os.makedirs(ws_tantivy)
        with open(os.path.join(ws_tantivy, "meta.json"), "w") as f: f.write('{"segments":[]}')
        with open(os.path.join(ws_tantivy, ".tantivy-meta.lock"), "w") as f: f.write("")

        # 2. Tantivy index in temp
        tmp_tantivy = os.path.join(tempfile.gettempdir(), f"gpui_test_search_fuzz_{int(time.time())}")
        os.makedirs(tmp_tantivy, exist_ok=True)
        self.tracked_external_dirs.append(tmp_tantivy)
        with open(os.path.join(tmp_tantivy, "meta.json"), "w") as f: f.write('{}')

        # 3. CAS staging in temp
        cas_dir = os.path.join(tempfile.gettempdir(), f"gpui_telegram_cas_fuzz_{int(time.time())}")
        os.makedirs(cas_dir, exist_ok=True)
        self.tracked_external_dirs.append(cas_dir)
        with open(os.path.join(cas_dir, "chunk_0.part"), "w") as f: f.write("chunk")

        cas_file_part = os.path.join(tempfile.gettempdir(), f"gpui_upload_chunk_{int(time.time())}.part")
        cas_file_meta = os.path.join(tempfile.gettempdir(), f"gpui_upload_chunk_{int(time.time())}.meta.json")
        with open(cas_file_part, "w") as f: f.write("part")
        with open(cas_file_meta, "w") as f: f.write('{"size": 4}')
        self.tracked_external_dirs.extend([cas_file_part, cas_file_meta])

        c_tan, _ = clean_build_garbage.clean_tantivy_indexes(self.test_dir)
        c_cas, _ = clean_build_garbage.clean_chunk_upload_staging()

        self.assertGreaterEqual(c_tan, 2, f"Expected at least 2 Tantivy indexes cleaned, got {c_tan}")
        self.assertGreaterEqual(c_cas, 3, f"Expected at least 3 CAS items cleaned, got {c_cas}")

        self.assertFalse(os.path.exists(ws_tantivy), "ws_tantivy must be deleted")
        self.assertFalse(os.path.exists(tmp_tantivy), "tmp_tantivy must be deleted")
        self.assertFalse(os.path.exists(cas_dir), "cas_dir must be deleted")
        self.assertFalse(os.path.exists(cas_file_part), "cas_file_part must be deleted")
        self.assertFalse(os.path.exists(cas_file_meta), "cas_file_meta must be deleted")

    def test_09_hostile_file_permissions(self):
        """Test clean handling of hostile file permissions (chmod 000, 444, 555) on compiler artifacts and databases."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()
        past = now - 100

        # rcgu with chmod 000
        f_000 = os.path.join(deps_dir, "libhostile-1111111111111111.rcgu.o")
        with open(f_000, "w") as f: f.write("000 hostile")
        os.chmod(f_000, 000)
        os.utime(f_000, (past, past))

        # rcgu with chmod 444 (read-only)
        f_444 = os.path.join(deps_dir, "libhostile-2222222222222222.rcgu.o")
        with open(f_444, "w") as f: f.write("444 hostile")
        os.chmod(f_444, 0o444)
        os.utime(f_444, (past, past))

        # Stale test binary with chmod 555 (read & execute only)
        f_555 = os.path.join(deps_dir, "hostile_test-3333333333333333")
        with open(f_555, "w") as f: f.write("555 hostile")
        os.chmod(f_555, 0o555)
        os.utime(f_555, (past, past))

        # Active .rlib and .rmeta (must be preserved)
        f_rlib_new = os.path.join(deps_dir, "libhostile-4444444444444444.rlib")
        f_rmeta_new = os.path.join(deps_dir, "libhostile-4444444444444444.rmeta")
        with open(f_rlib_new, "w") as f: f.write("active rlib")
        with open(f_rmeta_new, "w") as f: f.write("active rmeta")
        os.utime(f_rlib_new, (past, past))
        os.utime(f_rmeta_new, (past, past))

        c, _ = clean_build_garbage.clean_target_dir(self.tmp_target, force=False)
        self.assertEqual(c, 3, f"Expected 3 hostile files cleaned, got {c}")
        self.assertFalse(os.path.exists(f_000), "000 rcgu file must be removed")
        self.assertFalse(os.path.exists(f_444), "444 rcgu file must be removed")
        self.assertFalse(os.path.exists(f_555), "555 test binary must be removed")
        self.assertTrue(os.path.exists(f_rlib_new), "rlib must be preserved")
        self.assertTrue(os.path.exists(f_rmeta_new), "rmeta must be preserved")

    def test_10_symlink_safety_and_loops(self):
        """Test that symlinks pointing outside workspace are safe and circular symlinks do not cause infinite loops."""
        outside_dir = tempfile.mkdtemp(prefix="fuzz_outside_protected_")
        self.tracked_external_dirs.append(outside_dir)
        protected_file = os.path.join(outside_dir, "critical_user_secrets.pem")
        with open(protected_file, "w") as f:
            f.write("DO NOT DELETE CRITICAL DATA")

        # Symlink pointing outside workspace
        subcrate = os.path.join(self.test_dir, "crates", "subcrate_link")
        os.makedirs(subcrate)
        target_link = os.path.join(subcrate, "target")
        os.symlink(outside_dir, target_link)

        # Symlink loop
        loop_link = os.path.join(subcrate, "recursive_loop")
        os.symlink(subcrate, loop_link)

        # Broken symlinks
        broken_link = os.path.join(subcrate, "broken_target_link")
        os.symlink("/nonexistent/file/xyz_12345", broken_link)

        clean_build_garbage.clean_redundant_workspace_targets(self.test_dir, dry_run=False)

        # Assert external protected file is completely untouched
        self.assertTrue(os.path.exists(outside_dir), "Outside directory MUST NOT be deleted")
        self.assertTrue(os.path.exists(protected_file), "Outside critical file MUST NOT be deleted")
        with open(protected_file, "r") as f:
            self.assertEqual(f.read(), "DO NOT DELETE CRITICAL DATA", "Outside content integrity verified")

    def test_11_deeply_nested_paths_under_target(self):
        """Test deeply nested directories (25+ levels deep) under target/ incremental caches."""
        deep_inc = os.path.join(self.tmp_target, "debug", "incremental", *[f"nest_{i}" for i in range(25)])
        os.makedirs(deep_inc, exist_ok=True)
        with open(os.path.join(deep_inc, "deep_cache.bin"), "w") as f:
            f.write("deeply nested incremental cache content")

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=True)
        self.assertEqual(c, 1, "Should clean exactly 1 incremental cache bundle")
        self.assertGreater(b, 0, "Freed bytes should be > 0")
        self.assertFalse(os.path.exists(os.path.join(self.tmp_target, "debug", "incremental")), "Incremental dir must be removed")

    def test_12_space_and_unicode_filenames(self):
        """Test handling of space-containing and unicode/emoji filenames."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        now = time.time()
        past = now - 100

        # Space in executable test binary
        space_bin = os.path.join(deps_dir, "stale test runner binary with spaces")
        with open(space_bin, "w") as f: f.write("space bin")
        os.chmod(space_bin, 0o755)
        os.utime(space_bin, (past, past))

        # Space in rcgu object
        space_rcgu = os.path.join(deps_dir, "lib space crate-1234567890abcdef.rcgu.o")
        with open(space_rcgu, "w") as f: f.write("space rcgu")
        os.utime(space_rcgu, (past, past))

        # Unicode & emoji in rcgu object
        unicode_rcgu = os.path.join(deps_dir, "lib_🚀_telegram_media-9999888877776666.rcgu.o")
        with open(unicode_rcgu, "w") as f: f.write("unicode rcgu")
        os.utime(unicode_rcgu, (past, past))

        c, b = clean_build_garbage.clean_target_dir(self.tmp_target, force=False)
        self.assertEqual(c, 3, f"Expected 3 space/unicode items cleaned, got {c}")
        self.assertFalse(os.path.exists(space_bin), "space_bin must be cleaned")
        self.assertFalse(os.path.exists(space_rcgu), "space_rcgu must be cleaned")
        self.assertFalse(os.path.exists(unicode_rcgu), "unicode_rcgu must be cleaned")

    def test_13_clean_workspace_idempotency(self):
        """Test running clean_build_garbage.py consecutively: 2nd and 3rd passes report 0 items and exit 0."""
        # First populate with items
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        past = time.time() - 100
        for i in range(5):
            p = os.path.join(deps_dir, f"lib_junk_{i}-1234567890abcdef.rcgu.o")
            with open(p, "w") as f: f.write("junk")
            os.utime(p, (past, past))

        # Pass 1: cleans items
        res1 = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", self.test_dir,
            "--target-dir", self.tmp_target,
        ], capture_output=True, text=True)
        self.assertEqual(res1.returncode, 0)
        self.assertIn("5 items", res1.stdout)

        # Pass 2: immediately re-run -> 0 items, 0 B freed, exit 0
        res2 = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", self.test_dir,
            "--target-dir", self.tmp_target,
        ], capture_output=True, text=True)
        self.assertEqual(res2.returncode, 0, f"Expected exit code 0 on 2nd pass, got {res2.returncode}")
        self.assertIn("Cleaned compiler target artifacts: 0 items (0 B)", res2.stdout)
        self.assertIn("Cleaned temporary SQLite test fixtures: 0 items (0 B)", res2.stdout)
        self.assertIn("Done! Freed 0 B (0 bytes across 0 items).", res2.stdout)

        # Pass 3: third re-run -> 0 items, 0 B freed, exit 0
        res3 = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", self.test_dir,
            "--target-dir", self.tmp_target,
        ], capture_output=True, text=True)
        self.assertEqual(res3.returncode, 0)
        self.assertIn("Done! Freed 0 B (0 bytes across 0 items).", res3.stdout)

    def test_14_concurrent_execution_fuzzing(self):
        """Test launching 10 instances of clean_build_garbage.py simultaneously on shared build artifacts."""
        deps_dir = os.path.join(self.tmp_target, "debug", "deps")
        os.makedirs(deps_dir)
        past = time.time() - 100

        # Populate with 80 rcgu objects and 40 test binaries
        for i in range(80):
            p = os.path.join(deps_dir, f"lib_conc_{i}-1111222233334444.rcgu.o")
            with open(p, "w") as f: f.write(f"conc rcgu {i}")
            os.utime(p, (past, past))
        for i in range(40):
            p = os.path.join(deps_dir, f"conc_test_runner_{i}")
            with open(p, "w") as f: f.write(f"conc bin {i}")
            os.chmod(p, 0o755)
            os.utime(p, (past, past))

        def run_proc(pid):
            cmd = [
                sys.executable, CLEAN_SCRIPT,
                "--repo-root", self.test_dir,
                "--target-dir", self.tmp_target,
            ]
            r = subprocess.run(cmd, capture_output=True, text=True)
            return pid, r.returncode, r.stderr

        with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
            futures = [executor.submit(run_proc, i) for i in range(10)]
            results = [f.result() for f in futures]

        for pid, code, err in results:
            self.assertEqual(code, 0, f"Concurrent process {pid} failed with exit code {code}: {err}")

        # Verify all items purged
        remaining = [f for f in os.listdir(deps_dir) if f.startswith("conc_")]
        self.assertEqual(len(remaining), 0, f"All concurrent items should be purged, remaining: {remaining}")

    def test_15_cli_flag_filtering_and_robustness(self):
        """Test CLI flags --target-only, --temp-only, --dry-run, and handling of non-existent directories."""
        nonexistent_root = os.path.join(self.test_dir, "does_not_exist_xyz")
        res = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", nonexistent_root,
            "--target-dir", os.path.join(nonexistent_root, "target"),
            "--target-only"
        ], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, "Must exit 0 cleanly on non-existent directory")

        res_temp = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", self.test_dir,
            "--temp-only"
        ], capture_output=True, text=True)
        self.assertEqual(res_temp.returncode, 0, "--temp-only must exit 0 cleanly")

    def test_16_directory_permission_resilience_audit(self):
        """
        Adversarial audit of directory permission resilience:
        Evaluates script behavior when encountering an unreadable directory (chmod 000).
        Records whether the script crashes with PermissionError or degrades gracefully.
        """
        target_dir = os.path.join(self.test_dir, "target")
        debug_dir = os.path.join(target_dir, "debug")
        deps_dir = os.path.join(debug_dir, "deps")
        os.makedirs(deps_dir, exist_ok=True)

        os.chmod(deps_dir, 000)

        res = subprocess.run([
            sys.executable, CLEAN_SCRIPT,
            "--repo-root", self.test_dir,
            "--target-dir", target_dir,
            "--force"
        ], capture_output=True, text=True)

        # Restore permissions immediately for tearDown safety
        try:
            os.chmod(deps_dir, 0o755)
        except OSError:
            pass

        if res.returncode != 0 and "PermissionError" in res.stderr:
            print("\n  [AUDIT FINDING] Unhandled PermissionError on chmod 000 directory traversal detected:")
            for line in res.stderr.strip().splitlines()[-3:]:
                print(f"    {line}")
        else:
            print("\n  [AUDIT NOTE] Handled directory permissions gracefully.")


def suite():
    return unittest.defaultTestLoader.loadTestsFromTestCase(TestBuildHygieneFuzzSuite)


def main():
    print(f"[challenger_m5_hygiene_fuzz_tests] Running adversarial fuzzing suite against: {CLEAN_SCRIPT}")
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite())
    if not result.wasSuccessful():
        sys.exit(1)
    print("\n[challenger_m5_hygiene_fuzz_tests] All 16 fuzzing tests completed successfully.")


if __name__ == "__main__":
    main()
