use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_0000::dsl as schema;
use crate::schemas::efd_0000::table;
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
#[diesel(table_name = crate::schemas::efd_0000::dsl)]
pub struct Efd0000 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_ver: Option<String>,
    pub tipo_escrit: Option<String>,
    pub ind_sit_esp: Option<String>,
    pub num_rec_anterior: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fin: Option<String>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub uf: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub ind_nat_pj: Option<String>,
    pub ind_ativ: Option<String>,
}

#[async_trait]
impl Model for Efd0000 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd0000 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_ver: get_field(&fields, 2),
            tipo_escrit: get_field(&fields, 3),
            ind_sit_esp: get_field(&fields, 4),
            num_rec_anterior: get_field(&fields, 5),
            dt_ini: get_field(&fields, 6),
            dt_fin: get_field(&fields, 7),
            nome: get_field(&fields, 8),
            cnpj: get_field(&fields, 9),
            uf: get_field(&fields, 10),
            cod_mun: get_field(&fields, 11),
            suframa: get_field(&fields, 12),
            ind_nat_pj: get_field(&fields, 13),
            ind_ativ: get_field(&fields, 14),
        }
    }

    async fn get(file_id: i32, _parent_id: Option<i32>) -> Result<Vec<Efd0000>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .select(Efd0000::as_select())
            .load(&mut conn)?)
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
                    schema::cod_ver.eq(&self.cod_ver),
                    schema::tipo_escrit.eq(&self.tipo_escrit),
                    schema::ind_sit_esp.eq(&self.ind_sit_esp),
                    schema::num_rec_anterior.eq(&self.num_rec_anterior),
                    schema::dt_ini.eq(&self.dt_ini),
                    schema::dt_fin.eq(&self.dt_fin),
                    schema::nome.eq(&self.nome),
                    schema::cnpj.eq(&self.cnpj),
                    schema::uf.eq(&self.uf),
                    schema::cod_mun.eq(&self.cod_mun),
                    schema::suframa.eq(&self.suframa),
                    schema::ind_nat_pj.eq(&self.ind_nat_pj),
                    schema::ind_ativ.eq(&self.ind_ativ),
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
        "Efd0000".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd0000 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Efd0000,
    [
        reg,
        cod_ver,
        tipo_escrit,
        ind_sit_esp,
        num_rec_anterior,
        dt_ini,
        dt_fin,
        nome,
        cnpj,
        uf,
        cod_mun,
        suframa,
        ind_nat_pj,
        ind_ativ
    ]
);
register_model!(Efd0000, "0000");
