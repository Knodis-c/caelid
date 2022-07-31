use actix_web::{App, HttpServer};
use dotenv;
use std::net::{Ipv4Addr, SocketAddrV4};

mod handlers;
mod routes;
mod shared;
mod assets;

pub const DEFAULT_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: &'static str = "3000";

pub async fn init() -> std::io::Result<()> {
    use shared::template;

    dotenv::dotenv().ok();

    let app_factory = || {
        let templating_engine = template::Engine::init().unwrap();

        App::new()
            .app_data(templating_engine)
            .configure(assets::static_assets)
            .configure(routes::routes)
    };

    let host = dotenv::var("HOST")
        .unwrap_or(DEFAULT_HOST.to_owned())
        .parse::<Ipv4Addr>()
        .unwrap();

    let port = dotenv::var("PORT")
        .unwrap_or(DEFAULT_PORT.to_owned())
        .parse::<u16>()
        .unwrap();

    let socketaddr = SocketAddrV4::new(host, port);

    HttpServer::new(app_factory)
        .bind(socketaddr)?
        .run() 
        .await
}
