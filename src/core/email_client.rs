#[cfg(feature = "ssr")]
use lettre::SmtpTransport;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct EmailClient {
    pub mailer: SmtpTransport,
    pub from_name: String,
    pub from_email: String,
}

// TODO: We're assuming from_email == smtp_user provided via .env this could not be the case.

#[cfg(feature = "ssr")]
impl EmailClient {
    fn init(mailer: SmtpTransport, from_email: String, from_name: String) -> Self {
        Self {
            mailer,
            from_name,
            from_email,
        }
    }
}

#[cfg(feature = "ssr")]
pub fn init_email_client(
    smtp_user: &str,
    smtp_password: &str,
    from_name: &str,
) -> Result<EmailClient, anyhow::Error> {
    //use anyhow::anyhow;
    use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};

    let creds = Credentials::new(smtp_user.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // let test_connection = mailer.test_connection()?;
    // if !test_connection {
    //     return Err(anyhow!("Test connection not successfull"));
    // }

    Ok(EmailClient::init(
        mailer,
        smtp_user.to_string(),
        from_name.to_string(),
    ))
}
