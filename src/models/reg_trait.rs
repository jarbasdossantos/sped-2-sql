use std::future::Future;
use std::pin::Pin;
use sqlx::sqlite::SqlitePool;

pub trait Reg: std::fmt::Debug + Send + Sync {
    /*
     * Convert the register to a line of the file
     *
     * @return String - The line as a string
     */
    #[allow(dead_code)]
    fn to_line(&self) -> String;

    fn to_db<'a>(
        &'a self,
        conn: &'a SqlitePool,
    ) -> Pin<Box<dyn Future<Output=Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>>;
}
