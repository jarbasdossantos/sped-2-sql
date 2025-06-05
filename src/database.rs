use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use once_cell::sync::Lazy;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());
    let db_file = env::var("DB_FILE").unwrap_or_else(|_| "jarbas.sqlite".to_string());
    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    let database_url = if use_memory {
        ":memory:".to_string()
    } else {
        db_file.to_string()
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create database pool")
});

/// Runs all database migrations located in the "./migrations" directory.
/// This function should be called during application startup to ensure
/// the database schema is up to date.
///
/// # Panics
///
/// This function will panic if migrations fail to execute.
pub(crate) async fn migrate() {
    info!("Migrating database");

    let migrations = MIGRATIONS;
    let mut connection = DB_POOL.get().unwrap();

    connection
        .run_pending_migrations(migrations)
        .expect("Failed to run database migrations");
}

pub(crate) async fn clean() -> Result<(), anyhow::Error> {
    info!("Cleaning database");

    let db_file = std::env::var("DB_FILE").unwrap_or("jarbas.sqlite".to_string());

    let _ = std::fs::remove_file(db_file.clone());
    std::fs::File::create(db_file)?;

    migrate().await;

    Ok(())
}
