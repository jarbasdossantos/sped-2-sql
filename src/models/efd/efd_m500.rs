use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_m500::dsl as schema;
use crate::schemas::efd_m500::table;
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
#[diesel(table_name = crate::schemas::efd_m500::dsl)]
pub struct EfdM500 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_cred: Option<String>,
    pub ind_cred_ori: Option<String>,
    pub vl_bc_cred: Option<String>,
    pub aliq_cofins: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub aliq_cofins_quant: Option<String>,
    pub vl_cred: Option<String>,
    pub vl_ajus_acres: Option<String>,
    pub vl_ajus_reduc: Option<String>,
    pub vl_cred_dif: Option<String>,
    pub vl_cred_disp: Option<String>,
    pub ind_desc_cred: Option<String>,
    pub vl_cred_desc: Option<String>,
    pub sld_cred: Option<String>,
}

#[async_trait]
impl Model for EfdM500 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdM500 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_cred: get_field(&fields, 2),
            ind_cred_ori: get_field(&fields, 3),
            vl_bc_cred: get_field(&fields, 4),
            aliq_cofins: get_field(&fields, 5),
            quant_bc_cofins: get_field(&fields, 6),
            aliq_cofins_quant: get_field(&fields, 7),
            vl_cred: get_field(&fields, 8),
            vl_ajus_acres: get_field(&fields, 9),
            vl_ajus_reduc: get_field(&fields, 10),
            vl_cred_dif: get_field(&fields, 11),
            vl_cred_disp: get_field(&fields, 12),
            ind_desc_cred: get_field(&fields, 13),
            vl_cred_desc: get_field(&fields, 14),
            sld_cred: get_field(&fields, 15),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdM500>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdM500::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdM500::as_select())
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
                    schema::cod_cred.eq(&self.cod_cred),
                    schema::ind_cred_ori.eq(&self.ind_cred_ori),
                    schema::vl_bc_cred.eq(&self.vl_bc_cred),
                    schema::aliq_cofins.eq(&self.aliq_cofins),
                    schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
                    schema::aliq_cofins_quant.eq(&self.aliq_cofins_quant),
                    schema::vl_cred.eq(&self.vl_cred),
                    schema::vl_ajus_acres.eq(&self.vl_ajus_acres),
                    schema::vl_ajus_reduc.eq(&self.vl_ajus_reduc),
                    schema::vl_cred_dif.eq(&self.vl_cred_dif),
                    schema::vl_cred_disp.eq(&self.vl_cred_disp),
                    schema::ind_desc_cred.eq(&self.ind_desc_cred),
                    schema::vl_cred_desc.eq(&self.vl_cred_desc),
                    schema::sld_cred.eq(&self.sld_cred),
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
        "EfdM500".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdM500 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdM500, [reg, cod_cred, ind_cred_ori, vl_bc_cred, aliq_cofins, quant_bc_cofins, aliq_cofins_quant, vl_cred, vl_ajus_acres, vl_ajus_reduc, vl_cred_dif, vl_cred_disp, ind_desc_cred, vl_cred_desc, sld_cred]);
register_model!(EfdM500, "m500");
