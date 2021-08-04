#[macro_use]
extern crate rocket;

use std::fmt;

use rocket::serde::Deserialize;

pub mod auth;
pub mod db;
mod email;
pub mod quotes;
pub mod resetpass;

/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// errror handling crate or defining an error type as an `enum` of causes.
/// However, for our example, using a boxed `std::error::Error` is sufficient.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QuotesConfig {
    pub mailgun_api_key: String,
    pub mailgun_domain: String,
    pub send_emails: bool,
}

#[derive(Debug)]
pub enum QuotesError {
    DataBase(rusqlite::Error),
    Task(tokio::task::JoinError),
    Email(mailgun_sdk::ClientError),
}

impl From<rusqlite::Error> for QuotesError {
    fn from(err: rusqlite::Error) -> Self {
        QuotesError::DataBase(err)
    }
}

impl From<tokio::task::JoinError> for QuotesError {
    fn from(err: tokio::task::JoinError) -> Self {
        QuotesError::Task(err)
    }
}

impl From<mailgun_sdk::ClientError> for QuotesError {
    fn from(err: mailgun_sdk::ClientError) -> Self {
        QuotesError::Email(err)
    }
}

impl fmt::Display for QuotesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuotesError::DataBase(err) => err.fmt(f),
            QuotesError::Task(err) => err.fmt(f),
            QuotesError::Email(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for QuotesError {}
