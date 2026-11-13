#!/usr/bin/env python3
"""
Live Cloud VM Adversarial Verification Harness for Milestone 3
Target: Google Cloud VM 34.170.16.35:8080 (gpui-telegram-server & AI Agent)
Author: challenger_cloud_m3_2
"""

import concurrent.futures
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

VM_HOST = "34.170.16.35"
VM_PORT = 8080
BASE_URL = f"http://{VM_HOST}:{VM_PORT}"

GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
RESET = "\033[0m"

passed_assertions = 0
failed_assertions = 0
detailed_results = []


def record_result(desc: str, passed: bool, detail: str = ""):
    global passed_assertions, failed_assertions, detailed_results
    if passed:
        passed_assertions += 1
        msg = f"  {GREEN}[PASS]{RESET} {desc} {f'({detail})' if detail else ''}"
        print(msg)
        detailed_results.append((desc, "PASS", detail))
    else:
        failed_assertions += 1
        msg = f"  {RED}[FAIL]{RESET} {desc} - {detail}"
        print(msg)
        detailed_results.append((desc, "FAIL", detail))


def http_request(
    path: str,
    method: str = "GET",
    body: dict = None,
    token: str = None,
    timeout: int = 15,
):
    url = f"{BASE_URL}{path}"
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"

    data = json.dumps(body).encode("utf-8") if body is not None else None
    req = urllib.request.Request(url, data=data, headers=headers, method=method)

    start_time = time.time()
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            elapsed = time.time() - start_time
            resp_body = resp.read().decode("utf-8")
            try:
                parsed_json = json.loads(resp_body)
            except Exception:
                parsed_json = resp_body
            return resp.status, parsed_json, elapsed
    except urllib.error.HTTPError as e:
        elapsed = time.time() - start_time
        err_body = e.read().decode("utf-8")
        try:
            parsed_json = json.loads(err_body)
        except Exception:
            parsed_json = err_body
        return e.code, parsed_json, elapsed
    except Exception as e:
        elapsed = time.time() - start_time
        return 0, str(e), elapsed


