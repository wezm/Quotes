use std::collections::HashMap;

use rocket::response::status::NotFound;
use rocket::response::Debug;
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::db::{self, HomeRow, QuoteRow, QuotesDb, UserRow};

pub fn routes() -> Vec<Route> {
    routes![home, profile, quotes]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeContext {
    users: Vec<HomeRow>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct QuotesContext {
    username: String,
    users: HashMap<String, UserRow>,
    quotes: Vec<QuoteRow>,
    ratings: HashMap<i64, Vec<i64>>,
}

#[get("/")]
async fn home(db: QuotesDb) -> Result<Template, Debug<rusqlite::Error>> {
    let rows = db.run(|conn| db::home_query(conn)).await?;
    Ok(Template::render("home", HomeContext { users: rows }))
}

#[get("/<username>")]
async fn profile(db: QuotesDb, username: String) -> Result<Template, NotFound<&'static str>> {
    let users = db.run(|conn| db::user_map(conn)).await.expect("FIXME");
    let _user_id = match users.get(&username) {
        Some(user) => user.id,
        None => return Err(NotFound("User not found")),
    };
    unimplemented!()
}

#[get("/<username>/quotes")]
async fn quotes(db: QuotesDb, username: String) -> Result<Template, NotFound<&'static str>> {
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
            username,
            users,
            quotes,
            ratings,
        },
    ))
}
