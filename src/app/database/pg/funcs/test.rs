use super::util_types::PgCurrentUser;

use super::{
    Adapter,
    super::Pg,
};

#[test]
fn funcs() {
    let pg = Pg::init().expect("Test failed with error:");
    current_user(&pg);
}

fn current_user(pg: &Pg) {
    let pg_user = pg.with_conn::<_, PgCurrentUser>(|pg_conn| {
        let pg_current_user = Adapter::new(pg_conn).current_user()?;
        Ok(pg_current_user)
    }).expect("Test failed with error:");

    assert_eq!(
        pg_user.current_user,
        "caelid"
    );
}
