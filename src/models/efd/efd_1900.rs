#[allow(clippy::all)]
use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_1900::efd_1900::dsl as schema;
use crate::schemas::efd_1900::efd_1900::table;
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
#[diesel(table_name = crate::schemas::efd_1900::efd_1900::dsl)]
pub struct Efd1900 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cnpj: Option<String>,
    pub cod_mod: Option<String>,
    pub ser: Option<String>,
    pub sub_ser: Option<String>,
    pub cod_sit: Option<String>,
    pub vl_tot_rec: Option<String>,
    pub quant_doc: Option<String>,
    pub cst_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub cfop: Option<String>,
    pub info_compl: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for Efd1900 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Efd1900 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cnpj: get_field(&fields, 2),
            cod_mod: get_field(&fields, 3),
            ser: get_field(&fields, 4),
            sub_ser: get_field(&fields, 5),
            cod_sit: get_field(&fields, 6),
            vl_tot_rec: get_field(&fields, 7),
            quant_doc: get_field(&fields, 8),
            cst_pis: get_field(&fields, 9),
            cst_cofins: get_field(&fields, 10),
            cfop: get_field(&fields, 11),
            info_compl: get_field(&fields, 12),
            cod_cta: get_field(&fields, 13),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Efd1900>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Efd1900::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Efd1900::as_select())
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
                    schema::cnpj.eq(&self.cnpj),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::ser.eq(&self.ser),
                    schema::sub_ser.eq(&self.sub_ser),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::vl_tot_rec.eq(&self.vl_tot_rec),
                    schema::quant_doc.eq(&self.quant_doc),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::cfop.eq(&self.cfop),
                    schema::info_compl.eq(&self.info_compl),
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
        "Efd1900".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Efd1900 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(
    Efd1900,
    [
        reg, cnpj, cod_mod, ser, sub_ser, cod_sit, vl_tot_rec, quant_doc, cst_pis, cst_cofins,
        cfop, info_compl, cod_cta
    ]
);
register_model!(Efd1900, "1900");
