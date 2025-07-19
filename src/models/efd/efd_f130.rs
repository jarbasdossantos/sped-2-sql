use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f130::dsl as schema;
use crate::schemas::efd_f130::table;
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
#[diesel(table_name = crate::schemas::efd_f130::dsl)]
pub struct EfdF130 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub ident_bem_imob: Option<String>,
    pub ind_orig_cred: Option<String>,
    pub ind_util_bem_imob: Option<String>,
    pub mes_oper_aquis: Option<String>,
    pub vl_oper_aquis: Option<String>,
    pub parc_oper_nao_bc_cred: Option<String>,
    pub vl_bc_cred: Option<String>,
    pub ind_nr_parc: Option<String>,
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
impl Model for EfdF130 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF130 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nat_bc_cred: get_field(&fields, 2),
            ident_bem_imob: get_field(&fields, 3),
            ind_orig_cred: get_field(&fields, 4),
            ind_util_bem_imob: get_field(&fields, 5),
            mes_oper_aquis: get_field(&fields, 6),
            vl_oper_aquis: get_field(&fields, 7),
            parc_oper_nao_bc_cred: get_field(&fields, 8),
            vl_bc_cred: get_field(&fields, 9),
            ind_nr_parc: get_field(&fields, 10),
            cst_pis: get_field(&fields, 11),
            vl_bc_pis: get_field(&fields, 12),
            aliq_pis: get_field(&fields, 13),
            vl_pis: get_field(&fields, 14),
            cst_cofins: get_field(&fields, 15),
            vl_bc_cofins: get_field(&fields, 16),
            aliq_cofins: get_field(&fields, 17),
            vl_cofins: get_field(&fields, 18),
            cod_cta: get_field(&fields, 19),
            cod_ccus: get_field(&fields, 20),
            desc_bem_imob: get_field(&fields, 21),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdF130>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF130::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF130::as_select())
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
                    schema::ident_bem_imob.eq(&self.ident_bem_imob),
                    schema::ind_orig_cred.eq(&self.ind_orig_cred),
                    schema::ind_util_bem_imob.eq(&self.ind_util_bem_imob),
                    schema::mes_oper_aquis.eq(&self.mes_oper_aquis),
                    schema::vl_oper_aquis.eq(&self.vl_oper_aquis),
                    schema::parc_oper_nao_bc_cred.eq(&self.parc_oper_nao_bc_cred),
                    schema::vl_bc_cred.eq(&self.vl_bc_cred),
                    schema::ind_nr_parc.eq(&self.ind_nr_parc),
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
        "EfdF130".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF130 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF130, [reg, nat_bc_cred, ident_bem_imob, ind_orig_cred, ind_util_bem_imob, mes_oper_aquis, vl_oper_aquis, parc_oper_nao_bc_cred, vl_bc_cred, ind_nr_parc, cst_pis, vl_bc_pis, aliq_pis, vl_pis, cst_cofins, vl_bc_cofins, aliq_cofins, vl_cofins, cod_cta, cod_ccus, desc_bem_imob]);
register_model!(EfdF130, "f130");
