use diesel::{
    dsl::{select, sql},
    query_dsl::RunQueryDsl,
    PgConnection,
    result::Error as DieselError,
    sql_types::{Varchar, Bool},
};

#[cfg(test)]
mod test;

/// Adapter that borrows a connection to postgres, allowing for raw SQL execution,
/// returning data that is serialized to Rust-types that implement `QueryableByName` from diesel.
/// See the `util_types` module for examples.
pub struct Adapter<'a> {
    conn: &'a PgConnection
}

pub type AdapterResult<T> = Result<T, DieselError>;

sql_function! {
    #[sql_name = "authenticate_user_via_password"]
    fn authenticate_user_via_password(username: Varchar, password: Varchar) -> Bool;
}

impl<'a> Adapter<'a> {
    /// Return an `Adapter` that wraps around a borrowed `PgConnection`.
    fn new(conn: &'a PgConnection) -> Self {
        Self { conn }
    }

    /// Returns the active user of the postgres client.
    fn current_user(&self) -> AdapterResult<String> {
        select(sql::<Varchar>("current_user"))
            .get_result(self.conn)
    }

    /// Returns authentication info of user provided username and password.
    fn authenticate_user_via_password(&self, username: &str, password: &str) -> AdapterResult<bool> {
        select(authenticate_user_via_password(username, password))
            .get_result::<bool>(self.conn)
    }
}

