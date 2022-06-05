use std::collections::HashMap;

use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, Form, FromFormField, ValueField};
use rocket::request::FlashMessage;
use rocket::response::{Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::auth::{self, AuthenticatedUser};
use crate::db::{self, HomeRow, QuoteRow, QuotesDb, Rating, UserRow};

pub fn routes() -> Vec<Route> {
    routes![
        home,
        home_redirect,
        quotes,
        quotes_redirect,
        all_quotes,
        all_quotes_redirect,
        rate_quote,
        rate_quote_redirect,
        do_rate_quote,
    ]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
    users: Vec<HomeRow>,
    current_user: AuthenticatedUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct QuotesContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
    username: String,
    quotes: Vec<QuoteRow>,
    highlight: Option<i64>,
    ratings: HashMap<String, Vec<i64>>,
    current_user: AuthenticatedUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AllQuotesContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
    quotes: Vec<QuotesContext<'f>>,
    current_user: AuthenticatedUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RateQuoteContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
    quote: QuoteRow,
    user: UserRow,
    ratings: HashMap<i64, Vec<i64>>,
    current_user: AuthenticatedUser,
}

#[derive(FromForm)]
struct RateQuoteForm {
    quote_id: i64,
    rating: Rating,
}

#[get("/", rank = 2)]
fn home_redirect() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/")]
pub async fn home(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Debug<rusqlite::Error>> {
    let rows = db.run(db::home_query).await?;
    Ok(Template::render(
        "home",
        HomeContext {
            title: String::from("View Quotes"),
            flash,
            users: rows,
            current_user,
        },
    ))
}

#[get("/quotes/<_username>", rank = 2)]
fn quotes_redirect(_username: String) -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/quotes/<username>?<highlight>")]
async fn quotes(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    username: String,
    highlight: Option<i64>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Option<Template>, Debug<rusqlite::Error>> {
    let users = db.run(db::user_map).await?;
    let user_id = match users.get(&username) {
        Some(user) => user.id,
        None => return Ok(None),
    };
    let quotes = db.run(move |conn| db::user_quotes(conn, user_id)).await?;
    let ratings = db
        .run(move |conn| db::quote_raters(conn, user_id))
        .await
        .map(stringify_ratings)?;
    Ok(Some(Template::render(
        "quotes",
        QuotesContext {
            title: format!("{}'s quotes", username),
            flash,
            username,
            quotes,
            highlight,
            ratings,
            current_user,
        },
    )))
}

#[get("/quotes", rank = 2)]
fn all_quotes_redirect() -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/quotes?<highlight>")]
async fn all_quotes(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    highlight: Option<i64>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Option<Template>, Debug<rusqlite::Error>> {
    let users = db.run(db::user_map).await?;
    let mut quotes = Vec::new();
    for (username, user) in users {
        let user_id = user.id;
        let user_quotes = db.run(move |conn| db::user_quotes(conn, user_id)).await?;
        let ratings = db
            .run(move |conn| db::quote_raters(conn, user_id))
            .await
            .map(stringify_ratings)?;
        let context = QuotesContext {
            title: format!("{}'s quotes", username),
            flash: None,
            username,
            quotes: user_quotes,
            highlight,
            ratings,
            current_user: current_user.clone(),
        };
        quotes.push(context);
    }

    let context = AllQuotesContext {
        title: String::from("All quotes"),
        flash,
        quotes,
        current_user,
    };
    Ok(Some(Template::render("allquotes", context)))
}

#[get("/quotes/rate/<_quote_id>", rank = 2)]
fn rate_quote_redirect(_quote_id: i64) -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/quotes/rate/<quote_id>")]
async fn rate_quote(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    quote_id: i64,
    flash: Option<FlashMessage<'_>>,
) -> Result<Option<Template>, Debug<rusqlite::Error>> {
    let quote = match db.run(move |conn| db::get_quote(conn, quote_id)).await? {
        Some(quote) => quote,
        None => return Ok(None),
    };
    let quote_user_id = quote.user_id;
    let user = db
        .run(move |conn| db::get_user(conn, quote_user_id))
        .await?;
    let ratings = db
        .run(move |conn| db::quote_raters(conn, quote_user_id))
        .await?;

    let context = RateQuoteContext {
        title: String::from("Rate Quote"),
        flash,
        quote,
        user,
        ratings,
        current_user,
    };
    Ok(Some(Template::render("ratequote", context)))
}

#[post("/quotes/rate", data = "<form>")]
async fn do_rate_quote(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    form: Form<RateQuoteForm>,
) -> Result<Flash<Redirect>, Debug<rusqlite::Error>> {
    let quote_id = form.quote_id;
    let current_user_id = current_user.id();
    let quote = match db.run(move |conn| db::get_quote(conn, quote_id)).await? {
        Some(quote) => quote,
        None => return Ok(Flash::error(Redirect::to(uri!(home)), "Invalid quote id")),
    };
    if db
        .run(move |conn| db::rating_exists(conn, current_user_id, quote_id))
        .await?
    {
        return Ok(Flash::error(
            Redirect::to(uri!(home)),
            "You have already rated this quote",
        ));
    }
    db.run(move |conn| db::rate_quote(conn, current_user_id, form.quote_id, form.rating))
        .await?;
    let user = db
        .run(move |conn| db::get_user(conn, quote.user_id))
        .await?;

    Ok(Flash::success(
        Redirect::to(uri!(quotes(
            username = user.username,
            highlight = Some(quote_id)
        ))),
        "Rating added",
    ))
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Rating {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value.parse() {
            Ok(rating) => Ok(rating),
            Err(()) => Err(form::Error::validation("rating must be between 1 and 5"))?,
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `16B` as default.
        let limit = field.request.limits().get("rating").unwrap_or(16.bytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        match bytes.as_slice() {
            [1] => Ok(Rating::One),
            [2] => Ok(Rating::Two),
            [3] => Ok(Rating::Three),
            [4] => Ok(Rating::Four),
            [5] => Ok(Rating::Five),
            _ => Err(form::Error::validation("rating must be between 1 and 5"))?,
        }
    }
}

// Template contexts require all keys to be strings
fn stringify_ratings(ratings: HashMap<i64, Vec<i64>>) -> HashMap<String, Vec<i64>> {
    ratings
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}
