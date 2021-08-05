use log::info;
use mailgun_sdk::send_message::{SendMessageParam, SendMessageParamList, SendMessageResponse};
use mailgun_sdk::{Client, ParamList};

use crate::QuotesConfig;
use crate::{resetpass, QuotesError};

pub async fn forgot_password(
    config: &QuotesConfig,
    to: &str,
    token: &str,
) -> Result<SendMessageResponse, QuotesError> {
    let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);
    let reset_uri = uri!("https://quotes.randome.net", resetpass::resetpass(token));
    let body = format!(
        "Follow this link to reset your quotes page password:\n{}",
        reset_uri
    );
    let send_emails = config.send_emails;
    let to = to.to_owned();
    tokio::task::spawn_blocking(move || {
        let params = SendMessageParamList::default()
            .add(SendMessageParam::To(&to))
            .add(SendMessageParam::From("quotes@randome.net"))
            .add(SendMessageParam::Subject("Password reset"))
            .add(SendMessageParam::Text(&body));
        if send_emails {
            client.send_message(params)
        } else {
            info!("Sending disabled, not sending email: {:#?}", params);
            Ok(SendMessageResponse {
                id: String::from("test"),
                message: String::from("Ok"),
            })
        }
    })
    .await?
    .map_err(QuotesError::from)
}
