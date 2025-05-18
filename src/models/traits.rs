pub(crate) use crate::database::DB_POOL;
use anyhow::Result;
use async_trait::async_trait;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use std::future::Future;
use std::pin::Pin;
use diesel::{sql_query, RunQueryDsl};
use diesel::result::Error;
use crate::models::piscofins::reg_0000::Reg0000;

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
    // fn values(&self) -> IndexMap<&'static str, Option<String>>;

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
            "teste"
            // self.values()
            //     .iter()
            //     .skip(3)
            //     .map(|(_, v)| v.clone().unwrap_or_default())
            //     .collect::<Vec<_>>()
            //     .join("|")
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
        Pin<Box<dyn Future<Output=std::result::Result<Reg0000, Error>> + Send + 'a>>,
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
    type Table;
    type Id;
    type FileId;
    type ParentId;

    /// Database table name.
    fn table() -> Self::Table;

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
    fn new(fields: Vec<&str>, id: Option<Self::Id>, parent_id: Option<Self::ParentId>, file_id: Self::FileId) -> Self;

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
    async fn load(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Self>, anyhow::Error>
    where
        Self: Sized,
        Self::Table: diesel::Table,
    {
        let conn = &mut DB_POOL.get().expect("Failed to get DB connection from pool");
        let table = Self::table();

        let query = if parent_id.is_some() {
            format!(
                "SELECT * FROM {} WHERE FILE_ID = ? AND PARENT_ID = ?",
                table
            )
        } else {
            format!(
                "SELECT * FROM {} WHERE FILE_ID = ?",
                table
            )
        };

        let rows = sql_query(query).bind(file_id).bind(parent_id).load(conn).unwrap();

        let mut data = Vec::new();

        for row in rows {
            data.push(row);
        }

        // for row in rows {
        //     // Extrair os campos básicos necessários para criar uma nova instância
        //     let id: Option<i32> = row.try_get("ID").ok();
        //     let parent_id: Option<i32> = row.try_get("PARENT_ID").ok();
        //
        //     // Obter todos os campos como strings para passar para o método new
        //     let columns = row.columns();
        //     let mut field_values = Vec::new();
        //
        //     // Pular os primeiros campos administrativos (ID, FILE_ID, PARENT_ID)
        //     // e coletar os valores dos campos restantes
        //     for i in 3..columns.len() {
        //         let column = columns[i];
        //         let value: Option<String> = row.try_get(column.name()).ok();
        //         field_values.push(value.unwrap_or_default());
        //     }
        //
        //     // Converter para slice de &str
        //     let fields: Vec<&str> = field_values.iter().map(|s| s.as_str()).collect();
        //
        //     data.push(Self::new(
        //         fields,
        //         id,
        //         parent_id,
        //         file_id,
        //     ));

        Ok(data)
    }
}
