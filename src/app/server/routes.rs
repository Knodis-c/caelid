use actix_web::web;
use super::handlers::{base, sub};

pub fn routes(cfg: &mut web::ServiceConfig) {
    // Base
    cfg.service(web::scope("/")
        .route("", web::get().to(base::index))
    );

    // Sub
    cfg.service(web::scope("/s")
        .route("/{sub}", web::get().to(sub::test))
    );
}

