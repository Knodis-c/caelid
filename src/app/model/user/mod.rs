use chrono::naive::NaiveDateTime;
use crate::schema::users;
use super::prelude::*;

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

    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<NaiveDateTime>,

    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<NaiveDateTime>,

    #[serde(with = "ts_seconds_option")]
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn insert(conn: &PgConnection, username: &str, email: &str, password: Option<&str>) -> Result<User, DieselError> {
        let values = (
            users::columns::username.eq(username),
            users::columns::email.eq(email),
            users::columns::password.eq(password)
        );

        diesel::insert_into(Self::table())
            .values(&values)
            .get_result(conn)
    }

    fn table() -> users::table {
        users::table
    }
}
