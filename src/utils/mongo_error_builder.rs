use mongodb::error::{Error, ErrorKind, WriteFailure};
use regex::Regex;

pub fn map_mongo_error(error: Error) -> String {
    match error.kind.as_ref() {
        ErrorKind::Write(e) => match e {
            WriteFailure::WriteConcernError(e) => {
                return e.message.clone();
            }
            WriteFailure::WriteError(e) => {
                return extract_duplicate_value(&e.message).unwrap_or(e.message.clone());
            }
            _ => {}
        },
        _ => {}
    }
    "An error occurred while processing your request".to_string()
}

fn extract_duplicate_value(error_message: &str) -> Result<String, String> {
    let regex_pattern = r#"dup key: \{ ([^:]+): "([^"]+)" \}"#;
    let re = Regex::new(regex_pattern).map_err(|e| e.to_string())?;

    if let Some(captures) = re.captures(error_message) {
        let field_name = captures.get(1).unwrap().as_str().to_string();
        let field_value = captures.get(2).unwrap().as_str().to_string();
        Ok(format!(
            "{} with value {} already exists",
            field_name, field_value
        ))
    } else {
        Err("No duplicate value found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_duplicate_value() {
        let error_message = "E11000 duplicate key error collection: test.users index: email_1 dup key: { email: \"test@test\" }";
        let result = extract_duplicate_value(error_message).unwrap();
        assert_eq!(result, "email with value test@test already exists");
    }
}
