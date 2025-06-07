use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::models::utils::to_date;
use crate::schemas::reg_0000::dsl as schema;
use crate::schemas::reg_0000::table;
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
#[diesel(table_name = crate::schemas::reg_0000::dsl)]
pub struct Reg0000 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_ver: Option<String>,
    pub cod_fin: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fin: Option<String>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub uf: Option<String>,
    pub ie: Option<String>,
    pub cod_mun: Option<String>,
    pub im: Option<String>,
    pub suframa: Option<String>,
    pub ind_perfil: Option<String>,
    pub ind_ativ: Option<String>,
}

#[async_trait]
impl Model for Reg0000 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg0000 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_ver: get_field(&fields, 2),
            cod_fin: get_field(&fields, 3),
            dt_ini: to_date(get_field(&fields, 4)),
            dt_fin: to_date(get_field(&fields, 5)),
            nome: get_field(&fields, 6),
            cnpj: get_field(&fields, 7),
            cpf: get_field(&fields, 8),
            uf: get_field(&fields, 9),
            ie: get_field(&fields, 10),
            cod_mun: get_field(&fields, 11),
            im: get_field(&fields, 12),
            suframa: get_field(&fields, 13),
            ind_perfil: get_field(&fields, 14),
            ind_ativ: get_field(&fields, 15),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg0000>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .select(Reg0000::as_select())
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
                    schema::cod_fin.eq(&self.cod_fin),
                    schema::dt_ini.eq(&self.dt_ini),
                    schema::dt_fin.eq(&self.dt_fin),
                    schema::nome.eq(&self.nome),
                    schema::cnpj.eq(&self.cnpj),
                    schema::cpf.eq(&self.cpf),
                    schema::uf.eq(&self.uf),
                    schema::ie.eq(&self.ie),
                    schema::cod_mun.eq(&self.cod_mun),
                    schema::im.eq(&self.im),
                    schema::suframa.eq(&self.suframa),
                    schema::ind_perfil.eq(&self.ind_perfil),
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
        "Reg0000".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg0000 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Reg0000,
    [
        reg, cod_ver, cod_fin, dt_ini, dt_fin, nome, cnpj, cpf, uf, ie, cod_mun, im, suframa,
        ind_perfil, ind_ativ
    ]
);
register_model!(Reg0000, "0000");
