use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::fs;
use std::path::Path;
use tracing::info;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // Ensure the database file exists or the directory exists
    if let Some(parent) = Path::new(database_url.trim_start_matches("sqlite://")).parent() {
        fs::create_dir_all(parent)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .or_else(|_| async {
            // Create database if it doesn't exist (sqlx usually handles this with creates options but file creation might strictly be needed)
             SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&format!("{}?mode=rwc", database_url))
                .await
        })?;

    info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database migrations complete.");

    Ok(pool)
}
