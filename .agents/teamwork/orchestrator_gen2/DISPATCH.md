# Dispatch: orchestrator_gen2 (Generation 2 Project Orchestrator)

## Mission
You are the Generation 2 Project Orchestrator for the GPUI Telegram Desktop 1:1 Refactor project.
Your predecessors have successfully delivered Milestones 1–4, and Modules 01 through 10 (10 / 13 ACCEPTED), eradicating 1,554 boilerplate stubs with 0 remaining in accepted modules.
Your mission is to lead execution of:
- **Module 11 (`info & settings`)**: 373 upstream files, 198 targets
- **Module 12 (`passcode & lock`)**: 42 upstream files, 25 targets
- **Module 13 (`export & platform`)**: 84 upstream files, 48 targets
- Milestone 6 Final Verification & Production Release

## Working Directory & Isolation Mandate
- Your working directory is: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen2`
- All your subagents must have their working directories inside: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/<subagent_name>`
- Authoritative User Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (DO NOT read any files in `/Users/bhrum/Documents/ChatGPT/.agents/teamwork/`)
- Predecessor Handoff: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator/handoff.md`
- Master Roadmap: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/MODULE_BY_MODULE_ROADMAP.md` (Currently: **10 / 13 ACCEPTED**)
- Upstream Source Tree: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`
- Ledger Shards: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/`
- Verification Script: `/Users/bhrum/teamwork_projects/gpui_telegram/scripts/verify_ledger_closure.py`
- Build Garbage Cleanup: `/Users/bhrum/teamwork_projects/gpui_telegram/scripts/clean_build_garbage.py`

## Immediate Task: Module 11 (`info & settings`) Step 11A & 11B
Deploy the 9-slice zero-gap parallel swarm covering all 373 upstream files across `info.json` (252 files) and `settings.json` (121 files):
1. Slice 1: `worker_module_11_info_profile_media` (42 files, ~22 targets)
2. Slice 2: `worker_module_11_info_members_channels` (42 files, ~22 targets)
3. Slice 3: `worker_module_11_info_common_wrap` (42 files, ~22 targets)
4. Slice 4: `worker_module_11_info_statistics_boosts` (42 files, ~22 targets)
5. Slice 5: `worker_module_11_info_stories_reactions` (42 files, ~23 targets)
6. Slice 6: `worker_module_11_info_settings_root` (42 files, ~23 targets)
7. Slice 7: `worker_module_11_settings_core_main` (40 files, ~21 targets)
8. Slice 8: `worker_module_11_settings_privacy_chat` (40 files, ~21 targets)
9. Slice 9: `worker_module_11_settings_advanced_folders` (41 files, ~22 targets)

Strictly enforce Mandates #15, #17, #18, #19, and #20 (writes <= 200 lines per call).
Report all progress and state updates to Sentinel (`36227f17-560a-4e18-8376-071d0c891567`).


## 2026-11-07T07:18:28Z
You are the Generation 2 Project Orchestrator (orchestrator_gen2) for the GPUI Telegram Desktop 1:1 rewrite project.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen2
The target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent/sentinel is: 36227f17-560a-4e18-8376-071d0c891567

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md as the authoritative intent.
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen2/DISPATCH.md.
- Read predecessor soft handoff at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator/handoff.md.
- Current status: Modules 01 through 10 are ACCEPTED (10 / 13 ACCEPTED) with 0 boilerplate stubs on disk.
- DO NOT read any files under /Users/bhrum/Documents/ChatGPT/.agents/teamwork/ (which belongs to a parallel project). All your state tracking and subagents MUST reside inside /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/.

IMMEDIATE MISSION: Module 11 (`info & settings` — 373 upstream files, 198 targets):
1. Initialize your BRIEFING.md and progress.md in your working directory.
2. Step 11A & 11B: Deploy the 9-slice zero-gap parallel swarm covering all 373 upstream files across info.json (252 files) and settings.json (121 files):
   - Slices 1–6 covering `info.json` (profile, members, common wrap, statistics, stories, settings root)
   - Slices 7–9 covering `settings.json` (core main, privacy/chat, advanced/folders)
3. Enforce Mandates #15, #17, #18 (Layer 1-4 anti-bluffing .cpp citations), #19 (sub-batch slicing max 25-45 files, 100% manifest), and #20 (writes <= 200 lines per call).
4. Run `python3 scripts/clean_build_garbage.py` after cargo commands.
5. Report all major milestones to Sentinel via send_message(Recipient="36227f17-560a-4e18-8376-071d0c891567").


