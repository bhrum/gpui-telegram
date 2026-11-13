# Dispatch: challenger_module_10_2

## Role
Group Calls, Radial Waveforms & Volume Snapping Adversarial Challenger.

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_10_2`

## Project Root
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Authoritative Specification
- Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Module Spec: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/modules/module_10_calls.md`

## Instructions
1. Author adversarial blind C++ oracle test suite at:
   `crates/gpui_telegram_webrtc/tests/calls_group_adversarial_oracle_tests.rs`
   covering at least 5 blind C++ oracles:
   - Oracle 1: Speak Request Priority Queue: Verify priority ordering and hand-raise timestamp tie-breakers.
   - Oracle 2: Spacebar Push-To-Talk: Verify 250ms hysteresis delay (`kSpacePushToTalkDelay = 250ms`) preventing choppy audio on accidental release.
   - Oracle 3: RTMP Live Stream URL & Key: Verify RTMP URL parsing, stream key generation, and validation.
   - Oracle 4: Speaking Radial Energy Waveform: Verify multi-harmonic ring expansion and scale clamping (`kBlobScale = 0.605`).
   - Oracle 5: Participant Volume Magnetic Snap: Verify volume slider range [0..200%] and ±5% magnetic snap window at 100% (values in [95..105] snap to 100).
2. Comply with Mandate #20: writes <= 200 lines per call.
3. Execute tests:
   `cargo test --package gpui_telegram_webrtc --test calls_group_adversarial_oracle_tests`
4. Run `python3 scripts/clean_build_garbage.py`.
5. Author handoff report in `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_10_2/handoff.md` with explicit verdict `APPROVE` or `REQUEST_CHANGES` and send completion message to orchestrator.


## 2026-11-07T06:38:52Z
You are challenger_module_10_2.
Your working directory is /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_10_2.
The project workspace is /Users/bhrum/teamwork_projects/gpui_telegram.

Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md and /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_10_2/DISPATCH.md before starting work. DO NOT read files in /Users/bhrum/Documents/ChatGPT/.agents/teamwork/ORIGINAL_REQUEST.md.

Mandate #20 Stream Hygiene: Never generate or write >200 lines in a single write_to_file or replace_file_content call.
Mandate #11 / AGENTS.md Build Hygiene: Always run `python3 scripts/clean_build_garbage.py` after cargo commands.

Tasks:
1. Author adversarial blind C++ oracle test suite at:
   `crates/gpui_telegram_webrtc/tests/calls_group_adversarial_oracle_tests.rs`
   covering at least 5 blind C++ oracles:
   - Oracle 1: Speak Request Priority Queue: Verify priority ordering and hand-raise timestamp tie-breakers.
   - Oracle 2: Spacebar Push-To-Talk: Verify 250ms hysteresis delay (`kSpacePushToTalkDelay = 250ms`) preventing choppy audio on accidental release.
   - Oracle 3: RTMP Live Stream URL & Key: Verify RTMP URL parsing, stream key generation, and validation.
   - Oracle 4: Speaking Radial Energy Waveform: Verify multi-harmonic ring expansion and scale clamping (`kBlobScale = 0.605`).
   - Oracle 5: Participant Volume Magnetic Snap: Verify volume slider range [0..200%] and ±5% magnetic snap window at 100% (values in [95..105] snap to 100).
2. Run tests:
   `cargo test --package gpui_telegram_webrtc --test calls_group_adversarial_oracle_tests`
3. Run `python3 scripts/clean_build_garbage.py`.
4. Author handoff report in `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_10_2/handoff.md` with explicit verdict `APPROVE` or `REQUEST_CHANGES` and send completion message to orchestrator.
