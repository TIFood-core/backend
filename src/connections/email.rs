use std::sync::OnceLock;

use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::authentication::Credentials};

use crate::configs;

static EMAIL_MAILER: OnceLock<AsyncSmtpTransport<Tokio1Executor>> = OnceLock::new();

pub fn get_email_mailer() -> &'static AsyncSmtpTransport<Tokio1Executor> {
    EMAIL_MAILER.get_or_init(|| {
        let email_config = configs::email::get_email_config();

        let credentials =
            Credentials::new(email_config.email.clone(), email_config.password.clone());

        AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.smtp)
            .expect("Failed to connect on email server")
            .credentials(credentials)
            .build()
    })
}
