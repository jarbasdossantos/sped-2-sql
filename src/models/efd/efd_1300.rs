use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_1300::dsl as schema;
use crate::schemas::efd_1300::table;
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
#[diesel(table_name = crate::schemas::efd_1300::dsl)]
pub struct Efd1300 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_nat_ret: Option<String>,
    pub pr_rec_ret: Option<String>,
    pub vl_ret_apu: Option<String>,
    pub vl_ret_ded: Option<String>,
    pub vl_ret_per: Option<String>,
    pub vl_ret_dcomp: Option<String>,
    pub sld_ret: Option<String>,
}

#[async_trait]
impl Model for Efd1300 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd1300 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_nat_ret: get_field(&fields, 2),
            pr_rec_ret: get_field(&fields, 3),
            vl_ret_apu: get_field(&fields, 4),
            vl_ret_ded: get_field(&fields, 5),
            vl_ret_per: get_field(&fields, 6),
            vl_ret_dcomp: get_field(&fields, 7),
            sld_ret: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd1300>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd1300::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd1300::as_select())
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
                    schema::ind_nat_ret.eq(&self.ind_nat_ret),
                    schema::pr_rec_ret.eq(&self.pr_rec_ret),
                    schema::vl_ret_apu.eq(&self.vl_ret_apu),
                    schema::vl_ret_ded.eq(&self.vl_ret_ded),
                    schema::vl_ret_per.eq(&self.vl_ret_per),
                    schema::vl_ret_dcomp.eq(&self.vl_ret_dcomp),
                    schema::sld_ret.eq(&self.sld_ret),
                ))
                .execute(&mut DB_POOL.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Efd1300".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd1300 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd1300, [reg, ind_nat_ret, pr_rec_ret, vl_ret_apu, vl_ret_ded, vl_ret_per, vl_ret_dcomp, sld_ret]);
register_model!(Efd1300, "1300");
