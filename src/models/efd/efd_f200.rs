use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_f200::efd_f200::dsl as schema;
use crate::schemas::efd_f200::efd_f200::table;
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
#[diesel(table_name = crate::schemas::efd_f200::efd_f200::dsl)]
pub struct EfdF200 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub ind_oper: Option<String>,
    pub unid_imob: Option<String>,
    pub ident_emp: Option<String>,
    pub desc_unid_imob: Option<String>,
    pub num_cont: Option<String>,
    pub cpf_cnpj_adqu: Option<String>,
    pub dt_oper: Option<String>,
    pub vl_tot_vend: Option<String>,
    pub vl_rec_acum: Option<String>,
    pub vl_tot_rec: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub vl_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub vl_cofins: Option<String>,
    pub perc_rec_receb: Option<String>,
    pub ind_nat_emp: Option<String>,
    pub inf_comp: Option<String>,
}

#[async_trait]
impl Model for EfdF200 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdF200 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    ind_oper: get_field(&fields, 2),
        unid_imob: get_field(&fields, 3),
        ident_emp: get_field(&fields, 4),
        desc_unid_imob: get_field(&fields, 5),
        num_cont: get_field(&fields, 6),
        cpf_cnpj_adqu: get_field(&fields, 7),
        dt_oper: get_field(&fields, 8),
        vl_tot_vend: get_field(&fields, 9),
        vl_rec_acum: get_field(&fields, 10),
        vl_tot_rec: get_field(&fields, 11),
        cst_pis: get_field(&fields, 12),
        vl_bc_pis: get_field(&fields, 13),
        aliq_pis: get_field(&fields, 14),
        vl_pis: get_field(&fields, 15),
        cst_cofins: get_field(&fields, 16),
        vl_bc_cofins: get_field(&fields, 17),
        aliq_cofins: get_field(&fields, 18),
        vl_cofins: get_field(&fields, 19),
        perc_rec_receb: get_field(&fields, 20),
        ind_nat_emp: get_field(&fields, 21),
        inf_comp: get_field(&fields, 22),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdF200>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdF200::as_select())
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
schema::unid_imob.eq(&self.unid_imob),
schema::ident_emp.eq(&self.ident_emp),
schema::desc_unid_imob.eq(&self.desc_unid_imob),
schema::num_cont.eq(&self.num_cont),
schema::cpf_cnpj_adqu.eq(&self.cpf_cnpj_adqu),
schema::dt_oper.eq(&self.dt_oper),
schema::vl_tot_vend.eq(&self.vl_tot_vend),
schema::vl_rec_acum.eq(&self.vl_rec_acum),
schema::vl_tot_rec.eq(&self.vl_tot_rec),
schema::cst_pis.eq(&self.cst_pis),
schema::vl_bc_pis.eq(&self.vl_bc_pis),
schema::aliq_pis.eq(&self.aliq_pis),
schema::vl_pis.eq(&self.vl_pis),
schema::cst_cofins.eq(&self.cst_cofins),
schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
schema::aliq_cofins.eq(&self.aliq_cofins),
schema::vl_cofins.eq(&self.vl_cofins),
schema::perc_rec_receb.eq(&self.perc_rec_receb),
schema::ind_nat_emp.eq(&self.ind_nat_emp),
schema::inf_comp.eq(&self.inf_comp),
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
        "EfdF200".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdF200 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdF200, [reg, ind_oper, unid_imob, ident_emp, desc_unid_imob, num_cont, cpf_cnpj_adqu, dt_oper, vl_tot_vend, vl_rec_acum, vl_tot_rec, cst_pis, vl_bc_pis, aliq_pis, vl_pis, cst_cofins, vl_bc_cofins, aliq_cofins, vl_cofins, perc_rec_receb, ind_nat_emp, inf_comp]);
register_model!(EfdF200, "f200");
