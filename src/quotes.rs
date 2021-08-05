use std::collections::HashMap;

use rocket::response::status::NotFound;
use rocket::response::{Debug, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::auth::{self, AuthenticatedUser};
use crate::db::{self, HomeRow, QuoteRow, QuotesDb, UserRow};

pub fn routes() -> Vec<Route> {
    routes![home, home_redirect, quotes, quotes_redirect]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeContext {
    title: String,
    users: Vec<HomeRow>,
    current_user: AuthenticatedUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct QuotesContext {
    title: String,
    username: String,
    users: HashMap<String, UserRow>,
    quotes: Vec<QuoteRow>,
    ratings: HashMap<i64, Vec<i64>>,
    current_user: AuthenticatedUser,
}

#[get("/", rank = 2)]
fn home_redirect() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/")]
pub async fn home(
    current_user: AuthenticatedUser,
    db: QuotesDb,
) -> Result<Template, Debug<rusqlite::Error>> {
    let rows = db.run(|conn| db::home_query(conn)).await?;
    Ok(Template::render(
        "home",
        HomeContext {
            title: String::from("View Quotes"),
            users: rows,
            current_user,
        },
    ))
}

#[get("/quotes/<_username>", rank = 2)]
fn quotes_redirect(_username: String) -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/quotes/<username>")]
async fn quotes(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    username: String,
) -> Result<Template, NotFound<&'static str>> {
    let users = db.run(|conn| db::user_map(conn)).await.expect("FIXME");
    let user_id = match users.get(&username) {
        Some(user) => user.id,
        None => return Err(NotFound("User not found")),
    };
    let quotes = db
        .run(move |conn| db::user_quotes(conn, user_id))
        .await
        .expect("FIXME");
    let ratings = db
        .run(move |conn| db::quote_raters(conn, user_id))
        .await
        .expect("FIXME ratings");
    Ok(Template::render(
        "quotes",
        QuotesContext {
            title: format!("{}'s quotes", username),
            username,
            users,
            quotes,
            ratings,
            current_user,
        },
    ))
}
