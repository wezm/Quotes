use rocket::response::status::NotFound;
use rocket::response::{content, Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::{tera::Tera, Template};

use crate::db::{self, QuotesDb, HomeRow};

pub fn routes() -> Vec<Route> {
    routes![home, profile, quotes]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeContext {
    users: Vec<HomeRow>
}

#[get("/")]
async fn home(db: QuotesDb) -> Result<Template, Debug<rusqlite::Error>> {
    let rows = db.run(|conn| db::home_query(conn)).await?;
    Ok(Template::render("home", HomeContext { users: rows }))
}

#[get("/<username>")]
fn profile(db: QuotesDb, username: String) -> Result<content::Html<String>, NotFound<String>> {
    unimplemented!()
}

#[get("/<username>/quotes")]
fn quotes(db: QuotesDb, username: String) -> Result<content::Html<String>, NotFound<String>> {
    unimplemented!()
}
