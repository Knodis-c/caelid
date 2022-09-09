use actix_web::rt::{self, signal};
#[macro_use] extern crate diesel;
use nix::{
    unistd::Pid,
    sys::signal::{kill, Signal}
};
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

    let pid = internal::pid::create()? as i32;

    // Handle SIGINT as a SIGTERM.
    rt::spawn(async move {
        signal::ctrl_c().await.unwrap();
        kill(Pid::from_raw(pid), Signal::SIGTERM)
            .expect(&format!("Failed to kill {pid}"));
    });

    app::server::run().await?;

    cleanup()
}
