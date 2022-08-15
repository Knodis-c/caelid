use actix_web::{App, HttpServer, web};
use crate::app::{
    database::pg::{CONNS_PER_WORKER, Pg},
    template_engine
};
use middleware::request_info::RequestInfoFactory;
use std::net::{Ipv4Addr, SocketAddrV4};

pub mod assets;
mod handlers;
mod middleware;
mod routes;

pub const DEFAULT_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: &'static str = "3000";

pub async fn init() -> std::io::Result<()> {
    let app_factory = || {
        let pg = Pg::init().map(|pool| web::Data::new(pool)).unwrap();
        let template_engine = template_engine::Engine::init()
            .map(|engine| web::Data::new(engine))
            .unwrap();

        App::new()
            .app_data(template_engine)
            .app_data(pg)
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

    log::info!("Initialized {} Postgres connections per Actix worker", CONNS_PER_WORKER);
    log::info!("Listening on {}", socketaddr.to_string());

    HttpServer::new(app_factory)
        .bind(socketaddr)?
        .run() 
        .await
}
