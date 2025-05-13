use anyhow::Result;
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

/// Trait that defines the interface for records that can be saved to the database.
///
/// This trait is implemented by structures that represent EFD (Digital Tax Bookkeeping) records
/// and provides methods for manipulating and persisting these records.
///
/// Types that implement this trait must be `Debug`, `Send`, and `Sync` to ensure
/// compatibility with asynchronous operations and logging.
pub trait Reg: std::fmt::Debug + Send + Sync {
    /// Returns an ordered map containing the field values of the record.
    ///
    /// # Returns
    ///
    /// An `IndexMap` where the keys are the field names (as static strings)
    /// and the values are optional (`Option<String>`), representing the field values.
    fn values(&self) -> IndexMap<&'static str, Option<String>>;

    /// Converts the record to a text line representation.
    ///
    /// This method formats the record values as a pipe-delimited line (|),
    /// ignoring the first three fields (id, file_id, and parent_id).
    ///
    /// # Returns
    ///
    /// A `String` containing the record's representation in line format.
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

    /// Saves the record to the SQLite database.
    ///
    /// This method is asynchronous and returns a `Future` that, when executed,
    /// inserts the record into the database.
    ///
    /// # Returns
    ///
    /// A `Future` that resolves to a `Result` containing the SQL operation result
    /// or an SQLx error if the operation fails.
    fn save<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error>> + Send + 'a>,
    >;
}

/// Trait that defines the interface for models that can be loaded and created from fields.
///
/// This trait is implemented by structures that represent EFD data models
/// and provides methods for creating new instances and loading data from the database.
///
/// The trait uses `async_trait` to allow asynchronous methods.
#[async_trait]
pub trait Model {
    /// Creates a new instance of the model from fields and metadata.
    ///
    /// # Parameters
    ///
    /// * `fields` - A vector of strings containing the record field values
    /// * `id` - Optional identifier of the record in the database
    /// * `parent_id` - Optional identifier of the parent record
    /// * `file_id` - Identifier of the file to which the record belongs
    ///
    /// # Returns
    ///
    /// A new instance of the type that implements this trait.
    fn new(fields: Vec<&str>, id: Option<i64>, parent_id: Option<i64>, file_id: i64) -> Self;
    /// Loads records from the database based on the file and parent record.
    ///
    /// This asynchronous method queries the database and returns all records
    /// that match the specified file and, optionally, the parent record.
    ///
    /// # Parameters
    ///
    /// * `file_id` - Identifier of the file whose records should be loaded
    /// * `parent_id` - Optional identifier of the parent record to filter the results
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of instances of the type that implements this trait,
    /// or an error if the operation fails.
    async fn load(file_id: i64, parent_id: Option<i64>) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized;
}
