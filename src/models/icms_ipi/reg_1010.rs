use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1010::dsl as schema;
use crate::schemas::reg_1010::table;
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
#[diesel(table_name = crate::schemas::reg_1010::dsl)]
pub struct Reg1010 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_exp: Option<String>,
    pub ind_ccrf: Option<String>,
    pub ind_comb: Option<String>,
    pub ind_usina: Option<String>,
    pub ind_va: Option<String>,
    pub ind_ee: Option<String>,
    pub ind_cart: Option<String>,
    pub ind_form: Option<String>,
    pub ind_aer: Option<String>,
}

#[async_trait]
impl Model for Reg1010 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1010 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_exp: get_field(&fields, 2),
            ind_ccrf: get_field(&fields, 3),
            ind_comb: get_field(&fields, 4),
            ind_usina: get_field(&fields, 5),
            ind_va: get_field(&fields, 6),
            ind_ee: get_field(&fields, 7),
            ind_cart: get_field(&fields, 8),
            ind_form: get_field(&fields, 9),
            ind_aer: get_field(&fields, 10),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1010>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1010::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1010::as_select())
                .load(&mut conn)?)
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
                    schema::ind_exp.eq(&self.ind_exp),
                    schema::ind_ccrf.eq(&self.ind_ccrf),
                    schema::ind_comb.eq(&self.ind_comb),
                    schema::ind_usina.eq(&self.ind_usina),
                    schema::ind_va.eq(&self.ind_va),
                    schema::ind_ee.eq(&self.ind_ee),
                    schema::ind_cart.eq(&self.ind_cart),
                    schema::ind_form.eq(&self.ind_form),
                    schema::ind_aer.eq(&self.ind_aer),
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
        "Reg1010".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1010 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1010, [reg, ind_exp, ind_ccrf, ind_comb, ind_usina, ind_va, ind_ee, ind_cart, ind_form, ind_aer]);
register_model!(Reg1010, "1010");
