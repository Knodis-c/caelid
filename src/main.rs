#[macro_use] extern crate diesel;
use dotenv;

mod app;
mod lib;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    lib::log::init_logger();
    app::server::init().await
}
