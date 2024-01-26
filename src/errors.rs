#[derive(thiserror::Error, Debug, PartialEq)]
pub enum IdError {
    #[error("invalid id format: {0}")]
    InvalidIdFormat(String),

    #[error("invalid table, want: {0}, got: {1}")]
    InvalidTable(String, String),

    #[error("id cannot be empty")]
    IdCannotBeEmpty,
}
