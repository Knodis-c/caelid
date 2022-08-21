use diesel::{
    query_dsl::RunQueryDsl,
    PgConnection,
    result::Error as DieselError,
    sql_query
};

#[cfg(test)]
mod test;

/// Contains (de)serialization targets for data that comes from postgres.
pub mod util_types;

/// Adapter that borrows a connection to postgres, allowing for raw SQL execution,
/// returning data that is serialized to Rust-types that implement `QueryableByName` from diesel.
/// See the `util_types` module for examples.
pub struct Adapter<'a> {
    conn: &'a PgConnection
}

pub type AdapterResult<T> = Result<T, DieselError>;

impl<'a> Adapter<'a> {
    /// Return an `Adapter` that wraps around a borrowed `PgConnection`.
    fn new(conn: &'a PgConnection) -> Self {
        Self { conn }
    }

    /// Returns the active user of the postgres client.
    fn current_user(&self) -> AdapterResult<util_types::PgCurrentUser> {
        sql_query("SELECT current_user").get_result(self.conn)
    }

    //fn authenticate_user_via_password(&self, username: &str, password: &str) -> bool {
        //let statement = sql_query("SELECT authenticate_user_via_password(?, ?)")
            //.bind::<sql_types::VarChar, _>(username)
            //.bind::<sql_types::VarChar, _>(password);

        //dbg!(&statement);

        //todo!();
        //true
    //}
}

