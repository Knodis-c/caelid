/// Exposes the `Pg` struct which contains information about the state
/// of the application's communications with the database. `Pg` exposes 
/// a connection pool, wherewith each Actix worker is given one. Internally,
/// `Pg` contains its own private `ScheduledThreadPool`, which it uses to carry
/// out async operations such as connection creation/destruction.
pub mod pg;

/// To flesh out later.
pub mod redis;
