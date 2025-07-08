use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_h010::dsl as schema;
use crate::schemas::reg_h010::table;
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
#[diesel(table_name = crate::schemas::reg_h010::dsl)]
pub struct RegH010 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_item: Option<String>,
    pub unid: Option<String>,
    pub qtd: Option<String>,
    pub vl_unit: Option<String>,
    pub vl_item: Option<String>,
    pub ind_prop: Option<String>,
    pub cod_part: Option<String>,
    pub txt_compl: Option<String>,
    pub cod_cta: Option<String>,
    pub vl_item_ir: Option<String>,
}

#[async_trait]
impl Model for RegH010 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegH010 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_item: get_field(&fields, 2),
            unid: get_field(&fields, 3),
            qtd: get_field(&fields, 4),
            vl_unit: get_field(&fields, 5),
            vl_item: get_field(&fields, 6),
            ind_prop: get_field(&fields, 7),
            cod_part: get_field(&fields, 8),
            txt_compl: get_field(&fields, 9),
            cod_cta: get_field(&fields, 10),
            vl_item_ir: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegH010>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegH010::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegH010::as_select())
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
                    schema::cod_item.eq(&self.cod_item),
                    schema::unid.eq(&self.unid),
                    schema::qtd.eq(&self.qtd),
                    schema::vl_unit.eq(&self.vl_unit),
                    schema::vl_item.eq(&self.vl_item),
                    schema::ind_prop.eq(&self.ind_prop),
                    schema::cod_part.eq(&self.cod_part),
                    schema::txt_compl.eq(&self.txt_compl),
                    schema::cod_cta.eq(&self.cod_cta),
                    schema::vl_item_ir.eq(&self.vl_item_ir),
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
        "RegH010".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegH010 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegH010, [reg, cod_item, unid, qtd, vl_unit, vl_item, ind_prop, cod_part, txt_compl, cod_cta, vl_item_ir]);
register_model!(RegH010, "h010");
