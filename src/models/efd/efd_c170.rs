use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_c170::efd_c170::dsl as schema;
use crate::schemas::efd_c170::efd_c170::table;
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
#[diesel(table_name = crate::schemas::efd_c170::efd_c170::dsl)]
pub struct EfdC170 {
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
    pub num_item: Option<String>,
    pub cod_item: Option<String>,
    pub descr_compl: Option<String>,
    pub qtd: Option<String>,
    pub unid: Option<String>,
    pub vl_item: Option<String>,
    pub vl_desc: Option<String>,
    pub ind_mov: Option<String>,
    pub cst_icms: Option<String>,
    pub cfop: Option<String>,
    pub cod_nat: Option<String>,
    pub vl_bc_icms: Option<String>,
    pub aliq_icms: Option<String>,
    pub vl_icms: Option<String>,
    pub vl_bc_icms_st: Option<String>,
    pub aliq_st: Option<String>,
    pub vl_icms_st: Option<String>,
    pub ind_apur: Option<String>,
    pub cst_ipi: Option<String>,
    pub cod_enq: Option<String>,
    pub vl_bc_ipi: Option<String>,
    pub aliq_ipi: Option<String>,
    pub vl_ipi: Option<String>,
    pub cst_pis: Option<String>,
    pub vl_bc_pis: Option<String>,
    pub aliq_pis: Option<String>,
    pub quant_bc_pis: Option<String>,
    pub aliq_pis_quant: Option<String>,
    pub vl_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub vl_bc_cofins: Option<String>,
    pub aliq_cofins: Option<String>,
    pub quant_bc_cofins: Option<String>,
    pub aliq_cofins_quant: Option<String>,
    pub vl_cofins: Option<String>,
    pub cod_cta: Option<String>,
}

#[async_trait]
impl Model for EfdC170 {
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {
        EfdC170 {
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
                    num_item: get_field(&fields, 2),
        cod_item: get_field(&fields, 3),
        descr_compl: get_field(&fields, 4),
        qtd: get_field(&fields, 5),
        unid: get_field(&fields, 6),
        vl_item: get_field(&fields, 7),
        vl_desc: get_field(&fields, 8),
        ind_mov: get_field(&fields, 9),
        cst_icms: get_field(&fields, 10),
        cfop: get_field(&fields, 11),
        cod_nat: get_field(&fields, 12),
        vl_bc_icms: get_field(&fields, 13),
        aliq_icms: get_field(&fields, 14),
        vl_icms: get_field(&fields, 15),
        vl_bc_icms_st: get_field(&fields, 16),
        aliq_st: get_field(&fields, 17),
        vl_icms_st: get_field(&fields, 18),
        ind_apur: get_field(&fields, 19),
        cst_ipi: get_field(&fields, 20),
        cod_enq: get_field(&fields, 21),
        vl_bc_ipi: get_field(&fields, 22),
        aliq_ipi: get_field(&fields, 23),
        vl_ipi: get_field(&fields, 24),
        cst_pis: get_field(&fields, 25),
        vl_bc_pis: get_field(&fields, 26),
        aliq_pis: get_field(&fields, 27),
        quant_bc_pis: get_field(&fields, 28),
        aliq_pis_quant: get_field(&fields, 29),
        vl_pis: get_field(&fields, 30),
        cst_cofins: get_field(&fields, 31),
        vl_bc_cofins: get_field(&fields, 32),
        aliq_cofins: get_field(&fields, 33),
        quant_bc_cofins: get_field(&fields, 34),
        aliq_cofins_quant: get_field(&fields, 35),
        vl_cofins: get_field(&fields, 36),
        cod_cta: get_field(&fields, 37),
        }
    }

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<EfdC170>, Error> {
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select(EfdC170::as_select())
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
            schema::num_item.eq(&self.num_item),
schema::cod_item.eq(&self.cod_item),
schema::descr_compl.eq(&self.descr_compl),
schema::qtd.eq(&self.qtd),
schema::unid.eq(&self.unid),
schema::vl_item.eq(&self.vl_item),
schema::vl_desc.eq(&self.vl_desc),
schema::ind_mov.eq(&self.ind_mov),
schema::cst_icms.eq(&self.cst_icms),
schema::cfop.eq(&self.cfop),
schema::cod_nat.eq(&self.cod_nat),
schema::vl_bc_icms.eq(&self.vl_bc_icms),
schema::aliq_icms.eq(&self.aliq_icms),
schema::vl_icms.eq(&self.vl_icms),
schema::vl_bc_icms_st.eq(&self.vl_bc_icms_st),
schema::aliq_st.eq(&self.aliq_st),
schema::vl_icms_st.eq(&self.vl_icms_st),
schema::ind_apur.eq(&self.ind_apur),
schema::cst_ipi.eq(&self.cst_ipi),
schema::cod_enq.eq(&self.cod_enq),
schema::vl_bc_ipi.eq(&self.vl_bc_ipi),
schema::aliq_ipi.eq(&self.aliq_ipi),
schema::vl_ipi.eq(&self.vl_ipi),
schema::cst_pis.eq(&self.cst_pis),
schema::vl_bc_pis.eq(&self.vl_bc_pis),
schema::aliq_pis.eq(&self.aliq_pis),
schema::quant_bc_pis.eq(&self.quant_bc_pis),
schema::aliq_pis_quant.eq(&self.aliq_pis_quant),
schema::vl_pis.eq(&self.vl_pis),
schema::cst_cofins.eq(&self.cst_cofins),
schema::vl_bc_cofins.eq(&self.vl_bc_cofins),
schema::aliq_cofins.eq(&self.aliq_cofins),
schema::quant_bc_cofins.eq(&self.quant_bc_cofins),
schema::aliq_cofins_quant.eq(&self.aliq_cofins_quant),
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
        "EfdC170".to_string()
    }

    fn get_display_fields(&self) -> Vec<(String, String)> {
        self.generate_display_fields()
    }
}

impl fmt::Display for EfdC170 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_format(f)
    }
}

impl_display_fields!(EfdC170, [reg, num_item, cod_item, descr_compl, qtd, unid, vl_item, vl_desc, ind_mov, cst_icms, cfop, cod_nat, vl_bc_icms, aliq_icms, vl_icms, vl_bc_icms_st, aliq_st, vl_icms_st, ind_apur, cst_ipi, cod_enq, vl_bc_ipi, aliq_ipi, vl_ipi, cst_pis, vl_bc_pis, aliq_pis, quant_bc_pis, aliq_pis_quant, vl_pis, cst_cofins, vl_bc_cofins, aliq_cofins, quant_bc_cofins, aliq_cofins_quant, vl_cofins, cod_cta]);
register_model!(EfdC170, "c170");
