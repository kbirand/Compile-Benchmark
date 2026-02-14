use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: PaymentIntentStatus,
    pub payment_method: Option<PaymentMethod>,
    pub client_secret: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresAction,
    Processing,
    Succeeded,
    Canceled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: Uuid,
    pub payment_type: PaymentMethodType,
    pub card: Option<CardDetails>,
    pub billing_details: Option<BillingDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    BankTransfer,
    PayPal,
    ApplePay,
    GooglePay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub brand: String,
    pub last4: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub funding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Uuid,
    pub payment_intent_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: RefundStatus,
    pub reason: Option<RefundReason>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    Other,
}

pub struct PaymentService {
    api_key: String,
}

impl PaymentService {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn create_payment_intent(
        &self,
        order_id: Uuid,
        amount: f64,
        currency: &str,
    ) -> Result<PaymentIntent, PaymentError> {
        let intent = PaymentIntent {
            id: Uuid::new_v4(),
            order_id,
            amount,
            currency: currency.to_string(),
            status: PaymentIntentStatus::RequiresPaymentMethod,
            payment_method: None,
            client_secret: format!("pi_{}_secret_{}", Uuid::new_v4(), Uuid::new_v4()),
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        tracing::info!(
            payment_intent_id = %intent.id,
            order_id = %order_id,
            amount = %amount,
            "Payment intent created"
        );

        Ok(intent)
    }

    pub async fn confirm_payment(
        &self,
        payment_intent_id: Uuid,
        payment_method: PaymentMethod,
    ) -> Result<PaymentIntent, PaymentError> {
        let intent = PaymentIntent {
            id: payment_intent_id,
            order_id: Uuid::new_v4(),
            amount: 0.0,
            currency: "USD".to_string(),
            status: PaymentIntentStatus::Succeeded,
            payment_method: Some(payment_method),
            client_secret: String::new(),
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(intent)
    }

    pub async fn create_refund(
        &self,
        payment_intent_id: Uuid,
        amount: Option<f64>,
        reason: Option<RefundReason>,
    ) -> Result<Refund, PaymentError> {
        let refund = Refund {
            id: Uuid::new_v4(),
            payment_intent_id,
            amount: amount.unwrap_or(0.0),
            currency: "USD".to_string(),
            status: RefundStatus::Pending,
            reason,
            created_at: Utc::now(),
        };

        tracing::info!(
            refund_id = %refund.id,
            payment_intent_id = %payment_intent_id,
            "Refund created"
        );

        Ok(refund)
    }

    pub async fn get_payment_intent(
        &self,
        payment_intent_id: Uuid,
    ) -> Result<Option<PaymentIntent>, PaymentError> {
        let _ = payment_intent_id;
        Ok(None)
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Payment intent not found: {0}")]
    NotFound(Uuid),
    #[error("Payment failed: {0}")]
    PaymentFailed(String),
    #[error("Invalid payment method: {0}")]
    InvalidPaymentMethod(String),
    #[error("Refund failed: {0}")]
    RefundFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}
