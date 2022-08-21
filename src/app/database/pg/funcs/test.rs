use diesel::PgConnection;
use super::util_types::PgCurrentUser;

use super::{
    Adapter,
    super::Pg,
};

#[test]
fn funcs() {
    let maybe_pg = Pg::init();

    let pg = match maybe_pg {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            assert!(false);
            return;
        }
    };

    current_user(&pg);
}

fn current_user(pg: &Pg) {
    let current_user = pg.with_conn::<_, PgCurrentUser>(|pg_conn: &PgConnection| {
        let pg_current_user = Adapter::new(pg_conn).current_user()?;
        Ok(pg_current_user)
    });

    assert!(current_user.is_ok());
    assert_eq!(
        current_user.unwrap().current_user,
        "caelid"
    );
}
