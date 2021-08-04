use std::time::{SystemTime, UNIX_EPOCH};

use log::{info, warn};
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::{Route, State};
use rocket_dyn_templates::Template;

use crate::db::{self, QuotesDb};
use crate::{auth, email, quotes, QuotesConfig};

pub const TOKEN_VALIDITY_DURATION: std::time::Duration = std::time::Duration::from_secs(10 * 60); // 10 mins

pub fn routes() -> Vec<Route> {
    routes![forgotpass, do_forgotpass, resetpass, do_resetpass,]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ForgotPassContext<'a, 'b> {
    title: &'a str,
    flash: Option<FlashMessage<'b>>,
}

#[get("/forgotpass")]
pub fn forgotpass(flash: Option<FlashMessage<'_>>) -> Template {
    let context = ForgotPassContext {
        title: "Forgot Password",
        flash,
    };
    Template::render("forgotpass", context)
}

#[derive(FromForm)]
struct ForgotPassForm {
    username: String,
}

#[post("/forgotpass", data = "<form>")]
async fn do_forgotpass(
    db: QuotesDb,
    config: &State<QuotesConfig>,
    form: Form<ForgotPassForm>,
) -> Result<Flash<Redirect>, Debug<rusqlite::Error>> {
    // lookup user, need email
    let form = form.into_inner();
    let username = form.username;
    match db
        .run(move |conn| db::get_user_by_username(conn, &username))
        .await
    {
        Ok(user) => {
            // if found generate token and update user
            let user_id = user.id;
            // TODO: spawn task for this?
            let reset_token =
                generate_token().map_err(|_err| rusqlite::Error::QueryReturnedNoRows)?; // FIXME error
            let expires_at = (SystemTime::now() + TOKEN_VALIDITY_DURATION)
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let token = reset_token.clone(); // FIXME: Can this be avoided?
            db.run(move |conn| db::set_reset_token(conn, user_id, &token, expires_at))
                .await?;

            // send email
            info!("Send reset password email to {}", user.email);
            // TODO: spawn task
            if let Err(err) = email::forgot_password(&config, &user.email, &reset_token).await {
                warn!("Sending email to {} failed: {:?}", user.email, err);
            }

            Ok(Flash::success(
                Redirect::to(uri!(auth::login)),
                "Reset password email sent",
            ))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // This is a lie but apparently this is the done thing to prevent enumeration attacks
            Ok(Flash::success(
                Redirect::to(uri!(quotes::home)),
                "Reset password email sent",
            ))
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResetPassContext<'a, 'b> {
    title: &'a str,
    flash: Option<FlashMessage<'b>>,
    token: &'a str,
    expired: bool,
}

#[get("/resetpass?<token>")]
pub async fn resetpass(
    db: QuotesDb,
    token: String,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Debug<rusqlite::Error>> {
    // Find user by token, verify that it's not expired
    let token_copy = token.clone(); // FIXME: I thought Futures were supposed to work with references
    match db
        .run(move |conn| db::get_user_by_reset_token(conn, &token_copy))
        .await
    {
        Ok(user) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let expired = user
                .reset_token_expires
                .map_or(true, |expires| now > expires);

            let context = ResetPassContext {
                title: "Set Password",
                flash,
                token: &token,
                expired,
            };
            Ok(Template::render("setpass", context))
        }
        Err(err @ rusqlite::Error::QueryReturnedNoRows) => {
            Err(err.into()) // FIXME: 404 or something in this case
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(FromForm)]
pub struct ResetPassForm {
    token: String,
    password: String,
}

#[post("/resetpass", data = "<form>")]
pub async fn do_resetpass(
    db: QuotesDb,
    form: Form<ResetPassForm>,
) -> Result<Flash<Redirect>, Debug<rusqlite::Error>> {
    let form = form.into_inner();
    let token = form.token;
    match db
        .run(move |conn| db::get_user_by_reset_token(conn, &token))
        .await
    {
        Ok(user) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let expired = user
                .reset_token_expires
                .map_or(true, |expires| now > expires);
            if expired {
                Ok(Flash::error(
                    Redirect::to(uri!(auth::login)),
                    "Reset token expired",
                ))
            } else {
                let hash = match hash_password(form.password.as_bytes()) {
                    Ok(hash) => hash,
                    Err(err) => {
                        return Ok(Flash::error(
                            Redirect::to(uri!(auth::login)),
                            format!("Unable to set password: {}", err),
                        ))
                    }
                };
                // Update the user's password and burn the token
                let rows_updated = db
                    .run(move |conn| db::set_password(conn, user.id, &hash))
                    .await?;

                if rows_updated > 0 {
                    Ok(Flash::success(
                        Redirect::to(uri!(auth::login)),
                        "You password has been reset",
                    ))
                } else {
                    Ok(Flash::error(
                        Redirect::to(uri!(auth::login)),
                        "Ehhh doesn't look like that worked... this shouldn't happen",
                    ))
                }
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Flash::error(
            Redirect::to(uri!(auth::login)),
            "Token invalid or already used",
        )),
        Err(err) => Err(err.into()),
    }
}

fn hash_password(password: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut salt = [0u8; 16];
    getrandom::getrandom(&mut salt)?;
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(password, &salt, &config)?;
    Ok(hash)
}

fn generate_token() -> Result<String, getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(hexstring(&buf))
}

pub fn hexstring<'buf>(data: &[u8; 32]) -> String {
    let mut buf = vec![0; 64];
    for (i, byte) in data.iter().copied().enumerate() {
        buf[i * 2] = char::from_digit((u32::from(byte) & 0xF0) >> 4, 16).unwrap() as u8;
        buf[i * 2 + 1] = char::from_digit(u32::from(byte) & 0xF, 16).unwrap() as u8;
    }
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexstring() {
        let data = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        assert_eq!(
            hexstring(&data),
            String::from("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
        );
    }
}
