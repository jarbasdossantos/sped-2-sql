use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f210::dsl as schema;
use crate::schemas::efd_f210::table;
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
#[diesel(table_name = crate::schemas::efd_f210::dsl)]
pub struct EfdF210 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_cus_orc: Option<String>,
    pub vl_exc: Option<String>,
    pub vl_cus_orc_aju: Option<String>,
    pub vl_bc_cred: Option<String>,
    pub cst_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_cred_pis_util: Option<String>,
    pub cst_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cred_cofins_util: Option<String>,
}

#[async_trait]
impl Model for EfdF210 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF210 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_cus_orc: get_field(&fields, 2),
            vl_exc: get_field(&fields, 3),
            vl_cus_orc_aju: get_field(&fields, 4),
            vl_bc_cred: get_field(&fields, 5),
            cst_pis: get_field(&fields, 6),
            aliq_pis: get_field(&fields, 7),
            vl_cred_pis_util: get_field(&fields, 8),
            cst_cofins: get_field(&fields, 9),
            aliq_cofins: get_field(&fields, 10),
            vl_cred_cofins_util: get_field(&fields, 11),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdF210>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF210::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF210::as_select())
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
                    schema::vl_cus_orc.eq(&self.vl_cus_orc),
                    schema::vl_exc.eq(&self.vl_exc),
                    schema::vl_cus_orc_aju.eq(&self.vl_cus_orc_aju),
                    schema::vl_bc_cred.eq(&self.vl_bc_cred),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::vl_cred_pis_util.eq(&self.vl_cred_pis_util),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::vl_cred_cofins_util.eq(&self.vl_cred_cofins_util),
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
        "EfdF210".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF210 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF210, [reg, vl_cus_orc, vl_exc, vl_cus_orc_aju, vl_bc_cred, cst_pis, aliq_pis, vl_cred_pis_util, cst_cofins, aliq_cofins, vl_cred_cofins_util]);
register_model!(EfdF210, "f210");
