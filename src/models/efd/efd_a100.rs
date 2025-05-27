use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_a100::efd_a100::dsl as schema;
use crate::schemas::efd_a100::efd_a100::table;
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
#[diesel(table_name = crate::schemas::efd_a100::efd_a100::dsl)]
pub struct EfdA100 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_oper: Option<String>,
    pub ind_emit: Option<String>,
    pub cod_part: Option<String>,
    pub cod_sit: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub num_doc: Option<String>,
    pub chv_nfse: Option<String>,
    pub dt_doc: Option<String>,
    pub dt_exe_serv: Option<String>,
    pub vl_doc: Option<String>,
    pub ind_pgto: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub vl_cofins: Option<String>,
    pub vl_pis_ret: Option<String>,
    pub vl_cofins_ret: Option<String>,
    pub vl_iss: Option<String>,
}

#[async_trait]
impl Model for EfdA100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdA100 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_oper: get_field(&fields, 2),
            ind_emit: get_field(&fields, 3),
            cod_part: get_field(&fields, 4),
            cod_sit: get_field(&fields, 5),
            ser: get_field(&fields, 6),
            sub: get_field(&fields, 7),
            num_doc: get_field(&fields, 8),
            chv_nfse: get_field(&fields, 9),
            dt_doc: get_field(&fields, 10),
            dt_exe_serv: get_field(&fields, 11),
            vl_doc: get_field(&fields, 12),
            ind_pgto: get_field(&fields, 13),
            vl_desc: get_field(&fields, 14),
            vl_bc_pis: get_field(&fields, 15),
            vl_pis: get_field(&fields, 16),
            vl_bc_cofins: get_field(&fields, 17),
            vl_cofins: get_field(&fields, 18),
            vl_pis_ret: get_field(&fields, 19),
            vl_cofins_ret: get_field(&fields, 20),
            vl_iss: get_field(&fields, 21),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdA100>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(&parent_id.expect("Invalid parent id")))
            .select(EfdA100::as_select())
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
                    schema::ind_oper.eq(&self.ind_oper),
                    schema::ind_emit.eq(&self.ind_emit),
                    schema::cod_part.eq(&self.cod_part),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::num_doc.eq(&self.num_doc),
                    schema::chv_nfse.eq(&self.chv_nfse),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::dt_exe_serv.eq(&self.dt_exe_serv),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::ind_pgto.eq(&self.ind_pgto),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_bc_pis.eq(&self.vl_bc_pis),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::vl_pis_ret.eq(&self.vl_pis_ret),
                    schema::vl_cofins_ret.eq(&self.vl_cofins_ret),
                    schema::vl_iss.eq(&self.vl_iss),
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
        "EfdA100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdA100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdA100, [reg, ind_oper, ind_emit, cod_part, cod_sit, ser, sub, num_doc, chv_nfse, dt_doc, dt_exe_serv, vl_doc, ind_pgto, vl_desc, vl_bc_pis, vl_pis, vl_bc_cofins, vl_cofins, vl_pis_ret, vl_cofins_ret, vl_iss]);
register_model!(EfdA100, "a100");
