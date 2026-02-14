use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: Vec<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub attachments: Option<Vec<EmailAttachment>>,
    pub reply_to: Option<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub name: String,
    pub subject: String,
    pub body_html: String,
    pub body_text: Option<String>,
}

pub struct EmailService {
    from_address: String,
    from_name: String,
}

impl EmailService {
    pub fn new(from_address: String, from_name: String) -> Self {
        Self {
            from_address,
            from_name,
        }
    }

    pub async fn send(&self, message: EmailMessage) -> Result<(), EmailError> {
        tracing::info!(
            to = ?message.to,
            subject = %message.subject,
            "Sending email"
        );

        // In production, would integrate with SMTP or email API
        Ok(())
    }

    pub async fn send_welcome_email(&self, to: &str, username: &str) -> Result<(), EmailError> {
        let message = EmailMessage {
            to: vec![to.to_string()],
            cc: None,
            bcc: None,
            subject: "Welcome to Our Platform!".to_string(),
            body_text: Some(format!("Welcome, {}! We're glad to have you.", username)),
            body_html: Some(format!(
                "<h1>Welcome, {}!</h1><p>We're glad to have you on our platform.</p>",
                username
            )),
            attachments: None,
            reply_to: None,
            headers: None,
        };

        self.send(message).await
    }

    pub async fn send_password_reset(&self, to: &str, reset_token: &str) -> Result<(), EmailError> {
        let reset_url = format!("https://example.com/reset-password?token={}", reset_token);

        let message = EmailMessage {
            to: vec![to.to_string()],
            cc: None,
            bcc: None,
            subject: "Password Reset Request".to_string(),
            body_text: Some(format!(
                "Click this link to reset your password: {}",
                reset_url
            )),
            body_html: Some(format!(
                "<p>Click <a href=\"{}\">here</a> to reset your password.</p>",
                reset_url
            )),
            attachments: None,
            reply_to: None,
            headers: None,
        };

        self.send(message).await
    }

    pub async fn send_order_confirmation(
        &self,
        to: &str,
        order_number: &str,
    ) -> Result<(), EmailError> {
        let message = EmailMessage {
            to: vec![to.to_string()],
            cc: None,
            bcc: None,
            subject: format!("Order Confirmation - {}", order_number),
            body_text: Some(format!(
                "Your order {} has been confirmed. Thank you for your purchase!",
                order_number
            )),
            body_html: Some(format!(
                "<h1>Order Confirmed</h1><p>Your order <strong>{}</strong> has been confirmed.</p>",
                order_number
            )),
            attachments: None,
            reply_to: None,
            headers: None,
        };

        self.send(message).await
    }

    pub fn from_address(&self) -> &str {
        &self.from_address
    }

    pub fn from_name(&self) -> &str {
        &self.from_name
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email address: {0}")]
    InvalidAddress(String),
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
