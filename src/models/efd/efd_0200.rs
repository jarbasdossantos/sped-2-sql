use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_0200::dsl as schema;
use crate::schemas::efd_0200::table;
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
#[diesel(table_name = crate::schemas::efd_0200::dsl)]
pub struct Efd0200 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_item: Option<String>,
    pub descr_item: Option<String>,
    pub cod_barra: Option<String>,
    pub cod_ant_item: Option<String>,
    pub unid_inv: Option<String>,
    pub tipo_item: Option<String>,
    pub cod_ncm: Option<String>,
    pub ex_ipi: Option<String>,
    pub cod_gen: Option<String>,
    pub cod_lst: Option<String>,
    pub aliq_icms: Option<String>,
}

#[async_trait]
impl Model for Efd0200 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd0200 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_item: get_field(&fields, 2),
            descr_item: get_field(&fields, 3),
            cod_barra: get_field(&fields, 4),
            cod_ant_item: get_field(&fields, 5),
            unid_inv: get_field(&fields, 6),
            tipo_item: get_field(&fields, 7),
            cod_ncm: get_field(&fields, 8),
            ex_ipi: get_field(&fields, 9),
            cod_gen: get_field(&fields, 10),
            cod_lst: get_field(&fields, 11),
            aliq_icms: get_field(&fields, 12),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd0200>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd0200::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd0200::as_select())
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
                    schema::cod_item.eq(&self.cod_item),
                    schema::descr_item.eq(&self.descr_item),
                    schema::cod_barra.eq(&self.cod_barra),
                    schema::cod_ant_item.eq(&self.cod_ant_item),
                    schema::unid_inv.eq(&self.unid_inv),
                    schema::tipo_item.eq(&self.tipo_item),
                    schema::cod_ncm.eq(&self.cod_ncm),
                    schema::ex_ipi.eq(&self.ex_ipi),
                    schema::cod_gen.eq(&self.cod_gen),
                    schema::cod_lst.eq(&self.cod_lst),
                    schema::aliq_icms.eq(&self.aliq_icms),
                ))
                .execute(&mut conn)?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut conn)?
        })
    }

    fn get_id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn get_file_id(&self) -> Option<i32> {
        self.file_id
    }

    fn get_entity_name(&self) -> String {
        "Efd0200".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd0200 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Efd0200, [reg, cod_item, descr_item, cod_barra, cod_ant_item, unid_inv, tipo_item, cod_ncm, ex_ipi, cod_gen, cod_lst, aliq_icms]);
register_model!(Efd0200, "0200");
