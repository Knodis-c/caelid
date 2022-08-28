#[macro_use] extern crate diesel;
#[macro_use] extern crate proc_macros;
use dotenv;

/// Core business logic of the application.
mod app;

/// Auxiliary modules that aren't tightly coupled to core business logic.
mod internal;

/// Snapshot of the database schema.
mod schema;

fn cleanup() -> Result<(), std::io::Error> {
    log::info!("Gracefully shutting down");

    internal::pid::destroy().unwrap();

    log::info!("Graceful shutdown complete");

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    internal::log::init_logger();
    let _pid = internal::pid::create()?;
    app::server::run().await?;
    cleanup()
}