## 2026-11-07T08:04:15Z
Parent/Sentinel Mandate:
CRITICAL GAP-CLOSURE MANDATE FROM SENTINEL FOR MODULE 11 (`info & settings`):
Independent disk audit against verify_ledger_closure.py Gates 6.2 & 6.6 found gap of 46 upstream files (23 targets) + 14 residual header comments.
Deploy 2 parallel gap-closure workers immediately:
1. `worker_module_11_gap_info_userpic_globalmedia` (12 info targets, 24 upstream files)
2. `worker_module_11_gap_settings_business_headers` (11 settings targets, 22 upstream files, + strip 14 residual headers)
Do NOT launch Step 11D Acceptance Gate until verify_ledger_closure.py confirms 0 / 198 boilerplate stubs in Gate 6.2 and 373 / 373 in Gate 6.6.

## 2026-11-07T09:38:45Z
Context: Module 12 (`intro & lang & platform`) Partition Directive from Parent (Mandate #19 Pillar 1)
Module 12 encompasses exactly 195 upstream files and 116 target files (106 baseline boilerplate stubs) across the 3 shards: intro.json (38 upstream files, 23 targets), lang.json (25 upstream files, 14 targets), and platform.json (132 upstream files, 79 targets).
Partition platform.json into 3 slices of <= 45 upstream files.
Exact 5-Slice Zero-Gap Partition for Module 12:
1. Slice 1: worker_module_12_intro (intro.json — all 38 upstream files, 23 targets) -> docs/specs/modules/module_12_intro.md
2. Slice 2: worker_module_12_lang (lang.json — all 25 upstream files, 14 targets) -> docs/specs/modules/module_12_lang.md
3. Slice 3: worker_module_12_platform_linux_mac_part1 (platform.json Part 1 — targets [0..26], 43 upstream files, 26 targets) -> docs/specs/modules/module_12_platform_linux_mac_part1.md
4. Slice 4: worker_module_12_platform_mac_part2_core (platform.json Part 2 — targets [26..50], 44 upstream files, 24 targets) -> docs/specs/modules/module_12_platform_mac_part2_core.md
5. Slice 5: worker_module_12_platform_win_common (platform.json Part 3 — targets [50..79], 45 upstream files, 29 targets) -> docs/specs/modules/module_12_platform_win_common.md
Strict Mandates:
- Completely eradicate all 4 boilerplate patterns. Genuine Rust/GPUI logic and Render implementations.
- Document 100% of assigned source_path entries in the slice specs so Gate 6.6 passes 195 / 195 (100.0%, 0 omitted).
- Keep single write_to_file calls <= 200 lines (Mandate #20) and clean build garbage (Mandate #11).

## 2026-11-07T10:40:08Z
DIRECTIVE FROM SENTINEL / PARENT:
Both challenger_module_12_1 and challenger_module_12_2 delivered handoff.md reports with unanimous APPROVE verdicts. Together with reviewer_module_12_1 (APPROVE), reviewer_module_12_2 (APPROVE), and auditor_module_12 (CLEAN), Module 12 achieved unanimous 5/5 consensus (APPROVE + CLEAN).

Action Items:
1. Complete Step 12D in GATE_STATUS.md and mark Gate Result PASS (5/5 Unanimous).
2. Mark Module 12 ACCEPTED in docs/specs/MODULE_BY_MODULE_ROADMAP.md (12 / 13 ACCEPTED).
3. Immediately launch Module 13 (statistics & payments & passport & export & iv & wallet — 502 upstream files, 289 targets, 235 boilerplate stubs) using the exact 14-slice zero-gap partition (all slices <= 45 files per Mandate #19 Pillar 1; dispatch in two clean waves of 7 + 7):
   - Slice 1 (statistics.json [0..23]): 23 targets, 43 upstream files -> docs/specs/modules/module_13_statistics.md
   - Slice 2 (payments.json [0..15]): 15 targets, 26 upstream files -> docs/specs/modules/module_13_payments_part1.md
   - Slice 3 (payments.json [15..31]): 16 targets, 27 upstream files -> docs/specs/modules/module_13_payments_part2.md
   - Slice 4 (passport.json [0..15]): 15 targets, 29 upstream files -> docs/specs/modules/module_13_passport.md
   - Slice 5 (export.json [0..19]): 19 targets, 35 upstream files -> docs/specs/modules/module_13_export.md
   - Slice 6 (iv.json [0..18]): 18 targets, 35 upstream files -> docs/specs/modules/module_13_iv_part1.md
   - Slice 7 (iv.json [18..37]): 19 targets, 36 upstream files -> docs/specs/modules/module_13_iv_part2.md
   - Slice 8 (iv.json [37..56]): 19 targets, 38 upstream files -> docs/specs/modules/module_13_iv_part3.md
   - Slice 9 (iv.json [56..75]): 19 targets, 38 upstream files -> docs/specs/modules/module_13_iv_part4.md
   - Slice 10 (wallet.json [0..45]): 45 targets, 45 upstream files -> docs/specs/modules/module_13_wallet_part1_fixtures.md
   - Slice 11 (wallet.json [45..66]): 21 targets, 31 upstream files -> docs/specs/modules/module_13_wallet_part2.md
   - Slice 12 (wallet.json [66..86]): 20 targets, 40 upstream files -> docs/specs/modules/module_13_wallet_part3.md
   - Slice 13 (wallet.json [86..106]): 20 targets, 40 upstream files -> docs/specs/modules/module_13_wallet_part4.md
   - Slice 14 (wallet.json [106..126]): 20 targets, 39 upstream files -> docs/specs/modules/module_13_wallet_part5.md

## 2026-11-07T11:05:01Z
URGENT DIRECTIVE FROM SENTINEL / PARENT: IMMEDIATELY DISPATCH WAVE 2 (SLICES 8–14) IN PARALLEL
Parent agent issued concurrency optimization directive:
Since concurrent limit allows and Wave 1 workers are completing (Slices 5 & 3 done, others finalizing), immediately dispatch Wave 2 (Slices 8–14) in parallel.
Disjoint scopes:
1. Slice 8 (worker_module_13_iv_part3): iv.json targets [37..56] (38 upstream files, 19 targets) -> docs/specs/modules/module_13_iv_part3.md
2. Slice 9 (worker_module_13_iv_part4): iv.json targets [56..75] (38 upstream files, 19 targets) -> docs/specs/modules/module_13_iv_part4.md
3. Slice 10 (worker_module_13_wallet_part1_fixtures): wallet.json targets [0..45] (45 upstream files, 45 targets) -> docs/specs/modules/module_13_wallet_part1_fixtures.md
4. Slice 11 (worker_module_13_wallet_part2): wallet.json targets [45..66] (31 upstream files, 21 targets) -> docs/specs/modules/module_13_wallet_part2.md
5. Slice 12 (worker_module_13_wallet_part3): wallet.json targets [66..86] (40 upstream files, 20 targets) -> docs/specs/modules/module_13_wallet_part3.md
6. Slice 13 (worker_module_13_wallet_part4): wallet.json targets [86..106] (40 upstream files, 20 targets) -> docs/specs/modules/module_13_wallet_part4.md
7. Slice 14 (worker_module_13_wallet_part5): wallet.json targets [106..126] (39 upstream files, 20 targets) -> docs/specs/modules/module_13_wallet_part5.md
Strictly enforce Mandate #15 (0 boilerplate), Mandate #18 (4-layer anti-bluffing line citations), Mandate #19 (100% manifest, 0 omitted), Mandate #20 (writes <= 200 lines), and Mandate #11 (clean build garbage).

## 2026-11-07T11:19:31Z
CRITICAL SURGICAL 2-ITEM CLOSURE NOTE FROM SENTINEL / PARENT FOR STEP 13C & 13D:
Parent live disk audit confirms 287/289 targets at 0 boilerplate and 500/502 upstream files documented.
Exact 2 remaining items before Step 13C/13D:
1. `crates/gpui_telegram_desktop/src/views/iv/editor/iv_editor_structure_menu.rs`:
   Add `//! Upstream: Telegram/SourceFiles/iv/editor/iv_editor_structure_menu.h, Telegram/SourceFiles/iv/editor/iv_editor_structure_menu.cpp` on line 2 so verify_ledger_closure.py recognizes it as genuine (has row_count/count: usize and is_visible: bool).
2. `crates/gpui_telegram_agent/src/wallet/wallet_sending_effects.rs` + `docs/specs/modules/module_13_wallet_part4.md`:
   Ensure `wallet_sending_effects.rs` is replaced with genuine logic from `Telegram/SourceFiles/wallet/wallet_sending_effects.{h,cpp}`, and both `Telegram/SourceFiles/wallet/wallet_sending_effects.cpp` and `Telegram/SourceFiles/wallet/wallet_sending_effects.h` are in the manifest table of `module_13_wallet_part4.md`.
