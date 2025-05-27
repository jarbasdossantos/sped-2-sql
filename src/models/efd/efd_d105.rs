use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_d105::efd_d105::dsl as schema;
use crate::schemas::efd_d105::efd_d105::table;
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
#[diesel(table_name = crate::schemas::efd_d105::efd_d105::dsl)]
pub struct EfdD105 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_nat_frt: Option<String>,
    pub vl_item: Option<String>,
    pub cst_cofins: Option<String>,
    pub nat_bc_cred: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdD105 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdD105 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    ind_nat_frt: get_field(&fields, 2),
        vl_item: get_field(&fields, 3),
        cst_cofins: get_field(&fields, 4),
        nat_bc_cred: get_field(&fields, 5),
        vl_bc_cofins: get_field(&fields, 6),
        aliq_cofins: get_field(&fields, 7),
        vl_cofins: get_field(&fields, 8),
        cod_cta: get_field(&fields, 9),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdD105>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdD105::as_select())
            .load(&mut DB_POOL
                .get().unwrap())?)
    }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {
        Box::pin(async move {
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
            schema::ind_nat_frt.eq(&self.ind_nat_frt),
schema::vl_item.eq(&self.vl_item),
schema::cst_cofins.eq(&self.cst_cofins),
schema::nat_bc_cred.eq(&self.nat_bc_cred),
schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
schema::aliq_cofins.eq(&self.aliq_cofins),
schema::vl_cofins.eq(&self.vl_cofins),
schema::cod_cta.eq(&self.cod_cta),
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
        "EfdD105".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdD105 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdD105, [reg, ind_nat_frt, vl_item, cst_cofins, nat_bc_cred, vl_bc_cofins, aliq_cofins, vl_cofins, cod_cta]);
register_model!(EfdD105, "d105");
