use actix_web::rt::{self, signal};
#[macro_use] extern crate diesel;
use nix::{
    unistd::Pid,
    sys::signal::{kill, Signal}
};
#[macro_use] extern crate proc_macros;
use dotenv;
#[macro_use] extern crate tracing;

/// Core business logic of the application.
mod app;

/// Auxiliary modules that aren't tightly coupled to core business logic.
mod internal;

/// Snapshot of the database schema.
mod schema;

#[tracing::instrument]
fn cleanup(_test: u8) -> Result<(), std::io::Error> {
    info!("Cleaning up...");
    internal::pid::destroy().unwrap();

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    internal::tracing::subscribers::default::init();

    info!("haha");
    trace!("haha");
    debug!("haha");
    warn!("haha");
    error!("haha");

    let pid = internal::pid::create()? as i32;

    // Handle SIGINT as a SIGTERM.
    rt::spawn(async move {
        signal::ctrl_c().await.unwrap();
        kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();
    });

    app::server::run().await?;

    cleanup(8)
}
