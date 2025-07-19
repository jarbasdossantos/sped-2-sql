use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d161::dsl as schema;
use crate::schemas::reg_d161::table;
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
#[diesel(table_name = crate::schemas::reg_d161::dsl)]
pub struct RegD161 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_carga: Option<String>,
    pub cnpj_cpf_col: Option<String>,
    pub ie_col: Option<String>,
    pub cod_mun_col: Option<String>,
    pub cnpj_cpf_entg: Option<String>,
    pub ie_entg: Option<String>,
    pub cod_mun_entg: Option<String>,
}

#[async_trait]
impl Model for RegD161 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD161 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_carga: get_field(&fields, 2),
            cnpj_cpf_col: get_field(&fields, 3),
            ie_col: get_field(&fields, 4),
            cod_mun_col: get_field(&fields, 5),
            cnpj_cpf_entg: get_field(&fields, 6),
            ie_entg: get_field(&fields, 7),
            cod_mun_entg: get_field(&fields, 8),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegD161>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD161::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD161::as_select())
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
                    schema::ind_carga.eq(&self.ind_carga),
                    schema::cnpj_cpf_col.eq(&self.cnpj_cpf_col),
                    schema::ie_col.eq(&self.ie_col),
                    schema::cod_mun_col.eq(&self.cod_mun_col),
                    schema::cnpj_cpf_entg.eq(&self.cnpj_cpf_entg),
                    schema::ie_entg.eq(&self.ie_entg),
                    schema::cod_mun_entg.eq(&self.cod_mun_entg),
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
        "RegD161".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD161 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegD161, [reg, ind_carga, cnpj_cpf_col, ie_col, cod_mun_col, cnpj_cpf_entg, ie_entg, cod_mun_entg]);
register_model!(RegD161, "d161");
