use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::header::ContentType;

pub struct EmailSender {
    transport: Option<SmtpTransport>,
    from: String,
    frontend_url: String,
}

impl EmailSender {
    pub fn from_env() -> Self {
        let host = std::env::var("SMTP_HOST").ok();
        let from = std::env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@pulsarr.app".to_string());
        let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

        let transport = host.map(|host| {
            let port: u16 = std::env::var("SMTP_PORT").ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587);
            let username = std::env::var("SMTP_USERNAME").unwrap_or_default();
            let password = std::env::var("SMTP_PASSWORD").unwrap_or_default();
            let tls_mode = std::env::var("SMTP_TLS").unwrap_or_else(|_| "starttls".to_string()).to_lowercase();

            let creds = Credentials::new(username, password);

            // "ssl" = implicit TLS (port 465), "starttls" = upgrade (port 587), "none" = plaintext
            let builder = match tls_mode.as_str() {
                "ssl" | "tls" => SmtpTransport::relay(&host)
                    .expect("Failed to create SMTP transport"),
                "none" | "false" => SmtpTransport::builder_dangerous(&host),
                _ => SmtpTransport::starttls_relay(&host)
                    .expect("Failed to create SMTP transport"),
            };

            builder
                .port(port)
                .credentials(creds)
                .build()
        });

        if transport.is_none() {
            eprintln!("WARNING: SMTP_HOST not set — password reset emails will not be sent");
        }

        Self { transport, from, frontend_url }
    }

    pub fn send_password_reset(&self, to_email: &str, token: &str) -> Result<(), String> {
        let transport = match &self.transport {
            Some(t) => t,
            None => {
                eprintln!("SMTP not configured — skipping password reset email to {}", to_email);
                return Ok(());
            }
        };

        let reset_link = format!("{}/reset-password?token={}", self.frontend_url, token);

        let email = Message::builder()
            .from(self.from.parse().map_err(|e| format!("Invalid from address: {}", e))?)
            .to(to_email.parse().map_err(|e| format!("Invalid to address: {}", e))?)
            .subject("Pulsarr — Reset Your Password")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"<h2>Password Reset</h2>
<p>You requested a password reset for your Pulsarr account.</p>
<p><a href="{link}">Click here to reset your password</a></p>
<p>This link will expire in 1 hour.</p>
<p>If you did not request this, you can safely ignore this email.</p>"#,
                link = reset_link
            ))
            .map_err(|e| format!("Failed to build email: {}", e))?;

        transport.send(&email).map_err(|e| format!("Failed to send email: {}", e))?;
        Ok(())
    }
}
