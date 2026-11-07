# Telegram Desktop 1:1 Sharded Source-File Parity Ledger

## Overview

This directory contains the authoritative, 100% full source-file parity ledger mapping every single file from upstream Telegram Desktop (`Telegram/SourceFiles/`) to target Rust and GPUI modules. Designed in strict compliance with **User Mandates #4, #5, #6, and #7**.

### Ledger Health Metrics

- **Total Upstream Source Files**: `3441`
- **Total Mapped Files**: `3441` (100.0% coverage)
- **Omitted Files**: `0`
- **Unmapped Files**: `0`
- **Independent Subsystem Shards**: `28`
- **Anti-Symbol-Collapse**: Strictly enforced (Fan-in <= 2, companion pairs only)
- **Anti-Monolith Layout**: Strictly enforced (Max 12 files per target .rs file)
- **Deep Source Reading**: Authentic SHA-256 hashes, line counts, and AST/regex extracted symbols for every entry

## 28 Subsystem Shards Catalog

| # | Subsystem | Source Files | JSON Shard | Markdown Shard | Primary Feature IDs |
|---|---|---|---|---|---|
| 1 | `api` | 116 | [`api.json`](api.json) | [`api.md`](api.md) | TCG-N01, TCG-N02, TCG-N11, TCG-N12 |
| 2 | `boxes` | 219 | [`boxes.json`](boxes.json) | [`boxes.md`](boxes.md) | TCG-N02, TCG-N03, TCG-N11, TCG-N12 |
| 3 | `calls` | 88 | [`calls.json`](calls.json) | [`calls.md`](calls.md) | TCG-N24, TCG-N25, TCG-N26 |
| 4 | `chat_helpers` | 57 | [`chat_helpers.json`](chat_helpers.json) | [`chat_helpers.md`](chat_helpers.md) | TCG-N11, TCG-N15, TCG-N27, TCG-N40 |
| 5 | `core` | 276 | [`core.json`](core.json) | [`core.md`](core.md) | CAP-06, TCG-N02, TCG-N09, TCG-N11 |
| 6 | `data` | 226 | [`data.json`](data.json) | [`data.md`](data.md) | CAP-06, TCG-N02, TCG-N04, TCG-N11 |
| 7 | `dialogs` | 83 | [`dialogs.json`](dialogs.json) | [`dialogs.md`](dialogs.md) | TCG-N03, TCG-N11, TCG-N13, TCG-N15 |
| 8 | `editor` | 95 | [`editor.json`](editor.json) | [`editor.md`](editor.md) | TCG-N06, TCG-N08, TCG-N09, TCG-N15 |
| 9 | `export` | 35 | [`export.json`](export.json) | [`export.md`](export.md) | TCG-N43 |
| 10 | `history` | 302 | [`history.json`](history.json) | [`history.md`](history.md) | CAP-06, TCG-N02, TCG-N04, TCG-N07 |
| 11 | `info` | 252 | [`info.json`](info.json) | [`info.md`](info.md) | TCG-N11, TCG-N12, TCG-N14, TCG-N15 |
| 12 | `inline_bots` | 20 | [`inline_bots.json`](inline_bots.json) | [`inline_bots.md`](inline_bots.md) | TCG-N27 |
| 13 | `intro` | 38 | [`intro.json`](intro.json) | [`intro.md`](intro.md) | TCG-N01 |
| 14 | `iv` | 147 | [`iv.json`](iv.json) | [`iv.md`](iv.md) | TCG-N02, TCG-N09, TCG-N21, TCG-N31 |
| 15 | `lang` | 25 | [`lang.json`](lang.json) | [`lang.md`](lang.md) | TCG-N44 |
| 16 | `layout` | 12 | [`layout.json`](layout.json) | [`layout.md`](layout.md) | TCG-N04 |
| 17 | `main` | 21 | [`main.json`](main.json) | [`main.md`](main.md) | TCG-N02, TCG-N46 |
| 18 | `media` | 151 | [`media.json`](media.json) | [`media.md`](media.md) | TCG-N06, TCG-N07, TCG-N08, TCG-N09 |
| 19 | `mtproto` | 76 | [`mtproto.json`](mtproto.json) | [`mtproto.md`](mtproto.md) | TCG-N02, TCG-N20, TCG-N35 |
| 20 | `passport` | 29 | [`passport.json`](passport.json) | [`passport.md`](passport.md) | TCG-N02 |
| 21 | `payments` | 53 | [`payments.json`](payments.json) | [`payments.md`](payments.md) | CAP-06, TCG-N14, TCG-N24 |
| 22 | `platform` | 132 | [`platform.json`](platform.json) | [`platform.md`](platform.md) | CAP-06, TCG-N01, TCG-N10, TCG-N17 |
| 23 | `settings` | 121 | [`settings.json`](settings.json) | [`settings.md`](settings.md) | TCG-N01, TCG-N02, TCG-N11, TCG-N14 |
| 24 | `statistics` | 43 | [`statistics.json`](statistics.json) | [`statistics.md`](statistics.md) | TCG-N12, TCG-N37 |
| 25 | `storage` | 47 | [`storage.json`](storage.json) | [`storage.md`](storage.md) | TCG-N11, TCG-N34, TCG-N37, TCG-N51 |
| 26 | `ui` | 454 | [`ui.json`](ui.json) | [`ui.md`](ui.md) | TCG-N02, TCG-N07, TCG-N08, TCG-N09 |
| 27 | `wallet` | 195 | [`wallet.json`](wallet.json) | [`wallet.md`](wallet.md) | CAP-06, TCG-N15, TCG-N31, TCG-N35 |
| 28 | `window` | 128 | [`window.json`](window.json) | [`window.md`](window.md) | TCG-N02, TCG-N11, TCG-N15, TCG-N31 |

## Automated Verification Gate

To verify ledger closure across all 5 verification gates, execute:

```bash
python3 scripts/verify_ledger_closure.py
```
