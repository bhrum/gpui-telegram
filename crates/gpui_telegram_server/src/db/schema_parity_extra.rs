pub const MIGRATION_V24_MEDIA_STICKERS_STORIES_CALLS: &str = r#"
CREATE TABLE IF NOT EXISTS media_files (
    media_id TEXT PRIMARY KEY,
    uploader_id INTEGER NOT NULL REFERENCES users(user_id),
    chat_id INTEGER,
    filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    sha256 TEXT NOT NULL,
    width INTEGER,
    height INTEGER,
    duration_seconds INTEGER,
    waveform_json TEXT,
    thumbnail_data_url TEXT,
    is_voice INTEGER NOT NULL DEFAULT 0,
    is_video_note INTEGER NOT NULL DEFAULT 0,
    created_at_ms INTEGER NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS idx_media_files_uploader ON media_files(uploader_id);
CREATE INDEX IF NOT EXISTS idx_media_files_chat ON media_files(chat_id);

CREATE TABLE IF NOT EXISTS sticker_sets (
    set_id TEXT PRIMARY KEY,
    short_name TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    is_animated INTEGER NOT NULL DEFAULT 0,
    is_video INTEGER NOT NULL DEFAULT 0,
    is_official INTEGER NOT NULL DEFAULT 1,
    created_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS stickers (
    sticker_id TEXT PRIMARY KEY,
    set_id TEXT NOT NULL REFERENCES sticker_sets(set_id) ON DELETE CASCADE,
    emoji TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    width INTEGER NOT NULL DEFAULT 512,
    height INTEGER NOT NULL DEFAULT 512,
    file_id TEXT,
    lottie_json TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0
) STRICT;

CREATE TABLE IF NOT EXISTS user_sticker_sets (
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    set_id TEXT NOT NULL REFERENCES sticker_sets(set_id) ON DELETE CASCADE,
    installed_at_ms INTEGER NOT NULL,
    PRIMARY KEY (user_id, set_id)
) STRICT;

CREATE TABLE IF NOT EXISTS gifs_catalog (
    gif_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    tags TEXT NOT NULL,
    width INTEGER NOT NULL DEFAULT 320,
    height INTEGER NOT NULL DEFAULT 240,
    duration_secs INTEGER NOT NULL DEFAULT 3,
    mime_type TEXT NOT NULL DEFAULT 'image/gif',
    url TEXT NOT NULL,
    preview_url TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS stories (
    story_id TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    media_id TEXT,
    media_url TEXT,
    media_type TEXT NOT NULL DEFAULT 'photo',
    caption TEXT,
    privacy TEXT NOT NULL DEFAULT 'everyone',
    view_count INTEGER NOT NULL DEFAULT 0,
    created_at_ms INTEGER NOT NULL,
    expires_at_ms INTEGER NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS idx_stories_expires ON stories(expires_at_ms);

CREATE TABLE IF NOT EXISTS story_views (
    story_id TEXT NOT NULL REFERENCES stories(story_id) ON DELETE CASCADE,
    viewer_user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    reaction TEXT,
    viewed_at_ms INTEGER NOT NULL,
    PRIMARY KEY (story_id, viewer_user_id)
) STRICT;

CREATE TABLE IF NOT EXISTS calls (
    call_id TEXT PRIMARY KEY,
    chat_id INTEGER,
    caller_id INTEGER NOT NULL REFERENCES users(user_id),
    callee_id INTEGER NOT NULL REFERENCES users(user_id),
    is_video INTEGER NOT NULL DEFAULT 0,
    state TEXT NOT NULL,
    duration_secs INTEGER NOT NULL DEFAULT 0,
    rating INTEGER,
    feedback TEXT,
    started_at_ms INTEGER NOT NULL,
    ended_at_ms INTEGER
) STRICT;
CREATE INDEX IF NOT EXISTS idx_calls_participants ON calls(caller_id, callee_id, started_at_ms DESC);
"#;

pub const MIGRATION_V25_WALLET_ACP: &str = r#"
CREATE TABLE IF NOT EXISTS wallet_balances (
    user_id INTEGER PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE,
    stars_balance INTEGER NOT NULL DEFAULT 0,
    ton_nano INTEGER NOT NULL DEFAULT 0,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS wallet_transactions (
    tx_id TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    peer_user_id INTEGER,
    kind TEXT NOT NULL,
    amount_stars INTEGER NOT NULL,
    description TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS idx_wallet_tx_user ON wallet_transactions(user_id, created_at_ms DESC);

CREATE TABLE IF NOT EXISTS acp_tasks (
    task_id TEXT PRIMARY KEY,
    payload_json TEXT NOT NULL,
    status TEXT NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS acp_approval_gates (
    gate_id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    decision TEXT,
    payload_json TEXT NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS mcp_servers_db (
    server_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    status TEXT NOT NULL,
    tools_json TEXT NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS computer_leases_db (
    lease_id TEXT PRIMARY KEY,
    owner_user_id INTEGER NOT NULL,
    session_id TEXT,
    status TEXT NOT NULL,
    actions_json TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS talk_sessions_db (
    session_id TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    transcript_json TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;
"#;
