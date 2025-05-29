use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_d180::reg_d180::dsl as schema;
use crate::schemas::reg_d180::reg_d180::table;
use crate::{impl_display_fields, register_model};
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use diesel::{QueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::reg_d180::reg_d180::dsl)]
pub struct RegD180 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub num_seq: Option<String>,
    pub ind_emit: Option<String>,
    pub cnpj_cpf_emit: Option<String>,
    pub uf_emit: Option<String>,
    pub ie_emit: Option<String>,
    pub cod_mun_orig: Option<String>,
    pub cnpj_cpf_tom: Option<String>,
    pub uf_tom: Option<String>,
    pub ie_tom: Option<String>,
    pub cod_mun_dest: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_doc: Option<String>,
}

#[async_trait]
impl Model for RegD180 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegD180 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            num_seq: get_field(&fields, 2),
            ind_emit: get_field(&fields, 3),
            cnpj_cpf_emit: get_field(&fields, 4),
            uf_emit: get_field(&fields, 5),
            ie_emit: get_field(&fields, 6),
            cod_mun_orig: get_field(&fields, 7),
            cnpj_cpf_tom: get_field(&fields, 8),
            uf_tom: get_field(&fields, 9),
            ie_tom: get_field(&fields, 10),
            cod_mun_dest: get_field(&fields, 11),
            cod_mod: get_field(&fields, 12),
            ser: get_field(&fields, 13),
            sub: get_field(&fields, 14),
            num_doc: get_field(&fields, 15),
            dt_doc: get_field(&fields, 16),
            vl_doc: get_field(&fields, 17),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegD180>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegD180::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegD180::as_select())
                .load(&mut conn)?)
        }
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::num_seq.eq(&self.num_seq),
                    schema::ind_emit.eq(&self.ind_emit),
                    schema::cnpj_cpf_emit.eq(&self.cnpj_cpf_emit),
                    schema::uf_emit.eq(&self.uf_emit),
                    schema::ie_emit.eq(&self.ie_emit),
                    schema::cod_mun_orig.eq(&self.cod_mun_orig),
                    schema::cnpj_cpf_tom.eq(&self.cnpj_cpf_tom),
                    schema::uf_tom.eq(&self.uf_tom),
                    schema::ie_tom.eq(&self.ie_tom),
                    schema::cod_mun_dest.eq(&self.cod_mun_dest),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_doc.eq(&self.vl_doc),
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
        "RegD180".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegD180 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    RegD180,
    [
        reg,
        num_seq,
        ind_emit,
        cnpj_cpf_emit,
        uf_emit,
        ie_emit,
        cod_mun_orig,
        cnpj_cpf_tom,
        uf_tom,
        ie_tom,
        cod_mun_dest,
        cod_mod,
        ser,
        sub,
        num_doc,
        dt_doc,
        vl_doc
    ]
);
register_model!(RegD180, "d180");
