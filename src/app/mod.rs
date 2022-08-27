/// Facades over various database interfaces.
pub mod database;

/// Application entities which generally have an analogous database table. Each model which is
/// declared as a struct will be expected to be decorated with the `#[model]` attribute macro,
/// which imparts on that struct all of the behaviors that models in this application is expected
/// to have such as access to `diesel`'s query builder.
pub mod model;

/// Server configurations, route handlers, and shared application state.
pub mod server;

/// Engine that processes HTML server-side before delivery.
pub mod template_engine;
