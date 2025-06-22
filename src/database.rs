use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use once_cell::sync::Lazy;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "db.sqlite".to_string());
    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    let database_url = if use_memory {
        ":memory:".to_string()
    } else {
        database_url.to_string()
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create database pool")
});

pub(crate) async fn migrate() {
    info!("Migrating database");

    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());

    if use_memory_env != "true" {
        let db_file = env::var("DATABASE_URL").unwrap_or_else(|_| "db.sqlite".to_string());

        if let Ok(metadata) = std::fs::metadata(&db_file) {
            if metadata.permissions().readonly() {
                let mut perms = metadata.permissions();

                perms.set_readonly(false);

                std::fs::set_permissions(&db_file, perms)
                    .expect("Failed to set write permissions on database file");
                info!("Fixed readonly permissions on database file");
            }
        }
    }

    let migrations = MIGRATIONS;
    let mut connection = DB_POOL.get().unwrap();

    connection
        .run_pending_migrations(migrations)
        .expect("Failed to run database migrations");
}
// ... existing code ...

pub(crate) async fn clean() -> Result<(), anyhow::Error> {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());

    if use_memory_env == "true" {
        migrate().await;

        return Ok(());
    }

    info!("Cleaning database");

    let db_file = std::env::var("DATABASE_URL").unwrap_or("db.sqlite".to_string());

    let _ = std::fs::remove_file(db_file.clone());
    std::fs::File::create(db_file)?;

    migrate().await;

    Ok(())
}
