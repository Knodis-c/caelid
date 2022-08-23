mod prelude {
    pub use chrono::naive::serde::ts_seconds_option;
    pub use diesel::{
        expression_methods::ExpressionMethods,
        PgConnection,
        Queryable,
        result::Error as DieselError,
        RunQueryDsl,
    };
    pub use serde::{Serialize, Deserialize};
    pub use serde_with::skip_serializing_none;
}

pub mod user;
