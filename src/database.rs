use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;
use once_cell::sync::Lazy;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    let use_memory_env = env::var("USE_MEMORY_DB").unwrap_or_else(|_| "false".to_string());
    let db_file = env::var("DB_FILE").unwrap_or_else(|_| "jarbas.sqlite".to_string());
    let use_memory = use_memory_env.eq_ignore_ascii_case("true");

    let database_url = if use_memory {
        ":memory:".to_string()
    } else {
        format!("{}", db_file)
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

    // sqlx::migrate!("./migrations")
    //     .run(&*DB_POOL)
    //     .await
    //     .expect("Failed to migrate");
}

pub(crate) async fn clean() -> Result<(), anyhow::Error> {
    info!("Cleaning database");

    // let db_file = std::env::var("DB_FILE").unwrap_or("jarbas.sqlite".to_string());
    //
    // match std::fs::remove_file(db_file.clone()) {
    //     Ok(_) => {}
    //     Err(_) => {}
    // };
    //
    // std::fs::File::create(db_file)?;

    Ok(())
}
