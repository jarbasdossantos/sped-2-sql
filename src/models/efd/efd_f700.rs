use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f700::dsl as schema;
use crate::schemas::efd_f700::table;
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
#[diesel(table_name = crate::schemas::efd_f700::dsl)]
pub struct EfdF700 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_ori_ded: Option<String>,
    pub ind_nat_ded: Option<String>,
    pub vl_ded_pis: Option<String>,
    pub vl_ded_cofins: Option<String>,
    pub vl_bc_oper: Option<String>,
    pub cnpj: Option<String>,
    pub inf_comp: Option<String>,
}

#[async_trait]
impl Model for EfdF700 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF700 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_ori_ded: get_field(&fields, 2),
            ind_nat_ded: get_field(&fields, 3),
            vl_ded_pis: get_field(&fields, 4),
            vl_ded_cofins: get_field(&fields, 5),
            vl_bc_oper: get_field(&fields, 6),
            cnpj: get_field(&fields, 7),
            inf_comp: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF700>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF700::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF700::as_select())
                .load(&mut conn)?)
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
                    schema::ind_ori_ded.eq(&self.ind_ori_ded),
                    schema::ind_nat_ded.eq(&self.ind_nat_ded),
                    schema::vl_ded_pis.eq(&self.vl_ded_pis),
                    schema::vl_ded_cofins.eq(&self.vl_ded_cofins),
                    schema::vl_bc_oper.eq(&self.vl_bc_oper),
                    schema::cnpj.eq(&self.cnpj),
                    schema::inf_comp.eq(&self.inf_comp),
                ))
                .execute(&mut conn)?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "EfdF700".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF700 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF700, [reg, ind_ori_ded, ind_nat_ded, vl_ded_pis, vl_ded_cofins, vl_bc_oper, cnpj, inf_comp]);
register_model!(EfdF700, "f700");
