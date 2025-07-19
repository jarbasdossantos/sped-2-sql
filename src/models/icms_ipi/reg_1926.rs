use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1926::dsl as schema;
use crate::schemas::reg_1926::table;
use crate::{impl_display_fields, register_model};
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use diesel::{QueryDsl, SelectableHelper};
use serde::{Serialize, Deserialize};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use diesel::sqlite::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::reg_1926::dsl)]
pub struct Reg1926 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_or: Option<String>,
    pub vl_or: Option<String>,
    pub dt_vcto: Option<String>,
    pub cod_rec: Option<String>,
    pub num_proc: Option<String>,
    pub ind_proc: Option<String>,
    pub proc_: Option<String>,
    pub txt_compl: Option<String>,
    pub mes_ref: Option<String>,
}

#[async_trait]
impl Model for Reg1926 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1926 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_or: get_field(&fields, 2),
            vl_or: get_field(&fields, 3),
            dt_vcto: get_field(&fields, 4),
            cod_rec: get_field(&fields, 5),
            num_proc: get_field(&fields, 6),
            ind_proc: get_field(&fields, 7),
            proc_: get_field(&fields, 8),
            txt_compl: get_field(&fields, 9),
            mes_ref: get_field(&fields, 10),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<Reg1926>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1926::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1926::as_select())
                .load(conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = DB_POOL.lock().await.get().unwrap();

            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::cod_or.eq(&self.cod_or),
                    schema::vl_or.eq(&self.vl_or),
                    schema::dt_vcto.eq(&self.dt_vcto),
                    schema::cod_rec.eq(&self.cod_rec),
                    schema::num_proc.eq(&self.num_proc),
                    schema::ind_proc.eq(&self.ind_proc),
                    schema::proc_.eq(&self.proc_),
                    schema::txt_compl.eq(&self.txt_compl),
                    schema::mes_ref.eq(&self.mes_ref),
                ))
                .execute(&mut conn)?;

            Ok(sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?)
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Reg1926".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1926 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1926, [reg, cod_or, vl_or, dt_vcto, cod_rec, num_proc, ind_proc, proc_, txt_compl, mes_ref]);
register_model!(Reg1926, "1926");
