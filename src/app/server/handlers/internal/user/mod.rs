use actix_web::{web, Scope};

mod ops;

pub fn routes() -> Scope {
    web::scope("/user").service(
        web::resource("/{id}").name("internal/user/ops")
            .route(web::get().to(ops::read))
    )
}
