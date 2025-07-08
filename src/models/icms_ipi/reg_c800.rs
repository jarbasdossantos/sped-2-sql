use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_c800::dsl as schema;
use crate::schemas::reg_c800::table;
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
#[diesel(table_name = crate::schemas::reg_c800::dsl)]
pub struct RegC800 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_sit: Option<String>,
    pub num_cfe: Option<String>,
    pub dt_doc: Option<String>,
    pub vl_cfe: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
    pub cnpj_cpf: Option<String>,
    pub nr_sat: Option<String>,
    pub chv_cfe: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_merc: Option<String>,
    pub vl_out_da: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_pis_st: Option<String>,
    pub vl_cofins_st: Option<String>,
}

#[async_trait]
impl Model for RegC800 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegC800 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_mod: get_field(&fields, 2),
            cod_sit: get_field(&fields, 3),
            num_cfe: get_field(&fields, 4),
            dt_doc: get_field(&fields, 5),
            vl_cfe: get_field(&fields, 6),
            vl_pis: get_field(&fields, 7),
            vl_cofins: get_field(&fields, 8),
            cnpj_cpf: get_field(&fields, 9),
            nr_sat: get_field(&fields, 10),
            chv_cfe: get_field(&fields, 11),
            vl_desc: get_field(&fields, 12),
            vl_merc: get_field(&fields, 13),
            vl_out_da: get_field(&fields, 14),
            vl_icms: get_field(&fields, 15),
            vl_pis_st: get_field(&fields, 16),
            vl_cofins_st: get_field(&fields, 17),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<RegC800>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegC800::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegC800::as_select())
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
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::num_cfe.eq(&self.num_cfe),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::vl_cfe.eq(&self.vl_cfe),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::cnpj_cpf.eq(&self.cnpj_cpf),
                    schema::nr_sat.eq(&self.nr_sat),
                    schema::chv_cfe.eq(&self.chv_cfe),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_merc.eq(&self.vl_merc),
                    schema::vl_out_da.eq(&self.vl_out_da),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_pis_st.eq(&self.vl_pis_st),
                    schema::vl_cofins_st.eq(&self.vl_cofins_st),
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
        "RegC800".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegC800 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegC800, [reg, cod_mod, cod_sit, num_cfe, dt_doc, vl_cfe, vl_pis, vl_cofins, cnpj_cpf, nr_sat, chv_cfe, vl_desc, vl_merc, vl_out_da, vl_icms, vl_pis_st, vl_cofins_st]);
register_model!(RegC800, "c800");
