//! User authentication.

// TODO: Refresh session cookie on new requests

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::{try_outcome, IntoOutcome};
use rocket::request::{FlashMessage, FromRequest, Outcome, Request};
use rocket::response::{Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::Template;
use time::Duration; // for Cookie

use crate::db::{self, QuotesDb, UserRow};
use crate::quotes;

pub const QUOTES_SESSION: &str = "QUOTES_SESSION";

#[cfg(debug_assertions)]
pub const SECURE_COOKIE: bool = false;

#[cfg(not(debug_assertions))]
pub const SECURE_COOKIE: bool = true;

pub struct AuthenticatedUser {
    pub user: UserRow,
}

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

pub fn routes() -> Vec<Route> {
    routes![do_login, logout, login_user, login]
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = rusqlite::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO: use the request local state
        let db = try_outcome!(request
            .guard::<QuotesDb>()
            .await
            .map_failure(|(status, ())| (status, rusqlite::Error::QueryReturnedNoRows))); // FIXME error type

        let user_id = try_outcome!(request
            .cookies()
            .get_private(QUOTES_SESSION)
            .and_then(|cookie| cookie.value().parse().ok())
            .or_forward(()));

        db.run(move |conn| db::get_user(conn, user_id))
            .await
            .map(|user| AuthenticatedUser { user })
            .or_forward(())
    }
}

#[post("/login", data = "<form>")]
async fn login(
    db: QuotesDb,
    cookies: &CookieJar<'_>,
    form: Form<LoginForm>,
) -> Result<Flash<Redirect>, Debug<rusqlite::Error>> {
    let form = form.into_inner();
    let username = form.username;
    match db
        .run(move |conn| db::user_for_login(conn, &username))
        .await
    {
        Ok(user) => {
            if verify(&user.password_hash, form.password.as_bytes()) {
                let cookie = Cookie::build(QUOTES_SESSION, user.id.to_string())
                    .path("/")
                    .secure(SECURE_COOKIE)
                    .http_only(true)
                    .max_age(Duration::weeks(1))
                    .finish();
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
            // TODO: Make a better HTML response?
            Ok(Flash::error(Redirect::to(uri!(login)), "Invalid password."))
        }
        Err(err) => Err(err.into()),
    }
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(QUOTES_SESSION));
    Flash::success(Redirect::to(uri!(login)), "Successfully logged out.")
}

#[get("/login")]
fn login_user(_user: AuthenticatedUser) -> Redirect {
    Redirect::to(uri!(quotes::home))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginContext<'a, 'b> {
    title: &'a str,
    flash: Option<FlashMessage<'b>>,
}

#[get("/login", rank = 2)]
pub fn do_login(flash: Option<FlashMessage<'_>>) -> Template {
    let context = LoginContext {
        title: "Login",
        flash,
    };
    Template::render("login", context)
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
