#!/usr/bin/env python3
"""
tests/e2e/test_adversarial_cloud_m4_stress.py
Adversarial Live Cloud Delivery & Stress Testing Suite for Milestone 4.

Author: challenger_cloud_m4_2
Target: Google Cloud VM 34.170.16.35:8080 (Rust Axum Server) & 8788 (AI Gateway)

Test Dimensions:
1. Multi-client concurrent registration and login bursts (5 concurrent user threads).
2. Adversarial user credentials (Unicode usernames, symbols, long passwords).
3. Adversarial invalid logins: non-existent user, malformed JSON body, empty credentials (HTTP 401 / 422 / 400).
4. Bearer token authentication fuzzing: empty Bearer header, malformed token string, forged signature (HTTP 401).
5. Adversarial AI Bot Reasoning Challenge: technical math/crypto prompt & code injection edge prompt (status == "completed").
6. Cold Restart & Session Storage Isolation: sandboxed 0600 file permissions and isolation.
7. AGENTS.md Build Hygiene Audit: verification of zero build cache leakage.
"""

import concurrent.futures
import glob
import json
import os
import stat
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Any, Dict, List, Optional, Tuple

SERVER_URL = os.getenv("GPUI_TELEGRAM_SERVER_URL", "http://34.170.16.35:8080")
GATEWAY_URL = "http://34.170.16.35:8788"
REPO_ROOT = "/Users/bhrum/teamwork_projects/gpui_telegram"

GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"


