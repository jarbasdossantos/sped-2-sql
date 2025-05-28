use crate::database::DB_POOL;
use crate::impl_display_fields;
use crate::models::schema::reg_0005::dsl as schema;
use crate::models::schema::reg_0005::table;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use serde::Serialize;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::models::schema::reg_0005)]
pub struct Reg0005 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub fantasia: Option<String>,
    pub cep: Option<String>,
    pub end: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
    pub fone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
}

#[async_trait]
impl Model for Reg0005 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg0005 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: get_field(&fields, 1),
            fantasia: get_field(&fields, 2),
            cep: get_field(&fields, 3),
            end: get_field(&fields, 4),
            num: get_field(&fields, 5),
            compl: get_field(&fields, 6),
            bairro: get_field(&fields, 7),
            fone: get_field(&fields, 8),
            fax: get_field(&fields, 9),
            email: get_field(&fields, 10),
        }
    }

    async fn get(id: i32, parent: Option<i32>) -> Result<Box<Reg0005>, Error> {
        let conn = &mut DB_POOL
            .get()
            .expect("Failed to get DB connection from pool");

        if let Some(parent_id) = parent {
            let result = table
                .filter(schema::id.eq(id))
                .filter(schema::parent_id.eq(parent_id))
                .first::<Reg0005>(conn)?;

            return Ok(Box::new(result));
        }

        Ok(Box::new(
            table.filter(schema::id.eq(id)).first::<Reg0005>(conn)?,
        ))
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(self.file_id),
                    schema::parent_id.eq(self.parent_id),
                    schema::reg.eq(self.reg.clone()),
                    schema::fantasia.eq(self.fantasia.clone()),
                    schema::cep.eq(self.cep.clone()),
                    schema::end.eq(self.end.clone()),
                    schema::num.eq(self.num.clone()),
                    schema::compl.eq(self.compl.clone()),
                    schema::bairro.eq(self.bairro.clone()),
                    schema::fone.eq(self.fone.clone()),
                    schema::fax.eq(self.fax.clone()),
                    schema::email.eq(self.email.clone()),
                ))
                .execute(&mut DB_POOL.get().unwrap())?;

            Ok(sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.get().unwrap())?)
        })
    }

    fn get_entity_name(&self) -> String {
        "Reg0005".to_string()
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg0005 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Reg0005,
    [
        reg,
        fantasia,
        cep,
        end,
        num,
        compl,
        bairro,
        fone,
        fax,
        email
    ]
);
