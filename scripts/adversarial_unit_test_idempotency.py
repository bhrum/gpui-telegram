#!/usr/bin/env python3
"""
Adversarial Verification Suite for Unit Test Idempotency, In-Memory Isolation,
and Zero Residual Test Databases (AGENTS.md Rule 3).

Target: crates/gpui_telegram_server
Author: challenger_cloud_m2_iter3_1
"""

import glob
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time

PROJECT_ROOT = "/Users/bhrum/teamwork_projects/gpui_telegram"
SERVER_CRATE_DIR = os.path.join(PROJECT_ROOT, "crates", "gpui_telegram_server")

GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"

passed_assertions = 0
failed_assertions = 0


def record_result(desc: str, passed: bool, detail: str = ""):
    global passed_assertions, failed_assertions
    if passed:
        passed_assertions += 1
        print(f"  {GREEN}[PASS]{RESET} {desc} {f'({detail})' if detail else ''}")
    else:
        failed_assertions += 1
        print(f"  {RED}[FAIL]{RESET} {desc} - {detail}")


def run_command(cmd, cwd=PROJECT_ROOT, timeout=120):
    start = time.time()
    res = subprocess.run(
        cmd,
        cwd=cwd,
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=timeout,
    )
    elapsed = time.time() - start
    return res.returncode, res.stdout, res.stderr, elapsed


def check_residuals():
    """Check for residual database files and temporary test directories."""
    residuals = []

    # 1. crates/gpui_telegram_server/data
    server_data_dir = os.path.join(SERVER_CRATE_DIR, "data")
    if os.path.exists(server_data_dir):
        residuals.append(f"Directory exists: {server_data_dir}")

    # 2. Any .db, .db-wal, .db-shm under crates/gpui_telegram_server
    for pattern in ["*.db", "*.db-wal", "*.db-shm"]:
        for match in glob.glob(f"{SERVER_CRATE_DIR}/**/{pattern}", recursive=True):
            residuals.append(f"Residual DB file: {match}")

    # 3. Any gpui_test_persist_* in /tmp and std::env::temp_dir()
    temp_dirs_to_check = set(["/tmp", tempfile.gettempdir()])
    for t_dir in temp_dirs_to_check:
        if os.path.exists(t_dir):
            for match in glob.glob(os.path.join(t_dir, "gpui_test_persist_*")):
                residuals.append(f"Orphaned temp dir: {match}")

    return residuals


def test_consecutive_runs():
    print(f"\n{BOLD}{CYAN}=== 1. Stress Testing 3 Consecutive Test Executions ==={RESET}")
    cmd = "cargo test -p gpui_telegram_server --test server_routes_auth_tests"

    for run_idx in range(1, 4):
        print(f"\n---> Executing Run #{run_idx}: `{cmd}`")
        code, stdout, stderr, elapsed = run_command(cmd)

        all_output = stdout + "\n" + stderr
        match = re.search(r"test result: (ok|FAILED)\. (\d+) passed; (\d+) failed", all_output)

        passed_count = int(match.group(2)) if match else 0
        failed_count = int(match.group(3)) if match else -1
        is_ok = (code == 0) and (failed_count == 0) and (passed_count == 10)

        record_result(
            f"Run #{run_idx} execution exit code 0",
            code == 0,
            f"code={code}, elapsed={elapsed:.2f}s",
        )
        record_result(
            f"Run #{run_idx} exactly 10/10 tests passed",
            passed_count == 10 and failed_count == 0,
            f"passed={passed_count}, failed={failed_count}",
        )
        # Check for test failures / panics / unexpected 409 conflict failures
        has_test_panic = ("panicked at" in all_output) or ("FAILED" in all_output)
        has_409_conflict_failure = ("409 Conflict" in all_output and "FAILED" in all_output) or ("left: 409" in all_output)
        record_result(
            f"Run #{run_idx} zero 409 Conflict regressions or panics",
            (not has_test_panic) and (not has_409_conflict_failure),
            "clean test pass with no assertion failures",
        )

        # Check residuals immediately after this run
        residuals = check_residuals()
        record_result(
            f"Run #{run_idx} post-run zero filesystem residuals",
            len(residuals) == 0,
            f"{len(residuals)} residuals found: {residuals}",
        )


