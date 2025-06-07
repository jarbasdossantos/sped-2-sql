use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f800::dsl as schema;
use crate::schemas::efd_f800::table;
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
#[diesel(table_name = crate::schemas::efd_f800::dsl)]
pub struct EfdF800 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_nat_even: Option<String>,
    pub dt_even: Option<String>,
    pub cnpj_suced: Option<String>,
    pub pa_cont_cred: Option<String>,
    pub cod_cred: Option<String>,
    pub vl_cred_pis: Option<String>,
    pub vl_cred_cofins: Option<String>,
    pub per_cred_cis: Option<String>,
}

#[async_trait]
impl Model for EfdF800 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF800 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_nat_even: get_field(&fields, 2),
            dt_even: get_field(&fields, 3),
            cnpj_suced: get_field(&fields, 4),
            pa_cont_cred: get_field(&fields, 5),
            cod_cred: get_field(&fields, 6),
            vl_cred_pis: get_field(&fields, 7),
            vl_cred_cofins: get_field(&fields, 8),
            per_cred_cis: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF800>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF800::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF800::as_select())
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
                    schema::ind_nat_even.eq(&self.ind_nat_even),
                    schema::dt_even.eq(&self.dt_even),
                    schema::cnpj_suced.eq(&self.cnpj_suced),
                    schema::pa_cont_cred.eq(&self.pa_cont_cred),
                    schema::cod_cred.eq(&self.cod_cred),
                    schema::vl_cred_pis.eq(&self.vl_cred_pis),
                    schema::vl_cred_cofins.eq(&self.vl_cred_cofins),
                    schema::per_cred_cis.eq(&self.per_cred_cis),
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
        "EfdF800".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF800 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF800, [reg, ind_nat_even, dt_even, cnpj_suced, pa_cont_cred, cod_cred, vl_cred_pis, vl_cred_cofins, per_cred_cis]);
register_model!(EfdF800, "f800");
