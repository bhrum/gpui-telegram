## 2026-11-07T11:06:47Z

You are worker_module_13_iv_part3, a dedicated implementation worker for Module 13 Instant View Part 3.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_iv_part3
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Upstream C++ sources: /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/iv/
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/iv.json

SCOPE: Slice 8 covers `iv.json` sorted targets [37..56] (indices 37..55, exactly 19 target files, 38 upstream source files):
- Targets:
  1. `crates/gpui_telegram_desktop/src/views/iv/iv_zoom_controls.rs`
  2. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article.rs`
  3. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_caption_hit.rs`
  4. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_layout_blocks.rs`
  5. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_layout_structure.rs`
  6. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_paint.rs`
  7. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_quote.rs`
  8. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_scroll_forwarder.rs`
  9. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_selection.rs`
  10. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_article_text.rs`
  11. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_button_row.rs`
  12. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_common.rs`
  13. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_controller.rs`
  14. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_document.rs`
  15. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_embed_overlay.rs`
  16. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_history_view_media.rs`
  17. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_math.rs`
  18. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_math_renderer.rs`
  19. `crates/gpui_telegram_desktop/src/views/iv/markdown/iv_markdown_media_block.rs`
- Upstream files: All 38 upstream `source_path` entries mapped to these 19 targets in `iv.json`.

MISSION STEPS:
1. Initialize your working directory with DISPATCH.md, BRIEFING.md, and progress.md.
2. Step 13A: Author `docs/specs/modules/module_13_iv_part3.md` documenting 100% of all 38 upstream files in the manifest with Mandate #18 4-Layer Anti-Bluffing Proof (exact line citations `.cpp:L<start>-L<end>`, constants, algorithms). Verify 0 omitted files.
3. Step 13B: Completely eradicate boilerplate patterns A, B, C, D across all 19 assigned targets. Implement authentic GPUI Render logic and Markdown/Media controllers.
4. Step 13C: Verify with `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`. Keep single write calls <= 200 lines (Mandate #20).
5. Step 13D: Author handoff.md with 5 components (Observation, Logic Chain, Caveats, Conclusion, Verification Method) and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
