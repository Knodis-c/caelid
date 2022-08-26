use crate::schema::users;
use super::prelude::*;

mod authentication;
pub mod result;

mod prelude {
    pub use crate::schema::users;
    pub use super::User;
    pub use super::result::*; 
    pub use super::super::prelude::*;
}

#[skip_serializing_none]
#[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct User {
    pub id: i32,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    pub email: String,
    pub username: String,

    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,

    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<NaiveDateTime>,

    #[serde(with = "ts_seconds_option")]
    pub deleted_at: Option<NaiveDateTime>,
}

