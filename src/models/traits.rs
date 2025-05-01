use anyhow::Result;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

pub trait Reg: std::fmt::Debug + Send + Sync {
    /*
     * Convert the register to a line of the file
     *
     * @return String - The line as a string
     */
    #[allow(dead_code)]
    fn to_line(&self) -> String;

    fn values(&self) -> IndexMap<&'static str, Option<String>>;

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    >;
}

pub trait Model {
    fn new(fields: Vec<&str>, parent_id: Option<i64>, file_id: i64) -> Self;
    fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized;
}
