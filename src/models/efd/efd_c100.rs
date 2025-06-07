use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c100::dsl as schema;
use crate::schemas::efd_c100::table;
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
#[diesel(table_name = crate::schemas::efd_c100::dsl)]
pub struct EfdC100 {
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
    pub num_doc: Option<String>,
    pub chv_nfe: Option<String>,
    pub dt_doc: Option<String>,
    pub dt_e_s: Option<String>,
    pub vl_doc: Option<String>,
    pub ind_pgto: Option<String>,
    pub vl_desc: Option<String>,
    pub vl_abat_nt: Option<String>,
    pub vl_merc: Option<String>,
    pub ind_frt: Option<String>,
    pub vl_frt: Option<String>,
    pub vl_seg: Option<String>,
    pub vl_out_da: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub vl_ipi: Option<String>,
    pub vl_pis: Option<String>,
    pub vl_cofins: Option<String>,
    pub vl_pis_st: Option<String>,
    pub vl_cofins_st: Option<String>,
}

#[async_trait]
impl Model for EfdC100 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC100 {
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
            num_doc: get_field(&fields, 8),
            chv_nfe: get_field(&fields, 9),
            dt_doc: get_field(&fields, 10),
            dt_e_s: get_field(&fields, 11),
            vl_doc: get_field(&fields, 12),
            ind_pgto: get_field(&fields, 13),
            vl_desc: get_field(&fields, 14),
            vl_abat_nt: get_field(&fields, 15),
            vl_merc: get_field(&fields, 16),
            ind_frt: get_field(&fields, 17),
            vl_frt: get_field(&fields, 18),
            vl_seg: get_field(&fields, 19),
            vl_out_da: get_field(&fields, 20),
            vl_bc_icms: get_field(&fields, 21),
            vl_icms: get_field(&fields, 22),
            vl_bc_icms_st: get_field(&fields, 23),
            vl_icms_st: get_field(&fields, 24),
            vl_ipi: get_field(&fields, 25),
            vl_pis: get_field(&fields, 26),
            vl_cofins: get_field(&fields, 27),
            vl_pis_st: get_field(&fields, 28),
            vl_cofins_st: get_field(&fields, 29),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC100>, Error> {
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select(EfdC100::as_select())
                .load(&mut conn)?)
        } else {
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select(EfdC100::as_select())
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
                    schema::ind_oper.eq(&self.ind_oper),
                    schema::ind_emit.eq(&self.ind_emit),
                    schema::cod_part.eq(&self.cod_part),
                    schema::cod_mod.eq(&self.cod_mod),
                    schema::cod_sit.eq(&self.cod_sit),
                    schema::ser.eq(&self.ser),
                    schema::num_doc.eq(&self.num_doc),
                    schema::chv_nfe.eq(&self.chv_nfe),
                    schema::dt_doc.eq(&self.dt_doc),
                    schema::dt_e_s.eq(&self.dt_e_s),
                    schema::vl_doc.eq(&self.vl_doc),
                    schema::ind_pgto.eq(&self.ind_pgto),
                    schema::vl_desc.eq(&self.vl_desc),
                    schema::vl_abat_nt.eq(&self.vl_abat_nt),
                    schema::vl_merc.eq(&self.vl_merc),
                    schema::ind_frt.eq(&self.ind_frt),
                    schema::vl_frt.eq(&self.vl_frt),
                    schema::vl_seg.eq(&self.vl_seg),
                    schema::vl_out_da.eq(&self.vl_out_da),
                    schema::vl_bc_icms.eq(&self.vl_bc_icms),
                    schema::vl_icms.eq(&self.vl_icms),
                    schema::vl_bc_icms_st.eq(&self.vl_bc_icms_st),
                    schema::vl_icms_st.eq(&self.vl_icms_st),
                    schema::vl_ipi.eq(&self.vl_ipi),
                    schema::vl_pis.eq(&self.vl_pis),
                    schema::vl_cofins.eq(&self.vl_cofins),
                    schema::vl_pis_st.eq(&self.vl_pis_st),
                    schema::vl_cofins_st.eq(&self.vl_cofins_st),
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
        "EfdC100".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC100, [reg, ind_oper, ind_emit, cod_part, cod_mod, cod_sit, ser, num_doc, chv_nfe, dt_doc, dt_e_s, vl_doc, ind_pgto, vl_desc, vl_abat_nt, vl_merc, ind_frt, vl_frt, vl_seg, vl_out_da, vl_bc_icms, vl_icms, vl_bc_icms_st, vl_icms_st, vl_ipi, vl_pis, vl_cofins, vl_pis_st, vl_cofins_st]);
register_model!(EfdC100, "c100");
