#[cfg(feature = "tonic")]
use tonic::Status;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum IdError {
    #[error("invalid id format: {0}")]
    InvalidIdFormat(String),

    #[error("invalid table, want: {0}, got: {1}")]
    InvalidTable(String, String),

    #[error("id cannot be empty")]
    IdCannotBeEmpty,
}

#[cfg(feature = "tonic")]
impl From<IdError> for tonic::Status {
    fn from(error: IdError) -> Self {
        match error {
            IdError::InvalidIdFormat(_) => Status::invalid_argument(error.to_string()),
            IdError::InvalidTable(_, _) => Status::invalid_argument(error.to_string()),
            IdError::IdCannotBeEmpty => Status::invalid_argument(error.to_string()),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "tonic")]
mod tonic_tests {
    use super::*;
    use tonic::Status;

    #[test]
    fn test_id_error_conversion_invalid_id_format() {
        let error = IdError::InvalidIdFormat("invalid_format".to_string());
        let status: Status = error.into();

        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "invalid id format: invalid_format");
    }

    #[test]
    fn test_id_error_conversion_invalid_table() {
        let error = IdError::InvalidTable("expected_table".to_string(), "actual_table".to_string());
        let status: Status = error.into();

        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(
            status.message(),
            "invalid table, want: expected_table, got: actual_table"
        );
    }

    #[test]
    fn test_id_error_conversion_id_cannot_be_empty() {
        let error = IdError::IdCannotBeEmpty;
        let status: Status = error.into();

        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert_eq!(status.message(), "id cannot be empty");
    }
}
