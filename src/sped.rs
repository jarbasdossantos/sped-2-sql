use crate::models::traits::Model;
use crate::SpedType;

pub fn create_registry_model(
    reg: &str,
    fields: Vec<&str>,
    parent_id: Option<i32>,
    file_id: i32,
    sped_type: SpedType
) -> Option<Box<dyn Model>> {
    crate::models::registry::create_model(reg, fields, None, parent_id, file_id, sped_type)
}

pub async fn handle_line(
    line: &str,
    parent_id: i32,
    file_id: i32,
    sped_type: SpedType
) -> Result<Option<i32>, diesel::result::Error> {
    let fields = line.split("|").collect::<Vec<&str>>();
    let reg_code = fields.get(1).unwrap_or(&"");

    if let Some(factory) = create_registry_model(
        reg_code.to_lowercase().as_str(),
        fields,
        Some(parent_id),
        file_id,
        sped_type
    ) {
        let model: Box<dyn Model> = factory;

        match model.save().await {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(error),
        }
    } else {
        Ok(None)
    }
}
