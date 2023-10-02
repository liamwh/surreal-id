use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::{opt::RecordId, sql::Id};

pub const USERS_TABLE: &str = "users";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserId(RecordId);

impl NewId for UserId {
    const TABLE: &'static str = USERS_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        UserId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.id.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    id: UserId,
    name: String,
}

#[cfg(test)]
mod tests {
    use surreal_id::IdError;
    use surrealdb::engine::local::Mem;
    use surrealdb::Surreal;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn id_with_brackets() {
        let id = "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩";
        let user_id = UserId::new(id);
        assert_eq!(
            user_id.unwrap().id_with_brackets(),
            "⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩",
        );
    }

    #[test]
    fn id_without_brackets() {
        let id = "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩";
        let user_id = UserId::new(id);
        assert_eq!(
            user_id.unwrap().id_without_brackets(),
            "fa77edc3-56ed-4208-9e0b-c0b1c32e2d34",
        );
    }

    #[test]
    fn table_part_returns_correct_table() {
        let id = "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩";
        let user_id = UserId::new(id);
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap().table(), USERS_TABLE);
    }

    #[test]
    fn valid_id_and_table_creates_user_id_successfully() {
        let id = "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩";
        let user_id = UserId::new(id);
        assert!(user_id.is_ok());
        if let Ok(UserId(RecordId {
            tb: table,
            id: id_part,
        })) = user_id
        {
            assert_eq!(table, USERS_TABLE);
            assert_eq!(
                id_part.to_string(),
                "⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩"
            );
        }
    }

    #[test]
    fn invalid_table_name_returns_invalid_table_error() {
        let fx_table_name = "invalid_table";
        let id = format!("{}:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩", fx_table_name);
        let result = UserId::new(id);
        assert_eq!(
            result,
            Err(IdError::InvalidTable(
                USERS_TABLE.to_string(),
                fx_table_name.to_string()
            ))
        );
    }

    #[test]
    fn invalid_id_format_returns_error() {
        let invalid_ids = vec![
            "users:fa77edc3-56ed-4208-9e0b-c0b1c32e2d34", // Missing brackets
            "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34", // Missing closing bracket
            "users:fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩", // Missing opening bracket
            "users:⟨fa77edc3-56ed-4208-9e0b-c0b1c32e2d34⟩ ", // Trailing space
            "users:⟨fa77edc3⟩56ed⟩",                      // Multiple ⟩
        ];

        for id in invalid_ids {
            let result = UserId::new(id);
            assert_eq!(result, Err(IdError::InvalidIdFormat(id.to_string())));
        }
    }

    #[test]
    fn valid_id_without_table_uses_default_table_and_succeeds() {
        let fx_input_id = "fa77edc3-56ed-4208-9e0b-c0b1c32e2d34";
        let user_id = UserId::new(fx_input_id);
        let expected_id = format!("⟨{}⟩", fx_input_id);
        assert!(user_id.is_ok());
        if let Ok(UserId(RecordId { tb: table, id })) = user_id {
            assert_eq!(table, USERS_TABLE);
            assert_eq!(id.to_string(), expected_id);
        }
    }

    #[test]
    fn empty_id_returns_id_cannot_be_empty_error() {
        let id = "";
        let result = UserId::new(id);
        assert_eq!(result, Err(IdError::IdCannotBeEmpty));
    }

    #[test]
    fn valid_table_but_empty_id_part_returns_id_cannot_be_empty_error() {
        let full_id = format!("{}:⟨{}⟩", USERS_TABLE, "");
        let result = UserId::new(full_id);
        assert!(matches!(result, Err(IdError::IdCannotBeEmpty)));
    }

    #[tokio::test]
    async fn valid_user_object_saves_to_database_successfully_with_specified_table_syntax() {
        let fx_user = User {
            id: UserId::new("fa77edc3-56ed-4208-9e0b-c0b1c32e2d34").unwrap(),
            name: "John Doe".to_string(),
        };
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let result = db.create(USERS_TABLE).content(&fx_user).await;
        assert!(result.is_ok(), "Error: {:?}", result);
        let user: User = result.unwrap().remove(0);
        assert_eq!(fx_user, user)
    }
}
