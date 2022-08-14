use actix_web::{App, HttpServer};
use crate::app::database::pg::Pg;
use middleware::request_info::RequestInfoFactory;
use shared::template;
use std::net::{Ipv4Addr, SocketAddrV4};

mod assets;
mod handlers;
mod middleware;
mod routes;
mod shared;

pub const DEFAULT_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: &'static str = "3000";

pub async fn init() -> std::io::Result<()> {
    let pg = Pg::init().unwrap();

    let app_factory = || {
        let templating_engine = template::Engine::init().unwrap();

        App::new()
            .app_data(templating_engine)
            .wrap(RequestInfoFactory::new()) // This must be the innermost middleware i.e. it must go last.
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

    log::info!("Listening on {}", socketaddr.to_string());
    log::info!("Postgres configurations: {}", &pg);

    HttpServer::new(app_factory)
        .bind(socketaddr)?
        .run() 
        .await
}
