pub(crate) fn get_field(fields: &Vec<&str>, field: usize) -> Option<String> {
    fields.get(field).and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) })
}

pub(crate) fn to_date(date: Option<String>) -> Option<String> {
    date.and_then(|date_str| {
        if date_str.contains('/') {
            if date_str.len() != 10 {
                return None;
            }

            let parts: Vec<&str> = date_str.split('/').collect();
            if parts.len() != 3 {
                return None;
            }

            let day = parts[0];
            let month = parts[1];
            let year = parts[2];

            if day.len() != 2 || month.len() != 2 || year.len() != 4 {
                return None;
            }

            Some(format!("{}-{}-{}", year, month, day))
        } else if date_str.len() == 8 {
            let day = &date_str[0..2];
            let month = &date_str[2..4];
            let year = &date_str[4..8];

            Some(format!("{}-{}-{}", year, month, day))
        } else {
            None
        }
    })
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
    fn test_get_field_empty_string() {
        let fields = vec!["", "field2"];
        assert_eq!(get_field(&fields, 0), None);
        assert_eq!(get_field(&fields, 1), Some("field2".to_string()));
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
    fn test_convert_date_to_sqlite_format_valid() {
        // Test with separator format
        assert_eq!(
            to_date(Some("01/02/2023".to_string())),
            Some("2023-02-01".to_string())
        );
        assert_eq!(
            to_date(Some("31/12/2022".to_string())),
            Some("2022-12-31".to_string())
        );

        // Test without separator format
        assert_eq!(
            to_date(Some("01062020".to_string())),
            Some("2020-06-01".to_string())
        );
        assert_eq!(
            to_date(Some("31122022".to_string())),
            Some("2022-12-31".to_string())
        );
    }

    #[test]
    fn test_convert_date_to_sqlite_format_invalid() {
        assert_eq!(to_date(Some("2023-01-01".to_string())), None); // Wrong format
        assert_eq!(to_date(Some("01-02-2023".to_string())), None); // Wrong separator
        assert_eq!(to_date(Some("1/2/2023".to_string())), None); // Wrong length
        assert_eq!(to_date(Some("0106202".to_string())), None); // Wrong length without separator
        assert_eq!(to_date(None), None); // None input
    }
}