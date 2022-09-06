use actix_web::{web, Scope};

mod user;

pub fn routes() -> Scope {
    web::scope("/-")

        // /user
        .service(user::routes())
}
