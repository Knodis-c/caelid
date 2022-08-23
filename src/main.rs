#[macro_use] extern crate diesel;
use dotenv;

mod app;
mod internal;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    internal::log::init_logger();
    app::server::init().await
}
