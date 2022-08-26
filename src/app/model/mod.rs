/// Prelude to import in every new model. Comes with all of the traits and types
/// all model's in this application are expected to have.
mod prelude {
    pub use chrono::naive::{
        NaiveDateTime,
        serde::{ts_seconds, ts_seconds_option},
    };
    pub use crate::app::database::pg;
    pub use diesel::{
        associations::HasTable,
        dsl::{select, sql},
        expression_methods::{ExpressionMethods, PgExpressionMethods},
        pg::Pg,
        PgConnection,
        prelude::Insertable,
        QueryDsl,
        Queryable,
        result::Error as DieselError,
        RunQueryDsl,
    };
    pub use serde::{Serialize, Deserialize};
    pub use serde_with::skip_serializing_none;
}

pub mod user;
