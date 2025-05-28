use crate::database::DB_POOL;
use crate::impl_display_fields;
use crate::models::schema::reg_0000::dsl as schema;
use crate::models::schema::reg_0000::table;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, Selectable};
use serde::Serialize;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::models::schema::reg_0000)]
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
            reg: get_field(&fields, 1),
            cod_ver: get_field(&fields, 2),
            cod_fin: get_field(&fields, 3),
            dt_ini: get_field(&fields, 4),
            dt_fin: get_field(&fields, 5),
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

    async fn get(id: i32, parent: Option<i32>) -> Result<Box<Reg0000>, Error> {
        let conn = &mut DB_POOL
            .get()
            .expect("Failed to get DB connection from pool");

        if let Some(parent_id) = parent {
            let result = table
                .filter(schema::id.eq(id))
                .filter(schema::parent_id.eq(parent_id))
                .first::<Reg0000>(conn)?;

            return Ok(Box::new(result));
        }

        Ok(Box::new(
            table.filter(schema::id.eq(id)).first::<Reg0000>(conn)?,
        ))
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(self.file_id),
                    schema::parent_id.eq(self.parent_id),
                    schema::reg.eq(self.reg.clone()),
                    schema::cod_ver.eq(self.cod_ver.clone()),
                    schema::cod_fin.eq(self.cod_fin.clone()),
                    schema::dt_ini.eq(self.dt_ini.clone()),
                    schema::dt_fin.eq(self.dt_fin.clone()),
                    schema::nome.eq(self.nome.clone()),
                    schema::cnpj.eq(self.cnpj.clone()),
                    schema::cpf.eq(self.cpf.clone()),
                    schema::uf.eq(self.uf.clone()),
                    schema::ie.eq(self.ie.clone()),
                    schema::cod_mun.eq(self.cod_mun.clone()),
                    schema::im.eq(self.im.clone()),
                    schema::suframa.eq(self.suframa.clone()),
                    schema::ind_perfil.eq(self.ind_perfil.clone()),
                    schema::ind_ativ.eq(self.ind_ativ.clone()),
                ))
                .execute(&mut DB_POOL.get().unwrap())?;

            Ok(sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.get().unwrap())?)
        })
    }

    fn get_entity_name(&self) -> String {
        "Reg0000".to_string()
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
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
        reg,
        cod_ver,
        cod_fin,
        dt_ini,
        dt_fin,
        nome,
        cnpj,
        cpf,
        uf,
        ie,
        cod_mun,
        im,
        suframa,
        ind_perfil,
        ind_ativ
    ]
);