def test_tempdbguard_raii_unwinding():
    print(f"\n{BOLD}{CYAN}=== 2. Adversarial Panic Unwinding RAII Challenge ==={RESET}")

    # Locate libuuid rlib in target
    target_deps = os.path.join(PROJECT_ROOT, "target", "debug", "deps")
    uuid_rlibs = glob.glob(os.path.join(target_deps, "libuuid-*.rlib"))
    if not uuid_rlibs:
        print(f"  {RED}[ERROR]{RESET} No libuuid-*.rlib found in {target_deps}")
        record_result("Find libuuid rlib", False, "Missing compiled uuid rlib")
        return

    chosen_uuid = uuid_rlibs[0]

    rust_harness_code = r"""
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::thread;

/// Exact implementation from crates/gpui_telegram_server/tests/server_routes_auth_tests.rs
#[allow(dead_code)]
struct TempDbGuard {
    dir: std::path::PathBuf,
    db_path: std::path::PathBuf,
}

#[allow(dead_code)]
impl TempDbGuard {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!("gpui_test_persist_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).expect("Failed to create isolated temp directory for test DB");
        let db_path = dir.join("test_persist.db");
        Self { dir, db_path }
    }

    fn db_url(&self) -> String {
        format!("sqlite://{}", self.db_path.display())
    }
}

impl Drop for TempDbGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn main() {
    // -------------------------------------------------------------
    // Test Scenario A: Direct panic unwinding in catch_unwind
    // -------------------------------------------------------------
    let mut observed_dir = PathBuf::new();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let guard = TempDbGuard::new();
        observed_dir = guard.dir.clone();
        assert!(observed_dir.exists(), "Guard directory must exist immediately upon creation");
        panic!("Intentional panic inside TempDbGuard scope");
    }));
    assert!(result.is_err(), "catch_unwind must capture the panic");
    assert!(!observed_dir.exists(), "TempDbGuard directory must be purged upon panic unwinding! Dir was: {:?}", observed_dir);
    println!("SCENARIO_A_PASSED");

    // -------------------------------------------------------------
    // Test Scenario B: Panic after generating full SQLite file suite (.db, -wal, -shm)
    // -------------------------------------------------------------
    let mut observed_dir_b = PathBuf::new();
    let result_b = catch_unwind(AssertUnwindSafe(|| {
        let guard = TempDbGuard::new();
        observed_dir_b = guard.dir.clone();

        // Simulate active SQLite database write + WAL + SHM + subdirectories
        std::fs::write(&guard.db_path, b"SQLITE FORMAT 3 HEAVY TEST DATA").unwrap();
        std::fs::write(guard.dir.join("test_persist.db-wal"), b"WAL LOG RECORDS").unwrap();
        std::fs::write(guard.dir.join("test_persist.db-shm"), b"SHM SHARED INDEX").unwrap();
        let sub = guard.dir.join("nested_cache");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("chunk.bin"), b"CHUNKS").unwrap();

        assert!(guard.db_path.exists());
        assert!(guard.dir.join("test_persist.db-wal").exists());
        assert!(guard.dir.join("test_persist.db-shm").exists());
        assert!(sub.join("chunk.bin").exists());

        panic!("Simulated crash while SQLite transactions and WAL active");
    }));
    assert!(result_b.is_err());
    assert!(!observed_dir_b.exists(), "Multi-file temp dir must be completely unlinked on unwind! Dir: {:?}", observed_dir_b);
    println!("SCENARIO_B_PASSED");

    // -------------------------------------------------------------
    // Test Scenario C: Thread crash unwinding
    // -------------------------------------------------------------
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let guard = TempDbGuard::new();
        tx.send(guard.dir.clone()).unwrap();
        assert!(guard.dir.exists());
        panic!("Worker thread fatal panic holding TempDbGuard");
    });
    let thread_dir = rx.recv().unwrap();
    let join_res = handle.join();
    assert!(join_res.is_err(), "Child thread must terminate with panic error");
    assert!(!thread_dir.exists(), "Thread panic unwinding must drop TempDbGuard and purge directory: {:?}", thread_dir);
    println!("SCENARIO_C_PASSED");

    // -------------------------------------------------------------
    // Test Scenario D: High-frequency panic storm (50 consecutive panics)
    // -------------------------------------------------------------
    for i in 0..50 {
        let mut storm_dir = PathBuf::new();
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let guard = TempDbGuard::new();
            storm_dir = guard.dir.clone();
            std::fs::write(&guard.db_path, format!("Storm iteration {}", i)).unwrap();
            panic!("Storm iteration panic {}", i);
        }));
        assert!(!storm_dir.exists(), "Storm directory {} failed to drop cleanly", i);
    }
    println!("SCENARIO_D_PASSED");

    println!("ALL_RAII_SCENARIOS_PASSED");
}
"""

    temp_rs = "/tmp/adversarial_unwind_harness.rs"
    temp_bin = "/tmp/adversarial_unwind_harness"

    try:
        with open(temp_rs, "w") as f:
            f.write(rust_harness_code)

        compile_cmd = f"rustc -L {target_deps} --extern uuid={chosen_uuid} {temp_rs} -o {temp_bin}"
        code, stdout, stderr, elapsed = run_command(compile_cmd)
        record_result(
            "Compile adversarial RAII panic unwinding test harness",
            code == 0,
            f"elapsed={elapsed:.2f}s, errors={stderr.strip()}",
        )

        if code != 0:
            return

        # Execute harness
        code, stdout, stderr, elapsed = run_command(temp_bin)
        output = stdout + "\n" + stderr
        record_result(
            "Scenario A: catch_unwind stack unwinding purges isolated temp dir",
            "SCENARIO_A_PASSED" in output,
        )
        record_result(
            "Scenario B: Drop purges directory with active .db, -wal, -shm, and subdirs",
            "SCENARIO_B_PASSED" in output,
        )
        record_result(
            "Scenario C: Child thread panic unwinding purges TempDbGuard directory",
            "SCENARIO_C_PASSED" in output,
        )
        record_result(
            "Scenario D: 50 consecutive rapid panic storms leave 0 residual temp dirs",
            "SCENARIO_D_PASSED" in output,
        )
        record_result(
            "Overall RAII panic harness exited successfully",
            code == 0 and "ALL_RAII_SCENARIOS_PASSED" in output,
            f"elapsed={elapsed:.2f}s",
        )

    finally:
        if os.path.exists(temp_rs):
            os.remove(temp_rs)
        if os.path.exists(temp_bin):
            os.remove(temp_bin)


