use crate::errors::IdError;
use surrealdb::sql::Id;

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
    fn new<T: AsRef<str>>(id: T) -> Result<Self, IdError> {
        let id_ref = id.as_ref();

        let is_empty = id_ref.is_empty();
        if is_empty {
            return Err(IdError::IdCannotBeEmpty);
        }

        let mut split_at_colon = id_ref.splitn(2, ':');
        let table_part = split_at_colon.next().unwrap_or(Self::TABLE);
        let id_part = split_at_colon.next().unwrap_or(id_ref);
        let start_bracket_count = id_ref.chars().filter(|&c| c == '⟨').count();
        let end_bracket_count = id_ref.chars().filter(|&c| c == '⟩').count();
        let is_valid_format = id_part.starts_with('⟨')
            && id_part.ends_with('⟩')
            && start_bracket_count == 1
            && end_bracket_count == 1;

        if is_valid_format {
            let id_inner: String = id_part.chars().skip(1).take_while(|&c| c != '⟩').collect();
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
        }

        if id_ref.contains(':') {
            return Err(IdError::InvalidIdFormat(id_ref.to_string()));
        }

        Ok(Self::from_inner_id(id_part.to_string()))
    }

    /// Constructs an instance from an inner ID of type `T`.
    ///
    /// The inner ID is expected to implement the conversion into `surrealdb::sql::Id`.
    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self;

    /// Returns the table name associated with this ID type.
    fn table(&self) -> &'static str {
        Self::TABLE
    }

    /// Returns the inner ID as a string without the table name or colon, but including the brackets.
    fn id_with_brackets(&self) -> String {
        format!("⟨{}⟩", &self.id_without_brackets())
    }

    /// Returns the inner ID as a string without the table name, colon and brackets.
    fn id_without_brackets(&self) -> String {
        let original_id = self.get_inner_string();
        original_id
            .split(':')
            .next()
            .unwrap_or(&original_id)
            .chars()
            .filter(|&c| c != '⟨' && c != '⟩')
            .collect()
    }

    /// Provided by the implementer, returns the inner ID as a string.
    fn get_inner_string(&self) -> String;
}
