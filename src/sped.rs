use crate::models::traits::Model;

pub fn create_registry_model(
    reg: &str,
    fields: Vec<&str>,
    parent_id: Option<i32>,
    file_id: i32,
) -> Option<Box<dyn Model>> {
    crate::models::registry::create_model(reg, fields, None, parent_id, file_id)
}

pub async fn handle_line(
    line: &str,
    parent_id: i32,
    file_id: i32,
) -> Result<Option<i32>, diesel::result::Error> {
    let fields = line.split("|").collect::<Vec<&str>>();
    let reg_code = fields.get(1).unwrap_or(&"");

    if let Some(factory) = create_registry_model(reg_code.to_lowercase().as_str(), fields, Some(parent_id), file_id) {
        let reg: Box<dyn Model> = factory;

        match reg.save().await {
            Ok(result) => Ok(Some(result)),
            Err(error) => Err(error),
        }
    } else {
        Ok(None)
    }
}
