use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f120::efd_f120::dsl as schema;
use crate::schemas::efd_f120::efd_f120::table;
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
#[diesel(table_name = crate::schemas::efd_f120::efd_f120::dsl)]
pub struct EfdF120 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub ident_bem_imob: Option<String>,
    pub ind_orig_cred: Option<String>,
    pub ind_util_bem_imob: Option<String>,
    pub vl_oper_dep: Option<String>,
    pub parc_oper_nao_bc_cred: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
    pub cod_ccus: Option<String>,
    pub desc_bem_imob: Option<String>,
}

#[async_trait]
impl Model for EfdF120 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF120 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_bc_cred: get_field(&fields, 2),
            ident_bem_imob: get_field(&fields, 3),
            ind_orig_cred: get_field(&fields, 4),
            ind_util_bem_imob: get_field(&fields, 5),
            vl_oper_dep: get_field(&fields, 6),
            parc_oper_nao_bc_cred: get_field(&fields, 7),
            cst_pis: get_field(&fields, 8),
            vl_bc_pis: get_field(&fields, 9),
            aliq_pis: get_field(&fields, 10),
            vl_pis: get_field(&fields, 11),
            cst_cofins: get_field(&fields, 12),
            vl_bc_cofins: get_field(&fields, 13),
            aliq_cofins: get_field(&fields, 14),
            vl_cofins: get_field(&fields, 15),
            cod_cta: get_field(&fields, 16),
            cod_ccus: get_field(&fields, 17),
            desc_bem_imob: get_field(&fields, 18),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF120>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdF120::as_select())
            .load(&mut DB_POOL
                .get().unwrap())?)
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::nat_bc_cred.eq(&self.nat_bc_cred),
                    schema::ident_bem_imob.eq(&self.ident_bem_imob),
                    schema::ind_orig_cred.eq(&self.ind_orig_cred),
                    schema::ind_util_bem_imob.eq(&self.ind_util_bem_imob),
                    schema::vl_oper_dep.eq(&self.vl_oper_dep),
                    schema::parc_oper_nao_bc_cred.eq(&self.parc_oper_nao_bc_cred),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::cod_cta.eq(&self.cod_cta),
                    schema::cod_ccus.eq(&self.cod_ccus),
                    schema::desc_bem_imob.eq(&self.desc_bem_imob),
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
        "EfdF120".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF120 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF120, [reg, nat_bc_cred, ident_bem_imob, ind_orig_cred, ind_util_bem_imob, vl_oper_dep, parc_oper_nao_bc_cred, cst_pis, vl_bc_pis, aliq_pis, vl_pis, cst_cofins, vl_bc_cofins, aliq_cofins, vl_cofins, cod_cta, cod_ccus, desc_bem_imob]);
register_model!(EfdF120, "f120");
