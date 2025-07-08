use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c495::dsl as schema;
use crate::schemas::reg_c495::table;
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
#[diesel(table_name = crate::schemas::reg_c495::dsl)]
pub struct RegC495 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub aliq_icms: Option<String>,
    pub cod_item: Option<String>,
    pub qtd: Option<String>,
    pub qtd_canc: Option<String>,
    pub unid: Option<String>,
    pub vl_item: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_canc: Option<String>,
    pub vl_acmo: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_isen: Option<String>,
    pub vl_nt: Option<String>,
    pub vl_icms_st: Option<String>,
}

#[async_trait]
impl Model for RegC495 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC495 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            aliq_icms: get_field(&fields, 2),
            cod_item: get_field(&fields, 3),
            qtd: get_field(&fields, 4),
            qtd_canc: get_field(&fields, 5),
            unid: get_field(&fields, 6),
            vl_item: get_field(&fields, 7),
            vl_desc: get_field(&fields, 8),
            vl_canc: get_field(&fields, 9),
            vl_acmo: get_field(&fields, 10),
            vl_bc_icms: get_field(&fields, 11),
            vl_icms: get_field(&fields, 12),
            vl_isen: get_field(&fields, 13),
            vl_nt: get_field(&fields, 14),
            vl_icms_st: get_field(&fields, 15),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC495>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC495::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC495::as_select())
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
                    schema::aliq_icms.eq(&self.aliq_icms),
                    schema::cod_item.eq(&self.cod_item),
                    schema::qtd.eq(&self.qtd),
                    schema::qtd_canc.eq(&self.qtd_canc),
                    schema::unid.eq(&self.unid),
                    schema::vl_item.eq(&self.vl_item),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_canc.eq(&self.vl_canc),
                    schema::vl_acmo.eq(&self.vl_acmo),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_isen.eq(&self.vl_isen),
                    schema::vl_nt.eq(&self.vl_nt),
                    schema::vl_icms_st.eq(&self.vl_icms_st),
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
        "RegC495".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC495 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC495, [reg, aliq_icms, cod_item, qtd, qtd_canc, unid, vl_item, vl_desc, vl_canc, vl_acmo, vl_bc_icms, vl_icms, vl_isen, vl_nt, vl_icms_st]);
register_model!(RegC495, "c495");
