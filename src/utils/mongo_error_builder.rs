use mongodb::error::{Error, ErrorKind};

pub fn map_mongo_error(error: Error) -> String {
    match error.kind.as_ref() {
        ErrorKind::BulkWrite(bwf) => {
            if let Some(write_errors) = &bwf.write_errors {
                if !write_errors.is_empty() && write_errors[0].code == 11000 {
                    return write_errors[0].message.clone();
                }
            }
        }
        _ => {}
    }
    "An error occurred while processing your request".to_string()
}
