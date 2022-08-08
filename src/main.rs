use dotenv;

mod app;
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    lib::log::init_logger();
    app::server::init().await
}
