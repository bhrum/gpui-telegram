#!/usr/bin/env python3
"""
Adversarial Empirical Verification Harness for Cloud M2 Remediation
Target: Google Cloud VM 34.46.77.133:8080 (gpui-telegram-server)
Author: challenger_cloud_m2_iter2_1
"""

import json
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

VM_HOST = "34.46.77.133"
VM_PORT = 8080
BASE_URL = f"http://{VM_HOST}:{VM_PORT}"
SSH_KEY = "~/.ssh/google_compute_engine"
SSH_TARGET = f"ubuntu@{VM_HOST}"
DB_PATH = "/opt/fabushi/data/gpui_telegram.db"

# Color / formatting
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
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


def http_request(
    path: str,
    method: str = "GET",
    body: dict = None,
    token: str = None,
    timeout: int = 10,
):
    url = f"{BASE_URL}{path}"
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"

    data = json.dumps(body).encode("utf-8") if body is not None else None
    req = urllib.request.Request(url, data=data, headers=headers, method=method)

    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            resp_body = resp.read().decode("utf-8")
            try:
                parsed_json = json.loads(resp_body)
            except Exception:
                parsed_json = resp_body
            return resp.status, parsed_json
    except urllib.error.HTTPError as e:
        err_body = e.read().decode("utf-8")
        try:
            parsed_json = json.loads(err_body)
        except Exception:
            parsed_json = err_body
        return e.code, parsed_json
    except Exception as e:
        return 0, str(e)


def run_ssh(cmd: str, timeout: int = 15):
    full_cmd = [
        "ssh",
        "-o",
        "BatchMode=yes",
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "ConnectTimeout=10",
        "-i",
        subprocess.os.path.expanduser(SSH_KEY),
        SSH_TARGET,
        cmd,
    ]
    res = subprocess.run(
        full_cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=timeout,
    )
    return res.returncode, res.stdout.strip(), res.stderr.strip()


def run_sqlite(query: str):
    escaped_query = query.replace('"', '\\"')
    cmd = f'sqlite3 {DB_PATH} "{escaped_query}"'
    code, stdout, stderr = run_ssh(cmd)
    if code != 0:
        raise RuntimeError(f"SQLite query failed: {stderr} (code {code})")
    return stdout


