#[cfg(feature = "ssr")]
use lettre::SmtpTransport;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct EmailClient {
    pub mailer: SmtpTransport,
    pub from_name: String,
    pub from_email: String,
}

#[cfg(feature = "ssr")]
impl EmailClient {
    fn init(mailer: SmtpTransport) -> Self {
        Self {
            mailer,
            from_name: String::from("Coco"),
            from_email: String::from("Email"),
        }
    }
}

#[cfg(feature = "ssr")]
pub fn init_email_client(
    smtp_user: &str,
    smtp_password: &str,
) -> Result<EmailClient, anyhow::Error> {
    use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

    let creds = Credentials::new(smtp_user.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    Ok(EmailClient::init(mailer))
}
