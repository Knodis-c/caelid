use dotenv;

mod app;
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    app::server::init().await
}
