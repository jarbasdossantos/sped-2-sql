use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::Path;

static REG_MODELS_PREFIX: &str = "reg_";
static EFD_MODELS_PREFIX: &str = "efd_";
static MODELS_FOLDER: &str = "src/models/";
static REG_MODEL_PREFIX: &str = "Reg";
static EFD_MODEL_PREFIX: &str = "Efd";

fn main() -> std::io::Result<()> {
    // Ler o conteúdo do arquivo schemas.rs
    let mut schemas_content = String::new();
    fs::File::open("src/schemas.rs")
        .expect("Failed to open schemas.rs file")
        .read_to_string(&mut schemas_content)
        .expect("Failed to read schemas.rs file");

    // Extrair os nomes dos schemas usando regex
    let reg_re = Regex::new(r"diesel::table!\s*\{\s*(reg_[a-z0-9_]+)\s*\(")
        .expect("Failed to create regex for reg_");

    let efd_re = Regex::new(r"diesel::table!\s*\{\s*(efd_[a-z0-9_]+)\s*\(")
        .expect("Failed to create regex for efd_");

    // Extrair schemas que começam com reg_
    let reg_schemas = reg_re
        .captures_iter(&schemas_content)
        .map(|cap| {
            let full_name = cap[1].to_string();
            full_name.trim_start_matches(REG_MODELS_PREFIX).to_string()
        })
        .collect::<Vec<_>>();

    // Extrair schemas que começam com efd_
    let efd_schemas = efd_re
        .captures_iter(&schemas_content)
        .map(|cap| {
            let full_name = cap[1].to_string();
            full_name.trim_start_matches(EFD_MODELS_PREFIX).to_string()
        })
        .collect::<Vec<_>>();

    let existing_models = fs::read_dir(MODELS_FOLDER)
        .expect("Failed to read models folder")
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            file_name.starts_with(REG_MODELS_PREFIX) && file_name.ends_with(".rs")
        })
        .map(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            let model_name = file_name
                .trim_start_matches(REG_MODELS_PREFIX)
                .trim_end_matches(".rs");
            model_name.to_string()
        })
        .collect::<Vec<_>>();

    for schema in &efd_schemas {
        if !existing_models.contains(schema) {
            generate_model(schema, "efd_")?;
        }
    }

    for schema in &reg_schemas {
        if !existing_models.contains(schema) {
            generate_model(schema, "reg_")?;
        }
    }

    update_mod_file(&efd_schemas, "efd_");
    update_mod_file(&reg_schemas, "reg_");

    print!("EFD\n");
    print_models_registry(efd_schemas, "efd_");
    print!("ICMS IPI\n");
    print_models_registry(reg_schemas, "reg_");

    Ok(())
}

fn generate_model(schema: &str, schema_prefix: &str) -> std::io::Result<()> {
    // Ler o conteúdo do arquivo schemas.rs
    let mut schemas_content = String::new();
    fs::File::open("src/schemas.rs")
        .expect("Failed to open schemas.rs file")
        .read_to_string(&mut schemas_content)
        .expect("Failed to read schemas.rs file");

    let fields =
        extract_fields_from_schema(&schemas_content, &format!("{}{}", schema_prefix, schema));

    let folder = if schema_prefix == "efd_" {
        "efd/"
    } else {
        "icms_ipi/"
    };

    let prefix = if schema_prefix == "efd_" {
        EFD_MODEL_PREFIX
    } else {
        REG_MODEL_PREFIX
    };

    let model_name = format!("{}{}", prefix, schema.to_uppercase());
    let file_path = format!(
        "{}{}{}{}.rs",
        MODELS_FOLDER,
        folder,
        schema_prefix,
        schema.to_lowercase()
    );

    if Path::new(&file_path).exists() {
        return Ok(());
    }

    let struct_fields = fields
        .iter()
        .map(|f| format!("    pub {f}: Option<String>,"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut i = 1;

    let new_fields = fields
        .iter()
        .map(|f| {
            i += 1;
            format!("        {f}: get_field(&fields, {i}),")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let save_fields = fields
        .iter()
        .map(|f| format!("schema::{f}.eq(&self.{f}),"))
        .collect::<Vec<_>>()
        .join("\n");

    let display_fields = fields
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let content = format!(
        r#"use crate::database::DB_POOL;
use crate::models::traits::Model;
use crate::models::utils::get_field;
use crate::schemas::{6}{0}::dsl as schema;
use crate::schemas::{6}{0}::table;
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
#[diesel(table_name = crate::schemas::{6}{0}::dsl)]
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
        let mut conn = DB_POOL.get().unwrap();

        if let Some(id) = parent_id {{
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .filter(schema::parent_id.eq(&id))
                .select({1}::as_select())
                .load(&mut conn)?)
        }} else {{
            Ok(table
                .filter(schema::file_id.eq(&file_id))
                .select({1}::as_select())
                .load(&mut conn)?)
        }}
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
        display_fields,
        schema_prefix
    );

    fs::write(&file_path, content)?;
    println!("Modelo gerado: {file_path}");
    Ok(())
}

fn extract_fields_from_schema(schema_content: &str, schema_name: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut in_target_schema = false;
    let lines: Vec<&str> = schema_content.lines().collect();

    // Procurar pelo início da definição do schema
    let schema_start = format!("{schema_name} (id) {{");

    for line in lines {
        let trimmed = line.trim();

        // Verificar se estamos entrando na definição do schema alvo
        if !in_target_schema && line.contains(&schema_start) {
            in_target_schema = true;
            continue;
        }

        // Se encontrarmos o fechamento do bloco e estávamos no schema alvo, saímos
        if in_target_schema && trimmed == "}" {
            break;
        }

        // Se estamos no schema alvo e a linha contém uma definição de campo
        if in_target_schema
            && trimmed.contains("->")
            && (!trimmed.starts_with("id")
            && !trimmed.starts_with("file_id")
            && !trimmed.starts_with("parent_id")
            && !trimmed.starts_with("reg"))
        {
            if let Some(field) = trimmed.split_whitespace().next() {
                fields.push(field.to_string());
            }
        }
    }

    fields
}

fn update_mod_file(schema_files: &Vec<String>, model_prefix: &str) {
    let mut mod_file_content = String::new();

    let folder = if model_prefix == "efd_" {
        "efd/"
    } else {
        "icms_ipi/"
    };

    for schema in schema_files {
        mod_file_content.push_str(&format!(
            "pub mod {}{};\n",
            REG_MODELS_PREFIX,
            schema.to_lowercase()
        ));
    }

    fs::write(format!("{MODELS_FOLDER}{folder}mod.rs"), mod_file_content);
}

#[allow(dead_code)]
fn print_models_registry(schema_files: Vec<String>, model_prefix: &str) {
    let mut registry_content = String::new();

    for schema in schema_files {
        registry_content.push_str(&format!(
            "    crate::models::{}::{}{}::register();\n",
            &model_prefix[0..3],
            model_prefix,
            schema.to_lowercase()
        ));
    }

    println!("{}", registry_content);
}
