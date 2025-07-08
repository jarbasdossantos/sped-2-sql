use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c350::dsl as schema;
use crate::schemas::reg_c350::table;
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
#[diesel(table_name = crate::schemas::reg_c350::dsl)]
pub struct RegC350 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ser: Option<String>,
    pub sub_ser: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub cnpj_cpf: Option<String>,
    pub vl_merc: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofis: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for RegC350 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC350 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ser: get_field(&fields, 2),
            sub_ser: get_field(&fields, 3),
            num_doc: get_field(&fields, 4),
            dt_doc: get_field(&fields, 5),
            cnpj_cpf: get_field(&fields, 6),
            vl_merc: get_field(&fields, 7),
            vl_doc: get_field(&fields, 8),
            vl_desc: get_field(&fields, 9),
            vl_pis: get_field(&fields, 10),
            vl_cofis: get_field(&fields, 11),
            cod_cta: get_field(&fields, 12),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC350>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC350::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC350::as_select())
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
                    schema::ser.eq(&self.ser),
                    schema::sub_ser.eq(&self.sub_ser),
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::cnpj_cpf.eq(&self.cnpj_cpf),
                    schema::vl_merc.eq(&self.vl_merc),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofis.eq(&self.vl_cofis),
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
        "RegC350".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC350 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC350, [reg, ser, sub_ser, num_doc, dt_doc, cnpj_cpf, vl_merc, vl_doc, vl_desc, vl_pis, vl_cofis, cod_cta]);
register_model!(RegC350, "c350");
