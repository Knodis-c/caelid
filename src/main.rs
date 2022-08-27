#[macro_use] extern crate diesel;
#[macro_use] extern crate proc_macros;
use dotenv;

/// Core business logic of the application.
mod app;

/// Auxiliary modules that isn't tightly coupled to core business logic.
mod internal;

/// Snapshot of the database schema.
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    internal::log::init_logger();
    app::server::init().await
}
