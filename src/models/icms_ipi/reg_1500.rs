use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::reg_1500::dsl as schema;
use crate::schemas::reg_1500::table;
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
#[diesel(table_name = crate::schemas::reg_1500::dsl)]
pub struct Reg1500 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_oper: Option<String>,
    pub ind_emit: Option<String>,
    pub cod_part: Option<String>,
    pub cod_mod: Option<String>,
    pub cod_sit: Option<String>,
    pub ser: Option<String>,
    pub sub: Option<String>,
    pub cod_cons: Option<String>,
    pub num_doc: Option<String>,
    pub dt_doc: Option<String>,
    pub dt_e_s: Option<String>,
    pub vl_doc: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_forn: Option<String>,
    pub vl_serv_nt: Option<String>,
    pub vl_terc: Option<String>,
    pub vl_da: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub cod_inf: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofis: Option<String>,
    pub tp_ligacao: Option<String>,
    pub cod_grupo_tensao: Option<String>,
}

#[async_trait]
impl Model for Reg1500 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        Reg1500 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            ind_oper: get_field(&fields, 2),
            ind_emit: get_field(&fields, 3),
            cod_part: get_field(&fields, 4),
            cod_mod: get_field(&fields, 5),
            cod_sit: get_field(&fields, 6),
            ser: get_field(&fields, 7),
            sub: get_field(&fields, 8),
            cod_cons: get_field(&fields, 9),
            num_doc: get_field(&fields, 10),
            dt_doc: get_field(&fields, 11),
            dt_e_s: get_field(&fields, 12),
            vl_doc: get_field(&fields, 13),
            vl_desc: get_field(&fields, 14),
            vl_forn: get_field(&fields, 15),
            vl_serv_nt: get_field(&fields, 16),
            vl_terc: get_field(&fields, 17),
            vl_da: get_field(&fields, 18),
            vl_bc_icms: get_field(&fields, 19),
            vl_icms: get_field(&fields, 20),
            vl_bc_icms_st: get_field(&fields, 21),
            vl_icms_st: get_field(&fields, 22),
            cod_inf: get_field(&fields, 23),
            vl_pis: get_field(&fields, 24),
            vl_cofis: get_field(&fields, 25),
            tp_ligacao: get_field(&fields, 26),
            cod_grupo_tensao: get_field(&fields, 27),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<Reg1500>, Error> {
        let mut conn = DB_POOL.lock().await.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(Reg1500::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(Reg1500::as_select())
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
                    schema::ind_oper.eq(&self.ind_oper),
                    schema::ind_emit.eq(&self.ind_emit),
                    schema::cod_part.eq(&self.cod_part),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::ser.eq(&self.ser),
                    schema::sub.eq(&self.sub),
                    schema::cod_cons.eq(&self.cod_cons),
                    schema::num_doc.eq(&self.num_doc),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::dt_e_s.eq(&self.dt_e_s),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_forn.eq(&self.vl_forn),
                    schema::vl_serv_nt.eq(&self.vl_serv_nt),
                    schema::vl_terc.eq(&self.vl_terc),
                    schema::vl_da.eq(&self.vl_da),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_bc_icms_st.eq(&self.vl_bc_icms_st),
                    schema::vl_icms_st.eq(&self.vl_icms_st),
                    schema::cod_inf.eq(&self.cod_inf),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofis.eq(&self.vl_cofis),
                    schema::tp_ligacao.eq(&self.tp_ligacao),
                    schema::cod_grupo_tensao.eq(&self.cod_grupo_tensao),
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
        "Reg1500".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for Reg1500 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(Reg1500, [reg, ind_oper, ind_emit, cod_part, cod_mod, cod_sit, ser, sub, cod_cons, num_doc, dt_doc, dt_e_s, vl_doc, vl_desc, vl_forn, vl_serv_nt, vl_terc, vl_da, vl_bc_icms, vl_icms, vl_bc_icms_st, vl_icms_st, cod_inf, vl_pis, vl_cofis, tp_ligacao, cod_grupo_tensao]);
register_model!(Reg1500, "1500");
