use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_0100::reg_0100::dsl as schema;
use crate::schemas::reg_0100::reg_0100::table;
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
#[diesel(table_name = crate::schemas::reg_0100::reg_0100::dsl)]
pub struct Reg0100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub nome: Option<String>,
    pub cpf: Option<String>,
    pub crc: Option<String>,
    pub cnpj: Option<String>,
    pub cep: Option<String>,
    pub endereco: Option<String>,
    pub num: Option<String>,
    pub compl: Option<String>,
    pub bairro: Option<String>,
    pub fone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub cod_mun: Option<String>,
}

#[async_trait]
impl Model for Reg0100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg0100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            nome: get_field(&fields, 2),
            cpf: get_field(&fields, 3),
            crc: get_field(&fields, 4),
            cnpj: get_field(&fields, 5),
            cep: get_field(&fields, 6),
            endereco: get_field(&fields, 7),
            num: get_field(&fields, 8),
            compl: get_field(&fields, 9),
            bairro: get_field(&fields, 10),
            fone: get_field(&fields, 11),
            fax: get_field(&fields, 12),
            email: get_field(&fields, 13),
            cod_mun: get_field(&fields, 14),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg0100>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg0100::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg0100::as_select())
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
                    schema::nome.eq(&self.nome),
                    schema::cpf.eq(&self.cpf),
                    schema::crc.eq(&self.crc),
                    schema::cnpj.eq(&self.cnpj),
                    schema::cep.eq(&self.cep),
                    schema::endereco.eq(&self.endereco),
                    schema::num.eq(&self.num),
                    schema::compl.eq(&self.compl),
                    schema::bairro.eq(&self.bairro),
                    schema::fone.eq(&self.fone),
                    schema::fax.eq(&self.fax),
                    schema::email.eq(&self.email),
                    schema::cod_mun.eq(&self.cod_mun),
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
        "Reg0100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg0100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Reg0100,
    [reg, nome, cpf, crc, cnpj, cep, endereco, num, compl, bairro, fone, fax, email, cod_mun]
);
register_model!(Reg0100, "0100");
