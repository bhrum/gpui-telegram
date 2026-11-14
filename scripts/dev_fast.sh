#!/usr/bin/env bash
# scripts/dev_fast.sh — Local Turbo Development Runner for GPUI-Telegram
# Usage:
#   ./scripts/dev_fast.sh ui          # Fast incremental check of desktop UI (~2-4s after warm build)
#   ./scripts/dev_fast.sh check       # Fast incremental check of desktop + server + client binaries
#   ./scripts/dev_fast.sh test        # Parallel cargo-nextest execution of core & desktop parity suites
#   ./scripts/dev_fast.sh run         # Launch GPUI desktop app with release-fast (120fps preview, no LTO wait)
#   ./scripts/dev_fast.sh cache-build # Build using local sccache object cache (ideal across git worktrees/branches)
#   ./scripts/dev_fast.sh stats       # Show local sccache hit statistics

set -euo pipefail
export PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

MODE="${1:-ui}"

case "$MODE" in
  ui)
    echo "[dev_fast] Incremental check: gpui_telegram_desktop..."
    cargo dcheck
    ;;
  check)
    echo "[dev_fast] Incremental check: desktop + server + client..."
    cargo fcheck
    ;;
  test)
    echo "[dev_fast] Running parallel nextest suite..."
    AI_GATEWAY_URL="http://127.0.0.1:9/api/ai/chat/stream" cargo nextest run       -p gpui_telegram_proto --test proto_tests --test api_rpc_endpoints_tests       -p gpui_telegram_server --test db_pts_messaging_tests --test server_routes_auth_tests       -p gpui_telegram_client --test auth_client_tests       -p gpui_telegram_desktop         --test tdesktop_full_parity_integration_tests         --test desktop_views_tests         --test auth_wizard_tabs_tests         --test milestone_1_nav_calls_regression_tests         --test milestone_3_settings_regression_tests
    ;;
  run)
    echo "[dev_fast] Launching GPUI Telegram Desktop (profile: release-fast)..."
    cargo frun
    ;;
  cache-build)
    echo "[dev_fast] Checking workspace with sccache object cache (CARGO_INCREMENTAL=0)..."
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo fcheck
    sccache --show-stats
    ;;
  stats)
    sccache --show-stats
    ;;
  *)
    echo "Usage: $0 {ui|check|test|run|cache-build|stats}"
    exit 1
    ;;
esac
