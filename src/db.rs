use std::collections::HashMap;

use rocket::serde::Serialize;
use rocket_sync_db_pools::{database, rusqlite};

use crate::Result;

#[database("quotes_db")]
pub struct QuotesDb(rusqlite::Connection);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HomeRow {
    pub username: String,
    pub quote_count: usize,
    pub last_quoted: Option<u64>,
    pub last_posted: Option<u64>,
}

pub fn home_query(conn: &mut rusqlite::Connection) -> Result<Vec<HomeRow>, rusqlite::Error> {
    let sql = "\
    SELECT users.username, count(quotes.id), max(quotes.created_at), users.last_posted \
    FROM users LEFT JOIN quotes ON (quotes.user_id = users.id) \
    GROUP BY users.username \
    ORDER BY users.username";
    let mut stmt = conn.prepare_cached(sql)?;

    let mut results = Vec::new();
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        results.push(HomeRow {
            username: row.get(0)?,
            quote_count: row.get(1)?,
            last_quoted: row.get(2)?,
            last_posted: row.get(3)?,
        })
    }

    Ok(results)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QuoteRow {
    pub id: i64,
    pub quote_body: String,
    pub created_at: Option<u32>,
    pub poster_username: String,
    pub rating: u32,
    pub parent_quote_id: Option<i64>,
    pub parent_quote_username: Option<String>,
}

pub fn user_quotes(
    conn: &mut rusqlite::Connection,
    user_id: i64,
) -> Result<Vec<QuoteRow>, rusqlite::Error> {
    let sql = "\
    SELECT \
        quotes.id,
        quotes.quote_body,
        quotes.created_at,
        users.username AS poster_username,
        quotes.rating,
        quotes.parent_quote_id,
        u2.username AS parent_quote_username
    FROM quotes \
    LEFT JOIN users ON (users.id = quotes.poster_id) \
    LEFT JOIN quotes AS q2 ON (q2.id = quotes.parent_quote_id) \
    LEFT JOIN users u2 ON (u2.id = q2.user_id) \
    WHERE quotes.user_id = ?
    ORDER BY quotes.created_at, quotes.id";
    let mut stmt = conn.prepare_cached(sql)?;

    let mut results = Vec::new();
    let mut rows = stmt.query([user_id])?;
    while let Some(row) = rows.next()? {
        results.push(QuoteRow {
            id: row.get(0)?,
            quote_body: row.get(1)?,
            created_at: row.get(2)?,
            poster_username: row.get(3)?,
            rating: row.get(4)?,
            parent_quote_id: row.get(5)?,
            parent_quote_username: row.get(6)?,
        })
    }

    Ok(results)
}

/// Returns a map of quote id to rater ids
pub fn quote_raters(
    conn: &mut rusqlite::Connection,
    user_id: i64,
) -> Result<HashMap<i64, Vec<i64>>, rusqlite::Error> {
    let sql = "\
    SELECT quotes.id, ratings.user_id \
    FROM quotes JOIN ratings ON quotes.id = ratings.quote_id \
    WHERE quotes.user_id = ?";
    let mut stmt = conn.prepare_cached(sql)?;

    let mut results: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut rows = stmt.query([user_id])?;
    while let Some(row) = rows.next()? {
        let quote_id: i64 = row.get(0)?;
        let rater_id = row.get(1)?;
        results.entry(quote_id).or_default().push(rater_id);
    }

    Ok(results)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub firstname: String,
    pub surname: String,
    pub last_posted: Option<u32>,
    pub favourite_quote_id: Option<u32>,
}

pub fn user_map(
    conn: &mut rusqlite::Connection,
) -> Result<HashMap<String, UserRow>, rusqlite::Error> {
    let sql = "\
    SELECT \
        id,
        username,
        firstname,
        surname,
        last_posted,
        favourite_quote_id
    FROM users";
    let mut stmt = conn.prepare_cached(sql)?;

    let mut results = HashMap::new();
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let username: String = row.get(1)?;
        let user = UserRow {
            id: row.get(0)?,
            username: username.clone(),
            firstname: row.get(2)?,
            surname: row.get(3)?,
            last_posted: row.get(4)?,
            favourite_quote_id: row.get(5)?,
        };
        results.insert(username, user);
    }

    Ok(results)
}

pub fn migrate(conn: &mut rusqlite::Connection) -> Result<()> {
    embedded::migrations::runner().run(conn)?;
    Ok(())
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