def test_build_hygiene_cleanup():
    print(f"\n{BOLD}{CYAN}=== 3. AGENTS.md Build Hygiene Cleanup ==={RESET}")
    clean_script = os.path.join(PROJECT_ROOT, "scripts", "clean_build_garbage.py")
    cmd = f"python3 {clean_script}"
    code, stdout, stderr, elapsed = run_command(cmd)

    record_result(
        "Execute scripts/clean_build_garbage.py",
        code == 0,
        f"output={stdout.strip()}",
    )


def main():
    print(f"{BOLD}{CYAN}================================================================={RESET}")
    print(f"{BOLD}{CYAN}  ADVERSARIAL STRESS TEST: IDEMPOTENCY & ZERO DB RESIDUALS       {RESET}")
    print(f"{BOLD}{CYAN}================================================================={RESET}")

    # Initial residual check before tests
    initial_residuals = check_residuals()
    record_result(
        "Baseline check: zero residual database or temp files prior to tests",
        len(initial_residuals) == 0,
        f"{initial_residuals}",
    )

    test_consecutive_runs()
    test_tempdbguard_raii_unwinding()
    test_build_hygiene_cleanup()

    # Final deep residual check
    final_residuals = check_residuals()
    record_result(
        "Final check: zero residual database or temp files across all locations",
        len(final_residuals) == 0,
        f"{final_residuals}",
    )

    print(f"\n{BOLD}================================================================={RESET}")
    print(f"{BOLD}Summary: {GREEN}{passed_assertions} Passed{RESET}, {RED}{failed_assertions} Failed{RESET}")
    print(f"{BOLD}================================================================={RESET}")

    if failed_assertions > 0:
        print(f"\n{RED}VERDICT: REQUEST_CHANGES ({failed_assertions} assertions failed){RESET}\n")
        sys.exit(1)
    else:
        print(f"\n{GREEN}VERDICT: APPROVE (All {passed_assertions} assertions passed){RESET}\n")
        sys.exit(0)


if __name__ == "__main__":
    main()
