use proptest::prelude::*;
use surreal_id::{IdError, NewId};

mod user_id_integration;
use crate::user_id_integration::*;

proptest! {
    #[test]
    fn valid_id_and_table_always_creates_user_id(table in "users", id in "[a-f0-9\\-]{1,255}") {
        let full_id = format!("{}:⟨{}⟩", table, id);
        let result = UserId::new(&full_id);
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_table_name_returns_invalid_table_error(table in "[a-z]{5,9}", id in "[a-f0-9\\-]{1,255}") {
        if table != "users" {
            let full_id = format!("{}:⟨{}⟩", table, id);
            let result = UserId::new(&full_id);
            assert!(matches!(result, Err(IdError::InvalidTable(_, _))));
        }
    }

    #[test]
    fn empty_id_returns_id_cannot_be_empty_error(table in "users", id in "") {
        let full_id = format!("{}:⟨{}⟩", table, id);
        let result = UserId::new(&full_id);
        assert!(matches!(result, Err(IdError::IdCannotBeEmpty)));
    }

    #[test]
    fn invalid_id_format_returns_error(table in "users", id in "[a-f0-9\\-]{1,255}") {
        let full_id = format!("{}:{}", table, id);
        let result = UserId::new(&full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn correct_table_but_invalid_id_format_returns_error(table in "users", id in "[a-f0-9\\-]{1,255}") {
        let full_id = format!("{}:{}", table, id);
        let result = UserId::new(&full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn id_without_table_uses_default_and_succeeds(id in "[a-f0-9\\-]{1,255}") {
        let result = UserId::new(&id);
        assert!(result.is_ok());
    }
}
