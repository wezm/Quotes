use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use quotes::db::migrate;
use rusqlite::{params, Connection};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Quote {
    quote: String,
    poster: String,
    time: Option<u32>,
    ratings: String,
    reference: Option<String>,
}

// Profile files contain these lines:
// 0: password hash
// 1: first name
// 2: last name
// 3: last time this person posted a quote
// 4: ? (always 0)
// 5: favourite quote id
// The filename is the username
#[derive(Debug)]
pub struct User {
    path: PathBuf,
    username: String,
    first_name: String,
    last_name: String,
    last_posted: Option<u32>,
    favourite_quote: Option<String>,
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let qbase = match args.next() {
        Some(path) => PathBuf::from(path),
        None => exit_usage(),
    };
    let dbpath = match args.next() {
        Some(path) => PathBuf::from(path),
        None => exit_usage(),
    };

    let mut conn = Connection::open(&dbpath)?;
    migrate(&mut conn)?;

    // Add users to the db
    let users = read_users(&qbase)?;
    let mut uid_map = HashMap::new();
    let mut insert_user = conn.prepare("INSERT INTO users (username, firstname, surname, password_hash, email, last_posted) VALUES (?,?,?,?,?,?)")?;
    for user in &users {
        // Insert the user into the db and get its id
        let email = format!("{}@example.com", user.username);
        let rowid = insert_user.insert(params![
            user.username,
            user.first_name,
            user.last_name,
            "*",
            email,
            user.last_posted
        ])?;
        println!("Inserted {} with rowid {}", user.username, rowid);
        uid_map.insert(user.username.as_str(), rowid);
    }

    // Add quotes to the db
    let uid_map = uid_map; // drop mutability
    let mut insert_quote = conn.prepare("INSERT INTO quotes (quote_body, user_id, created_at, poster_id, rating) VALUES (?,?,?,?,?)")?;
    for user in &users {
        let user_id = uid_map
            .get(&user.username.as_str())
            .ok_or_else(|| format!("unable to find id of user {}", user.username))?;
        let user_quotes = read_quotes(&user.quotes_path())?;

        // for each quote insert that into the db
        for quote in &user_quotes {
            let poster_id = uid_map
                .get(&quote.poster.as_str())
                .ok_or_else(|| format!("unable to find id of user {}", quote.poster))?;
            insert_quote.execute(params![quote.quote, user_id, quote.time, poster_id, 0])?;
        }

        // and create a rating record for each user that has rated it
    }

    // Go back and populate favourite quotes and fix quote references

    Ok(())
}

fn read_users(qbase: &Path) -> Result<Vec<User>> {
    let mut users = Vec::new();
    for entry in fs::read_dir(qbase)? {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("profile")) {
            let username = path.file_stem().and_then(|stem| stem.to_str()).unwrap();
            let user = read_profile(username.to_owned(), path)?;
            users.push(user);
        }
    }
    users.sort_by(|a, b| a.username.cmp(&b.username));
    Ok(users)
}

fn read_profile(username: String, path: PathBuf) -> Result<User> {
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    let mut lines = file.lines();
    let _ = lines.next(); // password hash
    let first_name = lines.next().unwrap()?;
    let last_name = lines.next().unwrap()?;
    let last_posted =
        Some(lines.next().unwrap()?.parse()?)
            .and_then(|time| if time == 0 { None } else { Some(time) });
    let _ = lines.next(); // unused
    let favourite_quote =
        Some(lines.next().unwrap()?).and_then(|id| if id == "0" { None } else { Some(id) });

    Ok(User {
        path,
        username: username.to_owned(),
        first_name,
        last_name,
        last_posted,
        favourite_quote,
    })
}

fn read_quotes(path: &Path) -> Result<Vec<Quote>> {
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    let mut quotes = Vec::new();
    for line in file.lines() {
        let line = line?;
        let quote = line.parse()?;
        quotes.push(quote);
    }
    Ok(quotes)
}

fn parse_quote_reference(raw: &str) -> Result<(String, u32)> {
    let username = raw
        .chars()
        .take_while(|ch| !('0'..='9').contains(ch))
        .collect::<String>();
    raw[username.len()..]
        .parse()
        .map(|id| (username, id))
        .map_err(Error::from)
}

impl User {
    fn quotes_path(&self) -> PathBuf {
        self.path.with_extension("quotes")
    }

    fn favourite_quote(&self) -> Result<Option<(String, u32)>> {
        self.favourite_quote
            .as_ref()
            .map(|raw| parse_quote_reference(raw))
            .transpose()
    }
}

impl Quote {
    fn ratings(&self) -> Result<(u32, Vec<&str>)> {
        if self.ratings == "0" {
            return Ok((0, Vec::new()));
        }

        let (rating, users) = self
            .ratings
            .split_once(':')
            .ok_or("unable to split rating on :")?;
        let rating = rating.parse()?;
        let rating_users = users.split(',').collect::<Vec<_>>();
        Ok((rating, rating_users))
    }
}

impl FromStr for Quote {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split('|');
        Ok(Quote {
            quote: parts.next().map(ToOwned::to_owned).ok_or("missing quote")?,
            poster: parts
                .next()
                .map(ToOwned::to_owned)
                .ok_or("missing poster")?,
            time: parts
                .next()
                .and_then(|time| if time == "0" { None } else { Some(time) })
                .map(|time| time.parse().map_err(|_err| "unable to parse time"))
                .transpose()?,
            ratings: parts
                .next()
                .map(ToOwned::to_owned)
                .ok_or("missing ratings")?,
            reference: parts.next().and_then(|reference| {
                if reference == "0" {
                    None
                } else {
                    Some(reference.to_owned())
                }
            }),
        })
    }
}

fn exit_usage() -> ! {
    eprintln!("Usage: import path/to/quotes to/this/db.sqlite");
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn favourite_quote() {
        let user = User {
            path: PathBuf::from("/home/wmoore/Documents/quotes/wmoore.profile"),
            username: String::from("wmoore"),
            first_name: String::from("Wesley"),
            last_name: String::from("Moore"),
            last_posted: Some(1070020512),
            favourite_quote: Some(String::from("darnott184")),
        };
        assert_eq!(
            user.favourite_quote().unwrap(),
            Some((String::from("darnott"), 184))
        );
    }

    #[test]
    fn quote_without_ratings() {
        let quote = Quote {
            quote: String::from("Quote"),
            poster: String::from("user"),
            time: Some(1061953950),
            ratings: String::from("0"),
            reference: None,
        };
        assert_eq!(quote.ratings().unwrap(), (0, Vec::new()));
    }

    #[test]
    fn quote_with_ratings() {
        let quote = Quote {
            quote: String::from("Quote"),
            poster: String::from("user"),
            time: Some(1061953950),
            ratings: String::from("4:anryan,wmoore,rliebich"),
            reference: None,
        };
        assert_eq!(
            quote.ratings().unwrap(),
            (4, vec!["anryan", "wmoore", "rliebich"])
        );
    }

    #[test]
    fn test_parse_quote_reference() {
        assert_eq!(
            parse_quote_reference("anryan26").unwrap(),
            (String::from("anryan"), 26)
        );
    }
}
