use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d160::dsl as schema;
use crate::schemas::reg_d160::table;
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
#[diesel(table_name = crate::schemas::reg_d160::dsl)]
pub struct RegD160 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub despacho: Option<String>,
    pub cnpj_cpf_rem: Option<String>,
    pub ie_rem: Option<String>,
    pub cod_mun_ori: Option<String>,
    pub cnpj_cpf_dest: Option<String>,
    pub ie_dest: Option<String>,
    pub cod_mun_dest: Option<String>,
}

#[async_trait]
impl Model for RegD160 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD160 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            despacho: get_field(&fields, 2),
            cnpj_cpf_rem: get_field(&fields, 3),
            ie_rem: get_field(&fields, 4),
            cod_mun_ori: get_field(&fields, 5),
            cnpj_cpf_dest: get_field(&fields, 6),
            ie_dest: get_field(&fields, 7),
            cod_mun_dest: get_field(&fields, 8),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD160>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD160::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD160::as_select())
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
                    schema::despacho.eq(&self.despacho),
                    schema::cnpj_cpf_rem.eq(&self.cnpj_cpf_rem),
                    schema::ie_rem.eq(&self.ie_rem),
                    schema::cod_mun_ori.eq(&self.cod_mun_ori),
                    schema::cnpj_cpf_dest.eq(&self.cnpj_cpf_dest),
                    schema::ie_dest.eq(&self.ie_dest),
                    schema::cod_mun_dest.eq(&self.cod_mun_dest),
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
        "RegD160".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD160, [reg, despacho, cnpj_cpf_rem, ie_rem, cod_mun_ori, cnpj_cpf_dest, ie_dest, cod_mun_dest]);
register_model!(RegD160, "d160");
