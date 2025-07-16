use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel::{prelude::*, sql_query};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use once_cell::sync::Lazy;
use std::env;
use tokio::sync::Mutex;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

pub static DB_POOL: Lazy<Mutex<DbPool>> = Lazy::new(|| {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "db.sqlite".to_string());
    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    let database_url = if use_memory {
        "file:memdb1?mode=memory&cache=shared".to_string()
    } else {
        database_url.to_string()
    };

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool_size = 20;

    Mutex::new(
        Pool::builder()
            .max_size(pool_size)
            .build(manager)
            .expect("Failed to create database pool"),
    )
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

    let mut connection = DB_POOL.lock().await.get().unwrap();

    sql_query("PRAGMA busy_timeout = 6000;")
        .execute(&mut connection)
        .expect("Failed to set busy timeout");

    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    if use_memory {
        sql_query("PRAGMA journal_mode = MEMORY;")
            .execute(&mut connection)
            .expect("Failed to set journal mode");
    }

    connection
        .revert_all_migrations(MIGRATIONS)
        .expect("Failed to rollback migrations");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");

    info!("Database migrations applied");
}

pub(crate) async fn clean() -> Result<(), anyhow::Error> {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());

    info!("Cleaning database");

    if use_memory_env != "true" {
        let db_file = std::env::var("DATABASE_URL").unwrap_or("db.sqlite".to_string());

        std::fs::File::create(&db_file)?;

        let mut perms = std::fs::metadata(&db_file)?.permissions();

        perms.set_readonly(false);

        std::fs::set_permissions(&db_file, perms)?;
    }

    migrate().await;

    Ok(())
}
