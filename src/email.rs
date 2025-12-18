use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::error::Error;

pub struct EmailConfig {
    pub email_address: String,
    pub email_app_password: String,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let email_address =
            env::var("EMAIL_ADDRESS").map_err(|_| "EMAIL_ADDRESS environment variable not set")?;
        let email_app_password = env::var("EMAIL_APP_PASSWORD")
            .map_err(|_| "EMAIL_APP_PASSWORD environment variable not set")?;

        Ok(Self {
            email_address,
            email_app_password,
        })
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(self.email_address.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        let creds = Credentials::new(self.email_address.clone(), self.email_app_password.clone());

        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .credentials(creds)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}