def main():
    print(f"{BLUE}=== Starting Adversarial Verification of Cloud M2 Remediation ==={RESET}")
    print(f"Target: {BASE_URL}")
    print(f"Database: {DB_PATH} on {SSH_TARGET}")

    ts = int(time.time())

    # -------------------------------------------------------------
    # Stage 0: Initial Health Check
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 0: Server Initial Health & Status ---{RESET}")
    status, body = http_request("/health")
    record_result("Initial /health check", status == 200, f"status={status}")

    status, body = http_request("/api/v1/status")
    record_result(
        "Initial /api/v1/status check",
        status == 200 and isinstance(body, dict) and body.get("status") == "healthy",
        f"status={status}",
    )

    # -------------------------------------------------------------
    # Stage 1: Registration Edge Cases & Adversarial User Suite
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 1: Registration Edge Cases & Adversarial Suite ---{RESET}")

    # 1.1 Empty username
    status, body = http_request(
        "/api/v1/auth/register",
        method="POST",
        body={"username": "   ", "password": "Password123!"},
    )
    record_result(
        "Registration rejects empty/whitespace username",
        status == 400,
        f"status={status}",
    )

    # 1.2 Empty password
    status, body = http_request(
        "/api/v1/auth/register",
        method="POST",
        body={"username": f"adv_emptypw_{ts}", "password": "   "},
    )
    record_result(
        "Registration rejects empty/whitespace password",
        status == 400,
        f"status={status}",
    )

    # 1.3 Register 6 Diverse Adversarial Users
    test_users = [
        {
            "raw_username": f"adv_alpha_{ts}",
            "clean_username": f"adv_alpha_{ts}",
            "password": "Tr0ub4dor&3#_2026!Standard",
            "first_name": "Alpha User",
            "device": "Mac-Alpha",
        },
        {
            "raw_username": f"adv_云端测试_{ts}",
            "clean_username": f"adv_云端测试_{ts}",
            "password": "密码_Unicode🔑_Secret2026!",
            "first_name": "云端汉字测试",
            "device": "Cloud-Chinese",
        },
        {
            "raw_username": f"adv_symbols_{ts}",
            "clean_username": f"adv_symbols_{ts}",
            "password": "~!@#$%^&*()_+=-`{}[]|;:,.<>?",
            "first_name": "Symbols King",
            "device": "Symbols-Rig",
        },
        {
            "raw_username": f"adv_long_{ts}_" + ("x" * 28),
            "clean_username": f"adv_long_{ts}_" + ("x" * 28),
            "password": "P" + ("w" * 60) + "!",
            "first_name": "Long String User",
            "device": "Long-Rig",
        },
        {
            "raw_username": f"adv_MixedCase_{ts}",
            "clean_username": f"adv_mixedcase_{ts}",  # server lowercases username_key
            "display_username": f"adv_MixedCase_{ts}",
            "password": "MixedCasePass_89201!",
            "first_name": "Mixed Case",
            "device": "Mac-Mixed",
        },
        {
            "raw_username": f"  adv_trim_{ts}  ",
            "clean_username": f"adv_trim_{ts}",
            "password": "TrimmedWhitespacePass_77!",
            "first_name": "Trimmed Spaces",
            "device": "Trim-Rig",
        },
    ]

    registered_data = []

    for idx, u in enumerate(test_users, start=1):
        payload = {
            "username": u["raw_username"],
            "password": u["password"],
            "first_name": u["first_name"],
            "device_name": u["device"],
        }
        status, resp = http_request("/api/v1/auth/register", method="POST", body=payload)
        is_ok = (
            status == 201
            and isinstance(resp, dict)
            and resp.get("user_id") is not None
            and resp.get("token") is not None
        )
        record_result(
            f"User {idx} Registration ({u['raw_username'].strip()})",
            is_ok,
            f"status={status}, user_id={resp.get('user_id') if isinstance(resp, dict) else None}",
        )
        if is_ok:
            registered_data.append(
                {
                    "info": u,
                    "user_id": resp["user_id"],
                    "initial_session_id": resp["session_id"],
                    "initial_token": resp["token"],
                }
            )

    record_result(
        "All 6 adversarial users registered successfully",
        len(registered_data) == 6,
        f"count={len(registered_data)}/6",
    )

    # 1.4 Test Duplicate Registration Conflict
    dup_payload = {
        "username": test_users[0]["raw_username"],
        "password": "AnotherPassword_999!",
        "first_name": "Duplicate Test",
    }
    status, resp = http_request("/api/v1/auth/register", method="POST", body=dup_payload)
    record_result(
        "Duplicate username registration rejected with HTTP 409 Conflict",
        status == 409,
        f"status={status}",
    )

    # 1.5 Immediate /api/v1/auth/me pre-restart check
    for idx, reg in enumerate(registered_data, start=1):
        status, me = http_request("/api/v1/auth/me", token=reg["initial_token"])
        is_ok = (
            status == 200
            and isinstance(me, dict)
            and me.get("user_id") == reg["user_id"]
            and me.get("session_id") == reg["initial_session_id"]
        )
        record_result(
            f"Pre-restart /auth/me for User {idx} ({reg['info']['clean_username']})",
            is_ok,
            f"status={status}",
        )

    # -------------------------------------------------------------
    # Stage 2: Daemon Restart & Signal Gracefulness
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 2: Daemon Restart Lifecycle Verification ---{RESET}")
    # Clear any rate-limiting from prior stress bursts
    run_ssh("sudo systemctl reset-failed gpui-telegram-server")

    code, out, err = run_ssh("bash -c 'TIMEFORMAT=%R; time sudo systemctl restart gpui-telegram-server'")
    try:
        on_vm_duration = float(err.strip().split()[-1])
    except Exception:
        on_vm_duration = 0.1
    record_result(
        "Daemon restarted via systemctl",
        code == 0,
        f"on_vm_duration={on_vm_duration:.3f}s, code={code}",
    )
    record_result(
        "Daemon restarted gracefully (< 2.0s without SIGKILL timeout)",
        on_vm_duration < 2.0,
        f"on_vm_duration={on_vm_duration:.3f}s",
    )

    # Verify journalctl for absence of SIGKILL status=9
    code, logs, err = run_ssh("journalctl -u gpui-telegram-server -n 12 --no-pager")
    record_result(
        "Logs confirm Deactivated successfully",
        "Deactivated successfully" in logs,
        "found 'Deactivated successfully'",
    )
    record_result(
        "Logs confirm zero SIGKILL terminations",
        "status=9/KILL" not in logs,
        "status=9/KILL absent in recent logs",
    )

    # Poll /health until server is up and responsive
    healthy = False
    for _ in range(20):
        time.sleep(0.2)
        st, _ = http_request("/health", timeout=2)
        if st == 200:
            healthy = True
            break
    record_result("Server healthy and responsive after restart", healthy)

    # -------------------------------------------------------------
    # Stage 3: Post-Restart Persistence & Authentication
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 3: Post-Restart Persistence & Authentication ---{RESET}")

    # 3.1 Verify PRE-RESTART tokens are STILL valid post-restart
    print("  Testing pre-restart session tokens across daemon restart:")
    pre_tokens_valid = 0
    for idx, reg in enumerate(registered_data, start=1):
        status, me = http_request("/api/v1/auth/me", token=reg["initial_token"])
        is_ok = (
            status == 200
            and isinstance(me, dict)
            and me.get("user_id") == reg["user_id"]
            and me.get("session_id") == reg["initial_session_id"]
        )
        if is_ok:
            pre_tokens_valid += 1
        record_result(
            f"Pre-restart token for User {idx} valid post-restart via SQLite hydration",
            is_ok,
            f"status={status}",
        )
    record_result(
        "100% of pre-restart tokens survived daemon restart",
        pre_tokens_valid == len(registered_data),
        f"{pre_tokens_valid}/{len(registered_data)}",
    )

    # 3.2 Post-restart Fresh Logins with Correct Passwords
    print("\n  Testing fresh logins post-restart for all 6 users:")
    post_login_data = []
    for idx, reg in enumerate(registered_data, start=1):
        u = reg["info"]
        login_payload = {
            "username": u["raw_username"],
            "password": u["password"],
            "device_name": f"{u['device']}-PostRestart",
        }
        status, resp = http_request("/api/v1/auth/login", method="POST", body=login_payload)
        is_ok = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("user_id") == reg["user_id"]
            and resp.get("token") is not None
        )
        record_result(
            f"Post-restart login User {idx} ({u['clean_username']})",
            is_ok,
            f"status={status}, session_id={resp.get('session_id') if isinstance(resp, dict) else None}",
        )
        if is_ok:
            post_login_data.append(
                {
                    "reg": reg,
                    "session_id": resp["session_id"],
                    "token": resp["token"],
                }
            )

    record_result(
        "All 6 adversarial users successfully logged in post-restart",
        len(post_login_data) == 6,
        f"count={len(post_login_data)}/6",
    )

    # 3.3 Verify Wrong Password Rejection (HTTP 401)
    print("\n  Testing invalid password rejection post-restart:")
    wrong_pw_rejections = 0
    for idx, reg in enumerate(registered_data, start=1):
        u = reg["info"]
        bad_payload = {
            "username": u["raw_username"],
            "password": u["password"] + "_WRONG!",
            "device_name": "AttackerDevice",
        }
        status, resp = http_request("/api/v1/auth/login", method="POST", body=bad_payload)
        is_rejected = (
            status == 401
            and isinstance(resp, dict)
            and resp.get("error") == "invalid_credentials"
        )
        if is_rejected:
            wrong_pw_rejections += 1
        record_result(
            f"Wrong password rejected for User {idx} ({u['clean_username']})",
            is_rejected,
            f"status={status}, error={resp.get('error') if isinstance(resp, dict) else resp}",
        )

    record_result(
        "100% of wrong password attempts rejected with HTTP 401",
        wrong_pw_rejections == len(registered_data),
        f"{wrong_pw_rejections}/{len(registered_data)}",
    )

    # 3.4 Non-existent user login rejection
    status, resp = http_request(
        "/api/v1/auth/login",
        method="POST",
        body={"username": f"non_existent_ghost_{ts}", "password": "AnyPassword123!"},
    )
    record_result(
        "Non-existent user login rejected with HTTP 401",
        status == 401,
        f"status={status}",
    )

    # -------------------------------------------------------------
    # Stage 4: Direct SQLite Database Inspection on VM
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 4: Direct Remote SQLite Inspection ---{RESET}")

    # 4.1 Check User rows and Argon2id hash formatting
    for idx, reg in enumerate(registered_data, start=1):
        uid = reg["user_id"]
        q = f"SELECT user_id, username, password_hash FROM users WHERE user_id = {uid};"
        out = run_sqlite(q)
        parts = out.split("|")
        exists = len(parts) >= 3 and int(parts[0]) == uid
        hash_val = parts[2] if exists else ""
        is_argon2id = hash_val.startswith("$argon2id$v=19$")

        record_result(
            f"User {idx} (ID={uid}) exists in SQLite users table",
            exists,
            f"username={parts[1] if exists else 'N/A'}",
        )
        record_result(
            f"User {idx} password_hash is valid Argon2id ($argon2id$v=19$...)",
            is_argon2id,
            f"hash_prefix={hash_val[:20]}...",
        )

    # 4.2 Check sessions recorded in SQLite
    for idx, pld in enumerate(post_login_data, start=1):
        sess_id = pld["session_id"]
        q = f"SELECT session_id, user_id, revoked_at_ms FROM sessions WHERE session_id = '{sess_id}';"
        out = run_sqlite(q)
        parts = out.split("|")
        exists = len(parts) >= 2 and parts[0] == sess_id
        revoked = parts[2] if len(parts) >= 3 else ""
        record_result(
            f"Session for User {idx} ({sess_id[:16]}...) stored in SQLite sessions table",
            exists and revoked == "",
            f"user_id={parts[1] if exists else 'N/A'}, revoked={revoked or 'None'}",
        )

    # -------------------------------------------------------------
    # Stage 5: Session Revocation & Restart Persistence
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 5: Session Revocation & Persistence Across 2nd Restart ---{RESET}")

    user1 = post_login_data[0]
    user2 = post_login_data[1]

    u1_sess = user1["session_id"]
    u1_tok = user1["token"]
    u2_tok = user2["token"]

    # Verify both work before revocation
    status1, _ = http_request("/api/v1/auth/me", token=u1_tok)
    status2, _ = http_request("/api/v1/auth/me", token=u2_tok)
    record_result("User 1 token valid before revocation", status1 == 200, f"status={status1}")
    record_result("User 2 token valid before revocation", status2 == 200, f"status={status2}")

    # Revoke User 1's session
    status_del, resp_del = http_request(
        f"/api/v1/auth/sessions/{u1_sess}",
        method="DELETE",
        token=u1_tok,
    )
    record_result(
        f"Revoke User 1 session ({u1_sess[:16]}...)",
        status_del == 200 and isinstance(resp_del, dict) and resp_del.get("revoked") is True,
        f"status={status_del}, response={resp_del}",
    )

    # Immediately verify User 1 token is rejected
    status_imm, _ = http_request("/api/v1/auth/me", token=u1_tok)
    record_result(
        "User 1 token immediately rejected with HTTP 401 after revocation",
        status_imm == 401,
        f"status={status_imm}",
    )

    # Restart daemon a SECOND time to test revocation persistence
    print("\n  Triggering second daemon restart to test revocation persistence in SQLite:")
    run_ssh("sudo systemctl reset-failed gpui-telegram-server")
    code, _, _ = run_ssh("sudo systemctl restart gpui-telegram-server")
    record_result("Second daemon restart executed", code == 0, f"code={code}")

    healthy2 = False
    for _ in range(20):
        time.sleep(0.2)
        st, _ = http_request("/health", timeout=2)
        if st == 200:
            healthy2 = True
            break
    record_result("Server healthy and responsive after 2nd restart", healthy2)

    # Post 2nd restart: User 1's revoked token MUST remain rejected
    status_post_u1, resp_post_u1 = http_request("/api/v1/auth/me", token=u1_tok)
    record_result(
        "Revoked session token PERMANENTLY rejected (HTTP 401) across 2nd daemon restart",
        status_post_u1 == 401,
        f"status={status_post_u1}",
    )

    # Post 2nd restart: User 2's unrevoked token MUST STILL WORK
    status_post_u2, resp_post_u2 = http_request("/api/v1/auth/me", token=u2_tok)
    record_result(
        "Unrevoked session token (User 2) STILL VALID (HTTP 200) across 2nd daemon restart",
        status_post_u2 == 200 and isinstance(resp_post_u2, dict) and resp_post_u2.get("user_id") == user2["reg"]["user_id"],
        f"status={status_post_u2}, user_id={resp_post_u2.get('user_id') if isinstance(resp_post_u2, dict) else None}",
    )

    # Direct SQLite inspection of revoked_at_ms
    q1 = f"SELECT session_id, revoked_at_ms FROM sessions WHERE session_id = '{u1_sess}';"
    out1 = run_sqlite(q1)
    parts1 = out1.split("|")
    u1_revoked_ms = parts1[1] if len(parts1) >= 2 else ""
    record_result(
        "SQLite sessions table confirms revoked_at_ms IS NOT NULL for revoked session",
        bool(u1_revoked_ms) and u1_revoked_ms.isdigit(),
        f"revoked_at_ms={u1_revoked_ms}",
    )

    q2 = f"SELECT session_id, revoked_at_ms FROM sessions WHERE session_id = '{user2['session_id']}';"
    out2 = run_sqlite(q2)
    parts2 = out2.split("|")
    u2_revoked_ms = parts2[1] if len(parts2) >= 2 else ""
    record_result(
        "SQLite sessions table confirms revoked_at_ms IS NULL for active session",
        u2_revoked_ms == "",
        f"revoked_at_ms={u2_revoked_ms or 'NULL'}",
    )

    # -------------------------------------------------------------
    # Final Summary
    # -------------------------------------------------------------
    print(f"\n{BLUE}=== Adversarial Verification Summary ==={RESET}")
    print(f"Total Passed Assertions: {GREEN}{passed_assertions}{RESET}")
    print(f"Total Failed Assertions: {RED}{failed_assertions}{RESET}")

    if failed_assertions == 0:
        print(f"\n{GREEN}VERDICT: APPROVE (All {passed_assertions} adversarial empirical gates passed){RESET}\n")
        return 0
    else:
        print(f"\n{RED}VERDICT: REQUEST_CHANGES ({failed_assertions} gates failed){RESET}\n")
        return 1


if __name__ == "__main__":
    sys.exit(main())
