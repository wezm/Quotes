use std::borrow::Cow;
use std::collections::HashMap;

use chrono::DateTime;
use rocket::response::{Debug, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::{tera, Template};

use crate::auth::{self, AuthenticatedUser};
use crate::db::{self, QuoteRow, QuotesDb, UserRow};
use crate::QuotesError;

pub fn routes() -> Vec<Route> {
    routes![profile, profile_redirect,]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ProfileContext {
    title: String,
    username: String,
    rows: Vec<ProfileRow>,
    current_user: AuthenticatedUser,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ProfileRow {
    label: Cow<'static, str>,
    value: String,
}

impl ProfileRow {
    fn new<L: Into<Cow<'static, str>>, S: ToString>(label: L, value: S) -> Self {
        ProfileRow {
            label: label.into(),
            value: value.to_string(),
        }
    }
}

#[get("/user/<_username>", rank = 2)]
fn profile_redirect(_username: String) -> Redirect {
    Redirect::to(uri!(auth::login))
}

#[get("/user/<username>")]
async fn profile(
    current_user: AuthenticatedUser,
    db: QuotesDb,
    username: String,
) -> Result<Option<Template>, Debug<QuotesError>> {
    let user_map: HashMap<String, UserRow> = db
        .run(|conn| db::user_map(conn))
        .await
        .map_err(QuotesError::from)?;
    let users = user_map.values().collect::<Vec<_>>();
    let user = match user_map.get(&username) {
        Some(user) => user,
        None => return Ok(None),
    };
    let user_id = user.id;
    let quote_counts = db
        .run(move |conn| db::quote_counts(conn))
        .await
        .map_err(QuotesError::from)?;
    let pos = quote_counts
        .iter()
        .position(|(uid, _count)| *uid == user_id);

    let mut rows = Vec::new();
    let total_quotes = pos.map_or(0, |i| quote_counts[i].1);
    rows.push(ProfileRow::new("Total Quotes", total_quotes));

    let self_quotes = db
        .run(move |conn| db::self_quote_count(conn, user_id))
        .await
        .map_err(QuotesError::from)?;
    rows.push(ProfileRow::new("Self Quotes", self_quotes));

    let post_count = db
        .run(move |conn| db::user_post_count(conn, user_id))
        .await
        .map_err(QuotesError::from)?;
    rows.push(ProfileRow::new("Total Posts", post_count));

    rows.push(ProfileRow::new(
        "Ranking",
        pos.map(|rank| format!("{}{}", rank + 1, ordinal(rank + 1)))
            .unwrap_or_else(|| String::from("Last")),
    ));

    // Nearest Rival Ahead
    let rival_index = pos.and_then(|i| i.checked_sub(1));
    let value = if let Some((rival_id, rival_count)) = rival_index.and_then(|i| quote_counts.get(i))
    {
        // Not top user
        let rival = users.iter().find(|rival| rival.id == *rival_id).unwrap();
        format!(
            r#"<a href="{url}">{username}</a>, {num} quotes ahead"#,
            username = rival.username,
            num = rival_count - total_quotes,
            url = uri!(profile(username = &rival.username))
        )
    } else {
        String::from("None")
    };
    rows.push(ProfileRow::new("Nearest Rival Ahead", value));

    // Nearest Rival Behind
    let value = if let Some((rival_id, rival_count)) = pos.and_then(|i| quote_counts.get(i + 1)) {
        // Not last user
        let rival = users.iter().find(|rival| rival.id == *rival_id).unwrap();
        format!(
            r#"<a href="{url}">{username}</a>, {num} quotes behind"#,
            username = rival.username,
            num = total_quotes - rival_count,
            url = uri!(profile(username = &rival.username))
        )
    } else {
        String::from("None")
    };
    rows.push(ProfileRow::new("Nearest Rival Behind", value));

    // Quotes behind leader
    if let Some((leader_id, leader_count)) = quote_counts.get(0) {
        let leader = users.iter().find(|rival| rival.id == *leader_id).unwrap();
        let label = format!(
            r#"Quotes Behind Leader (<a href="{url}">{username}</a>)"#,
            username = leader.username,
            url = uri!(profile(username = &leader.username))
        );
        rows.push(ProfileRow::new(label, leader_count - total_quotes));
    }

    // Average Rating
    let average_rating = db
        .run(move |conn| db::average_rating(conn, user_id))
        .await
        .map_err(QuotesError::from)?;
    rows.push(ProfileRow::new(
        "Average Rating",
        format!("{:.2}", average_rating),
    ));

    // Favourite Quote
    let fav_quote = if let Some(fav_quote_id) = user.favourite_quote_id {
        db.run(move |conn| db::get_quote(conn, fav_quote_id))
            .await
            .map_err(QuotesError::from)?
            .map(|quote: QuoteRow| {
                let quote_user = users
                    .iter()
                    .find(|user| user.id == quote.user_id)
                    .map_or("Unknown", |user| user.username.as_str());
                format!(
                    r#"<span class="quote">{}</span> by {}"#,
                    tera::escape_html(&quote.quote_body),
                    quote_user
                )
            })
            .unwrap_or_else(|| String::from("Unknown"))
    } else {
        String::from("None")
    };
    // TODO: If user is viewing their own profile add the pencil icon to the label to set favourite
    rows.push(ProfileRow::new("Favourite Quote", fav_quote));

    let last_quoted_at = db
        .run(move |conn| db::last_quoted(conn, user_id))
        .await
        .map_err(QuotesError::from)?;
    let last_quoted = if let Some(timestamp) = last_quoted_at {
        html_date(timestamp)
    } else {
        String::from("Never")
    };
    rows.push(ProfileRow::new("Last Quoted", last_quoted));

    let last_posted = if let Some(timestamp) = user.last_posted {
        html_date(timestamp)
    } else {
        String::from("Never")
    };
    rows.push(ProfileRow::new("Last Post", last_posted));

    Ok(Some(Template::render(
        "userprofile",
        ProfileContext {
            title: format!("{}'s Profile", username),
            username,
            rows,
            current_user,
        },
    )))
}

fn ordinal(num: usize) -> &'static str {
    if (11..13).contains(&(num % 100)) {
        "th"
    } else {
        match num % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

fn html_date(timestamp: u32) -> String {
    if timestamp != 0 {
        // {{ timestamp + 36000 | date(format='%-d %b %Y <span class="time">%-I:%M %p</span>') | safe }}
        DateTime::from_timestamp(i64::from(timestamp) + 36000, 0)
            .map(|date| {
                date.format("%-d %b %Y <span class=\"time\">%-I:%M %p</span>")
                    .to_string()
            })
            .unwrap_or_else(|| String::from("Invalid"))
    } else {
        String::from("N/A")
    }
}
