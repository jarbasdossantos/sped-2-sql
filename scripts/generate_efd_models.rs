use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Lista de todos os schemas EFD disponíveis
    let schema_files = fs::read_dir("src/schemas/")?
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            file_name.starts_with("efd_") && file_name.ends_with(".rs")
        })
        .map(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            let model_name = file_name.trim_start_matches("efd_").trim_end_matches(".rs");
            model_name.to_string()
        })
        .collect::<Vec<_>>();

    eprintln!("{schema_files:#?}");

    // Modelos já existentes
    let existing_models = fs::read_dir("src/models/efd/")?
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            file_name.starts_with("efd_") && file_name.ends_with(".rs")
        })
        .map(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            let model_name = file_name.trim_start_matches("efd_").trim_end_matches(".rs");
            model_name.to_string()
        })
        .collect::<Vec<_>>();

    // Gerar modelos faltantes
    for schema in &schema_files {
        if !existing_models.contains(schema) {
            generate_model(schema)?;
        }
    }

    // Atualizar mod.rs
    update_mod_file(&schema_files)?;

    // Atualizar registry.rs
    update_registry_file(&schema_files)?;

    // Atualizar file_structure.rs
    // update_file_structure(&schema_files)?;

    Ok(())
}

fn generate_model(schema: &str) -> std::io::Result<()> {
    // Ler o conteúdo do arquivo de schema
    let schema_path = format!("src/schemas/efd_{}.rs", schema.to_lowercase());
    let schema_content = fs::read_to_string(&schema_path)?;

    // Extrair os campos do schema (exemplo simplificado)
    let fields = extract_fields_from_schema(&schema_content);

    // Gerar o conteúdo do modelo com os campos extraídos
    let model_name = format!("Efd{}", schema.to_uppercase());
    let file_path = format!("src/models/efd/efd_{}.rs", schema.to_lowercase());

    if Path::new(&file_path).exists() {
        return Ok(());
    }

    let struct_fields = fields
        .iter()
        .map(|f| format!("    pub {}: Option<String>,", f))
        .collect::<Vec<_>>()
        .join("\n");

    let mut i = 1;

    let new_fields = fields
        .iter()
        .map(|f| {
            i += 1;
            format!("        {}: get_field(&fields, {}),", f, i)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let save_fields = fields
        .iter()
        .map(|f| format!("schema::{}.eq(&self.{}),", f, f))
        .collect::<Vec<_>>()
        .join("\n");

    let display_fields = fields
        .iter()
        .map(|f| format!("{}", f))
        .collect::<Vec<_>>()
        .join(", ");

    let content = format!(
        r#"use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::efd_{0}::efd_{0}::dsl as schema;
use crate::schemas::efd_{0}::efd_{0}::table;
use crate::{{impl_display_fields, register_model}};
use async_trait::async_trait;
use diesel::dsl::sql;
use diesel::prelude::Queryable;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use diesel::{{ExpressionMethods, Selectable}};
use diesel::{{QueryDsl, SelectableHelper}};
use serde::{{Serialize, Deserialize}};
use std::fmt;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schemas::efd_{0}::efd_{0}::dsl)]
pub struct {1} {{
    pub id: i32,
    pub file_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub reg: Option<String>,
{2}
}}

#[async_trait]
impl Model for {1} {{
    fn new(
        fields: Vec<&str>,
        new_id: Option<i32>,
        new_parent_id: Option<i32>,
        new_file_id: i32,
    ) -> Self {{
        {1} {{
            id: new_id.unwrap_or(0),
            file_id: Some(new_file_id),
            parent_id: new_parent_id,
            reg: fields.get(1).map(|s| s.to_string()),
            {3}
        }}
    }}

    async fn get(file_id: i32, parent_id: Option<i32>) -> Result<Vec<{1}>, Error> {{
        Ok(table
            .filter(schema::file_id.eq(&file_id))
            .filter(schema::parent_id.eq(parent_id.expect("Invalid parent id")))
            .select({1}::as_select())
            .load(&mut DB_POOL
                .get().unwrap())?)
    }}

    fn save<'a>(&'a self) -> Pin<Box<dyn Future<Output=Result<i32, Error>> + Send + 'a>> {{
        Box::pin(async move {{
            diesel::insert_into(table)
                .values((
                    schema::file_id.eq(&self.file_id),
                    schema::parent_id.eq(&self.parent_id),
                    schema::reg.eq(&self.reg.clone()),
            {4}
                ))
                .execute(&mut DB_POOL.get().unwrap())?;

            sql::<Integer>("SELECT last_insert_rowid()")
                .get_result::<i32>(&mut DB_POOL.get().unwrap())
        }})
    }}

    fn get_id(&self) -> Option<i32> {{
        Some(self.id)
    }}

    fn get_file_id(&self) -> Option<i32> {{
        self.file_id
    }}

    fn get_entity_name(&self) -> String {{
        "{1}".to_string()
    }}

    fn get_display_fields(&self) -> Vec<(String, String)> {{
        self.generate_display_fields()
    }}
}}

