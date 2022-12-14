use crate::schema::users;
use super::prelude::*;

impl User {
    /// Returns `User` if provided `uname` and `pw` are valid.
    pub fn authenticate_via_uname_pw(conn: &mut PgConnection, uname: &str, pw: &str) -> UserResult<Self> {
        use pg::funcs;

        let result = users::table.filter(funcs::authenticate_user_via_uname_pw(uname, pw))
            .first::<Self>(conn);

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                if let DieselError::NotFound = e {
                    log::info!("Failed to authenticate username, '{}', via credentials", uname);
                    Err(Error::AuthenticationFailure)
                } else {
                    log::error!("Failed to authenticate username, '{}', due to: {}", uname, e);
                    Err(Error::Diesel(e))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[actix_web::test]
    async fn authenticate_via_uname_pw() {
        use super::super::prelude::*;

        let pg = pg::PgConnPool::init().expect("Test failed with error");

        let authenticated_user = pg.with_conn::<_, User>(|conn| {
            let email = "cthulhu@ryleh.com";
            let uname = "cthulhu";
            let pw = "randolphcarter!";

            let values = (
                users::email.eq(email),
                users::username.eq(uname),
                users::password.eq(pw),
            );

            let _ = diesel::insert_into(User::table())
                .values(&values)
                .execute(conn)?;

            let auth_user = User::authenticate_via_uname_pw(conn, uname, pw)?;

            Ok(auth_user)
        }).await;

        assert!(authenticated_user.is_ok());
    }
}

