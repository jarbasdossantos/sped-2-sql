use crate::database::DB_POOL;
use crate::models::schema::contribuicoes_0000::dsl::*;
use crate::models::schema::contribuicoes_0000::table;
use crate::models::traits::{Model, Reg};
use crate::models::utils::get_field;
use async_trait::async_trait;
use diesel::prelude::{Insertable, Queryable};
use diesel::result::Error;
use diesel::{RunQueryDsl, SelectableHelper};
use indexmap::IndexMap;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = crate::models::schema::contribuicoes_0000)]
pub struct Reg0000 {
    pub id: Option<id>,
    pub file_id: file_id,
    pub parent_id: Option<parent_id>,
    pub reg: Option<String>,
    pub cod_ver: Option<String>,
    pub tipo_escrit: Option<String>,
    pub ind_sit_esp: Option<String>,
    pub num_rec_anterior: Option<String>,
    pub dt_ini: Option<String>,
    pub dt_fin: Option<String>,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub uf: Option<String>,
    pub cod_mun: Option<String>,
    pub suframa: Option<String>,
    pub ind_nat_pj: Option<String>,
    pub ind_ativ: Option<String>,
}

#[async_trait]
impl Model for Reg0000 {
    type Table = table;
    type Id = id;
    type FileId = file_id;
    type ParentId = parent_id;

    fn table() -> table {
        table
    }

    fn new(fields: Vec<&str>, new_id: Option<Self::Id>, new_parent_id: Option<Self::ParentId>, new_file_id: Self::FileId) -> Self {
        Reg0000 {
            id: new_id,
            file_id: new_file_id,
            parent_id: new_parent_id,
            reg: get_field(&fields, 1),
            cod_ver: get_field(&fields, 2),
            tipo_escrit: get_field(&fields, 3),
            ind_sit_esp: get_field(&fields, 4),
            num_rec_anterior: get_field(&fields, 5),
            dt_ini: get_field(&fields, 6),
            dt_fin: get_field(&fields, 7),
            nome: get_field(&fields, 8),
            cnpj: get_field(&fields, 9),
            uf: get_field(&fields, 10),
            cod_mun: get_field(&fields, 11),
            suframa: get_field(&fields, 12),
            ind_nat_pj: get_field(&fields, 13),
            ind_ativ: get_field(&fields, 14),
        }
    }
}

impl Reg for Reg0000 {
    // fn values(&self) -> IndexMap<&'static str, Option<String>> {
    //     // let data = table::filter(id.eq(1)).first::<Self>(&mut DB_POOL.get().unwrap()).unwrap();
    //
    //     // let id: Option<String> = self.id.clone().map(|id| id.to_string());
    //     // let parent_id: Option<String> = self.parent_id.map(|id| id.to_string());
    //
    //     IndexMap::from([
    //         ("id", Some(String::from("asd"))),
    //         // ("id", self.id),
    //         // ("file_id", Some(self.file_id)),
    //         // ("parent_id", self.parent_id),
    //         // ("reg", self.reg.clone()),
    //         // ("cod_ver", self.cod_ver.clone()),
    //         // ("tipo_escrit", self.tipo_escrit.clone()),
    //         // ("ind_sit_esp", self.ind_sit_esp.clone()),
    //         // ("num_rec_anterior", self.num_rec_anterior.clone()),
    //         // ("dt_ini", self.dt_ini.clone()),
    //         // ("dt_fin", self.dt_fin.clone()),
    //         // ("nome", self.nome.clone()),
    //         // ("cnpj", self.cnpj.clone()),
    //         // ("uf", self.uf.clone()),
    //         // ("cod_mun", self.cod_mun.clone()),
    //         // ("suframa", self.suframa.clone()),
    //         // ("ind_nat_pj", self.ind_nat_pj.clone()),
    //         // ("ind_ativ", self.ind_ativ.clone()),
    //     ])
    // }

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<Reg0000, Error>> + Send + 'a>> {
        Box::pin(async move {
            let new = Reg0000 {
                id: None,
                file_id: self.file_id,
                parent_id: self.parent_id,
                reg: self.reg.clone(),
                cod_ver: self.cod_ver.clone(),
                tipo_escrit: self.tipo_escrit.clone(),
                ind_sit_esp: self.ind_sit_esp.clone(),
                num_rec_anterior: self.num_rec_anterior.clone(),
                dt_ini: self.dt_ini.clone(),
                dt_fin: self.dt_fin.clone(),
                nome: self.nome.clone(),
                cnpj: self.cnpj.clone(),
                uf: self.uf.clone(),
                cod_mun: self.cod_mun.clone(),
                suframa: self.suframa.clone(),
                ind_nat_pj: self.ind_nat_pj.clone(),
                ind_ativ: self.ind_ativ.clone(),
            };

            diesel::insert_into(table)
                .values(&new)
                .execute(&mut DB_POOL.get().unwrap())?
        })
    }
}
