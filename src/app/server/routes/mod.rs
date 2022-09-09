use actix_web::web;
use super::handlers::{base, internal, sub};

pub fn routes(cfg: &mut web::ServiceConfig) {
    // base
    cfg.service(web::scope("/")
        .route("", web::get().to(base::index))
    );

    // /-
    cfg.service(internal::routes());

    // sub
    cfg.service(web::scope("/s")
        .route("/{sub}", web::get().to(sub::test))
    );
}

