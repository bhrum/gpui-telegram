# FUS-R002: fail-visible durable storage startup

Status: implementation authorized; no acceptance claimed. Parent scope remains full Telegram/Fabushi migration.

## Evidence and goal

`AppState::open_database` in `crates/gpui_telegram_server/src/server.rs` currently catches any `ServerDatabase::open(path)` error and substitutes `ServerDatabase::open_in_memory()` with only a warning log. In addition:
- `ServerConfig::resolve_db_path()` strips `"sqlite://"` / `"sqlite:"` into `""` when the path portion is empty, and `AppState::open_database` treats `path.is_empty()` as in-memory mode;
- `SqlitePool::open` does not reject empty/whitespace paths (which SQLite's C API treats as a private temporary database deleted on close) and does not run `PRAGMA quick_check(1)` to catch structural B-tree corruption on open;
- `run_migrations` masks query errors on `_schema_migrations` via `.unwrap_or(false)`;
- `AppState::with_db` ignores `db.get_max_user_id()` errors via `if let Ok(max_id)`, allowing broken `users` table state to pass startup;
- `main.rs` calls infallible `AppState::new(config)` and binds `TcpListener`, serving `GET /health` (`200 OK`) and `GET /api/v1/status` (`"status": "healthy"`) against an empty ephemeral store when the production SQLite database is inaccessible or corrupt.

This slice makes production and file-backed SQLite startup failures fail visibly before the server binds its listening socket, while preserving explicit in-memory mode (`:memory:`, `sqlite::memory:`, `sqlite://:memory:`) for tests.

## Exclusive worker boundary

Worker may edit ONLY:
- `docs/specs/fusion-durable-storage-startup-20261108.md` (new spec)
- `crates/gpui_telegram_server/src/config.rs`
- `crates/gpui_telegram_server/src/db/pool.rs`
- `crates/gpui_telegram_server/src/db/migrations.rs`
- `crates/gpui_telegram_server/src/server.rs` (`AppState` constructors and `open_database` only; preserve existing S001 chat route changes and all other handlers)
- `crates/gpui_telegram_server/src/main.rs`
- `crates/gpui_telegram_server/tests/fusion_durable_storage_startup.rs` (new test file)

Do not touch `db/mod.rs`, schemas, Cargo manifests, desktop/client crates, or unrelated tests. Do not run builds, tests, or target cleanup while parallel writers are active.

## Behavior and failure cases

1. **Explicit In-Memory Mode vs. File-Backed Storage (`ServerConfig`)**:
   - `ServerConfig::is_in_memory(&self) -> bool` returns `true` only when `database_url.trim()` is explicitly `":memory:"`, `"sqlite::memory:"`, or `"sqlite://:memory:"`.
   - `ServerConfig::resolve_db_path(&self) -> &str` trims surrounding whitespace and strips `"sqlite://"` or `"sqlite:"` prefixes.
   - Empty or whitespace-only URLs (`""`, `"   "`) and bare scheme prefixes (`"sqlite://"`, `"sqlite:"`, `"sqlite://   "`) are NOT treated as in-memory mode; they resolve to an empty path and fail startup with `Err(TelegramError::Database(_))`.

2. **Connection Pool Open & Integrity Verification (`SqlitePool`)**:
   - `SqlitePool::open(path)` rejects empty or whitespace-only paths before calling `rusqlite::Connection::open`, preventing SQLite from creating an unnamed temporary database.
   - Directory creation failures on `path.parent()` and connection open failures return `Err(TelegramError::Database(_))`.
   - `SqlitePool::apply_pragmas(conn)` executes the WAL/foreign-key pragmas and verifies `PRAGMA quick_check(1)` returns `"ok"`, returning `Err(TelegramError::Database(_))` on header or B-tree page corruption.

3. **Schema Migration & Startup Alignment (`run_migrations`, `AppState`)**:
   - `run_migrations(conn)` propagates any error querying `_schema_migrations` as `Err(TelegramError::Database(_))` instead of `.unwrap_or(false)`.
   - `AppState::try_new(config: ServerConfig) -> gpui_telegram_core::error::Result<Self>` opens the database via fallible `AppState::open_database(&config)` (with zero fallback to `open_in_memory()`), verifies `db.get_max_user_id()?` succeeds on the migrated schema, and aligns `session_manager` user ID allocation.
   - `AppState::new(config: ServerConfig) -> Self` delegates to `Self::try_new(config)` and panics visibly with the database error if initialization fails, preserving source compatibility for existing test callers using `ServerConfig::default()` while ensuring broken file configs never silently switch to `:memory:`.
   - `main.rs` calls `AppState::try_new(config.clone())`; on `Err(e)`, it logs `tracing::error!`, prints a diagnostic error to `stderr`, and returns `Err(e.into())` before `TcpListener::bind(addr)`.

## Verification and acceptance criteria

Add focused integration tests in `crates/gpui_telegram_server/tests/fusion_durable_storage_startup.rs` using an owned RAII temp directory guard under `std::env::temp_dir()` (restoring permissions before removal on `Drop`):

- **Negative checks (must fail visibly without `:memory:` fallback)**:
  1. Malformed/empty database URLs (`""`, `"   "`, `"sqlite://"`, `"sqlite:"`, `"sqlite://   "`) fail `AppState::try_new` and `SqlitePool::open("")` with `Err(TelegramError::Database(_))`.
  2. Target path is an existing directory, or parent path is a regular file: `AppState::try_new` returns `Err(TelegramError::Database(_))`, and `AppState::new` panics visibly.
  3. Target path is inside a read-only (`0o500`) directory (when running as non-root): `AppState::try_new` returns `Err(TelegramError::Database(_))`.
  4. Corrupt SQLite file (both invalid header bytes and valid header with corrupted interior B-tree pages / dropped `users` table): `AppState::try_new` returns `Err(TelegramError::Database(_))` without truncating or overwriting the corrupt file and without opening `:memory:`.
  5. Process-level binary startup (`env!("CARGO_BIN_EXE_gpui_telegram_server")`) with `DATABASE_URL` pointing to a corrupt SQLite file and an ephemeral port: exits with a non-zero status code, emits diagnostic stderr output, and never binds `/health`.

- **Positive checks (must succeed and preserve durability)**:
  1. Valid file-backed `DATABASE_URL` (`sqlite://<temp_dir>/durable.db`) succeeds via `AppState::try_new`, reports `state.db.pool().path() == Some(expected_path)`, persists users and chats across drop and reopen, and monotonically advances `next_user_id` above `MAX(user_id)`.
  2. Explicit in-memory configurations (`:memory:`, `sqlite::memory:`, `sqlite://:memory:`, `ServerConfig::default()`) succeed via `AppState::try_new` and `AppState::new` with `state.db.pool().path() == None`.

Coordinator serializes test execution (`cargo test -p gpui_telegram_server --test fusion_durable_storage_startup` and `cargo test -p gpui_telegram_server --lib config`) and immediately runs `python3 scripts/clean_build_garbage.py`. Static edits and test definitions alone do not constitute acceptance.
