use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c510::dsl as schema;
use crate::schemas::reg_c510::table;
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
#[diesel(table_name = crate::schemas::reg_c510::dsl)]
pub struct RegC510 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub num_item: Option<String>,
    pub cod_item: Option<String>,
    pub cod_class: Option<String>,
    pub qtd: Option<String>,
    pub unid: Option<String>,
    pub vl_item: Option<String>,
    pub vl_desc: Option<String>,
    pub cst_icms: Option<String>,
    pub cfop: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub aliq_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub aliq_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub ind_rec: Option<String>,
    pub cod_part: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for RegC510 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC510 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            num_item: get_field(&fields, 2),
            cod_item: get_field(&fields, 3),
            cod_class: get_field(&fields, 4),
            qtd: get_field(&fields, 5),
            unid: get_field(&fields, 6),
            vl_item: get_field(&fields, 7),
            vl_desc: get_field(&fields, 8),
            cst_icms: get_field(&fields, 9),
            cfop: get_field(&fields, 10),
            vl_bc_icms: get_field(&fields, 11),
            aliq_icms: get_field(&fields, 12),
            vl_icms: get_field(&fields, 13),
            vl_bc_icms_st: get_field(&fields, 14),
            aliq_st: get_field(&fields, 15),
            vl_icms_st: get_field(&fields, 16),
            ind_rec: get_field(&fields, 17),
            cod_part: get_field(&fields, 18),
            vl_pis: get_field(&fields, 19),
            vl_cofins: get_field(&fields, 20),
            cod_cta: get_field(&fields, 21),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC510>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC510::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC510::as_select())
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
                    schema::num_item.eq(&self.num_item),
                    schema::cod_item.eq(&self.cod_item),
                    schema::cod_class.eq(&self.cod_class),
                    schema::qtd.eq(&self.qtd),
                    schema::unid.eq(&self.unid),
                    schema::vl_item.eq(&self.vl_item),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::cst_icms.eq(&self.cst_icms),
                    schema::cfop.eq(&self.cfop),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::aliq_icms.eq(&self.aliq_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_bc_icms_st.eq(&self.vl_bc_icms_st),
                    schema::aliq_st.eq(&self.aliq_st),
                    schema::vl_icms_st.eq(&self.vl_icms_st),
                    schema::ind_rec.eq(&self.ind_rec),
                    schema::cod_part.eq(&self.cod_part),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::cod_cta.eq(&self.cod_cta),
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
        "RegC510".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC510 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC510, [reg, num_item, cod_item, cod_class, qtd, unid, vl_item, vl_desc, cst_icms, cfop, vl_bc_icms, aliq_icms, vl_icms, vl_bc_icms_st, aliq_st, vl_icms_st, ind_rec, cod_part, vl_pis, vl_cofins, cod_cta]);
register_model!(RegC510, "c510");
