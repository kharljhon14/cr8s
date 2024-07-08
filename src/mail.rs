use lettre::{
    message::{header::ContentType, MessageBuilder},
    transport::smtp::{authentication::Credentials, response::Response},
    SmtpTransport, Transport,
};
use tera::{Context, Tera};

use std::error::Error;

pub struct HtmlMailer {
    pub template_engine: Tera,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl HtmlMailer {
    pub fn send(
        self,
        subject: &str,
        to: String,
        template_name: &str,
        template_context: Context,
    ) -> Result<Response, Box<dyn Error>> {
        let html_body = self
            .template_engine
            .render(template_name, &template_context)?;

        let message = MessageBuilder::new()
            .subject(subject)
            .from("Cr8s <noreply@cr8s.com>".parse()?)
            .to(to.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let credentials = Credentials::new(self.smtp_username, self.smtp_password);

        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
