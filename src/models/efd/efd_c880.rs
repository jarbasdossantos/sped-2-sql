use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c880::dsl as schema;
use crate::schemas::efd_c880::table;
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
#[diesel(table_name = crate::schemas::efd_c880::dsl)]
pub struct EfdC880 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_item: Option<String>,
    pub cfop: Option<String>,
    pub vl_item: Option<String>,
    pub vl_desc: Option<String>,
    pub cst_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub aliq_cofins_quant: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdC880 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC880 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_item: get_field(&fields, 2),
            cfop: get_field(&fields, 3),
            vl_item: get_field(&fields, 4),
            vl_desc: get_field(&fields, 5),
            cst_pis: get_field(&fields, 6),
            quant_bc_pis: get_field(&fields, 7),
            aliq_pis_quant: get_field(&fields, 8),
            vl_pis: get_field(&fields, 9),
            cst_cofins: get_field(&fields, 10),
            quant_bc_cofins: get_field(&fields, 11),
            aliq_cofins_quant: get_field(&fields, 12),
            vl_cofins: get_field(&fields, 13),
            cod_cta: get_field(&fields, 14),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<EfdC880>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdC880::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdC880::as_select())
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
                    schema::cod_item.eq(&self.cod_item),
                    schema::cfop.eq(&self.cfop),
                    schema::vl_item.eq(&self.vl_item),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::quant_bc_pis.eq(&self.quant_bc_pis),
                    schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
                    schema::aliq_cofins_quant.eq(&self.aliq_cofins_quant),
                    schema::vl_cofins.eq(&self.vl_cofins),
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
        "EfdC880".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC880 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC880, [reg, cod_item, cfop, vl_item, vl_desc, cst_pis, quant_bc_pis, aliq_pis_quant, vl_pis, cst_cofins, quant_bc_cofins, aliq_cofins_quant, vl_cofins, cod_cta]);
register_model!(EfdC880, "c880");
