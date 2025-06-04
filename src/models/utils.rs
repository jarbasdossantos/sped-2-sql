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
}
