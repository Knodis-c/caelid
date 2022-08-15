use chrono::naive::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime
}
