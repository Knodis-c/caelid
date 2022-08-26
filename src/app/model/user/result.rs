use diesel::result::Error as DieselError;
use std::fmt;

pub type UserResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Diesel(DieselError),
    AuthenticationFailure
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Diesel(e) => write!(f, "{}", e),
            Error::AuthenticationFailure => write!(f, "AuthenticationFailure")
        }
    }
}

impl std::error::Error for Error {}
