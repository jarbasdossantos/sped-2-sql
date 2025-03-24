/*
 * Get a field from a vector of strings
 * 
 * @param fields: &Vec<&str> - The vector of strings
 * @param field: usize - The index of the field to get
 * @return Option<String> - The field as a string, or None if the index is out of bounds
 */
pub(crate) fn get_field(fields: &Vec<&str>, field: usize) -> Option<String> {
    fields.get(field).map(|s| s.to_string())
}

/*
 * Get a date from a vector of strings
 * 
 * @param fields: &Vec<&str> - The vector of strings
 * @param field: usize - The index of the field to get
 * @return Option<chrono::NaiveDate> - The date as a chrono::NaiveDate, or None if the index is out of bounds
 */
pub(crate) fn get_date(fields: &Vec<&str>, field: usize) -> Option<chrono::NaiveDate> {
    let date = get_field(fields, field);

    if date.is_none() {
        return None;
    }

    date.map(|s| chrono::NaiveDate::parse_from_str(s.as_str(), "%d%m%Y").unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_field_valid_index() {
        let fields = vec!["field1", "field2", "field3"];
        assert_eq!(get_field(&fields, 0), Some("field1".to_string()));
        assert_eq!(get_field(&fields, 1), Some("field2".to_string()));
        assert_eq!(get_field(&fields, 2), Some("field3".to_string()));
    }

    #[test]
    fn test_get_field_invalid_index() {
        let fields = vec!["field1", "field2"];
        assert_eq!(get_field(&fields, 2), None);
        assert_eq!(get_field(&fields, 10), None);
    }

    #[test]
    fn test_get_field_empty_vec() {
        let fields: Vec<&str> = vec![];
        assert_eq!(get_field(&fields, 0), None);
    }

    #[test]
    fn test_get_date_valid_format() {
        let fields = vec!["field1", "01012023", "field3"];
        let expected_date = chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(get_date(&fields, 1), Some(expected_date));
    }

    #[test]
    fn test_get_date_invalid_format() {
        let fields = vec!["field1", "01-01-2023", "field3"]; // Formato inválido
        let default_date = chrono::NaiveDate::default();
        assert_eq!(get_date(&fields, 1), Some(default_date));
    }

    #[test]
    fn test_get_date_out_of_bounds() {
        let fields = vec!["field1", "field2"];
        assert_eq!(get_date(&fields, 2), None);
    }

    #[test]
    fn test_get_date_empty_vector() {
        let fields: Vec<&str> = vec![];
        assert_eq!(get_date(&fields, 0), None);
    }
}
