use diesel::{QueryableByName, sql_types};

#[derive(QueryableByName, PartialEq, Debug)]
pub struct PgCurrentUser {
    #[sql_type = "sql_types::VarChar"]
    pub current_user: String
}
