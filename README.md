[![Crates.io](https://img.shields.io/crates/v/surreal_id.svg)](https://crates.io/crates/surreal_id)
[![Documentation](https://docs.rs/surreal-id/badge.svg)](https://docs.rs/surreal-id/)
[![Codecov](https://codecov.io/github/liamwh/surreal-id/coverage.svg?branch=main)](https://codecov.io/gh/liamwh/surreal-id)
[![Dependency status](https://deps.rs/repo/github/liamwh/surreal-id/status.svg)](https://deps.rs/repo/github/liamwh/surreal-id)

## surreal-id

The `surreal-id` crate offers a standardized way to create and validate IDs in your application for usage with SurrealDB. By defining the `NewId` trait, the crate streamlines the ID creation process, handling errors like malformed or empty IDs, and ensures consistency with associated table names. This also enables developers to serialize and deserialize custom ID types from SurrealDB whilst still retrieving the ID field for usage in your application, making it using custom ID types and logic with SurrealDB seamless.

## Example

```rs
use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::{opt::RecordId, sql::Id};

pub const USERS_TABLE: &str = "users";

#[derive(Serialize, Deserialize)]
pub struct UserId(RecordId);

impl NewId for UserId {
    const TABLE: &'static str = USERS_TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        UserId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }
}
```

> NOTE: For most use cases, most of the above code is boilerplate that could be eliminated with a procerdural macro, where the only thing specified is the table name.

Now you can instantiate the `UserId` type using `new`, and use it in your struct with SurrealDB like so:

```rs
#[derive(Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
}

let user_to_be_created = User {
    id: UserId::new("fa77edc3-56ed-4208-9e0b-c0b1c32e2d34").unwrap(),
    name: "John Doe".to_string(),
};

let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let create_result = db.create(USERS_TABLE).content(&user_to_be_created).await;
let retrieved_user: User = create_result.unwrap().remove(0);

assert_eq!(user_to_be_created, retrieved_user)
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
