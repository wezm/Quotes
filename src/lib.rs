#[macro_use]
extern crate rocket;

use std::fmt;

use rocket::serde::Deserialize;

pub mod auth;
pub mod db;
mod email;
pub mod quotes;
pub mod resetpass;
pub mod user;

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
    Random(getrandom::Error),
    Password(argon2::Error),
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

impl From<getrandom::Error> for QuotesError {
    fn from(err: getrandom::Error) -> Self {
        QuotesError::Random(err)
    }
}

impl From<argon2::Error> for QuotesError {
    fn from(err: argon2::Error) -> Self {
        QuotesError::Password(err)
    }
}

impl fmt::Display for QuotesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuotesError::DataBase(err) => err.fmt(f),
            QuotesError::Task(err) => err.fmt(f),
            QuotesError::Email(err) => err.fmt(f),
            QuotesError::Random(err) => err.fmt(f),
            QuotesError::Password(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for QuotesError {}
