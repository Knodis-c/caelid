use actix_files as fs;
use actix_web::{
    App,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    error::Error,
    http::KeepAlive,
    HttpServer,
    self,
    web
};
use crate::app::{
    database::pg::{CONNS_PER_WORKER, PgConnPool},
    template_engine
};
use middleware::request_info::RequestInfoFactory;
use std::{
    time::Duration,
    net::{Ipv4Addr, SocketAddrV4}
};

mod handlers;
mod middleware;
mod routes;

pub const PUBLIC_PATH: &'static str = "/static";
pub const STATIC_ASSETS_PATH: &'static str = "./public";

pub fn static_assets(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new(PUBLIC_PATH, STATIC_ASSETS_PATH).show_files_listing());
}

/// App configurations and shared state that is initialized on a per worker basis.
fn app_factory() -> App<impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = Error,
    InitError = ()
>>
{
    let pg = PgConnPool::init().map(|pool| web::Data::new(pool)).unwrap();

    let template_engine = template_engine::Engine::init()
        .map(|engine| web::Data::new(engine))
        .unwrap();

    App::new()
        .app_data(template_engine)
        .app_data(pg)
        .wrap(RequestInfoFactory::new()) // This must be the innermost middleware i.e. it must go last.
        .configure(static_assets)
        .configure(routes::routes)
}

pub async fn run() -> std::io::Result<()> {
    let host = dotenv::var("HOST")
        .unwrap()
        .parse::<Ipv4Addr>()
        .unwrap();

    let port = dotenv::var("PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let socketaddr = SocketAddrV4::new(host, port);

    // Prefer logical if SMT is supported by host-machine and ever becomes desirable.
    let num_workers = num_cpus::get_physical();

    // How many pending pending connections before automatically sending error back to client.
    let backlog = dotenv::var("BACKLOG")
        .map(|val| val.parse::<u32>().unwrap())
        .unwrap();

    // Threadpool size per worker for blocking tasks.
    let blocking_workers_pool_size_per_worker = dotenv::var("BLOCKING_WORKER_POOL_SIZE")
        .map(|size| size.parse::<usize>().unwrap())
        .unwrap();

    // How long a connection is allowed to be kept alive before forceful shutdown.
    let client_disconnect_timeout_ms = dotenv::var("CLIENT_DISCONNECT_TIMEOUT_MS")
        .map(|val| val.parse::<u64>().unwrap())
        .map(|ms| Duration::from_millis(ms))
        .unwrap();

    // How long server waits for client to transmit headers before sending 408.
    let client_request_timeout_ms = dotenv::var("CLIENT_REQ_TIMEOUT_MS")
        .map(|val| val.parse::<u64>().unwrap())
        .map(|ms| Duration::from_millis(ms))
        .unwrap();

    // How long to keep a single connection open to re-use for follow-up requests e.g. assets.
    let keep_alive_s = dotenv::var("KEEP_ALIVE_S")
        .map(|val| val.parse::<u64>().unwrap())
        .map(|s| Duration::from_secs(s))
        .map(|duration| KeepAlive::Timeout(duration))
        .unwrap();

    // All socket listeners will stop listening for connections when this is reached for all workers.
    let max_connections_per_worker = dotenv::var("MAX_CONNECTIONS")
        .map(|val| val.parse::<usize>().unwrap())
        .unwrap();

    // How many active TLS/SSL handshakes can occur concurrently on any given worker. If limit
    // is reached across all workers, all socket listeners will stop accepting connections.
    let tls_rate_per_worker = dotenv::var("TLS_RATE")
        .map(|val| val.parse::<usize>().unwrap())
        .unwrap();

    // Allowed duration for graceful shutdown tasks. Forceful shutdown if exceeds limit.
    let shutdown_timeout_s = dotenv::var("SHUTDOWN_TIMEOUT_S")
        .map(|val| val.parse::<u64>().unwrap())
        .unwrap();

    log::info!("Initializing {} workers", num_workers);
    log::info!("Initializing {} Postgres connections per worker", CONNS_PER_WORKER);
    log::info!("Listening on {}", socketaddr.to_string());

    HttpServer::new(app_factory)
        .workers(num_workers)
        .backlog(backlog)
        .worker_max_blocking_threads(blocking_workers_pool_size_per_worker)
        .client_disconnect_timeout(client_disconnect_timeout_ms)
        .client_request_timeout(client_request_timeout_ms)
        .keep_alive(keep_alive_s)
        .max_connections(max_connections_per_worker)
        .max_connection_rate(tls_rate_per_worker)
        .shutdown_timeout(shutdown_timeout_s)
        .bind(socketaddr)?
        .run() 
        .await
}
