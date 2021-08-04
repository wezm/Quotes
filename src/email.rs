use mailgun_sdk::send_message::{SendMessageParam, SendMessageParamList, SendMessageResponse};
use mailgun_sdk::{Client, ClientError, ParamList};

use crate::auth;
use crate::QuotesConfig;

pub fn forgot_password(
    config: &QuotesConfig,
    to: &str,
    token: &str,
) -> Result<SendMessageResponse, ClientError> {
    let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

    let reset_uri = uri!("https://quotes.randome.net", auth::resetpass(token));
    let body = format!(
        "Follow this link to reset your quotes page password:\n{}",
        reset_uri
    );
    let params = SendMessageParamList::default()
        .add(SendMessageParam::To(to))
        .add(SendMessageParam::From("quotes@randome.net"))
        .add(SendMessageParam::Subject("Password reset"))
        .add(SendMessageParam::Text(&body));
    if config.send_emails {
        client.send_message(params)
    } else {
        println!("Sending disabled, not sending email: {:#?}", params);
        Ok(SendMessageResponse {
            id: String::from("test"),
            message: String::from("Ok"),
        })
    }
}