def main():
    print(f"{BLUE}======================================================================{RESET}")
    print(f"{BLUE}     M3 Empirical Adversarial Challenge: Live Cloud VM 34.170.16.35    {RESET}")
    print(f"{BLUE}======================================================================{RESET}")
    print(f"Target: {BASE_URL}")

    ts = int(time.time())

    # -------------------------------------------------------------
    # Stage 1: Health & Server Status Checks
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 1: Cloud VM Service Health & Identity ---{RESET}")
    status, body, elapsed = http_request("/health")
    record_result("GET /health responds HTTP 200 OK", status == 200 and body == "OK", f"status={status}, latency={elapsed:.3f}s")

    status, body, elapsed = http_request("/api/v1/status")
    record_result(
        "GET /api/v1/status returns healthy status",
        status == 200 and isinstance(body, dict) and body.get("status") == "healthy" and body.get("server") == "gpui_telegram_server",
        f"status={status}, server={body.get('server') if isinstance(body, dict) else ''}, latency={elapsed:.3f}s"
    )

    # -------------------------------------------------------------
    # Stage 2: Adversarial User Profile Registrations
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 2: Adversarial User Profile Registrations ---{RESET}")

    # 2.1 Adversarial profiles matrix
    adversarial_profiles = [
        {
            "category": "Unicode & Chinese",
            "username": f"云端用户_{ts}_测试",
            "password": "密码SecretKey_2026!汉字",
            "first_name": "张三(云端测试)",
            "device": "MacBook Pro M3 Max (中文终端)",
        },
        {
            "category": "Emojis & Symbols",
            "username": f"adv_bot_🚀🤖_{ts}",
            "password": "Pass🔐🔑_EmojiSparkles✨2026!",
            "first_name": "Bot 🚀 Agent",
            "device": "Emoji-Device 📱",
        },
        {
            "category": "Special Characters & Punctuation",
            "username": f"adv_spec_{ts}_!@#$-_",
            "password": "~`!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?",
            "first_name": "Special-Char-User",
            "device": "Special-Terminal-v1.0",
        },
        {
            "category": "Extreme Length (>64 chars)",
            "username": f"adv_long_user_name_stress_testing_boundary_conditions_{ts}_xyz",
            "password": "LongPasswordStr0ngKey_" + ("A" * 60) + "_2026!",
            "first_name": "LongFirstName_" + ("B" * 40),
            "device": "Super-Long-Device-Identifier-String-64Bit",
        },
    ]

    registered_users = []

    for prof in adversarial_profiles:
        reg_payload = {
            "username": prof["username"],
            "password": prof["password"],
            "first_name": prof["first_name"],
            "device_name": prof["device"],
        }
        status, resp, elapsed = http_request("/api/v1/auth/register", method="POST", body=reg_payload)
        is_ok = (
            status == 201
            and isinstance(resp, dict)
            and resp.get("user_id") is not None
            and resp.get("token") is not None
            and resp.get("session_id") is not None
        )
        record_result(
            f"Register {prof['category']} user ({prof['username'][:25]}...)",
            is_ok,
            f"status={status}, user_id={resp.get('user_id') if isinstance(resp, dict) else None}, latency={elapsed:.3f}s"
        )
        if is_ok:
            registered_users.append({
                "profile": prof,
                "user_id": resp["user_id"],
                "session_id": resp["session_id"],
                "token": resp["token"]
            })

    record_result(
        "All 4 diverse adversarial user profiles registered cleanly",
        len(registered_users) == 4,
        f"registered={len(registered_users)}/4"
    )

    # -------------------------------------------------------------
    # Stage 3: Duplicate Registration Conflict (409 Conflict)
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 3: Duplicate Registration Conflict Verification ---{RESET}")
    for u in registered_users[:2]:
        dup_payload = {
            "username": u["profile"]["username"],
            "password": "AnotherNewPassword_12345!",
            "first_name": "Impostor",
            "device_name": "Hacker-Terminal",
        }
        status, resp, elapsed = http_request("/api/v1/auth/register", method="POST", body=dup_payload)
        record_result(
            f"Duplicate registration for '{u['profile']['username'][:20]}' returns HTTP 409",
            status == 409,
            f"status={status}, message={resp.get('message') if isinstance(resp, dict) else resp}"
        )

    # -------------------------------------------------------------
    # Stage 4: Authentication Attack Scenarios (401 Unauthorized)
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 4: Bad Credentials & Invalid Auth Scenarios ---{RESET}")

    # 4.1 Bad password for existing user
    bad_pw_payload = {
        "username": registered_users[0]["profile"]["username"],
        "password": "TotallyWrongPassword_Wrong_9999!",
        "device_name": "Attacker-Rig",
    }
    status, resp, elapsed = http_request("/api/v1/auth/login", method="POST", body=bad_pw_payload)
    record_result(
        "Login with incorrect password returns HTTP 401 Unauthorized",
        status == 401,
        f"status={status}, resp={resp}"
    )

    # 4.2 Non-existent username
    non_existent_payload = {
        "username": f"non_existent_ghost_user_{ts}_{time.time_ns()}",
        "password": "AnyPassword123!",
        "device_name": "Attacker-Rig",
    }
    status, resp, elapsed = http_request("/api/v1/auth/login", method="POST", body=non_existent_payload)
    record_result(
        "Login with non-existent username returns HTTP 401 Unauthorized",
        status == 401,
        f"status={status}, resp={resp}"
    )

    # 4.3 Invalid Bearer token against /api/v1/auth/me
    status, resp, elapsed = http_request("/api/v1/auth/me", token="deadbeef_invalid_token_9999999999")
    record_result(
        "GET /api/v1/auth/me with forged token returns HTTP 401 Unauthorized",
        status == 401,
        f"status={status}"
    )

    # -------------------------------------------------------------
    # Stage 5: Valid Login & Bearer Token Authentication
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 5: Valid Login & Bearer Token Authentication ---{RESET}")
    target_user = registered_users[0]
    login_payload = {
        "username": target_user["profile"]["username"],
        "password": target_user["profile"]["password"],
        "device_name": "Verified-Desktop-Client",
    }
    status, resp, elapsed = http_request("/api/v1/auth/login", method="POST", body=login_payload)
    valid_login_ok = (
        status == 200
        and isinstance(resp, dict)
        and resp.get("token") is not None
        and resp.get("session_id") is not None
        and resp.get("user_id") == target_user["user_id"]
    )
    record_result(
        "Valid login returns HTTP 200 with new session token",
        valid_login_ok,
        f"status={status}, user_id={resp.get('user_id') if isinstance(resp, dict) else None}, latency={elapsed:.3f}s"
    )

    active_token = resp["token"]
    active_session_id = resp["session_id"]

    # Verify /api/v1/auth/me with new token
    status, me_resp, elapsed = http_request("/api/v1/auth/me", token=active_token)
    me_ok = (
        status == 200
        and isinstance(me_resp, dict)
        and me_resp.get("user_id") == target_user["user_id"]
        and me_resp.get("session_id") == active_session_id
    )
    record_result(
        "GET /api/v1/auth/me returns authentic user profile matching session",
        me_ok,
        f"status={status}, user_id={me_resp.get('user_id') if isinstance(me_resp, dict) else None}, session_id={me_resp.get('session_id') if isinstance(me_resp, dict) else None}"
    )

    # -------------------------------------------------------------
    # Stage 6: Live AI Bot Conversation Adversarial Stress Challenges
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 6: Live AI Bot Conversation Adversarial Challenges ---{RESET}")

    adversarial_prompts = [
        {
            "name": "SQL Injection Attack Vector",
            "prompt": "'; DROP TABLE users; SELECT * FROM sessions WHERE token = 'admin' --",
            "model": "grok-3",
        },
        {
            "name": "XSS & HTML Injection Attack Vector",
            "prompt": "<script>alert('XSS-INJECTION-VULN')</script><iframe src='javascript:malicious()'></iframe>",
            "model": "grok-3",
        },
        {
            "name": "Mathematical Logic & Reasoning",
            "prompt": "Please calculate: 24 * 36 + 1024 / 16 - 75. Provide the exact integer result and explain step-by-step.",
            "model": "grok-3",
        },
        {
            "name": "Multi-line Markdown, Unicode & Code Block",
            "prompt": "# 系统架构分析报告\n\n```rust\nfn calculate_hash(data: &[u8]) -> String {\n    format!(\"{:x}\", md5::compute(data))\n}\n```\n\n请针对上述 Rust 代码的安全隐患给出详细分析，并用中文列举改进方案。🚀",
            "model": "grok-3",
        },
    ]

    latencies = []

    for test_case in adversarial_prompts:
        agent_payload = {
            "prompt": test_case["prompt"],
            "session_id": active_session_id,
            "model_id": test_case["model"],
            "thinking_budget_tokens": 1024,
        }
        status, resp, elapsed = http_request("/api/v1/agent/runs", method="POST", body=agent_payload, token=active_token, timeout=20)
        latencies.append(elapsed)

        is_completed = (
            status == 200
            and isinstance(resp, dict)
            and resp.get("status") == "completed"
            and len(resp.get("response", "")) > 0
        )

        outputs = resp.get("outputs", []) if isinstance(resp, dict) else []
        has_thinking = any(isinstance(o, dict) and "Thinking" in o for o in outputs)
        has_final = any(isinstance(o, dict) and "FinalResponse" in o for o in outputs)

        record_result(
            f"AI Bot run: {test_case['name']} (status=completed, non-empty response)",
            is_completed,
            f"status={status}, latency={elapsed:.3f}s, resp_len={len(resp.get('response', '')) if isinstance(resp, dict) else 0}"
        )
        record_result(
            f"AI Bot structured outputs contain 'Thinking' & 'FinalResponse'",
            has_thinking and has_final,
            f"outputs_count={len(outputs)}, has_thinking={has_thinking}, has_final={has_final}"
        )
        record_result(
            f"Turn latency within acceptable threshold (< 5.0s)",
            elapsed < 5.0,
            f"latency={elapsed:.3f}s"
        )

    avg_latency = sum(latencies) / len(latencies)
    print(f"  {BLUE}[LATENCY STATS]{RESET} Min: {min(latencies):.3f}s, Max: {max(latencies):.3f}s, Avg: {avg_latency:.3f}s")

    # -------------------------------------------------------------
    # Stage 7: Rapid Prompt Bursts (5 Concurrent Requests)
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 7: Concurrency Stress Burst (5 Concurrent Requests) ---{RESET}")
    burst_count = 5

    def fire_concurrent_agent_run(i):
        payload = {
            "prompt": f"Burst request #{i}: verify concurrency resilience. Compute {i} * 111.",
            "session_id": active_session_id,
            "model_id": "grok-3",
            "thinking_budget_tokens": 512,
        }
        return http_request("/api/v1/agent/runs", method="POST", body=payload, token=active_token, timeout=25)

    print(f"  Firing {burst_count} simultaneous agent run requests with session '{active_session_id}'...")
    burst_start = time.time()
    burst_results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=burst_count) as executor:
        futures = [executor.submit(fire_concurrent_agent_run, i) for i in range(1, burst_count + 1)]
        for f in concurrent.futures.as_completed(futures):
            burst_results.append(f.result())
    burst_total_time = time.time() - burst_start

    successful_bursts = [
        r for r in burst_results
        if r[0] == 200 and isinstance(r[1], dict) and r[1].get("status") == "completed" and len(r[1].get("response", "")) > 0
    ]

    record_result(
        f"All {burst_count} concurrent agent runs succeeded with HTTP 200 OK",
        len(successful_bursts) == burst_count,
        f"success={len(successful_bursts)}/{burst_count}, total_wall_time={burst_total_time:.3f}s"
    )

    for idx, (b_status, b_resp, b_elapsed) in enumerate(burst_results, start=1):
        print(f"    -> Burst worker result #{idx}: status={b_status}, latency={b_elapsed:.3f}s, run_id={b_resp.get('run_id') if isinstance(b_resp, dict) else 'none'}")

    # -------------------------------------------------------------
    # Stage 8: Session Revocation Verification
    # -------------------------------------------------------------
    print(f"\n{YELLOW}--- Stage 8: Session Revocation & Invalidation ---{RESET}")
    status, resp, elapsed = http_request(f"/api/v1/auth/sessions/{active_session_id}", method="DELETE", token=active_token)
    record_result(
        "DELETE /api/v1/auth/sessions/{id} returns HTTP 200",
        status == 200,
        f"status={status}, latency={elapsed:.3f}s"
    )

    # Immediately verify that the revoked token can no longer authenticate
    status, resp, elapsed = http_request("/api/v1/auth/me", token=active_token)
    record_result(
        "Subsequent /api/v1/auth/me with revoked token returns HTTP 401 Unauthorized",
        status == 401,
        f"status={status}"
    )

    # -------------------------------------------------------------
    # Summary & Output
    # -------------------------------------------------------------
    print(f"\n{BLUE}======================================================================{RESET}")
    print(f"{BLUE}                     Empirical Verification Summary                    {RESET}")
    print(f"{BLUE}======================================================================{RESET}")
    print(f"  Total Assertions Passed: {GREEN}{passed_assertions}{RESET}")
    print(f"  Total Assertions Failed: {RED}{failed_assertions}{RESET}")

    if failed_assertions == 0:
        print(f"\n{GREEN}>>> VERDICT: APPROVE — ALL ADVERSARIAL STRESS CHALLENGES PASSED! <<<{RESET}\n")
        return 0
    else:
        print(f"\n{RED}>>> VERDICT: REQUEST_CHANGES — {failed_assertions} CHALLENGES FAILED! <<<{RESET}\n")
        return 1


if __name__ == "__main__":
    sys.exit(main())
