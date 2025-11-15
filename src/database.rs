use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use std::env;
use std::process;
use std::sync::OnceLock;
use tokio::sync::Mutex;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");
pub static DB_POOL: OnceLock<Mutex<DbPool>> = OnceLock::new();

pub fn initialize_pool(database_url: String) -> Result<(), String> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(20)
        .build(manager)
        .map_err(|e| format!("Failed to create the pool: {}", e))?;

    DB_POOL
        .set(Mutex::new(pool))
        .map_err(|_| "Pool already initialized".to_string())
}

pub fn get_pool() -> &'static Mutex<DbPool> {
    DB_POOL
        .get()
        .expect("The pool not initialized. Call the initialize_pool first.")
}

pub async fn migrate() {
    info!("Migrating the database");

    let mut connection = get_pool()
        .lock()
        .await
        .get()
        .expect("Failed to get DB connection");

    sql_query("PRAGMA busy_timeout = 6000;")
        .execute(&mut connection)
        .expect("Failed to set busy timeout");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");

    info!("Database migrations applied");
}
