#![allow(unused_imports)]

use diesel::sql_types::{Bool, Varchar};

sql_function! {
    /// Returns authentication info of user provided username and password.
    #[sql_name = "authenticate_user_via_uname_pw"]
    fn authenticate_user_via_uname_pw(username: Varchar, password: Varchar) -> Bool;
}
