pub mod prelude {
    pub use chrono::naive::serde::{ts_seconds, ts_seconds_option};
    pub use diesel::{
        associations::HasTable,
        expression_methods::{ExpressionMethods, PgExpressionMethods},
        pg::Pg,
        PgConnection,
        prelude::Insertable,
        Queryable,
        result::Error as DieselError,
        RunQueryDsl,
    };
    pub use serde::{Serialize, Deserialize};
    pub use serde_with::skip_serializing_none;
}

pub mod user;
