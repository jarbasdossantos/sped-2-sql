use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_g125::dsl as schema;
use crate::schemas::reg_g125::table;
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
use diesel::sqlite::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::reg_g125::dsl)]
pub struct RegG125 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub cod_ind_bem: Option<String>,
    pub dt_mov: Option<String>,
    pub tipo_mov: Option<String>,
    pub vl_imob_icms_op: Option<String>,
    pub vl_imob_icms_st: Option<String>,
    pub vl_imob_icms_frt: Option<String>,
    pub vl_imob_icms_dif: Option<String>,
    pub num_parc: Option<String>,
    pub vl_parc_pass: Option<String>,
}

#[async_trait]
impl Model for RegG125 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        RegG125 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            cod_ind_bem: get_field(&fields, 2),
            dt_mov: get_field(&fields, 3),
            tipo_mov: get_field(&fields, 4),
            vl_imob_icms_op: get_field(&fields, 5),
            vl_imob_icms_st: get_field(&fields, 6),
            vl_imob_icms_frt: get_field(&fields, 7),
            vl_imob_icms_dif: get_field(&fields, 8),
            num_parc: get_field(&fields, 9),
            vl_parc_pass: get_field(&fields, 10),
        }
    }

    fn get(file_id: i32, parent_id: Option<i32>, conn: &mut SqliteConnection) -> Result<Vec<RegG125>, Error> {
        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(RegG125::as_select())
                .load(conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(RegG125::as_select())
                .load(conn)?)
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
                    schema::cod_ind_bem.eq(&self.cod_ind_bem),
                    schema::dt_mov.eq(&self.dt_mov),
                    schema::tipo_mov.eq(&self.tipo_mov),
                    schema::vl_imob_icms_op.eq(&self.vl_imob_icms_op),
                    schema::vl_imob_icms_st.eq(&self.vl_imob_icms_st),
                    schema::vl_imob_icms_frt.eq(&self.vl_imob_icms_frt),
                    schema::vl_imob_icms_dif.eq(&self.vl_imob_icms_dif),
                    schema::num_parc.eq(&self.num_parc),
                    schema::vl_parc_pass.eq(&self.vl_parc_pass),
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
        "RegG125".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for RegG125 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(RegG125, [reg, cod_ind_bem, dt_mov, tipo_mov, vl_imob_icms_op, vl_imob_icms_st, vl_imob_icms_frt, vl_imob_icms_dif, num_parc, vl_parc_pass]);
register_model!(RegG125, "g125");
