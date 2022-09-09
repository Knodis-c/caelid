mod prelude {
    pub use actix_web::{
        HttpResponse,
        HttpRequest,
        http::header::ContentType,
        web,
        Result,
    };
}

pub mod base;
pub mod internal;
pub mod sub;
