use dotenv;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    app::server::init().await
}
