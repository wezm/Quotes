//! User authentication.

// TODO: Refresh session cookie on new requests
use std::collections::HashMap;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::{try_outcome, IntoOutcome};
use rocket::request::{FlashMessage, FromRequest, Outcome, Request};
use rocket::response::{Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::time::Duration;
use rocket::Route;
use rocket_dyn_templates::Template; // for Cookie

use crate::db::{self, QuotesDb, UserRow};
use crate::{quotes, QuotesError};

pub const QUOTES_SESSION: &str = "QUOTES_SESSION";

#[cfg(debug_assertions)]
pub const SECURE_COOKIE: bool = false;

#[cfg(not(debug_assertions))]
pub const SECURE_COOKIE: bool = true;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct AuthenticatedUser(UserRow);

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct FlashContext<'a, 'b> {
    pub(crate) title: &'a str,
    pub(crate) flash: Option<FlashMessage<'b>>,
}

#[derive(Debug)]
pub enum AuthenticatedUserError {
    Database(rusqlite::Error),
    GuardFailure,
}

pub fn routes() -> Vec<Route> {
    routes![login, do_login, login_user, logout,]
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthenticatedUserError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO: use the request local state
        let db = try_outcome!(request
            .guard::<QuotesDb>()
            .await
            .map_error(|(status, ())| (status, AuthenticatedUserError::GuardFailure)));

        let user_id = try_outcome!(request
            .cookies()
            .get_private(QUOTES_SESSION)
            .and_then(|cookie| cookie.value().parse().ok())
            .or_forward(Status::BadRequest));

        db.run(move |conn| db::get_user(conn, user_id))
            .await
            .map(AuthenticatedUser)
            .map_err(|err| err.into())
            .or_forward(Status::NotFound)
    }
}

#[post("/login", data = "<form>")]
async fn do_login(
    db: QuotesDb,
    cookies: &CookieJar<'_>,
    form: Form<LoginForm>,
) -> Result<Flash<Redirect>, Debug<QuotesError>> {
    let form = form.into_inner();
    let username = form.username;
    let user = db
        .run(move |conn| db::user_for_login(conn, &username))
        .await;
    match user {
        Ok(user) => {
            let password = form.password.as_bytes().to_owned();
            let password_hash = user.password_hash.clone();
            let valid = tokio::task::spawn_blocking(move || verify(&password_hash, &password))
                .await
                .map_err(QuotesError::from)?;

            if valid {
                let cookie = Cookie::build((QUOTES_SESSION, user.id.to_string()))
                    .path("/")
                    .secure(SECURE_COOKIE)
                    .http_only(true)
                    .max_age(Duration::weeks(1));
                cookies.add_private(cookie);

                Ok(Flash::success(
                    Redirect::to(uri!(quotes::home)),
                    "Login successful",
                ))
            } else {
                Ok(Flash::error(Redirect::to(uri!(login)), "Invalid password."))
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Ok(Flash::error(Redirect::to(uri!(login)), "Invalid password."))
        }
        Err(err) => Err(QuotesError::from(err).into()),
    }
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Template {
    cookies.remove_private(Cookie::from(QUOTES_SESSION));
    let mut context = HashMap::new();
    context.insert("title", "Goodbye");
    Template::render("logout", context)
}

#[get("/login")]
fn login_user(_user: AuthenticatedUser) -> Redirect {
    Redirect::to(uri!(quotes::home))
}

#[get("/login", rank = 2)]
pub fn login(flash: Option<FlashMessage<'_>>) -> Template {
    let context = FlashContext {
        title: "Login",
        flash,
    };
    Template::render("login", context)
}

impl AuthenticatedUser {
    pub fn id(&self) -> i64 {
        self.0.id
    }
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

impl From<rusqlite::Error> for AuthenticatedUserError {
    fn from(err: rusqlite::Error) -> Self {
        AuthenticatedUserError::Database(err)
    }
}
