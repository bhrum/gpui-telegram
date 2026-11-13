#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# GPUI-Telegram: One-Command Git Commit, Push & GitHub Actions Live Verifier
# Usage:
#   bash scripts/push_and_verify_gh_actions.sh [optional_remote_url] [optional_branch]
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

DEFAULT_REMOTE_URL="${GITHUB_REPO_URL:-git@github.com:bhrum/gpui-telegram.git}"
REMOTE_URL="${1:-${DEFAULT_REMOTE_URL}}"
COMMIT_MSG="${COMMIT_MSG:-feat(desktop): migrate 1:1 telegramdesktop features, GPUI UI/UX & GitHub Actions CI}"

echo "======================================================================"
echo "1. Running Pre-Push Static Ledger & File Hygiene Verification..."
echo "======================================================================"
python3 scripts/verify_ledger_closure.py --ci-without-upstream --skip-cargo
python3 scripts/clean_build_garbage.py

echo ""
echo "======================================================================"
echo "2. Preparing Git Repository & Staging Changes..."
echo "======================================================================"
if [ ! -d ".git" ]; then
  echo "No .git directory detected — initializing repository on branch 'main'..."
  git init -b main
  git remote add origin "${REMOTE_URL}"
else
  if ! git remote get-url origin >/dev/null 2>&1; then
    echo "Adding remote 'origin' -> ${REMOTE_URL}"
    git remote add origin "${REMOTE_URL}"
  fi
fi

CURRENT_BRANCH="${2:-$(git symbolic-ref --short HEAD 2>/dev/null || echo main)}"
echo "Target remote : $(git remote get-url origin)"
echo "Target branch : ${CURRENT_BRANCH}"

git add -A
if git diff --cached --quiet; then
  echo "Working tree already clean and committed."
else
  git status --short
  git commit -m "${COMMIT_MSG}"
fi

echo ""
echo "======================================================================"
echo "3. Pushing Commit to GitHub..."
echo "======================================================================"
git push -u origin "${CURRENT_BRANCH}"
HEAD_SHA="$(git rev-parse HEAD)"
echo "Pushed commit SHA: ${HEAD_SHA}"

echo ""
echo "======================================================================"
echo "4. Watching GitHub Actions Verification Run..."
echo "======================================================================"
if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI (gh) authenticated — polling workflow run for commit ${HEAD_SHA:0:8}..."
  RUN_ID=""
  for _ in {1..15}; do
    RUN_ID="$(gh run list --commit "${HEAD_SHA}" --workflow "ci.yml" --json databaseId --jq '.[0].databaseId // empty' 2>/dev/null || true)"
    if [ -n "${RUN_ID}" ]; then
      break
    fi
    sleep 3
  done

  if [ -n "${RUN_ID}" ]; then
    echo "Streaming GitHub Actions Run #${RUN_ID}..."
    gh run watch "${RUN_ID}" --exit-status
    echo "SUCCESS: GitHub Actions verification passed for commit ${HEAD_SHA}!"
  else
    echo "Workflow queued on GitHub. Check status with: gh run list --limit 5"
  fi
else
  ORIGIN_URL="$(git remote get-url origin)"
  HTTPS_URL="${ORIGIN_URL/git@github.com:/https://github.com/}"
  HTTPS_URL="${HTTPS_URL%.git}"
  echo "Push succeeded! View live GitHub Actions verification at:"
  echo "  ${HTTPS_URL}/actions"
fi
