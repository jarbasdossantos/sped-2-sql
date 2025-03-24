pub(crate) fn get_field(fields: &Vec<&str>, field: usize) -> Option<String> {
    fields.get(field).map(|s| s.to_string())
}

pub(crate) fn get_date(fields: &Vec<&str>, field: usize) -> Option<chrono::NaiveDate> {
    let date = get_field(fields, field);

    if date.is_none() {
        return None;
    }

    date.map(|s| chrono::NaiveDate::parse_from_str(s.as_str(), "%d%m%Y").unwrap_or_default())
}
