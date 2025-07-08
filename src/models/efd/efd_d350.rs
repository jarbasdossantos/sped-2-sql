use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_d350::dsl as schema;
use crate::schemas::efd_d350::table;
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
#[diesel(table_name = crate::schemas::efd_d350::dsl)]
pub struct EfdD350 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub ecf_mod: Option<String>,
    pub ecf_fab: Option<String>,
    pub dt_doc: Option<String>,
    pub cro: Option<String>,
    pub crz: Option<String>,
    pub num_coo_fin: Option<String>,
    pub gt_fin: Option<String>,
    pub vl_brt: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub aliq_cofins_quant: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdD350 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdD350 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            ecf_mod: get_field(&fields, 3),
            ecf_fab: get_field(&fields, 4),
            dt_doc: get_field(&fields, 5),
            cro: get_field(&fields, 6),
            crz: get_field(&fields, 7),
            num_coo_fin: get_field(&fields, 8),
            gt_fin: get_field(&fields, 9),
            vl_brt: get_field(&fields, 10),
            cst_pis: get_field(&fields, 11),
            vl_bc_pis: get_field(&fields, 12),
            aliq_pis: get_field(&fields, 13),
            quant_bc_pis: get_field(&fields, 14),
            aliq_pis_quant: get_field(&fields, 15),
            vl_pis: get_field(&fields, 16),
            cst_cofins: get_field(&fields, 17),
            vl_bc_cofins: get_field(&fields, 18),
            aliq_cofins: get_field(&fields, 19),
            quant_bc_cofins: get_field(&fields, 20),
            aliq_cofins_quant: get_field(&fields, 21),
            vl_cofins: get_field(&fields, 22),
            cod_cta: get_field(&fields, 23),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdD350>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdD350::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdD350::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ecf_mod.eq(&self.ecf_mod),
                    schema::ecf_fab.eq(&self.ecf_fab),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::cro.eq(&self.cro),
                    schema::crz.eq(&self.crz),
                    schema::num_coo_fin.eq(&self.num_coo_fin),
                    schema::gt_fin.eq(&self.gt_fin),
                    schema::vl_brt.eq(&self.vl_brt),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::aliq_pis.eq(&self.aliq_pis),
                    schema::quant_bc_pis.eq(&self.quant_bc_pis),
                    schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
                    schema::aliq_cofins_quant.eq(&self.aliq_cofins_quant),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::cod_cta.eq(&self.cod_cta),
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
        "EfdD350".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdD350 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdD350, [reg, cod_mod, ecf_mod, ecf_fab, dt_doc, cro, crz, num_coo_fin, gt_fin, vl_brt, cst_pis, vl_bc_pis, aliq_pis, quant_bc_pis, aliq_pis_quant, vl_pis, cst_cofins, vl_bc_cofins, aliq_cofins, quant_bc_cofins, aliq_cofins_quant, vl_cofins, cod_cta]);
register_model!(EfdD350, "d350");
