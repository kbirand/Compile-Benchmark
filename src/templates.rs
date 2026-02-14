use serde::Serialize;
use tera::{Context, Tera};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, TemplateError> {
        let mut tera = Tera::default();

        // Register built-in templates
        tera.add_raw_template("base.html", include_str!("templates/base.html"))
            .map_err(|e| TemplateError::ParseError(e.to_string()))?;

        tera.add_raw_template("email/welcome.html", WELCOME_EMAIL_TEMPLATE)
            .map_err(|e| TemplateError::ParseError(e.to_string()))?;

        tera.add_raw_template("email/order_confirmation.html", ORDER_CONFIRMATION_TEMPLATE)
            .map_err(|e| TemplateError::ParseError(e.to_string()))?;

        tera.add_raw_template("email/password_reset.html", PASSWORD_RESET_TEMPLATE)
            .map_err(|e| TemplateError::ParseError(e.to_string()))?;

        Ok(Self { tera })
    }

    pub fn render<T: Serialize>(&self, template: &str, data: &T) -> Result<String, TemplateError> {
        let context = Context::from_serialize(data)
            .map_err(|e| TemplateError::ContextError(e.to_string()))?;

        self.tera
            .render(template, &context)
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }

    pub fn render_string<T: Serialize>(
        &self,
        template_content: &str,
        data: &T,
    ) -> Result<String, TemplateError> {
        let context = Context::from_serialize(data)
            .map_err(|e| TemplateError::ContextError(e.to_string()))?;

        Tera::one_off(template_content, &context, false)
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize template engine")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template parse error: {0}")]
    ParseError(String),
    #[error("Template render error: {0}")]
    RenderError(String),
    #[error("Context error: {0}")]
    ContextError(String),
    #[error("Template not found: {0}")]
    NotFound(String),
}

const WELCOME_EMAIL_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Welcome!</title>
</head>
<body>
    <h1>Welcome, {{ username }}!</h1>
    <p>Thank you for joining our platform.</p>
    <p>Your account has been created successfully.</p>
    <a href="{{ verification_link }}">Verify your email</a>
</body>
</html>
"#;

const ORDER_CONFIRMATION_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Order Confirmation</title>
</head>
<body>
    <h1>Order Confirmed!</h1>
    <p>Order Number: {{ order_number }}</p>
    <p>Total: {{ currency }}{{ total }}</p>
    <h2>Items:</h2>
    <ul>
    {% for item in items %}
        <li>{{ item.name }} x {{ item.quantity }} - {{ currency }}{{ item.total_price }}</li>
    {% endfor %}
    </ul>
    <p>Shipping to:</p>
    <address>
        {{ shipping_address.first_name }} {{ shipping_address.last_name }}<br>
        {{ shipping_address.address_line_1 }}<br>
        {% if shipping_address.address_line_2 %}{{ shipping_address.address_line_2 }}<br>{% endif %}
        {{ shipping_address.city }}, {{ shipping_address.state }} {{ shipping_address.postal_code }}<br>
        {{ shipping_address.country }}
    </address>
</body>
</html>
"#;

const PASSWORD_RESET_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Password Reset</title>
</head>
<body>
    <h1>Password Reset Request</h1>
    <p>You requested to reset your password.</p>
    <p>Click the link below to reset your password:</p>
    <a href="{{ reset_link }}">Reset Password</a>
    <p>This link will expire in {{ expiry_hours }} hours.</p>
    <p>If you didn't request this, please ignore this email.</p>
</body>
</html>
"#;

#[derive(Debug, Serialize)]
pub struct WelcomeEmailData {
    pub username: String,
    pub verification_link: String,
}

#[derive(Debug, Serialize)]
pub struct OrderConfirmationData {
    pub order_number: String,
    pub total: f64,
    pub currency: String,
    pub items: Vec<OrderItemData>,
    pub shipping_address: ShippingAddressData,
}

#[derive(Debug, Serialize)]
pub struct OrderItemData {
    pub name: String,
    pub quantity: i32,
    pub total_price: f64,
}

#[derive(Debug, Serialize)]
pub struct ShippingAddressData {
    pub first_name: String,
    pub last_name: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Serialize)]
pub struct PasswordResetData {
    pub reset_link: String,
    pub expiry_hours: i32,
}
