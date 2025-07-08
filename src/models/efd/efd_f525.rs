use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f525::dsl as schema;
use crate::schemas::efd_f525::table;
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
#[diesel(table_name = crate::schemas::efd_f525::dsl)]
pub struct EfdF525 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub vl_rec: Option<String>,
    pub ind_rec: Option<String>,
    pub cnpj_cpf: Option<String>,
    pub num_doc: Option<String>,
    pub cod_item: Option<String>,
    pub vl_rec_det: Option<String>,
    pub cst_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub info_compl: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdF525 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF525 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            vl_rec: get_field(&fields, 2),
            ind_rec: get_field(&fields, 3),
            cnpj_cpf: get_field(&fields, 4),
            num_doc: get_field(&fields, 5),
            cod_item: get_field(&fields, 6),
            vl_rec_det: get_field(&fields, 7),
            cst_pis: get_field(&fields, 8),
            cst_cofins: get_field(&fields, 9),
            info_compl: get_field(&fields, 10),
            cod_cta: get_field(&fields, 11),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF525>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdF525::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdF525::as_select())
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
                    schema::vl_rec.eq(&self.vl_rec),
                    schema::ind_rec.eq(&self.ind_rec),
                    schema::cnpj_cpf.eq(&self.cnpj_cpf),
                    schema::num_doc.eq(&self.num_doc),
                    schema::cod_item.eq(&self.cod_item),
                    schema::vl_rec_det.eq(&self.vl_rec_det),
                    schema::cst_pis.eq(&self.cst_pis),
                    schema::cst_cofins.eq(&self.cst_cofins),
                    schema::info_compl.eq(&self.info_compl),
                    schema::cod_cta.eq(&self.cod_cta),
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
        "EfdF525".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF525 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF525, [reg, vl_rec, ind_rec, cnpj_cpf, num_doc, cod_item, vl_rec_det, cst_pis, cst_cofins, info_compl, cod_cta]);
register_model!(EfdF525, "f525");
