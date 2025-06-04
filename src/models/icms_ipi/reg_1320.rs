#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1320::reg_1320::dsl as schema;
use crate::schemas::reg_1320::reg_1320::table;
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
#[diesel(table_name = crate::schemas::reg_1320::reg_1320::dsl)]
pub struct Reg1320 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub num_bico: Option<String>,
    pub nr_interv: Option<String>,
    pub mot_interv: Option<String>,
    pub nom_interv: Option<String>,
    pub cnpj_interv: Option<String>,
    pub cpf_interv: Option<String>,
    pub val_fecha: Option<String>,
    pub val_abert: Option<String>,
    pub vol_aferi: Option<String>,
    pub vol_vendas: Option<String>,
}

#[async_trait]
impl Model for Reg1320 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1320 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            num_bico: get_field(&fields, 2),
            nr_interv: get_field(&fields, 3),
            mot_interv: get_field(&fields, 4),
            nom_interv: get_field(&fields, 5),
            cnpj_interv: get_field(&fields, 6),
            cpf_interv: get_field(&fields, 7),
            val_fecha: get_field(&fields, 8),
            val_abert: get_field(&fields, 9),
            vol_aferi: get_field(&fields, 10),
            vol_vendas: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1320>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1320::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1320::as_select())
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
                    schema::num_bico.eq(&self.num_bico),
                    schema::nr_interv.eq(&self.nr_interv),
                    schema::mot_interv.eq(&self.mot_interv),
                    schema::nom_interv.eq(&self.nom_interv),
                    schema::cnpj_interv.eq(&self.cnpj_interv),
                    schema::cpf_interv.eq(&self.cpf_interv),
                    schema::val_fecha.eq(&self.val_fecha),
                    schema::val_abert.eq(&self.val_abert),
                    schema::vol_aferi.eq(&self.vol_aferi),
                    schema::vol_vendas.eq(&self.vol_vendas),
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
        "Reg1320".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1320 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Reg1320,
    [
        reg,
        num_bico,
        nr_interv,
        mot_interv,
        nom_interv,
        cnpj_interv,
        cpf_interv,
        val_fecha,
        val_abert,
        vol_aferi,
        vol_vendas
    ]
);
register_model!(Reg1320, "1320");
