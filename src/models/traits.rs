use anyhow::Result;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

pub trait Reg: std::fmt::Debug + Send + Sync {
    fn values(&self) -> IndexMap<&'static str, Option<String>>;

    #[allow(dead_code)]
    fn to_line(&self) -> String {
        format!(
            "|{}|",
            self.values()
                .iter()
                .skip(3)
                .map(|(_, v)| v.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    >;
}

#[async_trait]
pub trait Model {
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self;
    async fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized;
}