impl fmt::Display for {1} {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        self.display_format(f)
    }}
}}

impl_display_fields!({1}, [reg, {5}]);
register_model!({1}, "{0}");
"#,
        schema.to_lowercase(),
        model_name,
        struct_fields,
        new_fields,
        save_fields,
        display_fields
    );

    fs::write(&file_path, content)?;
    println!("Modelo gerado: {}", file_path);
    Ok(())
}

// Função auxiliar para extrair campos do schema
fn extract_fields_from_schema(schema_content: &str) -> Vec<String> {
    // Implementação simplificada - você precisará ajustar conforme necessário
    let mut fields = Vec::new();
    let lines: Vec<&str> = schema_content.lines().collect();

    for line in lines {
        if line.trim().starts_with("//") || !line.contains("->") {
            continue;
        }

        if let Some(field) = line.split_whitespace().next() {
            if !["id", "file_id", "parent_id", "reg"].contains(&field) {
                fields.push(field.to_string());
            }
        }
    }

    fields
}

fn update_mod_file(schemas: &[String]) -> std::io::Result<()> {
    let mod_path = "src/models/efd/mod.rs";
    let mut mod_content = String::new();

    // Adiciona o cabeçalho
    mod_content.push_str("// @generated automatically by generate_efd_models.rs\n\n");

    // Adiciona os módulos em ordem alfabética
    let mut sorted_schemas = schemas.to_vec();
    sorted_schemas.sort();

    for schema in &sorted_schemas {
        mod_content.push_str(&format!("pub mod efd_{};\n", schema.to_lowercase()));
    }

    fs::write(mod_path, mod_content)
}

fn update_registry_file(schemas: &[String]) -> std::io::Result<()> {
    let registry_path = "src/models/registry.rs";
    let mut registry_content = String::new();

    // Lê o conteúdo atual do arquivo
    if Path::new(registry_path).exists() {
        registry_content = fs::read_to_string(registry_path)?;
    }

    // Encontra a função register_models
    if let Some(idx) = registry_content.find("pub fn register_models() {") {
        let start_idx = registry_content[..idx].rfind('}').unwrap_or(0);
        let mut new_content = registry_content[..start_idx + 1].to_string();

        // Adiciona os registros em ordem alfabética
        let mut sorted_schemas = schemas.to_vec();
        sorted_schemas.sort();

        new_content.push_str("\n    // Registra os modelos EFD\n");
        for schema in &sorted_schemas {
            new_content.push_str(&format!(
                "    crate::models::efd::efd_{}::register();\n",
                schema.to_lowercase()
            ));
        }

        new_content.push_str("}");

        // Substitui o conteúdo antigo pelo novo
        if let Some(end_idx) = registry_content[start_idx..].find('}') {
            new_content.push_str(&registry_content[start_idx + end_idx + 1..]);
            fs::write(registry_path, new_content)?;
        }
    }

    Ok(())
}

fn update_file_structure(schemas: &[String]) -> std::io::Result<()> {
    let file_structure_path = "src/utils/file_structure.rs";
    let content = fs::read_to_string(file_structure_path)?;

    // Encontra o início e o fim do bloco FILE_STRUCTURE
    if let Some(start_idx) = content
        .find("pub static FILE_STRUCTURE: Lazy<IndexMap<&str, RegistryEntry>> = Lazy::new(|| {")
    {
        let start_block = start_idx + content[start_idx..].find('{').unwrap_or(0);
        let end_block = start_block + 1;

        let mut new_content = content[..start_block + 1].to_string();

        // Adiciona as entradas em ordem alfabética
        let mut sorted_schemas = schemas.to_vec();
        sorted_schemas.sort();

        for schema in &sorted_schemas {
            let model_name = format!("Efd{}", schema.to_uppercase());
            new_content.push_str(&format!(
                "\n        (\n            \"{}\",\n            RegistryEntry {{\n                level: 1,\n                load_model: create_loader!(crate::models::efd::efd_{}::{}),\n            }},\n        ),",
                schema.to_uppercase(),
                schema.to_lowercase(),
                model_name
            ));
        }

        // Adiciona o restante do conteúdo
        if let Some(end_idx) = content[end_block..].find("])") {
            new_content.push_str("\n    ]");
            new_content.push_str(&content[end_block + end_idx + 2..]);
            fs::write(file_structure_path, new_content)?;
        }
    }

    Ok(())
}