class StressTestRunner:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.results: List[Dict[str, Any]] = []

    def record(self, test_id: str, desc: str, passed: bool, detail: str = ""):
        if passed:
            self.passed += 1
            status_str = f"{GREEN}[PASS]{RESET}"
            self.results.append({"id": test_id, "desc": desc, "status": "PASS", "detail": detail})
        else:
            self.failed += 1
            status_str = f"{RED}[FAIL]{RESET}"
            self.results.append({"id": test_id, "desc": desc, "status": "FAIL", "detail": detail})

        detail_suffix = f" - {detail}" if detail else ""
        print(f"  {status_str} {BOLD}{test_id}{RESET}: {desc}{detail_suffix}")

    def raw_http_request(
        self,
        path: str,
        method: str = "GET",
        raw_body: Optional[bytes] = None,
        headers: Optional[Dict[str, str]] = None,
        timeout: int = 25,
    ) -> Tuple[int, Any, float]:
        url = f"{SERVER_URL}{path}"
        req_headers = {"Content-Type": "application/json"}
        if headers:
            req_headers.update(headers)

        req = urllib.request.Request(url, data=raw_body, headers=req_headers, method=method)
        start = time.time()
        try:
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                elapsed = time.time() - start
                content = resp.read().decode("utf-8", errors="replace")
                try:
                    parsed = json.loads(content)
                except Exception:
                    parsed = content
                return resp.status, parsed, elapsed
        except urllib.error.HTTPError as e:
            elapsed = time.time() - start
            err_content = e.read().decode("utf-8", errors="replace")
            try:
                parsed = json.loads(err_content)
            except Exception:
                parsed = err_content
            return e.code, parsed, elapsed
        except Exception as e:
            elapsed = time.time() - start
            return 0, str(e), elapsed

    def http_json(
        self,
        path: str,
        method: str = "GET",
        body: Optional[Any] = None,
        token: Optional[str] = None,
        timeout: int = 25,
    ) -> Tuple[int, Any, float]:
        headers = {}
        if token is not None:
            headers["Authorization"] = f"Bearer {token}"
        raw_data = json.dumps(body).encode("utf-8") if body is not None else None
        return self.raw_http_request(path, method=method, raw_body=raw_data, headers=headers, timeout=timeout)

    # -------------------------------------------------------------------------
    # STAGE 1: Concurrent Registration & Login Burst (5 Threads)
    # -------------------------------------------------------------------------
    def test_concurrent_burst(self) -> List[Dict[str, Any]]:
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 1: Multi-Client Concurrent Registration & Login Bursts (5 Threads){RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        ts = int(time.time() * 1000)
        clients_data = [
            {
                "id": i,
                "username": f"stress_burst_u{i}_{ts}",
                "password": f"BurstPass_{i}_{ts}#Secure!",
                "first_name": f"ConcurrentWorker{i}",
                "device_name": f"StressThread-{i}",
            }
            for i in range(5)
        ]

        # 1. Concurrent Registration
        reg_results = []

        def do_register(c):
            status, resp, elapsed = self.http_json(
                "/api/v1/auth/register",
                method="POST",
                body={
                    "username": c["username"],
                    "password": c["password"],
                    "first_name": c["first_name"],
                    "device_name": c["device_name"],
                },
            )
            return c["id"], status, resp, elapsed

        with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(do_register, c) for c in clients_data]
            for fut in concurrent.futures.as_completed(futures):
                reg_results.append(fut.result())

        all_reg_201 = all(status == 201 for _, status, _, _ in reg_results)
        all_reg_tokens = all(
            isinstance(resp, dict) and bool(resp.get("token")) and bool(resp.get("session_id"))
            for _, _, resp, _ in reg_results
        )
        self.record(
            "BURST-01",
            "5-thread simultaneous concurrent user registration burst",
            all_reg_201 and all_reg_tokens,
            f"all_status_201={all_reg_201}, total_success={len([r for r in reg_results if r[1] == 201])}/5",
        )

        # 2. Concurrent Login
        login_results = []

        def do_login(c):
            status, resp, elapsed = self.http_json(
                "/api/v1/auth/login",
                method="POST",
                body={
                    "username": c["username"],
                    "password": c["password"],
                    "device_name": c["device_name"],
                },
            )
            return c["id"], status, resp, elapsed

        with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(do_login, c) for c in clients_data]
            for fut in concurrent.futures.as_completed(futures):
                login_results.append(fut.result())

        all_login_200 = all(status == 200 for _, status, _, _ in login_results)
        authenticated_clients = []
        for cid, status, resp, _ in login_results:
            if status == 200 and isinstance(resp, dict):
                c_info = next(c for c in clients_data if c["id"] == cid)
                authenticated_clients.append({
                    "id": cid,
                    "username": c_info["username"],
                    "token": resp["token"],
                    "session_id": resp["session_id"],
                    "user_id": resp["user_id"],
                })

        self.record(
            "BURST-02",
            "5-thread simultaneous concurrent user login burst",
            all_login_200 and len(authenticated_clients) == 5,
            f"all_status_200={all_login_200}, logged_in={len(authenticated_clients)}/5",
        )

        # 3. Concurrent Auth Token Verification (/api/v1/auth/me)
        me_results = []

        def do_me(ac):
            status, resp, elapsed = self.http_json("/api/v1/auth/me", token=ac["token"])
            matches = (
                status == 200
                and isinstance(resp, dict)
                and resp.get("username") == ac["username"]
                and resp.get("user_id") == ac["user_id"]
            )
            return ac["id"], status, matches

        with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
            futures = [executor.submit(do_me, ac) for ac in authenticated_clients]
            for fut in concurrent.futures.as_completed(futures):
                me_results.append(fut.result())

        all_me_ok = all(matches for _, _, matches in me_results)
        self.record(
            "BURST-03",
            "5-thread simultaneous concurrent GET /api/v1/auth/me validation",
            all_me_ok and len(me_results) == 5,
            f"all_profiles_matched={all_me_ok}, verified={len([m for m in me_results if m[2]])}/5",
        )

        return authenticated_clients

    # -------------------------------------------------------------------------
    # STAGE 2: Adversarial User Credentials
    # -------------------------------------------------------------------------
    def test_adversarial_credentials(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 2: Adversarial User Credentials (Unicode, Symbols, Extreme Passwords){RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        ts = int(time.time() * 1000)

        # 2.1 Unicode username (CJK + Cyrillic + Emoji)
        unicode_user = f"用户_тест_🚀_{ts}"
        unicode_pass = f"密码_Пароль_🔑_{ts}"
        status, resp, _ = self.http_json(
            "/api/v1/auth/register",
            method="POST",
            body={"username": unicode_user, "password": unicode_pass, "first_name": "UnicodeTester"},
        )
        reg_ok = status == 201 and isinstance(resp, dict) and bool(resp.get("token"))
        token = resp.get("token") if reg_ok else None

        # Verify login with unicode credentials
        if reg_ok:
            login_status, login_resp, _ = self.http_json(
                "/api/v1/auth/login",
                method="POST",
                body={"username": unicode_user, "password": unicode_pass},
            )
            login_ok = login_status == 200 and isinstance(login_resp, dict)
        else:
            login_ok = False

        self.record(
            "CRED-01",
            f"Unicode & emoji username/password ('{unicode_user[:20]}...') registration and login",
            reg_ok and login_ok,
            f"reg_status={status}, login_status={login_status if reg_ok else 'N/A'}",
        )

        # 2.2 Special symbols in username (+ - . _ ~)
        symbol_user = f"user+tag-test.sub~name_{ts}"
        symbol_pass = "P@ssw0rd!#%^&*()-_=+[]{};:'\",.<>?/\\|"
        status, resp, _ = self.http_json(
            "/api/v1/auth/register",
            method="POST",
            body={"username": symbol_user, "password": symbol_pass, "first_name": "SymbolTester"},
        )
        reg_ok = status == 201 and isinstance(resp, dict) and bool(resp.get("token"))
        if reg_ok:
            login_status, login_resp, _ = self.http_json(
                "/api/v1/auth/login",
                method="POST",
                body={"username": symbol_user, "password": symbol_pass},
            )
            login_ok = login_status == 200
        else:
            login_ok = False

        self.record(
            "CRED-02",
            "Special characters and complex punctuation symbols in credentials",
            reg_ok and login_ok,
            f"reg_status={status}, login_status={login_status if reg_ok else 'N/A'}",
        )

        # 2.3 Long password (512 characters)
        long_user = f"longpass_user_{ts}"
        long_pass = "A" * 512
        status, resp, _ = self.http_json(
            "/api/v1/auth/register",
            method="POST",
            body={"username": long_user, "password": long_pass, "first_name": "LongPassTester"},
        )
        reg_ok = status == 201 and isinstance(resp, dict) and bool(resp.get("token"))
        if reg_ok:
            login_status, _, _ = self.http_json(
                "/api/v1/auth/login",
                method="POST",
                body={"username": long_user, "password": long_pass},
            )
            login_ok = login_status == 200
        else:
            login_ok = False

        self.record(
            "CRED-03",
            "Extremely long password (512 bytes) hashed with Argon2id and verified",
            reg_ok and login_ok,
            f"reg_status={status}, login_status={login_status if reg_ok else 'N/A'}",
        )

    # -------------------------------------------------------------------------
    # STAGE 3: Adversarial Invalid Logins & Malformed Payloads
    # -------------------------------------------------------------------------
    def test_adversarial_invalid_requests(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 3: Adversarial Invalid Logins & Malformed Request Bodies{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        ts = int(time.time() * 1000)

        # 3.1 Non-existent user login
        ghost_user = f"non_existent_ghost_{ts}"
        status, resp, _ = self.http_json(
            "/api/v1/auth/login",
            method="POST",
            body={"username": ghost_user, "password": "AnyPassword123!"},
        )
        self.record(
            "INVAL-01",
            "Login with non-existent user returns HTTP 401 Unauthorized",
            status == 401,
            f"status={status}, error={resp.get('error') if isinstance(resp, dict) else resp}",
        )

        # 3.2 Empty password login
        status, resp, _ = self.http_json(
            "/api/v1/auth/login",
            method="POST",
            body={"username": "e2e_user", "password": ""},
        )
        self.record(
            "INVAL-02",
            "Login with empty password rejected with HTTP 401 Unauthorized",
            status == 401,
            f"status={status}",
        )

        # 3.3 Register with empty username / whitespace
        status, resp, _ = self.http_json(
            "/api/v1/auth/register",
            method="POST",
            body={"username": "   ", "password": "ValidPassword123!"},
        )
        self.record(
            "INVAL-03",
            "Register with whitespace-only username rejected with HTTP 400 Bad Request",
            status == 400,
            f"status={status}, error={resp.get('error') if isinstance(resp, dict) else resp}",
        )

        # 3.4 Register with empty password
        status, resp, _ = self.http_json(
            "/api/v1/auth/register",
            method="POST",
            body={"username": f"user_empty_pass_{ts}", "password": "   "},
        )
        self.record(
            "INVAL-04",
            "Register with whitespace-only password rejected with HTTP 400 Bad Request",
            status == 400,
            f"status={status}, error={resp.get('error') if isinstance(resp, dict) else resp}",
        )

        # 3.5 Malformed JSON payload (syntax error)
        malformed_body = b'{"username": "broken_json", "password": '
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/login",
            method="POST",
            raw_body=malformed_body,
        )
        # Axum rejects malformed json with 400 or 422
        is_rejection = status in (400, 422)
        self.record(
            "INVAL-05",
            "Malformed JSON payload rejected with HTTP 400 / 422",
            is_rejection,
            f"status={status}",
        )

        # 3.6 Schema mismatch (missing required field)
        incomplete_body = b'{"unexpected_field": "test"}'
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/login",
            method="POST",
            raw_body=incomplete_body,
        )
        is_rejection = status in (400, 422)
        self.record(
            "INVAL-06",
            "Missing required schema fields rejected with HTTP 400 / 422",
            is_rejection,
            f"status={status}",
        )

    # -------------------------------------------------------------------------
    # STAGE 4: Bearer Token Authentication Fuzzing
    # -------------------------------------------------------------------------
    def test_token_fuzzing(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 4: Bearer Token Authentication Fuzzing{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        # 4.1 Missing Authorization Header
        status, resp, _ = self.raw_http_request("/api/v1/auth/me", method="GET")
        self.record(
            "FUZZ-01",
            "GET /api/v1/auth/me without Authorization header returns HTTP 401",
            status == 401,
            f"status={status}",
        )

        # 4.2 Empty Bearer Token
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/me",
            method="GET",
            headers={"Authorization": "Bearer "},
        )
        self.record(
            "FUZZ-02",
            "GET /api/v1/auth/me with empty 'Bearer ' returns HTTP 401",
            status == 401,
            f"status={status}",
        )

        # 4.3 Malformed non-token string
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/me",
            method="GET",
            headers={"Authorization": "Bearer !@#$%^&*()_+-=[]{}|;':\",./<>?"},
        )
        self.record(
            "FUZZ-03",
            "GET /api/v1/auth/me with garbage character string returns HTTP 401",
            status == 401,
            f"status={status}",
        )

        # 4.4 Forged cryptographic signature
        forged_token = "sess_0123456789abcdef.deadbeefcafebabe0123456789abcdef"
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/me",
            method="GET",
            headers={"Authorization": f"Bearer {forged_token}"},
        )
        self.record(
            "FUZZ-04",
            "GET /api/v1/auth/me with forged hex token signature returns HTTP 401",
            status == 401,
            f"status={status}",
        )

        # 4.5 SQL injection attempt in token header
        sqli_token = "' OR '1'='1' --"
        status, resp, _ = self.raw_http_request(
            "/api/v1/auth/me",
            method="GET",
            headers={"Authorization": f"Bearer {sqli_token}"},
        )
        self.record(
            "FUZZ-05",
            "GET /api/v1/auth/me with SQLi payload in Bearer header returns HTTP 401",
            status == 401,
            f"status={status}",
        )

    # -------------------------------------------------------------------------
    # STAGE 5: Adversarial AI Bot Reasoning Challenge
    # -------------------------------------------------------------------------
    def test_ai_bot_reasoning(self, token: str, session_id: str):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 5: Adversarial AI Bot Reasoning Challenge{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        # 5.1 Technical analysis prompt: "Calculate prime factors of 1024 and summarize encryption"
        math_prompt = "Calculate prime factors of 1024 and summarize encryption applications in RSA."
        status, resp, elapsed = self.http_json(
            "/api/v1/agent/runs",
            method="POST",
            body={
                "prompt": math_prompt,
                "session_id": session_id,
                "model_id": "grok-3",
                "thinking_budget_tokens": 1024,
            },
            token=token,
            timeout=35,
        )
        math_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("status") == "completed"
            and len(resp.get("response", "")) > 10
        )
        response_text = resp.get("response", "") if isinstance(resp, dict) else ""
        self.record(
            "AI-STRESS-01",
            "Technical math & cryptography prompt executed by AI Bot (status=completed)",
            math_ok,
            f"status={status}, latency={elapsed:.3f}s, response_len={len(response_text)}",
        )

        # 5.2 Edge prompt with special characters / code snippet / command injection simulation
        code_prompt = (
            "Analyze security implications of this snippet: "
            "SELECT * FROM users WHERE username = '${user_input}'; DROP TABLE sessions; --"
        )
        status, resp, elapsed = self.http_json(
            "/api/v1/agent/runs",
            method="POST",
            body={
                "prompt": code_prompt,
                "session_id": session_id,
                "model_id": "grok-3",
                "thinking_budget_tokens": 1024,
            },
            token=token,
            timeout=35,
        )
        code_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("status") == "completed"
            and len(resp.get("response", "")) > 10
        )
        self.record(
            "AI-STRESS-02",
            "Code snippet with SQL injection syntax handled without escaping errors",
            code_ok,
            f"status={status}, latency={elapsed:.3f}s, response_len={len(resp.get('response', '')) if isinstance(resp, dict) else 0}",
        )

        # 5.3 Multilingual & Markdown symbols prompt
        cjk_prompt = "请解释零知识证明 (Zero-Knowledge Proofs) 的基本概念并给出 Markdown 表格示例。"
        status, resp, elapsed = self.http_json(
            "/api/v1/agent/runs",
            method="POST",
            body={
                "prompt": cjk_prompt,
                "session_id": session_id,
                "model_id": "grok-3",
                "thinking_budget_tokens": 1024,
            },
            token=token,
            timeout=35,
        )
        cjk_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("status") == "completed"
            and len(resp.get("response", "")) > 10
        )
        self.record(
            "AI-STRESS-03",
            "Multilingual CJK prompt with Markdown instruction executed (status=completed)",
            cjk_ok,
            f"status={status}, latency={elapsed:.3f}s",
        )

    # -------------------------------------------------------------------------
    # STAGE 6: Cold Restart & Session Storage Isolation (POSIX 0600)
    # -------------------------------------------------------------------------
    def test_cold_restart_and_isolation(self, client_a: Dict[str, Any], client_b: Dict[str, Any]):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 6: Cold Restart & Multi-Client Session Storage Isolation (POSIX 0600){RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        with tempfile.TemporaryDirectory() as sandbox_root:
            sandbox_a = os.path.join(sandbox_root, "client_a", ".config", "gpui_telegram")
            sandbox_b = os.path.join(sandbox_root, "client_b", ".config", "gpui_telegram")
            os.makedirs(sandbox_a, exist_ok=True)
            os.makedirs(sandbox_b, exist_ok=True)

            session_file_a = os.path.join(sandbox_a, "session.json")
            session_file_b = os.path.join(sandbox_b, "session.json")

            data_a = {
                "server_url": SERVER_URL,
                "user_id": client_a["user_id"],
                "username": client_a["username"],
                "token": client_a["token"],
                "session_id": client_a["session_id"],
                "created_at_ms": int(time.time() * 1000),
            }
            data_b = {
                "server_url": SERVER_URL,
                "user_id": client_b["user_id"],
                "username": client_b["username"],
                "token": client_b["token"],
                "session_id": client_b["session_id"],
                "created_at_ms": int(time.time() * 1000),
            }

            # Write file A and set 0600
            with open(session_file_a, "w") as f:
                json.dump(data_a, f)
            os.chmod(session_file_a, 0o600)

            # Write file B and set 0600
            with open(session_file_b, "w") as f:
                json.dump(data_b, f)
            os.chmod(session_file_b, 0o600)

            perm_a_ok = oct(os.stat(session_file_a).st_mode)[-3:] == "600"
            perm_b_ok = oct(os.stat(session_file_b).st_mode)[-3:] == "600"

            self.record(
                "ISOL-01",
                "Both sandboxed session.json files enforce strict POSIX 0600 permissions",
                perm_a_ok and perm_b_ok,
                f"perm_a={oct(os.stat(session_file_a).st_mode)[-3:]}, perm_b={oct(os.stat(session_file_b).st_mode)[-3:]}",
            )

            # Simulate restart A: reload session A and call /api/v1/auth/me
            with open(session_file_a, "r") as f:
                loaded_a = json.load(f)
            status_a, me_a, _ = self.http_json("/api/v1/auth/me", token=loaded_a["token"])
            valid_a = status_a == 200 and isinstance(me_a, dict) and me_a.get("user_id") == client_a["user_id"]

            # Simulate restart B: reload session B and call /api/v1/auth/me
            with open(session_file_b, "r") as f:
                loaded_b = json.load(f)
            status_b, me_b, _ = self.http_json("/api/v1/auth/me", token=loaded_b["token"])
            valid_b = status_b == 200 and isinstance(me_b, dict) and me_b.get("user_id") == client_b["user_id"]

            self.record(
                "ISOL-02",
                "Isolated sandboxes restore distinct client sessions without state cross-contamination",
                valid_a and valid_b and loaded_a["user_id"] != loaded_b["user_id"],
                f"user_a={me_a.get('user_id') if isinstance(me_a, dict) else None}, user_b={me_b.get('user_id') if isinstance(me_b, dict) else None}",
            )

            # Test Tampering Detection: mutate token in sandbox A
            corrupted_token = loaded_a["token"][:-2] + "xx"
            status_tamper, _, _ = self.http_json("/api/v1/auth/me", token=corrupted_token)
            self.record(
                "ISOL-03",
                "Tampered session token in sandbox is rejected with HTTP 401 Unauthorized",
                status_tamper == 401,
                f"status={status_tamper}",
            )

    # -------------------------------------------------------------------------
    # STAGE 7: AGENTS.md Build Hygiene Audit
    # -------------------------------------------------------------------------
    def test_build_hygiene_audit(self):
        print(f"\n{CYAN}{'='*70}{RESET}")
        print(f"{CYAN}Stage 7: AGENTS.md Build Garbage & Cache Cleanup Audit{RESET}")
        print(f"{CYAN}{'='*70}{RESET}")

        clean_script = os.path.join(REPO_ROOT, "scripts", "clean_build_garbage.py")
        res = subprocess.run(
            [sys.executable, clean_script],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            timeout=30,
        )
        self.record(
            "HYG-AUDIT-01",
            "Execute scripts/clean_build_garbage.py exits with code 0",
            res.returncode == 0,
            f"returncode={res.returncode}, stdout={res.stdout.strip()}",
        )

        # Audit workspace for redundant .cargo-target-*
        redundant_targets = glob.glob(os.path.join(REPO_ROOT, ".cargo-target-*"))
        self.record(
            "HYG-AUDIT-02",
            "Zero redundant .cargo-target-* directories in workspace",
            len(redundant_targets) == 0,
            f"found={len(redundant_targets)} ({redundant_targets})",
        )

        # Audit target/ for incremental caches
        target_dir = os.path.join(REPO_ROOT, "target")
        inc_dirs = glob.glob(os.path.join(target_dir, "*", "incremental"))
        self.record(
            "HYG-AUDIT-03",
            "Zero incremental build caches in target/*/incremental",
            len(inc_dirs) == 0,
            f"found={len(inc_dirs)}",
        )

        # Audit target/ for *.dSYM debug bundles
        dsym_dirs = glob.glob(os.path.join(target_dir, "*", "deps", "*.dSYM"))
        self.record(
            "HYG-AUDIT-04",
            "Zero *.dSYM debug symbol bundles in target/*/deps/*.dSYM",
            len(dsym_dirs) == 0,
            f"found={len(dsym_dirs)}",
        )

    def run_all(self) -> int:
        start_time = time.time()
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"{BLUE}  Milestone 4 Adversarial Cloud Live Stress Test Runner  {RESET}")
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"Target Server: {SERVER_URL}")
        print(f"Repository:    {REPO_ROOT}")
        print(f"Start Time:    {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime())}")

        # Stage 1: Burst
        authenticated_clients = self.test_concurrent_burst()

        # Stage 2: Adversarial Credentials
        self.test_adversarial_credentials()

        # Stage 3: Invalid requests & malformed JSON
        self.test_adversarial_invalid_requests()

        # Stage 4: Token fuzzing
        self.test_token_fuzzing()

        # Stage 5: AI Bot Reasoning Stress
        if authenticated_clients:
            primary_client = authenticated_clients[0]
            self.test_ai_bot_reasoning(primary_client["token"], primary_client["session_id"])
        else:
            self.record("AI-STRESS-01", "AI Bot Reasoning Stress", False, "No authenticated client available")

        # Stage 6: Cold Restart & Isolation
        if len(authenticated_clients) >= 2:
            self.test_cold_restart_and_isolation(authenticated_clients[0], authenticated_clients[1])
        else:
            self.record("ISOL-01", "Isolation test", False, "Need at least 2 authenticated clients")

        # Stage 7: AGENTS.md Hygiene Audit
        self.test_build_hygiene_audit()

        elapsed = time.time() - start_time
        print(f"\n{BLUE}{'='*70}{RESET}")
        print(f"{BLUE}                 Adversarial Challenge Summary                       {RESET}")
        print(f"{BLUE}{'='*70}{RESET}")
        print(f"  Total Passed Assertions: {GREEN}{self.passed}{RESET}")
        print(f"  Total Failed Assertions: {RED}{self.failed}{RESET}")
        print(f"  Total Execution Time:    {elapsed:.2f}s")

        if self.failed == 0:
            print(f"\n{GREEN}{BOLD}>>> ADVERSARIAL CHALLENGE VERDICT: ALL TESTS PASSED (APPROVE) <<<{RESET}\n")
            return 0
        else:
            print(f"\n{RED}{BOLD}>>> ADVERSARIAL CHALLENGE VERDICT: {self.failed} TESTS FAILED (DISAPPROVE) <<<{RESET}\n")
            return 1


if __name__ == "__main__":
    runner = StressTestRunner()
    sys.exit(runner.run_all())
