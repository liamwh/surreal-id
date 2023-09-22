use proptest::prelude::*;
use surreal_id::{IdError, NewId};

mod user_id_integration;
use crate::user_id_integration::*;

proptest! {
    #[test]
    fn valid_id_and_table_always_creates_user_id(table in "users", id in "[a-f0-9\\-:;@#$%^&*()_+={}\\[\\]~]{1,255}") {
        let full_id = format!("{}:⟨{}⟩", table, id);
        let result = UserId::new(full_id);
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_table_name_returns_invalid_table_error(table in "[a-z]{5,9}", id in "[a-f0-9\\-]{1,255}") {
        if table != "users" {
            let full_id = format!("{}:⟨{}⟩", table, id);
            let result = UserId::new(full_id);
            assert!(matches!(result, Err(IdError::InvalidTable(_, _))));
        }
    }

    #[test]
    fn multiple_opening_brackets_should_fail(table in "users", id in "[a-f0-9\\-]{1,253}") {
        let insert_pos = rand::random::<usize>() % id.len();
        let full_id = format!("{}:⟨{}⟨{}⟩", table, &id[0..insert_pos], &id[insert_pos..]);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn multiple_closing_brackets_should_fail(table in "users", id in "[a-f0-9\\-]{1,253}") {
        let insert_pos = rand::random::<usize>() % id.len();
        let full_id = format!("{}:⟨{}⟩{}⟩", table, &id[0..insert_pos], &id[insert_pos..]);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn opening_backet_before_colon_should_fail(table in "users", id in "[a-f0-9\\-]{1,253}") {
        let insert_pos = rand::random::<usize>() % table.len();
        let full_id = format!("{}⟨{}:⟨{}⟩", &table[0..insert_pos], &table[insert_pos..], id);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn closing_backet_before_colon_should_fail(table in "users", id in "[a-f0-9\\-]{1,253}") {
        let insert_pos = rand::random::<usize>() % table.len();
        let full_id = format!("{}⟩{}:{}", &table[0..insert_pos], &table[insert_pos..], id);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn invalid_id_format_returns_error(table in "users", id in "[a-f0-9\\-]{1,255}") {
        let full_id = format!("{}:{}", table, id);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn correct_table_but_invalid_id_format_returns_error(table in "users", id in "[a-f0-9\\-]{1,255}") {
        let full_id = format!("{}:{}", table, id);
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }

    #[test]
    fn id_without_table_uses_default_and_succeeds(id in "[a-f0-9\\-]{1,255}") {
        let result = UserId::new(id);
        assert!(result.is_ok());
    }

    #[test]
    fn id_without_table_but_starts_with_colon_should_fail(id in ":[a-f0-9\\-]{1,255}") {
        let result = UserId::new(id);
        assert!(matches!(result, Err(IdError::InvalidIdFormat(_))));
    }
}
