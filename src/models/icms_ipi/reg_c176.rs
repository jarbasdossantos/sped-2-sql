use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c176::dsl as schema;
use crate::schemas::reg_c176::table;
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::reg_c176::dsl)]
pub struct RegC176 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod_ult_e: Option<String>,
    pub num_doc_ult_e: Option<String>,
    pub ser_ult_e: Option<String>,
    pub dt_ult_e: Option<String>,
    pub cod_part_ult_e: Option<String>,
    pub quant_ult_e: Option<String>,
    pub vl_unit_ult_e: Option<String>,
    pub vl_unit_bc_st: Option<String>,
}

#[async_trait]
impl Model for RegC176 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC176 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod_ult_e: get_field(&fields, 2),
            num_doc_ult_e: get_field(&fields, 3),
            ser_ult_e: get_field(&fields, 4),
            dt_ult_e: get_field(&fields, 5),
            cod_part_ult_e: get_field(&fields, 6),
            quant_ult_e: get_field(&fields, 7),
            vl_unit_ult_e: get_field(&fields, 8),
            vl_unit_bc_st: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC176>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC176::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC176::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::cod_mod_ult_e.eq(&self.cod_mod_ult_e),
                    schema::num_doc_ult_e.eq(&self.num_doc_ult_e),
                    schema::ser_ult_e.eq(&self.ser_ult_e),
                    schema::dt_ult_e.eq(&self.dt_ult_e),
                    schema::cod_part_ult_e.eq(&self.cod_part_ult_e),
                    schema::quant_ult_e.eq(&self.quant_ult_e),
                    schema::vl_unit_ult_e.eq(&self.vl_unit_ult_e),
                    schema::vl_unit_bc_st.eq(&self.vl_unit_bc_st),
                ))
                .execute(&mut DB_POOL.lock().await.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.lock().await.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "RegC176".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC176 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC176, [reg, cod_mod_ult_e, num_doc_ult_e, ser_ult_e, dt_ult_e, cod_part_ult_e, quant_ult_e, vl_unit_ult_e, vl_unit_bc_st]);
register_model!(RegC176, "c176");
