use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use quotes::db::migrate;
use rusqlite::Connection;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Quote {
    quote: String,
    poster: String,
    time: u64,
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
    last_posted: Option<u64>,
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

    // let mut insert_user = conn.prepare("INSERT INTO users (")?;
    // let mut insert_quotes = conn.prepare("SELECT id, name, data FROM person")?;

    let users = dbg!(read_users(&qbase)?);
    for user in &users {
        let user_quotes = read_quotes(&user.quotes_path())?;

        // Insert the user into the db and get its id
        // for each quote insert that into the db
        // and create a rating record for each user that has rated it
    }

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
                .ok_or("missing time")
                .and_then(|time| time.parse().map_err(|_err| "unable to parse time"))?,
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
    fn test_parse_quote_reference() {
        assert_eq!(
            parse_quote_reference("anryan26").unwrap(),
            (String::from("anryan"), 26)
        );
    }
}
