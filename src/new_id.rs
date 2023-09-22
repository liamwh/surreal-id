use surrealdb::sql::Id;

use crate::errors::IdError;

/// Defines a trait for creating a new ID with table name and inner ID.
///
/// The trait provides a default implementation for `new`, which takes a string `id`
/// and validates its structure before converting it to the appropriate ID type.
///
/// Implementers of this trait must specify the table name via the associated constant `TABLE`
/// and implement `from_inner_id` to specify how to construct the ID type from the inner ID.
pub trait NewId: Sized {
    /// The table name associated with this ID type.
    const TABLE: &'static str;

    /// Creates a new instance of the implementing type by parsing and validating the given `id` string.
    ///
    /// # Errors
    ///
    /// Returns an error if the `id` string is empty, malformed, or doesn't match the expected table name.
    fn new(id: &str) -> Result<Self, IdError> {
        if id.is_empty() {
            return Err(IdError::IdCannotBeEmpty);
        }

        let mut split_at_colon = id.splitn(2, ':');
        let table_part = split_at_colon
            .next()
            .ok_or(IdError::InvalidIdFormat("Missing table part".to_string()))?;

        if let Some(id_part) = split_at_colon.next() {
            if id_part.starts_with('⟨') && id_part.ends_with('⟩') {
                let id_inner: String = id_part
                    .chars()
                    .skip(1)
                    .take(id_part.chars().count() - 2)
                    .collect();

                if id_inner.is_empty() {
                    return Err(IdError::IdCannotBeEmpty);
                }

                if table_part != Self::TABLE {
                    return Err(IdError::InvalidTable(
                        Self::TABLE.to_string(),
                        table_part.to_string(),
                    ));
                }

                return Ok(Self::from_inner_id(id_inner));
            } else {
                return Err(IdError::InvalidIdFormat(id.to_string()));
            }
        }

        Ok(Self::from_inner_id(id.to_string()))
    }

    /// Constructs an instance from an inner ID of type `T`.
    ///
    /// The inner ID is expected to implement the conversion into `surrealdb::sql::Id`.
    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self;
}
