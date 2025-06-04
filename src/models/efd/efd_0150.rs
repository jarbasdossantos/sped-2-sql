#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_0150::efd_0150::dsl as schema;
use crate::schemas::efd_0150::efd_0150::table;
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
#[diesel(table_name = crate::schemas::efd_0150::efd_0150::dsl)]
pub struct Efd0150 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_part: Option<String>,
    pub nome: Option<String>,
    pub cod_pais: Option<String>,
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub ie: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub end: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
}

#[async_trait]
impl Model for Efd0150 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd0150 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_part: get_field(&fields, 2),
            nome: get_field(&fields, 3),
            cod_pais: get_field(&fields, 4),
            cnpj: get_field(&fields, 5),
            cpf: get_field(&fields, 6),
            ie: get_field(&fields, 7),
            cod_mun: get_field(&fields, 8),
            suframa: get_field(&fields, 9),
            end: get_field(&fields, 10),
            num: get_field(&fields, 11),
            compl: get_field(&fields, 12),
            bairro: get_field(&fields, 13),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd0150>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd0150::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd0150::as_select())
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
                    schema::cod_part.eq(&self.cod_part),
                    schema::nome.eq(&self.nome),
                    schema::cod_pais.eq(&self.cod_pais),
                    schema::cnpj.eq(&self.cnpj),
                    schema::cpf.eq(&self.cpf),
                    schema::ie.eq(&self.ie),
                    schema::cod_mun.eq(&self.cod_mun),
                    schema::suframa.eq(&self.suframa),
                    schema::end.eq(&self.end),
                    schema::num.eq(&self.num),
                    schema::compl.eq(&self.compl),
                    schema::bairro.eq(&self.bairro),
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
        "Efd0150".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd0150 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Efd0150,
    [reg, cod_part, nome, cod_pais, cnpj, cpf, ie, cod_mun, suframa, end, num, compl, bairro]
);
register_model!(Efd0150, "0150");
