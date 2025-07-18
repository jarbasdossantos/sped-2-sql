use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f150::dsl as schema;
use crate::schemas::efd_f150::table;
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
#[diesel(table_name = crate::schemas::efd_f150::dsl)]
pub struct EfdF150 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub vl_tot_est: Option<String>,
    pub est_imp: Option<String>,
    pub vl_bc_est: Option<String>,
    pub vl_bc_men_est: Option<String>,
    pub cst_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_cred_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cred_cofins: Option<String>,
    pub desc_est: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdF150 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF150 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_bc_cred: get_field(&fields, 2),
            vl_tot_est: get_field(&fields, 3),
            est_imp: get_field(&fields, 4),
            vl_bc_est: get_field(&fields, 5),
            vl_bc_men_est: get_field(&fields, 6),
            cst_pis: get_field(&fields, 7),
            aliq_pis: get_field(&fields, 8),
            vl_cred_pis: get_field(&fields, 9),
            cst_cofins: get_field(&fields, 10),
            aliq_cofins: get_field(&fields, 11),
            vl_cred_cofins: get_field(&fields, 12),
            desc_est: get_field(&fields, 13),
            cod_cta: get_field(&fields, 14),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdF150>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF150::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF150::as_select())
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
                    schema::nat_bc_cred.eq(&self.nat_bc_cred),
                    schema::vl_tot_est.eq(&self.vl_tot_est),
                    schema::est_imp.eq(&self.est_imp),
                    schema::vl_bc_est.eq(&self.vl_bc_est),
                    schema::vl_bc_men_est.eq(&self.vl_bc_men_est),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::vl_cred_pis.eq(&self.vl_cred_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::vl_cred_cofins.eq(&self.vl_cred_cofins),
                    schema::desc_est.eq(&self.desc_est),
                    schema::cod_cta.eq(&self.cod_cta),
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
        "EfdF150".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF150 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF150, [reg, nat_bc_cred, vl_tot_est, est_imp, vl_bc_est, vl_bc_men_est, cst_pis, aliq_pis, vl_cred_pis, cst_cofins, aliq_cofins, vl_cred_cofins, desc_est, cod_cta]);
register_model!(EfdF150, "f150");
