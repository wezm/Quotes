use rocket::serde::Serialize;
use rocket_sync_db_pools::{database, rusqlite};

use crate::Result;

#[database("quotes_db")]
pub struct QuotesDb(rusqlite::Connection);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HomeRow {
    username: String,
    quote_count: usize,
    last_quoted: Option<u64>,
    last_posted: Option<u64>,
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

pub fn migrate(conn: &mut rusqlite::Connection) -> Result<()> {
    embedded::migrations::runner().run(conn)?;
    Ok(())
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
