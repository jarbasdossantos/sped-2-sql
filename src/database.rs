use once_cell::sync::Lazy;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

/// Global SQLite connection pool that can be used across the application.
/// This pool is lazily initialized when first accessed and maintains up to 20 connections.
/// The database can be configured to use either in-memory storage or a file-based database
/// through the USE_MEMORY_DB environment variable.
pub static DB_POOL: Lazy<Pool<Sqlite>> = Lazy::new(|| {
    let use_memory_env = std::env::var("USE_MEMORY_DB").unwrap_or_else(|_| "true".to_string());
    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    let database_url = if use_memory {
        "sqlite::memory:"
    } else {
        "sqlite:db.sqlite"
    };

    SqlitePoolOptions::new()
        .max_connections(20)
        .connect_lazy(database_url)
        .expect("Failed to create database pool")
});

/// Runs all database migrations located in the "./migrations" directory.
/// This function should be called during application startup to ensure
/// the database schema is up to date.
///
/// # Panics
///
/// This function will panic if migrations fail to execute.
pub async fn migrate() {
    sqlx::migrate!("./migrations")
        .run(&*DB_POOL)
        .await
        .expect("Failed to migrate");
}
