#!/usr/bin/env python3
"""
tests/e2e/verify_cloud_delivery.py
Comprehensive End-to-End Cloud Delivery & Release Verification Runner.

Validates the full delivery pipeline:
  Part 1: Cloud VM & Network Reachability (TCP ping on ports 8080 and 8788).
  Part 2: Backend Health Checks (8080 /health, /api/v1/status; 8788 /health).
  Part 3: Live User Authentication & Session Persistence (Register, Login, /me, negative auth, StoredSession file).
  Part 4: Authentic AI Bot Interaction (ACP Agent Runs on 8080 & SSE Stream on 8788).
  Part 5: Released Desktop Binary Execution Check (gpui_telegram_desktop --version & --help).
  Part 6: Build Hygiene Check (scripts/clean_build_garbage.py & target cache inspection).

Governed by:
  - ORIGINAL_REQUEST.md (§R1-R4)
  - PROJECT.md (§M4 F15)
  - AGENTS.md
"""

import argparse
import glob
import json
import os
import socket
import stat
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Any, Dict, List, Optional, Tuple

# ANSI Colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"


class E2EVerificationRunner:
    def __init__(
        self,
        server_url: str = "http://34.170.16.35:8080",
        gateway_url: str = "http://34.170.16.35:8788",
        repo_root: Optional[str] = None,
        desktop_binary: Optional[str] = None,
        skip_binary: bool = False,
        verbose: bool = False,
    ):
        self.server_url = server_url.rstrip("/")
        self.gateway_url = gateway_url.rstrip("/")
        self.server_host, self.server_port = self._parse_host_port(self.server_url, 8080)
        self.gateway_host, self.gateway_port = self._parse_host_port(self.gateway_url, 8788)
        self.repo_root = repo_root or self._detect_repo_root()
        self.desktop_binary = desktop_binary or os.path.join(
            self.repo_root, "target", "release", "gpui_telegram_desktop"
        )
        self.skip_binary = skip_binary
        self.verbose = verbose

        self.passed_count = 0
        self.failed_count = 0
        self.warn_count = 0
        self.results: List[Dict[str, Any]] = []

    @staticmethod
    def _parse_host_port(url: str, default_port: int) -> Tuple[str, int]:
        parsed = urllib.parse.urlparse(url)
        host = parsed.hostname or "127.0.0.1"
        port = parsed.port or default_port
        return host, port

    @staticmethod
    def _detect_repo_root() -> str:
        candidates = [
            os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")),
            os.path.abspath(os.getcwd()),
            "/Users/bhrum/teamwork_projects/gpui_telegram",
        ]
        for c in candidates:
            if os.path.isfile(os.path.join(c, "Cargo.toml")) and os.path.isdir(os.path.join(c, "crates")):
                return c
        return candidates[0]

    def record(self, test_id: str, desc: str, passed: bool, detail: str = "", is_warn: bool = False):
        if passed:
            self.passed_count += 1
            status_str = f"{GREEN}[PASS]{RESET}"
            self.results.append({"id": test_id, "desc": desc, "status": "PASS", "detail": detail})
        elif is_warn:
            self.warn_count += 1
            status_str = f"{YELLOW}[WARN]{RESET}"
            self.results.append({"id": test_id, "desc": desc, "status": "WARN", "detail": detail})
        else:
            self.failed_count += 1
            status_str = f"{RED}[FAIL]{RESET}"
            self.results.append({"id": test_id, "desc": desc, "status": "FAIL", "detail": detail})

        detail_suffix = f" - {detail}" if detail else ""
        print(f"  {status_str} {BOLD}{test_id}{RESET}: {desc}{detail_suffix}")

    def http_request(
        self,
        base_url: str,
        path: str,
        method: str = "GET",
        body: Optional[Dict[str, Any]] = None,
        token: Optional[str] = None,
        timeout: int = 15,
    ) -> Tuple[int, Any, float]:
        url = f"{base_url}{path}"
        headers = {"Content-Type": "application/json"}
        if token:
            headers["Authorization"] = f"Bearer {token}"

        data = json.dumps(body).encode("utf-8") if body is not None else None
        req = urllib.request.Request(url, data=data, headers=headers, method=method)

        start = time.time()
        try:
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                elapsed = time.time() - start
                content = resp.read().decode("utf-8")
                try:
                    parsed = json.loads(content)
                except Exception:
                    parsed = content
                return resp.status, parsed, elapsed
        except urllib.error.HTTPError as e:
            elapsed = time.time() - start
            err_content = e.read().decode("utf-8")
            try:
                parsed = json.loads(err_content)
            except Exception:
                parsed = err_content
            return e.code, parsed, elapsed
        except Exception as e:
            elapsed = time.time() - start
            return 0, str(e), elapsed

    def test_tcp_ping(self, host: str, port: int, timeout: float = 3.0) -> Tuple[bool, float, str]:
        start = time.time()
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(timeout)
        try:
            s.connect((host, port))
            s.close()
            elapsed = time.time() - start
            return True, elapsed, f"Connected to {host}:{port} in {elapsed * 1000:.1f}ms"
        except Exception as e:
            elapsed = time.time() - start
            return False, elapsed, str(e)

    # -------------------------------------------------------------------------
    # PART 1: Cloud VM & Network Reachability
    # -------------------------------------------------------------------------
    def run_part1_network_reachability(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 1: Cloud VM & Network Reachability{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        # 1.1 TCP Probe to Rust Server (8080)
        ok, elapsed, detail = self.test_tcp_ping(self.server_host, self.server_port)
        self.record(
            "NET-01",
            f"TCP socket reachability to Rust Axum Server ({self.server_host}:{self.server_port})",
            ok,
            detail,
        )

        # 1.2 TCP Probe to AI Gateway (8788)
        ok, elapsed, detail = self.test_tcp_ping(self.gateway_host, self.gateway_port)
        self.record(
            "NET-02",
            f"TCP socket reachability to AI Gateway ({self.gateway_host}:{self.gateway_port})",
            ok,
            detail,
        )

    # -------------------------------------------------------------------------
    # PART 2: Backend Health Checks
    # -------------------------------------------------------------------------
    def run_part2_backend_health(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 2: Backend Health Checks{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        # 2.1 Rust Server /health
        status, body, elapsed = self.http_request(self.server_url, "/health")
        is_ok = status == 200 and body.strip() == "OK"
        self.record(
            "HLT-01",
            "GET /health on Rust Axum server returns 200 OK ('OK')",
            is_ok,
            f"status={status}, latency={elapsed:.3f}s",
        )

        # 2.2 Rust Server /api/v1/status
        status, body, elapsed = self.http_request(self.server_url, "/api/v1/status")
        is_ok = (
            status == 200
            and isinstance(body, dict)
            and body.get("server") == "gpui_telegram_server"
            and body.get("status") == "healthy"
        )
        self.record(
            "HLT-02",
            "GET /api/v1/status returns healthy status payload",
            is_ok,
            f"status={status}, server={body.get('server') if isinstance(body, dict) else None}, latency={elapsed:.3f}s",
        )

        # 2.3 AI Gateway /health
        status, body, elapsed = self.http_request(self.gateway_url, "/health")
        is_ok = (
            status == 200
            and isinstance(body, dict)
            and body.get("status") == "ok"
            and body.get("service") == "dacheng-ai-backend"
        )
        self.record(
            "HLT-03",
            "GET /health on AI Gateway (8788) returns 200 OK ('dacheng-ai-backend')",
            is_ok,
            f"status={status}, service={body.get('service') if isinstance(body, dict) else None}, latency={elapsed:.3f}s",
        )

    # -------------------------------------------------------------------------
    # PART 3: Live User Authentication & Session Persistence
    # -------------------------------------------------------------------------
    def run_part3_auth_and_session(self) -> Tuple[Optional[str], Optional[str], Optional[int]]:
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 3: Live User Authentication & Session Persistence{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        ts = int(time.time() * 1000)
        username = f"e2e_cloud_user_{ts}"
        password = f"SecureP@ss_{ts}!"
        first_name = "CloudDeliverTester"

        # 3.1 Register unique user
        reg_payload = {
            "username": username,
            "password": password,
            "first_name": first_name,
            "device_name": "E2E-Automated-Runner",
        }
        status, resp, elapsed = self.http_request(
            self.server_url, "/api/v1/auth/register", method="POST", body=reg_payload
        )
        reg_ok = (
            status == 201
            and isinstance(resp, dict)
            and resp.get("user_id") is not None
            and bool(resp.get("token"))
            and bool(resp.get("session_id"))
        )
        user_id = resp.get("user_id") if isinstance(resp, dict) else None
        session_id = resp.get("session_id") if isinstance(resp, dict) else None
        token = resp.get("token") if isinstance(resp, dict) else None
        self.record(
            "AUTH-01",
            f"Register unique user '{username}' returns HTTP 201 Created with valid token & session_id",
            reg_ok,
            f"status={status}, user_id={user_id}, latency={elapsed:.3f}s",
        )

        # 3.2 Duplicate registration conflict check
        status, resp, elapsed = self.http_request(
            self.server_url, "/api/v1/auth/register", method="POST", body=reg_payload
        )
        self.record(
            "AUTH-02",
            f"Duplicate registration for '{username}' returns HTTP 409 Conflict",
            status == 409,
            f"status={status}",
        )

        # 3.3 Bad credentials rejected
        bad_login_payload = {
            "username": username,
            "password": "WrongPassword999!",
            "device_name": "Attacker-Rig",
        }
        status, resp, elapsed = self.http_request(
            self.server_url, "/api/v1/auth/login", method="POST", body=bad_login_payload
        )
        self.record(
            "AUTH-03",
            "Login with incorrect password returns HTTP 401 Unauthorized",
            status == 401,
            f"status={status}",
        )

        # 3.4 Valid Login
        login_payload = {
            "username": username,
            "password": password,
            "device_name": "E2E-Automated-Runner",
        }
        status, resp, elapsed = self.http_request(
            self.server_url, "/api/v1/auth/login", method="POST", body=login_payload
        )
        login_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("token") is not None
            and resp.get("user_id") == user_id
        )
        if login_ok:
            token = resp["token"]
            session_id = resp["session_id"]
        self.record(
            "AUTH-04",
            "Valid user login returns HTTP 200 OK and fresh session token",
            login_ok,
            f"status={status}, user_id={user_id}, latency={elapsed:.3f}s",
        )

        # 3.5 Authorized GET /api/v1/auth/me
        status, resp, elapsed = self.http_request(self.server_url, "/api/v1/auth/me", token=token)
        me_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("user_id") == user_id
            and resp.get("username") == username
        )
        self.record(
            "AUTH-05",
            "GET /api/v1/auth/me with Bearer token returns matching user profile",
            me_ok,
            f"status={status}, username={resp.get('username') if isinstance(resp, dict) else None}",
        )

        # 3.6 Forged token rejected
        status, resp, elapsed = self.http_request(
            self.server_url, "/api/v1/auth/me", token="forged_token_0123456789abcdef"
        )
        self.record(
            "AUTH-06",
            "GET /api/v1/auth/me with forged token returns HTTP 401 Unauthorized",
            status == 401,
            f"status={status}",
        )

        # 3.7 Simulated Client Session Persistence across Cold Restart
        session_persistence_ok = False
        with tempfile.TemporaryDirectory() as tmp_dir:
            config_dir = os.path.join(tmp_dir, ".config", "gpui_telegram")
            os.makedirs(config_dir, exist_ok=True)
            session_file = os.path.join(config_dir, "session.json")

            stored_session = {
                "server_url": self.server_url,
                "user_id": user_id,
                "username": username,
                "token": token,
                "session_id": session_id,
                "created_at_ms": int(time.time() * 1000),
            }

            # Save session atomically with 0600 permissions
            with open(session_file, "w") as f:
                json.dump(stored_session, f, indent=2)
            os.chmod(session_file, 0o600)

            # Check permissions
            st = os.stat(session_file)
            perm_ok = oct(st.st_mode)[-3:] == "600"

            # Simulate client cold restart by reading back session from disk
            with open(session_file, "r") as f:
                reloaded = json.load(f)

            # Validate loaded token against cloud VM
            reloaded_token = reloaded.get("token")
            status, reloaded_me, _ = self.http_request(
                self.server_url, "/api/v1/auth/me", token=reloaded_token
            )
            session_persistence_ok = (
                perm_ok
                and reloaded == stored_session
                and status == 200
                and isinstance(reloaded_me, dict)
                and reloaded_me.get("user_id") == user_id
            )

        self.record(
            "AUTH-07",
            "Simulated client cold restart: session.json serialized (0600) and reloaded token validates against cloud",
            session_persistence_ok,
            f"permissions_0600={perm_ok}, status={status}",
        )

        return token, session_id, user_id

    # -------------------------------------------------------------------------
    # PART 4: Authentic AI Bot Interaction
    # -------------------------------------------------------------------------
    def run_part4_ai_bot_interaction(self, token: Optional[str], session_id: Optional[str]):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 4: Authentic AI Bot Interaction{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        if not token or not session_id:
            self.record("AI-01", "AI Bot Turn execution", False, "Missing valid session token from Part 3")
            return

        # 4.1 Axum ACP Runner AI Agent Turn (8080)
        agent_payload = {
            "prompt": "Autonomous cloud verification prompt: compute 128 * 64 + 42, explain reasoning.",
            "session_id": session_id,
            "model_id": "grok-3",
            "thinking_budget_tokens": 1024,
        }
        status, resp, elapsed = self.http_request(
            self.server_url,
            "/api/v1/agent/runs",
            method="POST",
            body=agent_payload,
            token=token,
            timeout=30,
        )
        is_completed = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("status") == "completed"
            and len(resp.get("response", "")) > 10
        )
        outputs = resp.get("outputs", []) if isinstance(resp, dict) else []
        has_thinking = any(isinstance(o, dict) and "Thinking" in o for o in outputs)
        has_final = any(isinstance(o, dict) and "FinalResponse" in o for o in outputs)

        self.record(
            "AI-01",
            "POST /api/v1/agent/runs executes autonomous ACP agent (status=completed)",
            is_completed,
            f"status={status}, latency={elapsed:.3f}s, resp_len={len(resp.get('response', '')) if isinstance(resp, dict) else 0}",
        )
        self.record(
            "AI-02",
            "AI Bot outputs contain structured 'Thinking' and 'FinalResponse' tokens",
            has_thinking and has_final,
            f"outputs_count={len(outputs)}, has_thinking={has_thinking}, has_final={has_final}",
        )

        # 4.2 Node.js AI Gateway SSE Stream Probe (8788)
        stream_payload = json.dumps({"message": "E2E verification ping", "model": "deepseek-chat"}).encode(
            "utf-8"
        )
        req = urllib.request.Request(
            f"{self.gateway_url}/api/ai/chat/stream",
            data=stream_payload,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        stream_ok = False
        first_event = None
        try:
            with urllib.request.urlopen(req, timeout=15) as r:
                if r.status == 200:
                    lines = [r.readline().decode("utf-8", errors="replace").strip() for _ in range(6)]
                    for line in lines:
                        if line.startswith("event: meta") or line.startswith("data:"):
                            stream_ok = True
                            first_event = line
                            break
        except Exception as e:
            first_event = str(e)

        self.record(
            "AI-03",
            "POST /api/ai/chat/stream on AI Gateway (8788) initiates SSE stream and emits meta event",
            stream_ok,
            f"event_preview={first_event}",
        )

    # -------------------------------------------------------------------------
    # PART 5: Released Desktop Binary Execution Check
    # -------------------------------------------------------------------------
    def run_part5_desktop_binary(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 5: Released Desktop Binary Execution Check{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        if self.skip_binary:
            self.record("BIN-01", "Desktop binary checks skipped via --skip-binary flag", True, is_warn=True)
            return

        binary_path = os.path.abspath(self.desktop_binary)
        if not os.path.isfile(binary_path):
            # Check dist/bin/gpui_telegram_desktop fallback
            dist_binary = os.path.join(self.repo_root, "dist", "bin", "gpui_telegram_desktop")
            if os.path.isfile(dist_binary):
                binary_path = dist_binary

        if not os.path.isfile(binary_path):
            self.record(
                "BIN-01",
                f"Release binary exists at {binary_path}",
                False,
                "Binary not found. Ensure `cargo build --release -p gpui_telegram_desktop` has been run.",
            )
            return

        # Check executable permission
        is_exec = os.access(binary_path, os.X_OK)
        self.record("BIN-01", f"Release binary exists and is executable ({binary_path})", is_exec)

        # Test --version flag
        try:
            res = subprocess.run(
                [binary_path, "--version"],
                capture_output=True,
                text=True,
                timeout=5,
            )
            has_version = res.returncode == 0 and "gpui_telegram_desktop" in res.stdout
            self.record(
                "BIN-02",
                "Execute `gpui_telegram_desktop --version` returns exit code 0 and proper version string",
                has_version,
                f"returncode={res.returncode}, stdout={res.stdout.strip()[:60]}",
            )
        except Exception as e:
            self.record("BIN-02", "Execute `gpui_telegram_desktop --version`", False, str(e))

        # Test --help flag
        try:
            res = subprocess.run(
                [binary_path, "--help"],
                capture_output=True,
                text=True,
                timeout=5,
            )
            has_help = res.returncode == 0 and ("--server-url" in res.stdout or "USAGE" in res.stdout)
            self.record(
                "BIN-03",
                "Execute `gpui_telegram_desktop --help` returns exit code 0 and usage options",
                has_help,
                f"returncode={res.returncode}, stdout_len={len(res.stdout)}",
            )
        except Exception as e:
            self.record("BIN-03", "Execute `gpui_telegram_desktop --help`", False, str(e))

    # -------------------------------------------------------------------------
    # PART 6: Build Hygiene Check (AGENTS.md)
    # -------------------------------------------------------------------------
    def run_part6_build_hygiene(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Part 6: Build Hygiene Check (AGENTS.md Compliance){RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        clean_script = os.path.join(self.repo_root, "scripts", "clean_build_garbage.py")
        script_exists = os.path.isfile(clean_script)
        self.record("HYG-01", f"Verification utility exists at {clean_script}", script_exists)

        if not script_exists:
            return

        # Execute cleaner
        try:
            res = subprocess.run(
                [sys.executable, clean_script],
                cwd=self.repo_root,
                capture_output=True,
                text=True,
                timeout=30,
            )
            clean_ok = res.returncode == 0
            self.record(
                "HYG-02",
                "Execute `python3 scripts/clean_build_garbage.py` returns exit code 0",
                clean_ok,
                f"stdout={res.stdout.strip()}",
            )
        except Exception as e:
            self.record("HYG-02", "Execute `clean_build_garbage.py`", False, str(e))

        # Inspect target directory
        target_dir = os.path.join(self.repo_root, "target")
        if os.path.isdir(target_dir):
            # Check for leftover incremental directories
            inc_dirs = glob.glob(os.path.join(target_dir, "*", "incremental"))
            self.record(
                "HYG-03",
                "Zero leftover incremental cache directories in target/",
                len(inc_dirs) == 0,
                f"found={len(inc_dirs)} ({inc_dirs})",
            )

            # Check for *.dSYM debug packages
            dsym_dirs = glob.glob(os.path.join(target_dir, "*", "deps", "*.dSYM"))
            self.record(
                "HYG-04",
                "Zero leftover *.dSYM debug symbol bundles in target/*/deps/",
                len(dsym_dirs) == 0,
                f"found={len(dsym_dirs)}",
            )

    # -------------------------------------------------------------------------
    # Master Execution & Summary
    # -------------------------------------------------------------------------
    def run_all(self) -> int:
        start_time = time.time()
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"{BLUE}  Fabushi Cloud Migration & GPUI Desktop Delivery E2E Suite  {RESET}")
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"Server Target:   {self.server_url}")
        print(f"Gateway Target:  {self.gateway_url}")
        print(f"Repository Root: {self.repo_root}")
        print(f"Desktop Binary:  {self.desktop_binary}")
        print(f"Start Timestamp: {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime())}")

        self.run_part1_network_reachability()
        self.run_part2_backend_health()
        token, session_id, _ = self.run_part3_auth_and_session()
        self.run_part4_ai_bot_interaction(token, session_id)
        self.run_part5_desktop_binary()
        self.run_part6_build_hygiene()

        elapsed = time.time() - start_time
        print(f"\n{BLUE}{'='*70}{RESET}")
        print(f"{BLUE}                E2E Verification Execution Summary                   {RESET}")
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"  Total Assertions Passed:  {GREEN}{self.passed_count}{RESET}")
        print(f"  Total Assertions Failed:  {RED}{self.failed_count}{RESET}")
        print(f"  Total Warnings:           {YELLOW}{self.warn_count}{RESET}")
        print(f"  Total Execution Time:     {elapsed:.2f}s")

        if self.failed_count == 0:
            print(f"\n{GREEN}{BOLD}>>> VERDICT: APPROVE — ALL DELIVERY CRITERIA SATISFIED! <<<{RESET}\n")
            return 0
        else:
            print(f"\n{RED}{BOLD}>>> VERDICT: DISAPPROVE — {self.failed_count} CRITERIA FAILED! <<<{RESET}\n")
            return 1


def main():
    parser = argparse.ArgumentParser(
        description="Comprehensive E2E Verification Runner for Fabushi Cloud Migration & GPUI Desktop Delivery."
    )
    parser.add_argument(
        "--server-url",
        default=os.getenv("GPUI_TELEGRAM_SERVER_URL", "http://34.170.16.35:8080"),
        help="Base URL for Rust Axum Telegram Server (default: http://34.170.16.35:8080)",
    )
    parser.add_argument(
        "--gateway-url",
        default="http://34.170.16.35:8788",
        help="Base URL for Node.js AI Gateway (default: http://34.170.16.35:8788)",
    )
    parser.add_argument(
        "--repo-root",
        default=None,
        help="Root directory of gpui_telegram repository",
    )
    parser.add_argument(
        "--desktop-binary",
        default=None,
        help="Path to gpui_telegram_desktop binary (default: target/release/gpui_telegram_desktop)",
    )
    parser.add_argument(
        "--skip-binary",
        action="store_true",
        help="Skip Part 5 desktop binary execution checks if verifying cloud services only",
    )
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose assertion output",
    )
    args = parser.parse_args()

    runner = E2EVerificationRunner(
        server_url=args.server_url,
        gateway_url=args.gateway_url,
        repo_root=args.repo_root,
        desktop_binary=args.desktop_binary,
        skip_binary=args.skip_binary,
        verbose=args.verbose,
    )
    sys.exit(runner.run_all())


if __name__ == "__main__":
    main()
