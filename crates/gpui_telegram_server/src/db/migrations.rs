use crate::db::schema::ALL_MIGRATIONS;
use gpui_telegram_core::error::{Result, TelegramError};
use rusqlite::Connection;

/// Runs all database schema migrations sequentially and idempotently.
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS _schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at_ms INTEGER NOT NULL
        );
        "#,
    )
    .map_err(|e| TelegramError::Database(format!("Failed to create _schema_migrations: {}", e)))?;

    for (idx, &migration_sql) in ALL_MIGRATIONS.iter().enumerate() {
        let version = (idx + 1) as i64;
        let already_applied: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM _schema_migrations WHERE version = ?1)",
                [version],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if !already_applied {
            if let Err(e) = conn.execute_batch(migration_sql) {
                let err_str = e.to_string();
                if !err_str.contains("duplicate column name") {
                    return Err(TelegramError::Database(format!("Migration v{} failed: {}", version, e)));
                }
            }

            let now_ms = chrono::Utc::now().timestamp_millis();
            conn.execute(
                "INSERT INTO _schema_migrations (version, applied_at_ms) VALUES (?1, ?2)",
                (version, now_ms),
            )
            .map_err(|e| TelegramError::Database(format!("Failed to record migration v{}: {}", version, e)))?;
        } else if migration_sql.contains("CREATE TABLE IF NOT EXISTS") {
            let _ = conn.execute_batch(migration_sql);
        }
    }

    Ok(())
}
