use super::{
    Adapter,
    super::Pg,
};

#[test]
fn funcs() {
    let pg = Pg::init().expect("Test failed with error");

    current_user(&pg);
    authenticate_user_via_password(&pg);
}

fn authenticate_user_via_password(pg: &Pg) {
    use crate::app::model::{
        prelude::*, 
        user::User,
    };
    use crate::schema::users::columns::*;

    let is_authenticated = pg.with_conn::<_, bool>(|pg_conn| {
        let values = (
            email.eq("cthulhu@ryleh.com"),
            username.eq("cthulhu"),
            password.eq("!@#!@#"),
        );

        let new_user = diesel::insert_into(User::table())
            .values(&values)
            .get_result::<User>(pg_conn)?;

        let uname = &new_user.username;
        let pw = &new_user.password.unwrap_or("".to_string());

        dbg!("{}, {}", &uname, &pw);

        let authenticated = Adapter::new(pg_conn)
            .authenticate_user_via_password(uname, pw)?;

        Ok(authenticated)
    }).expect("Test failed with error");

    assert!(is_authenticated);
}

fn current_user(pg: &Pg) {
    let pg_user = pg.with_conn::<_, String>(|pg_conn| {
        let pg_current_user = Adapter::new(pg_conn).current_user()?;
        Ok(pg_current_user)
    }).expect("Test failed with error");

    assert_eq!(
        pg_user,
        "caelid"
    );
}
