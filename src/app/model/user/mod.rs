use chrono::naive::NaiveDateTime;
use crate::schema::users;
use super::prelude::*;

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub password: Option<String>,
    pub email: String,
    pub username: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn insert(conn: &PgConnection, username: &str, email: &str, password: &str) -> Result<User, DieselError> {
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
