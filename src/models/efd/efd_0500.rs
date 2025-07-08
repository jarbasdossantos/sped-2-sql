use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_0500::dsl as schema;
use crate::schemas::efd_0500::table;
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
#[diesel(table_name = crate::schemas::efd_0500::dsl)]
pub struct Efd0500 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub dt_alt: Option<String>,
    pub cod_nat_cc: Option<String>,
    pub ind_cta: Option<String>,
    pub nivel: Option<String>,
    pub cod_cta: Option<String>,
    pub nome_cta: Option<String>,
    pub cod_cta_ref: Option<String>,
    pub cnpj_est: Option<String>,
}

#[async_trait]
impl Model for Efd0500 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd0500 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            dt_alt: get_field(&fields, 2),
            cod_nat_cc: get_field(&fields, 3),
            ind_cta: get_field(&fields, 4),
            nivel: get_field(&fields, 5),
            cod_cta: get_field(&fields, 6),
            nome_cta: get_field(&fields, 7),
            cod_cta_ref: get_field(&fields, 8),
            cnpj_est: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd0500>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd0500::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd0500::as_select())
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
                    schema::dt_alt.eq(&self.dt_alt),
                    schema::cod_nat_cc.eq(&self.cod_nat_cc),
                    schema::ind_cta.eq(&self.ind_cta),
                    schema::nivel.eq(&self.nivel),
                    schema::cod_cta.eq(&self.cod_cta),
                    schema::nome_cta.eq(&self.nome_cta),
                    schema::cod_cta_ref.eq(&self.cod_cta_ref),
                    schema::cnpj_est.eq(&self.cnpj_est),
                ))
                .execute(&mut DB_POOL.lock().await.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.lock().await.get().unwrap())
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Efd0500".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd0500 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd0500, [reg, dt_alt, cod_nat_cc, ind_cta, nivel, cod_cta, nome_cta, cod_cta_ref, cnpj_est]);
register_model!(Efd0500, "0500");
